//! Tests of the transcripts' tokens read incrementally (EN-092).

use run_totals::tally::{FileTally, Tokens, Window, read_chunk};

const OPEN: Window = Window {
    from: 0,
    to: i64::MAX,
};

/// An assistant line of a request `id` at `time` with the given usage.
fn request(id: &str, time: &str, usage: [i64; 4]) -> String {
    format!(
        "{{\"type\":\"assistant\",\"timestamp\":\"{time}\",\"message\":{{\"id\":\"{id}\",\"usage\":{{\"input_tokens\":{},\"cache_creation_input_tokens\":{},\"cache_read_input_tokens\":{},\"output_tokens\":{}}}}}}}\n",
        usage[0], usage[1], usage[2], usage[3]
    )
}

#[test]
fn counts_a_request_written_over_several_lines_once() {
    let line = request("m1", "2026-10-17T10:00:00Z", [3, 100, 2_000, 50]);
    let text = format!("{line}{line}{{\"type\":\"user\",\"message\":{{}}}}\n");
    let tally = read_chunk(&FileTally::default(), &text, OPEN);

    // 3 + 100 + 2 000 + 50 = 2 153, the second line of m1 and the user line not counted.
    assert_eq!(
        tally.tokens,
        Tokens {
            input: 3,
            cache_write: 100,
            cache_read: 2_000,
            output: 50
        }
    );
    assert_eq!(tally.tokens.total(), 2_153);
    assert_eq!(tally.offset, text.len());
    assert_eq!(tally.last_id, "m1");
}

#[test]
fn keeps_a_line_without_its_line_break_for_the_next_reading() {
    let first = request("m1", "2026-10-17T10:00:00Z", [1, 0, 0, 1]);
    let second = request("m2", "2026-10-17T10:00:01Z", [0, 0, 10, 2]);
    let partial = &second[..second.len() - 10];
    let tally = read_chunk(&FileTally::default(), &format!("{first}{partial}"), OPEN);

    assert_eq!(tally.offset, first.len());
    assert_eq!(tally.tokens.total(), 2);

    // The next reading starts at the offset: the whole second line, then m1 again is new.
    let next = read_chunk(&tally, &second, OPEN);

    assert_eq!(next.offset, first.len() + second.len());
    assert_eq!(next.tokens.total(), 14);
    assert_eq!(read_chunk(&next, &second, OPEN).tokens.total(), 14);
}

#[test]
fn counts_only_the_requests_inside_the_run() {
    // 2026-10-17T10:00:00Z = 20 743 days × 86 400 000 + 36 000 000 ms.
    let ten = 20_743 * 86_400_000 + 36_000_000;
    let text = [
        request("before", "2026-10-17T09:59:59Z", [1, 0, 0, 0]),
        request("start", "2026-10-17T10:00:00Z", [0, 10, 0, 0]),
        request("end", "2026-10-17T11:00:00Z", [0, 0, 100, 0]),
        request("after", "2026-10-17T11:00:01Z", [0, 0, 0, 1_000]),
    ]
    .concat();
    let window = Window {
        from: ten,
        to: ten + 3_600_000,
    };

    assert_eq!(
        read_chunk(&FileTally::default(), &text, window)
            .tokens
            .total(),
        110
    );
}

#[test]
fn skips_lines_that_are_not_json_or_carry_no_message_id() {
    let text = format!(
        "garbage\n{}{}",
        request("", "2026-10-17T10:00:00Z", [5, 5, 5, 5]),
        request("m", "2026-10-17T10:00:00Z", [1, 1, 1, 1])
    );

    assert_eq!(
        read_chunk(&FileTally::default(), &text, OPEN)
            .tokens
            .total(),
        4
    );
    assert_eq!(
        read_chunk(&FileTally::default(), "", OPEN),
        FileTally::default()
    );
}

#[test]
fn counts_a_request_by_the_usage_of_its_last_line() {
    // A subagent's request: output 5 on its first line, 50 on its last; 1 + 10 + 100 + 50 = 161.
    let first = request("m1", "2026-10-17T10:00:00Z", [1, 10, 100, 5]);
    let last = request("m1", "2026-10-17T10:00:02Z", [1, 10, 100, 50]);
    let other = request("m2", "2026-10-17T10:00:03Z", [0, 0, 0, 7]);

    assert_eq!(
        read_chunk(&FileTally::default(), &format!("{first}{last}"), OPEN)
            .tokens
            .total(),
        161
    );

    // The last line read in a later refresh still replaces the first: 161, then 161 + 7 = 168.
    let early = read_chunk(&FileTally::default(), &first, OPEN);

    assert_eq!(early.tokens.total(), 116);
    assert_eq!(
        read_chunk(&early, &format!("{last}{other}"), OPEN)
            .tokens
            .total(),
        168
    );
}
