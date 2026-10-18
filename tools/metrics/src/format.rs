//! How the reports write numbers and durations, as the TypeScript scripts they replace wrote them
//! (EN-107): JavaScript's rounding, which differs from Rust's on a tie.

pub use run_totals::report::grouped;

/// Milliseconds in a second, as a float: the reports' durations are JavaScript numbers. Declared
/// once for the reports (NAM-002); `calendar::SECOND_MS` is the integer one.
pub const SECOND_MS: f64 = 1000.0;

/// Seconds in a minute.
const MINUTE_SECONDS: f64 = 60.0;

/// Decimal digits written before the rounding is decided: more than any duration's exact value
/// has, so that a tie is seen as one.
const EXACT_DIGITS: usize = 80;

/// A number rounded to a whole one as JavaScript's `Math.round` does: a half goes up.
#[must_use]
pub fn js_round(value: f64) -> f64 {
    (value + 0.5).floor()
}

/// A number written with `digits` decimals as JavaScript's `toFixed` does: the exact value
/// rounded, a tie going up ("if there are two such n, pick the larger n", ECMA-262,
/// `Number.prototype.toFixed`), where Rust's `{:.1}` rounds a tie to even (`12.25` is `12.3`, not
/// `12.2`).
#[must_use]
pub fn to_fixed(value: f64, digits: usize) -> String {
    let exact = format!("{:.EXACT_DIGITS$}", value.abs());
    let (whole, fraction) = exact.split_once('.').unwrap_or((&exact, ""));
    let mut kept: Vec<u8> = whole.bytes().chain(fraction.bytes().take(digits)).collect();

    if fraction
        .as_bytes()
        .get(digits)
        .is_some_and(|next| *next >= b'5')
    {
        let mut at = kept.len();

        loop {
            if at == 0 {
                kept.insert(0, b'1');
                break;
            }
            at -= 1;
            if kept[at] == b'9' {
                kept[at] = b'0';
            } else {
                kept[at] += 1;
                break;
            }
        }
    }

    let text = String::from_utf8_lossy(&kept).into_owned();
    let point = text.len() - digits;
    let sign = if value < 0.0 { "-" } else { "" };

    if digits == 0 {
        format!("{sign}{text}")
    } else {
        format!("{sign}{}.{}", &text[..point], &text[point..])
    }
}

/// A whole number of a JavaScript number, as `String()` writes it.
fn whole_text(value: f64) -> String {
    to_fixed(value, 0)
}

/// A duration written for a person: `850 ms`, `12.4 s` or `3 min 05 s`.
#[must_use]
pub fn format_duration(milliseconds: f64) -> String {
    let seconds = milliseconds / SECOND_MS;

    if seconds < 1.0 {
        return format!("{} ms", whole_text(js_round(milliseconds)));
    }

    if seconds < MINUTE_SECONDS {
        return format!("{} s", to_fixed(seconds, 1));
    }

    let minutes = (seconds / MINUTE_SECONDS).floor();

    format!(
        "{} min {:0>2} s",
        whole_text(minutes),
        whole_text(js_round(seconds % MINUTE_SECONDS))
    )
}

/// A duration of whole milliseconds written for a person.
#[must_use]
pub fn duration_text(milliseconds: i64) -> String {
    // Durations of a report are far below 2^53 milliseconds.
    #[allow(clippy::cast_precision_loss)]
    format_duration(milliseconds as f64)
}
