//! A small JSON reader (RFC 8259), standard library only: hook inputs and transcript lines.

use std::fmt::Write;

/// A parsed JSON value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    /// Members in their order; a repeated name keeps its first value when read.
    Object(Vec<(String, Value)>),
}

/// Value returned by a missing member.
static NULL: Value = Value::Null;

impl Value {
    /// Member of an object, `Null` when missing or when this is not an object.
    #[must_use]
    pub fn get(&self, key: &str) -> &Value {
        match self {
            Value::Object(members) => members
                .iter()
                .find(|(name, _)| name == key)
                .map_or(&NULL, |(_, value)| value),
            _ => &NULL,
        }
    }

    /// The string, empty when this is not a string.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Value::String(text) => text,
            _ => "",
        }
    }

    /// The items, none when this is not an array.
    #[must_use]
    pub fn items(&self) -> &[Value] {
        match self {
            Value::Array(items) => items,
            _ => &[],
        }
    }
}

/// Parses a JSON text, `None` when it is not one value of valid JSON.
#[must_use]
pub fn parse(text: &str) -> Option<Value> {
    let mut reader = Reader {
        bytes: text.as_bytes(),
        at: 0,
    };
    let value = reader.value()?;

    reader.space();
    (reader.at == reader.bytes.len()).then_some(value)
}

/// A JSON string literal, quotes included, for the hooks' output.
#[must_use]
pub fn quote(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len() + 2);

    quoted.push('"');
    for character in text.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            control if u32::from(control) < 0x20 => {
                let _ = write!(quoted, "\\u{:04x}", u32::from(control));
            }
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

/// Recursive descent over the bytes of a JSON text.
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    fn space(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.at += 1;
        }
    }

    fn eat(&mut self, expected: u8) -> Option<()> {
        self.space();
        (self.peek() == Some(expected)).then(|| self.at += 1)
    }

    fn word(&mut self, word: &str, value: Value) -> Option<Value> {
        let end = self.at + word.len();

        (self.bytes.get(self.at..end) == Some(word.as_bytes())).then(|| {
            self.at = end;
            value
        })
    }

    fn value(&mut self) -> Option<Value> {
        self.space();
        match self.peek()? {
            b'n' => self.word("null", Value::Null),
            b't' => self.word("true", Value::Bool(true)),
            b'f' => self.word("false", Value::Bool(false)),
            b'"' => self.string().map(Value::String),
            b'[' => self.array(),
            b'{' => self.object(),
            _ => self.number(),
        }
    }

    fn number(&mut self) -> Option<Value> {
        let start = self.at;

        while matches!(
            self.peek(),
            Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
        ) {
            self.at += 1;
        }

        std::str::from_utf8(&self.bytes[start..self.at])
            .ok()?
            .parse()
            .ok()
            .map(Value::Number)
    }

    fn array(&mut self) -> Option<Value> {
        self.at += 1;
        let mut items = Vec::new();

        if self.eat(b']').is_some() {
            return Some(Value::Array(items));
        }

        loop {
            items.push(self.value()?);
            if self.eat(b']').is_some() {
                return Some(Value::Array(items));
            }
            self.eat(b',')?;
        }
    }

    fn object(&mut self) -> Option<Value> {
        self.at += 1;
        let mut members = Vec::new();

        if self.eat(b'}').is_some() {
            return Some(Value::Object(members));
        }

        loop {
            self.space();
            if self.peek() != Some(b'"') {
                return None;
            }
            let name = self.string()?;

            self.eat(b':')?;
            members.push((name, self.value()?));
            if self.eat(b'}').is_some() {
                return Some(Value::Object(members));
            }
            self.eat(b',')?;
        }
    }

    fn hex(&mut self) -> Option<u32> {
        let digits = std::str::from_utf8(self.bytes.get(self.at..self.at + 4)?).ok()?;

        self.at += 4;
        u32::from_str_radix(digits, 16).ok()
    }

    fn escape(&mut self) -> Option<char> {
        let byte = self.peek()?;

        self.at += 1;
        match byte {
            b'"' => Some('"'),
            b'\\' => Some('\\'),
            b'/' => Some('/'),
            b'b' => Some('\u{8}'),
            b'f' => Some('\u{c}'),
            b'n' => Some('\n'),
            b'r' => Some('\r'),
            b't' => Some('\t'),
            b'u' => {
                let high = self.hex()?;

                if (0xD800..0xDC00).contains(&high) {
                    // A surrogate pair: `😀`.
                    self.word("\\u", Value::Null)?;
                    let low = self.hex()?;

                    char::from_u32(0x10000 + ((high - 0xD800) << 10) + low.checked_sub(0xDC00)?)
                } else {
                    char::from_u32(high)
                }
            }
            _ => None,
        }
    }

    fn string(&mut self) -> Option<String> {
        self.at += 1;
        let mut text = Vec::new();

        loop {
            match self.peek()? {
                b'"' => {
                    self.at += 1;
                    return String::from_utf8(text).ok();
                }
                b'\\' => {
                    self.at += 1;
                    let mut buffer = [0; 4];

                    text.extend_from_slice(self.escape()?.encode_utf8(&mut buffer).as_bytes());
                }
                byte => {
                    text.push(byte);
                    self.at += 1;
                }
            }
        }
    }
}
