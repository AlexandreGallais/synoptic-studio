//! Instants of the reports: read from Git's dates and the logs, written as Temporal wrote them.

use calendar::{DAY_MS, days_from_civil, days_in_month, duration_ms};
use hooks::time::epoch_ms;

/// Characters of a date `2026-10-12`.
const DAY_LENGTH: usize = 10;

/// Epoch milliseconds of an ISO 8601 instant with its offset, `Z` or `+02:00`, as Git's `%aI`
/// writes it; `None` when it is not one.
#[must_use]
pub fn offset_instant_ms(text: &str) -> Option<i64> {
    if text.ends_with('Z') {
        return Some(epoch_ms(text));
    }

    let split = text.len().checked_sub(6)?;
    let (local, offset) = (text.get(..split)?, text.get(split..)?);
    let sign = match offset.as_bytes().first()? {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let hours: i64 = offset.get(1..3)?.parse().ok()?;
    let minutes: i64 = offset.get(4..6)?.parse().ok()?;
    let utc = epoch_ms(&format!("{local}Z"));

    Some(utc - sign * duration_ms(hours, minutes, 0, 0))
}

/// The start of a day written `YYYY-MM-DD`, 00:00 UTC, in epoch milliseconds; `None` when it is
/// not a date.
#[must_use]
pub fn day_start_ms(day: &str) -> Option<i64> {
    let bytes = day.as_bytes();

    if bytes.len() != DAY_LENGTH || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }

    let year: i64 = day.get(0..4)?.parse().ok()?;
    let month: i64 = day.get(5..7)?.parse().ok()?;
    let date: i64 = day.get(8..10)?.parse().ok()?;

    days_in_month(year, month)
        .is_some_and(|days| (1..=days).contains(&date))
        .then(|| days_from_civil(year, month, date) * DAY_MS)
}
