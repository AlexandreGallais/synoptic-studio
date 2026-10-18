//! Tests of `.claude/settings.json` (EN-104, ADR-0078): every hook and the status line call a
//! binary of `tools/hooks` by its path in `target/release`, never through `cargo run` nor Node.js.

use std::fs;
use std::path::Path;

use hooks::json::{Value, parse};

/// The repository's root, two folders above the crate.
fn root() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
}

/// The binaries of the crate, `src/bin/<name>.rs`.
fn binaries() -> Vec<String> {
    fs::read_dir(root().join("tools/hooks/src/bin"))
        .expect("binaries")
        .flatten()
        .filter_map(|entry| Some(entry.path().file_stem()?.to_string_lossy().into_owned()))
        .collect()
}

#[test]
fn f21_ac1_every_hook_and_the_status_line_call_a_hooks_binary_by_its_path() {
    let settings =
        parse(&fs::read_to_string(root().join(".claude/settings.json")).expect("settings"))
            .expect("JSON");
    let known = binaries();
    let mut called = Vec::new();

    for (_, entries) in match settings.get("hooks") {
        Value::Object(events) => events.clone(),
        _ => Vec::new(),
    } {
        for handler in entries
            .items()
            .iter()
            .flat_map(|entry| entry.get("hooks").items())
        {
            let name = handler
                .get("command")
                .text()
                .strip_prefix("${CLAUDE_PROJECT_DIR}/target/release/")
                .unwrap_or_default()
                .to_owned();

            // Exec form, its arguments none: the path is one word, spaces and all (`REF-CC-HOOKS`).
            assert_eq!(handler.get("args"), &Value::Array(Vec::new()), "{name}");
            assert!(known.contains(&name), "{}", handler.get("command").text());
            called.push(name);
        }
    }
    called.sort();
    called.dedup();
    assert_eq!(
        called,
        [
            "check-handback",
            "guard-bash",
            "library-pages",
            "limit-agents",
            "log-tool-call",
            "session-state",
            "track-tool-call"
        ]
    );
    assert_eq!(
        settings.get("statusLine").get("command").text(),
        "\"$(git rev-parse --show-toplevel)/target/release/status-line\""
    );
}
