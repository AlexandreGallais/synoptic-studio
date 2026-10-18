//! Tests of the calendar: the days of a date and back, the days of a month, durations, instants
//! written. Moved here with their functions from `hooks`, `tool-use`, `shared-files`,
//! `repo-tools` and `library` (NAM-002), their cases unchanged.

use calendar::{
    DAY_MS, HOUR_MS, MINUTE_MS, SECOND_MS, civil_from_days, days_from_civil, days_in_month,
    duration_ms, is_leap, iso_date, iso_instant, iso_minute, now_ms, temporal_instant,
    temporal_instant_ms, today,
};

#[test]
fn counts_the_milliseconds_of_a_second_a_minute_an_hour_and_a_day() {
    assert_eq!(SECOND_MS, 1000);
    assert_eq!(MINUTE_MS, 60_000);
    assert_eq!(HOUR_MS, 3_600_000);
    // 24 × 3 600 000 = 86 400 000.
    assert_eq!(DAY_MS, 86_400_000);
}

#[test]
fn counts_days_from_the_epoch() {
    assert_eq!(days_from_civil(1970, 1, 1), 0);
    // `date -u -j -f %Y-%m-%d 2000-03-01 +%s` ÷ 86 400 = 11 017.
    assert_eq!(days_from_civil(2000, 3, 1), 11_017);
    // 1 709 164 800 s ÷ 86 400 = 19 782: a leap day.
    assert_eq!(days_from_civil(2024, 2, 29), 19_782);
    assert_eq!(days_from_civil(1969, 12, 31), -1);
    // 2000-03-01, after the leap day of a year divisible by 400: day 11 017.
    assert_eq!(civil_from_days(11_017), (2000, 3, 1));
    assert_eq!(civil_from_days(11_016), (2000, 2, 29));
}

#[test]
fn every_day_from_1900_to_2200_is_the_next_date_and_counts_back_to_its_number() {
    // 1900-01-01 is day −25 567; each next day is the next date of the calendar, within its
    // month's days.
    let mut previous = civil_from_days(-25_567);
    assert_eq!(previous, (1900, 1, 1));
    for days in -25_566..84_000 {
        let date = civil_from_days(days);
        let next_in_month = (previous.0, previous.1, previous.2 + 1);
        let next_month = if previous.1 == 12 {
            (previous.0 + 1, 1, 1)
        } else {
            (previous.0, previous.1 + 1, 1)
        };
        assert!(
            date == next_in_month || date == next_month,
            "{previous:?} then {date:?}"
        );
        assert!(
            date.2 <= days_in_month(date.0, date.1).unwrap_or(0),
            "{date:?}"
        );
        assert_eq!(days_from_civil(date.0, date.1, date.2), days);
        previous = date;
    }
}

#[test]
fn tells_a_leap_year_by_4_100_and_400() {
    // 2024 = 4 × 506; 2026 is not divisible by 4; 1900 = 100 × 19, not by 400; 2000 = 400 × 5.
    assert!(is_leap(2024));
    assert!(!is_leap(2026));
    assert!(!is_leap(1900));
    assert!(is_leap(2000));
    // The proleptic year 0 is 400 × 0; −100 is 100 × −1, not a multiple of 400.
    assert!(is_leap(0));
    assert!(!is_leap(-100));
}

#[test]
fn counts_the_days_of_each_month() {
    let common: Vec<Option<i64>> = (1..=12).map(|month| days_in_month(2026, month)).collect();
    let thirty_one = Some(31);
    let thirty = Some(30);

    assert_eq!(
        common,
        [
            thirty_one,
            Some(28),
            thirty_one,
            thirty,
            thirty_one,
            thirty,
            thirty_one,
            thirty_one,
            thirty,
            thirty_one,
            thirty,
            thirty_one
        ]
    );
    assert_eq!(days_in_month(2024, 2), Some(29));
    assert_eq!(days_in_month(1900, 2), Some(28));
    assert_eq!(days_in_month(2026, 0), None);
    assert_eq!(days_in_month(2026, 13), None);
}

#[test]
fn a_month_has_the_days_between_its_first_and_the_next_month_s_first() {
    for year in 1600..2400 {
        for month in 1..=12 {
            let next = if month == 12 {
                days_from_civil(year + 1, 1, 1)
            } else {
                days_from_civil(year, month + 1, 1)
            };

            assert_eq!(
                days_in_month(year, month),
                Some(next - days_from_civil(year, month, 1)),
                "{year}-{month}"
            );
        }
    }
}

#[test]
fn adds_hours_minutes_seconds_and_milliseconds() {
    // 20 × 3 600 000 + 41 × 60 000 + 3 × 1 000 + 123 = 72 000 000 + 2 460 000 + 3 123.
    assert_eq!(duration_ms(20, 41, 3, 123), 74_463_123);
    // An offset of −02:30: −(2 × 3 600 000 + 30 × 60 000) = −9 000 000.
    assert_eq!(-duration_ms(2, 30, 0, 0), -9_000_000);
    assert_eq!(duration_ms(0, 0, 0, 0), 0);
    assert_eq!(duration_ms(24, 0, 0, 0), DAY_MS);
}

