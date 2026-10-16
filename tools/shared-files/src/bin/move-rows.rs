//! `move-rows <story branch>` (ADR-0065, decision 4; EN-050): run by `story:finish` from the
//! repository's root once the story is merged and its base pulled. It moves the rows written under
//! the known headings of the story file's `## For the main thread` into their logs, and prints each
//! file it wrote, one per line: nothing when there were no rows. A log's rows are its own shape
//! only — a dated table row for `improvements.md` and `feedback.md`, a bullet for the journal; a
//! placeholder (`- None.`), a table's header and its separator are dropped, and any other line is
//! left in the story file and named on standard error, never copied.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use shared_files::rows::{insert_rows, take_rows};
use shared_files::shared::story_of;

/// The folder of the story files.
const STORIES: &str = "docs/backlog/stories";

/// The story file of an id, `None` when there is none.
fn story_file(story: &str) -> io::Result<Option<PathBuf>> {
    let marker = format!("-{story}-");

    Ok(fs::read_dir(STORIES)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().contains(&marker))
        }))
}

/// Moves the rows of a story file into their logs; the files written. Each line under a log's
/// heading that is not one of its rows is left in the story file and named on standard error.
fn move_rows(path: &Path) -> io::Result<Vec<String>> {
    let story = fs::read_to_string(path)?;
    let taken = take_rows(&story);
    let mut written = Vec::new();

    for (log, lines) in &taken.left {
        for line in lines {
            eprintln!(
                "move-rows: left in {} under {}, not a row of {}: {line}",
                path.display(),
                log.heading,
                log.path
            );
        }
    }

    for (log, rows) in &taken.rows {
        let text = insert_rows(&fs::read_to_string(log.path)?, log.place, rows)
            .map_err(|error| io::Error::other(format!("{}: {error}", log.path)))?;

        fs::write(log.path, text)?;
        written.push(log.path.to_owned());
    }

    if taken.story != story {
        fs::write(path, &taken.story)?;
        written.push(path.display().to_string());
    }

    Ok(written)
}

fn main() -> ExitCode {
    let Some(branch) = env::args().nth(1) else {
        eprintln!("usage: move-rows <story branch>");
        return ExitCode::from(2);
    };
    let Some(story) = story_of(&branch) else {
        return ExitCode::SUCCESS;
    };
    let result =
        story_file(&story).and_then(|file| file.map_or(Ok(Vec::new()), |path| move_rows(&path)));

    match result {
        Ok(written) => {
            for path in written {
                println!("{path}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("move-rows: {error}");
            ExitCode::from(1)
        }
    }
}
