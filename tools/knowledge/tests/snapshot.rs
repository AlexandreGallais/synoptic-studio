//! Tests of the snapshot (EN-153): its names, the seven days kept, the synced folder read from its
//! variable and refused around the working file, and `knowledge snapshot` as a binary while another
//! connection writes. Every folder is made in the system's temporary folder, the synced one too:
//! `AGENT_RECORDS_ROOT` and `AGENT_KNOWLEDGE_SNAPSHOT` are always set to such a folder or removed,
//! and `HOME` pointed there, so that neither the real records nor the real synced folder is touched
//! (`[F48.AC4]`).

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, SystemTime};

use knowledge::add::{Origin, add_document};
use knowledge::snapshot::{
    KEPT, NOT_SET, is_snapshot_name, is_temporary_name, refuse_synced, removed_snapshots,
    snapshot_folder, snapshot_name, temporary_name,
};
use knowledge::store::{SCHEMA_VERSION, open_store, write};
use rusqlite::Connection;

/// What the snapshot says of a synced folder that does not exist.
const MISSING: &str = "AGENT_KNOWLEDGE_SNAPSHOT names a folder that does not exist: create it, or correct the variable; nothing copied.";

/// What the tool says of a working file in the synced folder itself.
const IN_THE_FOLDER: &str = "the records database lies in the folder AGENT_KNOWLEDGE_SNAPSHOT names, which a sync service copies mid-write: move AGENT_RECORDS_ROOT out of it, or name another synced folder.";

/// What the tool says of a working file in a folder inside the synced one.
const BELOW_THE_FOLDER: &str = "the records database lies in a folder inside the one AGENT_KNOWLEDGE_SNAPSHOT names, which a sync service copies mid-write: move AGENT_RECORDS_ROOT out of it, or name another synced folder.";

/// Some names, as owned strings.
fn names(texts: &[&str]) -> Vec<String> {
    texts.iter().map(|text| (*text).to_owned()).collect()
}

#[test]
fn f48_ac3_names_the_days_snapshot_and_its_temporary_file() {
    assert_eq!(snapshot_name("2026-10-25"), "records-2026-10-25.sqlite");
    assert_eq!(temporary_name(42), ".records-42.sqlite.tmp");
    assert!(is_snapshot_name("records-2026-10-25.sqlite"));
    for other in [
        "records.sqlite",
        "records.sqlite-wal",
        "records-2026-10-25.sqlite-wal",
        "records-2026-10-25.sqlite.tmp",
        "records-2026-1-25.sqlite",
        "records-2026-10-250.sqlite",
        "records-abcd-ef-gh.sqlite",
        "records-2026_10_25.sqlite",
        "Records-2026-10-25.sqlite",
        ".records-42.sqlite.tmp",
        "notes.txt",
        "",
    ] {
        assert!(!is_snapshot_name(other), "{other}");
    }
    assert!(is_temporary_name(".records-42.sqlite.tmp"));
    for other in [
        ".records-.sqlite.tmp",
        ".records-4a.sqlite.tmp",
        "records-42.sqlite.tmp",
        ".records-42.sqlite",
        "records-2026-10-25.sqlite",
    ] {
        assert!(!is_temporary_name(other), "{other}");
    }
}

#[test]
fn f48_ac3_removes_the_oldest_beyond_the_seven_kept() {
    // Eight days, 18 to 25 October, given out of order with a file of another name: the 18th, the
    // oldest, goes; 8 − 7 = 1.
    let eight = names(&[
        "records-2026-10-21.sqlite",
        "records-2026-10-25.sqlite",
        "notes.txt",
        "records-2026-10-18.sqlite",
        "records-2026-10-24.sqlite",
        "records-2026-10-19.sqlite",
        "records-2026-10-23.sqlite",
        "records-2026-10-20.sqlite",
        "records-2026-10-22.sqlite",
    ]);

    assert_eq!(KEPT, 7);
    assert_eq!(
        removed_snapshots(&eight, KEPT),
        ["records-2026-10-18.sqlite"]
    );
    // Seven, or fewer: none goes.
    assert_eq!(
        removed_snapshots(&eight[1..], KEPT),
        [] as [std::string::String; 0]
    );
    assert_eq!(
        removed_snapshots(&names(&["records-2026-10-25.sqlite"]), KEPT),
        [] as [std::string::String; 0]
    );
    assert_eq!(removed_snapshots(&[], KEPT), [] as [std::string::String; 0]);
    // Two days kept of four across a year's end: the two oldest, oldest first.
    assert_eq!(
        removed_snapshots(
            &names(&[
                "records-2027-01-01.sqlite",
                "records-2026-12-30.sqlite",
                "records-2026-12-31.sqlite",
                "records-2026-12-29.sqlite",
            ]),
            2
        ),
        ["records-2026-12-29.sqlite", "records-2026-12-30.sqlite"]
    );
    // None kept: every snapshot goes, never another file.
    assert_eq!(
        removed_snapshots(&names(&["notes.txt", "records-2026-10-25.sqlite"]), 0),
        ["records-2026-10-25.sqlite"]
    );
}

