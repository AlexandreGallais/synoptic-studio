//! `records-save` (`target/release/records-save`, ADR-0081, EN-114), at the end of every session and at
//! every retrospective: the metrics logs, this project's transcripts and the source library copied
//! into `<AGENT_RECORDS_ROOT>/<project>`, a local folder outside the repository, never sent
//! anywhere. A missing source is skipped; the library is copied consistently by SQLite's `VACUUM
//! INTO`, even while the library server holds it open. Then it takes the day's snapshot of the
//! records into the synced folder `AGENT_KNOWLEDGE_SNAPSHOT` names, as `knowledge snapshot` does
//! (EN-153), saying so and failing nothing when the variable is not set.

use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use calendar::temporal_instant;
use hooks::pages::library_file;
use knowledge::records::{DATABASE_FILE, main_and_origin, project_records};
use knowledge::snapshot::{NOT_SET, SNAPSHOT_VARIABLE, snapshot_folder, take_snapshot};
use repo_tools::records_save::{
    Kind, LAST_SAVE, ROOT_VARIABLE, RecordCopy, copy_file, copy_folder, record_copies,
};

/// A SQLite database copied consistently by `VACUUM INTO`, the source opened read-only.
fn vacuum_into(from: &Path, to: &Path) -> Result<(), String> {
    let _ = fs::remove_file(to);

    let target = to.to_string_lossy().replace('\'', "''");
    let output = Command::new("sqlite3")
        .arg("-readonly")
        .arg(from)
        .arg(format!("VACUUM INTO '{target}'"))
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("sqlite3: {error}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "sqlite3 {}: {}",
            from.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Copies one record; false when its source is missing.
fn save(copy: &RecordCopy, folder: &Path) -> Result<bool, String> {
    if !copy.from.exists() {
        return Ok(false);
    }

    let to = folder.join(copy.to);
    let parent = to.parent().unwrap_or(folder);
    let failed = |error: std::io::Error| format!("{}: {error}", copy.from.display());

    fs::create_dir_all(parent).map_err(failed)?;
    match copy.kind {
        Kind::Database => vacuum_into(&copy.from, &to)?,
        Kind::Folder => copy_folder(&copy.from, &to).map_err(failed)?,
        Kind::File => copy_file(&copy.from, &to).map_err(failed)?,
    }
    Ok(true)
}

fn run() -> Result<(), String> {
    let (main, origin) = main_and_origin(Path::new("."))?;
    let folder = project_records(env::var(ROOT_VARIABLE).ok().as_deref(), &main, &origin)?;

    fs::create_dir_all(&folder).map_err(|error| format!("{}: {error}", folder.display()))?;

    let home = env::var("HOME").unwrap_or_default();
    let library =
        library_file(env::var("AGENT_LIBRARY_FILE").ok(), Some(home.clone())).unwrap_or_default();

    for copy in record_copies(&main, Path::new(&home), &library) {
        let saved = save(&copy, &folder)?;

        println!("{} {}", if saved { "saved" } else { "missing" }, copy.to);
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let instant = temporal_instant(
        i64::try_from(now.as_secs()).unwrap_or(0),
        now.subsec_nanos(),
    );

    fs::write(folder.join(LAST_SAVE), format!("{instant}\n"))
        .map_err(|error| format!("{}: {error}", folder.join(LAST_SAVE).display()))?;

    let snapshot = match snapshot_folder(env::var(SNAPSHOT_VARIABLE).ok().as_deref())? {
        Some(synced) => take_snapshot(&folder.join(DATABASE_FILE), &synced, &calendar::today())?,
        None => NOT_SET.to_owned(),
    };

    println!("{snapshot}");
    println!("Saved in {}.", folder.display());
    Ok(())
}

fn work() -> u8 {
    match run() {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("records-save: {error}");
            1
        }
    }
}

fn main() -> ExitCode {
    tool_use::run("records-save", "", work)
}
