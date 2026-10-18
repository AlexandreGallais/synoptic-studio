//! Tests of the turns' time, the saved turns and the tools' tables: the cases of
//! `scripts/metrics-time.test.ts`, `scripts/turns.test.ts` and `scripts/metrics-tools.test.ts`.

use metrics::report::{logged_since, since_start};
use metrics::tools::{
    agent_tool, locale_compare, tool_use_key, tool_uses_table, tools_table, uses_since,
};
use metrics::turn_time::{StartedCall, started_calls, time_table, turn_time};
use metrics::turns::{TurnRecord, read_turn, render_turn, turns_table};
use serde_json::{Value, json};

/// 2026-10-16T00:00Z: 20 742 days after 1970-01-01, 20 742 × 86 400 000 ms.
const OCTOBER_16: i64 = 1_792_108_800_000;

#[test]
fn keeps_the_main_agents_calls_that_have_a_start_in_order_background_ones_left_out() {
    let records = [
        json!({"agent": "main", "durationMs": 5, "inputChars": 10, "startedAt": 2000, "turnStart": 1}),
        json!({"agent": "auditor", "durationMs": 5, "startedAt": 1500, "turnStart": 1}),
        json!({"agent": "main", "durationMs": 5, "startedAt": 0, "turnStart": 1}),
        json!({"agent": "main", "durationMs": 5, "startedAt": 1100, "turnStart": 0}),
        json!({"agent": "main", "durationMs": 5, "isBackground": true, "startedAt": 1200, "turnStart": 1}),
        json!({"agent": "main", "durationMs": 7, "inputChars": 3, "startedAt": 1000, "turnStart": 1}),
    ];
    let starts: Vec<f64> = started_calls(&records)
        .iter()
        .map(|call| call.started_at)
        .collect();

    assert_eq!(starts, [1000.0, 2000.0]);
}

#[test]
fn splits_a_turn_between_its_tools_and_the_models_gaps_a_pause_over_5_minutes_left_out() {
    // Calls at 0 (1 s), 3 s (2 s), then 10 min later: gaps 2 s, then a pause.
    let call = |duration_ms: f64, input_chars: f64, started_at: f64| StartedCall {
        duration_ms,
        input_chars,
        started_at,
        turn_start: 0.0,
    };
    let time = turn_time(&[
        call(1000.0, 100.0, 0.0),
        call(2000.0, 50.0, 3000.0),
        call(500.0, 5.0, 605_000.0),
    ]);

    assert_eq!(time.calls, 3);
    assert!((time.chars_written - 155.0).abs() < f64::EPSILON);
    assert!((time.model_ms - 2000.0).abs() < f64::EPSILON);
    assert!((time.tool_ms - 3500.0).abs() < f64::EPSILON);
}

#[test]
fn writes_the_latest_turns_with_the_models_share() {
    // One turn at 2026-10-16T00:00Z: calls of 1 s and 3 s, 4 s apart: 1 s of tools, then 3 s of
    // the model, then 3 s of tools; the model's share 3 / (3 + 4) = 43 %.
    let records = [
        json!({"agent": "main", "durationMs": 1000, "inputChars": 1500, "startedAt": OCTOBER_16, "turnStart": OCTOBER_16}),
        json!({"agent": "main", "durationMs": 3000, "inputChars": 10, "startedAt": OCTOBER_16 + 4000, "turnStart": OCTOBER_16}),
    ];

    assert_eq!(
        time_table(&records, 10).lines().nth(2),
        Some("| 2026-10-16T00:00 | 2 | 4.0 s | 3.0 s | 43 % | 1,510 |")
    );
    assert_eq!(time_table(&records, 0).lines().count(), 2);
}

#[test]
fn f18_ac11_starts_a_report_at_00_00_utc_of_its_since_day_at_0_without_it() {
    let words = |text: &str| text.split(' ').map(str::to_owned).collect::<Vec<_>>();

    assert_eq!(since_start(&words("--since 2026-10-16")), Ok(OCTOBER_16));
    assert_eq!(since_start(&[]), Ok(0));
    assert!(since_start(&words("--since")).is_err());
    assert!(since_start(&words("--since 2026-02-30")).is_err());
    assert!(since_start(&words("--since 16/10/2026")).is_err());
}

#[test]
fn f18_ac11_keeps_the_logged_calls_that_ended_from_that_moment_on() {
    let records = [
        json!({"time": "2026-10-15T23:59:59Z", "tool": "Bash"}),
        json!({"time": "2026-10-16T00:00:00Z", "tool": "Read"}),
        json!({"time": "2026-10-17T08:00:00.123456789Z", "tool": "Edit"}),
    ];

    assert_eq!(logged_since(&records, OCTOBER_16), records[1..]);
    assert_eq!(logged_since(&records, 0), records);
}

/// A turn of 10 minutes, as the session report saves it.
fn record(start: i64, calls: i64) -> TurnRecord {
    TurnRecord {
        active_ms: 360_000,
        calls,
        commits: 2,
        failures: 1,
        output_tokens: 75,
        session: "168220a7-35a5".to_owned(),
        start,
        tokens: 3279,
        wall_ms: 600_000,
    }
}

#[test]
fn reads_a_saved_line_back_a_missing_field_as_0_or_empty() {
    let saved = record(1_792_139_400_000, 4);
    let line: Value = serde_json::from_str(&render_turn(&saved)).expect("JSON");

    assert_eq!(read_turn(&line), saved);
    assert_eq!(read_turn(&json!({})), TurnRecord::default());
}

