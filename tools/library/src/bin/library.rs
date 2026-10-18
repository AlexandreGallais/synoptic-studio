//! Command line of the local source library (`docs/tooling/sources.md`):
//!   `target/release/library search <words>`       sections best matching the words
//!   `target/release/library get <url> [--refresh]` table of contents, fetched when missing or stale
//!   `target/release/library read <url> <n> [<n>…]` sections of a page
//!   `target/release/library list`                 stored pages, last read first
//!   `target/release/library stats [YYYY-MM-DD]`   projects, sources consulted most, searches, share local
//!   `target/release/library path`                 the library's file, which `records-save` copies
//!
//! The first five print what `library-cli.ts` printed; an error is printed on the error output,
//! exit 1.

use std::env;
use std::process::ExitCode;

use calendar::iso_date;
use library::store::{library_file_of_process, list_pages, open_library};
use library::time::instant_ms;
use library::tools::call_tool;
use library::usage::{project_of, usage_report};
use rusqlite::Connection;
use serde_json::{Value, json};

/// The usage, printed for an unknown command.
const USAGE: &str = "Usage: target/release/library search <words> | get <url> [--refresh] | read <url> <n>… | list | stats [since] | path";

/// What a command prints, or the error it met.
fn output(database: &Connection, command: &str, rest: &[String]) -> Result<String, String> {
    let words: Vec<&str> = rest
        .iter()
        .map(String::as_str)
        .filter(|word| !word.starts_with("--"))
        .collect();
    let first = words.first().map_or(Value::Null, |url| json!(url));
    let input = match command {
        "stats" => {
            let since = match words.first() {
                Some(day) => instant_ms(&format!("{day}T00:00:00Z"))?,
                None => 0,
            };

            return usage_report(database, since).map_err(|error| format!("Error: {error}"));
        }
        "list" => {
            let pages = list_pages(database).map_err(|error| format!("Error: {error}"))?;

            return Ok(pages
                .iter()
                .map(|page| {
                    format!(
                        "{}  {}  {}",
                        iso_date(page.fetched_at),
                        page.url,
                        page.title
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"));
        }
        "get" => json!({ "refresh": rest.iter().any(|word| word == "--refresh"), "url": first }),
        "read" => json!({ "sections": words.iter().skip(1).collect::<Vec<_>>(), "url": first }),
        "search" => json!({ "query": words.join(" ") }),
        _ => return Err(USAGE.to_owned()),
    };
    let directory = env::current_dir().unwrap_or_default();

    call_tool(
        database,
        project_of(&directory.to_string_lossy()),
        &format!("library_{command}"),
        &input,
    )
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let (command, rest) = arguments
        .split_first()
        .map_or(("", &[][..]), |(command, rest)| (command.as_str(), rest));

    tool_use::run("library", command, || {
        let Some(file) = library_file_of_process() else {
            eprintln!("No library file: set AGENT_LIBRARY_FILE or HOME");
            return 1;
        };

        if command == "path" {
            println!("{}", file.display());
            return 0;
        }

        let printed = open_library(&file)
            .map_err(|error| format!("Error: {error}"))
            .and_then(|database| output(&database, command, rest));

        match printed {
            Ok(text) => {
                println!("{text}");
                0
            }
            Err(error) => {
                eprintln!("{error}");
                1
            }
        }
    })
}
