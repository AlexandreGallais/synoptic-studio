//! Tests of `.claude/settings.json` (EN-104, ADR-0078): every hook calls a binary
//! of `tools/hooks` by its path in `target/release`, never through `cargo run` nor Node.js;
//! the session's end alone calls `knowledge snapshot` (EN-153); the permissions of the tools in use only (AUD-051).

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
fn f21_ac1_every_hook_calls_a_hooks_binary_by_its_path_and_no_status_line_runs() {
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

            // Exec form, its arguments none but the snapshot's command: the path is one word,
            // spaces and all (`REF-CC-HOOKS`).
            // The end-of-session rituals, outside the hooks crate (EN-154).
            if name == "records-save" || name == "session-report" {
                assert_eq!(handler.get("args"), &Value::Array(Vec::new()), "{name}");
            } else if name == "knowledge" {
                assert_eq!(
                    handler.get("args"),
                    &Value::Array(vec![Value::String("snapshot".to_owned())])
                );
            } else {
                assert_eq!(handler.get("args"), &Value::Array(Vec::new()), "{name}");
                assert!(known.contains(&name), "{}", handler.get("command").text());
            }
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
            "knowledge",
            "library-pages",
            "limit-agents",
            "log-tool-call",
            "records-save",
            "session-report",
            "session-state",
            "track-tool-call"
        ]
    );
    // No status line: Claude Code shows by itself when the agent works (AUD-056).
    assert_eq!(settings.get("statusLine"), &Value::Null);
}

#[test]
fn aud_051_allows_no_push_without_a_prompt() {
    let settings =
        parse(&fs::read_to_string(root().join(".claude/settings.json")).expect("settings"))
            .expect("JSON");
    let pushes: Vec<&str> = settings
        .get("permissions")
        .get("allow")
        .items()
        .iter()
        .map(Value::text)
        .filter(|rule| rule.contains("git push"))
        .collect();

    // `git push --delete` runs the pre-push checks for nothing: a branch is deleted through the
    // GitHub API, which stays allowed.
    assert_eq!(pushes, [] as [&str; 0]);
}

#[test]
fn f32_ac1_runs_log_tool_call_when_any_subagent_stops() {
    let settings =
        parse(&fs::read_to_string(root().join(".claude/settings.json")).expect("settings"))
            .expect("JSON");
    let entries = settings.get("hooks").get("SubagentStop").items();
    let handlers: Vec<&Value> = entries
        .iter()
        .filter(|entry| matches!(entry.get("matcher").text(), "" | "*"))
        .flat_map(|entry| entry.get("hooks").items())
        .collect();

    // Every agent type, in the background, as on `PostToolUse`.
    assert!(handlers.iter().any(|handler| {
        handler.get("command").text() == "${CLAUDE_PROJECT_DIR}/target/release/log-tool-call"
            && handler.get("async") == &Value::Bool(true)
    }));
}

/// The snapshot's budget at a session's end, in seconds: `SessionEnd` gives its hooks 1.5 by
/// default, a hook's own `timeout` raising it up to 60 (`REF-CC-HOOKS`).
const SNAPSHOT_TIMEOUT: f64 = 30.0;

#[test]
fn f48_ac3_takes_the_snapshot_when_a_session_ends_within_its_timeout() {
    let settings =
        parse(&fs::read_to_string(root().join(".claude/settings.json")).expect("settings"))
            .expect("JSON");
    let handlers: Vec<&Value> = settings
        .get("hooks")
        .get("SessionEnd")
        .items()
        .iter()
        .filter(|entry| matches!(entry.get("matcher").text(), "" | "*"))
        .flat_map(|entry| entry.get("hooks").items())
        .collect();

    // Every reason a session ends; in the foreground, so that the copy ends before Claude Code.
    // The snapshot, then the end-of-session rituals, silently (2026-10-25, EN-154).
    assert_eq!(handlers.len(), 3);
    assert_eq!(
        handlers[1].get("command").text(),
        "${CLAUDE_PROJECT_DIR}/target/release/records-save"
    );
    assert_eq!(
        handlers[2].get("command").text(),
        "${CLAUDE_PROJECT_DIR}/target/release/session-report"
    );
    assert_eq!(
        handlers[0].get("command").text(),
        "${CLAUDE_PROJECT_DIR}/target/release/knowledge"
    );
    assert_eq!(
        handlers[0].get("args"),
        &Value::Array(vec![Value::String("snapshot".to_owned())])
    );
    assert_eq!(handlers[0].get("async"), &Value::Null);
    assert_eq!(handlers[0].get("timeout"), &Value::Number(SNAPSHOT_TIMEOUT));
}