#[test]
fn lists_each_turn_once_its_last_figures_oldest_first() {
    // 2026-10-13T08:30Z: 20 739 days, 8 h 30.
    let start = 20_739 * 86_400_000 + 30_600_000;
    let table = turns_table(&[
        record(start + 3_600_000, 4),
        record(start, 4),
        record(start, 9),
    ]);

    assert_eq!(
        table.lines().skip(2).collect::<Vec<_>>(),
        [
            "| 2026-10-13T08:30 | 168220a7 | 10 min 00 s | 6 min 00 s | 3,279 | 75 | 9 | 1 | 2 |",
            "| 2026-10-13T09:30 | 168220a7 | 10 min 00 s | 6 min 00 s | 3,279 | 75 | 4 | 1 | 2 |",
        ]
    );
}

#[test]
fn finds_the_agents_tool_a_command_runs_flags_and_directory_change_included() {
    assert_eq!(
        agent_tool("cd \"/repo\" && npm run -s replace <<'EOF'"),
        "replace"
    );
    assert_eq!(
        agent_tool("npm run backlog:status -- RET-002 done"),
        "backlog:status"
    );
    assert_eq!(
        agent_tool("npm run --silent metrics:session -- --all"),
        "metrics:session"
    );
    assert_eq!(agent_tool("npm run test"), "");
    assert_eq!(agent_tool("xnpm run replace"), "");
    // The first `npm run` naming a script is read: `-x` is no script, `activity` is.
    assert_eq!(agent_tool("npm run -x; npm run activity"), "activity");
    assert_eq!(agent_tool("ls"), "");
}

#[test]
fn counts_each_tools_runs_and_failures_the_most_used_first() {
    let records = [
        json!({"durationMs": 100, "isOk": true, "summary": "npm run -s replace", "tool": "Bash"}),
        json!({"durationMs": 300, "isOk": false, "summary": "npm run -s replace", "tool": "Bash"}),
        json!({"durationMs": 50, "isOk": true, "summary": "npm run activity", "tool": "Bash"}),
        json!({"durationMs": 9, "isOk": true, "summary": "npm run -s replace", "tool": "Read"}),
    ];
    let table = tools_table(&records);
    let rows: Vec<&str> = table.lines().collect();

    // replace: 100 and 300 ms, total 400, median 200, one failure; the Read call left out.
    assert_eq!(rows[2], "| `replace` | 2 | 400 ms | 200 ms | 300 ms | 1 |");
    assert_eq!(rows[3], "| `activity` | 1 | 50 ms | 50 ms | 50 ms | 0 |");
}

#[test]
fn f21_ac7_groups_the_recorded_uses_by_tool_and_subcommand() {
    let records: Vec<Value> = [
        (10, 0, "--check", "format"),
        (100, 0, "sync", "git-sequences"),
        (30, 1, "--check", "format"),
        (5, 0, "", "lint"),
        (20, 0, "--check", "format"),
        (300, 0, "sync", "git-sequences"),
        (40, 2, "--write", "format"),
    ]
    .iter()
    .map(|(duration, code, subcommand, tool)| {
        json!({"durationMs": duration, "exitCode": code, "subcommand": subcommand, "tool": tool, "start": "2026-10-18T10:00:00.000Z", "worktree": "main"})
    })
    .collect();

    // format --check: 10, 30, 20, median 20, slowest 30, one exit 1. git-sequences sync: 100 and
    // 300, median (100 + 300) / 2 = 200. Ties of one call sorted by name: --write before lint.
    assert_eq!(
        tool_uses_table(&records).lines().collect::<Vec<_>>(),
        [
            "| Tool | Calls | Failed | Median | Slowest |",
            "| --- | --- | --- | --- | --- |",
            "| `format --check` | 3 | 1 | 20 ms | 30 ms |",
            "| `git-sequences sync` | 2 | 0 | 200 ms | 300 ms |",
            "| `format --write` | 1 | 1 | 40 ms | 40 ms |",
            "| `lint` | 1 | 0 | 5 ms | 5 ms |",
        ]
    );
    assert_eq!(tool_uses_table(&[]).lines().count(), 2);
}

#[test]
fn f21_ac7_keeps_the_uses_started_from_the_reports_first_moment_on() {
    let records = [
        json!({"start": "2026-10-15T23:59:59.999Z", "tool": "lint"}),
        json!({"start": "2026-10-16T00:00:00.000Z", "tool": "format"}),
    ];

    assert_eq!(uses_since(&records, OCTOBER_16), records[1..]);
    assert_eq!(uses_since(&records, 0), records);
    assert_eq!(
        tool_use_key(&json!({"subcommand": "", "tool": "lint"})),
        "lint"
    );
}

#[test]
fn orders_names_as_locale_compare_does() {
    use std::cmp::Ordering::{Greater, Less};

    // White space before punctuation, punctuation before digits and letters, a letter's case only
    // when all else is equal.
    assert_eq!(locale_compare("a b", "a-b"), Less);
    assert_eq!(locale_compare("a-b", "a1"), Less);
    assert_eq!(locale_compare("a1", "ab"), Less);
    assert_eq!(locale_compare("B", "a"), Greater);
    assert_eq!(locale_compare("a", "A"), Less);
    assert_eq!(locale_compare("ab", "Ab"), Less);
}
