//! A value read into its parts: a port of `lib/parse.js` and `lib/unit.js` of
//! `postcss-value-parser` 4.2.0 (MIT, under ADR-0076), which Stylelint's rules read a declaration's
//! value and an at-rule's parameters with: words, strings, separators (`div`: `,` `/` `:`), spaces,
//! comments, unicode ranges and functions with their arguments. The parts live in one list, each
//! function knowing its arguments by index, so that a walk needs no recursion; a part's text is
//! its slice of the value, offsets in bytes.

/// What a part is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A word: a keyword, a number, a dimension, a `$variable`, an operator inside `calc()`.
    Word,
    /// A quoted string.
    String,
    /// A separator, `,` `/` or `:`, with the spaces around it.
    Div,
    /// Spaces between parts.
    Space,
    /// A comment, `/* … */`.
    Comment,
    /// A function, or parentheses without a name, and its arguments.
    Function,
    /// A unicode range, `U+0025-00FF`.
    UnicodeRange,
}

/// A part of a value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Part {
    /// What it is.
    pub kind: Kind,
    /// Its text: a word's, a string's without quotes, a comment's without `/*` and `*/`, a
    /// separator's character, a function's name.
    pub value: String,
    /// Its first byte in the value.
    pub start: usize,
    /// The byte after its last.
    pub end: usize,
    /// The spaces before a separator, or after a function's `(`.
    pub before: String,
    /// The spaces after a separator, or before a function's `)`.
    pub after: String,
    /// A string's quote.
    pub quote: Option<char>,
    /// Whether a string, a comment or a function is left unclosed.
    pub unclosed: bool,
    /// A function's arguments.
    pub nodes: Vec<usize>,
}

impl Part {
    /// A part of a kind.
    fn new(kind: Kind, value: String, start: usize, end: usize) -> Self {
        Self {
            kind,
            value,
            start,
            end,
            before: String::new(),
            after: String::new(),
            quote: None,
            unclosed: false,
            nodes: Vec::new(),
        }
    }
}

/// A value read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Value {
    /// Every part.
    pub parts: Vec<Part>,
    /// The parts at the top, in order.
    pub nodes: Vec<usize>,
}

impl Value {
    /// A part by its index.
    #[must_use]
    pub fn part(&self, index: usize) -> &Part {
        &self.parts[index]
    }

    /// Visits every part in the order written, a function before its arguments, which are
    /// skipped when the visit returns false, as `walk` of `postcss-value-parser`.
    pub fn walk(&self, visit: &mut dyn FnMut(usize, &Part) -> bool) {
        let mut stack: Vec<usize> = self.nodes.iter().rev().copied().collect();
        while let Some(index) = stack.pop() {
            let part = &self.parts[index];
            if visit(index, part) && part.kind == Kind::Function {
                stack.extend(part.nodes.iter().rev());
            }
        }
    }
}

/// The byte at an offset, none past the end, as `charCodeAt` gives `NaN`.
fn at(bytes: &[u8], offset: usize) -> Option<u8> {
    bytes.get(offset).copied()
}

/// Whether a byte is a space or a control character (`code <= 32`).
fn is_low(code: Option<u8>) -> bool {
    code.is_some_and(|code| code <= 32)
}

/// Whether `text` is a unicode range's digits, `/^[a-f0-9?-]+$/i`.
fn is_range(text: &str) -> bool {
    !text.is_empty()
        && text
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() || byte == b'?' || byte == b'-')
}

/// Where the next unescaped `close` is from `from`, the text closed when none is; its byte and
/// whether the text was left open.
fn closing(value: &mut String, from: usize, close: char) -> (usize, bool) {
    let mut next = from;
    loop {
        let found = value.get(next + 1..).and_then(|rest| rest.find(close));
        let Some(found) = found else {
            value.push(close);
            return (value.len() - 1, true);
        };
        next = next + 1 + found;
        let escapes = value.as_bytes()[..next]
            .iter()
            .rev()
            .take_while(|byte| **byte == b'\\')
            .count();
        if escapes % 2 == 0 {
            return (next, false);
        }
    }
}

