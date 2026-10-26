//! The tokens of a style sheet: a port of `scss-tokenize.js` of `postcss-scss` 4.0.9 (MIT, under
//! ADR-0076), the tokenizer of `postcss` 8.5.30 with its SCSS patches (`#{…}` interpolations, `//`
//! comments, a comma a word of its own, `url(…)` read to its matching parenthesis), so that a
//! sheet is cut where Stylelint's parser cuts it. Offsets are bytes; every character the tokenizer
//! tests is ASCII, so a multibyte character always falls inside a word.

/// What a token is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Spaces, tabs, line feeds, carriage returns and form feeds.
    Space,
    /// A word: a name, a number, a comma, an escape, an interpolation.
    Word,
    /// A quoted string, its quotes included.
    String,
    /// `@` and the name after it.
    AtWord,
    /// A parenthesised text read whole, `(…)` without quotes nor nested parentheses, or
    /// `url(…)`.
    Brackets,
    /// A comment, `/* … */`, or `// …` to the end of its line when `inline`.
    Comment {
        /// Whether it is a `//` comment.
        inline: bool,
    },
    /// `(` opening a text not read whole.
    Open,
    /// One of `[ ] { } : ; )`.
    Char(u8),
}

/// A token: its kind and where it is, as `PostCSS` gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    /// What it is.
    pub kind: Kind,
    /// Its first byte.
    pub start: usize,
    /// The byte after it, within the text.
    pub end: usize,
    /// Its last byte as `PostCSS`'s `token[3]` gives it: none for spaces, `(` and the
    /// one-character tokens; the byte after a comma, and past the text for a `url(` or an
    /// escape left open at its end.
    pub last: Option<usize>,
}

impl Token {
    /// Its text.
    #[must_use]
    pub fn text<'t>(&self, css: &'t str) -> &'t str {
        css.get(self.start..self.end).unwrap_or_default()
    }

    /// Its first byte, as `PostCSS`'s `token[2]` gives it: none for spaces.
    #[must_use]
    pub const fn position(&self) -> Option<usize> {
        match self.kind {
            Kind::Space => None,
            _ => Some(self.start),
        }
    }

    /// Its last byte, as `PostCSS`'s `token[3]` gives it.
    #[must_use]
    pub const fn last(&self) -> Option<usize> {
        self.last
    }

    /// Its last byte, else its first, as `token[3] || token[2]` reads it.
    #[must_use]
    pub fn last_or_position(&self) -> Option<usize> {
        js_or(self.last(), self.position())
    }
}

/// `a || b` on offsets as JavaScript reads it: `0` is false.
#[must_use]
pub fn js_or(first: Option<usize>, second: Option<usize>) -> Option<usize> {
    match first {
        Some(value) if value != 0 => Some(value),
        _ => second.filter(|value| *value != 0),
    }
}

/// An error of the tokenizer: what is left unclosed, at the byte where it opens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unclosed {
    /// `string`, `comment` or `interpolation`.
    pub what: &'static str,
    /// The byte the error is reported at.
    pub offset: usize,
}

/// The tokenizer over a sheet.
pub struct Tokenizer<'t> {
    /// The sheet's text, as bytes.
    css: &'t [u8],
    /// The sheet's text.
    source: &'t str,
    /// The next byte to read.
    pos: usize,
    /// The words read, the last of which `(` looks back at.
    buffer: Vec<Token>,
    /// The tokens given back, read again first.
    returned: Vec<Token>,
}

/// Whether a byte ends a word: `RE_WORD_END` but its `/` before `*`.
const fn ends_word(byte: u8) -> bool {
    matches!(
        byte,
        b',' | b'\t'
            | b'\n'
            | 0x0c
            | b'\r'
            | b' '
            | b'!'
            | b'"'
            | b'#'
            | b'\''
            | b'('
            | b')'
            | b':'
            | b';'
            | b'@'
            | b'['
            | b'\\'
            | b']'
            | b'{'
            | b'}'
    )
}

