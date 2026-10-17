//! Writes the API pages of the documentation site (EN-072).
//!
//! Usage: `api-pages <entry module> <output folder> <site path> <title>`, as `npm run docs:api`
//! runs it: `api-pages src/index.ts docs/api /api/ editor`. Writes nothing and fails, naming
//! each, when an export cannot be read or fails a check.

use std::path::Path;
use std::process::ExitCode;

use api_pages::{Output, api, pages, problems, write};

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [entry, folder, site_path, title] = arguments.as_slice() else {
        eprintln!("usage: api-pages <entry module> <output folder> <site path> <title>");
        return ExitCode::from(2);
    };

    let api = api::read(Path::new(entry));
    let problems = problems(&api);
    if !problems.is_empty() {
        for problem in &problems {
            eprintln!("{problem}");
        }
        eprintln!("api-pages: {} problem(s), no page written", problems.len());
        return ExitCode::FAILURE;
    }

    let output = Output {
        folder: Path::new(folder),
        site_path,
        title,
    };
    let pages = pages(&api.exports, &output);
    if let Err(error) = write(&pages, output.folder) {
        eprintln!("api-pages: cannot write {folder}: {error}");
        return ExitCode::FAILURE;
    }

    println!("api-pages: {} pages written to {folder}", pages.len() - 2);
    ExitCode::SUCCESS
}
