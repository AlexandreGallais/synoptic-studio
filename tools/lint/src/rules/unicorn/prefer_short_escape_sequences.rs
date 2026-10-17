//! `unicorn/prefer-short-escape-sequences`: a string or an untagged template writes `\n`,
//! `\t`, `\r`, `\b`, `\f`, `\v` and `\0` rather than their `\uXXXX` escapes
//! (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`; `\u0000` before a digit stays.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::syntax::text;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferShortEscapeSequences;

/// The short escape of a code point written with four hexadecimal digits, upper case.
fn short_escape(code_point: &str, next: Option<char>) -> Option<&'static str> {
    match code_point {
        "0000" if next.is_some_and(|character| character.is_ascii_digit()) => None,
        "0000" => Some("\\0"),
        "000B" => Some("\\v"),
        "0008" => Some("\\b"),
        "0009" => Some("\\t"),
        "000A" => Some("\\n"),
        "000C" => Some("\\f"),
        "000D" => Some("\\r"),
        _ => None,
    }
}

/// A raw text with its `\uXXXX` escapes of a short form shortened, when one is.
fn shortened(content: &str) -> Option<String> {
    let mut fixed = String::with_capacity(content.len());
    let mut at = 0;
    let mut changed = false;
    while at < content.len() {
        let rest = &content[at..];
        let backslashes = rest.bytes().take_while(|byte| *byte == b'\\').count();
        if backslashes == 0 {
            let character = rest.chars().next().unwrap_or_default();
            fixed.push(character);
            at += character.len_utf8();
            continue;
        }
        let after = &rest[backslashes..];
        let digits = after
            .strip_prefix('u')
            .and_then(|tail| tail.get(..4))
            .filter(|digits| digits.bytes().all(|byte| byte.is_ascii_hexdigit()));
        let Some(digits) = digits else {
            fixed.push_str(&rest[..backslashes]);
            at += backslashes;
            continue;
        };
        let length = backslashes + 5;
        let next = content[at + length..].chars().next();
        let replacement = (backslashes % 2 == 1)
            .then(|| short_escape(&digits.to_ascii_uppercase(), next))
            .flatten();
        match replacement {
            Some(short) => {
                fixed.push_str(&rest[..backslashes - 1]);
                fixed.push_str(short);
                changed = true;
            }
            None => fixed.push_str(&rest[..length]),
        }
        at += length;
    }
    changed.then_some(fixed)
}

impl Rule for PreferShortEscapeSequences {
    fn name(&self) -> &'static str {
        "unicorn/prefer-short-escape-sequences"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let nodes = source.semantic.nodes();
        let (start, end, at) = match node.kind() {
            AstKind::StringLiteral(literal) => {
                if !text(source, literal.span).starts_with(['\'', '"']) {
                    return;
                }
                (
                    literal.span.start + 1,
                    literal.span.end - 1,
                    literal.span.start,
                )
            }
            AstKind::TemplateElement(element) => {
                let parent = nodes.parent_id(node.id());
                if matches!(
                    nodes.parent_kind(parent),
                    AstKind::TaggedTemplateExpression(_)
                ) {
                    return;
                }
                (
                    element.span.start,
                    element.span.end,
                    element.span.start.saturating_sub(1),
                )
            }
            _ => return,
        };
        let content = text(source, oxc_span::Span::new(start, end));
        if let Some(fixed) = shortened(content) {
            findings.push(source.fixed(
                self.name(),
                at,
                "Prefer shorter alternatives to Unicode escape sequences.".to_owned(),
                Fix {
                    start,
                    end,
                    text: fixed,
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferShortEscapeSequences;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferShortEscapeSequences, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferShortEscapeSequences, "playground/a.ts", code),
        )
    }

    #[test]
    fn fixes_the_documented_escapes() {
        let code = "const text = \"Line one\\u000ALine two\";";
        assert_eq!(
            check(code),
            ["Prefer shorter alternatives to Unicode escape sequences."]
        );
        assert_eq!(fix(code), "const text = \"Line one\\nLine two\";");
        assert_eq!(
            fix("const template = `Line one\\u000ALine two`;"),
            "const template = `Line one\\nLine two`;"
        );
        assert_eq!(
            fix("const a = '\\u0009\\u000b\\u0000x\\u0008\\u000C\\u000D';"),
            "const a = '\\t\\v\\0x\\b\\f\\r';"
        );
        assert_eq!(fix("const a = `${b}\\u000a`;"), "const a = `${b}\\n`;");
    }

    #[test]
    fn accepts_other_escapes_and_escaped_backslashes() {
        for code in [
            "const a = '\\n';",
            "const a = '\\\\u000A';",
            "const a = '\\u00000';",
            "const a = '\\u0041';",
            "const a = '\\u{A}';",
            "const a = '\\x0A';",
            "const a = String.raw`\\u000A`;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
