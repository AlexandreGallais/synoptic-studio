//! `unicorn/prefer-literal-ascii`: a string or an untagged template writes a printable ASCII
//! character itself, never as `\xHH`, `\uHHHH` or `\u{H…}` (`eslint-plugin-unicorn` 78.0.0).
//! Autofixed, as in `ESLint`, but in a directive or where the character would extend a legacy
//! octal escape.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::escapes::{escape_template_raw, is_escaped, numeric_escape};
use super::syntax::text;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferLiteralAscii;

/// Whether a code point is printable ASCII, from the space to the tilde.
const fn is_printable_ascii(value: u32) -> bool {
    value >= 0x20 && value <= 0x7E
}

/// Whether a digit written after a text would join a legacy octal escape it ends with.
fn would_extend_legacy_escape(text: &str, character: char) -> bool {
    if !character.is_ascii_digit() {
        return false;
    }
    let bytes = text.as_bytes();
    let mut digit_start = bytes.len();
    while digit_start > 0
        && (b'0'..=b'7').contains(&bytes[digit_start - 1])
        && bytes.len() - digit_start < 3
    {
        digit_start -= 1;
    }
    if digit_start == 0 || bytes[digit_start - 1] != b'\\' || is_escaped(text, digit_start - 1) {
        return false;
    }
    let digits = &text[digit_start..];
    if digits == "0" {
        return true;
    }
    if character > '7' {
        return false;
    }
    let maximum = if digits.as_bytes()[0] <= b'3' { 3 } else { 2 };
    digits.len() < maximum
}

/// A raw text with its printable ASCII escapes written out, the quote and backslash kept
/// escaped, and whether the result can replace it.
fn replace_escapes(raw: &str, quote: char) -> (String, bool) {
    let mut fixed = String::with_capacity(raw.len());
    let mut can_fix = true;
    let mut at = 0;
    while at < raw.len() {
        let Some(character) = raw[at..].chars().next() else {
            break;
        };
        if character != '\\' {
            fixed.push(character);
            at += character.len_utf8();
            continue;
        }
        if raw[at + 1..].starts_with('\\') {
            fixed.push_str("\\\\");
            at += 2;
            continue;
        }
        let Some(escape) =
            numeric_escape(raw, at).filter(|escape| is_printable_ascii(escape.value))
        else {
            fixed.push('\\');
            at += 1;
            continue;
        };
        let replacement = char::from_u32(escape.value).unwrap_or('?');
        if would_extend_legacy_escape(&fixed, replacement) {
            can_fix = false;
        }
        if replacement == '\\' || replacement == quote {
            fixed.push('\\');
        }
        fixed.push(replacement);
        at = escape.end;
    }
    (fixed, can_fix)
}

impl Rule for PreferLiteralAscii {
    fn name(&self) -> &'static str {
        "unicorn/prefer-literal-ascii"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let nodes = source.semantic.nodes();
        let (span, quote, at, fixable, template) = match node.kind() {
            AstKind::StringLiteral(literal) => {
                let raw = text(source, literal.span);
                let Some(quote) = raw
                    .chars()
                    .next()
                    .filter(|first| matches!(first, '\'' | '"'))
                else {
                    return;
                };
                let directive = matches!(nodes.parent_kind(node.id()), AstKind::Directive(_));
                (literal.span, quote, literal.span.start, !directive, false)
            }
            AstKind::TemplateElement(element) => {
                let parent = nodes.parent_node(node.id());
                if matches!(
                    nodes.parent_kind(parent.id()),
                    AstKind::TaggedTemplateExpression(_)
                ) {
                    return;
                }
                let in_type = matches!(nodes.parent_kind(parent.id()), AstKind::TSLiteralType(_))
                    && element
                        .value
                        .cooked
                        .is_some_and(|cooked| cooked.contains('\\'));
                (
                    element.span,
                    '`',
                    element.span.start.saturating_sub(1),
                    !in_type,
                    true,
                )
            }
            _ => return,
        };
        let original = text(source, span);
        let (fixed, can_fix) = replace_escapes(original, quote);
        if fixed == original {
            return;
        }
        let message = "Prefer literal printable ASCII characters over escape sequences.".to_owned();
        if !(can_fix && fixable) {
            findings.push(source.finding(self.name(), at, message));
            return;
        }
        let replacement = if template {
            escape_template_raw(&fixed)
        } else {
            fixed
        };
        findings.push(source.fixed(
            self.name(),
            at,
            message,
            Fix {
                start: span.start,
                end: span.end,
                text: replacement,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferLiteralAscii;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferLiteralAscii, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferLiteralAscii, "playground/a.ts", code))
    }

    const MESSAGE: &str = "Prefer literal printable ASCII characters over escape sequences.";

    #[test]
    fn fixes_the_documented_escapes() {
        assert_eq!(check("const letter = '\\x41';"), [MESSAGE]);
        assert_eq!(fix("const letter = '\\x41';"), "const letter = 'A';");
        assert_eq!(fix("const word = '\\u0042\\u{43}';"), "const word = 'BC';");
        assert_eq!(
            fix("const template = `\\u0024\\u007Bvalue}`;"),
            "const template = `\\${value}`;"
        );
        assert_eq!(fix("const q = '\\x27';"), "const q = '\\'';");
        assert_eq!(fix("const q = \"\\x27\";"), "const q = \"'\";");
        assert_eq!(fix("const b = '\\x5C';"), "const b = '\\\\';");
        assert_eq!(fix("const t = `\\x60${x}\\x41`;"), "const t = `\\`${x}A`;");
    }

    #[test]
    fn reports_without_a_fix_a_legacy_octal_or_a_directive() {
        let octal = "const a = '\\0\\x31';";
        assert_eq!(check(octal), [MESSAGE]);
        assert_eq!(fix(octal), octal);
        let directive = "'\\x75se strict';";
        assert_eq!(check(directive), [MESSAGE]);
        assert_eq!(fix(directive), directive);
    }

    #[test]
    fn accepts_other_escapes_and_tagged_templates() {
        for code in [
            "const a = 'A';",
            "const a = '\\n\\t\\u00A9\\x7F\\x1F';",
            "const a = '\\\\x41';",
            "const a = String.raw`\\x41`;",
            "const a = /\\x41/u;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
