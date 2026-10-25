//! `page [<file>] <id or heading>` (`target/release/page`, EN-105): prints one row or one section of
//! a repository page rather than the whole page. With a file: the rows whose first cell is the id,
//! each preceded by its table's header line; else the sections under that heading, each preceded
//! by `<file>:<line>`. Without a file: the document `knowledge show <id>` prints, every item an id
//! names living in the records (EN-152). Exits 1, naming the file or the id, when nothing is found;
//! 2 on a wrong call. Paths are read from the working folder, the repository's root under
//! `npm run`.

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use page::fallback::{knowledge_beside, shown_by};
use page::section::sections;
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

/// An id, as `knowledge show <id>` prints its document, by the `knowledge` binary beside this one.
fn from_knowledge(id: &str) -> Result<String, Failure> {
    env::current_exe()
        .ok()
        .and_then(|page| shown_by(&knowledge_beside(&page), id))
        .ok_or_else(|| {
            (
                format!("{id}: no document of it in the records (knowledge show {id})"),
                1,
            )
        })
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/page [<file>] <id or heading>";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let words: Vec<String> = env::args().skip(1).collect();

    tool_use::run("page", "", || print_part(&words))
}

/// Prints the row or section asked for; the exit code.
fn print_part(words: &[String]) -> u8 {
    let result = match words {
        [id] => from_knowledge(id),
        [file, wanted] => in_page(Path::new(file), wanted),
        _ => Err((
            "usage: page [<file>] <id or heading>, as target/release/page .agents/rules/tooling.md \"Tools of our own\""
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