#[test]
fn writes_the_date_of_an_instant() {
    // 0 days: 1970-01-01; one millisecond before: the day before.
    assert_eq!(iso_date(0), "1970-01-01");
    assert_eq!(iso_date(-1), "1969-12-31");
    assert_eq!(iso_date(-DAY_MS), "1969-12-31");
    // 31 days of January + 28 of February 1970: 1970-03-01.
    assert_eq!(iso_date(59 * DAY_MS), "1970-03-01");
    // 30 years of 365 days + 7 leap days (1972 … 1996) = 10 957: 2000-01-01; + 31 + 28 = 11 016:
    // 2000-02-29, a leap day of a year divisible by 400.
    assert_eq!(iso_date(10_957 * DAY_MS), "2000-01-01");
    assert_eq!(iso_date(11_016 * DAY_MS + DAY_MS - 1), "2000-02-29");
    // 2026-10-17: 56 years × 365 = 20 440 days + 14 leap days (1972 … 2024) = 20 454, then
    // 273 + 16 = 289 days of 2026 before it (Jan 31, Feb 28, Mar 31, Apr 30, May 31, Jun 30,
    // Jul 31, Aug 31, Sep 30 = 273) → 20 743.
    assert_eq!(iso_date(20_743 * DAY_MS), "2026-10-17");
    // 1 760 000 000 000 ms ÷ 86 400 000 = 20 370.37 days: 2025-10-09.
    assert_eq!(iso_date(1_760_000_000_000), "2025-10-09");
}

#[test]
fn writes_the_minute_of_an_instant() {
    // 59 minutes before 1970: the minute counts back across midnight.
    assert_eq!(iso_minute(-60_000), "1969-12-31T23:59");
    // 1 792 108 800 000 ms is 2026-10-16T00:00 (20 742 days); + 3 723 045 ms = 1 h 2 min 3 s.
    assert_eq!(
        iso_minute(1_792_108_800_000 + 3_723_045),
        "2026-10-16T01:02"
    );
}

#[test]
fn writes_instants_at_the_epoch_on_a_leap_day_and_before_1970() {
    assert_eq!(iso_instant(0), "1970-01-01T00:00:00.000Z");
    // 2024-02-29: 54 years of 365 days and the 13 leap days 1972 to 2020 give 2024-01-01 at day
    // 19 723; then 31 + 28 days: 19 782 × 86 400 000 = 1 709 164 800 000 ms.
    assert_eq!(iso_instant(1_709_164_800_000), "2024-02-29T00:00:00.000Z");
    assert_eq!(iso_instant(-1), "1969-12-31T23:59:59.999Z");
    assert_eq!(
        iso_instant(1_792_108_800_000 + 3_723_045),
        "2026-10-16T01:02:03.045Z"
    );
}

#[test]
fn writes_instants_as_temporal_does() {
    // 1 792 108 800 s is 2026-10-16T00:00:00 (20 742 days × 86 400 s).
    assert_eq!(temporal_instant(1_792_108_800, 0), "2026-10-16T00:00:00Z");
    // + 3 723 s = 1 h 2 min 3 s; the fraction's trailing zeros dropped.
    assert_eq!(
        temporal_instant(1_792_108_800 + 3_723, 959_648_000),
        "2026-10-16T01:02:03.959648Z"
    );
    assert_eq!(temporal_instant(0, 5), "1970-01-01T00:00:00.000000005Z");
    // 1 791 838 800 000 ms is 2026-10-12T21:00:00 (20 738 days + 21 h).
    assert_eq!(
        temporal_instant_ms(1_791_838_800_000),
        "2026-10-12T21:00:00Z"
    );
    assert_eq!(
        temporal_instant_ms(1_791_838_800_120),
        "2026-10-12T21:00:00.12Z"
    );
    assert_eq!(
        temporal_instant_ms(1_791_838_800_007),
        "2026-10-12T21:00:00.007Z"
    );
    // `REF-TEMPORAL-INSTANT`'s own example.
    assert_eq!(
        temporal_instant_ms(1_574_074_321_816),
        "2019-11-18T10:52:01.816Z"
    );
    // One millisecond before 1970: the second before, 999 ms into it.
    assert_eq!(temporal_instant_ms(-1), "1969-12-31T23:59:59.999Z");
}

#[test]
fn reads_today_from_the_clock() {
    assert_eq!(today().len(), 10);
    assert!(today().as_str() > "2026-10-16");
    // 2026-10-16T00:00:00Z.
    assert!(now_ms() > 1_792_108_800_000);
}
