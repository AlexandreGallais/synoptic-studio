//! Times of transcript lines: ISO 8601 instants to epoch milliseconds, standard library only.

use std::time::{SystemTime, UNIX_EPOCH};

/// Milliseconds in a second, a minute, an hour and a day.
const SECOND: i64 = 1000;
const MINUTE: i64 = 60 * SECOND;
const HOUR: i64 = 60 * MINUTE;
const DAY: i64 = 24 * HOUR;

/// Days from 1970-01-01 to a date of the proleptic Gregorian calendar (H. Hinnant, `days_from_civil`).
#[must_use]
pub fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

    era * 146_097 + day_of_era - 719_468
}

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
            days_from_civil(number(0..4)?, number(5..7)?, number(8..10)?) * DAY
                + number(11..13)? * HOUR
                + number(14..16)? * MINUTE
                + number(17..19)? * SECOND
                + fraction,
        )
    })();

    parts.unwrap_or(0)
}

/// Epoch milliseconds now.
#[must_use]
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| i64::try_from(elapsed.as_millis()).unwrap_or(0))
}

/// A duration written short: `45 s`, `12 min`, `2 h 05`.
#[must_use]
pub fn short_duration(milliseconds: i64) -> String {
    let minutes = milliseconds / MINUTE;

    if minutes < 1 {
        format!("{} s", (milliseconds + SECOND / 2) / SECOND)
    } else if minutes < 60 {
        format!("{minutes} min")
    } else {
        format!("{} h {:02}", minutes / 60, minutes % 60)
    }
}
