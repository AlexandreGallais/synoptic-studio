//! `replace` (`target/release/replace`, RET-002, EN-103): exact text replacements in several files at
//! once, the blocks read from standard input, from the repository's root. Nothing is written unless
//! every block is read and every old text is found exactly once; then it prints `<n> edits in <m>
//! files`. Otherwise each problem, then `nothing written`, on standard error, and it exits 1.

use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

use repo_tools::replace::{Edit, apply_edits, files_of, parse_edits};

/// The new text of each file, or the problems met, one line per file.
fn results(edits: &[Edit]) -> Result<Vec<(String, String)>, Vec<String>> {
    let mut written = Vec::new();
    let mut problems = Vec::new();

    for file in files_of(edits) {
        let own: Vec<&Edit> = edits.iter().filter(|edit| edit.file == file).collect();
        let outcome = fs::read_to_string(file)
            .map_err(|error| format!("cannot be read: {error}"))
            .and_then(|text| apply_edits(&text, &own));

        match outcome {
            Ok(text) => written.push((file.to_owned(), text)),
            Err(problem) => problems.push(format!("{file}: {problem}")),
        }
    }

    if problems.is_empty() {
        Ok(written)
    } else {
        Err(problems)
    }
}

/// Reports the problems and fails the command.
fn refuse(problems: &[String]) -> u8 {
    eprintln!("{}\nnothing written", problems.join("\n"));
    1
}

fn work() -> u8 {
    let mut spec = String::new();

    if let Err(error) = io::stdin().read_to_string(&mut spec) {
        return refuse(&[format!("standard input: {error}")]);
    }

    let edits = match parse_edits(&spec) {
        Ok(edits) if edits.is_empty() => return refuse(&["no edit block read".to_owned()]),
        Ok(edits) => edits,
        Err(problem) => return refuse(&[problem]),
    };

    match results(&edits) {
        Ok(written) => {
            for (file, text) in &written {
                if let Err(error) = fs::write(file, text) {
                    eprintln!("{file}: {error}");
                    return 1;
                }
            }
            println!("{} edits in {} files", edits.len(), written.len());
            0
        }
        Err(problems) => refuse(&problems),
    }
}

fn main() -> ExitCode {
    tool_use::run("replace", "", work)
}
