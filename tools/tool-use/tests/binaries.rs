//! Every binary of the workspace records its use under its own name (EN-102): its `main` hands its
//! work to `tool_use::run("<binary>", …)`, so that a tool added later without it fails here.
//! `[F21.AC7]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};

/// The `tools/` folder of the workspace.
fn tools() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools/")
        .to_path_buf()
}

/// Each binary's name and source: `src/main.rs` named after its crate, `src/bin/<name>.rs`.
fn binaries() -> Vec<(String, PathBuf)> {
    let mut found = Vec::new();

    for entry in fs::read_dir(tools()).expect("tools/") {
        let crate_folder = entry.expect("a crate").path();
        let main = crate_folder.join("src/main.rs");
        if main.is_file() {
            let name = crate_folder
                .file_name()
                .expect("crate name")
                .to_string_lossy()
                .into_owned();
            found.push((name, main));
        }
        if let Ok(bins) = fs::read_dir(crate_folder.join("src/bin")) {
            for bin in bins {
                let path = bin.expect("a binary").path();
                if path.extension().is_some_and(|extension| extension == "rs") {
                    let name = path
                        .file_stem()
                        .expect("binary name")
                        .to_string_lossy()
                        .into_owned();
                    found.push((name, path));
                }
            }
        }
    }
    found.sort();
    found
}

/// Whether a binary's source calls `tool_use::run` or `run_or_poll` under its name, in its code,
/// however the formatter breaks its lines: a comment naming the call does not count (AUD-023).
fn calls_run(source: &str, name: &str) -> bool {
    let code: String = source
        .lines()
        .map(|line| line.split_once("//").map_or(line, |(code, _)| code))
        .flat_map(str::chars)
        .filter(|character| !character.is_whitespace())
        .collect();

    code.contains(&format!("tool_use::run(\"{name}\","))
        || code.contains(&format!("tool_use::run_or_poll(\"{name}\","))
}

#[test]
fn reads_the_call_in_the_code_never_in_a_comment() {
    assert!(calls_run(
        "fn main() {\n    tool_use::run(\"page\", \"\", work)\n}",
        "page"
    ));
    assert!(!calls_run(
        "// tool_use::run(\"page\", \"\", work)\nfn main() {}",
        "page"
    ));
    assert!(!calls_run(
        "fn main() {} // tool_use::run(\"page\",",
        "page"
    ));
    assert!(!calls_run("tool_use::run(\"pages\", \"\", work)", "page"));
    assert!(calls_run(
        "tool_use::run_or_poll(\n    \"page\",\n    \"\",",
        "page"
    ));
}

#[test]
fn f21_ac7_every_binary_of_the_workspace_records_its_use_under_its_name() {
    let binaries = binaries();
    // The 43 binaries of 2026-10-19, `bench` the last (AUD-023, EN-141): one added or lost changes
    // the count, which is then written again here.
    assert_eq!(binaries.len(), 43, "{binaries:?}");

    let missing: Vec<String> = binaries
        .iter()
        .filter(|(name, path)| !calls_run(&fs::read_to_string(path).expect("binary source"), name))
        .map(|(name, path)| format!("{name} ({})", path.display()))
        .collect();

    assert!(
        missing.is_empty(),
        "binaries whose main does not call tool_use::run under their name: {missing:?}"
    );
}
