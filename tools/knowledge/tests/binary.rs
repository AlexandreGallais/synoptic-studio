//! Tests of `knowledge` as a binary (EN-151): the file it opens, named after the repository's
//! `origin`, in a records folder of the temporary folder. `AGENT_RECORDS_ROOT` is always set to that
//! folder or removed, and `HOME` pointed there too, so that the real records folder is never read
//! (`[F48.AC4]`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use std::io::Write as _;
use std::process::Stdio;

use knowledge::add::{Origin, add_document};
use knowledge::store::{SCHEMA_VERSION, open_store, write};

/// The usage the binary prints for a wrong call.
const USAGE: &str = "Usage: target/release/knowledge init | path | add <kind> <file or -> | search <words> [<filters>] | show <id> [<heading>] [<filters>] | list [<filters>] | sync; the filters: --kind, --epic, --feature, --status, --since, --branch, --state <value>, --all";

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
    assert_eq!(stdout, format!("{}, schema 2", file.display()));
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
            "{}, schema 3, newer than this binary's 2: read only",
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
    assert_eq!(text(&output).1, format!("knowledge: {USAGE}"));
}

/// Commits nothing in a repository, as a test's author.
fn commit(repo: &Path, message: &str) {
    git(
        repo,
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
            message,
        ],
    );
}

/// A place whose repository has one commit on `develop`, then checks out a new branch.
fn on_branch(name: &str, branch: &str) -> PathBuf {
    let place = place(name);

    commit(&place.join("repo"), "start");
    git(&place.join("repo"), &["checkout", "-q", "-b", branch]);
    place
}

/// The binary run in the place's repository, its records root set to the place's.
fn run(place: &Path, words: &[&str]) -> Output {
    knowledge(place, &place.join("repo"))
        .args(words)
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap()
}

/// The place's records file.
fn records(place: &Path) -> PathBuf {
    place.join("records/some-project/records.sqlite")
}

/// A story with two sections under its title.
const STORY: &str = "---
id: EN-900
epic: E49
feature: F48
title: Add, search, show and list
status: ready
---

# E49 · F48 · EN-900 — Add, search, show and list

As ADR-0081 says.

## Acceptance criteria

- A busy timeout waits for the other writer.

## Tasks

- T1.
";

#[test]
fn f48_ac2_adds_a_story_as_a_draft_of_the_branch_and_shows_it_back_as_written() {
    let place = on_branch("add", "chore/en-900-add");

    fs::write(place.join("repo/story.md"), STORY).unwrap();

    let added = run(&place, &["add", "story", "story.md"]);
    let shown = run(&place, &["show", "EN-900"]);
    let row: (String, String, String) = open_store(&records(&place))
        .unwrap()
        .query_row(
            "SELECT branch, state, path FROM documents WHERE id = 'EN-900'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(text(&added).0, "EN-900 added on chore/en-900-add, a draft");
    assert!(shown.status.success());
    assert_eq!(String::from_utf8_lossy(&shown.stdout), STORY);
    assert_eq!(
        row,
        (
            "chore/en-900-add".to_owned(),
            "draft".to_owned(),
            "story.md".to_owned()
        )
    );
}

#[test]
fn f48_ac2_adds_a_log_row_read_from_the_standard_input() {
    let place = on_branch("stdin", "chore/en-900-stdin");
    let mut child = knowledge(&place, &place.join("repo"))
        .args(["add", "journal", "-"])
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"- `#run` EN-050: done.\n")
        .unwrap();

    let added = child.wait_with_output().unwrap();
    let id = format!("JOURNAL-{}-1", calendar::today());
    let shown = run(&place, &["show", &id]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        text(&added).0,
        format!("{id} added on chore/en-900-stdin, a draft")
    );
    assert_eq!(text(&shown).0, "- `#run` EN-050: done.");
}

#[test]
fn f48_ac2_searches_lists_and_shows_a_section() {
    let place = on_branch("read", "chore/en-900-read");

    fs::write(place.join("repo/story.md"), STORY).unwrap();
    run(&place, &["add", "story", "story.md"]);

    let found = run(&place, &["search", "busy", "timeout", "--kind", "story"]);
    let other_kind = run(&place, &["search", "busy", "--kind", "note"]);
    let listed = run(&place, &["list", "--epic", "E49"]);
    let section = run(&place, &["show", "EN-900", "Acceptance criteria"]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        text(&found).0,
        "EN-900 › Acceptance criteria\n    ## Acceptance criteria - A [busy] [timeout] waits for the other writer."
    );
    assert_eq!(text(&other_kind), (String::new(), String::new()));
    assert_eq!(
        text(&listed).0,
        format!(
            "EN-900 [story, ready, {}] Add, search, show and list — draft on chore/en-900-read",
            calendar::today()
        )
    );
    // Its heading on line 13 of the story.
    assert_eq!(
        text(&section).0,
        "EN-900:13\n## Acceptance criteria\n\n- A busy timeout waits for the other writer."
    );
}

