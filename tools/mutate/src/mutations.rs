//! The mutations of a source file, found in its text (EN-058): no parser, since crates.io
//! dependencies wait for F19's decision. Comments, strings and template literals are skipped;
//! binary operators are matched with a space on each side, as Prettier writes them, so a generic
//! (`Array<number>`) or an arrow (`=>`) is never taken for a comparison. A Rust file is read with
//! its own quotes: `'a` is a lifetime, `'x'` a character, and a return is never replaced, since no
//! value fits every type (REV-009).

use std::fmt;
use std::path::Path;

/// The language a source is read in, from its file's extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    /// TypeScript or JavaScript: `'…'` is a string, a return becomes `undefined`.
    Script,
    /// Rust, a `.rs` file: `'` opens a character or names a lifetime; no return mutated.
    Rust,
}

impl Language {
    /// The language of a file: Rust for `.rs`, a script otherwise.
    #[must_use]
    pub fn of(file: &str) -> Self {
        if Path::new(file)
            .extension()
            .is_some_and(|extension| extension == "rs")
        {
            Self::Rust
        } else {
            Self::Script
        }
    }
}

/// What a mutation changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// A comparison flipped: `<` to `>=`, `===` to `!==`, and back.
    Comparison,
    /// An arithmetic operator swapped: `+` and `-`, `*` and `/`.
    Arithmetic,
    /// An integer constant changed: `n` to `n + 1`.
    Constant,
    /// The condition of an `if` negated.
    Condition,
    /// A returned value replaced by `undefined`.
    Return,
}

impl Kind {
    /// Every kind, in report order.
    pub const ALL: [Self; 5] = [
        Self::Comparison,
        Self::Arithmetic,
        Self::Constant,
        Self::Condition,
        Self::Return,
    ];
}

impl fmt::Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Comparison => "comparison flipped",
            Self::Arithmetic => "operator swapped",
            Self::Constant => "constant changed",
            Self::Condition => "condition negated",
            Self::Return => "return replaced",
        })
    }
}

/// One change of a source text: the bytes `start..end` replaced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mutation {
    /// What it changes.
    pub kind: Kind,
    /// First byte replaced.
    pub start: usize,
    /// Byte after the last one replaced.
    pub end: usize,
    /// The text put in their place.
    pub replacement: String,
}

impl Mutation {
    /// The source with this mutation applied.
    #[must_use]
    pub fn apply(&self, source: &str) -> String {
        format!(
            "{}{}{}",
            &source[..self.start],
            self.replacement,
            &source[self.end..]
        )
    }

    /// Line and column of its first byte, both from 1.
    #[must_use]
    pub fn position(&self, source: &str) -> (usize, usize) {
        let before = &source[..self.start];
        let line = before.matches('\n').count() + 1;
        let column = before
            .rfind('\n')
            .map_or(before.len(), |at| before.len() - at - 1)
            + 1;

        (line, column)
    }
}

/// Binary operators and what each becomes, the longer first; each is matched with a space on
/// each side.
const OPERATORS: [(&str, &str, Kind); 10] = [
    ("===", "!==", Kind::Comparison),
    ("!==", "===", Kind::Comparison),
    ("<=", ">", Kind::Comparison),
    (">=", "<", Kind::Comparison),
    ("<", ">=", Kind::Comparison),
    (">", "<=", Kind::Comparison),
    ("+", "-", Kind::Arithmetic),
    ("-", "+", Kind::Arithmetic),
    ("*", "/", Kind::Arithmetic),
    ("/", "*", Kind::Arithmetic),
];

/// For each byte of a source, whether it is code: not in a comment, a string, a template literal
/// or, in Rust, a character.
#[must_use]
pub fn code_mask(source: &str, language: Language) -> Vec<bool> {
    let bytes = source.as_bytes();
    let mut mask = vec![true; bytes.len()];
    let mut at = 0;

    while at < bytes.len() {
        let end = match (bytes[at], bytes.get(at + 1)) {
            (b'/', Some(b'/')) => find_from(bytes, at, b"\n").unwrap_or(bytes.len()),
            (b'/', Some(b'*')) => {
                find_from(bytes, at + 2, b"*/").map_or(bytes.len(), |end| end + 2)
            }
            (b'\'', _) if language == Language::Rust => {
                let Some(end) = rust_character_end(source, at) else {
                    // A lifetime, `'a`: its quote opens nothing.
                    at += 1;
                    continue;
                };
                end
            }
            (quote @ (b'"' | b'\'' | b'`'), _) => string_end(bytes, at, quote),
            _ => {
                at += 1;
                continue;
            }
        };

        mask[at..end].fill(false);
        at = end;
    }

    mask
}

/// Byte after the Rust character opened at `start`, `'x'` or an escape `'\n'`, `'\''`; `None` for
/// the quote of a lifetime, `'a`, which no quote closes right after its one character.
fn rust_character_end(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();

    if bytes.get(start + 1) == Some(&b'\\') {
        return Some(string_end(bytes, start, b'\''));
    }
    let after = start + 1 + source[start + 1..].chars().next()?.len_utf8();

    (bytes.get(after) == Some(&b'\'')).then_some(after + 1)
}

