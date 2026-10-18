//! Tests of the agents' activity (EN-107): the cases of `scripts/activity.test.ts`, then the
//! whole listing and the background outputs found.

use std::env;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

use metrics::activity::{
    LoggedCall, activity_lines, call_line, latest_by_agent, logged_call, outputs_folder,
    recent_outputs,
};
use serde_json::json;

/// A call of the main agent, before a test changes some of it.
fn call() -> LoggedCall {
    LoggedCall {
        agent: "main".to_owned(),
        description: "List the files".to_owned(),
        duration_ms: 1234.0,
        is_ok: true,
        summary: "ls".to_owned(),
        time: "2026-10-14T21:04:05.123Z".to_owned(),
        tool: "Bash".to_owned(),
    }
}

#[test]
fn writes_a_call_on_one_line_its_time_agent_tool_description_and_duration() {
    assert_eq!(
        call_line(&call()),
        "  21:04:05 main Bash · List the files (1.2 s)"
    );
    assert_eq!(
        call_line(&LoggedCall {
            description: String::new(),
            is_ok: false,
            ..call()
        }),
        "✗ 21:04:05 main Bash · ls (1.2 s)"
    );
    // 1 250 ms is 1.25 s, a tie, rounded up as `toFixed` did.
    assert_eq!(
        call_line(&LoggedCall {
            duration_ms: 1250.0,
            ..call()
        }),
        "  21:04:05 main Bash · List the files (1.3 s)"
    );
}

#[test]
fn keeps_the_latest_calls_of_each_agent_the_main_agent_first_even_without_calls() {
    let auditor = |summary: &str| LoggedCall {
        agent: "auditor".to_owned(),
        summary: summary.to_owned(),
        ..call()
    };
    let calls = [auditor("a"), auditor("b"), auditor("c")];
    let latest = latest_by_agent(&calls, 2);

    assert_eq!(
        latest
            .iter()
            .map(|(agent, _)| agent.as_str())
            .collect::<Vec<_>>(),
        ["main", "auditor"]
    );
    assert_eq!(
        latest[1]
            .1
            .iter()
            .map(|call| call.summary.as_str())
            .collect::<Vec<_>>(),
        ["b", "c"]
    );
    // 0 keeps them all, as the script's `slice(-0)` did.
    assert_eq!(latest_by_agent(&calls, 0)[1].1.len(), 3);
}

#[test]
fn reads_a_logged_call_the_main_agent_when_it_names_none() {
    let read = logged_call(&json!({"durationMs": 5, "time": "t", "tool": "Read", "isOk": false}));

    assert_eq!(read.agent, "main");
    assert!(!read.is_ok);
    assert!(logged_call(&json!({})).is_ok);
}

#[test]
fn lists_the_calls_running_each_agents_latest_and_the_end_of_each_output() {
    let running = vec!["x".repeat(170)];
    let outputs = vec![(
        "/tmp/a.output".to_owned(),
        "1\n2\n3\n4\n5\n6\n\n".to_owned(),
    )];
    let text = activity_lines(&running, &[call()], 10, &outputs);

    // The record cut at 160 characters; the output's last five lines once trimmed.
    assert_eq!(
        text,
        format!(
            "Running now:\n  {}\nLatest calls of main:\n    21:04:05 main Bash · List the files (1.2 s)\nBackground output /tmp/a.output:\n  2\n  3\n  4\n  5\n  6\n",
            "x".repeat(160)
        )
    );
    assert_eq!(
        activity_lines(&[], &[], 10, &[]),
        "Running now:\nLatest calls of main:\n"
    );
}

#[test]
fn finds_the_background_outputs_of_the_last_hour_the_newest_first() {
    let root = env::temp_dir().join(format!("metrics-outputs-{}", std::process::id()));
    let write = |path: &Path| {
        fs::create_dir_all(path.parent().expect("folder")).expect("folders");
        fs::write(path, "x").expect("file");
    };

    let _ = fs::remove_dir_all(&root);
    write(&root.join("s1/tasks/old.output"));
    write(&root.join("s2/tasks/new.output"));
    write(&root.join("s2/tasks/notes.txt"));
    write(&root.join("loose.output"));

    let now = SystemTime::now();
    // Written now: both outputs, the folder's names sorted, then the newest first; an hour later,
    // none.
    let found = recent_outputs(&root, now);

    assert_eq!(found.len(), 2);
    assert!(
        found
            .iter()
            .all(|path| path.to_string_lossy().ends_with(".output"))
    );
    assert_eq!(
        recent_outputs(&root, now + Duration::from_secs(3601)),
        [] as [std::path::PathBuf; 0]
    );
    assert_eq!(
        recent_outputs(&root.join("missing"), now),
        [] as [std::path::PathBuf; 0]
    );
    assert_eq!(
        outputs_folder("501", Path::new("/Users/a/SVG editor")),
        Path::new("/private/tmp/claude-501/-Users-a-SVG-editor")
    );
    let _ = fs::remove_dir_all(&root);
}
