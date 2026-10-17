//! What several Markdown rules read in a page's text, written as the patterns of
//! `@eslint/markdown` (`util.js`, `REF-ESLINT-MARKDOWN-RULES`) read it: HTML comments, white
//! space as JavaScript's `\s` counts it, escapes, identifiers, HTML tags.

/// Whether a character is white space as JavaScript's `\s` counts it.
#[must_use]
pub const fn is_js_space(character: char) -> bool {
    matches!(
        character,
        '\t' | '\n' | '\u{0b}' | '\u{0c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// Whether a byte is a line terminator, which JavaScript's `.` does not match.
#[must_use]
pub const fn is_line_end(byte: u8) -> bool {
    matches!(byte, b'\n' | b'\r')
}

/// Whether the byte at an index is preceded by an even number of backslashes, none included:
/// the pattern `(?<=(?<!\\)(?:\\{2})*)`, a character that no backslash escapes.
#[must_use]
pub fn is_unescaped(bytes: &[u8], index: usize) -> bool {
    let backslashes = bytes[..index]
        .iter()
        .rev()
        .take_while(|byte| **byte == b'\\')
        .count();

    backslashes % 2 == 0
}

/// A text with each HTML comment's characters turned to spaces, its line breaks kept, so that
/// offsets do not move (`stripHtmlComments`).
#[must_use]
pub fn strip_html_comments(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut rest = text;

    while let Some(open) = rest.find("<!--") {
        let Some(close) = rest[open + 4..].find("-->") else {
            break;
        };
        let end = open + 4 + close + 3;
        result.push_str(&rest[..open]);
        result.extend(rest[open..end].chars().map(|character| match character {
            '\n' | '\r' => character,
            _ => ' ',
        }));
        rest = &rest[end..];
    }
    result.push_str(rest);
    result
}

/// A label normalized as `micromark-util-normalize-identifier` does, then lower-cased: white
/// space collapsed and trimmed, case folded, as mdast writes a definition's identifier.
#[must_use]
pub fn normalize_identifier(label: &str) -> String {
    label
        .split(['\t', '\n', '\r', ' '])
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
        .to_uppercase()
        .to_lowercase()
}

/// The end of an HTML attribute list starting at an index, after the tag's name:
/// `(?:\s(?:[^>"']|"[^"]*"|'[^']*')*)?`; the index itself when no white space opens it.
#[must_use]
pub fn attributes_end(text: &str, index: usize) -> usize {
    let mut characters = text[index..].char_indices();
    match characters.next() {
        Some((_, first)) if is_js_space(first) => {}
        _ => return index,
    }
    let bytes = text.as_bytes();
    let mut at = index + text[index..].chars().next().map_or(0, char::len_utf8);

    while at < bytes.len() {
        match bytes[at] {
            b'>' => break,
            quote @ (b'"' | b'\'') => match text[at + 1..].find(char::from(quote)) {
                Some(close) => at += close + 2,
                None => break,
            },
            _ => at += text[at..].chars().next().map_or(1, char::len_utf8),
        }
    }
    at
}

/// The end of a tag's name starting at an index, `[a-z0-9]+(?:-[a-z0-9]+)*` without case;
/// `None` when no name starts there.
#[must_use]
pub fn tag_name_end(text: &str, index: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut at = index;
    let word = |from: usize| {
        bytes[from..]
            .iter()
            .take_while(|byte| byte.is_ascii_alphanumeric())
            .count()
    };
    let first = word(at);
    if first == 0 {
        return None;
    }
    at += first;
    while bytes.get(at) == Some(&b'-') && word(at + 1) > 0 {
        at += 1 + word(at + 1);
    }
    Some(at)
}

/// An opening HTML tag at an index: `<name attributes? /?>`, the pattern of `no-html`; its name
/// and its end.
#[must_use]
pub fn opening_tag(text: &str, index: usize) -> Option<(&str, usize)> {
    if text.as_bytes().get(index) != Some(&b'<') {
        return None;
    }
    let name_end = tag_name_end(text, index + 1)?;
    let mut at = attributes_end(text, name_end);
    let bytes = text.as_bytes();
    if bytes.get(at) == Some(&b'/') {
        at += 1;
    }
    (bytes.get(at) == Some(&b'>')).then(|| (&text[index + 1..name_end], at + 1))
}

/// A text with every HTML tag removed, opening or closing: the pattern
/// `<\/?[a-z0-9]+(?:-[a-z0-9]+)*(?:\s(?:[^>"']|"[^"]*"|'[^']*')*)?(?:\/\s*)?>` of
/// `no-missing-link-fragments`.
#[must_use]
pub fn without_tags(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut result = String::with_capacity(text.len());
    let mut copied = 0;
    let mut at = 0;

    while let Some(open) = text[at..].find('<').map(|found| at + found) {
        let mut name_start = open + 1;
        if bytes.get(name_start) == Some(&b'/') {
            name_start += 1;
        }
        let tag_end = tag_name_end(text, name_start).and_then(|name_end| {
            let mut end = attributes_end(text, name_end);
            if bytes.get(end) == Some(&b'/') {
                end += 1;
                end += text[end..]
                    .chars()
                    .take_while(|character| is_js_space(*character))
                    .map(char::len_utf8)
                    .sum::<usize>();
            }
            (bytes.get(end) == Some(&b'>')).then_some(end + 1)
        });
        match tag_end {
            Some(end) => {
                result.push_str(&text[copied..open]);
                copied = end;
                at = end;
            }
            None => at = open + 1,
        }
    }
    result.push_str(&text[copied..]);
    result
}

/// The first index at or after `from` where a text starts with a word, without case.
#[must_use]
pub fn find_without_case(text: &str, word: &str, from: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let length = word.len();

    (from..=bytes.len().saturating_sub(length))
        .find(|at| bytes[*at..*at + length].eq_ignore_ascii_case(word.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::{
        is_unescaped, normalize_identifier, opening_tag, strip_html_comments, without_tags,
    };

    #[test]
    fn blanks_comments_and_keeps_offsets_and_line_breaks() {
        assert_eq!(
            strip_html_comments("a<!-- b\nc -->d<!-- e"),
            "a      \n     d<!-- e"
        );
    }

    #[test]
    fn counts_the_backslashes_before_a_character() {
        // `a\[`: one backslash, escaped; `a\\[`: two, not escaped.
        assert!(!is_unescaped(br"a\[", 2));
        assert!(is_unescaped(br"a\\[", 3));
        assert!(is_unescaped(b"[", 0));
    }

    #[test]
    fn normalizes_identifiers_as_mdast_does() {
        assert_eq!(normalize_identifier("  Foo \n\t Bar "), "foo bar");
        assert_eq!(normalize_identifier("ẞ"), "ss");
    }

    #[test]
    fn reads_opening_tags_with_quoted_attributes() {
        assert_eq!(
            opening_tag("<span v-pre>", 0),
            Some(("span", "<span v-pre>".len()))
        );
        assert_eq!(opening_tag("<a title=\"x>y\" />z", 0), Some(("a", 17)));
        assert_eq!(opening_tag("<my-tag>", 0), Some(("my-tag", 8)));
        assert_eq!(opening_tag("</a>", 0), None);
        assert_eq!(opening_tag("<a.b>", 0), None);
        assert_eq!(opening_tag("<a title=\"x>", 0), None);
    }

    #[test]
    fn removes_opening_and_closing_tags() {
        assert_eq!(without_tags("<b>Bold</b> <br/ > 1 < 2"), "Bold  1 < 2");
    }
}
