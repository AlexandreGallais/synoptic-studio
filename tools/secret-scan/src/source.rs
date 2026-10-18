//! A file to scan: its path, its bytes and its text, and the finding a rule reports in it.
//!
//! The text is the bytes decoded as UTF-8, an invalid sequence replaced by U+FFFD as Node.js's
//! `Buffer#toString` does, a byte order mark at its start removed as `@secretlint/core` 13.0.7's
//! source code removes it, so that ranges count from the character after it. A file is text when
//! it holds no NUL byte: the scan's own rule, where secretlint asks `istextorbinary` (not ported:
//! its licence, Artistic-2.0, is none ADR-0076 lets the tools port); only the GCP rule reads a
//! binary file.
//!
//! Lines and columns follow `structured-source` 4.0.0, which `@secretlint/core` reads them with: a
//! line ends at LF, CR, CR LF, LS or PS; a line counts from 1, a column from 0, both in UTF-16
//! code units.

use crate::js::utf16_len;

/// A file to scan.
#[derive(Debug)]
pub struct Source<'a> {
    /// Its path, as the scan was given it.
    pub path: &'a str,
    /// Its bytes, as read.
    pub bytes: &'a [u8],
    /// Its text, without the byte order mark.
    pub content: String,
    /// Whether it is text, holding no NUL byte.
    pub is_text: bool,
}

/// What a rule reports: the rule, the message and the values it masks, and where in the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The rule's id, `@secretlint/secretlint-rule-github`.
    pub rule_id: &'static str,
    /// The message's id, `GITHUB_TOKEN`.
    pub message_id: &'static str,
    /// The message, its values in clear.
    pub message: String,
    /// The values the message holds, each masked in the report, in the order the rule gives them.
    pub data: Vec<String>,
    /// Its start and end in the text, in UTF-16 code units.
    pub range: (usize, usize),
}

impl<'a> Source<'a> {
    /// The source of a file's bytes.
    #[must_use]
    pub fn new(path: &'a str, bytes: &'a [u8]) -> Self {
        let decoded = String::from_utf8_lossy(bytes);
        let content = decoded
            .strip_prefix('\u{FEFF}')
            .unwrap_or(&decoded)
            .to_owned();

        Self {
            path,
            bytes,
            content,
            is_text: !bytes.contains(&0),
        }
    }

    /// Whether the path ends with the extension, `.p12`, as Node.js's `path.extname` gives it:
    /// the file name is longer than the extension, so that a file named `.p12` has none.
    #[must_use]
    pub fn has_extension(&self, extension: &str) -> bool {
        let name = file_name(self.path);

        name.ends_with(extension) && name.len() > extension.len()
    }

    /// The UTF-16 index of a byte of the text.
    #[must_use]
    pub fn index(&self, byte: usize) -> usize {
        utf16_len(&self.content[..byte])
    }

    /// The range a rule reports for a match starting at the byte `start`: from there, the UTF-16
    /// length of `text`, the match or the group the rule names.
    #[must_use]
    pub fn range(&self, start: usize, text: &str) -> (usize, usize) {
        let index = self.index(start);

        (index, index + utf16_len(text))
    }

    /// The UTF-16 index where each line starts, the first at 0: one more after each line end,
    /// the last one included (`structured-source`).
    #[must_use]
    pub fn line_starts(&self) -> Vec<usize> {
        let mut starts = vec![0];
        let mut index = 0;
        let mut characters = self.content.chars().peekable();

        while let Some(character) = characters.next() {
            index += character.len_utf16();
            match character {
                '\r' if characters.peek() == Some(&'\n') => {
                    characters.next();
                    index += 1;
                    starts.push(index);
                }
                '\n' | '\r' | '\u{2028}' | '\u{2029}' => starts.push(index),
                _ => {}
            }
        }
        starts
    }
}

/// The line, from 1, and the column, from 0, of a UTF-16 index, given the lines' starts.
#[must_use]
pub fn position(line_starts: &[usize], index: usize) -> (usize, usize) {
    let line = line_starts.partition_point(|start| *start <= index);

    (line, index - line_starts[line - 1])
}

/// The last part of a path, after its last `/`.
#[must_use]
pub fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_byte_order_mark_is_removed_and_invalid_bytes_replaced() {
        // EF BB BF is U+FEFF in UTF-8; FF is no UTF-8 byte: one U+FFFD.
        let source = Source::new("a.txt", b"\xEF\xBB\xBFab\xFF");

        assert_eq!(source.content, "ab\u{FFFD}");
        assert!(source.is_text);
        assert!(!Source::new("a.bin", b"a\0b").is_text);
    }

    #[test]
    fn the_extension_needs_a_name_before_it() {
        // Node.js: extname("a.p12") is ".p12", extname(".p12") and extname("a/.p12") are "".
        assert!(Source::new("a.p12", b"").has_extension(".p12"));
        assert!(Source::new("d/..p12", b"").has_extension(".p12"));
        assert!(!Source::new(".p12", b"").has_extension(".p12"));
        assert!(!Source::new("d/.p12", b"").has_extension(".p12"));
        assert!(!Source::new("a.P12", b"").has_extension(".p12"));
    }

    #[test]
    fn indices_count_utf16_code_units() {
        // "é😀x": é is 1 unit (2 bytes), 😀 2 units (4 bytes): x is at byte 6, unit 3.
        let source = Source::new("a.txt", "é😀x".as_bytes());

        assert_eq!(source.index(6), 3);
        assert_eq!(source.range(2, "😀x"), (1, 4));
    }

    #[test]
    fn lines_end_at_every_javascript_line_terminator() {
        // "a\r\nb\rc\nd\u{2028}e\n": lines start at 0, 3 (after CR LF), 5, 7, 9 and 11, the
        // last after the final LF.
        let source = Source::new("a.txt", "a\r\nb\rc\nd\u{2028}e\n".as_bytes());
        let starts = source.line_starts();

        assert_eq!(starts, [0, 3, 5, 7, 9, 11]);
        assert_eq!(position(&starts, 0), (1, 0));
        assert_eq!(position(&starts, 4), (2, 1));
        assert_eq!(position(&starts, 11), (6, 0));
    }

    #[test]
    fn the_file_name_is_the_last_part() {
        assert_eq!(file_name("a/b/c.json"), "c.json");
        assert_eq!(file_name("c.json"), "c.json");
    }
}