/// A folder of its own in the temporary folder, emptied first, its links resolved.
fn folder(name: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("knowledge-snapshot-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    fs::create_dir_all(&folder).unwrap();
    fs::canonicalize(folder).unwrap()
}

#[test]
fn f48_ac3_reads_the_synced_folder_from_its_variable_never_making_it() {
    let place = folder("setting");
    let synced = place.join("synced");
    let file = place.join("a-file");

    fs::create_dir_all(&synced).unwrap();
    fs::write(&file, "").unwrap();

    let unset = snapshot_folder(None);
    let empty = snapshot_folder(Some(""));
    let set = snapshot_folder(synced.to_str());
    let missing = snapshot_folder(place.join("mistyped").to_str());
    let not_a_folder = snapshot_folder(file.to_str());
    let made = place.join("mistyped").exists();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(unset, Ok(None));
    assert_eq!(empty, Ok(None));
    assert_eq!(set, Ok(Some(synced)));
    assert_eq!(missing, Err(MISSING.to_owned()));
    assert_eq!(not_a_folder, Err(MISSING.to_owned()));
    assert!(!made);
}

#[test]
fn f48_ac3_refuses_a_working_file_in_the_synced_folder_or_below_it() {
    let place = folder("refused");
    let synced = place.join("synced");
    let link = place.join("link");

    fs::create_dir_all(synced.join("records/some-project")).unwrap();
    fs::create_dir_all(place.join("synced-2")).unwrap();
    fs::create_dir_all(place.join("records")).unwrap();
    symlink(&synced, &link).unwrap();

    let in_it = refuse_synced(&synced.join("records.sqlite"), &synced);
    let below = refuse_synced(&synced.join("records/some-project/records.sqlite"), &synced);
    // A project folder not made yet: the part that does not exist is read as written.
    let not_made = refuse_synced(&synced.join("records/new-project/records.sqlite"), &synced);
    // The synced folder named by a link, or the file reached through one.
    let by_link = refuse_synced(&synced.join("records.sqlite"), &link);
    let through_link = refuse_synced(&link.join("records/some-project/records.sqlite"), &synced);
    // A neighbour whose name starts with the synced folder's, the synced folder inside the
    // records, and a folder apart: none is inside.
    let neighbour = refuse_synced(&place.join("synced-2/records.sqlite"), &synced);
    let records = place.join("records");
    let around = refuse_synced(&records.join("records.sqlite"), &records.join("snapshots"));
    let apart = refuse_synced(&records.join("records.sqlite"), &synced);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(in_it, Err(IN_THE_FOLDER.to_owned()));
    assert_eq!(below, Err(BELOW_THE_FOLDER.to_owned()));
    assert_eq!(not_made, Err(BELOW_THE_FOLDER.to_owned()));
    assert_eq!(by_link, Err(IN_THE_FOLDER.to_owned()));
    assert_eq!(through_link, Err(BELOW_THE_FOLDER.to_owned()));
    assert_eq!(neighbour, Ok(()));
    assert_eq!(around, Ok(()));
    assert_eq!(apart, Ok(()));
}

/// A place for the binary: `records/` for the root, `synced/` for the synced folder, `repo/` a
/// repository whose `origin` names `some-project`, `home/` for `HOME`.
fn place(name: &str) -> PathBuf {
    let place = folder(&format!("binary-{name}"));

    for part in ["records", "synced", "home", "repo"] {
        fs::create_dir_all(place.join(part)).unwrap();
    }
    git(&place.join("repo"), &["init", "-q", "-b", "develop"]);
    git(
        &place.join("repo"),
        &[
            "remote",
            "add",
            "origin",
            "https://example.org/owner/some-project.git",
        ],
    );
    place
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

/// The binary in the place's repository, `HOME` in the place, neither variable inherited.
fn knowledge(place: &Path) -> Command {
    let mut command = test_support::command(env!("CARGO_BIN_EXE_knowledge"));

    command
        .current_dir(place.join("repo"))
        .env("HOME", place.join("home"))
        .env_remove("AGENT_RECORDS_ROOT")
        .env_remove("AGENT_KNOWLEDGE_SNAPSHOT");
    command
}

/// `knowledge snapshot`, the records root and the synced folder the place's.
fn snapshot(place: &Path) -> Output {
    knowledge(place)
        .arg("snapshot")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .env("AGENT_KNOWLEDGE_SNAPSHOT", place.join("synced"))
        .output()
        .unwrap()
}

/// The output's standard output and error, as text.
fn text(output: &Output) -> (String, String) {
    (
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    )
}

/// The working file of the place.
fn working(place: &Path) -> PathBuf {
    place.join("records/some-project/records.sqlite")
}

/// Adds a story of an id to a store, committed.
fn add(database: &Connection, id: &str) {
    write(database, |transaction| {
        add_document(
            transaction,
            &format!("---\nid: {id}\ntitle: A story\n---\n\n# {id}\n\nSome text.\n"),
            Origin {
                kind: "story",
                path: "",
                branch: "develop",
                today: "2026-10-25",
            },
        )
    })
    .unwrap();
}

/// The names of a folder's files, sorted.
fn listed(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();

    names.sort();
    names
}

/// A text value a query returns.
fn read_text(database: &Connection, query: &str) -> String {
    database.query_row(query, [], |row| row.get(0)).unwrap()
}

#[test]
fn f48_ac3_snapshots_every_committed_row_while_another_connection_writes() {
    let place = place("writer");
    let database = open_store(&working(&place)).unwrap();

    add(&database, "EN-1");
    add(&database, "EN-2");

    // Another connection holds the write lock, a row written and not committed.
    let writer = open_store(&working(&place)).unwrap();

    writer.execute_batch("BEGIN IMMEDIATE").unwrap();
    writer
        .execute(
            "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
            VALUES ('EN-3', 'story', 'Not committed', '', '', '', '2026-10-25', 'develop', 'draft', 0, '')",
            [],
        )
        .unwrap();

    let output = snapshot(&place);
    let day = snapshot_name(&calendar::today());

    writer.execute_batch("ROLLBACK").unwrap();

    let synced = place.join("synced");
    let files = listed(&synced);
    let copy = Connection::open(synced.join(&day)).unwrap();
    let ids: Vec<String> = copy
        .prepare("SELECT id FROM documents ORDER BY id")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    let sections: i64 = copy
        .query_row(
            "SELECT count(*) FROM sections_index WHERE sections_index MATCH 'text'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let integrity = read_text(&copy, "PRAGMA integrity_check");
    let mode = read_text(&copy, "PRAGMA journal_mode");
    let version: i64 = copy
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap();
    let after_reading = listed(&synced);
    let (stdout, stderr) = text(&output);

    drop(copy);
    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success(), "{stderr}");
    assert_eq!(
        stdout,
        format!("{day} written into the folder AGENT_KNOWLEDGE_SNAPSHOT names: 1 kept, 0 removed")
    );
    assert!(!stdout.contains(&place.display().to_string()));
    // The day's file alone: neither the working file, nor its `-wal`, nor the temporary one.
    assert_eq!(files, std::slice::from_ref(&day));
    assert_eq!(ids, ["EN-1", "EN-2"]);
    // The full-text index came with its sections: each story's one section holds "text".
    assert_eq!(sections, 2);
    assert_eq!(integrity, "ok");
    assert_eq!(version, SCHEMA_VERSION);
    // In the rollback journal: reading the copy leaves no `-wal` nor `-shm` in the synced folder.
    assert_eq!(mode, "delete");
    assert_eq!(after_reading, [day]);
}

#[test]
fn f48_ac3_replaces_the_days_snapshot_when_it_runs_again() {
    let place = place("again");
    let database = open_store(&working(&place)).unwrap();

    add(&database, "EN-1");

    let first = snapshot(&place);

    add(&database, "EN-2");

    let second = snapshot(&place);
    let day = snapshot_name(&calendar::today());
    let files = listed(&place.join("synced"));
    let count: i64 = Connection::open(place.join("synced").join(&day))
        .unwrap()
        .query_row("SELECT count(*) FROM documents", [], |row| row.get(0))
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert!(first.status.success());
    assert!(second.status.success());
    assert_eq!(
        text(&second).0,
        format!("{day} written into the folder AGENT_KNOWLEDGE_SNAPSHOT names: 1 kept, 0 removed")
    );
    assert_eq!(files, [day]);
    assert_eq!(count, 2);
}

/// Writes a file of the synced folder, its modification time set some seconds back.
fn written(folder: &Path, name: &str, age: u64) {
    let path = folder.join(name);

    fs::write(&path, "not a database").unwrap();
    fs::File::options()
        .write(true)
        .open(&path)
        .unwrap()
        .set_modified(SystemTime::now() - Duration::from_secs(age))
        .unwrap();
}

#[test]
fn f48_ac3_keeps_the_seven_newest_days_and_removes_a_killed_copys_leftover() {
    let place = place("seven");
    let synced = place.join("synced");

    open_store(&working(&place)).unwrap();
    // Seven older days, then today's: eight, the oldest removed. A file of another name stays; a
    // temporary file two hours old is a killed copy's leftover, one a minute old a live copy's.
    for day in 1..=7 {
        written(&synced, &format!("records-2000-01-0{day}.sqlite"), 0);
    }
    written(&synced, "notes.txt", 7_200);
    written(&synced, ".records-1.sqlite.tmp", 7_200);
    written(&synced, ".records-2.sqlite.tmp", 60);

    let output = snapshot(&place);
    let day = snapshot_name(&calendar::today());
    let files = listed(&synced);

    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success(), "{}", text(&output).1);
    assert_eq!(
        text(&output).0,
        format!("{day} written into the folder AGENT_KNOWLEDGE_SNAPSHOT names: 7 kept, 1 removed")
    );
    assert_eq!(
        files,
        [
            ".records-2.sqlite.tmp".to_owned(),
            "notes.txt".to_owned(),
            "records-2000-01-02.sqlite".to_owned(),
            "records-2000-01-03.sqlite".to_owned(),
            "records-2000-01-04.sqlite".to_owned(),
            "records-2000-01-05.sqlite".to_owned(),
            "records-2000-01-06.sqlite".to_owned(),
            "records-2000-01-07.sqlite".to_owned(),
            day,
        ]
    );
}

#[test]
fn f48_ac3_copies_nothing_without_the_setting_and_does_not_fail() {
    let place = place("unset");

    open_store(&working(&place)).unwrap();

    let unset = knowledge(&place)
        .arg("snapshot")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();
    let empty = knowledge(&place)
        .arg("snapshot")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .env("AGENT_KNOWLEDGE_SNAPSHOT", "")
        .output()
        .unwrap();
    // Neither variable set, as on a machine without the records: the same, no failure.
    let neither = knowledge(&place).arg("snapshot").output().unwrap();
    let synced = listed(&place.join("synced"));
    let records = listed(&place.join("records/some-project"));

    let _ = fs::remove_dir_all(&place);
    for output in [unset, empty, neither] {
        assert!(output.status.success());
        assert_eq!(text(&output), (NOT_SET.to_owned(), String::new()));
    }
    assert_eq!(synced, [] as [std::string::String; 0]);
    assert_eq!(records, ["records.sqlite"]);
}

#[test]
fn f48_ac3_fails_on_a_synced_folder_that_does_not_exist_and_makes_none() {
    let place = place("missing");
    let mistyped = place.join("mistyped");

    open_store(&working(&place)).unwrap();

    let output = knowledge(&place)
        .arg("snapshot")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .env("AGENT_KNOWLEDGE_SNAPSHOT", &mistyped)
        .output()
        .unwrap();
    let made = mistyped.exists();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(text(&output).1, format!("knowledge: {MISSING}"));
    assert!(!made);
}

#[test]
fn f48_ac3_says_so_when_there_is_no_working_file_yet() {
    let place = place("no-file");
    let output = snapshot(&place);
    let synced = listed(&place.join("synced"));
    let made = working(&place).exists();

    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success());
    assert_eq!(
        text(&output).0,
        "no records.sqlite in the records folder yet: nothing copied."
    );
    assert_eq!(synced, [] as [std::string::String; 0]);
    assert!(!made);
}

