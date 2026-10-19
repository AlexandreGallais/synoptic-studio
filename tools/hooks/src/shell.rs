//! A shell command line read as the shell reads it, for the guard (AUD-023): split into simple
//! commands at `;`, `&`, `|`, a line feed and the parentheses of a subshell, each word unquoted
//! (`REF-POSIX-SHELL-QUOTING`): a backslash keeps the next character and drops a line feed after
//! it, single quotes keep every character, double quotes keep all but `$`, `` ` `` and the
//! backslash before one of `$`, `` ` ``, `"`, `\`. The commands of a substitution, `$(…)`,
//! `` `…` `` or `<(…)`, are read as simple commands of their own. Redirections are kept apart
//! from the words. The bodies of here-documents are left to the caller.

/// One simple command: its words unquoted, then its redirections.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Simple {
    /// The words, unquoted, assignments before the program included: `["GIT_DIR=.", "git", "push"]`.
    pub words: Vec<String>,
    /// Each redirection, its operator then its target unquoted: `>out.txt`, `2>&1`, `<<EOF`.
    pub redirections: Vec<String>,
    /// Whether a word held a substitution, whose commands are read as commands of their own.
    pub substituted: bool,
    /// The byte of the line it starts at; a backquoted command's is its backquote's (AUD-033).
    pub at: usize,
}

/// The simple commands of a command line, those of its substitutions and subshells included,
/// none empty.
#[must_use]
pub fn commands(text: &str) -> Vec<Simple> {
    let mut reader = Reader {
        chars: text.chars().collect(),
        bytes: text.char_indices().map(|(byte, _)| byte).collect(),
        length: text.len(),
        at: 0,
        out: Vec::new(),
    };

    reader.list(None);
    reader.out
}

/// The command line being read and the simple commands read so far.
struct Reader {
    chars: Vec<char>,
    /// The byte each character starts at.
    bytes: Vec<usize>,
    /// The line's length in bytes.
    length: usize,
    at: usize,
    out: Vec<Simple>,
}

/// The simple command being read: its words, the word being read, a redirection waiting for its
/// target.
#[derive(Default)]
struct Current {
    simple: Simple,
    word: Option<String>,
    redirection: Option<String>,
}

impl Current {
    /// The word read ends: it is a word, or the target of the redirection before it.
    fn end_word(&mut self) {
        if let Some(word) = self.word.take() {
            match self.redirection.take() {
                Some(operator) => self.simple.redirections.push(operator + &word),
                None => self.simple.words.push(word),
            }
        }
    }

    /// The text of the word being read, started if it was not.
    fn word(&mut self) -> &mut String {
        self.word.get_or_insert_with(String::new)
    }

    /// Whether nothing of the simple command is read yet.
    fn is_empty(&self) -> bool {
        self.word.is_none()
            && self.redirection.is_none()
            && self.simple.words.is_empty()
            && self.simple.redirections.is_empty()
    }
}

impl Reader {
    fn peek(&self, ahead: usize) -> Option<char> {
        self.chars.get(self.at + ahead).copied()
    }

    /// The byte the character being read starts at.
    fn byte(&self) -> usize {
        self.bytes.get(self.at).copied().unwrap_or(self.length)
    }

    /// The simple command read ends: kept unless empty.
    fn end_simple(&mut self, current: &mut Current) {
        current.end_word();
        let simple = std::mem::take(&mut current.simple);

        current.redirection = None;
        if !simple.words.is_empty() || !simple.redirections.is_empty() {
            self.out.push(simple);
        }
    }

    /// Reads simple commands up to `close` outside quotes, consumed, or the end.
    fn list(&mut self, close: Option<char>) {
        let mut current = Current::default();

        while let Some(character) = self.peek(0) {
            if Some(character) == close {
                self.at += 1;
                break;
            }
            if current.is_empty() && !matches!(character, ' ' | '\t' | '\n' | ';' | '|' | ')') {
                current.simple.at = self.byte();
            }
            match character {
                ' ' | '\t' => {
                    current.end_word();
                    self.at += 1;
                }
                '&' if self.peek(1) == Some('>') => self.redirection(&mut current),
                '\n' | ';' | '&' | '|' | ')' => {
                    self.end_simple(&mut current);
                    self.at += 1;
                }
                '(' if current.word.is_none() => {
                    self.end_simple(&mut current);
                    self.at += 1;
                    self.list(Some(')'));
                }
                '#' if current.word.is_none() => {
                    while self.peek(0).is_some_and(|next| next != '\n') {
                        self.at += 1;
                    }
                }
                '<' | '>' if self.peek(1) == Some('(') => {
                    self.at += 2;
                    self.list(Some(')'));
                    current.simple.substituted = true;
                    current.word();
                }
                '<' | '>' => self.redirection(&mut current),
                _ => self.word_part(&mut current),
            }
        }
        self.end_simple(&mut current);
    }

