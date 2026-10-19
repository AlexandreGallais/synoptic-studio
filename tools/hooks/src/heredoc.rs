//! The here-documents of a command line, read as the shell reads them (`REF-POSIX-SHELL-QUOTING`,
//! AUD-033): a `<<` or `<<-` opens one only outside quotes, comments, backquotes and arithmetic
//! expansions; its word is the shell's, quotes removed; its body begins after the line feed that
//! ends its logical line, the one outside quotes, and runs to the first line holding only that
//! word, leading blanks allowed. A document opened inside a `$(…)` or `<(…)` must close before
//! that substitution does, and one whose line feed comes at another depth, or whose word is never
//! met, opens none: its lines stay commands, which judges more, never less. The bodies are data
//! written to a command's input, so the guard leaves them out of the commands it judges; the
//! substitutions of a body whose word is unquoted, which the shell expands, are judged apart.

/// One here-document: its body's byte range, and whether its word was quoted.
struct Document {
    body: (usize, usize),
    quoted: bool,
}

/// The here-documents opened on one logical line: the line feed ending that line, the documents in
/// order, and the end of the last one's word on its closing line.
struct Group {
    opening_end: usize,
    documents: Vec<Document>,
    end: usize,
}

/// A here-document waiting for the end of its logical line: its word, whether it was quoted, and
/// how many substitutions it was opened in.
struct Pending {
    word: String,
    quoted: bool,
    depth: usize,
}

/// What an open bracket or quote of the line is.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Context {
    /// A double-quoted text.
    Double,
    /// A command substitution, `$(…)`, or a process substitution, `<(…)`, `>(…)`.
    Substitution,
    /// A subshell, `(…)`.
    Subshell,
}

/// The line read, the documents found and those waiting for the end of their line.
struct Scanner<'a> {
    text: &'a str,
    bytes: &'a [u8],
    at: usize,
    stack: Vec<Context>,
    pending: Vec<Pending>,
    groups: Vec<Group>,
}

/// The here-documents of a command line, by logical line.
fn groups(text: &str) -> Vec<Group> {
    let mut scanner = Scanner {
        text,
        bytes: text.as_bytes(),
        at: 0,
        stack: Vec::new(),
        pending: Vec::new(),
        groups: Vec::new(),
    };

    while scanner.at < scanner.bytes.len() {
        if scanner.stack.last() == Some(&Context::Double) {
            scanner.in_double_quotes();
        } else {
            scanner.in_code();
        }
    }
    scanner.groups
}

/// Whether the byte before `at` ends a word, so that `#` or `((` there opens a comment or an
/// arithmetic command.
fn starts_word(bytes: &[u8], at: usize) -> bool {
    at == 0
        || matches!(
            bytes[at - 1],
            b' ' | b'\t' | b'\n' | b';' | b'&' | b'|' | b'(' | b')'
        )
}

/// The byte after the bracket closing the one at `open`, brackets of its kind counted outside
/// quotes and escapes, or the end: a quote met inside is skipped whole, so that the scan resumes
/// in the state the shell resumes in.
fn after_balanced(bytes: &[u8], open: usize) -> usize {
    let (opening, closing) = match bytes[open] {
        b'{' => (b'{', b'}'),
        b'[' => (b'[', b']'),
        _ => (b'(', b')'),
    };
    let mut depth = 0_usize;
    let mut at = open;

    while let Some(byte) = bytes.get(at) {
        match *byte {
            b'\\' => at += 2,
            b'\'' => at = after_quote(bytes, at + 1, b'\'', false),
            b'"' => at = after_quote(bytes, at + 1, b'"', true),
            b'`' => at = after_quote(bytes, at + 1, b'`', true),
            byte if byte == opening => {
                depth += 1;
                at += 1;
            }
            byte if byte == closing => {
                depth -= 1;
                at += 1;
                if depth == 0 {
                    return at;
                }
            }
            _ => at += 1,
        }
    }
    bytes.len()
}

/// The byte after the `]` closing a bracket expression inside a word, `a[1<<2]`, when it closes on
/// the same line with no quote, escape nor expansion before it; else the byte after the `[`.
fn after_bracket(bytes: &[u8], open: usize) -> usize {
    bytes[open + 1..]
        .iter()
        .position(|byte| matches!(byte, b']' | b'\n' | b'\'' | b'"' | b'`' | b'\\' | b'$'))
        .filter(|length| bytes[open + 1 + length] == b']')
        .map_or(open + 1, |length| open + length + 2)
}

