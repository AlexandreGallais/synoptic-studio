//! The records file of the repository a folder belongs to, as every command of `knowledge` and
//! the tools that write the backlog find it (EN-154): `$AGENT_RECORDS_ROOT/<repo>/records.sqlite`,
//! refused inside the synced folder before any message names a path (EN-153).

use std::env;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::records::{
    DATABASE_FILE, ROOT_VARIABLE, main_and_origin, project_name, project_records, records_folder,
};
use crate::snapshot::{SNAPSHOT_VARIABLE, refuse_synced};
use crate::store::open_store;

/// The records file of the repository a folder belongs to, from the two variables: refused
/// inside the synced folder, a synced folder named but not made yet read as written, so that no
/// message prints a path inside it.
///
/// # Errors
/// When the folder is in no repository, `AGENT_RECORDS_ROOT` is unset or names no folder, or the
/// file would lie in the synced folder.
pub fn records_file(cwd: &Path) -> Result<PathBuf, String> {
    let setting = env::var(SNAPSHOT_VARIABLE).ok();
    let root = env::var(ROOT_VARIABLE).ok();
    let (main, origin) = main_and_origin(cwd)?;
    let folder = records_folder(root.as_deref(), &project_name(&origin, &main))?;

    if let Some(named) = setting.as_deref().filter(|named| !named.is_empty()) {
        refuse_synced(&folder.join(DATABASE_FILE), Path::new(named))?;
    }
    Ok(project_records(root.as_deref(), &main, &origin)?.join(DATABASE_FILE))
}

/// The records of the repository a folder belongs to, opened: made or brought up to the schema.
///
/// # Errors
/// As [`records_file`], or when SQLite cannot open the file.
pub fn open_records(cwd: &Path) -> Result<Connection, String> {
    open_store(&records_file(cwd)?)
}
