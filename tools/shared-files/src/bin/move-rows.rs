//! `move-rows <story branch>` (ADR-0065, decision 4; EN-050; ADR-0091): run by the main thread
//! from the root of the story's worktree before its `done` commit, which holds the files it
//! prints. It moves the rows written under the known headings of the story file's `## For the
//! main thread` into their logs, and prints each file it wrote, one per line: nothing when there
//! were no rows. A log's rows are its own shape only — a dated table row for `improvements.md` and
//! `feedback.md`, a bullet opening with its tag (``- `#run` …``) for the journal, under a section
//! dated today, `## <today> — Run continued`, opened when its newest section is older (AUD-020),
//! a card for `tool-incidents.md`, its dated `#### ` heading and its bullets, written at the log's
//! end under `## ` (ADR-0085); a placeholder (`- None.`), a table's header and its separator are
//! dropped, and any other line is left in the story file and named on standard error, never
//! copied. The notes for the main thread, the section's lines outside a log's heading, stay in the
//! story file, each named on standard error with its story (EN-140).
//!
//! `move-rows --notes <story branch>`, run by the `commit-msg` hook of a `done` commit, writes
//! nothing and names those notes only, so that the commit shows them.

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::ExitCode;

use calendar::today;
use shared_files::backlog::{STORIES, story_file};
use shared_files::rows::{Taken, insert_rows, take_rows};
use shared_files::shared::story_of;

/// Names on standard error each note of a story file for the main thread, with its story.
fn name_notes(story: &str, taken: &Taken) {
    for note in &taken.notes {
        eprintln!("move-rows: {story}: a note for the main thread, not moved: {note}");
    }
}

/// Moves the rows of a story file into their logs; the files written. Each line under a log's
/// heading that is not one of its rows is left in the story file and named on standard error, and
/// so is each note; with `notes_only`, the notes are named and nothing is written.
fn move_rows(story_id: &str, path: &Path, notes_only: bool) -> io::Result<Vec<String>> {
    let story = fs::read_to_string(path)?;
    let taken = take_rows(&story);
    let today = today();
    let mut written = Vec::new();

    name_notes(story_id, &taken);
    if notes_only {
        return Ok(written);
    }

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
        let text = insert_rows(&fs::read_to_string(log.path)?, log.place, rows, &today)
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

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/move-rows [--notes] <story branch>";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    tool_use::run("move-rows", "", move_story_rows)
}

/// Moves the rows of the branch's story, or names its notes with `--notes`; the exit code.
fn move_story_rows() -> u8 {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let (notes_only, branch) = match arguments.as_slice() {
        [flag, branch] if flag == "--notes" => (true, branch),
        [branch] if branch != "--notes" => (false, branch),
        _ => {
            eprintln!("usage: target/release/move-rows [--notes] <story branch>");
            return 2;
        }
    };
    let Some(story) = story_of(branch) else {
        return 0;
    };
    let result = story_file(Path::new("."), &story).and_then(|file| {
        file.map_or(Ok(Vec::new()), |name| {
            move_rows(&story, &Path::new(STORIES).join(name), notes_only)
        })
    });

    match result {
        Ok(written) => {
            for path in written {
                println!("{path}");
            }
            0
        }
        Err(error) => {
            eprintln!("move-rows: {error}");
            1
        }
    }
}
