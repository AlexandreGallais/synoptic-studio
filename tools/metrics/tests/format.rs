//! Tests of how the reports write numbers and durations: JavaScript's rounding kept (EN-107).

use metrics::format::{duration_text, format_duration, grouped, js_round, to_fixed};

#[test]
fn rounds_a_tie_up_as_javascript_does() {
    // 12.25 and 0.25 are exact in binary: a tie, which `toFixed` rounds up and Rust's `{:.1}` to
    // even (12.2, 0.2).
    assert_eq!(to_fixed(12.25, 1), "12.3");
    assert_eq!(to_fixed(0.25, 1), "0.3");
    // 12.35 is 12.3499999999999996… in binary: below the tie, down for both.
    assert_eq!(to_fixed(12.35, 1), "12.3");
    // 9.96 rounds up through every digit: 10.0.
    assert_eq!(to_fixed(9.96, 1), "10.0");
    assert_eq!(to_fixed(0.0, 1), "0.0");
    assert_eq!(to_fixed(2.5, 0), "3");
    assert_eq!(to_fixed(-1.25, 1), "-1.3");
}

#[test]
fn rounds_a_half_up_to_a_whole_number() {
    assert!((js_round(2.5) - 3.0).abs() < f64::EPSILON);
    assert!((js_round(2.49) - 2.0).abs() < f64::EPSILON);
}

#[test]
fn writes_milliseconds_seconds_then_minutes_and_seconds() {
    assert_eq!(format_duration(850.0), "850 ms");
    // A median of two calls may hold a half: 850.5 is rounded up.
    assert_eq!(format_duration(850.5), "851 ms");
    assert_eq!(format_duration(12_400.0), "12.4 s");
    // 12 250 ms is 12.25 s, a tie: up, as the TypeScript report wrote it.
    assert_eq!(format_duration(12_250.0), "12.3 s");
    // 185 s = 3 min 5 s.
    assert_eq!(format_duration(185_000.0), "3 min 05 s");
    // 97 min 33 s = 5 853 s.
    assert_eq!(duration_text(5_853_000), "97 min 33 s");
    assert_eq!(duration_text(0), "0 ms");
}

#[test]
fn groups_thousands() {
    assert_eq!(grouped(2_949_836_831), "2,949,836,831");
    assert_eq!(grouped(75), "75");
}

#[test]
fn f32_ac2_carries_the_minute_a_rounded_second_reaches() {
    // 59.6 s rounds to 60 s, a whole minute: 1 min 00 s, never 59.6 s beside a 0 min 60 s (I111).
    assert_eq!(format_duration(59_600.0), "1 min 00 s");
    // 59.4 s rounds to 59 s, under the minute: its tenth kept.
    assert_eq!(format_duration(59_400.0), "59.4 s");
    // 59.5 s is a tie, up to 60 s as `Math.round` does: the minute.
    assert_eq!(format_duration(59_500.0), "1 min 00 s");
    // 1617 min 59.6 s = 97 079.6 s, rounded to 97 080 s = 1618 min 00 s, not 1617 min 60 s.
    assert_eq!(duration_text(97_079_600), "1618 min 00 s");
    // 2 min 59.4 s = 179.4 s, rounded to 179 s = 2 min 59 s.
    assert_eq!(duration_text(179_400), "2 min 59 s");
    // 999.6 ms rounds to 1000 ms, a whole second: 1.0 s, never 1000 ms.
    assert_eq!(format_duration(999.6), "1.0 s");
    assert_eq!(format_duration(999.4), "999 ms");
    // 60 s exactly is the first minute.
    assert_eq!(duration_text(60_000), "1 min 00 s");
}
