//! `unicorn/prefer-unicode-code-point-escapes`: a string, an untagged template or a regular
//! expression writes `\u{…}` rather than `\xHH`, `\uHHHH`, a surrogate pair, a legacy octal or,
//! in a regular expression, `\cX` (`eslint-plugin-unicorn` 78.0.0). Printable ASCII and the
//! short escapes of strings are left to their own rules. Autofixed, as in `ESLint`, but in a
//! regular expression without the `u` or `v` flag, where adding it is a suggestion: reported only.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::Span;

use super::syntax::text;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferUnicodeCodePointEscapes;

/// The code points with a short escape in a string: `\0`, `\b`, `\t`, `\n`, `\v`, `\f`, `\r`,
/// `\"`, `\'`, `/`, `\\` and `` \` ``.
const SHORT_ESCAPE_CODE_POINTS: [u32; 12] = [0, 8, 9, 10, 11, 12, 13, 34, 39, 47, 92, 96];

/// A code point escape, `\u{…}` in upper case.
fn code_point_escape(value: u32) -> String {
    format!("\\u{{{value:X}}}")
}

/// The value of a run of hexadecimal digits of an exact length at an offset.
fn hex_at(text: &[u8], at: usize, length: usize) -> Option<u32> {
    let digits = text.get(at..at + length)?;
    if !digits.iter().all(u8::is_ascii_hexdigit) {
        return None;
    }
    u32::from_str_radix(std::str::from_utf8(digits).ok()?, 16).ok()
}

/// A replaced escape: where it ends, its code point escape, and the value it wrote.
struct Replaced {
    /// The offset after it.
    end: usize,
    /// Its replacement.
    replacement: String,
    /// The code point, for `\x` and `\u`.
    value: Option<u32>,
}

/// Where an escape is read.
#[derive(Clone, Copy)]
struct Context {
    /// In a regular expression.
    is_regex: bool,
    /// In a character class of one.
    in_class: bool,
    /// Whether legacy octal escapes are read, in strings.
    allow_octal: bool,
}

/// Whether a code point is a high surrogate.
const fn is_high_surrogate(value: u32) -> bool {
    value >= 0xD800 && value <= 0xDBFF
}

/// Whether a code point is a low surrogate.
const fn is_low_surrogate(value: u32) -> bool {
    value >= 0xDC00 && value <= 0xDFFF
}

/// The `\uHHHH` escape at an offset, joined with a following low surrogate when it is a high one.
fn unicode_escape(text: &[u8], at: usize, context: Context) -> Option<Replaced> {
    if text.get(at + 1) != Some(&b'u') || text.get(at + 2) == Some(&b'{') {
        return None;
    }
    let value = hex_at(text, at + 2, 4)?;
    let prefer_short = !context.is_regex;
    if prefer_short
        && SHORT_ESCAPE_CODE_POINTS.contains(&value)
        && (value != 0 || !text.get(at + 6).is_some_and(u8::is_ascii_digit))
    {
        return None;
    }
    let allow_surrogate = !context.is_regex || !context.in_class;
    if !allow_surrogate && (is_high_surrogate(value) || is_low_surrogate(value)) {
        return None;
    }
    let next = at + 6;
    if !context.in_class
        && is_high_surrogate(value)
        && text.get(next) == Some(&b'\\')
        && text.get(next + 1) == Some(&b'u')
        && let Some(low) = hex_at(text, next + 2, 4).filter(|low| is_low_surrogate(*low))
    {
        let code_point = (value - 0xD800) * 0x400 + (low - 0xDC00) + 0x10000;
        return Some(Replaced {
            end: next + 6,
            replacement: code_point_escape(code_point),
            value: Some(code_point),
        });
    }
    Some(Replaced {
        end: at + 6,
        replacement: code_point_escape(value),
        value: Some(value),
    })
}

