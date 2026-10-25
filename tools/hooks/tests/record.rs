//! Tests of the tool call records and of the `track-tool-call` and `log-tool-call` hooks (EN-104),
//! the cases of `.claude/hooks/log-tool-call.test.ts` and `status-place.test.ts` carried over.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use hooks::json::{Value, js_length, number_text, parse, quote, write};
use hooks::record::{
    CallContext, is_call_of, js_prefix, relative, running_call, session_moment, tool_call_record,
    tool_summary,
};

/// Fields of a hook input for a shell command that succeeded, as documented by `REF-CC-HOOKS`.
const BASH: &str = r#""cwd": "/repo", "duration_ms": 4187, "session_id": "abc123", "tool_name": "Bash",
  "tool_input": {"command": "npm run check", "description": "Run the checks"},
  "tool_use_id": "toolu_01""#;

/// A parsed hook input from its members.
fn input(fields: &str) -> Value {
    parse(&format!("{{{fields}}}")).expect("JSON")
}

/// Where and when the call ends: the hook ran at 2026-10-12T21:00:00Z, no snapshot.
fn context() -> CallContext {
    CallContext {
        branch: "develop".into(),
        snapshot: Value::Null,
        started_at: 0.0,
        time: 1_791_838_800_000,
    }
}

/// The session's members of a record without a snapshot.
const NO_MOMENT: &str = r#""contextPercent":0,"model":"","pendingAgents":0,"pendingCommands":0,"snapshotAgeMs":0,"turnElapsedMs":0,"turnOutputTokens":0,"turnStart":0,"turnTokens":0"#;

#[test]
fn records_a_successful_command_with_its_duration() {
    let record = tool_call_record(
        &input(&format!("{BASH}, \"hook_event_name\": \"PostToolUse\"")),
        &context(),
    );

    // {"command":"npm run check","description":"Run the checks"}: 58 characters.
    assert_eq!(
        record,
        format!(
            "{{{NO_MOMENT},\"agent\":\"main\",\"agentId\":\"\",\"branch\":\"develop\",\"description\":\"Run the checks\",\"durationMs\":4187,\"error\":\"\",\"inputChars\":58,\"isBackground\":false,\"isOk\":true,\"outputChars\":0,\"permissionMode\":\"\",\"session\":\"abc123\",\"startedAt\":0,\"summary\":\"npm run check\",\"time\":\"2026-10-12T21:00:00Z\",\"tool\":\"Bash\",\"toolUseId\":\"toolu_01\"}}"
        )
    );
    assert!(parse(&record).is_some());
}

#[test]
fn copies_the_session_s_state_from_the_status_line_s_snapshot() {
    let snapshot = parse(
        r#"{"contextPercent": 42, "model": "Opus 5.5", "pendingAgents": 1, "pendingCommands": 2, "time": 9000, "turnOutputTokens": 300, "turnStart": 4000, "turnTokens": 5000}"#,
    )
    .expect("JSON");

    // Its age 10 000 − 9 000 = 1 000; the turn's time 10 000 − 4 000 = 6 000.
    assert_eq!(
        session_moment(&snapshot, 10_000),
        r#""contextPercent":42,"model":"Opus 5.5","pendingAgents":1,"pendingCommands":2,"snapshotAgeMs":1000,"turnElapsedMs":6000,"turnOutputTokens":300,"turnStart":4000,"turnTokens":5000"#
    );
    assert_eq!(session_moment(&Value::Null, 10_000), NO_MOMENT);
}

