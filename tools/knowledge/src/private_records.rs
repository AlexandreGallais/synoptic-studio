//! The private records that join the file (E49, the move into the records): the agent's memories,
//! the metrics' logs, the library's store, the archives' bundles and the instants of the saves,
//! formerly files of the records folder or of Claude Code's. Each importer reads its source,
//! never changes it, and may run again: a row already held is replaced or kept, never doubled.

use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use rusqlite::types::Value;
use rusqlite::{Connection, OpenFlags, OptionalExtension, params, params_from_iter};
use serde_json::Value as Json;

use crate::front_matter::front_matter;
use crate::moved::{Moved, markdown_files};
use crate::store::write;

/// The columns of `calls`, the fields of a line of `tool-calls.jsonl` (schema 1).
pub const CALL_COLUMNS: [&str; 27] = [
    "toolUseId",
    "time",
    "startedAt",
    "durationMs",
    "session",
    "branch",
    "step",
    "agent",
    "agentId",
    "tool",
    "summary",
    "description",
    "isBackground",
    "inputChars",
    "outputChars",
    "isOk",
    "error",
    "permissionMode",
    "turnStart",
    "turnElapsedMs",
    "turnTokens",
    "turnOutputTokens",
    "contextPercent",
    "model",
    "pendingAgents",
    "pendingCommands",
    "snapshotAgeMs",
];

/// The columns of `turns`, the fields of a line of `turns.jsonl`.
pub const TURN_COLUMNS: [&str; 9] = [
    "session",
    "start",
    "wallMs",
    "activeMs",
    "tokens",
    "outputTokens",
    "calls",
    "failures",
    "commits",
];

/// The columns of `tool_uses`, the fields of a line of `tool-uses.jsonl`.
pub const TOOL_USE_COLUMNS: [&str; 6] = [
    "tool",
    "subcommand",
    "worktree",
    "start",
    "durationMs",
    "exitCode",
];

/// A memory of Claude Code's, from its file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    pub name: String,
    pub description: String,
    /// `feedback`, `user`, `project`, `reference`; empty for the index.
    pub kind: String,
    pub modified: String,
}

/// A memory from its file's name and text: its front matter's `name`, else the file's stem.
#[must_use]
pub fn memory_of(file: &str, text: &str) -> Memory {
    let fields = front_matter(text);
    let field = |key: &str| fields.get(key).cloned().unwrap_or_default();
    let stem = file.strip_suffix(".md").unwrap_or(file);
    let name = field("name");

    Memory {
        name: if name.is_empty() {
            stem.to_owned()
        } else {
            name
        },
        description: field("description"),
        kind: field("type"),
        modified: field("modified"),
    }
}

/// Imports the agent's memories — the Markdown files of Claude Code's memory folder for the
/// project — into `memories`, private, a memory already held updated in place, so that its
/// triggers keep `records_index` whole, which a `REPLACE` would not (its delete fires no trigger);
/// `show MEMORY-<name>` reads it there. The files are read, never changed nor removed.
///
/// # Errors
/// When SQLite fails.
pub fn import_memories(database: &Connection, folder: &Path) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (file, text) in markdown_files(folder) {
        let memory = memory_of(&file, &text);

        database.execute(
            "INSERT INTO memories (name, description, type, private, modified, text)
            VALUES (?, ?, ?, 1, ?, ?)
            ON CONFLICT (name) DO UPDATE SET description = excluded.description,
              type = excluded.type, private = 1, modified = excluded.modified, text = excluded.text",
            params![
                memory.name,
                memory.description,
                memory.kind,
                memory.modified,
                text
            ],
        )?;
        moved.rows += 1;
    }
    Ok(moved)
}

