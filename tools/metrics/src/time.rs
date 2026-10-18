//! Instants of the reports: read from Git's dates and the logs, written as Temporal wrote them.

use hooks::time::{days_from_civil, epoch_ms};
use tool_use::iso_instant;

/// Milliseconds in a day, an hour and a minute.
const DAY_MS: i64 = 86_400_000;
const HOUR_MS: i64 = 3_600_000;
const MINUTE_MS: i64 = 60_000;

/// Characters of an ISO 8601 instant that give its day (`2026-10-12`).
const DAY_LENGTH: usize = 10;

/// Characters of an ISO 8601 instant that give its minute (`2026-10-12T10:05`).
const MINUTE_LENGTH: usize = 16;

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

    Some(utc - sign * (hours * HOUR_MS + minutes * MINUTE_MS))
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
    let days_in_month = days_from_civil(year + i64::from(month == 12), month % 12 + 1, 1)
        - days_from_civil(year, month, 1);

    ((1..=12).contains(&month) && (1..=days_in_month).contains(&date))
        .then(|| days_from_civil(year, month, date) * DAY_MS)
}

/// An instant as Temporal writes it: `2026-10-18T09:00:00Z`, its milliseconds only when not 0 and
/// without their trailing zeros (`.12Z`).
#[must_use]
pub fn temporal_text(epoch_ms: i64) -> String {
    let full = iso_instant(epoch_ms);
    let (seconds, fraction) = full.split_once('.').unwrap_or((&full, "000Z"));
    let digits = fraction.trim_end_matches('Z').trim_end_matches('0');

    if digits.is_empty() {
        format!("{seconds}Z")
    } else {
        format!("{seconds}.{digits}Z")
    }
}

/// The day of an instant, UTC: `2026-10-12`.
#[must_use]
pub fn day_text(epoch_ms: i64) -> String {
    iso_instant(epoch_ms)[..DAY_LENGTH].to_owned()
}

/// The minute of an instant, UTC: `2026-10-12T10:05`.
#[must_use]
pub fn minute_text(epoch_ms: i64) -> String {
    iso_instant(epoch_ms)[..MINUTE_LENGTH].to_owned()
}