/// The reading of a value: the text, where it is, and the parts read so far.
struct Reader {
    /// The value's text, a quote or `)` added at its end when left open.
    value: String,
    /// The text's length before any addition.
    max: usize,
    /// The next byte to read.
    pos: usize,
    /// The byte at `pos`, none past the end.
    code: Option<u8>,
    /// The word read before a `(`, the function's name.
    name: String,
    /// The spaces before a separator.
    before: String,
    /// The spaces before a function's `)`.
    after: String,
    /// Every part.
    parts: Vec<Part>,
    /// The parts at the top.
    top: Vec<usize>,
    /// The functions open, innermost last.
    stack: Vec<usize>,
    /// Whether a function was closed: `postcss-value-parser`'s `parent` is then its list of the
    /// top, no longer undefined, and spaces before a `/` at the top stay a space part.
    closed: bool,
}

impl Reader {
    /// Adds a part to the innermost function, or to the top; its index.
    fn push(&mut self, part: Part) -> usize {
        let index = self.parts.len();
        self.parts.push(part);
        match self.stack.last() {
            Some(&parent) => self.parts[parent].nodes.push(index),
            None => self.top.push(index),
        }
        index
    }

    /// The last part of the innermost function, or of the top.
    fn last(&self) -> Option<usize> {
        match self.stack.last() {
            Some(&parent) => self.parts[parent].nodes.last().copied(),
            None => self.top.last().copied(),
        }
    }

    /// Whether the innermost function is `calc`.
    fn in_calc(&self) -> bool {
        self.stack
            .last()
            .is_some_and(|&parent| self.parts[parent].value == "calc")
    }

    /// The byte at an offset.
    fn at(&self, offset: usize) -> Option<u8> {
        at(self.value.as_bytes(), offset)
    }

    /// Moves to a byte.
    fn go(&mut self, pos: usize) {
        self.pos = pos;
        self.code = self.at(pos);
    }

    /// The byte after the spaces from the one after `from`, `code` set to it.
    fn skip_spaces(&mut self, from: usize) -> usize {
        let mut next = from;
        loop {
            next += 1;
            self.code = self.at(next);
            if !is_low(self.code) {
                return next;
            }
        }
    }

    /// Spaces: a function's last, a separator's after, a separator's before, or a part.
    fn spaces(&mut self) {
        let pos = self.pos;
        let next = self.skip_spaces(pos);
        let token = self.value[pos..next].to_owned();
        let code = self.code;
        let divided = self
            .last()
            .filter(|&index| self.parts[index].kind == Kind::Div);
        let slash_before = code == Some(b'/')
            && self.at(next + 1) != Some(b'*')
            && if self.stack.is_empty() {
                !self.closed
            } else {
                !self.in_calc()
            };
        if code == Some(b')') && !self.stack.is_empty() {
            self.after = token;
        } else if let Some(previous) = divided {
            self.parts[previous].end += token.len();
            self.parts[previous].after = token;
        } else if code == Some(b',') || code == Some(b':') || slash_before {
            self.before = token;
        } else {
            self.push(Part::new(Kind::Space, token, pos, next));
        }
        self.pos = next;
    }

    /// A quoted string.
    fn string(&mut self, quote: char) {
        let pos = self.pos;
        let (next, unclosed) = closing(&mut self.value, pos, quote);
        let mut part = Part::new(
            Kind::String,
            self.value[pos + 1..next].to_owned(),
            pos,
            if unclosed { next } else { next + 1 },
        );
        part.quote = Some(quote);
        part.unclosed = unclosed;
        self.push(part);
        self.go(next + 1);
    }

    /// A comment.
    fn comment(&mut self) {
        let pos = self.pos;
        let found = self.value[pos..].find("*/").map(|at| pos + at);
        let (next, unclosed) = found.map_or((self.value.len(), true), |next| (next, false));
        let mut part = Part::new(
            Kind::Comment,
            self.value.get(pos + 2..next).unwrap_or_default().to_owned(),
            pos,
            if unclosed { next } else { next + 2 },
        );
        part.unclosed = unclosed;
        self.push(part);
        self.go(next + 2);
    }

