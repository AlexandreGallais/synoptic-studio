//! Tests of `git-sequences` run in repositories of its own (EN-100): a bare `origin`, a clone on
//! a story branch from its feature branch, and a second clone that moves the feature branch on.
//! Every `GIT_*` variable is removed from the commands (`.claude/rules/tooling.md`).

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

const STORY: &str = "chore/en-100-git-sequences";
const FEATURE: &str = "feature/f20-a-context-diet";
/// What a `knowledge` script standing for the records prints for EN-100, a story of F20 (EN-154).
const EN_100: &str = "kind\\tfeature\\ttopic\\nEN\\tF20\\tgit-sequences\\n";

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

/// A copy of the tool beside a `knowledge` script standing for the records, in the folder of the
/// test's repositories: it holds EN-100 alone, so that no test reads the real records.
fn beside_records(dir: &Path) -> PathBuf {
    let folder = dir.parent().expect("the repositories' folder").join("bin");
    let binary = folder.join("git-sequences");

    if !binary.exists() {
        fs::create_dir_all(&folder).expect("bin made");
        fs::copy(env!("CARGO_BIN_EXE_git-sequences"), &binary).expect("tool copied");
        write(
            &folder,
            "knowledge",
            &format!(
                "#!/bin/sh\ncase \"$2\" in *\"'EN-100'\"*) printf '{EN_100}';; *) printf 'kind\\tfeature\\ttopic\\n';; esac\n"
            ),
        );
        #[cfg(unix)]
        fs::set_permissions(
            folder.join("knowledge"),
            <fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(0o755),
        )
        .expect("script runs");
    }
    binary
}

/// Runs the tool in `dir`, `input` on its standard input.
fn tool(dir: &Path, words: &[&str], input: &str) -> Output {
    let mut child = clean(beside_records(dir).to_str().expect("a path"), dir)
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

/// Repositories of one test: `origin.git`, `work` on the story branch, `other` on the feature.
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

    git(&work, &["add", "."]);
    git(&work, &["commit", "--quiet", "-m", "first"]);
    git(&work, &["push", "--quiet", "origin", "main"]);
    git(&work, &["switch", "--quiet", "-c", FEATURE]);
    git(&work, &["push", "--quiet", "-u", "origin", FEATURE]);
    git(&work, &["switch", "--quiet", "-c", STORY]);
    git(
        &root,
        &[
            "clone",
            "--quiet",
            "-b",
            FEATURE,
            &origin.to_string_lossy(),
            "other",
        ],
    );
    identify(&other);

    Repos { root, work, other }
}

/// A commit of `path` on the feature branch, pushed from the other clone.
fn move_feature_on(repos: &Repos, path: &str, text: &str) {
    write(&repos.other, path, text);
    git(&repos.other, &["add", path]);
    git(
        &repos.other,
        &["commit", "--quiet", "-m", "feature moves on"],
    );
    git(&repos.other, &["push", "--quiet", "origin", FEATURE]);
}

#[test]
fn rebases_the_story_on_its_fetched_feature_branch_in_one_call() {
    let repos = repos();

    write(&repos.work, "story.md", "story\n");
    git(&repos.work, &["add", "story.md"]);
    git(&repos.work, &["commit", "--quiet", "-m", "story work"]);
    move_feature_on(&repos, "feature.md", "feature\n");

    let output = tool(&repos.work, &["sync"], "");

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains(&format!("{STORY} rebased on origin/{FEATURE}, 1 commit(s)")),
        "{}",
        stdout(&output)
    );
    // The feature's commit is under the story's.
    assert_eq!(
        git(&repos.work, &["log", "--format=%s", "-2"]),
        "story work\nfeature moves on"
    );
}

#[test]
fn moves_a_feature_branch_forward_without_rebasing_it() {
    let repos = repos();

    git(&repos.work, &["switch", "--quiet", FEATURE]);
    move_feature_on(&repos, "feature.md", "feature\n");

    let output = tool(&repos.work, &["sync"], "");

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("up to date with its upstream: "));
    assert_eq!(
        git(&repos.work, &["log", "--format=%s", "-1"]),
        "feature moves on"
    );
}

#[test]
fn stops_at_a_conflict_with_gits_own_message() {
    let repos = repos();

    write(&repos.work, "README.md", "story side\n");
    git(&repos.work, &["commit", "--quiet", "-am", "story side"]);
    move_feature_on(&repos, "README.md", "feature side\n");

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

    git(&repos.work, &["switch", "--quiet", "-c", "develop"]);
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
    // On a feature branch, the same call commits and pushes.
    git(&repos.work, &["switch", "--quiet", FEATURE]);
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
fn rev009_commits_a_path_whose_deletion_is_staged() {
    // `git add -- old` failed with "pathspec did not match" once `git rm -r old` had removed the
    // folder (incident card of 2026-10-24); a path neither there nor deleted still fails.
    let repos = repos();
    let message = "chore(tooling): drop the old folder\n\nRefs: EN-100.T1\n";

    write(&repos.work, "old/x.md", "x\n");
    write(&repos.work, "old/y.md", "y\n");
    git(&repos.work, &["add", "old"]);
    git(&repos.work, &["commit", "--quiet", "-m", "old"]);
    git(&repos.work, &["rm", "--quiet", "-r", "old"]);
    write(&repos.work, "a.md", "a\n");

    let output = tool(&repos.work, &["commit", "-F", "-", "old", "a.md"], message);

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        git(&repos.work, &["show", "--name-only", "--format=", "HEAD"]),
        "a.md\nold/x.md\nold/y.md"
    );

    let missing = tool(&repos.work, &["commit", "-F", "-", "nowhere.md"], message);

    assert!(!missing.status.success());
    assert!(
        stderr(&missing).contains("pathspec 'nowhere.md' did not match"),
        "{}",
        stderr(&missing)
    );
}

