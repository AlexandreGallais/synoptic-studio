//! What the comment rules share: `JavaScript`'s white space and line breaks, as `\s` and the
//! plugin's `LINEBREAK_MATCHER` read them.

/// Whether a character is `JavaScript`'s white space, a line break included (`\s`).
#[must_use]
pub fn is_space(character: char) -> bool {
    character.is_whitespace() || character == '\u{feff}'
}

/// Whether a text is only white space (`/^\s*$/`).
#[must_use]
pub fn is_blank(text: &str) -> bool {
    text.chars().all(is_space)
}

/// The lines of a text, split at `\r\n`, `\r`, `\n`, U+2028 and U+2029.
#[must_use]
pub fn lines(text: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut characters = text.char_indices().peekable();

    while let Some((at, character)) = characters.next() {
        if matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}') {
            lines.push(&text[start..at]);
            let mut end = at + character.len_utf8();
            if character == '\r' && characters.peek().is_some_and(|(_, next)| *next == '\n') {
                characters.next();
                end += 1;
            }
            start = end;
        }
    }
    lines.push(&text[start..]);
    lines
}

/// Whether a text holds a line break.
#[must_use]
pub fn has_line_break(text: &str) -> bool {
    text.contains(['\n', '\r', '\u{2028}', '\u{2029}'])
}

#[cfg(test)]
mod tests {
    use super::{has_line_break, is_blank, lines};

    #[test]
    fn splits_at_every_line_break() {
        assert_eq!(lines("a\nb\r\nc\rd\u{2028}e"), ["a", "b", "c", "d", "e"]);
        assert_eq!(lines(""), [""]);
        assert_eq!(lines("a\n"), ["a", ""]);
    }

    #[test]
    fn tells_blank_text_and_line_breaks() {
        assert!(is_blank(" \t\u{feff}\n"));
        assert!(!is_blank(" a"));
        assert!(has_line_break("a\u{2029}"));
        assert!(!has_line_break("a b"));
    }
}
