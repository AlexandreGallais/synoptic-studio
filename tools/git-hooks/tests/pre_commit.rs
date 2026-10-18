//! Tests of `git-hooks pre-commit` in temporary repositories (EN-108, F21.AC4): the tasks of a file
//! run on the staged files, their fixes staged again, a partly staged file keeping its unstaged
//! lines, the original state back after a failure, and an empty commit refused.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Output;

// Every command starts without the inherited `GIT_*` variables: the pre-push hook running
// `check:all` sets `GIT_DIR`, which would point every `git` call at this repository.
use test_support::command;

/// Replaces the first line of each file by `ONE!`, as a fixer of one line would.
const FIX_FIRST_LINE: &str = "#!/bin/sh\nfor f; do { echo 'ONE!'; tail -n +2 \"$f\"; } > \"$f.new\"; mv \"$f.new\" \"$f\"; done\n";

/// Fails, saying why.
const FAIL: &str = "#!/bin/sh\necho \"bad file: $1\"\nexit 1\n";

/// Writes `x` into each file.
const WRITE_X: &str = "#!/bin/sh\nfor f; do echo x > \"$f\"; done\n";

/// Appends its arguments to `calls.log`, outside the files it is given.
const LOG: &str = "#!/bin/sh\necho \"$*\" >> calls.log\n";

/// A temporary repository with its stub commands and a first commit.
struct Repository {
    root: PathBuf,
}