#[cfg(unix)]
#[test]
fn commits_a_path_given_through_a_symbolic_link_to_a_folder() {
    // `.claude/skills` links to `../.agents/skills`: Git refused `.claude/skills/run/x.md` as
    // beyond a symbolic link (incident of 2026-10-25); the tool commits it by its target.
    let repos = repos();
    let message = "chore(tooling): change a skill\n\nRefs: EN-100.T1\n";

    write(&repos.work, ".agents/skills/run/x.md", "x\n");
    fs::create_dir_all(repos.work.join(".claude")).expect("folder");
    std::os::unix::fs::symlink("../.agents/skills", repos.work.join(".claude/skills"))
        .expect("link");
    git(&repos.work, &["add", ".claude/skills"]);
    git(&repos.work, &["commit", "--quiet", "-m", "link"]);
    write(&repos.work, ".agents/skills/run/x.md", "changed\n");

    let output = tool(
        &repos.work,
        &["commit", "-F", "-", ".claude/skills/run/x.md"],
        message,
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        git(&repos.work, &["show", "--name-only", "--format=", "HEAD"]),
        ".agents/skills/run/x.md"
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
fn f33_ac2_prints_what_the_hooks_write_when_the_commit_succeeds() {
    let repos = repos();

    // A `commit-msg` hook's notes go to standard error and pass: they reach the agent, though the
    // commit succeeds (AUD-035, F3).
    write(
        &repos.work,
        ".git/hooks/commit-msg",
        "#!/bin/sh\necho \"hook: 1 note\" >&2\nexit 0\n",
    );
    let chmod = clean("chmod", &repos.work)
        .args(["+x", ".git/hooks/commit-msg"])
        .status()
        .expect("chmod runs");
    assert!(chmod.success());
    write(&repos.work, "a.md", "a\n");

    let output = tool(&repos.work, &["commit", "-F", "-", "a.md"], "chore: x\n");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stderr(&output), "hook: 1 note\n");
    assert!(stdout(&output).contains("chore: x on chore/en-100"));

    // A commit whose hooks write nothing prints nothing on standard error.
    fs::remove_file(repos.work.join(".git/hooks/commit-msg")).expect("hook removed");
    write(&repos.work, "b.md", "b\n");
    let quiet = tool(&repos.work, &["commit", "-F", "-", "b.md"], "chore: y\n");

    assert!(quiet.status.success(), "{}", stderr(&quiet));
    assert_eq!(stderr(&quiet), "");
}

#[test]
fn refuses_a_force_push_on_a_shared_branch() {
    let repos = repos();

    git(&repos.work, &["switch", "--quiet", FEATURE]);
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

    move_feature_on(&repos, "feature.md", "feature\n");
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
    let output = clean(
        beside_records(&repos.work).to_str().expect("a path"),
        &repos.work,
    )
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

    move_feature_on(&repos, "feature.md", "feature\n");
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

#[test]
fn f31_ac1_prints_a_worktree_s_state_in_one_call() {
    let other = repos();
    let repos = repos();

    write(&repos.work, "story.md", "story\n");
    git(&repos.work, &["add", "story.md"]);
    git(&repos.work, &["commit", "--quiet", "-m", "story work"]);
    write(&repos.work, "README.md", "two\n");
    write(&repos.work, "new.md", "new\n");

    let output = tool(&repos.work, &["state"], "");
    let head = git(&repos.work, &["log", "-1", "--format=%h"]);

    assert!(output.status.success(), "{}", stderr(&output));
    // The leading space of each `status --short` and `diff --stat` line is kept.
    assert_eq!(
        stdout(&output),
        format!(
            "work: {STORY} on origin/{FEATURE}, 1 commit(s) ahead\nstatus --short:\n M README.md\n?? new.md\ndiff --stat HEAD:\n README.md | 2 +-\n 1 file changed, 1 insertion(+), 1 deletion(-)\ncommits ahead:\n{head} story work\n"
        )
    );
    // It only reads: the tree is as it was.
    assert_eq!(
        git(&repos.work, &["status", "--short"]),
        "M README.md\n?? new.md"
    );

    // A feature branch is counted from its upstream.
    git(&other.work, &["switch", "--quiet", FEATURE]);
    let output = tool(&other.work, &["state"], "");

    assert_eq!(
        stdout(&output),
        format!(
            "work: {FEATURE} on origin/{FEATURE}, 0 commit(s) ahead\nstatus --short: clean\ndiff --stat HEAD: none\ncommits ahead: none\n"
        )
    );
}