    /// A one-byte part: an operator of `calc()`, a word, or a separator.
    fn single(&mut self, kind: Kind, byte: u8) {
        let mut part = Part::new(
            kind,
            char::from(byte).to_string(),
            self.pos - self.before.len(),
            self.pos + 1,
        );
        if kind == Kind::Div {
            part.before = std::mem::take(&mut self.before);
        }
        self.push(part);
        self.go(self.pos + 1);
    }

    /// `(`: a function, `url(…)` unquoted read whole.
    fn open(&mut self) {
        let open = self.pos;
        let next = self.skip_spaces(open);
        let named = std::mem::take(&mut self.name);
        let start = open - named.len();
        let mut part = Part::new(Kind::Function, named, start, next + 1);
        self.value[open + 1..next].clone_into(&mut part.before);
        self.pos = next;
        if part.value == "url" && self.code != Some(b'\'') && self.code != Some(b'"') {
            self.url(part, open, next);
        } else {
            part.end = next + 1;
            let index = self.push(part);
            self.stack.push(index);
        }
    }

    /// The rest of an unquoted `url(`, from the first byte after its spaces.
    fn url(&mut self, part: Part, open: usize, first: usize) {
        let (next, unclosed) = closing(&mut self.value, first - 1, ')');
        let mut space = next;
        loop {
            space -= 1;
            if !is_low(self.at(space)) {
                break;
            }
        }
        let index = self.push(part);
        let mut arguments = Vec::new();
        if open < space {
            if first != space + 1 {
                arguments.push(Part::new(
                    Kind::Word,
                    self.value[first..=space].to_owned(),
                    first,
                    space + 1,
                ));
            }
            if unclosed && space + 1 != next {
                arguments.push(Part::new(
                    Kind::Space,
                    self.value[space + 1..next].to_owned(),
                    space + 1,
                    next,
                ));
            } else {
                self.value[space + 1..next].clone_into(&mut self.parts[index].after);
            }
        }
        for argument in arguments {
            let at = self.parts.len();
            self.parts.push(argument);
            self.parts[index].nodes.push(at);
        }
        self.go(next + 1);
        self.parts[index].unclosed = unclosed;
        self.parts[index].end = if unclosed { next } else { self.pos };
    }

    /// `)` closing the innermost function.
    fn close(&mut self) {
        self.go(self.pos + 1);
        if let Some(parent) = self.stack.pop() {
            self.parts[parent].after = std::mem::take(&mut self.after);
            self.parts[parent].end = self.pos;
        }
        self.closed = true;
    }

    /// A word, a unicode range, or a function's name before its `(`.
    fn word(&mut self) {
        let pos = self.pos;
        let calc = self.in_calc();
        let open = !self.stack.is_empty();
        let mut next = pos;
        loop {
            if self.code == Some(b'\\') {
                next += 1;
            }
            next += 1;
            self.code = self.at(next);
            let ends = is_low(self.code)
                || matches!(self.code, Some(b'\'' | b'"' | b',' | b':' | b'/' | b'('))
                || (self.code == Some(b'*') && calc)
                || (self.code == Some(b')') && open);
            if next >= self.max || ends {
                break;
            }
        }
        let token = self
            .value
            .get(pos..next.min(self.value.len()))
            .unwrap_or_default()
            .to_owned();
        if self.code == Some(b'(') {
            self.name = token;
        } else if token.len() > 1
            && matches!(token.as_bytes()[0], b'u' | b'U')
            && token.as_bytes()[1] == b'+'
            && is_range(&token[2..])
        {
            self.push(Part::new(Kind::UnicodeRange, token, pos, next));
        } else {
            self.push(Part::new(Kind::Word, token, pos, next));
        }
        self.pos = next;
    }
}

