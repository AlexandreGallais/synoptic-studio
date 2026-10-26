//! The line and the column of a byte of a file's text, as `ESLint` counts them: from 1, the
//! column in UTF-16 units. Shared by the TypeScript files, the Markdown pages and the templates.

/// The byte where each line of a text starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lines {
    /// Each line's first byte, the first line's 0.
    starts: Vec<usize>,
}

impl Lines {
    /// The lines of a text.
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self {
            starts: std::iter::once(0)
                .chain(text.match_indices('\n').map(|(at, _)| at + 1))
                .collect(),
        }
    }

    /// The line of a byte, from 1.
    #[must_use]
    pub fn line(&self, offset: usize) -> usize {
        self.starts.partition_point(|start| *start <= offset)
    }

    /// The line and the column of a byte of the text the lines were read from, from 1.
    #[must_use]
    pub fn place(&self, text: &str, offset: usize) -> (usize, usize) {
        let line = self.line(offset);
        let start = self.starts[line - 1];
        let end = offset.min(text.len());
        let column = text
            .get(start..end)
            .map_or(0, |before| before.encode_utf16().count());

        (line, column + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Lines;

    #[test]
    fn places_a_byte_by_line_and_utf16_column() {
        let text = "ab\né𝄞x\n";
        let lines = Lines::new(text);

        assert_eq!(lines.place(text, 0), (1, 1));
        assert_eq!(lines.place(text, 3), (2, 1));
        // `é` is 2 bytes and 1 unit, `𝄞` 4 bytes and 2 units: `x` at byte 3 + 6 = 9, column 4.
        assert_eq!(lines.place(text, 9), (2, 4));
        assert_eq!(lines.place(text, 99), (3, 1));
    }
}
