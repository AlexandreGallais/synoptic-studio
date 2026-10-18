//! Dates the library reads and writes: a page's last change as it states it, and the days shown
//! in a table of contents and a list. JavaScript read them with Temporal (`Instant.from`,
//! `PlainDate.from`); the same strings are accepted here, the same refused, as an error.

use calendar::{DAY_MS, days_from_civil, days_in_month, duration_ms};

/// Characters of an ISO 8601 instant that give its day (`2026-10-12`).
const DAY_LENGTH: usize = 10;

/// A text read as digits only, `None` when it holds anything else or nothing.
fn digits(text: &str) -> Option<i64> {
    (!text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
}

/// Days from 1970-01-01 to a calendar date `YYYY-MM-DD` or `YYYYMMDD`, the date checked.
fn date_days(date: &str) -> Option<i64> {
    let (year, month, day) = match date.len() {
        10 if date.as_bytes()[4] == b'-' && date.as_bytes()[7] == b'-' => (
            digits(&date[..4])?,
            digits(&date[5..7])?,
            digits(&date[8..])?,
        ),
        8 => (
            digits(&date[..4])?,
            digits(&date[4..6])?,
            digits(&date[6..])?,
        ),
        _ => return None,
    };
    (1..=days_in_month(year, month)?)
        .contains(&day)
        .then(|| days_from_civil(year, month, day))
}

/// Milliseconds after midnight of a time `HH[:MM[:SS[.fraction]]]` or `HHMM[SS]`, its fraction
/// cut to milliseconds; a leap second read as the last second of its minute, as Temporal does.
fn time_ms(time: &str) -> Option<i64> {
    let (clock, fraction) = match time.find(['.', ',']) {
        Some(at) => (&time[..at], Some(&time[at + 1..])),
        None => (time, None),
    };
    let fields: Vec<&str> = if clock.contains(':') {
        clock.split(':').collect()
    } else {
        (0..clock.len())
            .step_by(2)
            .map(|at| clock.get(at..at + 2).unwrap_or(""))
            .collect()
    };

    if fields.is_empty() || fields.len() > 3 || fields.iter().any(|field| field.len() != 2) {
        return None;
    }

    let hour = digits(fields[0])?;
    let minute = fields.get(1).map_or(Some(0), |field| digits(field))?;
    let second = fields
        .get(2)
        .map_or(Some(0), |field| digits(field))?
        .min(59);
    let milliseconds = match fraction {
        Some(fraction) if fields.len() == 3 && (1..=9).contains(&fraction.len()) => {
            digits(&format!("{fraction:0<3}")[..3])?
        }
        Some(_) => return None,
        None => 0,
    };

    (hour < 24 && minute < 60).then_some(duration_ms(hour, minute, second, milliseconds))
}

/// Milliseconds an offset `Z`, `±HH`, `±HH:MM` or `±HHMM` adds to UTC.
fn offset_ms(offset: &str) -> Option<i64> {
    if offset.eq_ignore_ascii_case("z") {
        return Some(0);
    }

    let sign = match offset.as_bytes().first()? {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let rest = offset[1..].replace(':', "");
    let hours = digits(rest.get(..2)?)?;
    let minutes = match rest.len() {
        2 => 0,
        4 => digits(&rest[2..])?,
        _ => return None,
    };

    (hours < 24 && minutes < 60).then_some(sign * duration_ms(hours, minutes, 0, 0))
}

/// Epoch milliseconds of an ISO 8601 instant, as `Temporal.Instant.from` reads it: a date, `T`,
/// `t` or a space, a time and an offset, then any bracketed annotations.
///
/// # Errors
///
/// When the text is not such an instant, as Temporal throws.
pub fn instant_ms(instant: &str) -> Result<i64, String> {
    let refused = || "RangeError: Invalid time value for Temporal".to_owned();
    let bare = instant.split('[').next().unwrap_or_default();
    let separator = bare.find(['T', 't', ' ']).ok_or_else(refused)?;
    let (date, rest) = (&bare[..separator], &bare[separator + 1..]);
    let offset_at = rest.find(['Z', 'z', '+', '-']).ok_or_else(refused)?;
    let days = date_days(date).ok_or_else(refused)?;
    let time = time_ms(&rest[..offset_at]).ok_or_else(refused)?;
    let offset = offset_ms(&rest[offset_at..]).ok_or_else(refused)?;

    if !instant[bare.len()..]
        .split_inclusive(']')
        .all(|annotation| annotation.starts_with('[') && annotation.ends_with(']'))
    {
        return Err(refused());
    }

    Ok(days * DAY_MS + time - offset)
}

/// Epoch milliseconds of a date a page states: an instant, or a calendar date taken at midnight
/// UTC, or 0 when empty.
///
/// # Errors
///
/// When the date is neither, as Temporal threw.
pub fn epoch_of(date: &str) -> Result<i64, String> {
    if date.is_empty() {
        return Ok(0);
    }

    let has_time = date
        .as_bytes()
        .windows(2)
        .any(|pair| matches!(pair[0], b'T' | b' ') && pair[1].is_ascii_digit());

    if has_time {
        return instant_ms(date);
    }

    let day: String = date.chars().take(DAY_LENGTH).collect();

    date_days(&day)
        .map(|days| days * DAY_MS)
        .ok_or_else(|| "RangeError: Invalid time value for Temporal".to_owned())
}

/// A day written for a person: `2026-10-12`, or `not stated` for 0.
#[must_use]
pub fn day_of(epoch: i64) -> String {
    if epoch == 0 {
        "not stated".to_owned()
    } else {
        calendar::iso_date(epoch)
    }
}