/// The legacy octal escape at an offset, `\1` to `\377`, but a lone `\0`.
fn octal_escape(text: &[u8], at: usize) -> Option<Replaced> {
    let is_octal = |byte: Option<&u8>| byte.is_some_and(|each| (b'0'..=b'7').contains(each));
    let first = text.get(at + 1)?;
    if *first == b'0' && !is_octal(text.get(at + 2)) {
        return None;
    }
    if !is_octal(Some(first)) {
        return None;
    }
    let maximum = if *first <= b'3' { 3 } else { 2 };
    let mut end = at + 1;
    while end < at + 1 + maximum && is_octal(text.get(end)) {
        end += 1;
    }
    let digits = std::str::from_utf8(&text[at + 1..end]).ok()?;
    Some(Replaced {
        end,
        replacement: code_point_escape(u32::from_str_radix(digits, 8).ok()?),
        value: None,
    })
}

/// The escape at an offset to write as a code point escape, when there is one.
fn replacement_at(text: &[u8], at: usize, context: Context) -> Option<Replaced> {
    let legacy = if text.get(at + 1) == Some(&b'x') {
        hex_at(text, at + 2, 2).map(|value| Replaced {
            end: at + 4,
            replacement: code_point_escape(value),
            value: Some(value),
        })
    } else {
        unicode_escape(text, at, context)
    };
    if !context.is_regex
        && legacy
            .as_ref()
            .and_then(|each| each.value)
            .is_some_and(|value| (0x20..=0x7E).contains(&value))
    {
        return None;
    }
    if legacy.is_some() {
        return legacy;
    }
    if context.is_regex {
        let letter = text.get(at + 2).copied().filter(u8::is_ascii_alphabetic)?;
        return (text.get(at + 1) == Some(&b'c')).then(|| Replaced {
            end: at + 3,
            replacement: code_point_escape(u32::from(letter.to_ascii_uppercase()) % 32),
            value: None,
        });
    }
    if context.allow_octal {
        return octal_escape(text, at);
    }
    None
}

/// Whether a `\u{…}` escape stands at an offset, up to the largest code point.
fn is_code_point_escape(text: &[u8], at: usize) -> bool {
    if text.get(at..at + 3) != Some(b"\\u{") {
        return false;
    }
    let mut end = at + 3;
    while text.get(end).is_some_and(u8::is_ascii_hexdigit) {
        end += 1;
    }
    end > at + 3
        && text.get(end) == Some(&b'}')
        && std::str::from_utf8(&text[at + 3..end])
            .ok()
            .and_then(|digits| u32::from_str_radix(digits, 16).ok())
            .is_some_and(|value| value <= 0x10_FFFF)
}

/// A text with its legacy escapes replaced: the result, whether one was, and whether a
/// regular expression already holds a code point escape.
fn replace_escapes(
    raw: &str,
    is_regex: bool,
    nested_classes: bool,
    allow_octal: bool,
) -> (String, bool, bool) {
    let text = raw.as_bytes();
    let mut fixed = String::with_capacity(raw.len());
    let (mut depth, mut replaced, mut has_code_point) = (0_u32, false, false);
    let mut at = 0;
    while at < text.len() {
        let character = raw[at..].chars().next().unwrap_or_default();
        if character == '\\' {
            if is_regex && is_code_point_escape(text, at) {
                has_code_point = true;
            }
            let context = Context {
                is_regex,
                in_class: depth > 0,
                allow_octal,
            };
            if let Some(each) = replacement_at(text, at, context) {
                fixed.push_str(&each.replacement);
                at = each.end;
                replaced = true;
                continue;
            }
            fixed.push('\\');
            at += 1;
            if let Some(next) = raw[at..].chars().next() {
                fixed.push(next);
                at += next.len_utf8();
            }
            continue;
        }
        if is_regex && character == '[' && (depth == 0 || nested_classes) {
            depth += 1;
        } else if is_regex && character == ']' && depth > 0 {
            depth -= 1;
        }
        fixed.push(character);
        at += character.len_utf8();
    }
    (fixed, replaced, has_code_point)
}

