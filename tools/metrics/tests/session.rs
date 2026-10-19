//! Tests of the session report (EN-107): the cases of `scripts/session-report.test.ts`, the tokens
//! per model of EN-066 included.

use metrics::session::{
    markdown_report, report_rows, reported_events, session_figures, terminal_report,
};
use metrics::transcript::{Answer, Call, Events, Request, ToolUse, Transcript, read_chunk};
use metrics::turns::{render_turn, turn_record};
use run_totals::tally::Tokens;

/// A request of `model` at `time` with its tokens: cache read, cache written, input, output.
fn request(id: &str, model: &str, time: i64, tokens: [i64; 4]) -> Request {
    Request {
        id: id.to_owned(),
        model: model.to_owned(),
        branch: "develop".to_owned(),
        session: "s".to_owned(),
        time,
        tokens: Tokens {
            cache_read: tokens[0],
            cache_write: tokens[1],
            input: tokens[2],
            output: tokens[3],
        },
    }
}

/// Two requests, one on Opus and one on Sonnet, and one failed tool call, 10 minutes apart.
fn events() -> Events {
    Events {
        requests: vec![
            request("m1", "claude-opus-5-5", 0, [1000, 200, 3, 50]),
            request("m2", "claude-sonnet-5-5", 60_000, [2000, 0, 1, 25]),
        ],
        calls: vec![Call {
            asked: ToolUse {
                id: "t1".to_owned(),
                tool: "Bash".to_owned(),
                summary: "npm run check".to_owned(),
                branch: "develop".to_owned(),
                session: "s".to_owned(),
                start: 300_000,
            },
            answer: Answer {
                end: 600_000,
                is_ok: false,
            },
        }],
        model_usage: Vec::new(),
    }
}

#[test]
fn sums_tokens_by_kind_and_times_the_session() {
    let figures = session_figures(&events(), vec!["abc1234 docs: x".to_owned()]);

    // Gaps 60 s and 540 s, the second capped at 5 minutes: 60 + 300 = 360 s.
    assert_eq!(figures.active_ms, 360_000);
    assert_eq!(figures.wall_ms, 600_000);
    assert_eq!(
        figures.tokens,
        Tokens {
            input: 4,
            cache_write: 200,
            cache_read: 3000,
            output: 75
        }
    );
    assert_eq!(
        (figures.requests, figures.calls, figures.failures),
        (2, 1, 1)
    );
    // Opus: 1000 + 200 + 3 + 50 = 1253; Sonnet: 2000 + 0 + 1 + 25 = 2026; the most first.
    assert_eq!(
        figures.tokens_by_model,
        [
            ("claude-sonnet-5-5".to_owned(), 2026),
            ("claude-opus-5-5".to_owned(), 1253)
        ]
    );
}

#[test]
fn f20_ac2_gives_the_tokens_of_each_model_the_most_first_without_the_claude_prefix() {
    let rows = report_rows(&session_figures(&events(), Vec::new()));

    // 4 + 200 + 3000 + 75 = 3279.
    assert_eq!(rows[1], ("Tokens", "3,279".to_owned()));
    assert_eq!(
        rows[2],
        ("Per model", "sonnet-5-5 2,026 · opus-5-5 1,253".to_owned())
    );
}

#[test]
fn names_a_request_without_a_model_and_leaves_out_a_model_without_tokens() {
    let figures = session_figures(
        &Events {
            requests: vec![
                request("m1", "", 0, [1000, 200, 3, 50]),
                // A synthetic answer written by Claude Code itself counts no token.
                request("m2", "<synthetic>", 60_000, [0, 0, 0, 0]),
            ],
            calls: Vec::new(),
            model_usage: Vec::new(),
        },
        Vec::new(),
    );

    // Request m1 alone: 1253 tokens, under no model's name.
    assert_eq!(figures.tokens_by_model, [(String::new(), 1253)]);
    assert_eq!(
        report_rows(&figures)[2],
        ("Per model", "unknown 1,253".to_owned())
    );
    assert_eq!(
        report_rows(&session_figures(&Events::default(), Vec::new()))[2],
        ("Per model", "none".to_owned())
    );
}

