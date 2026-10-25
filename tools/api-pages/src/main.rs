//! Writes the API pages of the documentation site (EN-072).
//!
//! Usage: `api-pages <entry module> <output folder> <site path> <title>`, as the `docs:build` check of `check-all` runs it
//! runs it: `api-pages src/index.ts docs/api /api/ editor`. Writes nothing and fails, naming
//! each, when an export cannot be read or fails a check.

use std::path::Path;
use std::process::ExitCode;

use api_pages::{Output, api, pages, problems, write};

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: api-pages <entry module> <output folder> <site path> <title>";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    tool_use::run("api-pages", "", || write_pages(&arguments))
}

/// Reads the API, checks it and writes its pages; the exit code.
fn write_pages(arguments: &[String]) -> u8 {
    let [entry, folder, site_path, title] = arguments else {
        eprintln!("usage: api-pages <entry module> <output folder> <site path> <title>");
        return 2;
    };

    let api = api::read(Path::new(entry));
    let problems = problems(&api);
    if !problems.is_empty() {
        for problem in &problems {
            eprintln!("{problem}");
        }
        eprintln!("api-pages: {} problem(s), no page written", problems.len());
        return 1;
    }

    let output = Output {
        folder: Path::new(folder),
        site_path,
        title,
    };
    let pages = pages(&api.exports, &output);
    if let Err(error) = write(&pages, output.folder) {
        eprintln!("api-pages: cannot write {folder}: {error}");
        return 1;
    }

    println!("api-pages: {} pages written to {folder}", pages.len() - 2);
    0
}
