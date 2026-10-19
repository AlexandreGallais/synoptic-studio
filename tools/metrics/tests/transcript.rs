//! Tests of the transcripts' reader (EN-107): the cases of `scripts/metrics-events.test.ts`, then
//! the incremental reading.

use metrics::transcript::{Transcript, all_events, call_summary, model_usage, read_chunk};
use run_totals::tally::Tokens;

/// 2026-10-12T10:00:01Z: 20 738 days after 1970-01-01, plus 10 h and 1 s.
const AT_10_00_01: i64 = 20_738 * 86_400_000 + 36_001_000;

/// One request written on two lines (a text, then a tool call), its failed result, and a line
/// without a time; each line ended.
fn transcript() -> String {
    let usage = r#""usage":{"cache_creation_input_tokens":20,"cache_read_input_tokens":100,"input_tokens":3,"output_tokens":7}"#;

    [
        format!(
            r#"{{"gitBranch":"main","message":{{"content":[{{"text":"Checking.","type":"text"}}],"id":"msg_1","model":"claude-opus-5-5",{usage}}},"sessionId":"s1","timestamp":"2026-10-12T10:00:00.000Z","type":"assistant"}}"#
        ),
        format!(
            r#"{{"gitBranch":"main","message":{{"content":[{{"id":"toolu_1","input":{{"command":"npm  run\ncheck"}},"name":"Bash","type":"tool_use"}}],"id":"msg_1","model":"claude-opus-5-5",{usage}}},"sessionId":"s1","timestamp":"2026-10-12T10:00:01.000Z","type":"assistant"}}"#
        ),
        r#"{"gitBranch":"main","message":{"content":[{"is_error":true,"tool_use_id":"toolu_1","type":"tool_result"}]},"sessionId":"s1","timestamp":"2026-10-12T10:00:31.000Z","type":"user"}"#.to_owned(),
        r#"{"sessionId":"s1","type":"last-prompt"}"#.to_owned(),
        String::new(),
    ]
    .join("\n")
}

#[test]
fn counts_a_request_written_on_several_lines_once_with_its_last_line() {
    let read = read_chunk(Transcript::default(), &transcript());
    let events = read.events();

    assert_eq!(events.requests.len(), 1);
    let request = &events.requests[0];

    assert_eq!(request.id, "msg_1");
    assert_eq!(request.model, "claude-opus-5-5");
    assert_eq!(
        (request.branch.as_str(), request.session.as_str()),
        ("main", "s1")
    );
    // The second line's time, 10:00:01.
    assert_eq!(request.time, AT_10_00_01);
    assert_eq!(
        request.tokens,
        Tokens {
            input: 3,
            cache_write: 20,
            cache_read: 100,
            output: 7
        }
    );
}

#[test]
fn times_a_tool_call_from_its_request_to_its_result_and_keeps_its_failure() {
    let events = read_chunk(Transcript::default(), &transcript()).events();

    assert_eq!(events.calls.len(), 1);
    let call = &events.calls[0];

    assert_eq!(call.asked.id, "toolu_1");
    assert_eq!(call.asked.tool, "Bash");
    assert_eq!(call.asked.summary, "npm run check");
    assert_eq!(call.asked.start, AT_10_00_01);
    // The result 30 s later.
    assert_eq!(call.answer.end, AT_10_00_01 + 30_000);
    assert!(!call.answer.is_ok);
}

#[test]
fn takes_the_first_field_naming_what_the_tool_acted_on() {
    let input = serde_json::json!({"file_path": "a.ts", "pattern": "x"});

    assert_eq!(call_summary(&input), "a.ts");
    assert_eq!(call_summary(&serde_json::Value::Null), "");
    // Each run of white space, a no-break space and a tab included, becomes one space.
    assert_eq!(
        call_summary(&serde_json::json!({"description": "a\u{a0}\t b\n"})),
        "a b "
    );
}

