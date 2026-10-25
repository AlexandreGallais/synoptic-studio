//! Writes the stories index, `stories/README.md`, from each story's front matter (EN-049,
//! ADR-0065 decision 3), so that no branch edits it by hand.
//!
//! Usage: `backlog-index <backlog folder>` writes the index; `backlog-index <backlog folder>
//! --check` writes nothing and fails when the index differs from the story files, padding
//! aside. The table is written without padding, as the formatter writes it (ADR-0068), so the
//! script `backlog:index` runs this tool alone and npm passes its arguments to it: `npm run
//! backlog:index -- --check` reaches the check (EN-140).

mod order;
mod story;
mod table;

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::process::ExitCode;

use backlog_index::front_matter::front_matter;
use order::{Parent, ordered};
use story::Story;
use table::{normalized, render};

/// The Markdown files of a backlog folder, its index and templates left out: (name, text).
fn items(folder: &Path) -> io::Result<Vec<(String, String)>> {
    let mut items = Vec::new();

    for entry in fs::read_dir(folder)? {
        let name = entry?.file_name().to_string_lossy().into_owned();

        if Path::new(&name).extension() == Some(OsStr::new("md"))
            && name != "README.md"
            && !name.starts_with('_')
        {
            let text = fs::read_to_string(folder.join(&name))?;
            items.push((name, text));
        }
    }

    Ok(items)
}

/// The parents of one folder, epics or features, from their front matter.
fn parents(folder: &Path) -> io::Result<Vec<Parent>> {
    Ok(items(folder)?
        .into_iter()
        .map(|(_, text)| {
            let fields = front_matter(&text);
            let field = |key: &str| fields.get(key).cloned().unwrap_or_default();

            Parent {
                id: field("id"),
                epic: field("epic"),
                text,
            }
        })
        .collect())
}

/// The index the story files give, unformatted.
fn generated(backlog: &Path) -> io::Result<String> {
    let stories: Vec<Story> = items(&backlog.join("stories"))?
        .iter()
        .map(|(name, text)| Story::from_fields(name, &front_matter(text)))
        .collect();
    let epics = parents(&backlog.join("epics"))?;
    let features = parents(&backlog.join("features"))?;

    Ok(render(&ordered(&stories, &epics, &features)))
}

/// Writes the index, or with `--check` compares it with the one written.
fn run(backlog: &Path, check: bool) -> io::Result<Result<(), String>> {
    let index = backlog.join("stories").join("README.md");
    let expected = generated(backlog)?;

    if !check {
        fs::write(&index, expected)?;
        return Ok(Ok(()));
    }

    let written = fs::read_to_string(&index)?;
    let (want, have) = (normalized(&expected), normalized(&written));

    if want == have {
        return Ok(Ok(()));
    }

    let line = want
        .iter()
        .zip(&have)
        .position(|(left, right)| left != right)
        .unwrap_or(want.len().min(have.len()));

    Ok(Err(format!(
        "{} differs from the story files at row {}: expected \"{}\", found \"{}\". \
         It is generated: run `npm run backlog:index`, never edit it by hand.",
        index.display(),
        line + 1,
        want.get(line).map_or("", String::as_str),
        have.get(line).map_or("", String::as_str)
    )))
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: backlog-index <backlog folder> [--check]";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    let check = args.iter().any(|arg| arg == "--check");

    tool_use::run("backlog-index", if check { "--check" } else { "" }, || {
        index(&args, check)
    })
}

/// Writes or checks the stories index; its exit code.
fn index(args: &[String], check: bool) -> u8 {
    let folder = args.iter().find(|arg| !arg.starts_with("--"));

    let Some(folder) = folder else {
        eprintln!("usage: backlog-index <backlog folder> [--check]");
        return 2;
    };

    match run(Path::new(folder), check) {
        Ok(Ok(())) => 0,
        Ok(Err(message)) => {
            eprintln!("{message}");
            1
        }
        Err(error) => {
            eprintln!("backlog-index: {folder}: {error}");
            2
        }
    }
}