#[test]
fn writes_a_markdown_table_and_the_commits() {
    let report = markdown_report(&session_figures(
        &events(),
        vec!["abc1234 docs: x".to_owned()],
    ));

    assert_eq!(
        report.lines().take(3).collect::<Vec<_>>(),
        [
            "| Duration | Tokens | Per model | Written | Tool calls | Commits |",
            "| --- | --- | --- | --- | --- | --- |",
            "| 10 min 00 s (active 6 min 00 s) | 3,279 | sonnet-5-5 2,026 · opus-5-5 1,253 | 75 | 1 (1 failed) | 1 |",
        ]
    );
    assert!(report.ends_with("\n\n- `abc1234 docs: x`"));
}

#[test]
fn lists_no_commit_beyond_eight() {
    let commits: Vec<String> = (0..9).map(|at| format!("c{at} x")).collect();
    let report = markdown_report(&session_figures(&events(), commits));

    assert_eq!(report.lines().count(), 3);
    assert!(report.ends_with("| 9 |"));
}

#[test]
fn frames_the_terminal_report_its_values_aligned_right() {
    let report = terminal_report(&session_figures(
        &events(),
        vec!["abc1234 docs: x".to_owned()],
    ));
    let lines: Vec<&str> = report.lines().collect();

    assert_eq!(lines[0], "\u{1b}[35m\u{1b}[1mSession report\u{1b}[0m");
    // The widest row, `Per model` (9) and its value (33 characters), plus 4: a rule of 46.
    assert_eq!(lines[1], format!("\u{1b}[2m{}\u{1b}[0m", "─".repeat(46)));
    assert_eq!(
        lines[3],
        format!(
            "\u{1b}[36mTokens\u{1b}[0m{}\u{1b}[1m3,279\u{1b}[0m",
            " ".repeat(35)
        )
    );
    assert_eq!(lines[9], "\u{1b}[32m●\u{1b}[0m abc1234 docs: x");
}

/// A transcript line of a request: an answer and its tokens, 1 input and 10 written.
fn answer(id: &str, time: &str, stop: &str) -> String {
    format!(
        r#"{{"type": "assistant", "sessionId": "s", "timestamp": "{time}", "message": {{"id": "{id}", "stop_reason": "{stop}", "usage": {{"input_tokens": 1, "output_tokens": 10}}}}}}"#
    )
}

/// Three turns, each opened by a prompt and closed by the agent; a subagent answered during the
/// first one.
fn turns() -> [Transcript; 2] {
    let main = [
        r#"{"type": "user", "timestamp": "2026-10-15T08:00:00Z", "message": {"content": "first"}}"#.to_owned(),
        answer("a", "2026-10-15T08:01:00Z", "end_turn"),
        r#"{"type": "user", "timestamp": "2026-10-15T09:00:00Z", "message": {"content": "second"}}"#.to_owned(),
        answer("b", "2026-10-15T09:10:00Z", "end_turn"),
        r#"{"type": "user", "timestamp": "2026-10-15T18:00:00Z", "message": {"content": "third"}}"#.to_owned(),
        answer("c", "2026-10-15T18:00:02Z", "end_turn"),
        String::new(),
    ]
    .join("\n");
    let subagent = answer("s1", "2026-10-15T08:00:30Z", "tool_use") + "\n";

    [
        read_chunk(Transcript::default(), &main),
        read_chunk(Transcript::default(), &subagent),
    ]
}

#[test]
fn f18_ac9_counts_every_turn_of_the_session_with_all_the_subagents_requests_included() {
    let figures = session_figures(&reported_events(&turns(), true), Vec::new());

    // Requests a, s1, b and c: 4 × 11 tokens; from 08:00:30 to 18:00:02, 35 972 s.
    assert_eq!(figures.requests, 4);
    assert_eq!(figures.tokens.input + figures.tokens.output, 44);
    assert_eq!(figures.wall_ms, 35_972_000);
}

#[test]
fn counts_the_current_turn_only_without_all() {
    let figures = session_figures(&reported_events(&turns(), false), Vec::new());

    // Request c alone, from the prompt at 18:00:00: one request, no duration.
    assert_eq!(figures.requests, 1);
    assert_eq!(figures.wall_ms, 0);
    assert_eq!(reported_events(&[], false), Events::default());
}