/// A JSON value as SQLite holds it: a boolean as 0 or 1, a whole number as an integer, an array
/// or an object as its JSON text.
#[must_use]
pub fn sql_value(value: &Json) -> Value {
    match value {
        Json::Null => Value::Null,
        Json::Bool(flag) => Value::Integer(i64::from(*flag)),
        Json::Number(number) => number.as_i64().map_or_else(
            || number.as_f64().map_or(Value::Null, Value::Real),
            Value::Integer,
        ),
        Json::String(text) => Value::Text(text.clone()),
        other => Value::Text(other.to_string()),
    }
}

/// Writes the JSON lines of a text into a table, each line's fields under the columns of their
/// names, a field missing null, a line that is no object skipped; `verb` the insert's,
/// `INSERT OR IGNORE` or `INSERT OR REPLACE`. A call without its `toolUseId`, as the oldest lines,
/// takes `<session>-<time>`, so that it is written once. The lines written.
///
/// # Errors
/// When SQLite fails.
pub fn insert_lines(
    database: &Connection,
    text: &str,
    (verb, table, columns): (&str, &str, &[&str]),
) -> rusqlite::Result<usize> {
    let names: Vec<String> = columns.iter().map(|name| format!("\"{name}\"")).collect();
    let marks = vec!["?"; columns.len()].join(", ");
    let mut statement = database.prepare(&format!(
        "{verb} INTO \"{table}\" ({}) VALUES ({marks})",
        names.join(", ")
    ))?;
    let mut written = 0;

    for line in text.lines() {
        let Ok(Json::Object(mut fields)) = serde_json::from_str::<Json>(line) else {
            continue;
        };

        if table == "calls" && !fields.contains_key("toolUseId") {
            let key = |name: &str| fields.get(name).map(Json::to_string).unwrap_or_default();
            let id = format!("{}-{}", key("session"), key("time")).replace('"', "");

            fields.insert("toolUseId".to_owned(), Json::String(id));
        }

        let values = columns
            .iter()
            .map(|column| fields.get(*column).map_or(Value::Null, sql_value));

        written += statement.execute(params_from_iter(values))?;
    }
    Ok(written)
}

/// Imports the metrics' logs of a folder — `tool-calls.jsonl` into `calls`, `turns.jsonl` into
/// `turns`, `tool-uses.jsonl` into `tool_uses` — a file missing skipped: the hooks' folder of the
/// main checkout, or the metrics the records folder kept. The new rows written.
///
/// # Errors
/// When SQLite fails.
pub fn import_metrics(database: &Connection, folder: &Path) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (file, insert) in [
        (
            "tool-calls.jsonl",
            ("INSERT OR IGNORE", "calls", &CALL_COLUMNS[..]),
        ),
        (
            "turns.jsonl",
            ("INSERT OR REPLACE", "turns", &TURN_COLUMNS[..]),
        ),
        (
            "tool-uses.jsonl",
            ("INSERT OR IGNORE", "tool_uses", &TOOL_USE_COLUMNS[..]),
        ),
    ] {
        if let Ok(text) = fs::read_to_string(folder.join(file)) {
            moved.rows += insert_lines(database, &text, insert)?;
        }
    }
    Ok(moved)
}

