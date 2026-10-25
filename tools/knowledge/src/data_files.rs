//! The data files the CI reads in place of the records (AUD-054): the CI has no records, so what
//! one of its checks needs from them is exported to a committed file, written by `knowledge
//! export-data` from the checkout's root and checked against the records by `export-data --check`,
//! which `story-finish` runs before a pull request into `develop`. Each file is a JSON array, one
//! object per row and per line, as `export-criteria` writes `tests/data/criteria.json`.

use std::fs;
use std::path::Path;

use rusqlite::Connection;

use crate::backlog::json_string;

/// The register of exceptions (ADR-0066), which `tests/exceptions.test.ts` reads.
pub const LINT_EXCEPTIONS_FILE: &str = "tests/data/lint-exceptions.json";

/// The dependencies of both npm projects, the root's and the playground's (ADR-0104), which
/// `tests/package.test.ts` and `dependencies outdated` read.
pub const DEPENDENCIES_FILE: &str = "tests/data/dependencies.json";

/// One data file: its path from the checkout's root, the query of its rows, and the names of its
/// fields, each a column of the query in its order.
struct DataFile {
    path: &'static str,
    query: &'static str,
    fields: &'static [&'static str],
}

/// The data files, in the order they are written.
const DATA_FILES: [DataFile; 2] = [
    DataFile {
        path: LINT_EXCEPTIONS_FILE,
        query: "SELECT file, item, rule, reason FROM lint_exceptions ORDER BY file, item, rule, id",
        fields: &["file", "item", "rule", "reason"],
    },
    DataFile {
        path: DEPENDENCIES_FILE,
        query: "SELECT project, name, kind, version FROM dependencies ORDER BY project, kind, name",
        fields: &["project", "name", "kind", "version"],
    },
];

/// A JSON array of rows, one object a line, its fields in the order given.
#[must_use]
pub fn rows_json(fields: &[&str], rows: &[Vec<String>]) -> String {
    let lines: Vec<String> = rows
        .iter()
        .map(|row| {
            let members: Vec<String> = fields
                .iter()
                .zip(row)
                .map(|(field, value)| format!("{}: {}", json_string(field), json_string(value)))
                .collect();

            format!("  {{ {} }}", members.join(", "))
        })
        .collect();

    if lines.is_empty() {
        "[]\n".to_owned()
    } else {
        format!("[\n{}\n]\n", lines.join(",\n"))
    }
}

/// The rows of a query, each column read as text, a null as an empty text.
fn text_rows(
    database: &Connection,
    query: &str,
    width: usize,
) -> rusqlite::Result<Vec<Vec<String>>> {
    let mut statement = database.prepare(query)?;

    statement
        .query_map([], |row| {
            (0..width)
                .map(|at| {
                    row.get::<_, Option<String>>(at)
                        .map(Option::unwrap_or_default)
                })
                .collect()
        })?
        .collect()
}

/// Each data file's path and the text the records give it.
///
/// # Errors
/// When a table is missing or SQLite fails, naming the file.
pub fn data_files(database: &Connection) -> Result<Vec<(&'static str, String)>, String> {
    DATA_FILES
        .iter()
        .map(|file| {
            let rows = text_rows(database, file.query, file.fields.len())
                .map_err(|error| format!("{}: {error}", file.path))?;

            Ok((file.path, rows_json(file.fields, &rows)))
        })
        .collect()
}

/// The data files of a checkout that differ from what the records give them, or are missing.
///
/// # Errors
/// When a table is missing or SQLite fails.
pub fn stale_data_files(database: &Connection, root: &Path) -> Result<Vec<String>, String> {
    Ok(data_files(database)?
        .into_iter()
        .filter(|(path, text)| fs::read_to_string(root.join(path)).ok().as_ref() != Some(text))
        .map(|(path, _)| path.to_owned())
        .collect())
}

/// Why a pull request is not opened: the data files the records disagree with; `None` when there
/// are none.
#[must_use]
pub fn stale_refusal(stale: &[String]) -> Option<String> {
    (!stale.is_empty()).then(|| {
        format!(
            "data files the CI reads differ from the records: run target/release/knowledge export-data and commit them:\n{}",
            stale.join("\n")
        )
    })
}
