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

/// A JSON string literal, quotes included, for the hooks' output: escaped as JavaScript's
/// `JSON.stringify` escapes it, so that a ported hook writes the same bytes.
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
            '\u{8}' => quoted.push_str("\\b"),
            '\u{c}' => quoted.push_str("\\f"),
            control if u32::from(control) < 0x20 => {
                let _ = write!(quoted, "\\u{:04x}", u32::from(control));
            }
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

/// A number as JavaScript writes it (ECMA-262, `Number::toString`): the shortest digits that read
/// back the same, in plain notation from 1e-7 to 1e21, else in exponent notation, `1e+21`;
/// `null` for a number that is not finite, as `JSON.stringify` writes it.
#[must_use]
pub fn number_text(number: f64) -> String {
    if !number.is_finite() {
        return "null".to_owned();
    }
    if number == 0.0 {
        return "0".to_owned();
    }

    // Rust's `{:e}` writes the same shortest digits: `-1.5e-7`.
    let scientific = format!("{:e}", number.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let count = i64::try_from(digits.len()).unwrap_or(i64::MAX);
    let point = exponent.parse::<i64>().unwrap_or(0) + 1;
    let sign = if number < 0.0 { "-" } else { "" };
    let zeros = |amount: i64| "0".repeat(usize::try_from(amount).unwrap_or(0));
    let at = usize::try_from(point).unwrap_or(0);

    let body = if count <= point && point <= 21 {
        format!("{digits}{}", zeros(point - count))
    } else if 0 < point && point <= 21 {
        format!("{}.{}", &digits[..at], &digits[at..])
    } else if -6 < point && point <= 0 {
        format!("0.{}{digits}", zeros(-point))
    } else {
        let rest = if digits.len() > 1 {
            format!(".{}", &digits[1..])
        } else {
            String::new()
        };
        let exponent_sign = if point - 1 < 0 { "-" } else { "+" };

        format!(
            "{}{rest}e{exponent_sign}{}",
            &digits[..1],
            (point - 1).abs()
        )
    };

    format!("{sign}{body}")
}

/// The length of a text in UTF-16 code units, as JavaScript's `length` counts it.
#[must_use]
pub fn js_length(text: &str) -> usize {
    text.encode_utf16().count()
}

/// A value written back as JSON text, members in their order, as `JSON.stringify` writes it: a
/// hook's updated tool input, the size of a call's input or result.
#[must_use]
pub fn write(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number_text(*number),
        Value::String(text) => quote(text),
        Value::Array(items) => format!(
            "[{}]",
            items.iter().map(write).collect::<Vec<_>>().join(",")
        ),
        Value::Object(members) => format!(
            "{{{}}}",
            members
                .iter()
                .map(|(name, member)| format!("{}:{}", quote(name), write(member)))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
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