impl Repository {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().canonicalize().unwrap().join(format!(
            "git-hooks-pre-commit-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let repository = Self { root };

        repository.git(&["init", "--quiet", "--initial-branch=develop"]);
        repository.write(".gitignore", "*.sh\ntasks.txt\ncalls.log\n");
        for (name, script) in [
            ("fix.sh", FIX_FIRST_LINE),
            ("fail.sh", FAIL),
            ("x.sh", WRITE_X),
            ("log.sh", LOG),
        ] {
            let path = repository.root.join(name);

            fs::write(&path, script).unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
        repository
    }

    /// The same, with `a.md` committed as `one`, `two`.
    fn with_commit(name: &str) -> Self {
        let repository = Self::new(name);

        repository.write("a.md", "one\ntwo\n");
        repository.git(&["add", "--all"]);
        repository.git(&["commit", "--quiet", "-m", "chore: start"]);
        repository
    }

    fn git(&self, arguments: &[&str]) -> String {
        let output = self.git_output(arguments);

        assert!(output.status.success(), "git {arguments:?}: {output:?}");
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    fn git_output(&self, arguments: &[&str]) -> Output {
        command("git")
            .args(["-c", "commit.gpgsign=false"])
            .args(arguments)
            .current_dir(&self.root)
            .env("GIT_AUTHOR_NAME", "test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .unwrap()
    }

    fn write(&self, path: &str, text: &str) {
        fs::write(self.root.join(path), text).unwrap();
    }

    fn read(&self, path: &str) -> String {
        fs::read_to_string(self.root.join(path)).unwrap()
    }

    /// The staged content of a file.
    fn staged(&self, path: &str) -> String {
        self.git(&["show", &format!(":{path}")])
    }

    /// Runs the hook's command with the tasks given, one per line.
    fn pre_commit(&self, tasks: &str) -> Output {
        self.write("tasks.txt", tasks);
        command(env!("CARGO_BIN_EXE_git-hooks"))
            .args(["pre-commit", "--tasks", "tasks.txt"])
            .current_dir(&self.root)
            .output()
            .unwrap()
    }

    fn remove(self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn f21_ac4_stages_the_fixes_of_the_tasks_and_prints_nothing() {
    let repository = Repository::with_commit("fix");

    repository.write("a.md", "one\ntwo!\n");
    repository.git(&["add", "a.md"]);

    let output = repository.pre_commit("*.md ./fix.sh\n");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        (text(&output.stdout), text(&output.stderr)),
        (String::new(), String::new())
    );
    assert_eq!(repository.staged("a.md"), "ONE!\ntwo!\n");
    assert_eq!(repository.read("a.md"), "ONE!\ntwo!\n");
    // The backup stash is dropped once the run succeeded.
    assert_eq!(repository.git(&["stash", "list"]), "");
    repository.remove();
}

#[test]
fn f21_ac4_a_partly_staged_file_keeps_its_unstaged_lines() {
    let repository = Repository::with_commit("partly");

    // Line 2 staged, then line 2 changed again without staging.
    repository.write("a.md", "one\ntwo!\n");
    repository.git(&["add", "a.md"]);
    repository.write("a.md", "one\ntwo?\n");

    let output = repository.pre_commit("*.md ./fix.sh\n");

    assert!(output.status.success(), "{output:?}");
    // The fix ran on the staged version and is staged; the unstaged line is back on top of it.
    assert_eq!(repository.staged("a.md"), "ONE!\ntwo!\n");
    assert_eq!(repository.read("a.md"), "ONE!\ntwo?\n");
    repository.remove();
}

#[test]
fn f21_ac4_a_failing_task_puts_the_original_state_back() {
    let repository = Repository::with_commit("fails");

    repository.write("a.md", "one\ntwo!\n");
    repository.git(&["add", "a.md"]);
    repository.write("a.md", "one\ntwo?\n");

    let output = repository.pre_commit("*.md ./fix.sh\n*.md ./fail.sh\n");

    assert_eq!(output.status.code(), Some(1));
    let stderr = text(&output.stderr);
    // The failing command named with its output; the original state restored.
    assert!(
        stderr.contains("✖ ./fail.sh failed (exit 1):\nbad file: a.md\n"),
        "{stderr}"
    );
    assert!(
        stderr.contains("Reverted to the original state"),
        "{stderr}"
    );
    assert_eq!(repository.staged("a.md"), "one\ntwo!\n");
    assert_eq!(repository.read("a.md"), "one\ntwo?\n");
    assert_eq!(repository.git(&["stash", "list"]), "");
    repository.remove();
}

#[test]
fn f21_ac4_refuses_a_commit_the_tasks_left_empty() {
    let repository = Repository::new("empty");

    repository.write("b.md", "x\n");
    repository.git(&["add", "--all"]);
    repository.git(&["commit", "--quiet", "-m", "chore: start"]);
    repository.write("b.md", "y\n");
    repository.git(&["add", "b.md"]);

    let output = repository.pre_commit("*.md ./x.sh\n");

    assert_eq!(output.status.code(), Some(1));
    assert!(text(&output.stderr).contains("Prevented an empty git commit"));
    repository.remove();
}

#[test]
fn runs_nothing_without_staged_files_nor_matching_tasks() {
    let repository = Repository::with_commit("nothing");

    // Nothing staged.
    let output = repository.pre_commit("* ./log.sh\n");

    assert!(output.status.success());
    assert!(!repository.root.join("calls.log").exists());

    // A staged file that no pattern matches.
    repository.write("logo.png", "png\n");
    repository.git(&["add", "logo.png"]);
    assert!(repository.pre_commit("*.md ./fail.sh\n").status.success());
    repository.remove();
}

#[test]
fn gives_each_task_its_files_and_skips_deletions() {
    let repository = Repository::with_commit("files");

    repository.write("b.ts", "b\n");
    repository.write("c.json", "{}\n");
    repository.git(&["add", "b.ts", "c.json"]);
    repository.git(&["rm", "--quiet", "a.md"]);

    let output =
        repository.pre_commit("*.ts ./log.sh ts\n*.{json,md} ./log.sh other\n* ./log.sh all\n");

    assert!(output.status.success(), "{output:?}");
    // The deleted a.md reaches no task (`--diff-filter=ACMR`).
    assert_eq!(
        repository.read("calls.log"),
        "ts b.ts\nother c.json\nall b.ts c.json\n"
    );
    repository.remove();
}

#[test]
fn refuses_files_added_with_intent_to_add() {
    let repository = Repository::with_commit("intent");

    repository.write("a.md", "one!\ntwo\n");
    repository.write("new.md", "new\n");
    repository.git(&["add", "a.md"]);
    repository.git(&["add", "--intent-to-add", "new.md"]);

    let output = repository.pre_commit("* ./log.sh\n");

    assert_eq!(output.status.code(), Some(1));
    assert!(text(&output.stderr).contains("--intent-to-add:\n- new.md\n"));
    repository.remove();
}

#[test]
fn f21_ac4_fixes_the_first_commit_without_a_backup() {
    let repository = Repository::new("first");

    repository.write("a.md", "one\ntwo\n");
    repository.git(&["add", "a.md"]);

    let output = repository.pre_commit("*.md ./fix.sh\n");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(repository.staged("a.md"), "ONE!\ntwo\n");
    repository.remove();
}

/// Makes `.githooks/pre-commit` run the binary with the repository's tasks file.
fn install_hook(repository: &Repository, tasks: &str) {
    let hook = repository.root.join(".githooks/pre-commit");

    repository.write("tasks.txt", tasks);
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nexec '{}' pre-commit --tasks tasks.txt\n",
            env!("CARGO_BIN_EXE_git-hooks")
        ),
    )
    .unwrap();
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    repository.git(&["config", "core.hooksPath", ".githooks"]);
}

#[test]
fn f21_ac4_a_commit_holds_the_fixes_by_the_index_or_by_named_paths() {
    let repository = Repository::with_commit("commit");

    install_hook(&repository, "*.md ./fix.sh\n");
    repository.write("a.md", "one\ntwo!\n");
    repository.git(&["add", "a.md"]);
    repository.git(&["commit", "--quiet", "-m", "docs: change a line"]);
    assert_eq!(repository.git(&["show", "HEAD:a.md"]), "ONE!\ntwo!\n");

    // A commit of named paths runs on a temporary index: the default index gets the fix too, so
    // nothing is left to commit.
    repository.write("a.md", "one\ntwo?\n");
    repository.git(&["commit", "--quiet", "-m", "docs: change it again", "a.md"]);
    assert_eq!(repository.git(&["show", "HEAD:a.md"]), "ONE!\ntwo?\n");
    assert_eq!(repository.git(&["status", "--porcelain", "a.md"]), "");
    repository.remove();
}
