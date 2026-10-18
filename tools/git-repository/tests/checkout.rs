//! Tests of a checkout read from the files Git writes (EN-104, moved from `hooks` at NAM-002): the
//! same folders and branch as Git gives, in the main checkout and in a linked worktree; and of a
//! Git command's output.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use git_repository::{Failure, branch_of, checkout_of, common_folder, lexical_join, stdout_of};

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself (`.claude/rules/tooling.md`); started through the
/// shared `test_support::command` (EN-103), with a home of its own.
fn clean(program: &str) -> Command {
    let mut command = test_support::command(program);
    let home = std::env::temp_dir().join(format!("git-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
    command
        .env("HOME", home)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@example.org")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@example.org");
    command
}

/// Git's trimmed output in a folder, panicking on a failure.
fn git(folder: &Path, arguments: &[&str]) -> String {
    let output = clean("git")
        .args(arguments)
        .current_dir(folder)
        .output()
        .expect("git runs");

    assert!(output.status.success(), "git {arguments:?}");
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// A path with its links resolved, to compare with Git's.
fn real(path: &Path) -> PathBuf {
    fs::canonicalize(path).expect("path exists")
}

#[test]
fn reads_the_folders_and_branch_git_gives_in_a_checkout_and_its_worktree() {
    let root = std::env::temp_dir().join(format!("git-checkout-{}", std::process::id()));
    let main = root.join("main");
    let worktree = main.join(".claude/worktrees/en-1");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(main.join("sub")).expect("folders");
    git(&main, &["init", "-q", "-b", "develop"]);
    git(&main, &["commit", "-q", "--allow-empty", "-m", "start"]);
    git(
        &main,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "chore/en-1-x",
            ".claude/worktrees/en-1",
        ],
    );
    fs::create_dir_all(worktree.join("deep/er")).expect("folder");

    for (folder, branch) in [
        (main.join("sub"), "develop"),
        (worktree.join("deep/er"), "chore/en-1-x"),
    ] {
        let found = checkout_of(&folder).expect("a checkout");

        assert_eq!(
            real(&found.top),
            real(Path::new(&git(&folder, &["rev-parse", "--show-toplevel"])))
        );
        assert_eq!(
            real(&found.common),
            real(Path::new(&git(
                &folder,
                &["rev-parse", "--path-format=absolute", "--git-common-dir"]
            )))
        );
        assert_eq!(
            real(&found.git_dir),
            real(Path::new(&git(
                &folder,
                &["rev-parse", "--path-format=absolute", "--git-dir"]
            )))
        );
        assert_eq!(branch_of(&found), branch);
        assert_eq!(
            common_folder(&folder).map(|common| real(Path::new(&common))),
            Some(real(&main.join(".git")))
        );
    }

    // A detached head has no branch, as `git branch --show-current` prints none.
    git(&worktree, &["checkout", "-q", "--detach"]);
    assert_eq!(branch_of(&checkout_of(&worktree).expect("a checkout")), "");
    // A `.git` file that names no folder is no checkout.
    fs::write(worktree.join(".git"), "nothing").expect("broken");
    assert_eq!(checkout_of(&worktree), None);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn joins_a_folder_without_reading_the_disk() {
    assert_eq!(lexical_join("/a/b", "c/./d"), "/a/b/c/d");
    assert_eq!(lexical_join("/a/b", "../../.."), "/");
    assert_eq!(lexical_join("/a", "/x/y"), "/x/y");
}

#[test]
fn gives_a_command_s_output_or_why_it_gave_none() {
    let root = std::env::temp_dir().join(format!("git-output-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("folder");
    // Its output, untrimmed.
    assert!(
        stdout_of(clean("git").arg("--version"))
            .expect("git runs")
            .starts_with("git version ")
    );
    assert!(
        stdout_of(clean("git").arg("--version"))
            .expect("git runs")
            .ends_with('\n')
    );
    // Outside a repository, Git exits 128: its words and error output, trimmed.
    let failed = stdout_of(
        clean("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(&root)
            .env("GIT_CEILING_DIRECTORIES", &root),
    )
    .expect_err("no repository");

    match &failed {
        Failure::Exited { words, error } => {
            assert_eq!(words, "git rev-parse --git-dir");
            assert!(error.starts_with("fatal: not a git repository"), "{error}");
            assert!(!error.ends_with('\n'));
        }
        Failure::NotStarted { .. } => panic!("git started"),
    }
    assert!(
        failed
            .to_string()
            .starts_with("git rev-parse --git-dir: fatal: not a git repository")
    );
    // A program that cannot start: its name and the system's error.
    let missing = stdout_of(&mut test_support::command("no-such-program-of-nam-002"))
        .expect_err("not started");

    assert!(matches!(missing, Failure::NotStarted { .. }));
    // The system's error is its source; Git's own exit has none.
    assert!(std::error::Error::source(&missing).is_some());
    assert!(std::error::Error::source(&failed).is_none());
    assert!(
        missing
            .to_string()
            .starts_with("no-such-program-of-nam-002: "),
        "{missing}"
    );
    let _ = fs::remove_dir_all(&root);
}
