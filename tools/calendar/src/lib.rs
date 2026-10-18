//! Dates and times of the proleptic Gregorian calendar in UTC, as epoch milliseconds, the one copy
//! of the workspace (NAM-002): the days since 1970-01-01 of a date and back, the days of a month, a
//! duration in hours, minutes and seconds (`REF-HINNANT-DATES`), and an instant written as ISO
//! 8601 or as Temporal writes it (`REF-TEMPORAL-INSTANT`). The standard library only.

use std::time::{SystemTime, UNIX_EPOCH};

/// Milliseconds in a second.
pub const SECOND_MS: i64 = 1000;

/// Milliseconds in a minute.
pub const MINUTE_MS: i64 = 60 * SECOND_MS;

/// Milliseconds in an hour.
pub const HOUR_MS: i64 = 60 * MINUTE_MS;

/// Milliseconds in a day: UTC counts no leap second.
pub const DAY_MS: i64 = 24 * HOUR_MS;

/// Nanoseconds in a millisecond.
const MILLISECOND_NS: i64 = 1_000_000;

/// Days from 1970-01-01 to a date of the proleptic Gregorian calendar (`REF-HINNANT-DATES`,
/// `days_from_civil`), the inverse of [`civil_from_days`].
#[must_use]
pub fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

    era * 146_097 + day_of_era - 719_468
}

/// The date of the proleptic Gregorian calendar a number of days after 1970-01-01 falls on, as
/// year, month and day (`REF-HINNANT-DATES`, `civil_from_days`), the inverse of
/// [`days_from_civil`].
#[must_use]
pub fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);

    (year, month, day)
}

/// Whether a year of the proleptic Gregorian calendar is a leap year (`REF-HINNANT-DATES`,
/// `is_leap`).
#[must_use]
pub fn is_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// The days of a month, its last day (`REF-HINNANT-DATES`, `last_day_of_month`); `None` for a
/// month outside 1 to 12.
#[must_use]
pub fn days_in_month(year: i64, month: i64) -> Option<i64> {
    match month {
        2 if is_leap(year) => Some(29),
        2 => Some(28),
        4 | 6 | 9 | 11 => Some(30),
        1..=12 => Some(31),
        _ => None,
    }
}

/// A duration of hours, minutes, seconds and milliseconds, in milliseconds: a time of day, or a
/// UTC offset (`REF-HINNANT-DATES`, `days(…) + hours(20) + minutes(59) - utc_offset`).
#[must_use]
pub fn duration_ms(hours: i64, minutes: i64, seconds: i64, milliseconds: i64) -> i64 {
    hours * HOUR_MS + minutes * MINUTE_MS + seconds * SECOND_MS + milliseconds
}

/// The UTC date of an instant, `2026-10-16`; before 1970 too.
#[must_use]
pub fn iso_date(epoch_ms: i64) -> String {
    let (year, month, day) = civil_from_days(epoch_ms.div_euclid(DAY_MS));

    format!("{year:04}-{month:02}-{day:02}")
}

/// Today's date in UTC, `2026-10-16`.
#[must_use]
pub fn today() -> String {
    iso_date(now_ms())
}

/// The UTC minute of an instant, `2026-10-16T09:05`.
#[must_use]
pub fn iso_minute(epoch_ms: i64) -> String {
    let in_day = epoch_ms.rem_euclid(DAY_MS);

    format!(
        "{}T{:02}:{:02}",
        iso_date(epoch_ms),
        in_day / HOUR_MS,
        in_day % HOUR_MS / MINUTE_MS
    )
}

/// A UTC instant written as ISO 8601 with milliseconds, `2026-10-16T09:05:00.123Z`.
#[must_use]
pub fn iso_instant(epoch_ms: i64) -> String {
    let in_minute = epoch_ms.rem_euclid(MINUTE_MS);

    format!(
        "{}:{:02}.{:03}Z",
        iso_minute(epoch_ms),
        in_minute / SECOND_MS,
        in_minute % SECOND_MS
    )
}

/// An instant as `Temporal.Instant.prototype.toString` writes it by default: UTC, `Z`, the
/// fraction of its second without trailing zeros, none for a whole second
/// (`REF-TEMPORAL-INSTANT`): `2026-10-17T23:33:53.959648Z`, `2026-10-17T23:33:53Z`.
#[must_use]
pub fn temporal_instant(epoch_seconds: i64, nanoseconds: u32) -> String {
    let fraction = format!("{nanoseconds:09}");
    let fraction = fraction.trim_end_matches('0');

    format!(
        "{}:{:02}{}{fraction}Z",
        iso_minute(epoch_seconds.saturating_mul(SECOND_MS)),
        epoch_seconds.rem_euclid(60),
        if fraction.is_empty() { "" } else { "." }
    )
}

/// [`temporal_instant`] of an instant in epoch milliseconds: `2026-10-12T21:00:00.12Z`.
#[must_use]
pub fn temporal_instant_ms(epoch_ms: i64) -> String {
    let nanoseconds = epoch_ms.rem_euclid(SECOND_MS) * MILLISECOND_NS;

    temporal_instant(
        epoch_ms.div_euclid(SECOND_MS),
        u32::try_from(nanoseconds).unwrap_or(0),
    )
}

/// Epoch milliseconds now, 0 for a clock before 1970.
#[must_use]
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| i64::try_from(elapsed.as_millis()).unwrap_or(0))
}