/// The byte after the quote closing a quoted text opened before `from`: `'` for a single-quoted
/// text, `` ` `` for a backquoted command, whose backslash escapes, `'` for `$'…'`, whose
/// backslash escapes too.
fn after_quote(bytes: &[u8], from: usize, close: u8, escapes: bool) -> usize {
    let mut at = from;

    while let Some(byte) = bytes.get(at) {
        if escapes && *byte == b'\\' {
            at += 2;
        } else if *byte == close {
            return at + 1;
        } else {
            at += 1;
        }
    }
    bytes.len()
}

impl Scanner<'_> {
    /// How many substitutions the byte read is inside.
    fn depth(&self) -> usize {
        self.stack
            .iter()
            .filter(|context| **context == Context::Substitution)
            .count()
    }

    /// One step inside double quotes: an escape, the closing quote, a backquoted command, a `$`.
    fn in_double_quotes(&mut self) {
        match self.bytes[self.at] {
            b'\\' => self.at += 2,
            b'"' => {
                self.stack.pop();
                self.at += 1;
            }
            b'`' => self.at = after_quote(self.bytes, self.at + 1, b'`', true),
            b'$' => self.dollar(),
            _ => self.at += 1,
        }
    }

    /// A `$`: an arithmetic expansion, a parameter or `$[…]` skipped whole, a substitution opened.
    fn dollar(&mut self) {
        match (self.bytes.get(self.at + 1), self.bytes.get(self.at + 2)) {
            (Some(b'('), Some(b'(')) | (Some(b'{' | b'['), _) => {
                self.at = after_balanced(self.bytes, self.at + 1);
            }
            (Some(b'('), _) => {
                self.stack.push(Context::Substitution);
                self.at += 2;
            }
            _ => self.at += 1,
        }
    }

    /// One step outside quotes.
    fn in_code(&mut self) {
        let bytes = self.bytes;
        let at = self.at;
        let next = bytes.get(at + 1).copied();

        match bytes[at] {
            b'\\' => self.at += 2,
            b'\'' => self.at = after_quote(bytes, at + 1, b'\'', false),
            b'"' => {
                self.stack.push(Context::Double);
                self.at += 1;
            }
            b'`' => self.at = after_quote(bytes, at + 1, b'`', true),
            b'$' if next == Some(b'\'') => self.at = after_quote(bytes, at + 2, b'\'', true),
            b'$' => self.dollar(),
            b'#' if starts_word(bytes, at) => {
                self.at = self.text[at..]
                    .find('\n')
                    .map_or(bytes.len(), |end| at + end);
            }
            b'(' if next == Some(b'(') && starts_word(bytes, at) => {
                self.at = after_balanced(bytes, at);
            }
            b'[' if !starts_word(bytes, at) => self.at = after_bracket(bytes, at),
            b'<' | b'>' if next == Some(b'(') => {
                self.stack.push(Context::Substitution);
                self.at += 2;
            }
            b'(' => {
                self.stack.push(Context::Subshell);
                self.at += 1;
            }
            b')' => {
                if self.stack.pop() == Some(Context::Substitution) {
                    let depth = self.depth();

                    self.pending.retain(|pending| pending.depth <= depth);
                }
                self.at += 1;
            }
            b'<' if self.text[at..].starts_with("<<<") => self.at += 3,
            b'<' if next == Some(b'<') => self.opening(),
            b'\n' if !self.pending.is_empty() => self.line_end(),
            _ => self.at += 1,
        }
    }

    /// A `<<` or `<<-` and its word: a document waiting for its line's end, or nothing when its
    /// word is missing or holds an expansion the guard does not read.
    fn opening(&mut self) {
        let mut at = self.at + 2;

        if self.bytes.get(at) == Some(&b'-') {
            at += 1;
        }
        while matches!(self.bytes.get(at), Some(b' ' | b'\t')) {
            at += 1;
        }
        match delimiter(self.text, at) {
            Some((word, quoted, end)) => {
                let depth = self.depth();

                self.pending.push(Pending {
                    word,
                    quoted,
                    depth,
                });
                self.at = end;
            }
            None => self.at = at,
        }
    }

    /// The line feed ending a logical line with documents waiting: those opened at this depth read
    /// their bodies, in order, the others dropped.
    fn line_end(&mut self) {
        let depth = self.depth();
        let pending: Vec<Pending> = std::mem::take(&mut self.pending)
            .into_iter()
            .filter(|pending| pending.depth == depth)
            .collect();

        match bodies(self.text, self.at, &pending) {
            Some(group) => {
                self.at = group.end;
                self.groups.push(group);
            }
            None => self.at += 1,
        }
    }
}

