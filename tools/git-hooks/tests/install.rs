//! Tests of `git-hooks install` (EN-108, F21.AC4): the hooks folder set once, in a temporary
//! repository, and a linked worktree running the hooks of its own checkout (EN-052).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

// Every command starts without the inherited `GIT_*` variables: the pre-push hook running
// `check:all` sets `GIT_DIR`, which would point every `git` call at this repository.
use test_support::command;

/// A fresh temporary folder, removed first if a previous run left it.
fn folder(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("git-hooks-install-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
}

fn git(folder: &Path, arguments: &[&str]) -> Output {
    command("git")
        .args(["-c", "commit.gpgsign=false"])
        .args(arguments)
        .current_dir(folder)
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .output()
        .unwrap()
}

fn install(folder: &Path) -> Output {
    command(env!("CARGO_BIN_EXE_git-hooks"))
        .arg("install")
        .current_dir(folder)
        .output()
        .unwrap()
}

fn hooks_path(folder: &Path) -> String {
    String::from_utf8_lossy(&git(folder, &["config", "--get", "core.hooksPath"]).stdout)
        .trim()
        .to_owned()
}

/// A tracked `pre-commit` hook that writes the name of the folder it runs in into `ran-in`.
fn write_hook(root: &Path) {
    let hook = root.join(".githooks/pre-commit");

    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\nbasename \"$(pwd)\" > ran-in\n").unwrap();
    // Unix-only: Git runs a hook only when it is executable there; Windows has no such bit.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

#[test]
fn f21_ac4_sets_the_hooks_folder_once() {
    let root = folder("once");
    assert!(git(&root, &["init", "--quiet"]).status.success());

    let first = install(&root);
    // First run: the setting is written and named.
    assert!(first.status.success());
    assert_eq!(
        String::from_utf8_lossy(&first.stdout),
        "git-hooks: core.hooksPath set to .githooks\n"
    );
    assert_eq!(hooks_path(&root), ".githooks");

    // Second run: already set, nothing written nor printed.
    let second = install(&root);
    assert!(second.status.success());
    assert_eq!(String::from_utf8_lossy(&second.stdout), "");
    assert_eq!(hooks_path(&root), ".githooks");

    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn f21_ac4_replaces_husky_s_folder() {
    let root = folder("husky");
    assert!(git(&root, &["init", "--quiet"]).status.success());
    assert!(
        git(&root, &["config", "core.hooksPath", ".husky/_"])
            .status
            .success()
    );

    assert!(install(&root).status.success());
    assert_eq!(hooks_path(&root), ".githooks");

    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn outside_a_repository_installs_nothing_and_succeeds() {
    let root = folder("outside");

    let output = install(&root);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "git-hooks: not in a Git work tree, no hook installed\n"
    );

    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn f21_ac4_a_linked_worktree_runs_its_own_hooks() {
    let root = folder("worktree");
    let main = root.join("main");
    fs::create_dir_all(&main).unwrap();
    assert!(
        git(&main, &["init", "--quiet", "--initial-branch=develop"])
            .status
            .success()
    );
    write_hook(&main);
    assert!(git(&main, &["add", "--all"]).status.success());
    assert!(install(&main).status.success());
    assert!(
        git(&main, &["commit", "--quiet", "-m", "chore: start"])
            .status
            .success()
    );
    // The main checkout's commit ran its hook, in its own root.
    assert_eq!(fs::read_to_string(main.join("ran-in")).unwrap(), "main\n");

    let linked = root.join("linked");
    let added = git(
        &main,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "chore/en-999-topic",
            linked.to_str().unwrap(),
        ],
    );
    assert!(
        added.status.success(),
        "{}",
        String::from_utf8_lossy(&added.stderr)
    );
    fs::write(linked.join("file.txt"), "text\n").unwrap();
    assert!(git(&linked, &["add", "file.txt"]).status.success());
    assert!(
        git(&linked, &["commit", "--quiet", "-m", "chore: add a file"])
            .status
            .success()
    );
    // The linked worktree's commit ran the hook of its own checkout, nothing installed there.
    assert_eq!(
        fs::read_to_string(linked.join("ran-in")).unwrap(),
        "linked\n"
    );

    fs::remove_dir_all(&root).unwrap();
}
