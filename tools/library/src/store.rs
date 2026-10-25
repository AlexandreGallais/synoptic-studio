//! Storage of the local source library (`docs/tooling/sources.md`, ADR-0039): one SQLite file
//! shared by every project of the machine, a table of pages and a full-text index of their
//! sections (FTS5, ranked by BM25, `REF-SQLITE-FTS5`), through `rusqlite` (`REF-RUSQLITE`). The
//! schema is the one `library-store.ts` wrote, so that a file written here is read by an older
//! checkout's scripts and the reverse, without a migration.

use std::fs;
use std::path::Path;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, OptionalExtension, Row, ToSql, params};

use crate::chunks::Chunk;
use crate::text;

/// A stored page and what it says about itself.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StoredPage {
    pub etag: String,
    /// When the library last read it online, epoch milliseconds.
    pub fetched_at: i64,
    pub last_modified: String,
    pub markdown: String,
    /// When the page says it last changed, epoch milliseconds, 0 when it does not say.
    pub modified_at: i64,
    pub title: String,
    pub url: String,
}

/// One section found by a search.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    pub heading: String,
    pub index: i64,
    pub snippet: String,
    pub title: String,
    pub url: String,
}

/// A stored page as the list gives it: address, title and reading time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Listed {
    pub fetched_at: i64,
    pub title: String,
    pub url: String,
}

/// Tables of the library: pages, the full-text index of their sections, and their uses.
const SCHEMA: &str = "
  CREATE TABLE IF NOT EXISTS pages (
    url TEXT PRIMARY KEY, title TEXT NOT NULL, fetched_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL, etag TEXT NOT NULL, last_modified TEXT NOT NULL,
    markdown TEXT NOT NULL
  );
  CREATE VIRTUAL TABLE IF NOT EXISTS chunks USING fts5(
    url UNINDEXED, position UNINDEXED, heading, text, tokenize = 'porter unicode61'
  );
  CREATE TABLE IF NOT EXISTS project_uses (
    time INTEGER NOT NULL, tool TEXT NOT NULL, url TEXT NOT NULL, query TEXT NOT NULL,
    origin TEXT NOT NULL, project TEXT NOT NULL
  );
";

/// The uses as the library recorded them before EN-101, without their project: a view of
/// `project_uses`, which an older copy of the library's scripts still reads and writes, its rows
/// taking an empty project.
const USES_VIEW: &str = "
  CREATE VIEW IF NOT EXISTS uses AS SELECT time, tool, url, query, origin FROM project_uses;
  CREATE TRIGGER IF NOT EXISTS uses_insert INSTEAD OF INSERT ON uses BEGIN
    INSERT INTO project_uses VALUES (NEW.time, NEW.tool, NEW.url, NEW.query, NEW.origin, '');
  END;
";

/// The library's file, found as the hooks find it: one function for both (AUD-023).
pub use hooks::pages::{FILE_VARIABLE, library_file, library_file_of_process};

/// Brings a library made before EN-101 up to the schema: its `uses` table moved into
/// `project_uses` with an empty project, then `uses` made a view of it.
fn migrate(database: &Connection) -> rusqlite::Result<()> {
    let old = database
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'uses'",
            [],
            |_| Ok(()),
        )
        .optional()?;

    if old.is_some() {
        database.execute_batch(
            "BEGIN IMMEDIATE;
            INSERT INTO project_uses SELECT time, tool, url, query, origin, '' FROM uses;
            DROP TABLE uses;
            COMMIT;",
        )?;
    }

    database.execute_batch(USES_VIEW)
}

/// Opens the library, creating its folder, its file and its tables when missing; `:memory:` opens
/// a library of its own in memory.
///
/// # Errors
///
/// When the folder cannot be made or SQLite cannot open the file or create the tables.
pub fn open_library(path: &Path) -> rusqlite::Result<Connection> {
    let database = if path == Path::new(":memory:") {
        Connection::open_in_memory()?
    } else {
        if let Some(folder) = path
            .parent()
            .filter(|folder| !folder.as_os_str().is_empty())
        {
            fs::create_dir_all(folder)
                .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
        }

        Connection::open(path)?
    };

    database.execute_batch(SCHEMA)?;
    migrate(&database)?;

    Ok(database)
}

/// A number column, written by `node:sqlite` as an integer or a real (`position` is a real in the
/// files it wrote), as the integer it stands for.
fn number(row: &Row<'_>, index: usize) -> rusqlite::Result<i64> {
    Ok(match row.get_ref(index)? {
        ValueRef::Integer(value) => value,
        ValueRef::Real(value) => whole(value),
        _ => 0,
    })
}

/// A real that holds a whole number, as that integer.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a real here is a whole number of milliseconds or a position"
)]
fn whole(value: f64) -> i64 {
    value as i64
}