/// First index of `needle` in `bytes` from `from`.
fn find_from(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    bytes[from..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|index| from + index)
}

/// Byte after the string opened by `quote` at `start`; escaped quotes stay inside.
fn string_end(bytes: &[u8], start: usize, quote: u8) -> usize {
    let mut at = start + 1;

    while at < bytes.len() {
        match bytes[at] {
            b'\\' => at += 2,
            byte if byte == quote => return at + 1,
            _ => at += 1,
        }
    }

    bytes.len()
}

/// Whether a byte can be part of an identifier or a number.
fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

/// Whether the bytes `start..end` are all code.
fn is_code(mask: &[bool], start: usize, end: usize) -> bool {
    mask[start..end].iter().all(|code| *code)
}

/// Whether a keyword starts at `at`, as a whole word.
fn keyword_at(bytes: &[u8], at: usize, keyword: &[u8]) -> bool {
    bytes[at..].starts_with(keyword)
        && (at == 0 || !is_word(bytes[at - 1]))
        && bytes
            .get(at + keyword.len())
            .is_none_or(|next| !is_word(*next))
}

/// Every mutation of a source in its language, in text order.
#[must_use]
pub fn mutations(source: &str, language: Language) -> Vec<Mutation> {
    let bytes = source.as_bytes();
    let mask = code_mask(source, language);
    let mut found = Vec::new();

    for at in 0..bytes.len() {
        if !mask[at] {
            continue;
        }
        found.extend(operator_at(bytes, &mask, at));
        found.extend(constant_at(source, at));
        found.extend(condition_at(source, &mask, at));
        if language == Language::Script {
            found.extend(return_at(source, &mask, at));
        }
    }

    found
}

/// The operator written ` op ` at `at`, the space before it included.
fn operator_at(bytes: &[u8], mask: &[bool], at: usize) -> Option<Mutation> {
    if bytes[at] != b' ' {
        return None;
    }

    OPERATORS.iter().find_map(|(operator, replacement, kind)| {
        let start = at + 1;
        let end = start + operator.len();
        let spaced =
            bytes[start..].starts_with(operator.as_bytes()) && bytes.get(end) == Some(&b' ');

        (spaced && is_code(mask, at, end + 1)).then(|| Mutation {
            kind: *kind,
            start,
            end,
            replacement: (*replacement).to_owned(),
        })
    })
}

/// The decimal integer starting at `at`, not part of a word nor of a decimal number.
fn constant_at(source: &str, at: usize) -> Option<Mutation> {
    let bytes = source.as_bytes();
    let starts = bytes[at].is_ascii_digit()
        && (at == 0 || !(is_word(bytes[at - 1]) || bytes[at - 1] == b'.'));

    if !starts {
        return None;
    }

    let end = at
        + bytes[at..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
    let whole = bytes
        .get(end)
        .is_none_or(|next| !(is_word(*next) || *next == b'.'));
    let value: u64 = source[at..end].parse().ok()?;
    let replacement = value.checked_add(1)?.to_string();

    whole.then_some(Mutation {
        kind: Kind::Constant,
        start: at,
        end,
        replacement,
    })
}

/// Byte of the bracket closing the one at `open`, counting brackets in code only.
fn closing(bytes: &[u8], mask: &[bool], open: usize) -> Option<usize> {
    let mut depth = 0_usize;

    for at in open..bytes.len() {
        if !mask[at] {
            continue;
        }
        match bytes[at] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }

    None
}

/// The condition of `if (…)` at `at`, wrapped in `!(…)`.
fn condition_at(source: &str, mask: &[bool], at: usize) -> Option<Mutation> {
    let bytes = source.as_bytes();

    if !keyword_at(bytes, at, b"if") || !bytes[at..].starts_with(b"if (") {
        return None;
    }

    let open = at + 3;
    let close = closing(bytes, mask, open)?;

    Some(Mutation {
        kind: Kind::Condition,
        start: open + 1,
        end: close,
        replacement: format!("!({})", &source[open + 1..close]),
    })
}

/// The value of `return …;` at `at`, replaced by `undefined`.
fn return_at(source: &str, mask: &[bool], at: usize) -> Option<Mutation> {
    let bytes = source.as_bytes();

    if !keyword_at(bytes, at, b"return") || bytes.get(at + 6) != Some(&b' ') {
        return None;
    }

    let start = at + 7;
    let mut depth = 0_usize;

    for end in start..bytes.len() {
        if !mask[end] {
            continue;
        }
        match bytes[end] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' if depth == 0 => return None,
            b')' | b']' | b'}' => depth -= 1,
            b';' if depth == 0 => {
                return Some(Mutation {
                    kind: Kind::Return,
                    start,
                    end,
                    replacement: "undefined".to_owned(),
                });
            }
            _ => {}
        }
    }

    None
}

/// The mutations to run: every one when `all`, otherwise the first of each kind.
#[must_use]
pub fn selected(found: Vec<Mutation>, all: bool) -> Vec<Mutation> {
    if all {
        return found;
    }

    Kind::ALL
        .iter()
        .filter_map(|kind| {
            found
                .iter()
                .find(|mutation| mutation.kind == *kind)
                .cloned()
        })
        .collect()
}
