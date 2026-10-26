//! The numbers of a value, as the CSS tokenizer (`@csstools/css-tokenizer`, which Stylelint
//! 17.16.0's `number-max-precision` and `unit-no-unknown` read values with) finds them, by a
//! reading of ours: numbers, percentages and dimensions, outside strings, comments, names and
//! hashes, each with the functions it sits in; `url()`'s content skipped.

/// A number found: where it starts, where its digits end, its unit, and the functions around it,
/// innermost last.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number {
    /// Its first byte.
    pub start: usize,
    /// The byte after its digits.
    pub end: usize,
    /// Its unit, `%` for a percentage, empty for a plain number.
    pub unit: String,
    /// The names of the functions it sits in, outermost first.
    pub functions: Vec<String>,
}

/// Whether a byte may continue a name.
fn is_name(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte >= 0x80
}

/// The numbers of a text.
#[must_use]
pub fn numbers(text: &str) -> Vec<Number> {
    let bytes = text.as_bytes();
    let digit_at = |offset: usize| bytes.get(offset).is_some_and(u8::is_ascii_digit);
    let mut found = Vec::new();
    let mut functions: Vec<String> = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let byte = bytes[at];
        let signed_number = matches!(byte, b'+' | b'-')
            && (digit_at(at + 1) || (bytes.get(at + 1) == Some(&b'.') && digit_at(at + 2)));
        if byte == b'"' || byte == b'\'' {
            at += 1;
            while at < bytes.len() && bytes[at] != byte {
                at += 1 + usize::from(bytes[at] == b'\\');
            }
            at += 1;
        } else if byte == b'/' && bytes.get(at + 1) == Some(&b'*') {
            at = text[at + 2..]
                .find("*/")
                .map_or(bytes.len(), |end| at + 2 + end + 2);
        } else if byte.is_ascii_digit() || (byte == b'.' && digit_at(at + 1)) || signed_number {
            let start = at;
            at += usize::from(matches!(byte, b'+' | b'-'));
            while digit_at(at) {
                at += 1;
            }
            if bytes.get(at) == Some(&b'.') && digit_at(at + 1) {
                at += 1;
                while digit_at(at) {
                    at += 1;
                }
            }
            if matches!(bytes.get(at), Some(b'e' | b'E'))
                && (digit_at(at + 1)
                    || (matches!(bytes.get(at + 1), Some(b'+' | b'-')) && digit_at(at + 2)))
            {
                at += 2;
                while digit_at(at) {
                    at += 1;
                }
            }
            let end = at;
            if bytes.get(at) == Some(&b'%') {
                at += 1;
            } else if bytes
                .get(at)
                .is_some_and(|next| next.is_ascii_alphabetic() || *next == b'_' || *next >= 0x80)
                || (bytes.get(at) == Some(&b'-')
                    && bytes
                        .get(at + 1)
                        .is_some_and(|next| next.is_ascii_alphabetic() || *next == b'-'))
            {
                while bytes.get(at).is_some_and(|next| is_name(*next)) {
                    at += 1;
                }
            }
            found.push(Number {
                start,
                end,
                unit: text[end..at].to_owned(),
                functions: functions.clone(),
            });
        } else if is_name(byte) || byte == b'\\' {
            let start = at;
            at += 1;
            while bytes
                .get(at)
                .is_some_and(|next| is_name(*next) || *next == b'\\')
            {
                at += 1;
            }
            if bytes.get(at) == Some(&b'(') {
                let name = text[start..at].to_owned();
                at += 1;
                if name.eq_ignore_ascii_case("url") {
                    at = text[at..].find(')').map_or(bytes.len(), |end| at + end + 1);
                } else {
                    functions.push(name);
                }
            }
        } else if byte == b'#' {
            at += 1;
            while bytes.get(at).is_some_and(|next| is_name(*next)) {
                at += 1;
            }
        } else {
            match byte {
                b'(' => functions.push(String::new()),
                b')' => {
                    functions.pop();
                }
                _ => {}
            }
            at += 1;
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::numbers;

    #[test]
    fn finds_the_numbers_their_units_and_functions() {
        let found = numbers("calc(1.5px + -2%) 'a 3' /* 4 */ #5a b6 url(7x) image-set(a 2x) 1e3em");
        let read: Vec<(&str, Vec<&str>)> = found
            .iter()
            .map(|each| {
                (
                    each.unit.as_str(),
                    each.functions.iter().map(String::as_str).collect(),
                )
            })
            .collect();
        assert_eq!(
            read,
            [
                ("px", vec!["calc"]),
                ("%", vec!["calc"]),
                ("x", vec!["image-set"]),
                ("em", vec![])
            ]
        );
        assert_eq!((found[0].start, found[0].end), (5, 8));
    }
}
