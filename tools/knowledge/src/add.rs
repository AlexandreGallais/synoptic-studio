//! A document written into the knowledge on a branch, as a `draft` (research note 0035, The rows
//! of a branch): its row of `documents`, its fields, its sections and its citations, in the
//! transaction of the write that calls it.

use rusqlite::{Connection, params};

use crate::document::{Document, read_document};

/// What `add` is given beside the text: the kind, the file it came from, the branch it is written
/// on and the day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Origin<'a> {
    /// The document's kind: `story`, `note`, `log`.
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
    !kind.starts_with('-')
        && !kind.ends_with('-')
        && !kind.is_empty()
        && kind
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
}

/// The table of a kind whose records are rows with their columns rather than documents: the
/// journal, the incidents, the improvements and the feedback, read by `knowledge sql` and written
/// by it, one way to write each; `None` for a kind of documents.
#[must_use]
pub fn row_table(kind: &str) -> Option<&'static str> {
    match kind {
        "journal" => Some("journal"),
        "incident" => Some("incidents"),
        "improvement" => Some("improvements"),
        "feedback" => Some("feedback"),
        _ => None,
    }
}

/// The id made for a document whose front matter gives none: `<KIND>-<date>-<n>`, `n` one more
/// than the highest of that kind and day in the store, any branch or state counted, so that two
/// branches never make the same id.
///
/// # Errors
/// When SQLite fails to read.
pub fn next_id(database: &Connection, kind: &str, date: &str) -> rusqlite::Result<String> {
    let prefix = format!("{}-{date}-", kind.to_uppercase());
    let mut statement =
        database.prepare_cached("SELECT id FROM documents WHERE substr(id, 1, length(?1)) = ?1")?;
    let ids: Vec<String> = statement
        .query_map([&prefix], |row| row.get(0))?
        .collect::<rusqlite::Result<_>>()?;
    let highest = ids
        .iter()
        .filter_map(|id| id[prefix.len()..].parse::<u64>().ok())
        .max()
        .unwrap_or(0);

    Ok(format!("{prefix}{}", highest + 1))
}

/// Writes a text as a `draft` of its branch, replacing that branch's draft of the same id, and
/// returns its id: its merged row of the id stays, read by the other branches until the new draft
/// is merged. Its fields, its sections and its citations go to their tables, its own id left
/// out of its citations.
///
/// # Errors
/// When SQLite fails.
pub fn add_document(
    database: &Connection,
    text: &str,
    origin: Origin<'_>,
) -> rusqlite::Result<String> {
    let document = read_document(text);
    let id = match &document.id {
        Some(id) => id.clone(),
        None => next_id(
            database,
            origin.kind,
            document.date.as_deref().unwrap_or(origin.today),
        )?,
    };

    database.execute(
        "DELETE FROM documents WHERE id = ? AND branch = ? AND state = 'draft'",
        params![id, origin.branch],
    )?;
    insert_document(database, &id, &document, origin, Written::Draft)?;
    Ok(id)
}

/// How a document is written: a draft of its branch, or a merged row, public or private.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Written {
    /// A `draft` of its branch, as `add` writes.
    Draft,
    /// A `merged` row, as an import writes what the trunk held; `true` for a private one.
    Merged(bool),
}

/// Writes a text as the merged document of an id, replacing the merged row of that id, the
/// drafts of every branch kept (E49, the move into the records): what an import of the files the
/// trunk held writes, so that `search` and `show` find it. A private one, a memory, is flagged.
///
/// # Errors
/// When SQLite fails.
pub fn put_merged(
    database: &Connection,
    id: &str,
    text: &str,
    origin: Origin<'_>,
    private: bool,
) -> rusqlite::Result<()> {
    database.execute(
        "DELETE FROM documents WHERE id = ? AND state = 'merged'",
        [id],
    )?;
    insert_document(
        database,
        id,
        &read_document(text),
        origin,
        Written::Merged(private),
    )
}

/// Inserts a document's row, its fields, its sections and its citations, its own id left out of
/// them, in the state asked.
fn insert_document(
    database: &Connection,
    id: &str,
    document: &Document,
    origin: Origin<'_>,
    written: Written,
) -> rusqlite::Result<()> {
    let date = document.date.as_deref().unwrap_or(origin.today);
    let (state, private) = match written {
        Written::Draft => ("draft", false),
        Written::Merged(private) => ("merged", private),
    };

    database.execute(
        "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            id,
            origin.kind,
            document.title,
            document.epic,
            document.feature,
            document.status,
            date,
            origin.branch,
            state,
            private,
            origin.path
        ],
    )?;
    for (key, value) in &document.fields {
        database.execute(
            "INSERT INTO fields (document, branch, state, key, value) VALUES (?, ?, ?, ?, ?)",
            params![id, origin.branch, state, key, value],
        )?;
    }
    for (position, section) in (0_i64..).zip(&document.sections) {
        database.execute(
            "INSERT INTO sections (document, branch, state, position, heading, text)
            VALUES (?, ?, ?, ?, ?, ?)",
            params![
                id,
                origin.branch,
                state,
                position,
                section.heading,
                section.text
            ],
        )?;
    }
    for cited in document.citations.iter().filter(|cited| *cited != id) {
        database.execute(
            "INSERT INTO links (\"from\", branch, state, \"to\") VALUES (?, ?, ?, ?)",
            params![id, origin.branch, state, cited],
        )?;
    }
    Ok(())
}
