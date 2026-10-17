//! What a run's totals are shown as (EN-092): the report at the run's end, its duration and its
//! tokens by kind and nothing else, and the status line's part, `run …` beside the turn.

use hooks::time::short_duration;

use crate::tally::Tokens;

/// A count written with thousands separators: `41,141,108`.
#[must_use]
pub fn grouped(count: i64) -> String {
    let digits = count.unsigned_abs().to_string();
    let mut text = String::with_capacity(digits.len() + digits.len() / 3 + 1);

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            text.push(',');
        }
        text.push(digit);
    }

    if count < 0 { format!("-{text}") } else { text }
}

/// The report: one Markdown table of the run's duration and its tokens by kind, `open` after the
/// duration while the run goes on.
#[must_use]
pub fn report_table(duration_ms: i64, tokens: &Tokens, is_open: bool) -> String {
    let duration = short_duration(duration_ms);
    let rows = [
        (
            "Duration",
            if is_open {
                format!("{duration}, open")
            } else {
                duration
            },
        ),
        ("Tokens", grouped(tokens.total())),
        ("Input", grouped(tokens.input)),
        ("Cache written", grouped(tokens.cache_write)),
        ("Cache read", grouped(tokens.cache_read)),
        ("Output", grouped(tokens.output)),
    ];
    let labels: Vec<&str> = rows.iter().map(|(label, _)| *label).collect();
    let values: Vec<&str> = rows.iter().map(|(_, value)| value.as_str()).collect();

    format!(
        "| {} |\n|{}\n| {} |\n",
        labels.join(" | "),
        " --- |".repeat(rows.len()),
        values.join(" | ")
    )
}

/// The status line's part: `run 2 h 05 · 48,123,456 tokens`, then the week's usage when known.
#[must_use]
pub fn status_text(duration_ms: i64, tokens: &Tokens, week_percent: Option<i64>) -> String {
    let run = format!(
        "run {} · {} tokens",
        short_duration(duration_ms),
        grouped(tokens.total())
    );

    match week_percent {
        Some(percent) => format!("{run} · week {percent} % used"),
        None => run,
    }
}
