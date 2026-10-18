//! Tests of what the preset's JavaScript expressions mean in Rust (`secret_scan::js`, AUD-023):
//! JavaScript's `\s`, `.` and ASCII `\b`, lengths in UTF-16 code units, and the repeated unit of
//! a dummy password.

use secret_scan::js::{BOUNDARY, DOT, SPACE, compile, is_letter, is_one_unit_repeated, utf16_len};
use secret_scan::pattern::Pattern;

#[test]
fn tells_one_code_unit_repeated_four_times_at_least() {
    assert!(is_one_unit_repeated("aaaa"));
    assert!(is_one_unit_repeated("11111"));
    // Three are too few; two units differ.
    assert!(!is_one_unit_repeated("aaa"));
    assert!(!is_one_unit_repeated("aaab"));
    // `.` matches no line terminator: LF, CR, LS (U+2028), PS (U+2029).
    for terminator in ["\n", "\r", "\u{2028}", "\u{2029}"] {
        assert!(
            !is_one_unit_repeated(&terminator.repeat(4)),
            "{terminator:?}"
        );
    }
    // Outside the BMP a character is two different units, so it never repeats one.
    assert!(!is_one_unit_repeated(&"😀".repeat(4)));
}

#[test]
fn counts_a_length_in_utf16_code_units() {
    // ASCII one unit each, `é` one, `😀` two: 1 + 1 + 2 = 4.
    assert_eq!(utf16_len("aé😀"), 4);
    assert_eq!(utf16_len("😀"), 2);
    assert_eq!(utf16_len(""), 0);
}

#[test]
fn reads_space_dot_and_boundary_as_javascript_does() {
    let space = compile(&format!("^[{SPACE}]$"));
    let dot = compile(&format!("^{DOT}$"));
    let boundary = compile(&format!("{BOUNDARY}ab"));

    // `\s` holds TAB, VT, FF, ZWNBSP, NBSP (a `Zs`) and the line terminators, not NEL (U+0085).
    for character in [
        "\t", "\u{0B}", "\u{0C}", "\u{FEFF}", "\u{A0}", "\n", "\u{2028}",
    ] {
        assert!(space.is_match(character), "{character:?}");
    }
    assert!(!space.is_match("\u{85}"));
    // `.` is any character but a line terminator: it matches NEL, not LS nor LF.
    assert!(dot.is_match("\u{85}"));
    assert!(!dot.is_match("\u{2028}"));
    assert!(!dot.is_match("\n"));
    // `\b` is ASCII: `é` is no word character, so a boundary stands before `ab` in `éab`, and
    // none inside `xab`.
    assert!(boundary.is_match("éab"));
    assert!(!boundary.is_match("xab"));
}

#[test]
fn refuses_a_match_after_a_letter_only() {
    assert!(is_letter('é'));
    // `Ⅻ` (U+216B) is a number, `Nl`, not a letter: a match after it stands.
    assert!(!is_letter('Ⅻ'));
    assert_eq!(Pattern::bounded("ab", "a-z").find_all("Ⅻab").len(), 1);
    assert_eq!(Pattern::bounded("ab", "a-z").find_all("éab").len(), 0);
    // Nor before a character of the class `ahead`.
    assert_eq!(Pattern::bounded("ab", "a-z").find_all("abc").len(), 0);
}
