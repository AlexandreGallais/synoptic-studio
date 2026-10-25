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
const USAGE: &str = "Usage: target/release/knowledge init | path | add <kind> <file or -> | search <words> [<filters>] | show <id> [<heading>] [<filters>] | list [<filters>] | sync | snapshot | sql \"<statement>\" [--purpose \"<text>\"] | import-backlog <folder> | export-criteria <file or -> | check-ids <paths> | link-ids <paths> | branch-state <branch> merged|dropped | import-decisions, import-research, import-derivations, import-rules, import-logs, import-guide, import-memories, import-metrics, import-archives, import-transcripts <folder> | import-sources, import-library, import-saves <file>; the filters: --kind, --epic, --feature, --status, --since, --branch, --state <value>, --all";

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

/// The binary, run in a folder with `HOME` in the place and neither the records root nor the
/// synced folder inherited.
fn knowledge(place: &Path, cwd: &Path) -> Command {
    let mut command = test_support::command(env!("CARGO_BIN_EXE_knowledge"));

    command
        .current_dir(cwd)
        .env("HOME", place.join("home"))
        .env_remove("AGENT_RECORDS_ROOT")
        .env_remove("AGENT_KNOWLEDGE_SNAPSHOT");
    command
}

#[test]
fn f48_ac4_runs_the_binary_without_the_real_records_root_nor_the_real_synced_folder() {
    let place = place("variables");
    let command = knowledge(&place, &place.join("repo"));
    let removed: Vec<String> = command
        .get_envs()
        .filter(|(_, value)| value.is_none())
        .map(|(name, _)| name.to_string_lossy().into_owned())
        .filter(|name| name.starts_with("AGENT_"))
        .collect();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(removed, ["AGENT_KNOWLEDGE_SNAPSHOT", "AGENT_RECORDS_ROOT"]);
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
    let file = place.join("records/some-project.sqlite");
    let made = file.is_file();
    let (stdout, _) = text(&output);

    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success());
    assert!(made);
    assert_eq!(stdout, format!("{}, schema 5", file.display()));
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
            .join("records/some-project.sqlite")
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
    let file = place.join("records/some-project.sqlite");

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
            "{}, schema 6, newer than this binary's 5: read only",
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
    place.join("records/some-project.sqlite")
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
fn f48_ac2_adds_a_note_read_from_the_standard_input() {
    let place = on_branch("stdin", "chore/en-900-stdin");
    let mut child = knowledge(&place, &place.join("repo"))
        .args(["add", "note", "-"])
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
    let id = format!("NOTE-{}-1", calendar::today());
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

/// Merges a branch into the one checked out by a merge commit, as a pull request does.
fn merge(repo: &Path, branch: &str) {
    git(
        repo,
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.org",
            "-c",
            "commit.gpgsign=false",
            "merge",
            "-q",
            "--no-ff",
            "-m",
            "Merge",
            branch,
        ],
    );
}

