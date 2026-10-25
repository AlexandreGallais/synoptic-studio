//! The store of the project's records (research note 0035, The database): one SQLite file shared
//! by every branch and worktree, through `rusqlite` as the library's store (`REF-RUSQLITE`). Its
//! knowledge tables hold every kind of document cut into sections, searched by FTS5
//! (`REF-SQLITE-FTS5`). Two worktrees write at once: the file in WAL (`REF-SQLITE-WAL`), each write
//! in `BEGIN IMMEDIATE` (`REF-SQLITE-TRANSACTION`), a second writer waiting within rusqlite's busy
//! timeout (`REF-RUSQLITE-CONNECTION`, `REF-SQLITE-BUSY-TIMEOUT`). The schema's number is
//! `PRAGMA user_version` (`REF-SQLITE-PRAGMA`); its changes are additive, so that an older binary
//! still reads a newer file, and refuses to write to it.

use std::path::Path;

use rusqlite::{Connection, Transaction};

/// The number of the schema this binary writes, `PRAGMA user_version` of the file.
pub const SCHEMA_VERSION: i64 = 1;

/// Opens the store, creating its folder and its file when missing, in WAL, and bringing a file of
/// an older schema up to `SCHEMA_VERSION` in one `BEGIN IMMEDIATE` transaction; a file of a newer
/// schema is opened as it is, for reading.
///
/// # Errors
/// When the folder cannot be made, or SQLite cannot open the file or create the tables.
pub fn open_store(path: &Path) -> Result<Connection, String> {
    let _ = path;
    Err("not written yet".to_owned())
}

/// The schema's number the file holds, `PRAGMA user_version`.
///
/// # Errors
/// When SQLite fails to read.
pub fn schema_version(database: &Connection) -> rusqlite::Result<i64> {
    let _ = database;
    Ok(0)
}

/// Runs a write in one `BEGIN IMMEDIATE` transaction, committed when the work succeeds and rolled
/// back otherwise; refused, untouched, on a file whose schema is newer than this binary's.
///
/// # Errors
/// When the file's schema is newer, or SQLite fails, a busy timeout spent included.
pub fn write<T>(
    database: &Connection,
    work: impl FnOnce(&Transaction<'_>) -> rusqlite::Result<T>,
) -> Result<T, String> {
    let _ = (database, work);
    Err("not written yet".to_owned())
}