/// Whether a byte ends an at-word name (`RE_AT_END`).
const fn ends_at_word(byte: u8) -> bool {
    matches!(
        byte,
        b'\t'
            | b'\n'
            | 0x0c
            | b'\r'
            | b' '
            | b'"'
            | b'#'
            | b'\''
            | b'('
            | b')'
            | b'/'
            | b';'
            | b'['
            | b'\\'
            | b']'
            | b'{'
            | b'}'
    )
}

/// Whether a byte is a space of CSS.
const fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\n' | b'\t' | b'\r' | 0x0c)
}

impl<'t> Tokenizer<'t> {
    /// The tokenizer of a sheet, at its start.
    #[must_use]
    pub fn new(css: &'t str) -> Self {
        Self {
            css: css.as_bytes(),
            source: css,
            pos: 0,
            buffer: Vec::new(),
            returned: Vec::new(),
        }
    }

    /// The byte at an offset, `0` past the end as `charCodeAt` gives `NaN`.
    fn at(&self, offset: usize) -> u8 {
        self.css.get(offset).copied().unwrap_or(0)
    }

    /// The next byte to read.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.pos
    }

    /// Whether every token was read.
    #[must_use]
    pub fn end_of_file(&self) -> bool {
        self.returned.is_empty() && self.pos >= self.css.len()
    }

    /// Gives a token back, to be read again next.
    pub fn back(&mut self, token: Token) {
        self.returned.push(token);
    }

    /// Reads an interpolation from its `#` at `next` to its `}`; the byte of the `}`.
    fn interpolation(&self, mut next: usize) -> Result<usize, Unclosed> {
        let mut deep = 1;
        let mut quote = 0;
        let mut escaped = false;
        while deep > 0 {
            next += 1;
            if self.css.len() <= next {
                return Err(Unclosed {
                    what: "interpolation",
                    offset: self.pos,
                });
            }
            let code = self.at(next);
            let following = self.at(next + 1);
            if quote != 0 {
                if !escaped && code == quote {
                    quote = 0;
                    escaped = false;
                } else if code == b'\\' {
                    escaped = !escaped;
                } else if escaped {
                    escaped = false;
                }
            } else if code == b'\'' || code == b'"' {
                quote = code;
            } else if code == b'}' {
                deep -= 1;
            } else if code == b'#' && following == b'{' {
                deep += 1;
            }
        }
        Ok(next)
    }

    /// The next token, none at the end.
    ///
    /// # Errors
    ///
    /// A string, a comment or an interpolation left unclosed.
    pub fn next_token(&mut self) -> Result<Option<Token>, Unclosed> {
        if let Some(token) = self.returned.pop() {
            return Ok(Some(token));
        }
        if self.pos >= self.css.len() {
            return Ok(None);
        }
        let pos = self.pos;
        let code = self.at(pos);
        let (kind, last) = match code {
            b'\n' | b' ' | b'\t' | b'\r' | 0x0c => {
                let mut next = pos + 1;
                while is_space(self.at(next)) {
                    next += 1;
                }
                (Kind::Space, next - 1)
            }
            b'[' | b']' | b'{' | b'}' | b':' | b';' | b')' => (Kind::Char(code), pos),
            b',' => (Kind::Word, pos),
            b'(' => self.parenthesis(pos),
            b'\'' | b'"' => (Kind::String, self.string(pos, code)?),
            b'@' => {
                let mut next = pos + 1;
                while next < self.css.len() && !ends_at_word(self.at(next)) {
                    next += 1;
                }
                // `RE_AT_END` finding nothing reads to the last byte; finding the byte after
                // `@` gives `@` alone, as its `lastIndex - 2`.
                (Kind::AtWord, (next - 1).max(pos))
            }
            b'\\' => (Kind::Word, self.escape(pos)),
            _ => self.other(pos, code)?,
        };
        let token = Token {
            kind,
            start: pos,
            end: (last + 1).min(self.css.len()),
            last: match kind {
                Kind::Space | Kind::Open | Kind::Char(_) => None,
                // A comma is `['word', ',', pos, pos + 1]`.
                Kind::Word if code == b',' => Some(pos + 1),
                _ => Some(last),
            },
        };
        // Only a plain word is kept as the word a `(` looks back at, never a comma, an escape
        // nor an interpolation.
        let interpolation = code == b'#' && self.at(pos + 1) == b'{';
        if kind == Kind::Word && code != b',' && code != b'\\' && !interpolation {
            self.buffer.push(token);
        }
        self.pos = last + 1;
        Ok(Some(token))
    }

    /// `(`: `url(…)` read to its matching parenthesis, a text without quotes nor nested
    /// parentheses read whole, otherwise `(` alone.
    fn parenthesis(&mut self, pos: usize) -> (Kind, usize) {
        let previous = self
            .buffer
            .pop()
            .map(|token| token.text(self.text()).to_owned())
            .unwrap_or_default();
        let following = self.at(pos + 1);
        if previous == "url" && following != b'\'' && following != b'"' {
            // `PostCSS` follows the escapes here but reads nothing from them.
            let mut brackets = 1;
            let mut next = pos + 1;
            while next < self.css.len() {
                match self.at(next) {
                    b'(' => brackets += 1,
                    b')' => {
                        brackets -= 1;
                        if brackets == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                next += 1;
            }
            // Unclosed, it reads to the end, its last byte past it as `PostCSS` gives it.
            return (Kind::Brackets, next);
        }
        // `indexOf(')')` then `RE_BAD_BRACKET` on the text up to it: the first of `)` and the
        // bytes it refuses decides, read once, so that nested parentheses cost no quadratic time.
        let first = self.css[pos + 1..]
            .iter()
            .position(|byte| {
                matches!(
                    byte,
                    b')' | b'\r' | b'\n' | b'"' | b'\'' | b'(' | b'/' | b'\\'
                )
            })
            .map(|at| pos + 1 + at);
        match first {
            Some(close) if self.css[close] == b')' => (Kind::Brackets, close),
            _ => (Kind::Open, pos),
        }
    }

    /// A string from its quote; the byte of its closing quote.
    fn string(&self, pos: usize, quote: u8) -> Result<usize, Unclosed> {
        let mut next = pos;
        let mut escaped = false;
        while next < self.css.len() {
            next += 1;
            if next == self.css.len() {
                return Err(Unclosed {
                    what: "string",
                    offset: pos,
                });
            }
            let code = self.at(next);
            let following = self.at(next + 1);
            if !escaped && code == quote {
                break;
            } else if code == b'\\' {
                escaped = !escaped;
            } else if escaped {
                escaped = false;
            } else if code == b'#' && following == b'{' {
                next = self.interpolation(next)?;
            }
        }
        Ok(next)
    }

    /// An escape from its backslash; its last byte.
    fn escape(&self, pos: usize) -> usize {
        let mut next = pos;
        let mut escape = true;
        while self.at(next + 1) == b'\\' {
            next += 1;
            escape = !escape;
        }
        let code = self.at(next + 1);
        if escape && code != b'/' && !is_space(code) {
            next += 1;
            if self.at(next).is_ascii_hexdigit() {
                while self.at(next + 1).is_ascii_hexdigit() {
                    next += 1;
                }
                if self.at(next + 1) == b' ' {
                    next += 1;
                }
            }
        }
        next
    }

    /// An interpolation, a comment or a word; its kind and its last byte.
    fn other(&self, pos: usize, code: u8) -> Result<(Kind, usize), Unclosed> {
        let following = self.at(pos + 1);
        if code == b'#' && following == b'{' {
            return Ok((Kind::Word, self.interpolation(pos)?));
        }
        if code == b'/' && following == b'*' {
            let close = self.css[pos + 2..]
                .windows(2)
                .position(|pair| pair == b"*/")
                .map(|at| pos + 2 + at + 1);
            return close.map_or(
                Err(Unclosed {
                    what: "comment",
                    offset: pos,
                }),
                |last| Ok((Kind::Comment { inline: false }, last)),
            );
        }
        if code == b'/' && following == b'/' {
            let mut next = pos + 1;
            while next < self.css.len() && !matches!(self.at(next), b'\n' | 0x0c | b'\r') {
                next += 1;
            }
            return Ok((Kind::Comment { inline: true }, next - 1));
        }
        let mut next = pos + 1;
        while next < self.css.len() {
            let byte = self.at(next);
            if ends_word(byte) || (byte == b'/' && self.at(next + 1) == b'*') {
                break;
            }
            next += 1;
        }
        Ok((Kind::Word, next - 1))
    }

    /// The sheet's text.
    const fn text(&self) -> &'t str {
        self.source
    }
}

