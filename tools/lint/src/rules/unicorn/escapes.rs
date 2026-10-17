//! The escapes of string literals and templates, as Unicorn's string rules read their raw text
//! (EN-080): a numeric escape `\xHH`, `\uHHHH` or `\u{H…}` at an offset, whether a backslash is
//! itself escaped, and a template's raw text with its backticks and `${` escaped
//! (`eslint-plugin-unicorn`'s `utils/is-escaped-character`, `utils/escape-template-element-raw`).

/// A numeric escape: where it ends in the text, and the code point it writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericEscape {
    /// The offset after it.
    pub end: usize,
    /// The code point.
    pub value: u32,
    /// Its form.
    pub form: EscapeForm,
}

/// How a numeric escape is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscapeForm {
    /// `\xHH`.
    Hex,
    /// `\uHHHH`.
    Unicode,
    /// `\u{H…}`.
    CodePoint,
}

/// The value of hexadecimal digits.
fn hex_value(digits: &str) -> Option<u32> {
    u32::from_str_radix(digits, 16).ok()
}

/// The numeric escape starting at an offset of a raw text, at its backslash.
#[must_use]
pub fn numeric_escape(text: &str, at: usize) -> Option<NumericEscape> {
    let rest = text.get(at..)?;
    let is_hex = |each: &str| !each.is_empty() && each.bytes().all(|byte| byte.is_ascii_hexdigit());
    if let Some(digits) = rest.strip_prefix("\\x").and_then(|after| after.get(..2))
        && is_hex(digits)
    {
        return Some(NumericEscape {
            end: at + 4,
            value: hex_value(digits)?,
            form: EscapeForm::Hex,
        });
    }
    let after_u = rest.strip_prefix("\\u")?;
    if let Some(digits) = after_u.get(..4)
        && is_hex(digits)
    {
        return Some(NumericEscape {
            end: at + 6,
            value: hex_value(digits)?,
            form: EscapeForm::Unicode,
        });
    }
    let braced = after_u.strip_prefix('{')?;
    let close = braced.find('}')?;
    let digits = &braced[..close];
    if !is_hex(digits) {
        return None;
    }
    Some(NumericEscape {
        end: at + 3 + close + 1,
        value: hex_value(digits).unwrap_or(u32::MAX),
        form: EscapeForm::CodePoint,
    })
}

/// Whether the character at an offset is escaped: an odd number of backslashes before it.
#[must_use]
pub fn is_escaped(text: &str, at: usize) -> bool {
    text.as_bytes()[..at]
        .iter()
        .rev()
        .take_while(|byte| **byte == b'\\')
        .count()
        % 2
        == 1
}

/// A template's raw text with every unescaped backtick and `${` escaped, so that it stays
/// literal (`escapeTemplateElementRaw`).
#[must_use]
pub fn escape_template_raw(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut backslashes = 0;
    let characters: Vec<char> = text.chars().collect();
    for (index, character) in characters.iter().enumerate() {
        let symbol =
            *character == '`' || (*character == '$' && characters.get(index + 1) == Some(&'{'));
        if symbol && backslashes % 2 == 0 {
            out.push('\\');
        }
        out.push(*character);
        backslashes = if *character == '\\' {
            backslashes + 1
        } else {
            0
        };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{EscapeForm, NumericEscape, escape_template_raw, is_escaped, numeric_escape};

    #[test]
    fn reads_the_three_numeric_escapes() {
        // `\x41` is 0x41 = 65, ending 4 bytes on; `B` 66 after 6; `\u{1F4A9}` 128169 after 9.
        assert_eq!(
            numeric_escape("\\x41", 0),
            Some(NumericEscape {
                end: 4,
                value: 65,
                form: EscapeForm::Hex
            })
        );
        assert_eq!(
            numeric_escape("a\\u0042", 1),
            Some(NumericEscape {
                end: 7,
                value: 66,
                form: EscapeForm::Unicode
            })
        );
        assert_eq!(
            numeric_escape("\\u{1F4A9}", 0),
            Some(NumericEscape {
                end: 9,
                value: 0x1F4A9,
                form: EscapeForm::CodePoint
            })
        );
        assert_eq!(numeric_escape("\\n", 0), None);
        assert_eq!(numeric_escape("\\x4", 0), None);
        assert_eq!(numeric_escape("\\u{}", 0), None);
    }

    #[test]
    fn counts_the_backslashes_before_a_character() {
        assert!(is_escaped("\\n", 1));
        assert!(!is_escaped("\\\\n", 2));
        assert!(!is_escaped("n", 0));
    }

    #[test]
    fn escapes_backticks_and_interpolation_openings() {
        assert_eq!(escape_template_raw("a`b${c}"), "a\\`b\\${c}");
        assert_eq!(escape_template_raw("\\`$x"), "\\`$x");
        assert_eq!(escape_template_raw("\\\\`"), "\\\\\\`");
    }
}
