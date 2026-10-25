//! The store of the project's records (research note 0035, The database): one SQLite file shared
//! by every branch and worktree, through `rusqlite` as the library's store (`REF-RUSQLITE`). Its
//! knowledge tables hold every kind of document cut into sections, searched by FTS5
//! (`REF-SQLITE-FTS5`). Two worktrees write at once: the file in WAL (`REF-SQLITE-WAL`), each write
//! in `BEGIN IMMEDIATE` (`REF-SQLITE-TRANSACTION`), a second writer waiting within rusqlite's busy
//! timeout (`REF-RUSQLITE-CONNECTION`, `REF-SQLITE-BUSY-TIMEOUT`). The schema's number is
//! `PRAGMA user_version` (`REF-SQLITE-PRAGMA`); its changes are additive, so that an older binary
//! still reads a newer file, and refuses to write to it.

use std::fs;
use std::path::Path;

use rusqlite::{Connection, Transaction, TransactionBehavior};

/// The number of the schema this binary writes, `PRAGMA user_version` of the file.
pub const SCHEMA_VERSION: i64 = 1;

/// The tables of schema 1 (research note 0035, Its tables). A document is any former file or
/// record, cut into sections at its headings; `sections_index` indexes the sections without a copy
/// of their text, kept by the three triggers (`REF-SQLITE-FTS5`, §4.4.3); `calls` and `turns` take
/// the fields of the hooks' `tool-calls.jsonl` and `turns.jsonl` under their names, a field an older
/// line lacks left null.
const SCHEMA: &str = "
  CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY, kind TEXT NOT NULL, title TEXT NOT NULL, epic TEXT NOT NULL,
    feature TEXT NOT NULL, status TEXT NOT NULL, date TEXT NOT NULL, branch TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('draft', 'merged', 'dropped')),
    private INTEGER NOT NULL CHECK (private IN (0, 1)), path TEXT NOT NULL
  );
  CREATE TABLE IF NOT EXISTS fields (
    document TEXT NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    key TEXT NOT NULL, value TEXT NOT NULL
  );
  CREATE INDEX IF NOT EXISTS fields_by_key ON fields (key, value);
  CREATE TABLE IF NOT EXISTS sections (
    id INTEGER PRIMARY KEY,
    document TEXT NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    position INTEGER NOT NULL, heading TEXT NOT NULL, text TEXT NOT NULL,
    UNIQUE (document, position)
  );
  CREATE VIRTUAL TABLE IF NOT EXISTS sections_index USING fts5(
    heading, text, content = 'sections', content_rowid = 'id', tokenize = 'porter unicode61'
  );
  CREATE TRIGGER IF NOT EXISTS sections_insert AFTER INSERT ON sections BEGIN
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
  CREATE TRIGGER IF NOT EXISTS sections_delete AFTER DELETE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
  END;
  CREATE TRIGGER IF NOT EXISTS sections_update AFTER UPDATE ON sections BEGIN
    INSERT INTO sections_index (sections_index, rowid, heading, text)
      VALUES ('delete', old.id, old.heading, old.text);
    INSERT INTO sections_index (rowid, heading, text) VALUES (new.id, new.heading, new.text);
  END;
  CREATE TABLE IF NOT EXISTS links (
    \"from\" TEXT NOT NULL REFERENCES documents (id) ON DELETE CASCADE, \"to\" TEXT NOT NULL,
    PRIMARY KEY (\"from\", \"to\")
  );
  CREATE TABLE IF NOT EXISTS calls (
    toolUseId TEXT UNIQUE, time TEXT, startedAt INTEGER, durationMs INTEGER, session TEXT,
    branch TEXT, step TEXT, agent TEXT, agentId TEXT, tool TEXT, summary TEXT, description TEXT,
    isBackground INTEGER, inputChars INTEGER, outputChars INTEGER, isOk INTEGER, error TEXT,
    permissionMode TEXT, turnStart INTEGER, turnElapsedMs INTEGER, turnTokens INTEGER,
    turnOutputTokens INTEGER, contextPercent INTEGER, model TEXT, pendingAgents INTEGER,
    pendingCommands INTEGER, snapshotAgeMs INTEGER
  );
  CREATE TABLE IF NOT EXISTS turns (
    session TEXT NOT NULL, start INTEGER NOT NULL, wallMs INTEGER, activeMs INTEGER,
    tokens INTEGER, outputTokens INTEGER, calls INTEGER, failures INTEGER, commits INTEGER,
    PRIMARY KEY (session, start)
  );
";

/// A failure of SQLite on the store's file, named by its path.
fn on_file(path: &Path, error: &impl std::fmt::Display) -> String {
    format!("{}: {error}", path.display())
}

/// Brings a file of an older schema up to `SCHEMA_VERSION`, in one `BEGIN IMMEDIATE`: of several
/// connections opening a new file at once, the first makes the tables while the others wait, then
/// find them made.
fn migrate(database: &Connection) -> rusqlite::Result<()> {
    if schema_version(database)? >= SCHEMA_VERSION {
        return Ok(());
    }

    let transaction = Transaction::new_unchecked(database, TransactionBehavior::Immediate)?;

    if schema_version(&transaction)? < SCHEMA_VERSION {
        transaction.execute_batch(SCHEMA)?;
        transaction.pragma_update(None, "user_version", SCHEMA_VERSION)?;
    }
    transaction.commit()
}

/// Sets what a connection needs: the deletes cascading to a document's rows, which SQLite turns on
/// per connection, and WAL, set once on the file and kept by it (`REF-SQLITE-WAL`, §3.3).
fn prepare(database: &Connection) -> rusqlite::Result<()> {
    database.pragma_update(None, "foreign_keys", true)?;

    let mode: String = database.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;

    if !mode.eq_ignore_ascii_case("wal") {
        database.query_row("PRAGMA journal_mode = WAL", [], |row| {
            row.get::<_, String>(0)
        })?;
    }
    migrate(database)
}

/// Opens the store, creating its folder and its file when missing, in WAL, and bringing a file of
/// an older schema up to `SCHEMA_VERSION` in one `BEGIN IMMEDIATE` transaction; a file of a newer
/// schema is opened as it is, for reading.
///
/// # Errors
/// When the folder cannot be made, or SQLite cannot open the file or create the tables.
pub fn open_store(path: &Path) -> Result<Connection, String> {
    if let Some(folder) = path
        .parent()
        .filter(|folder| !folder.as_os_str().is_empty())
    {
        fs::create_dir_all(folder).map_err(|error| on_file(folder, &error))?;
    }

    let database = Connection::open(path).map_err(|error| on_file(path, &error))?;

    prepare(&database).map_err(|error| on_file(path, &error))?;
    Ok(database)
}

/// The schema's number the file holds, `PRAGMA user_version`.
///
/// # Errors
/// When SQLite fails to read.
pub fn schema_version(database: &Connection) -> rusqlite::Result<i64> {
    database.query_row("PRAGMA user_version", [], |row| row.get(0))
}

/// Runs a write in one `BEGIN IMMEDIATE` transaction, committed when the work succeeds and rolled
/// back otherwise; refused, untouched, on a file whose schema is newer than this binary's, read
/// once the lock is held so that no newer binary migrates the file in between.
///
/// # Errors
/// When the file's schema is newer, or SQLite fails, a busy timeout spent included.
pub fn write<T>(
    database: &Connection,
    work: impl FnOnce(&Transaction<'_>) -> rusqlite::Result<T>,
) -> Result<T, String> {
    let failed = |error: rusqlite::Error| error.to_string();
    let transaction =
        Transaction::new_unchecked(database, TransactionBehavior::Immediate).map_err(failed)?;
    let version = schema_version(&transaction).map_err(failed)?;

    if version > SCHEMA_VERSION {
        return Err(format!(
            "the file's schema is {version}, newer than this binary's {SCHEMA_VERSION}: rebuild the tools from your base, target/release/git-sequences sync"
        ));
    }

    let done = work(&transaction).map_err(failed)?;

    transaction.commit().map_err(failed)?;
    Ok(done)
}