#[cfg(test)]
mod tests {
    use super::{Kind, Tokenizer, Unclosed, js_or};

    /// Each token's kind and text, or the error.
    fn tokens(css: &str) -> Result<Vec<(Kind, &str)>, Unclosed> {
        let mut tokenizer = Tokenizer::new(css);
        let mut found = Vec::new();
        while let Some(token) = tokenizer.next_token()? {
            found.push((token.kind, token.text(css)));
        }
        Ok(found)
    }

    #[test]
    fn cuts_a_rule_into_words_spaces_and_characters() {
        assert_eq!(
            tokens("a{b:c;}").ok(),
            Some(vec![
                (Kind::Word, "a"),
                (Kind::Char(b'{'), "{"),
                (Kind::Word, "b"),
                (Kind::Char(b':'), ":"),
                (Kind::Word, "c"),
                (Kind::Char(b';'), ";"),
                (Kind::Char(b'}'), "}"),
            ])
        );
        assert_eq!(tokens(" \n\t").ok(), Some(vec![(Kind::Space, " \n\t")]));
    }

    #[test]
    fn reads_scss_interpolations_inline_comments_and_commas() {
        assert_eq!(
            tokens("#{$a + \"}\"}px, // x\n/* y */").ok(),
            Some(vec![
                (Kind::Word, "#{$a + \"}\"}"),
                (Kind::Word, "px"),
                (Kind::Word, ","),
                (Kind::Space, " "),
                (Kind::Comment { inline: true }, "// x"),
                (Kind::Space, "\n"),
                (Kind::Comment { inline: false }, "/* y */"),
            ])
        );
        // A comment at the end of the text, without its line feed.
        assert_eq!(
            tokens("//").ok(),
            Some(vec![(Kind::Comment { inline: true }, "//")])
        );
    }

