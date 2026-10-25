//! Tests of the data files the CI reads (AUD-054): their JSON, one row a line, and those of a
//! checkout that differ from the records. The records are a file of the system's temporary
//! folder; the real records are never opened.

use std::fs;

use knowledge::data_files::{
    DEPENDENCIES_FILE, LINT_EXCEPTIONS_FILE, data_files, rows_json, stale_data_files, stale_refusal,
};
use rusqlite::Connection;

/// Records holding the two tables, a row in each.
fn records() -> Connection {
    let database = Connection::open_in_memory().unwrap();

    database
        .execute_batch(
            "CREATE TABLE lint_exceptions (id INTEGER PRIMARY KEY, file TEXT, item TEXT, rule TEXT, reason TEXT);
             CREATE TABLE dependencies (project TEXT, name TEXT, kind TEXT, version TEXT, purpose TEXT);
             INSERT INTO lint_exceptions (file, item, rule, reason) VALUES ('b.ts', 'g', 'y', 'later'), ('a.ts', 'f', 'x', NULL);
             INSERT INTO dependencies VALUES ('root', 'vite', 'npm', '8.3.4', 'the server'), ('root', 'regex', 'crate', '1.13.1', 'patterns'), ('playground', 'typescript', 'npm', '6.0.3', 'Angular');",
        )
        .unwrap();
    database
}

#[test]
fn writes_one_object_a_line_each_field_in_order_and_an_empty_array_for_no_row() {
    assert_eq!(
        rows_json(&["a", "b"], &[vec!["1".to_owned(), "x\ny".to_owned()]]),
        "[\n  { \"a\": \"1\", \"b\": \"x\\ny\" }\n]\n"
    );
    assert_eq!(rows_json(&["a"], &[]), "[]\n");
}

#[test]
fn sorts_the_rows_and_reads_a_null_as_an_empty_text() {
    let files = data_files(&records()).unwrap();

    assert_eq!(
        files,
        [
            (
                LINT_EXCEPTIONS_FILE,
                "[\n  { \"file\": \"a.ts\", \"item\": \"f\", \"rule\": \"x\", \"reason\": \"\" },\n  { \"file\": \"b.ts\", \"item\": \"g\", \"rule\": \"y\", \"reason\": \"later\" }\n]\n".to_owned()
            ),
            (
                DEPENDENCIES_FILE,
                "[\n  { \"project\": \"playground\", \"name\": \"typescript\", \"kind\": \"npm\", \"version\": \"6.0.3\" },\n  { \"project\": \"root\", \"name\": \"regex\", \"kind\": \"crate\", \"version\": \"1.13.1\" },\n  { \"project\": \"root\", \"name\": \"vite\", \"kind\": \"npm\", \"version\": \"8.3.4\" }\n]\n".to_owned()
            )
        ]
    );
    assert!(
        data_files(&Connection::open_in_memory().unwrap())
            .unwrap_err()
            .starts_with("tests/data/lint-exceptions.json: no such table")
    );
}

#[test]
fn names_a_data_file_missing_or_different_and_refuses_only_then() {
    let root = std::env::temp_dir().join(format!("knowledge-data-files-{}", std::process::id()));
    let database = records();

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("tests/data")).unwrap();

    let both = stale_data_files(&database, &root).unwrap();

    for (path, text) in data_files(&database).unwrap() {
        fs::write(root.join(path), text).unwrap();
    }

    let none = stale_data_files(&database, &root).unwrap();

    fs::write(root.join(LINT_EXCEPTIONS_FILE), "[]\n").unwrap();

    let one = stale_data_files(&database, &root).unwrap();

    let _ = fs::remove_dir_all(&root);
    assert_eq!(both, [LINT_EXCEPTIONS_FILE, DEPENDENCIES_FILE]);
    assert_eq!(none, Vec::<String>::new());
    assert_eq!(one, [LINT_EXCEPTIONS_FILE]);
    assert_eq!(stale_refusal(&none), None);
    assert_eq!(
        stale_refusal(&one),
        Some("data files the CI reads differ from the records: run target/release/knowledge export-data and commit them:\ntests/data/lint-exceptions.json".to_owned())
    );
}
