//! Tests of `git-hooks post-merge` and `git-hooks post-rewrite` (EN-131, F29.AC3): a clone of an
//! upstream repository holding a Cargo workspace of two crates, `alpha` and `beta`, its release
//! binaries written older than nothing, pulls a change; a `cargo` stub first on the `PATH` logs
//! its call and writes the binaries it is told to again. Every command starts without the
//! inherited `GIT_*` variables, nor the user's Git config (`.claude/rules/tooling.md`). Unix only:
//! the hooks and the stub are shell scripts, run by their executable bit.

#![cfg(unix)]

use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};

use git_hooks::rebuild::{AfterPull, message, rebuilt};
use stale_binaries::RELEASE_BUILD;

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A time long past: the sources' in the clone, before any pull.
const SOURCES_WRITTEN: Duration = Duration::from_secs(1_000_000_000);

/// The binaries' time, after the sources' and before any pull.
const BINARIES_BUILT: Duration = Duration::from_secs(1_000_000_100);

/// The temporary folders of one test: `upstream`, its clone `work`, the stub's folder and log.
struct Repos {
    root: PathBuf,
    upstream: PathBuf,
    work: PathBuf,
}

impl Drop for Repos {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// A command without the `GIT_*` variables of the process running the tests, nor the user's or the
/// system's Git config, the stub's folder first on the `PATH`.
fn clean(repos: &Repos, program: &str, dir: &Path) -> Command {
    let mut command = test_support::command(program);
    let home = repos.root.join("home");
    let path = format!(
        "{}:{}",
        repos.root.join("stubs").display(),
        env::var("PATH").unwrap_or_default()
    );

    fs::create_dir_all(&home).unwrap();
    command
        .current_dir(dir)
        .env("HOME", home)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("PATH", path)
        .stdin(Stdio::null());
    command
}

fn run(repos: &Repos, dir: &Path, words: &[&str]) -> Output {
    clean(repos, "git", dir).args(words).output().unwrap()
}

fn git(repos: &Repos, dir: &Path, words: &[&str]) -> String {
    let output = run(repos, dir, words);

    assert!(
        output.status.success(),
        "git {}: {}",
        words.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// What a command printed, its standard output then its error output: Git prints a hook's output
/// on either.
fn printed(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn write(dir: &Path, path: &str, text: &str) {
    let file = dir.join(path);

    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, text).unwrap();
}

fn written_at(path: &Path, since_epoch: Duration) {
    File::options()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(SystemTime::UNIX_EPOCH + since_epoch)
        .unwrap();
}

fn identify(repos: &Repos, dir: &Path) {
    git(repos, dir, &["config", "user.name", "Test"]);
    git(repos, dir, &["config", "user.email", "test@example.com"]);
    git(repos, dir, &["config", "commit.gpgsign", "false"]);
    git(repos, dir, &["config", "pull.rebase", "false"]);
}

/// A shell script, with its executable bit.
fn executable(dir: &Path, path: &str, text: &str) {
    use std::os::unix::fs::PermissionsExt;

    write(dir, path, text);
    fs::set_permissions(dir.join(path), fs::Permissions::from_mode(0o755)).unwrap();
}

/// The `cargo` stub: logs `<folder> <arguments>`, writes again the binaries named, prints an error
/// and exits with `code`.
fn stub_cargo(repos: &Repos, writes: &[&str], code: u8) {
    let touches = writes
        .iter()
        .map(|binary| format!("target/release/{binary}"))
        .collect::<Vec<_>>()
        .join(" ");

    executable(
        &repos.root,
        "stubs/cargo",
        &format!(
            "#!/bin/sh\necho \"$PWD $*\" >> \"{}\"\nfor binary in {touches}; do touch \"$binary\"; done\necho 'error: it broke' >&2\nexit {code}\n",
            repos.root.join("cargo.log").display()
        ),
    );
}

fn cargo_calls(repos: &Repos) -> String {
    fs::read_to_string(repos.root.join("cargo.log")).unwrap_or_default()
}

/// The upstream repository with its workspace, and its clone with the hooks installed and its
/// release binaries built after their sources.
fn repos() -> Repos {
    let root = env::temp_dir().join(format!(
        "rebuild-after-a-pull-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&root);
    let repos = Repos {
        upstream: root.join("upstream"),
        work: root.join("work"),
        root,
    };

    fs::create_dir_all(&repos.upstream).unwrap();
    git(
        &repos,
        &repos.upstream,
        &["init", "--quiet", "--initial-branch=develop"],
    );
    identify(&repos, &repos.upstream);
    write(&repos.upstream, "Cargo.toml", "[workspace]\n");
    write(&repos.upstream, ".gitignore", "target/\n.githooks/\n");
    write(&repos.upstream, "docs/a.md", "a\n");
    for name in ["alpha", "beta"] {
        write(
            &repos.upstream,
            &format!("tools/{name}/Cargo.toml"),
            &format!("[package]\nname = \"{name}\"\n"),
        );
        write(
            &repos.upstream,
            &format!("tools/{name}/src/main.rs"),
            "fn main() {}\n",
        );
    }
    git(&repos, &repos.upstream, &["add", "--all"]);
    git(
        &repos,
        &repos.upstream,
        &["commit", "--quiet", "-m", "chore: start"],
    );

    git(
        &repos,
        &repos.root,
        &["clone", "--quiet", "upstream", "work"],
    );
    identify(&repos, &repos.work);

    let work = fs::canonicalize(&repos.work).unwrap();

    for name in ["alpha", "beta"] {
        let source = work.join(format!("tools/{name}/src/main.rs"));
        let binary = format!("target/release/{name}");

        written_at(&source, SOURCES_WRITTEN);
        write(&work, &binary, "binary\n");
        write(
            &work,
            &format!("{binary}.d"),
            &format!("{}: {}\n", work.join(&binary).display(), source.display()),
        );
        written_at(&work.join(&binary), BINARIES_BUILT);
    }
    for hook in ["post-merge", "post-rewrite"] {
        executable(
            &repos.work,
            &format!(".githooks/{hook}"),
            &format!(
                "#!/bin/sh\nexec '{}' {hook} \"$@\"\n",
                env!("CARGO_BIN_EXE_git-hooks")
            ),
        );
    }
    git(
        &repos,
        &repos.work,
        &["config", "core.hooksPath", ".githooks"],
    );
    assert_eq!(stale_binaries::stale_binaries(&work), Vec::<String>::new());
    repos
}

/// Commits a change of `path` upstream.
fn change_upstream(repos: &Repos, path: &str) {
    write(&repos.upstream, path, "fn main() { println!(\"new\"); }\n");
    git(repos, &repos.upstream, &["add", path]);
    git(
        repos,
        &repos.upstream,
        &["commit", "--quiet", "-m", "chore: change a file"],
    );
}

/// The call the hook makes to Cargo, in the clone's folder.
fn build_call(repos: &Repos) -> String {
    format!(
        "{} {}\n",
        fs::canonicalize(&repos.work).unwrap().display(),
        RELEASE_BUILD.join(" ")
    )
}

#[test]
fn f29_ac3_a_fast_forward_rebuilds_the_binaries_a_rust_change_made_stale() {
    let repos = repos();

    stub_cargo(&repos, &["alpha"], 0);
    change_upstream(&repos, "tools/alpha/src/main.rs");

    let output = run(&repos, &repos.work, &["pull", "--ff-only"]);

    assert!(output.status.success(), "{}", printed(&output));
    assert_eq!(cargo_calls(&repos), build_call(&repos));
    // `alpha` alone was written again: `beta` is named nowhere.
    assert!(
        printed(&output).contains("git-hooks: release binaries rebuilt after the pull: alpha\n"),
        "{}",
        printed(&output)
    );
    assert!(!printed(&output).contains("beta"));
    assert_eq!(
        stale_binaries::stale_binaries(&fs::canonicalize(&repos.work).unwrap()),
        Vec::<String>::new()
    );
}

#[test]
fn f29_ac3_a_merge_rebuilds_the_binaries() {
    let repos = repos();

    stub_cargo(&repos, &["alpha", "beta"], 0);
    // A commit of the clone's own: the pull makes a merge commit, not a fast-forward.
    write(&repos.work, "docs/local.md", "local\n");
    git(&repos, &repos.work, &["add", "docs/local.md"]);
    git(
        &repos,
        &repos.work,
        &["commit", "--quiet", "-m", "docs: add a page"],
    );
    change_upstream(&repos, "Cargo.lock");

    let output = run(&repos, &repos.work, &["pull", "--no-rebase", "--no-edit"]);

    assert!(output.status.success(), "{}", printed(&output));
    assert_eq!(
        git(
            &repos,
            &repos.work,
            &["rev-list", "--count", "--merges", "HEAD"]
        ),
        "1"
    );
    assert_eq!(cargo_calls(&repos), build_call(&repos));
    assert!(
        printed(&output)
            .contains("git-hooks: release binaries rebuilt after the pull: alpha, beta\n"),
        "{}",
        printed(&output)
    );
}

#[test]
fn f29_ac3_a_rebase_rebuilds_the_binaries() {
    let repos = repos();

    stub_cargo(&repos, &["beta"], 0);
    write(&repos.work, "docs/local.md", "local\n");
    git(&repos, &repos.work, &["add", "docs/local.md"]);
    git(
        &repos,
        &repos.work,
        &["commit", "--quiet", "-m", "docs: add a page"],
    );
    change_upstream(&repos, "tools/beta/src/main.rs");

    let output = run(&repos, &repos.work, &["pull", "--rebase"]);

    assert!(output.status.success(), "{}", printed(&output));
    // The clone's commit now sits on the upstream's: a rebase, no merge.
    assert_eq!(
        git(
            &repos,
            &repos.work,
            &["rev-list", "--count", "--merges", "HEAD"]
        ),
        "0"
    );
    assert_eq!(
        git(&repos, &repos.work, &["rev-list", "--count", "HEAD"]),
        "3"
    );
    assert_eq!(cargo_calls(&repos), build_call(&repos));
    assert!(
        printed(&output).contains("git-hooks: release binaries rebuilt after the pull: beta\n"),
        "{}",
        printed(&output)
    );
    assert_eq!(
        stale_binaries::stale_binaries(&fs::canonicalize(&repos.work).unwrap()),
        Vec::<String>::new()
    );
}

#[test]
fn f29_ac3_a_pull_without_a_rust_change_builds_and_prints_nothing() {
    let repos = repos();

    stub_cargo(&repos, &["alpha"], 0);
    change_upstream(&repos, "docs/a.md");

    let merged = run(&repos, &repos.work, &["pull", "--ff-only"]);

    assert!(merged.status.success(), "{}", printed(&merged));
    assert!(
        !printed(&merged).contains("git-hooks: "),
        "{}",
        printed(&merged)
    );

    // By a rebase too.
    write(&repos.work, "docs/local.md", "local\n");
    git(&repos, &repos.work, &["add", "docs/local.md"]);
    git(
        &repos,
        &repos.work,
        &["commit", "--quiet", "-m", "docs: add a page"],
    );
    change_upstream(&repos, "docs/b.md");

    let rebased = run(&repos, &repos.work, &["pull", "--rebase"]);

    assert!(rebased.status.success(), "{}", printed(&rebased));
    assert!(
        !printed(&rebased).contains("git-hooks: "),
        "{}",
        printed(&rebased)
    );
    assert_eq!(cargo_calls(&repos), "");
}

#[test]
fn f29_ac3_a_failed_build_is_printed_with_the_command_to_run_again() {
    let repos = repos();

    stub_cargo(&repos, &[], 101);
    change_upstream(&repos, "tools/alpha/src/main.rs");

    let output = run(&repos, &repos.work, &["pull", "--ff-only"]);

    // The pull is done: the hook's failure is printed, never Git's.
    assert!(output.status.success(), "{}", printed(&output));
    assert_eq!(
        git(&repos, &repos.work, &["rev-parse", "HEAD"]),
        git(&repos, &repos.upstream, &["rev-parse", "HEAD"])
    );
    assert_eq!(cargo_calls(&repos), build_call(&repos));
    assert!(
        printed(&output).contains(
            "git-hooks: the pull is done, but the rebuild of target/release failed; run `npm run rust:build` again:\nerror: it broke\n"
        ),
        "{}",
        printed(&output)
    );
}

#[test]
fn f29_ac3_exits_0_from_the_binary_after_a_failed_build() {
    let repos = repos();

    stub_cargo(&repos, &[], 101);
    change_upstream(&repos, "tools/alpha/src/main.rs");
    git(&repos, &repos.work, &["pull", "--quiet", "--ff-only"]);
    fs::remove_file(repos.root.join("cargo.log")).unwrap();

    // Git ignores a post hook's exit code: the binary run by hand, `ORIG_HEAD` still the head
    // before the pull, shows its own (AUD-031, F7). `post-rewrite` reads its standard input.
    let failure = "git-hooks: the pull is done, but the rebuild of target/release failed; run `npm run rust:build` again:\nerror: it broke\n";

    for (hook, options, input) in [("post-merge", "0", ""), ("post-rewrite", "rebase", "a b\n")] {
        let mut child = clean(&repos, env!("CARGO_BIN_EXE_git-hooks"), &repos.work)
            .args([hook, options])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        std::io::Write::write_all(&mut child.stdin.take().unwrap(), input.as_bytes()).unwrap();

        let output = child.wait_with_output().unwrap();

        assert_eq!(
            output.status.code(),
            Some(0),
            "{hook}: {}",
            printed(&output)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), "", "{hook}");
        assert_eq!(String::from_utf8_lossy(&output.stderr), failure, "{hook}");
    }
    assert_eq!(
        cargo_calls(&repos),
        format!("{}{}", build_call(&repos), build_call(&repos))
    );
}

#[test]
fn an_amend_builds_nothing() {
    let repos = repos();

    stub_cargo(&repos, &["alpha"], 0);
    // A pull of a Rust change first: `ORIG_HEAD` names a head before it, which a rebuild after the
    // amend would read.
    change_upstream(&repos, "tools/alpha/src/main.rs");
    git(&repos, &repos.work, &["pull", "--quiet", "--ff-only"]);
    assert_eq!(cargo_calls(&repos), build_call(&repos));
    fs::remove_file(repos.root.join("cargo.log")).unwrap();

    write(&repos.work, "tools/alpha/src/main.rs", "fn main() { }\n");
    git(&repos, &repos.work, &["add", "--all"]);
    git(
        &repos,
        &repos.work,
        &["commit", "--quiet", "-m", "chore: change alpha"],
    );

    let output = run(
        &repos,
        &repos.work,
        &["commit", "--amend", "--no-edit", "--quiet"],
    );

    assert!(output.status.success(), "{}", printed(&output));
    assert_eq!(cargo_calls(&repos), "");
    assert_eq!(printed(&output), "");
}

#[test]
fn a_checkout_without_a_workspace_builds_nothing() {
    let repos = repos();

    stub_cargo(&repos, &["alpha"], 0);
    git(&repos, &repos.upstream, &["rm", "--quiet", "Cargo.toml"]);
    change_upstream(&repos, "tools/alpha/src/main.rs");

    let output = run(&repos, &repos.work, &["pull", "--ff-only"]);

    assert!(output.status.success(), "{}", printed(&output));
    assert_eq!(cargo_calls(&repos), "");
    assert!(
        !printed(&output).contains("git-hooks: "),
        "{}",
        printed(&output)
    );
}

#[test]
fn without_orig_head_builds_nothing() {
    let repos = repos();

    stub_cargo(&repos, &["alpha"], 0);
    // A clone that never pulled has no `ORIG_HEAD`: the hook run by hand finds no change.
    let output = clean(&repos, env!("CARGO_BIN_EXE_git-hooks"), &repos.work)
        .arg("post-merge")
        .arg("0")
        .output()
        .unwrap();

    assert!(output.status.success(), "{}", printed(&output));
    assert_eq!(printed(&output), "");
    assert_eq!(cargo_calls(&repos), "");
}

#[test]
fn names_the_binaries_written_again() {
    let first = Some(SystemTime::UNIX_EPOCH + BINARIES_BUILT);
    let later = Some(SystemTime::UNIX_EPOCH + BINARIES_BUILT + Duration::from_secs(1));
    let named = |name: &str, time: Option<SystemTime>| (name.to_owned(), time);
    let before = [
        named("alpha", first),
        named("beta", first),
        named("gamma", None),
        named("delta", first),
    ];
    // `alpha` written again, `beta` untouched, `gamma` built for the first time, `delta` removed.
    let after = [
        named("alpha", later),
        named("beta", first),
        named("gamma", later),
        named("delta", None),
    ];

    assert_eq!(rebuilt(&before, &after), ["alpha", "gamma"]);
    assert_eq!(rebuilt(&before, &before), Vec::<String>::new());
    // A binary new to the workspace, absent from the first reading, was built.
    assert_eq!(rebuilt(&[], &after[..1]), ["alpha"]);
}

#[test]
fn prints_what_the_hook_did() {
    assert_eq!(message(&AfterPull::Unchanged), "");
    assert_eq!(message(&AfterPull::Rebuilt(Vec::new())), "");
    assert_eq!(
        message(&AfterPull::Rebuilt(vec![
            "alpha".to_owned(),
            "beta".to_owned()
        ])),
        "git-hooks: release binaries rebuilt after the pull: alpha, beta\n"
    );
    assert_eq!(
        message(&AfterPull::Failed("error: it broke\n".to_owned())),
        "git-hooks: the pull is done, but the rebuild of target/release failed; run `npm run rust:build` again:\nerror: it broke\n"
    );
}