#[test]
fn records_a_call_in_the_background_its_start_result_size_and_subagent_id() {
    let record = tool_call_record(
        &input(
            r#""tool_name": "Agent", "agent_id": "ag1", "permission_mode": "auto",
      "tool_input": {"subagent_type": "auditor"}, "tool_response": "ok""#,
        ),
        &CallContext {
            started_at: 7.0,
            ..context()
        },
    );
    let parsed = parse(&record).expect("JSON");

    assert_eq!(parsed.get("agentId").text(), "ag1");
    assert_eq!(parsed.get("isBackground"), &Value::Bool(true));
    // "ok" written as JSON: 4 characters.
    assert_eq!(parsed.get("outputChars"), &Value::Number(4.0));
    assert_eq!(parsed.get("permissionMode").text(), "auto");
    assert_eq!(parsed.get("startedAt"), &Value::Number(7.0));
    // No step: the agent writes no step file any more.
    assert_eq!(parsed.get("step"), &Value::Null);
    // A monitor and a command sent to the background run there too.
    for (tool, tool_input) in [
        ("Monitor", "{}"),
        ("Bash", r#"{"run_in_background": true}"#),
    ] {
        let line = tool_call_record(
            &input(&format!(
                "\"tool_name\": \"{tool}\", \"tool_input\": {tool_input}"
            )),
            &context(),
        );

        assert!(line.contains("\"isBackground\":true"), "{tool}");
    }
}

#[test]
fn records_a_failure_with_the_first_line_of_its_error_and_the_subagent_that_ran_it() {
    let record = tool_call_record(
        &input(&format!(
            "{BASH}, \"hook_event_name\": \"PostToolUseFailure\", \"agent_type\": \"auditor\", \"error\": \"Exit code 1\\nError: Cannot find module\""
        )),
        &context(),
    );
    let parsed = parse(&record).expect("JSON");

    assert_eq!(parsed.get("agent").text(), "auditor");
    assert_eq!(parsed.get("error").text(), "Exit code 1");
    assert_eq!(parsed.get("isOk"), &Value::Bool(false));
}

#[test]
fn leaves_out_a_missing_duration() {
    let record = tool_call_record(
        &input(r#""tool_name": "Bash", "tool_input": {"command": "ls"}"#),
        &context(),
    );

    assert!(!record.contains("durationMs"));
    // No input nor result at all: 0 characters each.
    assert!(tool_call_record(&input(""), &context()).contains("\"inputChars\":0,"));
}

#[test]
fn summarizes_each_kind_of_tool_by_what_it_acted_on() {
    assert_eq!(
        tool_summary("Read", &input(r#""file_path": "/repo/src/a.ts""#), "/repo"),
        "src/a.ts"
    );
    assert_eq!(
        tool_summary("Grep", &input(r#""pattern": "foo""#), "/repo"),
        "foo"
    );
    assert_eq!(
        tool_summary("WebFetch", &input(r#""url": "https://x.org""#), "/repo"),
        "https://x.org"
    );
    assert_eq!(
        tool_summary(
            "Agent",
            &input(r#""description": "audit", "subagent_type": "auditor""#),
            ""
        ),
        "auditor: audit"
    );
    assert_eq!(tool_summary("Other", &input(""), "/repo"), "");
    // An absolute path outside the folder, and a path without a folder to shorten it against.
    assert_eq!(
        tool_summary("Read", &input(r#""file_path": "/etc/hosts""#), "/repo/src"),
        "../../etc/hosts"
    );
    assert_eq!(
        tool_summary("Read", &input(r#""file_path": "/etc/hosts""#), ""),
        "/etc/hosts"
    );
}

#[test]
fn cuts_a_long_command_and_keeps_its_length() {
    let summary = tool_summary(
        "Bash",
        &input(&format!("\"command\": \"echo {}\"", "a".repeat(400))),
        "/repo",
    );

    // "echo " and 295 a: 300 characters kept of 405.
    assert_eq!(
        summary,
        format!("echo {}… (405 characters)", "a".repeat(295))
    );
    // White space runs become one space, line feeds and tabs too.
    assert_eq!(
        tool_summary("Bash", &input("\"command\": \"a \\n\\t b\""), ""),
        "a b"
    );
}

#[test]
fn counts_and_cuts_as_javascript_does_in_utf_16() {
    // "😀" is two UTF-16 units: 299 a and it make 301, cut before it.
    let text = format!("{}😀", "a".repeat(299));

    assert_eq!(js_length(&text), 301);
    assert_eq!(js_prefix(&text, 300), "a".repeat(299));
    assert_eq!(js_prefix("abc", 300), "abc");
}

#[test]
fn shortens_a_path_as_node_js_does() {
    assert_eq!(relative("/repo", "/repo/src/a.ts"), "src/a.ts");
    assert_eq!(relative("/repo/a", "/repo/b/c"), "../b/c");
    assert_eq!(relative("/repo/", "/repo"), "");
    assert_eq!(relative("/repo/./x/..", "/repo/y"), "y");
}

#[test]
fn writes_numbers_as_javascript_does() {
    for (number, text) in [
        (0.0, "0"),
        (-0.0, "0"),
        (4187.0, "4187"),
        (34.4, "34.4"),
        (-2.5, "-2.5"),
        (0.000_001, "0.000001"),
        (1.5e-7, "1.5e-7"),
        (1e21, "1e+21"),
        (1.25e21, "1.25e+21"),
        (123_456_789_012_345_680_000.0, "123456789012345680000"),
        (f64::NAN, "null"),
    ] {
        assert_eq!(number_text(number), text, "{number}");
    }
    // Escapes as `JSON.stringify` writes them.
    assert_eq!(quote("\u{8}\u{c}\u{1f}"), r#""\b\f\u001f""#);
    assert_eq!(
        write(&parse(r#"{"a":[1e21,0.5,"x"],"b":null,"c":true}"#).expect("JSON")),
        r#"{"a":[1e+21,0.5,"x"],"b":null,"c":true}"#
    );
}

#[test]
fn records_a_call_starting_as_the_status_line_reads_it() {
    let start = input(
        r#""tool_name": "Bash", "cwd": "/repo", "tool_input": {"command": "npm run check", "description": "Run the checks"}"#,
    );

    assert_eq!(
        running_call(&start, 1000),
        r#"{"agent":"main","agentId":"","description":"Run the checks","session":"","start":1000,"summary":"npm run check","tool":"Bash"}"#
    );
}

#[test]
fn f32_ac1_records_the_subagent_and_the_session_of_a_call_starting() {
    // The call EN-119's worker made at 2026-10-18T23:59:34Z, refused by `guard-bash`: no
    // `PostToolUse` came, and its file stayed in `running/` (C5).
    let start = input(
        r#""tool_name": "Bash", "agent_id": "a119885dcbc2e3522", "agent_type": "story-worker", "session_id": "e149182b", "tool_input": {"command": "git log -3", "description": "Read tool incidents file"}"#,
    );

    assert_eq!(
        running_call(&start, 1_792_367_974_686),
        r#"{"agent":"story-worker","agentId":"a119885dcbc2e3522","description":"Read tool incidents file","session":"e149182b","start":1792367974686,"summary":"git log -3","tool":"Bash"}"#
    );
}

#[test]
fn f32_ac1_tells_the_calls_of_an_ended_subagent_by_its_id() {
    let record = parse(r#"{"agent":"story-worker","agentId":"a1","start":5}"#).expect("JSON");

    assert!(is_call_of(&record, "a1"));
    assert!(!is_call_of(&record, "a2"));
    // The main agent's call, and a file written before EN-120, name no subagent: never one's.
    assert!(!is_call_of(
        &parse(r#"{"agent":"main","agentId":""}"#).expect("JSON"),
        ""
    ));
    assert!(!is_call_of(
        &parse(r#"{"agent":"story-worker"}"#).expect("JSON"),
        ""
    ));
}

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself (`.claude/rules/tooling.md`); started through the
/// shared `test_support::command` (EN-103), with a home of its own.
fn clean(program: &str) -> Command {
    let mut command = test_support::command(program);
    let home = std::env::temp_dir().join(format!("record-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
    command.env("HOME", home).env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

/// A repository of its own on branch `chore/en-1-x`.
fn repository(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("record-{name}-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("folder");
    let status = clean("git")
        .args(["init", "-q", "-b", "chore/en-1-x"])
        .current_dir(&root)
        .status()
        .expect("git runs");

    assert!(status.success());
    root
}

/// A hook binary run in a folder on an input.
fn run(binary: &str, folder: &Path, input: &str) -> Output {
    let mut child = clean(binary)
        .current_dir(folder)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("hook runs");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input written");
    child.wait_with_output().expect("hook ends")
}

#[test]
fn tracks_a_call_then_logs_it_and_removes_its_running_file() {
    let root = repository("hooks");
    let metrics = root.join(".git/metrics");
    let start = format!(
        "{{\"tool_name\":\"Bash\",\"tool_use_id\":\"toolu_9\",\"cwd\":{},\"tool_input\":{{\"command\":\"ls\"}}}}",
        quote(&root.to_string_lossy())
    );

    assert!(
        run(env!("CARGO_BIN_EXE_track-tool-call"), &root, &start)
            .status
            .success()
    );
    let running = fs::read_to_string(metrics.join("running/toolu_9.json")).expect("running file");
    let started = parse(&running).expect("JSON");

    assert_eq!(started.get("summary").text(), "ls");

    fs::write(metrics.join("now.json"), r#"{"model":"Opus 5.5","time":5}"#).expect("now");
    let end = start.replacen('{', "{\"hook_event_name\":\"PostToolUse\",", 1);

    assert!(
        run(env!("CARGO_BIN_EXE_log-tool-call"), &root, &end)
            .status
            .success()
    );
    let log = fs::read_to_string(metrics.join("tool-calls.jsonl")).expect("log");
    let record = parse(log.trim_end()).expect("one JSON line");

    assert!(log.ends_with('\n'));
    assert_eq!(record.get("branch").text(), "chore/en-1-x");

    assert_eq!(record.get("model").text(), "Opus 5.5");
    assert_eq!(record.get("startedAt"), started.get("start"));
    assert!(!metrics.join("running/toolu_9.json").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn f32_ac1_removes_the_running_files_of_a_subagent_when_it_stops() {
    let root = repository("stop");
    let metrics = root.join(".git/metrics");
    let start = |id: &str, agent: &str| {
        format!(
            "{{\"tool_name\":\"Bash\",\"tool_use_id\":\"{id}\",\"agent_id\":\"{agent}\",\"agent_type\":\"story-worker\",\"session_id\":\"s\",\"cwd\":{},\"tool_input\":{{\"command\":\"git log\"}}}}",
            quote(&root.to_string_lossy())
        )
    };

    // Two calls of `a1`, refused before they ran so that no `PostToolUse` removed them, and one of
    // `a2`, still running.
    for (id, agent) in [("toolu_1", "a1"), ("toolu_2", "a1"), ("toolu_3", "a2")] {
        assert!(
            run(
                env!("CARGO_BIN_EXE_track-tool-call"),
                &root,
                &start(id, agent)
            )
            .status
            .success()
        );
    }
    let stop = format!(
        "{{\"hook_event_name\":\"SubagentStop\",\"session_id\":\"s\",\"agent_id\":\"a1\",\"agent_type\":\"story-worker\",\"cwd\":{}}}",
        quote(&root.to_string_lossy())
    );

    assert!(
        run(env!("CARGO_BIN_EXE_log-tool-call"), &root, &stop)
            .status
            .success()
    );
    assert!(!metrics.join("running/toolu_1.json").exists());
    assert!(!metrics.join("running/toolu_2.json").exists());
    assert!(metrics.join("running/toolu_3.json").exists());
    // A subagent's end is no tool call: nothing logged.
    assert!(!metrics.join("tool-calls.jsonl").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn names_a_call_without_id_as_javascript_did_and_fails_on_what_is_not_json() {
    let root = repository("no-id");

    assert!(
        run(
            env!("CARGO_BIN_EXE_track-tool-call"),
            &root,
            "{\"tool_name\":\"Read\"}"
        )
        .status
        .success()
    );
    assert!(root.join(".git/metrics/running/undefined.json").exists());
    for binary in [
        env!("CARGO_BIN_EXE_track-tool-call"),
        env!("CARGO_BIN_EXE_log-tool-call"),
    ] {
        let output = run(binary, &root, "not json");

        assert_eq!(output.status.code(), Some(1), "{binary}");
        assert_eq!(output.stdout, [] as [u8; 0]);
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn logs_nothing_outside_a_repository() {
    let folder = std::env::temp_dir().join(format!("record-outside-{}", std::process::id()));

    fs::create_dir_all(&folder).expect("folder");
    let input = format!("{{\"cwd\":{}}}", quote(&folder.to_string_lossy()));
    let output = clean(env!("CARGO_BIN_EXE_log-tool-call"))
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir())
        .current_dir(&folder)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .take()
                .expect("stdin")
                .write_all(input.as_bytes())?;
            child.wait_with_output()
        })
        .expect("hook runs");

    assert!(output.status.success());
    assert!(!folder.join(".git").exists());
    let _ = fs::remove_dir_all(&folder);
}
