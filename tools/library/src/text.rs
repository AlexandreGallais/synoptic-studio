//! Text as JavaScript reads it, where the stored text depends on it: its white space (`\s`,
//! `trim()`), its line terminators (the `.` and `$` of a pattern) and its length in UTF-16 code
//! units (`length`), so that a page is cut and described as `library-chunks.ts` did.

/// Whether a character is white space for JavaScript: `\s` of a pattern and what `trim()` removes
/// (ECMA-262, `WhiteSpace` and `LineTerminator`): unlike Rust's, U+FEFF is one and U+0085 is not.
#[must_use]
pub fn is_space(character: char) -> bool {
    matches!(
        character,
        '\t' | '\n' | '\u{B}' | '\u{C}' | '\r' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
                | '\u{FEFF}'
    )
}

/// Whether a character ends a line for a JavaScript pattern: the `.` never matches it.
#[must_use]
pub fn is_line_end(character: char) -> bool {
    matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// A text without its leading and trailing white space, as `trim()` gives it.
#[must_use]
pub fn trim(text: &str) -> &str {
    text.trim_matches(is_space)
}

/// A text without its trailing white space, as `trimEnd()` gives it.
#[must_use]
pub fn trim_end(text: &str) -> &str {
    text.trim_end_matches(is_space)
}

/// The length of a text as JavaScript counts it, in UTF-16 code units.
#[must_use]
pub fn length(text: &str) -> usize {
    text.chars().map(char::len_utf16).sum()
}

/// Whether a character is a word character of a pattern (`\w`, `\b`).
#[must_use]
pub fn is_word(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}
