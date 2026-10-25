//! Tests of `sync` (EN-152): the branches a merge commit brought into `develop`, and their drafts
//! of documents marked merged, the memories' documents of the schemas before 6. Every store is made
//! in a folder of the system's temporary folder; the real records folder is never opened.

use std::fs;
use std::path::{Path, PathBuf};

use knowledge::store::{open_store, write};
use knowledge::sync::{mark_merged, merged_branches};
use rusqlite::{Connection, params};

/// A store of its own in the temporary folder, emptied first.
fn place(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("knowledge-sync-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    folder.join("records.sqlite")
}

/// Removes a test's folder.
fn clean(file: &Path) {
    let _ = fs::remove_dir_all(file.parent().unwrap());
}

/// Writes a document of an id, a title, a branch and a state, as the tool wrote one.
fn document(database: &Connection, (id, title): (&str, &str), branch: &str, state: &str) {
    database
        .execute(
            "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
            VALUES (?, 'memory', ?, '', '', '', '2026-10-24', ?, ?, 1, '')",
            params![id, title, branch, state],
        )
        .unwrap();
}

/// The documents' rows, `(id, title, branch, state)`, by id and branch.
fn rows(database: &Connection) -> Vec<(String, String, String, String)> {
    database
        .prepare("SELECT id, title, branch, state FROM documents ORDER BY id, branch")
        .unwrap()
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap()
}

#[test]
fn f48_ac2_marks_merged_the_drafts_of_the_merged_branches_each_replacing_its_merged_row() {
    let file = place("sync");
    let database = open_store(&file).unwrap();

    document(&database, ("MEMORY-a", "Merged"), "feature/a", "merged");
    document(&database, ("MEMORY-a", "Changed on b"), "chore/b", "draft");
    document(&database, ("MEMORY-b", "Of b"), "chore/b", "draft");
    document(&database, ("MEMORY-c", "Of c"), "chore/c", "draft");
    document(&database, ("MEMORY-d", "Of d"), "chore/d", "dropped");

    let marked = write(&database, |transaction| {
        mark_merged(transaction, &["chore/b".to_owned(), "develop".to_owned()])
    })
    .unwrap();
    let again = write(&database, |transaction| {
        mark_merged(transaction, &["chore/b".to_owned()])
    })
    .unwrap();
    let after = rows(&database);

    clean(&file);
    // chore/b's two drafts; feature/a's merged MEMORY-a replaced by chore/b's; the dropped row kept.
    assert_eq!(marked, 2);
    assert_eq!(again, 0);
    assert_eq!(
        after,
        [
            ("MEMORY-a", "Changed on b", "chore/b", "merged"),
            ("MEMORY-b", "Of b", "chore/b", "merged"),
            ("MEMORY-c", "Of c", "chore/c", "draft"),
            ("MEMORY-d", "Of d", "chore/d", "dropped"),
        ]
        .map(|(id, title, branch, state)| {
            (
                id.to_owned(),
                title.to_owned(),
                branch.to_owned(),
                state.to_owned(),
            )
        })
    );
}

#[test]
fn f48_ac2_keeps_the_draft_written_last_of_two_merged_at_once() {
    let file = place("two-drafts");
    let database = open_store(&file).unwrap();

    document(&database, ("MEMORY-e", "Written first"), "chore/c", "draft");
    document(&database, ("MEMORY-e", "Written last"), "chore/e", "draft");

    // Given in either order, the row written last holds.
    let marked = write(&database, |transaction| {
        mark_merged(transaction, &["chore/e".to_owned(), "chore/c".to_owned()])
    })
    .unwrap();
    let after = rows(&database);

    clean(&file);
    assert_eq!(marked, 2);
    assert_eq!(
        after,
        [(
            "MEMORY-e".to_owned(),
            "Written last".to_owned(),
            "chore/e".to_owned(),
            "merged".to_owned()
        )]
    );
}

#[test]
fn f48_ac2_reads_as_merged_develop_and_the_branches_a_merge_commit_brought_into_it() {
    // The tips `git branch --merged develop` lists, then the parents of develop's merge commits:
    // feature/f48-x's tip and chore/en-2's are a merge's second or third parent, chore/en-1's is
    // reachable but no merge brought it, chore/en-3's is a first parent only, as a branch made at
    // develop's tip without a commit of its own.
    assert_eq!(
        merged_branches(
            "develop 1111\n(HEAD detached at 1a2b3c4) 3333\n\n  feature/f48-x 3333  \nchore/en-1 4444\nchore/en-2 6666\nchore/en-3 0000\n",
            "0000 3333\n\n5555 7777 6666\n"
        ),
        ["develop", "feature/f48-x", "chore/en-2"]
    );
    assert_eq!(merged_branches("", ""), Vec::<String>::new());
    // develop is merged into itself, its rows the next sync's, whatever its merges.
    assert_eq!(merged_branches("develop 1111\n", ""), ["develop"]);
}
