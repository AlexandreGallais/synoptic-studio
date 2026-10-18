//! Today's date, `2026-10-17`, in UTC, standard library only: the journal opens a section for it
//! (AUD-020).

use std::time::{SystemTime, UNIX_EPOCH};

/// Seconds in a day.
const DAY: u64 = 86_400;

/// The date of the proleptic Gregorian calendar a number of days after 1970-01-01 falls on, written
/// `YYYY-MM-DD`, from `tool_use`'s `civil_from_days` (`REF-HINNANT-DATES`).
#[must_use]
pub fn date_of_days(days: i64) -> String {
    let (year, month, day) = tool_use::civil_from_days(days);

    format!("{year:04}-{month:02}-{day:02}")
}

/// Today's date in UTC, `YYYY-MM-DD`.
#[must_use]
pub fn today() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());

    date_of_days(i64::try_from(seconds / DAY).unwrap_or(0))
}