/// A value read into its parts.
#[must_use]
pub fn parse(input: &str) -> Value {
    let mut reader = Reader {
        value: input.to_owned(),
        max: input.len(),
        pos: 0,
        code: at(input.as_bytes(), 0),
        name: String::new(),
        before: String::new(),
        after: String::new(),
        parts: Vec::new(),
        top: Vec::new(),
        stack: Vec::new(),
        closed: false,
    };
    while reader.pos < reader.max {
        let current = reader.code.unwrap_or_default();
        let comment = current == b'/' && reader.at(reader.pos + 1) == Some(b'*');
        match current {
            0..=32 => reader.spaces(),
            b'\'' | b'"' => reader.string(char::from(current)),
            _ if comment => reader.comment(),
            b'/' | b'*' if reader.in_calc() => reader.single(Kind::Word, current),
            b'/' | b',' | b':' => reader.single(Kind::Div, current),
            b'(' => reader.open(),
            b')' if !reader.stack.is_empty() => reader.close(),
            _ => reader.word(),
        }
    }
    let length = reader.value.len();
    for &open in &reader.stack {
        reader.parts[open].unclosed = true;
        reader.parts[open].end = length;
    }
    Value {
        parts: reader.parts,
        nodes: reader.top,
    }
}

/// A number and its unit, as `unit` of `postcss-value-parser` reads them; none for a text that
/// does not start like a number.
#[must_use]
pub fn unit(value: &str) -> Option<(&str, &str)> {
    let bytes = value.as_bytes();
    let digit = |offset: usize| bytes.get(offset).is_some_and(u8::is_ascii_digit);
    let first = *bytes.first()?;
    let like_number = match first {
        b'+' | b'-' => digit(1) || (bytes.get(1) == Some(&b'.') && digit(2)),
        b'.' => digit(1),
        _ => first.is_ascii_digit(),
    };
    if !like_number {
        return None;
    }
    let mut pos = usize::from(first == b'+' || first == b'-');
    while digit(pos) {
        pos += 1;
    }
    if bytes.get(pos) == Some(&b'.') && digit(pos + 1) {
        pos += 2;
        while digit(pos) {
            pos += 1;
        }
    }
    if matches!(bytes.get(pos), Some(b'e' | b'E')) {
        let sign = matches!(bytes.get(pos + 1), Some(b'+' | b'-'));
        if digit(pos + 1) || (sign && digit(pos + 2)) {
            pos += if sign { 3 } else { 2 };
            while digit(pos) {
                pos += 1;
            }
        }
    }
    Some((&value[..pos], &value[pos..]))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use oxc_allocator::Allocator;
    use oxc_ast::ast::Statement;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    use super::{Kind, Value, parse, unit};
    use crate::template::corpus::{Value as Js, value as js_value};

    /// The array `var tests = […]` of a test file of `postcss-value-parser` 4.2.0, copied as it
    /// is into `tests/fixtures/scss/postcss-value-parser/`, named `.txt` so that the formatter
    /// leaves it byte for byte.
    fn upstream(file: &str) -> Vec<Js> {
        let path = format!(
            "{}/tests/fixtures/scss/postcss-value-parser/{file}",
            env!("CARGO_MANIFEST_DIR")
        );
        let text = fs::read_to_string(path).unwrap_or_default();
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, &text, SourceType::cjs()).parse();
        for statement in &parsed.program.body {
            if let Statement::VariableDeclaration(variables) = statement {
                for declarator in &variables.declarations {
                    let named = declarator.id.get_identifier_name();
                    if named.as_deref() == Some("tests")
                        && let Some(Js::List(cases)) = declarator
                            .init
                            .as_ref()
                            .and_then(|init| js_value(init, &[]))
                    {
                        return cases;
                    }
                }
            }
        }
        Vec::new()
    }

    /// An object's property as JavaScript reads it: the last one written of a name.
    fn get<'v>(object: &'v Js, key: &str) -> Option<&'v Js> {
        match object {
            Js::Object(properties) => properties
                .iter()
                .rev()
                .find(|(each, _)| each == key)
                .map(|(_, found)| found),
            _ => None,
        }
    }

    /// A value's parts as `postcss-value-parser` prints them, each object's keys sorted.
    fn printed(value: &Value, nodes: &[usize]) -> Js {
        Js::List(
            nodes
                .iter()
                .map(|&index| {
                    let part = value.part(index);
                    let kind = match part.kind {
                        Kind::Word => "word",
                        Kind::String => "string",
                        Kind::Div => "div",
                        Kind::Space => "space",
                        Kind::Comment => "comment",
                        Kind::Function => "function",
                        Kind::UnicodeRange => "unicode-range",
                    };
                    let number =
                        |offset: usize| Js::Number(f64::from(u32::try_from(offset).unwrap_or(0)));
                    let mut properties = vec![
                        ("type".to_owned(), Js::Text(kind.to_owned())),
                        ("sourceIndex".to_owned(), number(part.start)),
                        ("sourceEndIndex".to_owned(), number(part.end)),
                        ("value".to_owned(), Js::Text(part.value.clone())),
                    ];
                    if let Some(quote) = part.quote {
                        properties.push(("quote".to_owned(), Js::Text(quote.to_string())));
                    }
                    if part.unclosed {
                        properties.push(("unclosed".to_owned(), Js::Bool(true)));
                    }
                    if matches!(part.kind, Kind::Div | Kind::Function) {
                        properties.push(("before".to_owned(), Js::Text(part.before.clone())));
                        properties.push(("after".to_owned(), Js::Text(part.after.clone())));
                    }
                    if part.kind == Kind::Function {
                        properties.push(("nodes".to_owned(), printed(value, &part.nodes)));
                    }
                    sorted(Js::Object(properties))
                })
                .collect(),
        )
    }

    /// A value with each object's keys sorted, so that two compare whatever their order.
    fn sorted(value: Js) -> Js {
        match value {
            Js::List(items) => Js::List(items.into_iter().map(sorted).collect()),
            Js::Object(mut properties) => {
                properties.sort_by(|a, b| a.0.cmp(&b.0));
                Js::Object(
                    properties
                        .into_iter()
                        .map(|(key, each)| (key, sorted(each)))
                        .collect(),
                )
            }
            other => other,
        }
    }

    /// Every case of `test/parse.js`: 74, each value read into the parts it expects.
    #[test]
    fn passes_the_parse_tests_of_postcss_value_parser() {
        let cases = upstream("parse.txt");

        assert_eq!(cases.len(), 74);
        let failures: Vec<String> = cases
            .iter()
            .filter_map(|case| {
                let fixture = get(case, "fixture").map(Js::display).unwrap_or_default();
                let expected = sorted(get(case, "expected")?.clone());
                let value = parse(&fixture);
                let found = printed(&value, &value.nodes);
                (found != expected).then(|| format!("{fixture:?}: {found:?}"))
            })
            .collect();
        assert_eq!(failures, Vec::<String>::new());
    }

    /// Every case of `test/unit.js`, each text's number and unit or `false`: 67, the one object
    /// writing `fixture` twice read by its last.
    #[test]
    fn passes_the_unit_tests_of_postcss_value_parser() {
        let cases = upstream("unit.txt");

        assert_eq!(cases.len(), 67);
        for case in &cases {
            let fixture = get(case, "fixture").map(Js::display).unwrap_or_default();
            let expected = get(case, "expected").cloned();
            let found = unit(&fixture).map_or(Js::Bool(false), |(number, unit)| {
                Js::Object(vec![
                    ("number".to_owned(), Js::Text(number.to_owned())),
                    ("unit".to_owned(), Js::Text(unit.to_owned())),
                ])
            });
            assert_eq!(Some(found), expected, "{fixture:?}");
        }
    }

    #[test]
    fn walks_a_function_s_arguments_unless_told_to_skip_them() {
        // `a(b) c(d)`: a, b, the space, c, d; skipping c's arguments leaves d out.
        let value = parse("a(b) c(d)");
        let mut seen = Vec::new();
        value.walk(&mut |_, part| {
            seen.push(part.value.clone());
            part.value != "c"
        });

        assert_eq!(seen, ["a", "b", " ", "c"]);
    }
}
