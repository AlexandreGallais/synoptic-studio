//! Tests of what a run's totals are shown as (EN-092).

use run_totals::report::{grouped, report_table, status_text};
use run_totals::tally::Tokens;

const TOKENS: Tokens = Tokens {
    input: 1_234,
    cache_write: 2_000_000,
    cache_read: 41_000_000,
    output: 98_766,
};

#[test]
fn groups_thousands() {
    assert_eq!(grouped(0), "0");
    assert_eq!(grouped(999), "999");
    assert_eq!(grouped(1_000), "1,000");
    assert_eq!(grouped(41_141_108), "41,141,108");
    assert_eq!(grouped(-1_234), "-1,234");
}

#[test]
fn reports_the_duration_and_the_tokens_by_kind_only() {
    // 1 234 + 2 000 000 + 41 000 000 + 98 766 = 43 100 000; 125 min = 2 h 05.
    assert_eq!(
        report_table(7_500_000, &TOKENS, false),
        "| Duration | Tokens | Input | Cache written | Cache read | Output |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | 2 h 05 | 43,100,000 | 1,234 | 2,000,000 | 41,000,000 | 98,766 |\n"
    );
    assert!(report_table(60_000, &TOKENS, true).contains("| 1 min, open |"));
}

#[test]
fn f20_ac4_shows_the_run_on_the_status_line() {
    assert_eq!(
        status_text(7_500_000, &TOKENS, None),
        "run 2 h 05 · 43,100,000 tokens"
    );
    assert_eq!(
        status_text(45_000, &Tokens::default(), Some(34)),
        "run 45 s · 0 tokens · week 34 % used"
    );
}

#[test]
fn f21_ac7_records_every_subcommand_but_the_status_line_s_poll() {
    // The status line calls `status` once a second during a run: a poll, not recorded (EN-102);
    // `start`, `end` and `should-stop` are uses (AUD-023: tested by its function, not its text).
    assert!(run_totals::is_poll("status"));
    for subcommand in ["start", "end", "should-stop", ""] {
        assert!(!run_totals::is_poll(subcommand), "{subcommand}");
    }
}