#[test]
fn f48_ac2_marks_merged_the_rows_of_the_branches_merged_into_develop_and_no_other() {
    let place = on_branch("sync", "feature/a");
    let repo = place.join("repo");

    // feature/a's tip is develop's: merged. chore/c is one commit ahead: not merged.
    git(&repo, &["checkout", "-q", "-b", "chore/c"]);
    commit(&repo, "ahead");

    let database = open_store(&records(&place)).unwrap();

    for (id, branch) in [
        ("EN-901", "feature/a"),
        ("EN-902", "develop"),
        ("EN-903", "chore/c"),
    ] {
        write(&database, |transaction| {
            add_document(
                transaction,
                &format!("---\nid: {id}\n---\n# {id}\n"),
                Origin {
                    kind: "story",
                    path: "",
                    branch,
                    today: "2026-10-25",
                },
            )
        })
        .unwrap();
    }

    let synced = run(&place, &["sync"]);
    let states: Vec<String> = database
        .prepare("SELECT id || ' ' || state FROM documents ORDER BY id")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    let of_c = run(&place, &["list"]);

    git(&repo, &["checkout", "-q", "-b", "chore/d", "develop"]);

    let of_d = run(&place, &["list"]);

    git(&repo, &["checkout", "-q", "develop"]);
    fs::write(repo.join("one.md"), "---\nid: EN-904\n---\n# EN-904\n").unwrap();
    run(&place, &["add", "story", "one.md"]);

    let one = run(&place, &["sync"]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        text(&synced).0,
        "2 rows marked merged, of the 2 branches merged into develop"
    );
    assert_eq!(states, ["EN-901 merged", "EN-902 merged", "EN-903 draft"]);
    // The row added on develop; develop, feature/a and chore/d, made from develop, merged.
    assert_eq!(
        text(&one).0,
        "1 row marked merged, of the 3 branches merged into develop"
    );
    assert_eq!(text(&of_c).0.lines().count(), 3);
    // chore/d reads the merged rows, never chore/c's draft.
    assert_eq!(
        text(&of_d)
            .0
            .lines()
            .map(|line| line.split(' ').next().unwrap_or_default())
            .collect::<Vec<_>>(),
        ["EN-901", "EN-902"]
    );
}

#[test]
fn f48_ac2_fails_aloud_on_a_wrong_add_or_an_id_it_reads_nowhere() {
    let place = on_branch("wrong", "chore/en-900-wrong");

    fs::write(place.join("repo/empty.md"), "\n").unwrap();

    let failures = [
        run(&place, &["show", "EN-999"]),
        run(&place, &["show", "EN-999", "Tasks"]),
        run(&place, &["add", "Story", "empty.md"]),
        run(&place, &["add", "story", "missing.md"]),
        run(&place, &["add", "story", "empty.md"]),
        // `output` gives the child an empty standard input.
        run(&place, &["add", "journal", "-"]),
        run(&place, &["list", "--colour", "red"]),
        run(&place, &["sync", "--kind", "story"]),
    ]
    .map(|output| (output.status.code(), text(&output).1));

    fs::write(place.join("repo/story.md"), STORY).unwrap();
    run(&place, &["add", "story", "story.md"]);

    let no_heading = run(&place, &["show", "EN-900", "Missing"]);
    let usages = [
        run(&place, &["add", "story"]),
        run(&place, &["search"]),
        run(&place, &["search", "--kind", "story"]),
        run(&place, &["show"]),
    ]
    .map(|output| (output.status.code(), text(&output).1));

    git(&place.join("repo"), &["checkout", "-q", "--detach"]);

    let detached = run(&place, &["add", "story", "story.md"]);
    let detached_show = run(&place, &["show", "EN-999"]);

    // A repository without develop: sync cannot ask Git which branches are merged into it.
    git(&place.join("repo"), &["branch", "-m", "develop", "trunk"]);

    let no_develop = run(&place, &["sync"]);
    let missing = fs::read_to_string(place.join("repo/missing.md"))
        .unwrap_err()
        .to_string();

    let _ = fs::remove_dir_all(&place);

    assert_eq!(
        failures,
        [
            "knowledge: EN-999: no document of it read on chore/en-900-wrong".to_owned(),
            "knowledge: EN-999: no document of it read on chore/en-900-wrong".to_owned(),
            "knowledge: Story: a kind is one word of lower-case letters, as story or journal"
                .to_owned(),
            format!("knowledge: missing.md: {missing}"),
            "knowledge: empty.md: nothing to add".to_owned(),
            "knowledge: the standard input: nothing to add".to_owned(),
            "knowledge: --colour: no such filter".to_owned(),
            format!("knowledge: {USAGE}"),
        ]
        .map(|message| (Some(1), message))
    );
    assert_eq!(
        (no_heading.status.code(), text(&no_heading).1),
        (
            Some(1),
            "knowledge: EN-900: no heading Missing in it".to_owned()
        )
    );
    assert_eq!(
        usages,
        [(); 4].map(|()| (Some(1), format!("knowledge: {USAGE}")))
    );
    assert_eq!(
        (detached.status.code(), text(&detached).1),
        (
            Some(1),
            "knowledge: add writes on a branch: check one out".to_owned()
        )
    );
    assert_eq!(
        text(&detached_show).1,
        "knowledge: EN-999: no document of it read on a detached HEAD"
    );
    assert_eq!(no_develop.status.code(), Some(1));
    assert!(
        text(&no_develop).1.ends_with(
            "knowledge: git branch --merged develop --format=%(refname:short) failed in ."
        ),
        "{}",
        text(&no_develop).1
    );
}
