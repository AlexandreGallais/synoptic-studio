//! Tests of `knowledge` as a binary (EN-151): the file it opens, named after the repository's
//! `origin`, in a records folder of the temporary folder. `AGENT_RECORDS_ROOT` is always set to that
//! folder or removed, and `HOME` pointed there too, so that the real records folder is never read
//! (`[F48.AC4]`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use knowledge::store::{SCHEMA_VERSION, open_store};

/// A folder of its own: `records/` for the root, `repo/` a repository whose `origin` names
/// `some-project`, `home/` for `HOME`.
fn place(name: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("knowledge-binary-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    fs::create_dir_all(folder.join("records")).unwrap();
    fs::create_dir_all(folder.join("home")).unwrap();
    fs::create_dir_all(folder.join("repo")).unwrap();
    git(&folder.join("repo"), &["init", "-q", "-b", "develop"]);
    git(
        &folder.join("repo"),
        &[
            "remote",
            "add",
            "origin",
            "https://example.org/owner/some-project.git",
        ],
    );
    folder
}

/// Runs Git in a folder, no `GIT_*` variable inherited.
fn git(cwd: &Path, words: &[&str]) {
    let status = test_support::command("git")
        .args(words)
        .current_dir(cwd)
        .status()
        .unwrap();

    assert!(status.success(), "git {words:?}");
}

/// The binary, run in a folder with `HOME` in the place and no records root inherited.
fn knowledge(place: &Path, cwd: &Path) -> Command {
    let mut command = test_support::command(env!("CARGO_BIN_EXE_knowledge"));

    command
        .current_dir(cwd)
        .env("HOME", place.join("home"))
        .env_remove("AGENT_RECORDS_ROOT");
    command
}

/// The output's standard output and error, as text.
fn text(output: &Output) -> (String, String) {
    (
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    )
}

#[test]
fn f48_ac1_makes_records_sqlite_in_the_folder_named_after_origin() {
    let place = place("made");
    let output = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();
    let file = place.join("records/some-project/records.sqlite");
    let made = file.is_file();
    let (stdout, _) = text(&output);

    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success());
    assert!(made);
    assert_eq!(stdout, format!("{}, schema 1", file.display()));
}

#[test]
fn f48_ac1_opens_the_same_file_from_a_linked_worktree() {
    let place = place("worktree");
    let repo = place.join("repo");

    git(
        &repo,
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.org",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "start",
        ],
    );
    git(
        &repo,
        &["worktree", "add", "-q", "-b", "chore/en-151", "../linked"],
    );

    let path = |cwd: &Path| {
        let output = knowledge(&place, cwd)
            .arg("path")
            .env("AGENT_RECORDS_ROOT", place.join("records"))
            .output()
            .unwrap();
        text(&output).0
    };
    let (main, linked) = (path(&repo), path(&place.join("linked")));

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        main,
        place
            .join("records/some-project/records.sqlite")
            .display()
            .to_string()
    );
    assert_eq!(linked, main);
}

#[test]
fn f48_ac1_fails_without_the_variable_and_says_what_to_set() {
    let place = place("unset");
    let unset = knowledge(&place, &place.join("repo"))
        .arg("init")
        .output()
        .unwrap();
    let empty = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", "")
        .output()
        .unwrap();
    let nothing = fs::read_dir(place.join("records")).unwrap().count();

    let _ = fs::remove_dir_all(&place);
    for output in [unset, empty] {
        assert_eq!(output.status.code(), Some(1));
        assert_eq!(
            text(&output).1,
            "knowledge: AGENT_RECORDS_ROOT is not set: name the folder of the records in the env of ~/.claude/settings.json."
        );
    }
    assert_eq!(nothing, 0);
}

#[test]
fn f48_ac1_fails_on_a_records_root_that_does_not_exist_and_makes_none() {
    let place = place("missing-root");
    let root = place.join("mistyped");
    let output = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", &root)
        .output()
        .unwrap();
    let made = root.exists();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        text(&output).1,
        format!(
            "knowledge: The records' folder {} does not exist: create it, or correct AGENT_RECORDS_ROOT.",
            root.display()
        )
    );
    assert!(!made);
}

#[test]
fn f48_ac1_reads_a_newer_file_and_says_it_is_read_only() {
    let place = place("newer");
    let file = place.join("records/some-project/records.sqlite");

    open_store(&file)
        .unwrap()
        .pragma_update(None, "user_version", SCHEMA_VERSION + 1)
        .unwrap();

    let output = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success());
    assert_eq!(
        text(&output).0,
        format!(
            "{}, schema 2, newer than this binary's 1: read only",
            file.display()
        )
    );
}

#[test]
fn prints_its_usage_for_an_unknown_command() {
    let place = place("usage");
    let output = knowledge(&place, &place.join("repo"))
        .arg("drop")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        text(&output).1,
        "knowledge: Usage: target/release/knowledge init | path"
    );
}