    #[test]
    fn reads_brackets_whole_unless_they_hold_a_quote_or_a_line() {
        assert_eq!(tokens("(a b)").ok(), Some(vec![(Kind::Brackets, "(a b)")]));
        assert_eq!(
            tokens("f(\"a\")").ok(),
            Some(vec![
                (Kind::Word, "f"),
                (Kind::Open, "("),
                (Kind::String, "\"a\""),
                (Kind::Char(b')'), ")"),
            ])
        );
        assert_eq!(tokens("(").ok(), Some(vec![(Kind::Open, "(")]));
        // `url(…)` unquoted is read to its matching parenthesis, a nested one and `;` included.
        assert_eq!(
            tokens("url(a(b);c) x").ok(),
            Some(vec![
                (Kind::Word, "url"),
                (Kind::Brackets, "(a(b);c)"),
                (Kind::Space, " "),
                (Kind::Word, "x"),
            ])
        );
        assert_eq!(
            tokens("url(a").ok(),
            Some(vec![(Kind::Word, "url"), (Kind::Brackets, "(a")])
        );
    }

    #[test]
    fn reads_at_words_strings_and_escapes() {
        assert_eq!(
            tokens("@use 'a\\'b';").ok(),
            Some(vec![
                (Kind::AtWord, "@use"),
                (Kind::Space, " "),
                (Kind::String, "'a\\'b'"),
                (Kind::Char(b';'), ";"),
            ])
        );
        assert_eq!(tokens("@").ok(), Some(vec![(Kind::AtWord, "@")]));
        assert_eq!(tokens("@{").ok().map(|all| all.len()), Some(2));
        // `\31 a`: a hexadecimal escape takes one space after it; `\\` escapes nothing.
        assert_eq!(
            tokens("\\31 a\\\\ \\").ok(),
            Some(vec![
                (Kind::Word, "\\31 "),
                (Kind::Word, "a"),
                (Kind::Word, "\\\\"),
                (Kind::Space, " "),
                (Kind::Word, "\\"),
            ])
        );
        assert_eq!(tokens("\"#{'\\''}\"").ok().map(|all| all.len()), Some(1));
    }