#[test]
fn f48_ac2_marks_merged_the_rows_of_the_branches_merged_into_develop_and_no_other() {
    let place = on_branch("sync", "feature/a");
    let repo = place.join("repo");

    // feature/a's tip is develop's, no commit of its own: not merged. chore/c is one commit
    // ahead: not merged. chore/m's commit is merged into develop by a merge commit: merged.
    git(&repo, &["checkout", "-q", "-b", "chore/c"]);
    commit(&repo, "ahead");
    git(&repo, &["checkout", "-q", "-b", "chore/m", "develop"]);
    commit(&repo, "merged");
    git(&repo, &["checkout", "-q", "develop"]);
    merge(&repo, "chore/m");
    git(&repo, &["checkout", "-q", "chore/c"]);

    let database = open_store(&records(&place)).unwrap();

    for (id, branch) in [
        ("EN-901", "feature/a"),
        ("EN-902", "develop"),
        ("EN-903", "chore/c"),
        ("EN-905", "chore/m"),
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
    // develop's row and chore/m's; feature/a, at develop's start, merged nothing of its own.
    assert_eq!(
        text(&synced).0,
        "2 rows marked merged, of the 2 branches merged into develop"
    );
    assert_eq!(
        states,
        [
            "EN-901 draft",
            "EN-902 merged",
            "EN-903 draft",
            "EN-905 merged"
        ]
    );
    // The row added on develop; develop and chore/m merged, never chore/d, made from develop.
    assert_eq!(
        text(&one).0,
        "1 row marked merged, of the 2 branches merged into develop"
    );
    assert_eq!(text(&of_c).0.lines().count(), 3);
    // chore/d reads the merged rows, never chore/c's nor feature/a's draft.
    assert_eq!(
        text(&of_d)
            .0
            .lines()
            .map(|line| line.split(' ').next().unwrap_or_default())
            .collect::<Vec<_>>(),
        ["EN-902", "EN-905"]
    );
}

#[test]
fn f48_ac2_keeps_a_merged_row_read_by_every_branch_while_its_branch_writes_a_new_draft_of_it() {
    let place = on_branch("redraft", "story/b");
    let repo = place.join("repo");
    let adr = |title: &str| format!("---\nid: ADR-0900\n---\n# {title}\n");

    git(&repo, &["checkout", "-q", "develop"]);
    fs::write(repo.join("adr.md"), adr("First")).unwrap();
    run(&place, &["add", "adr", "adr.md"]);
    run(&place, &["sync"]);
    fs::write(repo.join("adr.md"), adr("Second")).unwrap();
    run(&place, &["add", "adr", "adr.md"]);

    let on_develop = run(&place, &["show", "ADR-0900"]);

    git(&repo, &["checkout", "-q", "story/b"]);

    let before = run(&place, &["show", "ADR-0900"]);

    git(&repo, &["checkout", "-q", "develop"]);
    run(&place, &["sync"]);
    git(&repo, &["checkout", "-q", "story/b"]);

    let after = run(&place, &["show", "ADR-0900"]);
    let rows: i64 = open_store(&records(&place))
        .unwrap()
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    // develop reads its new draft; story/b the merged row until that draft is merged.
    assert_eq!(text(&on_develop).0, "---\nid: ADR-0900\n---\n# Second");
    assert_eq!(
        (before.status.code(), text(&before).0),
        (Some(0), "---\nid: ADR-0900\n---\n# First".to_owned())
    );
    assert_eq!(text(&after).0, "---\nid: ADR-0900\n---\n# Second");
    assert_eq!(rows, 1);
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
        run(&place, &["add", "note", "-"]),
        // A journal row, an incident, an improvement or a feedback is a row of its table.
        run(&place, &["add", "incident", "-"]),
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
        run(&place, &["show", "EN-900", "Tasks", "more"]),
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
            "knowledge: Story: a kind is one word of lower-case letters, as story or note"
                .to_owned(),
            format!("knowledge: missing.md: {missing}"),
            "knowledge: empty.md: nothing to add".to_owned(),
            "knowledge: the standard input: nothing to add".to_owned(),
            "knowledge: incident: a row of the table incidents, with its columns, not a document: target/release/knowledge sql \"INSERT INTO incidents (…) VALUES (…)\"".to_owned(),
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
        [(); 5].map(|()| (Some(1), format!("knowledge: {USAGE}")))
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
            "knowledge: git branch --merged develop --format=%(refname:short) %(objectname) failed in ."
        ),
        "{}",
        text(&no_develop).1
    );
}

#[test]
fn f48_ac1_names_the_file_after_the_main_checkouts_folder_without_a_remote() {
    let place = place("no-remote");
    let repo = place.join("repo");

    git(&repo, &["remote", "remove", "origin"]);

    let output = knowledge(&place, &repo)
        .arg("path")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        text(&output).0,
        place.join("records/repo.sqlite").display().to_string()
    );
}

