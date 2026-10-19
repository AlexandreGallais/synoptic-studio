//! Tests of the transcript times.

use hooks::time::{epoch_ms, short_duration};

#[test]
fn reads_an_instant_to_the_millisecond() {
    // 20 742 days × 86 400 s + 20 h 41 min 3 s = 1 792 183 263 s (`date -u`), then 123 ms.
    assert_eq!(epoch_ms("2026-10-16T20:41:03.123Z"), 1_792_183_263_123);
    assert_eq!(epoch_ms("2026-10-16T20:41:03Z"), 1_792_183_263_000);
    assert_eq!(epoch_ms("2026-10-16T20:41:03.5Z"), 1_792_183_263_500);
    assert_eq!(epoch_ms("2026-10-16T20:41:03.123456Z"), 1_792_183_263_123);
}

#[test]
fn reads_no_instant_from_anything_else() {
    for text in [
        "",
        "2026-10-16",
        "2026-10-16T20:41:03",
        "2026-10-16T20:41:03+02:00",
        "x",
    ] {
        assert_eq!(epoch_ms(text), 0, "{text}");
    }
}

#[test]
fn writes_durations_in_seconds_minutes_then_hours() {
    assert_eq!(short_duration(45_000), "45 s");
    assert_eq!(short_duration(44_600), "45 s");
    // Rounded half up, as `Math.round`: 44.5 s is 45 s.
    assert_eq!(short_duration(44_500), "45 s");
    assert_eq!(short_duration(44_499), "44 s");
    // Below zero, as `Math.round`: −1.5 s is −1 s.
    assert_eq!(short_duration(-1500), "-1 s");
    assert_eq!(short_duration(720_000), "12 min");
    // 125 min = 2 h 05.
    assert_eq!(short_duration(7_500_000), "2 h 05");
}

#[test]
fn chooses_the_unit_from_the_seconds_rounded_as_shown() {
    // AUD-034: 59.5 s rounds to 60 s, so a minute, never `60 s`; 59.499 s stays 59 s.
    assert_eq!(short_duration(59_499), "59 s");
    assert_eq!(short_duration(59_500), "1 min");
    assert_eq!(short_duration(59_999), "1 min");
    assert_eq!(short_duration(60_000), "1 min");
    // The minutes stay rounded down: 119.5 s is 1 min, 3 599.9 s 59 min.
    assert_eq!(short_duration(119_500), "1 min");
    assert_eq!(short_duration(3_599_900), "59 min");
}
