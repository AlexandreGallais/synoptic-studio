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

#[test]
fn f21_ac7_every_binary_of_the_workspace_records_its_use_under_its_name() {
    let binaries = binaries();
    // The 18 binaries of 2026-10-18: a crate's binary is never lost from the count unseen.
    assert!(binaries.len() >= 18, "{binaries:?}");

    let missing: Vec<String> = binaries
        .iter()
        .filter(|(name, path)| {
            let source = fs::read_to_string(path).expect("binary source");
            !source.contains(&format!("tool_use::run(\"{name}\","))
                && !source.contains(&format!("tool_use::run_or_poll(\"{name}\","))
        })
        .map(|(name, path)| format!("{name} ({})", path.display()))
        .collect();

    assert!(
        missing.is_empty(),
        "binaries whose main does not call tool_use::run under their name: {missing:?}"
    );
}