/// A here-document's word from `at`, as the shell reads it: up to a blank or an operator outside
/// quotes, its quotes removed; whether a part of it was quoted; and the byte after it. `None` for
/// no word, or one holding a `$` or a backquote, whose delimiter the guard does not compute.
fn delimiter(text: &str, at: usize) -> Option<(String, bool, usize)> {
    let mut word = String::new();
    let mut quoted = false;
    let mut characters = text[at..].char_indices().peekable();
    let mut end = text.len();

    while let Some((index, character)) = characters.next() {
        match character {
            ' ' | '\t' | '\n' | ';' | '&' | '|' | '<' | '>' | '(' | ')' => {
                end = at + index;
                break;
            }
            '$' | '`' => return None,
            '\\' => {
                quoted = true;
                match characters.next() {
                    Some((_, '\n')) | None => {}
                    Some((_, next)) => word.push(next),
                }
            }
            '\'' => {
                quoted = true;
                word.extend(
                    characters
                        .by_ref()
                        .map(|(_, inner)| inner)
                        .take_while(|inner| *inner != '\''),
                );
            }
            '"' => {
                quoted = true;
                while let Some((_, inner)) = characters.next() {
                    match inner {
                        '"' => break,
                        '$' | '`' => return None,
                        '\\' => match characters.next() {
                            Some((_, next @ ('$' | '`' | '"' | '\\'))) => word.push(next),
                            Some((_, '\n')) => {}
                            Some((_, next)) => {
                                word.push('\\');
                                word.push(next);
                            }
                            None => word.push('\\'),
                        },
                        _ => word.push(inner),
                    }
                }
            }
            _ => word.push(character),
        }
    }
    (quoted || !word.is_empty()).then_some((word, quoted, end))
}

/// The bodies of the documents waiting at the line feed `line_feed`: each from the line after the
/// previous one's closing line to the first line holding only its word, leading blanks allowed.
/// `None` when a word is never met.
fn bodies(text: &str, line_feed: usize, pending: &[Pending]) -> Option<Group> {
    let mut start = line_feed + 1;
    let mut end = line_feed;
    let mut documents = Vec::new();

    for document in pending {
        let mut line = start;

        loop {
            if line > text.len() {
                return None;
            }
            let line_end = text[line..].find('\n').map_or(text.len(), |at| line + at);

            if text[line..line_end].trim_start_matches([' ', '\t']) == document.word {
                documents.push(Document {
                    body: (start, line.saturating_sub(1).max(start)),
                    quoted: document.quoted,
                });
                end = line_end;
                start = line_end + 1;
                break;
            }
            line = line_end + 1;
        }
    }
    Some(Group {
        opening_end: line_feed,
        documents,
        end,
    })
}

/// The shell command without the bodies of its here-documents: each body and its closing line go,
/// the line opening it kept.
#[must_use]
pub fn without_heredocs(command: &str) -> String {
    let mut kept = String::with_capacity(command.len());
    let mut at = 0;

    for group in groups(command) {
        kept.push_str(&command[at..group.opening_end]);
        at = group.end;
    }
    kept.push_str(&command[at..]);
    kept
}

/// The bodies of a command's here-documents, in order.
#[must_use]
pub fn heredoc_bodies(command: &str) -> Vec<&str> {
    groups(command)
        .into_iter()
        .flat_map(|group| group.documents)
        .map(|document| &command[document.body.0..document.body.1])
        .collect()
}

/// The commands the substitutions of an unquoted here-document's body run, `$(…)` and
/// backquotes, which the shell expands before it writes the body (`REF-POSIX-SHELL-QUOTING`).
#[must_use]
pub fn expanded_in_bodies(command: &str) -> Vec<String> {
    let mut found = Vec::new();

    for document in groups(command)
        .into_iter()
        .flat_map(|group| group.documents)
        .filter(|document| !document.quoted)
    {
        let body = &command.as_bytes()[document.body.0..document.body.1];
        let mut at = 0;

        while let Some(byte) = body.get(at) {
            match (byte, body.get(at + 1), body.get(at + 2)) {
                (b'\\', _, _) => at += 2,
                (b'$', Some(b'('), Some(b'(')) => at = after_balanced(body, at + 1),
                (b'$', Some(b'('), _) => {
                    let end = after_balanced(body, at + 1);

                    found.push(
                        String::from_utf8_lossy(&body[at + 2..end.saturating_sub(1).max(at + 2)])
                            .into_owned(),
                    );
                    at = end;
                }
                (b'`', _, _) => {
                    let end = after_quote(body, at + 1, b'`', true);

                    found.push(
                        String::from_utf8_lossy(&body[at + 1..end.saturating_sub(1).max(at + 1)])
                            .into_owned(),
                    );
                    at = end;
                }
                _ => at += 1,
            }
        }
    }
    found
}