#[test]
fn saves_a_turn_its_tokens_summed_its_keys_in_the_saved_order() {
    let figures = session_figures(
        &events(),
        vec!["abc docs: x".to_owned(), "def docs: y".to_owned()],
    );
    let record = turn_record(&figures, 1_792_139_400_000, "168220a7-35a5");

    // 4 + 200 + 3000 + 75 = 3279.
    assert_eq!(record.tokens, 3279);
    assert_eq!(
        render_turn(&record),
        "{\"activeMs\":360000,\"calls\":1,\"commits\":2,\"failures\":1,\"outputTokens\":75,\"session\":\"168220a7-35a5\",\"start\":1792139400000,\"tokens\":3279,\"wallMs\":600000}\n"
    );
}

/// Session 48a404b1 of 2026-10-15 cut down: the main agent's Opus request, then Claude Code's count
/// of the session (`cost-state`), Sonnet's as written, Opus's made small; a Sonnet subagent's
/// request with the tokens its transcript holds for all its requests (98 input, 657 written,
/// 1 315 508 read from the cache, 84 759 written to it).
fn sonnet_session() -> [Transcript; 2] {
    let main = [
        r#"{"type": "user", "timestamp": "2026-10-15T08:00:00Z", "message": {"content": "run"}}"#.to_owned(),
        r#"{"type": "assistant", "sessionId": "s", "timestamp": "2026-10-15T08:01:00Z", "message": {"id": "o1", "model": "claude-opus-5-5", "stop_reason": "end_turn", "usage": {"input_tokens": 1, "output_tokens": 10}}}"#.to_owned(),
        r#"{"type":"cost-state","sessionId":"s","startTime":1792183009097,"modelUsage":{"claude-opus-5-5":{"inputTokens":3,"outputTokens":40,"thinkingTokens":7,"cacheReadInputTokens":900,"cacheCreationInputTokens":57},"claude-sonnet-5-5":{"inputTokens":4451,"outputTokens":23875,"thinkingTokens":2823,"cacheReadInputTokens":1693941,"cacheCreationInputTokens":92320}}}"#.to_owned(),
        String::new(),
    ]
    .join("\n");
    let subagent = r#"{"type": "assistant", "sessionId": "s", "timestamp": "2026-10-15T08:00:30Z", "message": {"id": "s1", "model": "claude-sonnet-5-5", "stop_reason": "tool_use", "usage": {"input_tokens": 98, "output_tokens": 657, "cache_read_input_tokens": 1315508, "cache_creation_input_tokens": 84759}}}"#.to_owned() + "\n";

    [
        read_chunk(Transcript::default(), &main),
        read_chunk(Transcript::default(), &subagent),
    ]
}

#[test]
fn f32_ac1_counts_a_sonnet_subagent_s_tokens_from_the_session_s_model_usage() {
    let figures = session_figures(&reported_events(&sonnet_session(), true), Vec::new());

    // CHK-021 (I83): the transcripts held 657 tokens written by Sonnet where Claude Code counted
    // 23 875. Sonnet: 4 451 + 92 320 + 1 693 941 + 23 875 = 1 814 587; Opus: 3 + 57 + 900 + 40 =
    // 1 000; in all 1 815 587, of which 23 875 + 40 = 23 915 written.
    assert_eq!(
        figures.tokens_by_model,
        [
            ("claude-sonnet-5-5".to_owned(), 1_814_587),
            ("claude-opus-5-5".to_owned(), 1000)
        ]
    );
    assert_eq!(figures.tokens.total(), 1_815_587);
    assert_eq!(figures.tokens.output, 23_915);
    assert_eq!(figures.requests, 2);
}

#[test]
fn f32_ac1_counts_a_turn_from_its_requests_since_the_session_s_count_cannot_be_cut() {
    let figures = session_figures(&reported_events(&sonnet_session(), false), Vec::new());

    // The turn from 08:00:00: Sonnet 98 + 657 + 1 315 508 + 84 759 = 1 401 022; Opus 1 + 10 = 11.
    assert_eq!(
        figures.tokens_by_model,
        [
            ("claude-sonnet-5-5".to_owned(), 1_401_022),
            ("claude-opus-5-5".to_owned(), 11)
        ]
    );
}

#[test]
fn f32_ac2_the_session_report_carries_the_minute() {
    // REV-004's session lasted 1617 min 59.6 s: 97 079 600 ms, shown 1618 min 00 s (I111).
    let figures = metrics::session::SessionFigures {
        wall_ms: 97_079_600,
        active_ms: 59_600,
        ..Default::default()
    };

    assert_eq!(
        report_rows(&figures)[0],
        ("Duration", "1618 min 00 s (active 1 min 00 s)".to_owned())
    );
}
