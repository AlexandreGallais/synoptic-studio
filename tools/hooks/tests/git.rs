//! Tests of a checkout read from the files Git writes (EN-104): the same folders and branch as Git
//! gives, in the main checkout and in a linked worktree.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use hooks::git::{branch_of, checkout_of, common_folder};

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
