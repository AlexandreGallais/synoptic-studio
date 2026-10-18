//! `page [<file>] <id or heading>` (`npm run -s page`, EN-105): prints one row or one section of
//! a repository page rather than the whole page. With a file: the rows whose first cell is the id,
//! each preceded by its table's header line; else the sections under that heading, each preceded
//! by `<file>:<line>`. Without a file: a `REF-` id's row in `docs/references.md`, or the first
//! section of an ADR's, a derivation's or a backlog item's file, preceded by `<file>:1`. Exits 1,
//! naming the file, when nothing is found; 2 on a wrong call. Paths are read from the working
//! folder, the repository's root under `npm run`.

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use page::locate::{Home, home};
use page::section::{first_section, sections};
use page::table::rows;

/// A failure: its message and its exit code.
type Failure = (String, u8);

fn read(file: &Path) -> Result<String, Failure> {
    fs::read_to_string(file).map_err(|error| (format!("{}: {error}", file.display()), 2))
}

/// The rows of `id` in `file`, or its sections under the heading `wanted`.
fn in_page(file: &Path, wanted: &str) -> Result<String, Failure> {
    let text = read(file)?;
    let found = rows(&text, wanted);

    if !found.is_empty() {
        return Ok(found.join("\n"));
    }

    let parts: Vec<String> = sections(&text, wanted)
        .into_iter()
        .map(|section| format!("{}:{}\n{}", file.display(), section.line, section.text))
        .collect();

    if parts.is_empty() {
        return Err((
            format!(
                "{wanted}: no row starting with it nor heading of it in {}",
                file.display()
            ),
            1,
        ));
    }

    Ok(parts.join("\n"))
}

/// The row or the first section of `id`, in the page that holds it.
fn by_id(id: &str) -> Result<String, Failure> {
    match home(Path::new("."), id).map_err(|message| (message, 1))? {
        Home::Row(file) => {
            let found = rows(&read(&file)?, id);

            if found.is_empty() {
                return Err((
                    format!("{id}: no row starting with it in {}", file.display()),
                    1,
                ));
            }

            Ok(found.join("\n"))
        }
        Home::Page(file) => Ok(format!(
            "{}:1\n{}",
            file.display(),
            first_section(&read(&file)?)
        )),
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = env::args().skip(1).collect();

    tool_use::run("page", "", || print_part(&words))
}

/// Prints the row or section asked for; the exit code.
fn print_part(words: &[String]) -> u8 {
    let result = match words {
        [id] => by_id(id),
        [file, wanted] => in_page(Path::new(file), wanted),
        _ => Err((
            "usage: page [<file>] <id or heading>, as npm run -s page -- docs/tooling/agent.md Hooks"
                .to_owned(),
            2,
        )),
    };

    match result {
        Ok(text) => {
            print!("{text}");
            0
        }
        Err((message, code)) => {
            eprintln!("{message}");
            code
        }
    }
}