/// Copies the library of another file into the records' tables (`library::store`), the source
/// opened apart and only read: each page replaced unless the records read it later, its sections
/// written again, each use kept once. The pages copied.
///
/// # Errors
/// When SQLite fails on either file.
pub fn import_library(database: &Connection, source: &Connection) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();
    let mut pages = source.prepare(
        "SELECT url, title, fetched_at, modified_at, etag, last_modified, markdown FROM pages",
    )?;
    let mut rows = pages.query([])?;

    while let Some(row) = rows.next()? {
        let url: String = row.get(0)?;
        let fetched: Value = row.get(2)?;
        // A page the records read later than the source is kept: an older copy never wins.
        let newer: Option<i64> = database
            .query_row(
                "SELECT 1 FROM pages WHERE url = ?1 AND fetched_at > ?2",
                params![url, fetched],
                |_| Ok(1),
            )
            .optional()?;

        if newer.is_some() {
            continue;
        }
        database.execute(
            "INSERT OR REPLACE INTO pages (url, title, fetched_at, modified_at, etag, last_modified, markdown)
            VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                url,
                row.get::<_, String>(1)?,
                row.get::<_, Value>(2)?,
                row.get::<_, Value>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?
            ],
        )?;
        database.execute("DELETE FROM chunks WHERE url = ?", [&url])?;

        let mut chunks =
            source.prepare_cached("SELECT position, heading, text FROM chunks WHERE url = ?")?;
        let mut sections = chunks.query([&url])?;

        while let Some(section) = sections.next()? {
            database.execute(
                "INSERT INTO chunks (url, position, heading, text) VALUES (?, ?, ?, ?)",
                params![
                    url,
                    section.get::<_, Value>(0)?,
                    section.get::<_, String>(1)?,
                    section.get::<_, String>(2)?
                ],
            )?;
        }
        moved.rows += 1;
    }

    let mut uses =
        source.prepare("SELECT time, tool, url, query, origin, project FROM project_uses")?;
    let mut rows = uses.query([])?;

    while let Some(row) = rows.next()? {
        let values: Vec<Value> = (0..6)
            .map(|at| row.get::<_, Value>(at))
            .collect::<rusqlite::Result<_>>()?;

        database.execute(
            "INSERT INTO project_uses (time, tool, url, query, origin, project)
            SELECT ?1, ?2, ?3, ?4, ?5, ?6 WHERE NOT EXISTS (
              SELECT 1 FROM project_uses WHERE time IS ?1 AND tool IS ?2 AND url IS ?3
                AND query IS ?4 AND origin IS ?5 AND project IS ?6
            )",
            params_from_iter(values),
        )?;
    }
    Ok(moved)
}

/// Imports the files of a folder of archives — the Git bundles of the branches archived — into
/// `archives`, each with its size, its time and its bytes, a bundle of the same name replaced.
///
/// # Errors
/// When a file cannot be read, or SQLite fails.
pub fn import_archives(database: &Connection, folder: &Path) -> Result<Moved, String> {
    let mut moved = Moved::default();
    let mut names: Vec<String> = fs::read_dir(folder)
        .map_err(|error| format!("{}: {error}", folder.display()))?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();

    names.sort();
    for name in names {
        let path = folder.join(&name);
        let failed = |error: std::io::Error| format!("{}: {error}", path.display());
        let bytes = fs::read(&path).map_err(failed)?;
        let modified = fs::metadata(&path)
            .and_then(|metadata| metadata.modified())
            .map_err(failed)?
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| i64::try_from(since.as_secs()).unwrap_or(0));

        database
            .execute(
                "INSERT OR REPLACE INTO archives (name, size, modified, bytes) VALUES (?, ?, ?, ?)",
                params![
                    name,
                    i64::try_from(bytes.len()).unwrap_or(i64::MAX),
                    modified,
                    bytes
                ],
            )
            .map_err(|error| error.to_string())?;
        moved.rows += 1;
    }
    Ok(moved)
}

/// Writes the instants of a text of saves, one a line as `last-save.txt` held its one, into
/// `saves`, each once. The new rows written.
///
/// # Errors
/// When SQLite fails.
pub fn import_saves(database: &Connection, text: &str) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for instant in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        moved.rows += database.execute("INSERT OR IGNORE INTO saves (at) VALUES (?)", [instant])?;
    }
    Ok(moved)
}

/// The last save's instant, `None` before the first.
///
/// # Errors
/// When SQLite fails.
pub fn last_save(database: &Connection) -> rusqlite::Result<Option<String>> {
    database.query_row("SELECT max(at) FROM saves", [], |row| row.get(0))
}

/// [`import_library`] of a library's file, opened read-only, in one write of the records.
///
/// # Errors
/// When the file cannot be opened, or SQLite fails on either file.
pub fn import_library_file(database: &Connection, path: &Path) -> Result<Moved, String> {
    let source = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|error| format!("{}: {error}", path.display()))?;

    write(database, |done| import_library(done, &source))
}
