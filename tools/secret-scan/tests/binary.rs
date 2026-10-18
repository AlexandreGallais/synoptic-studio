//! The `secret-scan` command in a fixture repository of its own (EN-110): the tracked files, the
//! staged ones or the paths given, the ignore file and the `.gitignore` files, the report and the
//! exit codes. It runs `git`, and the binary, through `test_support::command`, without the
//! inherited `GIT_*` variables. `f21_ac4_…` marks a test proving the feature's criterion.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

/// A fixture repository, removed at the end of the test.
struct Repo(PathBuf);

impl Drop for Repo {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A GitHub personal access token, built at run time so that none is in the source.
fn token() -> String {
    format!("ghp_{}", "T".repeat(36))
}

/// Runs git in the repository.
fn git(root: &Path, arguments: &[&str]) {
    let status = test_support::command("git")
        .args(arguments)
        .current_dir(root)
        .status()
        .expect("git runs");

    assert!(status.success(), "git {arguments:?}");
}

/// A repository holding `files`, all of them tracked.
fn repo(name: &str, files: &[(&str, String)]) -> Repo {
    let root = env::temp_dir().join(format!("secret-scan-{}-{name}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("the folder");
    git(&root, &["init", "-q"]);
    for (file, text) in files {
        let path = root.join(file);

        fs::create_dir_all(path.parent().expect("a folder")).expect("folders");
        fs::write(path, text).expect("a file");
    }
    git(&root, &["add", "-A", "-f"]);
    Repo(root)
}

/// The scan's run in the repository.
fn scan(root: &Path, arguments: &[&str]) -> Output {
    test_support::command(env!("CARGO_BIN_EXE_secret-scan"))
        .args(arguments)
        .current_dir(root)
        .output()
        .expect("the scan runs")
}

/// The standard output.
fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn f21_ac4_a_tracked_secret_fails_naming_the_file_the_line_and_the_rule_never_the_secret() {
    let repo = repo(
        "tracked",
        &[
            ("clean.md", "# Nothing here\n".to_owned()),
            (
                "docs/leak.txt",
                format!("first line\ntoken = {}\n", token()),
            ),
        ],
    );
    let output = scan(&repo.0, &[]);
    let report = stdout(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        report.starts_with("docs/leak.txt\n  2:8  error  [GITHUB_TOKEN] "),
        "{report}"
    );
    assert!(report.contains("@secretlint/secretlint-rule-github"));
    assert!(report.ends_with("✖ 1 problem (1 error, 0 warnings, 0 infos)\n"));
    assert!(!report.contains(&token()));
}

#[test]
fn a_tree_without_a_secret_passes_silently() {
    let repo = repo("clean", &[("a.md", "# A\n".to_owned())]);
    let output = scan(&repo.0, &[]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout(&output), "");
}

#[test]
fn f21_ac4_the_ignore_file_gitignore_and_node_modules_are_skipped() {
    let repo = repo(
        "ignored",
        &[
            (".secret-scan-ignore", "vendor\n".to_owned()),
            ("vendor/a.txt", token()),
            ("generated/.gitignore", "*.txt\n".to_owned()),
            ("generated/b.txt", token()),
            ("node_modules/c.txt", token()),
            ("kept.txt", token()),
        ],
    );
    let report = stdout(&scan(&repo.0, &[]));

    assert!(report.starts_with("kept.txt\n"), "{report}");
    assert!(report.contains("✖ 1 problem"));
}

#[test]
fn staged_scans_only_the_staged_files() {
    let repo = repo("staged", &[("old.txt", token())]);

    git(
        &repo.0,
        &[
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "commit",
            "-q",
            "-m",
            "old",
        ],
    );
    fs::write(repo.0.join("new.txt"), token()).expect("a file");
    fs::write(repo.0.join("unstaged.txt"), token()).expect("a file");
    git(&repo.0, &["add", "new.txt"]);

    let output = scan(&repo.0, &["--staged"]);
    let report = stdout(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(report.starts_with("new.txt\n"), "{report}");
    assert!(!report.contains("old.txt") && !report.contains("unstaged.txt"));
}

#[test]
fn paths_given_are_scanned_and_a_missing_one_skipped() {
    let repo = repo("paths", &[("a.txt", token()), ("b.txt", token())]);
    let output = scan(&repo.0, &["b.txt", "gone.txt"]);
    let report = stdout(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        report.starts_with("b.txt\n") && !report.contains("a.txt"),
        "{report}"
    );
}

#[test]
fn a_wrong_call_or_only_missing_paths_is_fatal() {
    let repo = repo("fatal", &[("a.md", "# A\n".to_owned())]);

    assert_eq!(scan(&repo.0, &["--all"]).status.code(), Some(2));
    assert_eq!(scan(&repo.0, &["--staged", "a.md"]).status.code(), Some(2));
    assert_eq!(scan(&repo.0, &["gone.txt"]).status.code(), Some(2));
}

#[test]
fn outside_a_repository_git_fails_and_the_scan_is_fatal() {
    let root = env::temp_dir().join(format!("secret-scan-{}-outside", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("the folder");
    let output = test_support::command(env!("CARGO_BIN_EXE_secret-scan"))
        .current_dir(&root)
        .env("GIT_CEILING_DIRECTORIES", env::temp_dir())
        .output()
        .expect("the scan runs");
    let _ = fs::remove_dir_all(&root);

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("secret-scan: git ls-files"));
}