#[test]
fn starts_the_turn_at_the_first_prompt_after_the_agent_ended_its_last_turn() {
    let lines = [
        r#"{"type": "user", "timestamp": "2026-10-12T10:00:00Z", "message": {"content": "first"}}"#,
        r#"{"type": "assistant", "timestamp": "2026-10-12T10:01:00Z", "message": {"stop_reason": "end_turn"}}"#,
        r#"{"type": "user", "timestamp": "2026-10-12T11:00:00Z", "message": {"content": "second"}}"#,
        r#"{"type": "user", "timestamp": "2026-10-12T11:05:00Z", "isMeta": true, "message": {"content": "skill"}}"#,
        r#"{"type": "user", "timestamp": "2026-10-12T11:30:00Z", "message": {"content": [{"type": "text", "text": "while working"}]}}"#,
        "",
    ]
    .join("\n");

    // 2026-10-12T11:00:00Z: 20 738 days and 11 h.
    assert_eq!(
        read_chunk(Transcript::default(), &lines).turn_start,
        20_738 * 86_400_000 + 39_600_000
    );
    assert_eq!(read_chunk(Transcript::default(), "").turn_start, 0);
}

#[test]
fn leaves_out_a_prompt_from_the_system_a_compaction_summary_and_a_tool_result() {
    let lines = [
        r#"{"type":"user","timestamp":"2026-10-12T10:00:00Z","promptSource":"system","message":{"content":"task done"}}"#,
        r#"{"type":"user","timestamp":"2026-10-12T10:01:00Z","isCompactSummary":true,"message":{"content":"summary"}}"#,
        r#"{"type":"user","timestamp":"2026-10-12T10:02:00Z","message":{"content":[{"type":"tool_result","tool_use_id":"t"}]}}"#,
        "",
    ]
    .join("\n");

    assert_eq!(read_chunk(Transcript::default(), &lines).turn_start, 0);
}

#[test]
fn reads_in_chunks_as_in_one_reading_a_line_cut_kept_for_the_next() {
    let text = transcript();
    let whole = read_chunk(Transcript::default(), &text);

    // Cut inside the second line: the first reading stops at the end of the first line.
    let cut = text.find("toolu_1").expect("the call");
    let first = read_chunk(Transcript::default(), &text[..cut]);

    assert_eq!(first.offset, text.find('\n').expect("a line") + 1);
    assert_eq!(first.events().requests.len(), 1);

    let second = read_chunk(first.clone(), &text[first.offset..]);

    assert_eq!(second, whole);
    assert_eq!(second.offset, text.len());
}

#[test]
fn takes_a_result_that_came_before_its_call_and_the_last_of_two_results() {
    let lines = [
        r#"{"type":"user","timestamp":"2026-10-12T10:00:05Z","message":{"content":[{"type":"tool_result","tool_use_id":"t1","is_error":true}]}}"#,
        r#"{"type":"assistant","timestamp":"2026-10-12T10:00:00Z","message":{"id":"m","content":[{"type":"tool_use","id":"t1","name":"Read","input":{"file_path":"a"}}]}}"#,
        r#"{"type":"user","timestamp":"2026-10-12T10:00:09Z","message":{"content":[{"type":"tool_result","tool_use_id":"t1"}]}}"#,
        // Not JSON, then a call never answered: both left out.
        "{not json",
        r#"{"type":"assistant","timestamp":"2026-10-12T10:00:10Z","message":{"id":"","content":[{"type":"tool_use","id":"t2","name":"Bash","input":{}}]}}"#,
        "",
    ]
    .join("\n");
    let events = read_chunk(Transcript::default(), &lines).events();

    // One call, answered 9 s after the hour by its second result, a success; no request without id
    // beyond `m`.
    assert_eq!(events.calls.len(), 1);
    assert_eq!(
        events.calls[0].answer.end - events.calls[0].asked.start,
        9_000
    );
    assert!(events.calls[0].answer.is_ok);
    assert_eq!(events.requests.len(), 1);
}

