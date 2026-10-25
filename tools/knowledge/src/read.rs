//! What a branch reads of the knowledge (research note 0035, The rows of a branch; The tool): the
//! `merged` rows and its own `draft` rows, its draft of an id over the merged row of it, the
//! `dropped` rows with `--all`; searched by FTS5, best first by `bm25()` (`REF-SQLITE-FTS5`).

use rusqlite::Connection;

/// The filters of `search`, `list` and `show`, each `None` when not given.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Filters {
    /// `--kind`: the documents of that kind.
    pub kind: Option<String>,
    /// `--epic`: the documents of that epic.
    pub epic: Option<String>,
    /// `--feature`: the documents of that feature.
    pub feature: Option<String>,
    /// `--status`: the documents of that status.
    pub status: Option<String>,
    /// `--since`: the documents dated that day or later, `2026-10-25`.
    pub since: Option<String>,
    /// `--branch`: the rows written on that branch, read as that branch reads them.
    pub branch: Option<String>,
    /// `--state`: the rows of that state; `dropped` reads them as `--all` does.
    pub state: Option<String>,
    /// `--all`: the `dropped` rows too.
    pub all: bool,
}

/// A section found by `search`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    /// Its document's id.
    pub id: String,
    /// Its heading, empty for the text above the first heading.
    pub heading: String,
    /// The words found in their context, each between `[` and `]`.
    pub snippet: String,
}

/// A document found by `list`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listed {
    /// Its id.
    pub id: String,
    /// Its kind.
    pub kind: String,
    /// Its status, empty without one.
    pub status: String,
    /// Its date.
    pub date: String,
    /// Its state, `draft`, `merged` or `dropped`.
    pub state: String,
    /// The branch it was written on.
    pub branch: String,
    /// Its title.
    pub title: String,
}

/// How many sections `search` prints at most, as the library's search.
pub const SEARCH_LIMIT: i64 = 8;

/// The filters of the words of a command, and the words that are none: `--all`, and each other
/// flag followed by its value.
///
/// # Errors
/// On an unknown flag, or a flag without its value.
pub fn parse_filters(words: &[String]) -> Result<(Vec<String>, Filters), String> {
    let _ = words;
    Err("not written yet".to_owned())
}

/// The sections best matching some words, best first by BM25, at most [`SEARCH_LIMIT`]: every word
/// if a section holds them all, else any; of the rows `reader`, a branch, reads, filtered.
///
/// # Errors
/// When SQLite fails to read, or no word is given.
pub fn search(
    database: &Connection,
    reader: &str,
    words: &str,
    filters: &Filters,
) -> rusqlite::Result<Vec<Hit>> {
    let _ = (database, reader, words, filters);
    Ok(Vec::new())
}

/// The documents `reader`, a branch, reads, filtered, sorted by id.
///
/// # Errors
/// When SQLite fails to read.
pub fn list(
    database: &Connection,
    reader: &str,
    filters: &Filters,
) -> rusqlite::Result<Vec<Listed>> {
    let _ = (database, reader, filters);
    Ok(Vec::new())
}

/// The text of the document `id` as `reader`, a branch, reads it, filtered: its own draft, else
/// the merged row, else, with `--all`, the dropped row written last; `None` when it reads none.
///
/// # Errors
/// When SQLite fails to read.
pub fn show(
    database: &Connection,
    reader: &str,
    id: &str,
    filters: &Filters,
) -> rusqlite::Result<Option<String>> {
    let _ = (database, reader, id, filters);
    Ok(None)
}

/// The sections of a document's text under a heading, each preceded by `<id>:<line>` as `page`
/// prints a section; `None` when no heading of it names it.
#[must_use]
pub fn section_of(id: &str, text: &str, heading: &str) -> Option<String> {
    let _ = (id, text, heading);
    None
}

/// A hit's two lines: `<id> › <heading>`, the heading left out when empty, then its snippet on one
/// line, indented.
#[must_use]
pub fn hit_lines(hit: &Hit) -> String {
    let _ = hit;
    String::new()
}

/// A document's line: `<id> [<kind>, <status>, <date>] <title> — <state> on <branch>`, an empty
/// status left out.
#[must_use]
pub fn listed_line(listed: &Listed) -> String {
    let _ = listed;
    String::new()
}