/// Stores a page and indexes its sections, replacing what was stored for its address.
///
/// # Errors
///
/// When SQLite fails to write.
pub fn save_page(
    database: &Connection,
    page: &StoredPage,
    chunks: &[Chunk],
) -> rusqlite::Result<()> {
    database
        .prepare_cached("INSERT OR REPLACE INTO pages VALUES (?, ?, ?, ?, ?, ?, ?)")?
        .execute(params![
            page.url,
            page.title,
            page.fetched_at,
            page.modified_at,
            page.etag,
            page.last_modified,
            page.markdown
        ])?;
    database
        .prepare_cached("DELETE FROM chunks WHERE url = ?")?
        .execute([&page.url])?;

    let mut insert = database.prepare_cached("INSERT INTO chunks VALUES (?, ?, ?, ?)")?;

    for chunk in chunks {
        insert.execute(params![page.url, chunk.index, chunk.heading, chunk.text])?;
    }

    Ok(())
}

/// Marks a stored page as read again online without change (HTTP 304).
///
/// # Errors
///
/// When SQLite fails to write.
pub fn touch_page(database: &Connection, url: &str, fetched_at: i64) -> rusqlite::Result<()> {
    database
        .prepare_cached("UPDATE pages SET fetched_at = ? WHERE url = ?")?
        .execute(params![fetched_at, url])
        .map(|_| ())
}

/// A stored page, `None` when it is not stored.
///
/// # Errors
///
/// When SQLite fails to read.
pub fn find_page(database: &Connection, url: &str) -> rusqlite::Result<Option<StoredPage>> {
    database
        .prepare_cached(
            "SELECT etag, fetched_at, last_modified, markdown, modified_at, title, url
            FROM pages WHERE url = ?",
        )?
        .query_row([url], |row| {
            Ok(StoredPage {
                etag: row.get(0)?,
                fetched_at: number(row, 1)?,
                last_modified: row.get(2)?,
                markdown: row.get(3)?,
                modified_at: number(row, 4)?,
                title: row.get(5)?,
                url: row.get(6)?,
            })
        })
        .optional()
}

/// Words of a search as an FTS5 query: each word quoted, so that no character of the text is read
/// as query syntax; `"bm25" AND "rank"` for `bm25 rank` and `AND`.
#[must_use]
pub fn fts_query(words: &str, operator: &str) -> String {
    words
        .split(text::is_space)
        .filter(|word| !word.is_empty())
        .map(|word| format!("\"{}\"", word.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(&format!(" {operator} "))
}

/// Sections matching an FTS5 query, best first (BM25), each with a snippet around the words.
fn matching_sections(
    database: &Connection,
    query: &str,
    limit: &dyn ToSql,
) -> rusqlite::Result<Vec<Hit>> {
    let mut statement = database.prepare_cached(
        "SELECT chunks.url, position, heading, pages.title,
          snippet(chunks, 3, '[', ']', '…', 24) AS snippet
        FROM chunks JOIN pages ON pages.url = chunks.url
        WHERE chunks MATCH ? ORDER BY bm25(chunks) LIMIT ?",
    )?;

    statement
        .query_map(params![query, limit], |row| {
            Ok(Hit {
                heading: row.get(2)?,
                index: number(row, 1)?,
                snippet: row.get(4)?,
                title: row.get(3)?,
                url: row.get(0)?,
            })
        })?
        .collect()
}

/// What a search finds for some words: `matching` run on their FTS5 query of every word, then, when
/// it finds nothing, on the query of any; shared by the library's search and `knowledge search`.
///
/// # Errors
///
/// When `matching` fails.
pub fn every_word_else_any<T>(
    words: &str,
    mut matching: impl FnMut(&str) -> rusqlite::Result<Vec<T>>,
) -> rusqlite::Result<Vec<T>> {
    let every = matching(&fts_query(words, "AND"))?;

    if every.is_empty() {
        matching(&fts_query(words, "OR"))
    } else {
        Ok(every)
    }
}

/// Sections best matching some words, best first (BM25): every word if possible, else any.
///
/// # Errors
///
/// When SQLite fails to read, or reads no query in the words (none given).
pub fn search_library(
    database: &Connection,
    words: &str,
    limit: &dyn ToSql,
) -> rusqlite::Result<Vec<Hit>> {
    every_word_else_any(words, |query| matching_sections(database, query, limit))
}

/// Sections of a stored page, in page order.
///
/// # Errors
///
/// When SQLite fails to read.
pub fn page_sections(database: &Connection, url: &str) -> rusqlite::Result<Vec<Chunk>> {
    let mut statement = database.prepare_cached(
        "SELECT position, heading, text FROM chunks WHERE url = ? ORDER BY position",
    )?;

    statement
        .query_map([url], |row| {
            Ok(Chunk {
                heading: row.get(1)?,
                index: number(row, 0)?,
                text: row.get(2)?,
            })
        })?
        .collect()
}

/// Stored pages, last read first.
///
/// # Errors
///
/// When SQLite fails to read.
pub fn list_pages(database: &Connection) -> rusqlite::Result<Vec<Listed>> {
    let mut statement = database
        .prepare_cached("SELECT url, title, fetched_at FROM pages ORDER BY fetched_at DESC")?;

    statement
        .query_map([], |row| {
            Ok(Listed {
                fetched_at: number(row, 2)?,
                title: row.get(1)?,
                url: row.get(0)?,
            })
        })?
        .collect()
}
