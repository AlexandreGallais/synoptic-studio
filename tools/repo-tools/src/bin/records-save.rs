//! `records-save` (`target/release/records-save`, ADR-0081, EN-114, E49), at the end of every
//! session and at every retrospective: writes into the project's records file,
//! `<AGENT_RECORDS_ROOT>/<project>.sqlite`, outside the repository and never sent anywhere, what
//! the agent's traces hold that serves — the metrics' logs of the main checkout, each session's
//! times, tokens and commands and the Product Owner's own messages from the transcripts, the
//! agent's memories — then the library of another file, while one is still named, and the
//! instant of the save. A missing source is skipped. Then it takes the day's snapshot of the
//! records into the synced folder `AGENT_KNOWLEDGE_SNAPSHOT` names, as `knowledge snapshot` does
//! (EN-153), saying so and failing nothing when the variable is not set.

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use calendar::temporal_instant;
use hooks::pages::library_file;
use knowledge::moved::Moved;
use knowledge::private_records::{
    import_library_file, import_memories, import_metrics, import_saves,
};
use knowledge::records::{main_and_origin, project_records};
use knowledge::snapshot::{NOT_SET, SNAPSHOT_VARIABLE, snapshot_folder, take_snapshot};
use knowledge::store::{open_store, write};
use knowledge::transcripts::import_transcripts;
use repo_tools::records_save::{ROOT_VARIABLE, claude_project_folder};

/// What one import printed: `<what>: <n> rows`, or `<what>: missing` without its source.
fn line(what: &str, source: &Path, import: impl FnOnce() -> Result<Moved, String>) -> String {
    if !source.exists() {
        return format!("{what}: missing");
    }
    match import() {
        Ok(moved) => format!("{what}: {} rows", moved.rows),
        Err(error) => format!("{what}: {error}"),
    }
}

fn run() -> Result<(), String> {
    let (main, origin) = main_and_origin(Path::new("."))?;
    let file = project_records(env::var(ROOT_VARIABLE).ok().as_deref(), &main, &origin)?;
    let database = open_store(&file)?;
    let home = env::var("HOME").unwrap_or_default();
    let claude = claude_project_folder(&main, Path::new(&home));
    let metrics = main.join(".git").join("metrics");
    let today = calendar::today();
    let memory = claude.join("memory");
    // The library of its own file, the variable's or the home's, while one is named.
    let library = library_file(env::var("AGENT_LIBRARY_FILE").ok(), None, Some(home))
        .filter(|library| fs::canonicalize(library).ok() != fs::canonicalize(&file).ok())
        .unwrap_or_default();

    println!(
        "{}",
        line("metrics", &metrics, || write(&database, |done| {
            import_metrics(done, &metrics)
        }))
    );
    println!(
        "{}",
        line("transcripts", &claude, || write(&database, |done| {
            import_transcripts(done, &claude)
        }))
    );
    println!(
        "{}",
        line("memories", &memory, || write(&database, |done| {
            import_memories(done, &memory)
        }))
    );
    println!(
        "{}",
        line("library", &library, || import_library_file(
            &database, &library
        ))
    );

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let instant = temporal_instant(
        i64::try_from(now.as_secs()).unwrap_or(0),
        now.subsec_nanos(),
    );

    write(&database, |done| import_saves(done, &instant))?;
    drop(database);

    let snapshot = match snapshot_folder(env::var(SNAPSHOT_VARIABLE).ok().as_deref())? {
        Some(synced) => take_snapshot(&file, &synced, &today)?,
        None => NOT_SET.to_owned(),
    };

    println!("{snapshot}");
    println!("Saved in {}.", file.display());
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
