//! What a JavaScript regular expression of the preset means, written for Rust's `regex`, whose
//! classes are Unicode where JavaScript's are ASCII (`REF-MDN-REGEXP-CLASSES`).
//!
//! - `\s` is JavaScript's white space and line terminators: TAB, VT, FF, ZWNBSP, every
//!   `Space_Separator`, LF, CR, LS and PS, and not NEL (U+0085), which Rust's `\s` holds.
//! - `.` without the `s` flag is any character but the four line terminators, where Rust's
//!   excludes LF only.
//! - `\w`, `\d` and `\b` are ASCII: `[A-Za-z0-9_]`, `[0-9]` and the boundary between them,
//!   `(?-u:\b)` in Rust.
//!
//! Indices are counted in UTF-16 code units, as a JavaScript string's are, so that each finding's
//! range and column read as secretlint's.

use std::sync::LazyLock;

use regex::Regex;

/// JavaScript's `\s`, the inside of a class: spliced into `[…]` or `[^…]`.
pub const SPACE: &str = r"\t\x0B\x0C\x{FEFF}\p{Zs}\n\r\x{2028}\x{2029}";

/// JavaScript's `.` without the `s` flag: any character but a line terminator.
pub const DOT: &str = r"[^\n\r\x{2028}\x{2029}]";

/// JavaScript's `\b`, ASCII only.
pub const BOUNDARY: &str = r"(?-u:\b)";

/// One letter, `\p{L}`: what the preset's look-behinds refuse before a match.
static LETTER: LazyLock<Regex> = LazyLock::new(|| compile(r"^\p{L}$"));

/// Compiles one of the scan's own patterns, which are constants: a failure is a bug, caught by the
/// tests that run every rule.
///
/// # Panics
///
/// The pattern is not a valid Rust regular expression, or is over the size limit.
#[must_use]
pub fn compile(pattern: &str) -> Regex {
    regex::RegexBuilder::new(pattern)
        .size_limit(64 << 20)
        .build()
        .unwrap_or_else(|error| panic!("the pattern {pattern} does not compile: {error}"))
}

/// Whether a character is a letter, as `\p{L}` tests it.
#[must_use]
pub fn is_letter(character: char) -> bool {
    let mut buffer = [0; 4];

    LETTER.is_match(character.encode_utf8(&mut buffer))
}

/// The length of a text in UTF-16 code units, JavaScript's `length`.
#[must_use]
pub fn utf16_len(text: &str) -> usize {
    text.chars().map(char::len_utf16).sum()
}

/// Whether a text is one UTF-16 code unit repeated, at least four times: `^(.)\1{3,}$`, the
/// backreference the preset uses to tell a dummy password (`aaaa`, `1111`), checked in code since
/// `regex` has no backreference. `.` is one code unit in a pattern without the `u` flag, so a
/// character outside the BMP never repeats: its two halves differ.
#[must_use]
pub fn is_one_unit_repeated(text: &str) -> bool {
    let units: Vec<u16> = text.encode_utf16().collect();

    units.len() >= 4
        && units[0] != 0x0A
        && units[0] != 0x0D
        && units[0] != 0x2028
        && units[0] != 0x2029
        && units.iter().all(|unit| *unit == units[0])
}