#[test]
fn f48_ac3_refuses_to_open_a_working_file_inside_the_synced_folder() {
    let place = place("inside");
    let synced = place.join("synced");
    let root = synced.join("records");

    fs::create_dir_all(&root).unwrap();

    let run = |words: &[&str]| {
        knowledge(&place)
            .args(words)
            .env("AGENT_RECORDS_ROOT", &root)
            .env("AGENT_KNOWLEDGE_SNAPSHOT", &synced)
            .output()
            .unwrap()
    };
    let opened = run(&["init"]);
    let listed_rows = run(&["list"]);
    let snapshot = run(&["snapshot"]);
    // `path` opens nothing: it still prints where the file is.
    let path = run(&["path"]);
    // The project's folder itself synced.
    let project = knowledge(&place)
        .arg("init")
        .env("AGENT_RECORDS_ROOT", &root)
        .env("AGENT_KNOWLEDGE_SNAPSHOT", root.join("some-project"))
        .output();
    let made = root.join("some-project/records.sqlite").exists();
    let in_synced = listed(&synced);

    let _ = fs::remove_dir_all(&place);
    for output in [opened, listed_rows, snapshot] {
        let (_, stderr) = text(&output);

        assert_eq!(output.status.code(), Some(1));
        assert_eq!(stderr, format!("knowledge: {BELOW_THE_FOLDER}"));
        assert!(!stderr.contains(&place.display().to_string()));
    }
    assert!(path.status.success());
    let project = project.unwrap();
    assert_eq!(project.status.code(), Some(1));
    assert_eq!(text(&project).1, format!("knowledge: {IN_THE_FOLDER}"));
    assert!(!made);
    assert_eq!(in_synced, ["records"]);
}