    /// A redirection operator, the digits of a word just before it its descriptor: a duplication
    /// (`2>&1`, `>&-`) is whole, any other waits for its target, the next word.
    fn redirection(&mut self, current: &mut Current) {
        let mut operator = match current.word.take() {
            Some(word) if word.bytes().all(|byte| byte.is_ascii_digit()) => word,
            word => {
                current.word = word;
                current.end_word();
                String::new()
            }
        };

        while let Some(character @ ('<' | '>' | '&' | '|' | '-')) = self.peek(0) {
            if character == '-' && !operator.ends_with("<<") && !operator.ends_with('&') {
                break;
            }
            operator.push(character);
            self.at += 1;
            if operator.ends_with("&-") {
                break;
            }
        }
        if operator.ends_with('&') {
            while let Some(digit) = self.peek(0).filter(char::is_ascii_digit) {
                operator.push(digit);
                self.at += 1;
            }
        }
        let duplicated = operator.contains('&')
            && (operator.ends_with('-') || operator.ends_with(|last: char| last.is_ascii_digit()));

        if duplicated {
            current.simple.redirections.push(operator);
        } else {
            current.redirection = Some(operator);
        }
    }

    /// One piece of a word outside quotes: an escaped character, a quoted text, a substitution or
    /// one character.
    fn word_part(&mut self, current: &mut Current) {
        match self.peek(0) {
            Some('\\') => {
                match self.peek(1) {
                    Some('\n') => {}
                    Some(next) => current.word().push(next),
                    None => current.word().push('\\'),
                }
                self.at += 2;
            }
            Some('\'') => {
                self.at += 1;
                let text = self.until('\'');

                current.word().push_str(&text);
            }
            Some('"') => self.double_quoted(current),
            Some('$') if self.peek(1) == Some('\'') => self.dollar_single_quoted(current),
            Some('$' | '`') => self.dollar_or_backquote(current),
            Some(character) => {
                current.word().push(character);
                self.at += 1;
            }
            None => {}
        }
    }

    /// The characters up to `close`, consumed, or to the end.
    fn until(&mut self, close: char) -> String {
        let mut text = String::new();

        while let Some(character) = self.peek(0) {
            self.at += 1;
            if character == close {
                break;
            }
            text.push(character);
        }
        text
    }

    /// A double-quoted text, from its opening quote.
    fn double_quoted(&mut self, current: &mut Current) {
        self.at += 1;
        current.word();
        while let Some(character) = self.peek(0) {
            match character {
                '"' => {
                    self.at += 1;
                    break;
                }
                '\\' => {
                    match self.peek(1) {
                        Some('\n') => {}
                        Some(next @ ('$' | '`' | '"' | '\\')) => current.word().push(next),
                        _ => {
                            current.word().push('\\');
                            self.at -= 1;
                        }
                    }
                    self.at += 2;
                }
                '$' | '`' => self.dollar_or_backquote(current),
                _ => {
                    current.word().push(character);
                    self.at += 1;
                }
            }
        }
    }

    /// A `$'…'` text: as single quotes, but a backslash escapes, `\xHH` and `\NNN` naming a byte.
    fn dollar_single_quoted(&mut self, current: &mut Current) {
        self.at += 2;
        while let Some(character) = self.peek(0) {
            self.at += 1;
            match character {
                '\'' => break,
                '\\' => {
                    let decoded = self.escape();

                    current.word().push(decoded);
                }
                _ => current.word().push(character),
            }
        }
        current.word();
    }

    /// The character an escape of `$'…'` names, after its backslash.
    fn escape(&mut self) -> char {
        let Some(letter) = self.peek(0) else {
            return '\\';
        };
        self.at += 1;
        let (radix, longest) = match letter {
            'x' => (16, 2),
            '0'..='7' => {
                self.at -= 1;
                (8, 3)
            }
            'n' => return '\n',
            't' => return '\t',
            other => return other,
        };
        let digits: String = (0..longest)
            .map_while(|_| {
                let digit = self.peek(0).filter(|next| next.is_digit(radix))?;

                self.at += 1;
                Some(digit)
            })
            .collect();

        u32::from_str_radix(&digits, radix)
            .ok()
            .and_then(char::from_u32)
            .unwrap_or(letter)
    }

    /// A `$` or a backquote: a command substitution read as commands, an arithmetic expansion or a
    /// parameter kept as text, or a lone `$`.
    fn dollar_or_backquote(&mut self, current: &mut Current) {
        if self.peek(0) == Some('`') {
            let at = self.byte();

            self.at += 1;
            let mut inner = String::new();

            while let Some(character) = self.peek(0) {
                self.at += 1;
                match character {
                    '`' => break,
                    '\\' if matches!(self.peek(0), Some('`' | '\\' | '$')) => {
                        inner.extend(self.peek(0));
                        self.at += 1;
                    }
                    _ => inner.push(character),
                }
            }
            self.out.extend(
                commands(&inner)
                    .into_iter()
                    .map(|simple| Simple { at, ..simple }),
            );
            current.simple.substituted = true;
            current.word();
            return;
        }
        match (self.peek(1), self.peek(2)) {
            (Some('('), Some('(')) => {
                let text = self.balanced('(', ')');

                current.word().push_str(&text);
            }
            (Some('('), _) => {
                self.at += 2;
                self.list(Some(')'));
                current.simple.substituted = true;
                current.word();
            }
            (Some('{'), _) => {
                let text = self.balanced('{', '}');

                current.word().push_str(&text);
            }
            _ => {
                current.word().push('$');
                self.at += 1;
            }
        }
    }

    /// The text from a `$` to the bracket closing the one after it, kept as it is.
    fn balanced(&mut self, open: char, close: char) -> String {
        let mut text = String::from('$');
        let mut depth = 0;

        self.at += 1;
        while let Some(character) = self.peek(0) {
            self.at += 1;
            text.push(character);
            if character == open {
                depth += 1;
            } else if character == close {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
        }
        text
    }
}