impl Rule for PreferUnicodeCodePointEscapes {
    fn name(&self) -> &'static str {
        "unicorn/prefer-unicode-code-point-escapes"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let message = "Prefer Unicode code point escapes.".to_owned();
        let nodes = source.semantic.nodes();
        match node.kind() {
            AstKind::StringLiteral(literal) => {
                let raw = text(source, literal.span);
                if !raw.starts_with(['\'', '"']) {
                    return;
                }
                let (fixed, replaced, _) = replace_escapes(raw, false, false, true);
                if replaced {
                    findings.push(source.fixed(
                        self.name(),
                        literal.span.start,
                        message,
                        Fix {
                            start: literal.span.start,
                            end: literal.span.end,
                            text: fixed,
                        },
                    ));
                }
            }
            AstKind::TemplateElement(element) => {
                let parent = nodes.parent_id(node.id());
                if matches!(
                    nodes.parent_kind(parent),
                    AstKind::TaggedTemplateExpression(_)
                ) {
                    return;
                }
                let (fixed, replaced, _) =
                    replace_escapes(text(source, element.span), false, false, false);
                if replaced {
                    findings.push(source.fixed(
                        self.name(),
                        element.span.start.saturating_sub(1),
                        message,
                        Fix {
                            start: element.span.start,
                            end: element.span.end,
                            text: fixed,
                        },
                    ));
                }
            }
            AstKind::RegExpLiteral(literal) => {
                let raw = text(source, literal.span);
                let pattern_end = raw.rfind('/').unwrap_or(raw.len());
                let pattern = raw.get(1..pattern_end).unwrap_or_default();
                let flags = &raw[pattern_end + 1..];
                let unicode = flags.contains(['u', 'v']);
                let (fixed, replaced, has_code_point) =
                    replace_escapes(pattern, true, flags.contains('v'), true);
                if !replaced && (unicode || !has_code_point) {
                    return;
                }
                if !unicode {
                    findings.push(source.finding(self.name(), literal.span.start, message));
                    return;
                }
                let span = Span::new(
                    literal.span.start + 1,
                    literal.span.start + u32::try_from(pattern_end).unwrap_or(0),
                );
                findings.push(source.fixed(
                    self.name(),
                    literal.span.start,
                    message,
                    Fix {
                        start: span.start,
                        end: span.end,
                        text: fixed,
                    },
                ));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferUnicodeCodePointEscapes;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &PreferUnicodeCodePointEscapes,
            "playground/a.ts",
            code,
        ))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferUnicodeCodePointEscapes, "playground/a.ts", code),
        )
    }

    #[test]
    fn fixes_the_documented_legacy_escapes() {
        assert_eq!(
            fix("const foo = '\\xA9';\nconst bar = '\\u2661';\nconst baz = '\\uD83D\\uDCA9';"),
            "const foo = '\\u{A9}';\nconst bar = '\\u{2661}';\nconst baz = '\\u{1F4A9}';"
        );
        assert_eq!(
            fix("const foo = `\\xA9${bar}\\u2661`;"),
            "const foo = `\\u{A9}${bar}\\u{2661}`;"
        );
        assert_eq!(fix("const foo = /\\u0061/u;"), "const foo = /\\u{61}/u;");
        assert_eq!(
            fix("const a = /\\cJ[\\uD83D]/u;"),
            "const a = /\\u{A}[\\uD83D]/u;"
        );
    }

    #[test]
    fn reports_without_a_fix_a_regex_without_the_unicode_flag() {
        for code in ["const foo = /\\u0061/;", "const foo = /\\u{61}/;"] {
            assert_eq!(
                check(code),
                ["Prefer Unicode code point escapes."],
                "{code}"
            );
            assert_eq!(fix(code), code);
        }
    }

    #[test]
    fn accepts_code_points_printable_ascii_and_short_escapes() {
        for code in [
            "const foo = '\\u{A9}';",
            "const foo = '\\x41\\u0041';",
            "const foo = '\\u000A\\u0000';",
            "const foo = String.raw`\\xA9`;",
            "const foo = /\\u{61}/u;",
            "const foo = /a/;",
            "const foo = '\\\\xA9';",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