/// A backlog of one epic, one done feature of two criteria and one story, under `repo/backlog`.
fn backlog(place: &Path) {
    for (path, text) in [
        (
            "backlog/epics/E49-keep.md",
            "---\nid: E49\ntitle: Keep\nstatus: in-progress\n---\n\n# E49\n",
        ),
        (
            "backlog/features/E49-F48-the-database.md",
            "---\nid: F48\nepic: E49\ntitle: The database\nstatus: done\n---\n\n## Acceptance criteria\n\n1. One file.\n2. A snapshot.\n",
        ),
        (
            "backlog/stories/E49-F48-EN-151-the-tables.md",
            "---\nid: EN-151\nepic: E49\nfeature: F48\ntitle: The tables\nstatus: done\n---\n\n# EN-151\n",
        ),
    ] {
        let file = place.join("repo").join(path);

        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, text).unwrap();
    }
}

#[test]
fn imports_the_backlog_reads_and_writes_it_by_sql_and_exports_its_criteria() {
    let place = on_branch("backlog", "chore/en-154-x");

    backlog(&place);

    let imported = run(&place, &["import-backlog", "backlog"]);
    let not_a_backlog = run(&place, &["import-backlog", "."]);
    let read = run(&place, &["sql", "SELECT id, status FROM stories"]);
    let created = run(
        &place,
        &[
            "sql",
            "CREATE TABLE ideas (id TEXT, branch TEXT NOT NULL DEFAULT '', state TEXT NOT NULL DEFAULT 'draft')",
            "--purpose",
            "a note",
        ],
    );
    let inserted = run(&place, &["sql", "INSERT INTO ideas (id) VALUES ('a')"]);
    let refused = run(&place, &["sql", "DROP TABLE stories"]);
    let purpose_on_a_reading = run(&place, &["sql", "SELECT 1", "--purpose", "x"]);
    let wrong_flag = run(&place, &["sql", "SELECT 1", "--kind", "x"]);
    let merged = run(&place, &["branch-state", "chore/en-154-x", "merged"]);
    let dropped = run(&place, &["branch-state", "chore/en-154-x", "dropped"]);
    let wrong_state = run(&place, &["branch-state", "chore/en-154-x", "gone"]);
    let notes = run(&place, &["sql", "SELECT id, branch, state FROM ideas"]);
    let printed = run(&place, &["export-criteria", "-"]);
    let written = run(&place, &["export-criteria", "criteria.json"]);
    let file = fs::read_to_string(place.join("repo/criteria.json")).unwrap_or_default();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        text(&imported).0,
        "1 epics, 1 features, 1 stories, 2 criteria and 0 tasks imported"
    );
    assert_eq!(
        text(&not_a_backlog),
        (
            String::new(),
            "knowledge: .: no stories folder, not a backlog".to_owned()
        )
    );
    assert_eq!(text(&read).0, "id\tstatus\nEN-151\tdone");
    assert_eq!(text(&created).0, "0 rows changed, ideas registered");
    assert_eq!(text(&inserted).0, "1 row changed");
    assert_eq!(
        text(&refused).1,
        "knowledge: refused: DROP TABLE stories: a core table of the records, kept by the tool"
    );
    assert_eq!(
        text(&purpose_on_a_reading).1,
        "knowledge: --purpose names a table a statement creates"
    );
    assert_eq!(text(&wrong_flag).1, format!("knowledge: {USAGE}"));
    // The note's draft merged; dropping the branch after leaves its merged rows alone.
    assert_eq!(text(&merged).0, "1 row of chore/en-154-x marked merged");
    assert_eq!(text(&dropped).0, "0 rows of chore/en-154-x marked dropped");
    assert_eq!(text(&wrong_state).1, format!("knowledge: {USAGE}"));
    assert_eq!(
        text(&notes).0,
        "id\tbranch\tstate\na\tchore/en-154-x\tmerged"
    );
    assert_eq!(
        text(&printed).0,
        "[\n  { \"id\": \"F48.AC1\" },\n  { \"id\": \"F48.AC2\" }\n]"
    );
    assert_eq!(text(&written).0, "criteria.json written: 2 criteria");
    assert_eq!(file, format!("{}\n", text(&printed).0));
}

