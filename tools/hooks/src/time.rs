//! Times of transcript lines: ISO 8601 instants to epoch milliseconds, and durations written short,
//! on the dates of `calendar`.

use calendar::{DAY_MS, MINUTE_MS, SECOND_MS, days_from_civil, duration_ms};

/// Epoch milliseconds of a UTC instant written `2026-10-16T20:41:03.123Z`, 0 when it is not one.
#[must_use]
pub fn epoch_ms(instant: &str) -> i64 {
    let number =
        |range: std::ops::Range<usize>| -> Option<i64> { instant.get(range)?.parse().ok() };
    let parts = (|| {
        let fraction = instant
            .get(19..)?
            .strip_suffix('Z')?
            .strip_prefix('.')
            .map_or(Some(0), |digits| {
                format!("{digits:0<3}").get(..3)?.parse::<i64>().ok()
            })?;

        Some(
            days_from_civil(number(0..4)?, number(5..7)?, number(8..10)?) * DAY_MS
                + duration_ms(number(11..13)?, number(14..16)?, number(17..19)?, fraction),
        )
    })();

    parts.unwrap_or(0)
}

/// A duration written short: `45 s`, `12 min` or `2 h 05`, as `Math.floor` and `Math.round` give:
/// the minutes rounded down, the seconds to the nearest, a half up. The one copy of the hooks,
/// the status line's and the run's (NAM-002).
#[must_use]
pub fn short_duration(milliseconds: i64) -> String {
    let minutes = milliseconds.div_euclid(MINUTE_MS);

    if minutes < 1 {
        format!("{} s", (milliseconds + SECOND_MS / 2).div_euclid(SECOND_MS))
    } else if minutes < 60 {
        format!("{minutes} min")
    } else {
        format!("{} h {:02}", minutes / 60, minutes % 60)
    }
}
