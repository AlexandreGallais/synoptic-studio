//! Tests of `git-sequences` run in repositories of its own (EN-100): a bare `origin`, a clone on
//! a story branch from `develop` (ADR-0086), and a second clone that moves `develop` on.
//! Every `GIT_*` variable is removed from the commands (`.claude/rules/tooling.md`).

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

const STORY: &str = "chore/en-100-git-sequences";
const DEVELOP: &str = "develop";
const FEATURE: &str = "feature/f20-a-context-diet";
const STORY_FILE: &str = "docs/backlog/stories/E17-F20-EN-100-git-sequences.md";

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A command without the `GIT_*` variables of the process running the tests, nor the user's or the
/// system's Git config: an empty home of its own.
fn clean(program: &str, dir: &Path) -> Command {
    let mut command = test_support::command(program);
    let home = env::temp_dir().join(format!("git-sequences-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home made");
    command.current_dir(dir);
    command.env("HOME", home).env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

fn git(dir: &Path, words: &[&str]) -> String {
    let output = clean("git", dir).args(words).output().expect("git runs");

    assert!(
        output.status.success(),
        "git {}: {}",
        words.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// Runs the tool in `dir`, `input` on its standard input.
fn tool(dir: &Path, words: &[&str], input: &str) -> Output {
    let mut child = clean(env!("CARGO_BIN_EXE_git-sequences"), dir)
        .args(words)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the tool runs");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("stdin written");
    child.wait_with_output().expect("the tool ends")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn identify(dir: &Path) {
    git(dir, &["config", "user.name", "Test"]);
    git(dir, &["config", "user.email", "test@example.com"]);
    git(dir, &["config", "commit.gpgsign", "false"]);
    git(dir, &["config", "core.hooksPath", ".git/hooks"]);
}

fn write(dir: &Path, path: &str, text: &str) {
    let file = dir.join(path);

    fs::create_dir_all(file.parent().expect("a parent")).expect("folder made");
    fs::write(file, text).expect("file written");
}

/// Repositories of one test: `origin.git`, `work` on the story branch, `other` on `develop`.
struct Repos {
    root: PathBuf,
    work: PathBuf,
    other: PathBuf,
}

impl Drop for Repos {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn repos() -> Repos {
    let root = env::temp_dir().join(format!(
        "git-sequences-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("root made");

    let origin = root.join("origin.git");
    let work = root.join("work");
    let other = root.join("other");

    git(
        &root,
        &["init", "--quiet", "--bare", "-b", "main", "origin.git"],
    );
    git(&root, &["init", "--quiet", "-b", "main", "work"]);
    identify(&work);
    git(
        &work,
        &["remote", "add", "origin", &origin.to_string_lossy()],
    );
    write(&work, "README.md", "one\n");
    write(&work, STORY_FILE, "# EN-100\n");
    git(&work, &["add", "."]);
    git(&work, &["commit", "--quiet", "-m", "first"]);
    git(&work, &["push", "--quiet", "origin", "main"]);
    git(&work, &["switch", "--quiet", "-c", DEVELOP]);
    git(&work, &["push", "--quiet", "-u", "origin", DEVELOP]);
    git(&work, &["switch", "--quiet", "-c", STORY]);
    git(
        &root,
        &[
            "clone",
            "--quiet",
            "-b",
            DEVELOP,
            &origin.to_string_lossy(),
            "other",
        ],
    );
    identify(&other);

    Repos { root, work, other }
}

/// A commit of `path` on `develop`, pushed from the other clone.
fn move_develop_on(repos: &Repos, path: &str, text: &str) {
    write(&repos.other, path, text);
    git(&repos.other, &["add", path]);
    git(
        &repos.other,
        &["commit", "--quiet", "-m", "develop moves on"],
    );
    git(&repos.other, &["push", "--quiet", "origin", DEVELOP]);
}

#[test]
fn f22_ac4_rebases_the_story_on_the_fetched_develop_in_one_call() {
    // [F22.AC4] A story of F20 is rebased on `develop`, never on a feature branch (ADR-0086).
    let repos = repos();

    write(&repos.work, "story.md", "story\n");
    git(&repos.work, &["add", "story.md"]);
    git(&repos.work, &["commit", "--quiet", "-m", "story work"]);
    move_develop_on(&repos, "develop.md", "develop\n");

    let output = tool(&repos.work, &["sync"], "");

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains(&format!("{STORY} rebased on origin/{DEVELOP}, 1 commit(s)")),
        "{}",
        stdout(&output)
    );
    // `develop`'s commit is under the story's.
    assert_eq!(
        git(&repos.work, &["log", "--format=%s", "-2"]),
        "story work\ndevelop moves on"
    );
}

#[test]
fn moves_develop_forward_without_rebasing_it() {
    let repos = repos();

    git(&repos.work, &["switch", "--quiet", DEVELOP]);
    move_develop_on(&repos, "develop.md", "develop\n");

    let output = tool(&repos.work, &["sync"], "");

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("up to date with its upstream: "));
    assert_eq!(
        git(&repos.work, &["log", "--format=%s", "-1"]),
        "develop moves on"
    );
}

#[test]
fn stops_at_a_conflict_with_gits_own_message() {
    let repos = repos();

    write(&repos.work, "README.md", "story side\n");
    git(&repos.work, &["commit", "--quiet", "-am", "story side"]);
    move_develop_on(&repos, "README.md", "develop side\n");

    let output = tool(&repos.work, &["sync"], "");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("git rebase --quiet origin/"));
    assert!(stderr(&output).contains("CONFLICT"), "{}", stderr(&output));
}

#[test]
fn never_touches_main() {
    let repos = repos();

    git(&repos.work, &["switch", "--quiet", "main"]);
    write(&repos.work, "README.md", "changed\n");

    let sync = tool(&repos.work, &["sync"], "");
    let commit = tool(
        &repos.work,
        &["commit", "-F", "-", "README.md"],
        "chore: x\n",
    );

    assert!(!sync.status.success());
    assert!(stderr(&sync).contains("`main`"));
    assert!(!commit.status.success());
    // Nothing staged, nothing committed.
    assert_eq!(git(&repos.work, &["diff", "--cached", "--name-only"]), "");
    assert_eq!(git(&repos.work, &["log", "--format=%s", "-1"]), "first");
}

#[test]
fn refuses_main_before_the_fetch() {
    let repos = repos();

    git(&repos.work, &["switch", "--quiet", "main"]);
    git(
        &repos.work,
        &["remote", "set-url", "origin", "/nowhere/origin.git"],
    );

    let sync = tool(&repos.work, &["sync"], "");

    // Refused for `main`, not for a fetch that could not run.
    assert!(!sync.status.success());
    assert!(stderr(&sync).contains("`main`"), "{}", stderr(&sync));
    assert!(!stderr(&sync).contains("fetch"), "{}", stderr(&sync));
}

#[test]
fn f20_ac7_commits_on_develop_but_never_pushes_it() {
    let repos = repos();

    git(&repos.work, &["switch", "--quiet", DEVELOP]);
    write(&repos.work, "a.md", "a\n");

    let pushed = tool(
        &repos.work,
        &["commit", "-F", "-", "--push", "a.md"],
        "chore: x\n",
    );

    assert!(!pushed.status.success());
    assert!(
        stderr(&pushed).contains("never pushes it"),
        "{}",
        stderr(&pushed)
    );
    assert_eq!(git(&repos.work, &["log", "--format=%s", "-1"]), "first");
    // On a feature branch left from before ADR-0086, the same call commits and pushes.
    git(&repos.work, &["switch", "--quiet", "-c", FEATURE]);
    let feature = tool(
        &repos.work,
        &["commit", "-F", "-", "--push", "a.md"],
        "chore: x\n",
    );

    assert!(feature.status.success(), "{}", stderr(&feature));
    assert_eq!(
        git(&repos.work, &["rev-parse", &format!("origin/{FEATURE}")]),
        git(&repos.work, &["rev-parse", "HEAD"])
    );
}

#[test]
fn commits_the_named_files_with_a_message_of_several_lines_from_stdin() {
    let repos = repos();
    let message = "chore(tooling): add the sequences\n\nA body line.\n\nRefs: EN-100.T1\n";

    write(&repos.work, "a.md", "a\n");
    write(&repos.work, "b.md", "b\n");
    write(&repos.work, "left.md", "left\n");

    let output = tool(&repos.work, &["commit", "-F", "-", "a.md", "b.md"], message);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("chore(tooling): add the sequences on chore/en-100"));
    assert_eq!(
        git(&repos.work, &["log", "--format=%B", "-1"]),
        message.trim()
    );
    assert_eq!(
        git(&repos.work, &["show", "--name-only", "--format=", "HEAD"]),
        "a.md\nb.md"
    );
    // A file not named stays out.
    assert_eq!(git(&repos.work, &["status", "--porcelain"]), "?? left.md");
}

#[test]
fn commits_from_a_message_file_and_pushes() {
    let repos = repos();
    let file = repos.root.join("message.txt");

    fs::write(&file, "docs(backlog): one\n\nRefs: EN-100\n").expect("message written");
    write(&repos.work, "a.md", "a\n");

    let output = tool(
        &repos.work,
        &["commit", "-F", &file.to_string_lossy(), "--push", "a.md"],
        "",
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).ends_with(", pushed\n"));
    assert_eq!(
        git(&repos.work, &["rev-parse", &format!("origin/{STORY}")]),
        git(&repos.work, &["rev-parse", "HEAD"])
    );
}

#[test]
fn stops_at_the_first_failing_step() {
    let repos = repos();

    write(&repos.work, "a.md", "a\n");

    // `git add` refuses a path that does not exist: no commit follows.
    let missing = tool(
        &repos.work,
        &["commit", "-F", "-", "a.md", "nowhere.md"],
        "chore: x\n",
    );

    assert!(!missing.status.success());
    assert!(stderr(&missing).contains("git add -- a.md nowhere.md failed"));
    assert!(stderr(&missing).contains("nowhere.md"));
    assert_eq!(git(&repos.work, &["log", "--format=%s", "-1"]), "first");

    // An empty message is refused before anything is staged.
    let empty = tool(&repos.work, &["commit", "-F", "-", "a.md"], "\n  \n");

    assert!(stderr(&empty).contains("empty"));
    assert_eq!(git(&repos.work, &["diff", "--cached", "--name-only"]), "");
}

#[test]
fn runs_the_hooks_and_stops_on_their_failure() {
    let repos = repos();

    // The hook fails: the commit stops with its message.
    write(
        &repos.work,
        ".git/hooks/pre-commit",
        "#!/bin/sh\necho \"hook refuses\" >&2\nexit 1\n",
    );
    let chmod = clean("chmod", &repos.work)
        .args(["+x", ".git/hooks/pre-commit"])
        .status()
        .expect("chmod runs");
    assert!(chmod.success());
    write(&repos.work, "a.md", "a\n");

    let output = tool(&repos.work, &["commit", "-F", "-", "a.md"], "chore: x\n");

    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("hook refuses"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn refuses_a_force_push_on_a_shared_branch() {
    let repos = repos();

    git(&repos.work, &["switch", "--quiet", DEVELOP]);
    write(&repos.work, "a.md", "a\n");

    let output = tool(
        &repos.work,
        &["commit", "-F", "-", "--push-with-lease", "a.md"],
        "chore: x\n",
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("shared"));
    assert_eq!(git(&repos.work, &["diff", "--cached", "--name-only"]), "");
}

#[test]
fn prints_its_usage_on_a_malformed_call() {
    let repos = repos();
    let output = tool(&repos.work, &["push"], "");

    assert!(!output.status.success());
    assert!(stderr(&output).contains("usage: target/release/git-sequences sync"));
}

#[test]
fn f21_ac2_names_the_uncommitted_files_that_stop_a_rebase() {
    let repos = repos();

    move_develop_on(&repos, "develop.md", "develop\n");
    // A tracked file changed, a new file staged; an untracked file blocks nothing.
    write(&repos.work, "README.md", "changed\n");
    write(&repos.work, "staged.md", "staged\n");
    git(&repos.work, &["add", "staged.md"]);
    write(&repos.work, "loose.md", "untracked\n");

    let head = git(&repos.work, &["rev-parse", "HEAD"]);
    let output = tool(&repos.work, &["sync"], "");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        stderr(&output),
        format!(
            "git-sequences: {STORY} not rebased: the tree holds uncommitted changes, which the rebase refuses; commit or stash them, then run again:\n  M README.md\n  A staged.md\n"
        )
    );
    assert_eq!(git(&repos.work, &["rev-parse", "HEAD"]), head);
    assert_eq!(
        git(
            &repos.work,
            &["status", "--porcelain", "--untracked-files=no"]
        ),
        "M README.md\nA  staged.md"
    );
}

/// Runs a sync with a `cargo` stub first on `PATH`, a shell script (Unix only) which logs its call
/// and exits with `code`.
#[cfg(unix)]
fn sync_with_cargo(repos: &Repos, code: u8) -> (Output, String) {
    let stubs = repos.root.join("stubs");
    let log = repos.root.join("cargo.log");

    write(
        &stubs,
        "cargo",
        &format!(
            "#!/bin/sh\necho \"$PWD $*\" >> \"{}\"\necho 'error: it broke' >&2\nexit {code}\n",
            log.display()
        ),
    );
    fs::set_permissions(
        stubs.join("cargo"),
        <fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(0o755),
    )
    .expect("stub runs");
    let path = format!(
        "{}:{}",
        stubs.display(),
        env::var("PATH").unwrap_or_default()
    );
    let output = clean(env!("CARGO_BIN_EXE_git-sequences"), &repos.work)
        .arg("sync")
        .env("PATH", path)
        .stdin(Stdio::null())
        .output()
        .expect("the tool runs");

    (output, fs::read_to_string(log).unwrap_or_default())
}

#[cfg(unix)]
#[test]
fn rebuilds_the_release_binaries_of_a_cargo_workspace_after_a_sync() {
    let repos = repos();

    move_develop_on(&repos, "develop.md", "develop\n");
    // Without a workspace, nothing is built.
    let (output, calls) = sync_with_cargo(&repos, 0);

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(calls, "");
    assert!(!stdout(&output).contains("rebuilt"));
    // With one, its release binaries are, in the checkout's folder.
    write(&repos.work, "Cargo.toml", "[workspace]\n");
    let (output, calls) = sync_with_cargo(&repos, 0);
    let work = fs::canonicalize(&repos.work).expect("work");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        calls,
        format!(
            "{} build --release --workspace --locked --quiet\n",
            work.display()
        )
    );
    assert!(
        stdout(&output).ends_with("; release binaries rebuilt\n"),
        "{}",
        stdout(&output)
    );
    // A failed build stops with Cargo's message.
    let (output, _) = sync_with_cargo(&repos, 1);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("synced, but the rebuild of target/release failed"));
    assert!(stderr(&output).contains("error: it broke"));
}