#[test]
fn keeps_the_order_of_the_transcripts() {
    let one = read_chunk(Transcript::default(), &transcript());
    let other = read_chunk(
        Transcript::default(),
        "{\"type\":\"assistant\",\"timestamp\":\"2026-10-12T09:00:00Z\",\"message\":{\"id\":\"msg_0\"}}\n",
    );
    let events = all_events(&[one, other]);

    assert_eq!(
        events
            .requests
            .iter()
            .map(|request| request.id.as_str())
            .collect::<Vec<_>>(),
        ["msg_1", "msg_0"]
    );
}

/// The last `cost-state` line of session 48a404b1 (2026-10-15), its Haiku and Sonnet counts as
/// written, its Opus count made small; Sonnet's output given.
fn cost_state(sonnet_output: i64) -> String {
    format!(
        r#"{{"type":"cost-state","sessionId":"48a404b1","totalCostUSD":1359.66,"startTime":1792183009097,"modelUsage":{{"claude-haiku-5-5":{{"inputTokens":593913,"outputTokens":65138,"thinkingTokens":41688,"cacheReadInputTokens":114212,"cacheCreationInputTokens":123371,"webSearchRequests":6,"costUSD":0.16}},"claude-opus-5-5":{{"inputTokens":1,"outputTokens":2,"thinkingTokens":9,"cacheReadInputTokens":3,"cacheCreationInputTokens":4,"webSearchRequests":0,"costUSD":1.0}},"claude-sonnet-5-5":{{"inputTokens":4451,"outputTokens":{sonnet_output},"thinkingTokens":2823,"cacheReadInputTokens":1693941,"cacheCreationInputTokens":92320,"webSearchRequests":0,"costUSD":0.64}}}},"hasUnknownModelCost":false}}"#
    )
}

/// Tokens by kind: input, cache written, cache read, output.
fn tokens(input: i64, cache_write: i64, cache_read: i64, output: i64) -> Tokens {
    Tokens {
        input,
        cache_write,
        cache_read,
        output,
    }
}

#[test]
fn f32_ac1_reads_the_tokens_of_each_model_from_a_cost_state_line() {
    let line: serde_json::Value = serde_json::from_str(&cost_state(23_875)).expect("JSON");

    // `thinkingTokens` left out: Opus 1 + 4 + 3 + 2, never 9 more.
    assert_eq!(
        model_usage(&line),
        [
            (
                "claude-haiku-5-5".to_owned(),
                tokens(593_913, 123_371, 114_212, 65_138)
            ),
            ("claude-opus-5-5".to_owned(), tokens(1, 4, 3, 2)),
            (
                "claude-sonnet-5-5".to_owned(),
                tokens(4451, 92_320, 1_693_941, 23_875)
            ),
        ]
    );
    let request: serde_json::Value =
        serde_json::from_str(r#"{"type":"assistant","modelUsage":{"x":{"inputTokens":1}}}"#)
            .expect("JSON");

    assert_eq!(model_usage(&request), []);
}

#[test]
fn f32_ac1_keeps_the_last_count_of_a_session_unless_a_request_came_after_it() {
    let request = r#"{"type":"assistant","timestamp":"2026-10-15T10:00:00Z","message":{"id":"m","model":"claude-opus-5-5","usage":{"output_tokens":5}}}"#;
    // The count has no time, yet is read; a later one replaces it.
    let left = read_chunk(
        Transcript::default(),
        &format!("{request}\n{}\n{}\n", cost_state(10), cost_state(23_875)),
    );

    assert_eq!(left.model_usage.len(), 3);
    assert_eq!(left.model_usage[2].1.output, 23_875);
    assert_eq!(left.requests.len(), 1);
    // Resumed, the session made a request its count does not hold yet: no count.
    let resumed = read_chunk(left.clone(), &format!("{request}\n"));

    assert_eq!(resumed.model_usage, []);
    // The events carry it, summed per model over the transcripts.
    let events = all_events(&[left.clone(), Transcript::default(), left]);

    assert_eq!(
        events.model_usage[1],
        ("claude-opus-5-5".to_owned(), tokens(2, 8, 6, 4))
    );
    assert_eq!(events.model_usage.len(), 3);
}
