//! Tests of `knowledge sql` (EN-154): a reading's rows, a write in a transaction, a new table
//! registered with its purpose, a row without a branch given the branch checked out, and what is
//! refused: a core table dropped or rewritten, the documents written, a pragma set, a
//! transaction, an attached file. Every file is made in a folder of the temporary folder.

use std::fs;
use std::path::PathBuf;

use knowledge::sql::{is_added_column, is_reading, read, unescaped, write};
use knowledge::store::{CORE_TABLES, open_store, write_or_refuse};
use rusqlite::Connection;

/// A store in a folder of its own.
fn place(name: &str) -> (PathBuf, Connection) {
    let folder = std::env::temp_dir().join(format!("knowledge-sql-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);
    let database = open_store(&folder.join("records.sqlite")).unwrap();

    (folder, database)
}

/// A statement run as `knowledge sql` runs it, on a branch.
fn run(database: &Connection, statement: &str, purpose: Option<&str>) -> Result<String, String> {
    if is_reading(database, statement)? {
        return read(database, statement);
    }
    write_or_refuse(database, |transaction| {
        write(transaction, statement, purpose, "chore/en-154-x")
    })
}

#[test]
fn reads_rows_with_their_column_names_and_escapes_tabs_and_line_ends() {
    let (folder, database) = place("read");
    let rows = run(
        &database,
        "SELECT 1 AS one, 'a\tb' AS two, NULL AS three, 'x\ny\\z' AS four",
        None,
    );

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        rows.as_deref(),
        Ok("one\ttwo\tthree\tfour\n1\ta\\tb\t\tx\\ny\\\\z\n")
    );
    assert_eq!(unescaped("a\\tb\\ny\\\\z"), "a\tb\ny\\z");
}

#[test]
fn registers_a_new_table_with_its_purpose_and_refuses_one_without() {
    let (folder, database) = place("create");
    let without = run(&database, "CREATE TABLE ideas (id TEXT PRIMARY KEY)", None);
    let made = run(
        &database,
        "CREATE TABLE ideas (id TEXT PRIMARY KEY, branch TEXT NOT NULL DEFAULT '', state TEXT NOT NULL DEFAULT 'draft')",
        Some("one row per idea"),
    );
    let registered = run(
        &database,
        "SELECT purpose FROM tables_registry WHERE name = 'ideas'",
        None,
    );
    let inserted = run(
        &database,
        "INSERT INTO ideas (id) VALUES ('a'), ('b')",
        None,
    );
    let branch = run(
        &database,
        "SELECT id, branch, state FROM ideas ORDER BY id",
        None,
    );
    let index = run(
        &database,
        "CREATE INDEX notes_by_state ON ideas (state)",
        None,
    );
    let dropped = run(&database, "DROP TABLE ideas", None);
    let gone = run(
        &database,
        "SELECT count(*) AS n FROM tables_registry WHERE name = 'ideas'",
        None,
    );

    let _ = fs::remove_dir_all(&folder);
    assert!(without.is_err_and(|error| error.contains("--purpose")));
    assert_eq!(made.as_deref(), Ok("0 rows changed, ideas registered\n"));
    assert_eq!(registered.as_deref(), Ok("purpose\none row per idea\n"));
    assert_eq!(inserted.as_deref(), Ok("2 rows changed\n"));
    // A row written without a branch takes the branch checked out.
    assert_eq!(
        branch.as_deref(),
        Ok("id\tbranch\tstate\na\tchore/en-154-x\tdraft\nb\tchore/en-154-x\tdraft\n")
    );
    assert!(index.is_ok());
    // The agent's own table may be dropped; its registry row goes with it.
    assert!(dropped.is_ok());
    assert_eq!(gone.as_deref(), Ok("n\n0\n"));
}

#[test]
fn lists_every_table_of_a_new_file_in_the_registry() {
    let (folder, database) = place("registry");
    let unlisted = run(
        &database,
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'sections_index_%' AND name NOT LIKE 'chunks_%' AND name NOT IN (SELECT name FROM tables_registry)",
        None,
    );

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(unlisted.as_deref(), Ok("name\n"));
}

#[test]
fn writes_the_backlogs_tables_and_refuses_to_drop_or_rewrite_any_core_table() {
    let (folder, database) = place("core");
    let story = run(
        &database,
        "INSERT INTO stories (id, epic, kind, title, status) VALUES ('EN-160', 'E49', 'EN', 'New', 'ready')",
        None,
    );
    let updated = run(
        &database,
        "UPDATE stories SET status = 'parked' WHERE id = 'EN-160'",
        None,
    );
    let column = run(
        &database,
        "ALTER TABLE stories ADD COLUMN risk TEXT NOT NULL DEFAULT ''",
        None,
    );
    let refusals: Vec<String> = CORE_TABLES
        .iter()
        .map(|table| run(&database, &format!("DROP TABLE {table}"), None).unwrap_err())
        .collect();
    let renamed = run(&database, "ALTER TABLE stories RENAME TO tales", None);
    let dropped_column = run(&database, "ALTER TABLE stories DROP COLUMN risk", None);
    let dropped_index = run(&database, "DROP INDEX stories_by_status", None);
    let documents = [
        "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path) VALUES ('X', 'a', 'a', '', '', '', '', 'b', 'draft', 0, '')",
        "UPDATE sections SET text = ''",
        "DELETE FROM links",
        "DELETE FROM sections_index_data",
    ]
    .map(|statement| run(&database, statement, None));
    let others = [
        "PRAGMA user_version = 9",
        "PRAGMA journal_mode = DELETE",
        "ATTACH DATABASE 'other.sqlite' AS other",
        "BEGIN",
        "CREATE TRIGGER t AFTER INSERT ON documents BEGIN SELECT 1; END",
    ]
    .map(|statement| run(&database, statement, None));
    let read_pragma = run(&database, "PRAGMA user_version", None);
    let kept = run(&database, "SELECT status, risk FROM stories", None);

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(story.as_deref(), Ok("1 row changed\n"));
    assert_eq!(updated.as_deref(), Ok("1 row changed\n"));
    assert!(column.is_ok(), "{column:?}");
    for (table, refusal) in CORE_TABLES.iter().zip(&refusals) {
        assert_eq!(
            refusal,
            &format!("refused: DROP TABLE {table}: a core table of the records, kept by the tool")
        );
    }
    for refused in [&renamed, &dropped_column, &dropped_index] {
        assert!(
            refused
                .as_ref()
                .is_err_and(|error| error.starts_with("refused: ")),
            "{refused:?}"
        );
    }
    for refused in documents.iter().chain(&others) {
        assert!(
            refused
                .as_ref()
                .is_err_and(|error| error.starts_with("refused: ")),
            "{refused:?}"
        );
    }
    assert_eq!(read_pragma.as_deref(), Ok("user_version\n5\n"));
    assert_eq!(kept.as_deref(), Ok("status\trisk\nparked\t\n"));
}

#[test]
fn rolls_back_a_write_that_fails_and_compiles_one_statement_only() {
    let (folder, database) = place("rollback");
    let duplicate = run(
        &database,
        "INSERT INTO epics (id, title, status) VALUES ('E49', 'a', 'done'), ('E49', 'b', 'done')",
        None,
    );
    let count = run(&database, "SELECT count(*) AS n FROM epics", None);
    let two = run(&database, "SELECT 1; SELECT 2", None);
    let wrong = run(&database, "SELEKT 1", None);

    let _ = fs::remove_dir_all(&folder);
    assert!(duplicate.is_err());
    assert_eq!(count.as_deref(), Ok("n\n0\n"));
    assert!(two.is_err());
    assert!(wrong.is_err());
}

#[test]
fn tells_an_added_column_from_another_change_of_a_table() {
    assert!(is_added_column("alter table stories add column risk TEXT"));
    assert!(is_added_column("ALTER  TABLE \"stories\"\nADD risk"));
    assert!(!is_added_column("ALTER TABLE stories RENAME TO tales"));
    assert!(!is_added_column("ALTER TABLE stories DROP COLUMN risk"));
    assert!(!is_added_column("SELECT 1"));
}
