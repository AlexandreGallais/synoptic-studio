//! A document written into the knowledge on a branch, as a `draft` (research note 0035, The rows
//! of a branch): its row of `documents`, its fields, its sections and its citations, in the
//! transaction of the write that calls it.

use rusqlite::Connection;

/// What `add` is given beside the text: the kind, the file it came from, the branch it is written
/// on and the day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Origin<'a> {
    /// The document's kind: `story`, `note`, `journal`.
    pub kind: &'a str,
    /// The file it was read from, empty for the standard input.
    pub path: &'a str,
    /// The branch it is written on.
    pub branch: &'a str,
    /// The day it is written, `2026-10-25`: its date when its front matter gives none.
    pub today: &'a str,
}

/// Whether a kind is one word of lower-case letters, dashes inside: `story`, `log-row`.
#[must_use]
pub fn is_kind(kind: &str) -> bool {
    let _ = kind;
    false
}

/// The id made for a document whose front matter gives none: `<KIND>-<date>-<n>`, `n` one more
/// than the highest of that kind and day in the store, any branch or state counted, so that two
/// branches never make the same id.
///
/// # Errors
/// When SQLite fails to read.
pub fn next_id(database: &Connection, kind: &str, date: &str) -> rusqlite::Result<String> {
    let _ = (database, kind, date);
    Ok(String::new())
}

/// Writes a text as a `draft` of its branch, replacing that branch's row of the same id, and
/// returns its id. Its fields, its sections and its citations go to their tables, its own id left
/// out of its citations.
///
/// # Errors
/// When SQLite fails.
pub fn add_document(
    database: &Connection,
    text: &str,
    origin: Origin<'_>,
) -> rusqlite::Result<String> {
    let _ = (database, text, origin);
    Ok(String::new())
}
