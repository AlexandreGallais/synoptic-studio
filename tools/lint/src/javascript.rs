//! What JavaScript's built-ins read in a text, for the rules that port a plugin's JavaScript:
//! white space as `\s` counts it and `trim` takes it off, `Number(text)`, `parseInt(text, 10)`,
//! `parseFloat(text)` and the `isNumeric` the plugins write with them, each after ECMAScript's
//! conversions
//! (`StringToNumber`, `parseInt`, `parseFloat`). Shared by the Markdown pages, the templates and
//! the style sheets; the tests are this module's own cases.

/// Whether a character is white space as JavaScript's `\s` counts it.
#[must_use]
pub const fn is_js_space(character: char) -> bool {
    matches!(
        character,
        '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// A text without the white space at its ends, as JavaScript's `trim` takes it off (the
/// characters of `\s`, the byte order mark among them; not U+0085, which Rust's `trim` takes).
#[must_use]
pub fn js_trim(text: &str) -> &str {
    text.trim_matches(is_js_space)
}

/// JavaScript's reading of white space on a text, for the ports of its code: `trim`,
/// `trimStart`, `trimEnd` and `split(/\s+/)` without the empty parts.
pub trait JsText {
    /// The text without the white space at its ends (`trim`).
    fn js_trim(&self) -> &str;
    /// The text without the white space at its start (`trimStart`).
    fn js_trim_start(&self) -> &str;
    /// The text without the white space at its end (`trimEnd`).
    fn js_trim_end(&self) -> &str;
    /// The words of the text between runs of white space.
    fn js_split_whitespace(&self) -> impl Iterator<Item = &str>;
}

impl JsText for str {
    fn js_trim(&self) -> &str {
        js_trim(self)
    }

    fn js_trim_start(&self) -> &str {
        self.trim_start_matches(is_js_space)
    }

    fn js_trim_end(&self) -> &str {
        self.trim_end_matches(is_js_space)
    }

    fn js_split_whitespace(&self) -> impl Iterator<Item = &str> {
        self.split(is_js_space).filter(|word| !word.is_empty())
    }
}

/// A text as JavaScript's `Number(text)` reads it: its whitespace trimmed, empty as 0, a decimal
/// literal, `Infinity` with its sign, or an unsigned `0x`, `0o` or `0b` integer; `NaN` otherwise.
#[must_use]
pub fn to_number(text: &str) -> f64 {
    let trimmed = text.trim_matches(is_js_space);
    if trimmed.is_empty() {
        return 0.0;
    }
    let radix = match trimmed.get(..2) {
        Some("0x" | "0X") => 16,
        Some("0o" | "0O") => 8,
        Some("0b" | "0B") => 2,
        _ => 10,
    };
    if radix != 10 {
        let digits = &trimmed[2..];
        if digits.is_empty() || !digits.chars().all(|each| each.is_digit(radix)) {
            return f64::NAN;
        }
        // Every digit checked above, `filter_map` drops none.
        return digits
            .chars()
            .filter_map(|each| each.to_digit(radix))
            .fold(0.0, |value, digit| {
                value * f64::from(radix) + f64::from(digit)
            });
    }
    let (sign, body) = match trimmed.as_bytes().first() {
        Some(b'-') => (-1.0, &trimmed[1..]),
        Some(b'+') => (1.0, &trimmed[1..]),
        _ => (1.0, trimmed),
    };
    if body == "Infinity" {
        return sign * f64::INFINITY;
    }
    // Rust reads `inf` and `nan` too, which JavaScript does not: only digits, a point and an
    // exponent are let through, with a digit before the exponent.
    let mantissa = body.split(['e', 'E']).next().unwrap_or_default();
    let decimal = body
        .chars()
        .all(|each| each.is_ascii_digit() || matches!(each, '.' | 'e' | 'E' | '+' | '-'))
        && mantissa.chars().any(|each| each.is_ascii_digit())
        && !body.starts_with(['+', '-']);
    if !decimal {
        return f64::NAN;
    }
    body.parse::<f64>().map_or(f64::NAN, |value| sign * value)
}

/// The leading number of a text, as JavaScript's `parseInt(text, 10)` reads it; `NaN` when none.
#[must_use]
pub fn parse_int(text: &str) -> f64 {
    let rest = text.trim_start_matches(is_js_space);
    let (sign, digits) = match rest.as_bytes().first() {
        Some(b'-') => (-1.0, &rest[1..]),
        Some(b'+') => (1.0, &rest[1..]),
        _ => (1.0, rest),
    };
    let end = digits
        .find(|each: char| !each.is_ascii_digit())
        .unwrap_or(digits.len());
    digits[..end]
        .parse::<f64>()
        .map_or(f64::NAN, |value| sign * value)
}

/// The leading number of a text, as JavaScript's `parseFloat` reads it; `NaN` when none.
#[must_use]
pub fn parse_float(text: &str) -> f64 {
    let rest = text.trim_start_matches(is_js_space);
    let (sign, body) = match rest.as_bytes().first() {
        Some(b'-') => (-1.0, &rest[1..]),
        Some(b'+') => (1.0, &rest[1..]),
        _ => (1.0, rest),
    };
    if body.starts_with("Infinity") {
        return sign * f64::INFINITY;
    }
    let bytes = body.as_bytes();
    let digits = |from: usize| {
        bytes[from..]
            .iter()
            .take_while(|each| each.is_ascii_digit())
            .count()
    };
    let whole = digits(0);
    let mut end = whole;
    let mut fraction = 0;
    if bytes.get(end) == Some(&b'.') {
        fraction = digits(end + 1);
        if whole + fraction > 0 {
            end += 1 + fraction;
        }
    }
    if whole + fraction == 0 {
        return f64::NAN;
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let signed = usize::from(matches!(bytes.get(end + 1), Some(b'+' | b'-')));
        let exponent = digits(end + 1 + signed);
        if exponent > 0 {
            end += 1 + signed + exponent;
        }
    }
    body[..end]
        .parse::<f64>()
        .map_or(f64::NAN, |value| sign * value)
}

/// Whether a text is a number as the plugins' `isNumeric` tells, `!isNaN(parseFloat(text)) &&
/// isFinite(text)`: a leading number, and a finite one once the whole text is read.
#[must_use]
pub fn is_numeric(text: &str) -> bool {
    !parse_float(text).is_nan() && to_number(text).is_finite()
}

#[cfg(test)]
mod tests {
    use super::{JsText, is_js_space, is_numeric, js_trim, parse_float, parse_int, to_number};

    #[test]
    fn reads_whitespace_as_javascript_does() {
        assert!(is_js_space('\u{feff}') && is_js_space('\u{2009}'));
        // NEL is whitespace for Rust, not for `\s`.
        assert!(!is_js_space('\u{85}') && '\u{85}'.is_whitespace());
    }

    #[test]
    fn trims_and_splits_as_javascript_does() {
        // `"\u{feff} a \u{85}".trim()` is `"a \u{85}"`: the mark is `\s`, NEL is not.
        assert_eq!(js_trim("\u{feff} a \u{85}"), "a \u{85}");
        assert_eq!(" a ".js_trim_start(), "a ");
        assert_eq!("\u{feff}a\u{feff}".js_trim_end(), "\u{feff}a");
        assert_eq!(
            "a\u{feff} b\u{85}c  "
                .js_split_whitespace()
                .collect::<Vec<_>>(),
            ["a", "b\u{85}c"]
        );
    }

    #[test]
    fn reads_a_number_as_javascript_does() {
        // Our own cases, after ECMAScript's StringToNumber.
        assert_eq!(to_number(" 12 "), 12.0);
        assert_eq!(to_number(""), 0.0);
        assert_eq!(to_number("-1.5e1"), -15.0);
        assert_eq!(to_number(".5"), 0.5);
        assert_eq!(to_number("0x1F"), 31.0);
        // 0o17 is 1 × 8 + 7 = 15; 0b101 is 4 + 0 + 1 = 5; 0O7 and 0B1 in upper case.
        assert_eq!(to_number("0o17"), 15.0);
        assert_eq!(to_number("0O7"), 7.0);
        assert_eq!(to_number("0b101"), 5.0);
        assert_eq!(to_number("0B1"), 1.0);
        assert_eq!(to_number("-Infinity"), f64::NEG_INFINITY);
        // U+FEFF is JavaScript's white space, not Rust's.
        assert_eq!(to_number("\u{feff}3\u{feff}"), 3.0);
        for text in [
            "inf", "NaN", "1px", "0x", "-0x1", "+-1", "e5", "1e", "0o8", "0b2", "0o", "0b",
        ] {
            assert!(to_number(text).is_nan(), "{text}");
        }
    }

    #[test]
    fn reads_a_leading_number_as_javascript_does() {
        // Our own cases, after ECMAScript's parseInt and parseFloat.
        assert_eq!(parse_int("  12abc"), 12.0);
        assert_eq!(parse_int("-3.9"), -3.0);
        assert!(parse_int("abc").is_nan() && parse_int("").is_nan());
        assert_eq!(parse_float(" 1.5e2x"), 150.0);
        assert_eq!(parse_float(".5"), 0.5);
        assert_eq!(parse_float("1e"), 1.0);
        assert_eq!(parse_float("-Infinity"), f64::NEG_INFINITY);
        assert!(parse_float(".").is_nan());
    }

    #[test]
    fn tells_a_number_as_the_plugins_is_numeric_does() {
        // Our own cases: `parseFloat` gives a number and `Number` a finite one.
        for numeric in [
            "3",
            " 3 ",
            "-1.5e3",
            ".5",
            "5.",
            "0x1F",
            "0b10",
            "0o7",
            "\u{feff}3",
        ] {
            assert!(is_numeric(numeric), "{numeric}");
        }
        for text in [
            "", " ", "null", "3abc", "Infinity", "1e", "0x", "inf", "1_0", "+",
        ] {
            assert!(!is_numeric(text), "{text}");
        }
    }
}
