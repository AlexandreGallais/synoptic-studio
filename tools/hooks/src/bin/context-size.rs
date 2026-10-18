//! `npm run context:size -- <folder>…` (EN-065): what each subagent carried at its first request.
//! A folder is a session's (its `subagents/` read) or a `subagents/` folder itself; the agents are
//! listed in their files' order, then the median of each type.

use std::fs;
use std::path::{Path, PathBuf};

use hooks::context::{Start, first_request, tables};
use hooks::json::{self, Value};

/// The subagent transcripts of a folder, sorted by name.
fn transcripts(folder: &Path) -> Vec<PathBuf> {
    let subagents = folder.join("subagents");
    let folder = if subagents.is_dir() {
        subagents
    } else {
        folder.to_path_buf()
    };
    let mut files: Vec<PathBuf> = fs::read_dir(folder)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| {
                    path.extension()
                        .is_some_and(|extension| extension == "jsonl")
                        && path
                            .file_name()
                            .is_some_and(|name| name.to_string_lossy().starts_with("agent-"))
                })
                .collect()
        })
        .unwrap_or_default();

    files.sort();
    files
}

/// What one subagent started with, from its transcript and the `meta.json` beside it.
fn start_of(transcript: &Path) -> Option<Start> {
    let meta = fs::read_to_string(transcript.with_extension("meta.json"))
        .ok()
        .and_then(|text| json::parse(&text))
        .unwrap_or(Value::Null);

    first_request(&fs::read_to_string(transcript).ok()?, &meta)
}

fn main() -> std::process::ExitCode {
    tool_use::run("context-size", "", sizes)
}

/// Prints the tables of the folders named; the exit code.
fn sizes() -> u8 {
    let folders: Vec<String> = std::env::args().skip(1).collect();

    if folders.is_empty() {
        eprintln!("Usage: npm run context:size -- <session or subagents folder>…");
        return 2;
    }

    let starts: Vec<Start> = folders
        .iter()
        .flat_map(|folder| transcripts(Path::new(folder)))
        .filter_map(|transcript| start_of(&transcript))
        .collect();

    print!("{}", tables(&starts));
    0
}
