//! The clean snapshot of the records (E49, research note 0035, A clean snapshot): the working file
//! copied by `VACUUM INTO` a temporary file of the synced folder (`REF-SQLITE-VACUUM`, a consistent
//! snapshot of a live database), renamed to the day's name (`REF-POSIX-RENAME`, the name always the
//! old file or the new), the last seven days kept (E49, Decisions, Q8). The working file and its
//! `-wal` are never copied, a sync service catching them mid-transaction (`REF-SQLITE-CORRUPT`).
//! The synced folder is named by `AGENT_KNOWLEDGE_SNAPSHOT` alone: its path names the user's
//! account, so no message of this module prints it.

use std::path::{Path, PathBuf};
use std::time::Duration;

/// The variable naming the synced folder the snapshots go to.
pub const SNAPSHOT_VARIABLE: &str = "AGENT_KNOWLEDGE_SNAPSHOT";

/// How many daily snapshots are kept (E49, Decisions, Q8).
pub const KEPT: usize = 7;

/// The age past which a temporary file of the synced folder is a leftover of a snapshot killed
/// mid-copy, a hook's timeout spent: a live copy writes its file within the hook's 60 seconds at
/// most (`REF-CC-HOOKS`).
pub const LEFTOVER_AGE: Duration = Duration::from_secs(3600);

/// What the snapshot says when its variable is not set: it copies nothing, and does not fail.
pub const NOT_SET: &str = "AGENT_KNOWLEDGE_SNAPSHOT is not set: name the synced folder in the env of ~/.claude/settings.json, beside AGENT_RECORDS_ROOT; nothing copied.";

/// The name of a day's snapshot, `records-2026-10-25.sqlite`.
#[must_use]
pub fn snapshot_name(day: &str) -> String {
    let _ = day;
    String::new()
}

/// Whether a file's name is a day's snapshot: `records-`, a date `YYYY-MM-DD`, `.sqlite`.
#[must_use]
pub fn is_snapshot_name(name: &str) -> bool {
    let _ = name;
    false
}

/// The name of the temporary file a process copies into, `.records-<pid>.sqlite.tmp`.
#[must_use]
pub fn temporary_name(process: u32) -> String {
    let _ = process;
    String::new()
}

/// Whether a file's name is a snapshot's temporary file, `.records-<digits>.sqlite.tmp`.
#[must_use]
pub fn is_temporary_name(name: &str) -> bool {
    let _ = name;
    false
}

/// The snapshots to remove so that the `kept` newest stay: the oldest by their date, which their
/// names order; a name other than a snapshot's is never among them.
#[must_use]
pub fn removed_snapshots(names: &[String], kept: usize) -> Vec<String> {
    let _ = (names, kept);
    Vec::new()
}

/// The synced folder the variable names: `None` when it is unset or empty.
///
/// # Errors
/// When the folder does not exist: it is never made, so that a mistyped path never syncs a new
/// folder; the message names the variable, not the path.
pub fn snapshot_folder(setting: Option<&str>) -> Result<Option<PathBuf>, String> {
    let _ = setting;
    Ok(None)
}

/// Refuses a working file inside the synced folder, or in that folder itself: a sync service would
/// copy it, and its `-wal`, mid-transaction (`REF-SQLITE-CORRUPT`). Both paths are read with their
/// links resolved, the part of the file's path that does not exist yet kept as written.
///
/// # Errors
/// When the file lies in the synced folder, the message naming the two paths' relation by their
/// variables, never a path.
pub fn refuse_synced(file: &Path, folder: &Path) -> Result<(), String> {
    let _ = (file, folder);
    Ok(())
}

/// Takes the day's snapshot of the working file into the synced folder: `VACUUM INTO` a temporary
/// file, put in the rollback journal so that reading it leaves no `-wal` beside it, renamed over the
/// day's snapshot, the oldest beyond `KEPT` and the leftovers of a killed copy removed. What it
/// prints, one line naming the file by its name alone.
///
/// # Errors
/// When the working file lies in the synced folder, or SQLite or the file system fails.
pub fn take_snapshot(file: &Path, folder: &Path, day: &str) -> Result<String, String> {
    let _ = (file, folder, day);
    Ok(String::new())
}
