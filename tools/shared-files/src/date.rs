//! Today's date, `2026-10-17`, in UTC, standard library only: the journal opens a section for it
//! (AUD-020).

use std::time::{SystemTime, UNIX_EPOCH};

/// Seconds in a day.
const DAY: u64 = 86_400;

/// The date of the proleptic Gregorian calendar a number of days after 1970-01-01 falls on, written
/// `YYYY-MM-DD` (H. Hinnant, `civil_from_days`, "chrono-Compatible Low-Level Date Algorithms").
#[must_use]
pub fn civil_from_days(days: i64) -> String {
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

    format!("{year:04}-{month:02}-{day:02}")
}

/// Today's date in UTC, `YYYY-MM-DD`.
#[must_use]
pub fn today() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());

    civil_from_days(i64::try_from(seconds / DAY).unwrap_or(0))
}