    #[test]
    fn reports_what_is_left_unclosed_where_it_opens() {
        assert_eq!(
            tokens("a 'b"),
            Err(Unclosed {
                what: "string",
                offset: 2
            })
        );
        assert_eq!(
            tokens("/* a"),
            Err(Unclosed {
                what: "comment",
                offset: 0
            })
        );
        assert_eq!(
            tokens("a #{b"),
            Err(Unclosed {
                what: "interpolation",
                offset: 2
            })
        );
    }

    #[test]
    fn gives_back_a_token_and_reads_offsets_as_javascript() {
        let mut tokenizer = Tokenizer::new("a b");
        let first = tokenizer.next_token().ok().flatten();
        assert!(first.is_some());
        if let Some(token) = first {
            tokenizer.back(token);
        }
        assert_eq!(tokenizer.next_token().ok().flatten(), first);
        assert!(!tokenizer.end_of_file());
        assert_eq!(tokenizer.position(), 1);
        let space = tokenizer.next_token().ok().flatten();
        assert_eq!(
            space.map(|token| (token.position(), token.last())),
            Some((None, None))
        );
        assert_eq!(js_or(Some(0), Some(3)), Some(3));
        assert_eq!(js_or(Some(0), Some(0)), None);
        assert_eq!(js_or(Some(2), None), Some(2));
        assert_eq!(js_or(None, Some(0)), None);
    }

    /// Each token as `PostCSS` prints it: its kind, text, `token[2]` and `token[3]`.
    fn printed(css: &str) -> Vec<(Kind, &str, Option<usize>, Option<usize>)> {
        let mut tokenizer = Tokenizer::new(css);
        let mut found = Vec::new();
        while let Ok(Some(token)) = tokenizer.next_token() {
            found.push((token.kind, token.text(css), token.position(), token.last()));
        }
        found
    }

    /// The cases of `test/tokenize.test.js` of `postcss-scss` 4.0.9, each by its title: 10.
    #[test]
    fn passes_the_tokenizer_tests_of_postcss_scss() {
        let comment = Kind::Comment { inline: true };
        // tokenizes inline comments; … with any new line; … in end of file.
        assert_eq!(
            printed("// a\n"),
            [
                (comment, "// a", Some(0), Some(3)),
                (Kind::Space, "\n", None, None)
            ]
        );
        assert_eq!(
            printed("// a\r\n"),
            [
                (comment, "// a", Some(0), Some(3)),
                (Kind::Space, "\r\n", None, None)
            ]
        );
        assert_eq!(printed("// a"), [(comment, "// a", Some(0), Some(3))]);
        // tokenizes interpolation; with escaped brace, quote, backslash; recursively.
        for (css, last) in [
            ("#{a\nb}", 5),
            ("#{\"\\}\"}", 6),
            ("#{\"\\\"\"}", 6),
            ("#{\"\\\\\"}", 6),
            ("#{#{#{}}}", 8),
        ] {
            assert_eq!(printed(css), [(Kind::Word, css, Some(0), Some(last))]);
        }
        // tokenizes multiline url().
        assert_eq!(
            printed("url(a\nb)"),
            [
                (Kind::Word, "url", Some(0), Some(2)),
                (Kind::Brackets, "(a\nb)", Some(3), Some(7)),
            ]
        );
        // tokenizes url() following a comma.
        assert_eq!(
            printed("background:center/auto,url"),
            [
                (Kind::Word, "background", Some(0), Some(9)),
                (Kind::Char(b':'), ":", Some(10), None),
                (Kind::Word, "center/auto", Some(11), Some(21)),
                (Kind::Word, ",", Some(22), Some(23)),
                (Kind::Word, "url", Some(23), Some(25)),
            ]
        );
    }
}