#[test]
fn checks_the_ids_a_page_cites_and_writes_its_links_to_backlog_files_as_ids() {
    let place = on_branch("ids", "chore/en-154-y");

    backlog(&place);
    run(&place, &["import-backlog", "backlog"]);
    fs::create_dir_all(place.join("repo/docs")).unwrap();
    fs::write(
        place.join("repo/docs/page.md"),
        "[EN-151](../backlog/stories/E49-F48-EN-151-the-tables.md) and EN-999.\n",
    )
    .unwrap();
    // A test's made-up id is not read.
    fs::write(place.join("repo/docs/page.test.ts"), "// EN-998\n").unwrap();

    let missing = run(&place, &["check-ids", "docs"]);
    let linked = run(&place, &["link-ids", "docs"]);
    let page = fs::read_to_string(place.join("repo/docs/page.md")).unwrap();

    fs::write(place.join("repo/docs/page.md"), "EN-151 and F48.\n").unwrap();

    let found = run(&place, &["check-ids", "docs"]);
    let nowhere = run(&place, &["check-ids", "none"]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(missing.status.code(), Some(1));
    assert_eq!(
        text(&missing).1,
        "knowledge: docs/page.md:1: EN-999 is not in the records"
    );
    assert_eq!(text(&linked).0, "1 file written with ids");
    assert_eq!(page, "EN-151 and EN-999.\n");
    assert_eq!(
        text(&found).0,
        "every id cited is in the records, 1 file read"
    );
    assert_eq!(text(&nowhere).1, "knowledge: none: no such file or folder");
}

#[test]
fn imports_what_moved_by_its_commands_a_missing_source_refused() {
    let place = on_branch("imports", "chore/en-155-x");
    let repo = place.join("repo");

    fs::create_dir_all(repo.join("adr")).unwrap();
    fs::write(
        repo.join("adr/0001-scope.md"),
        "# ADR-0001 — Scope\n\n**Status**: Accepted (2026-10-08).\n\n## Decision\n\nGeometry only.\n",
    )
    .unwrap();
    fs::write(repo.join("last-save.txt"), "2026-10-25T11:32:57Z\n").unwrap();

    let imported = run(&place, &["import-decisions", "adr"]);
    let again = run(&place, &["import-decisions", "adr"]);
    let saves = run(&place, &["import-saves", "last-save.txt"]);
    let shown = run(&place, &["show", "ADR-0001", "Decision"]);
    let no_folder = run(&place, &["import-guide", "nowhere"]);
    let no_file = run(&place, &["import-sources", "adr"]);
    let itself = run(
        &place,
        &["import-library", "../records/some-project.sqlite"],
    );
    let two_words = run(&place, &["import-rules", "a", "b"]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(text(&imported).0, "1 row and 1 document imported from adr");
    assert_eq!(text(&again), text(&imported));
    assert_eq!(
        text(&saves).0,
        "1 row and 0 documents imported from last-save.txt"
    );
    assert_eq!(text(&shown).0, "ADR-0001:5\n## Decision\n\nGeometry only.");
    assert_eq!(text(&no_folder).1, "knowledge: nowhere: no such folder");
    assert_eq!(text(&no_file).1, "knowledge: adr: no such file");
    assert_eq!(
        text(&itself).1,
        "knowledge: ../records/some-project.sqlite: the records themselves, nothing to import"
    );
    assert_eq!(text(&two_words).1, format!("knowledge: {USAGE}"));
}