/// The release binaries allowed without a prompt, each by name and, for a binary that also
/// publishes or deletes, by its subcommand (AUD-023): `heavy-lock` runs any command given after it,
/// `branches-clean` deletes branches and worktrees, `git-hooks install` sets the hooks folder and
/// `release publish` makes a release, so each of them asks.
const ALLOWED_BINARIES: [&str; 18] = [
    "backlog-status",
    "context-size",
    "dependencies outdated",
    "dependencies unused",
    "epic-check",
    "git-sequences commit",
    "git-sequences state",
    "git-sequences sync",
    "knowledge",
    "library",
    "metrics",
    "page",
    "release check",
    "release notes",
    "replace",
    "run-totals",
    "stale-binaries",
    "worktree-new",
];

#[test]
fn the_release_binaries_are_allowed_by_name_never_by_a_wildcard() {
    let settings =
        parse(&fs::read_to_string(root().join(".claude/settings.json")).expect("settings"))
            .expect("JSON");
    let mut allowed: Vec<&str> = settings
        .get("permissions")
        .get("allow")
        .items()
        .iter()
        .map(Value::text)
        .filter(|rule| rule.contains("target/release/"))
        .collect();
    let mut expected: Vec<String> = ALLOWED_BINARIES
        .iter()
        .map(|name| format!("Bash(target/release/{name} *)"))
        .collect();

    allowed.sort_unstable();
    expected.sort();
    assert_eq!(allowed, expected);
}

/// The matchers of the `PreToolUse` entries whose handler runs `guard-bash` failing closed, and
/// every handler of the settings with an `onFailure`, by command.
fn closed_guards(settings: &Value) -> (Vec<String>, Vec<String>) {
    let mut guarded = Vec::new();
    let mut failing_closed = Vec::new();

    for (event, entries) in match settings.get("hooks") {
        Value::Object(events) => events.clone(),
        _ => Vec::new(),
    } {
        for entry in entries.items() {
            for handler in entry.get("hooks").items() {
                let command = handler.get("command").text();

                if !handler.get("onFailure").text().is_empty() {
                    failing_closed.push(format!("{command} {}", handler.get("onFailure").text()));
                }
                if event == "PreToolUse"
                    && command == "${CLAUDE_PROJECT_DIR}/target/release/guard-bash"
                    && handler.get("onFailure").text() == "block"
                {
                    guarded.push(entry.get("matcher").text().to_owned());
                }
            }
        }
    }
    guarded.sort();
    (guarded, failing_closed)
}

#[test]
fn the_guard_that_cannot_run_blocks_the_shell_command_and_the_write() {
    let settings =
        parse(&fs::read_to_string(root().join(".claude/settings.json")).expect("settings"))
            .expect("JSON");
    let block = "${CLAUDE_PROJECT_DIR}/target/release/guard-bash block";

    // A missing or crashing binary is a non-blocking error, its call let through, unless the
    // handler's `onFailure` says `block` (`REF-CC-HOOKS`, AUD-023): only the guard does, on the
    // shell and on the writes (AUD-033), the other hooks letting a call through when they fail,
    // so that their own failure never blocks work.
    assert_eq!(
        closed_guards(&settings),
        (
            vec!["Bash".to_owned(), "Write|Edit|NotebookEdit".to_owned()],
            vec![block.to_owned(), block.to_owned()]
        )
    );
    // The writes' entry removed, or its `onFailure` dropped: the check sees it.
    let without_writes = parse(
        r#"{"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"command":"${CLAUDE_PROJECT_DIR}/target/release/guard-bash","onFailure":"block"}]},{"matcher":"Write|Edit|NotebookEdit","hooks":[{"command":"${CLAUDE_PROJECT_DIR}/target/release/guard-bash"}]}]}}"#,
    )
    .expect("JSON");

    assert_eq!(closed_guards(&without_writes).0, ["Bash"]);
}
