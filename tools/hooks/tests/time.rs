//! Tests of the transcript times.

use hooks::time::{days_from_civil, epoch_ms, short_duration};

#[test]
fn counts_days_from_the_epoch() {
    assert_eq!(days_from_civil(1970, 1, 1), 0);
    // `date -u -j -f %Y-%m-%d 2000-03-01 +%s` ÷ 86 400 = 11 017.
    assert_eq!(days_from_civil(2000, 3, 1), 11_017);
    // 1 709 164 800 s ÷ 86 400 = 19 782: a leap day.
    assert_eq!(days_from_civil(2024, 2, 29), 19_782);
    assert_eq!(days_from_civil(1969, 12, 31), -1);
}

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
fn writes_durations_short() {
    assert_eq!(short_duration(44_600), "45 s");
    assert_eq!(short_duration(720_000), "12 min");
    // 125 min = 2 h 05.
    assert_eq!(short_duration(7_500_000), "2 h 05");
}
