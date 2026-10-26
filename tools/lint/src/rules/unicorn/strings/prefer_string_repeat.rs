//! `unicorn/prefer-string-repeat`: a string of three or more of the same whitespace character
//! is written `' '.repeat(n)`, its count readable (`eslint-plugin-unicorn` 78.0.0, its default
//! `minimumRepetitions` of 3). Autofixed, as in `ESLint`; a string the syntax requires, a key,
//! a module name or a literal type, stays.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::method_call;
use crate::source::Source;

/// The rule.
pub struct PreferStringRepeat;

/// The fewest repetitions reported, the plugin's default.
const MINIMUM_REPETITIONS: usize = 3;

/// Whether a character is whitespace as JavaScript's `\s` reads it.
fn is_js_whitespace(character: char) -> bool {
    matches!(
        character,
        ' ' | '\t' | '\n' | '\u{B}' | '\u{C}' | '\r' | '\u{A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
                | '\u{FEFF}'
    )
}

/// The whitespace character a string repeats, and how often, when it repeats one enough.
fn repeated_whitespace(value: &str) -> Option<(char, usize)> {
    let mut characters = value.chars();
    let first = characters.next()?;
    let count = value.chars().count();
    (count >= MINIMUM_REPETITIONS
        && is_js_whitespace(first)
        && value.chars().all(|each| each == first))
    .then_some((first, count))
}

/// A whitespace character as a readable quoted string.
fn quoted(character: char) -> String {
    match character {
        ' ' => "' '".to_owned(),
        '\t' => "'\\t'".to_owned(),
        '\n' => "'\\n'".to_owned(),
        '\r' => "'\\r'".to_owned(),
        '\u{C}' => "'\\f'".to_owned(),
        other => format!("'\\u{{{:X}}}'", u32::from(other)),
    }
}

/// Whether the syntax requires a string literal where a node stands: a directive, a key that is
/// not computed, a module name, an import attribute, an enum member, a module declaration's
/// name, a literal type, or an inline snapshot of `expect()`.
fn is_required(source: &Source, id: NodeId, span: Span) -> bool {
    let nodes = source.semantic.nodes();
    match nodes.parent_kind(id) {
        AstKind::Directive(_)
        | AstKind::ImportDeclaration(_)
        | AstKind::ExportFromDeclaration(_)
        | AstKind::ExportAllDeclaration(_)
        | AstKind::ImportAttribute(_)
        | AstKind::ImportSpecifier(_)
        | AstKind::ExportSpecifier(_)
        | AstKind::TSEnumMember(_)
        | AstKind::TSExternalModuleReference(_)
        | AstKind::TSLiteralType(_)
        | AstKind::TSImportType(_)
        | AstKind::TSPropertySignature(_)
        | AstKind::TSMethodSignature(_) => true,
        AstKind::ObjectProperty(property) => !property.computed && property.key.span() == span,
        AstKind::PropertyDefinition(property) => !property.computed && property.key.span() == span,
        AstKind::MethodDefinition(method) => !method.computed && method.key.span() == span,
        AstKind::AccessorProperty(property) => !property.computed && property.key.span() == span,
        AstKind::CallExpression(call) => method_call(call).is_some_and(|method| {
            method.name == "toMatchInlineSnapshot"
                && method.is_plain()
                && call.arguments.len() == 1
                && matches!(method.object, Expression::CallExpression(expect)
                    if !expect.optional
                        && expect.arguments.len() == 1
                        && matches!(&expect.callee, Expression::Identifier(callee) if callee.name == "expect"))
        }),
        _ => false,
    }
}

/// Whether the text at an offset is a letter, which a keyword next to the literal would end or
/// start with.
fn is_letter_at(source: &Source, offset: Option<usize>) -> bool {
    offset
        .and_then(|at| source.text.as_bytes().get(at))
        .is_some_and(u8::is_ascii_alphabetic)
}

impl Rule for PreferStringRepeat {
    fn name(&self) -> &'static str {
        "unicorn/prefer-string-repeat"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (span, value) = match node.kind() {
            AstKind::StringLiteral(literal) => (literal.span, literal.value.as_str()),
            AstKind::TemplateLiteral(template) => {
                if !template.expressions.is_empty()
                    || matches!(
                        source.semantic.nodes().parent_kind(node.id()),
                        AstKind::TaggedTemplateExpression(_)
                    )
                {
                    return;
                }
                let Some(cooked) = template.quasis.first().and_then(|quasi| quasi.value.cooked)
                else {
                    return;
                };
                (template.span, cooked.as_str())
            }
            _ => return,
        };
        let Some((character, count)) = value.and_then(repeated_whitespace) else {
            return;
        };
        if is_required(source, node.id(), span) {
            return;
        }
        let before = if is_letter_at(source, (span.start as usize).checked_sub(1)) {
            " "
        } else {
            ""
        };
        let after = if is_letter_at(source, Some(span.end as usize)) {
            " "
        } else {
            ""
        };
        findings.push(source.fixed(
            self.name(),
            span.start,
            "Use `String#repeat()` for repeated whitespace.".to_owned(),
            Fix {
                start: span.start,
                end: span.end,
                text: format!("{before}{}.repeat({count}){after}", quoted(character)),
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferStringRepeat;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferStringRepeat, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferStringRepeat, "playground/a.ts", code))
    }

    #[test]
    fn fixes_the_documented_repeated_whitespace() {
        assert_eq!(
            check("const indentation = '    ';"),
            ["Use `String#repeat()` for repeated whitespace."]
        );
        assert_eq!(
            fix("const indentation = '    ';"),
            "const indentation = ' '.repeat(4);"
        );
        assert_eq!(
            fix("const padding = '\\t\\t\\t';"),
            "const padding = '\\t'.repeat(3);"
        );
        assert_eq!(
            fix("const spaces = '\\u2003\\u2003\\u2003';"),
            "const spaces = '\\u{2003}'.repeat(3);"
        );
        assert_eq!(fix("const a = `   `;"), "const a = ' '.repeat(3);");
        assert_eq!(
            fix("function f() { return'   '; }"),
            "function f() { return ' '.repeat(3); }"
        );
    }

    #[test]
    fn accepts_the_documented_and_required_strings() {
        for code in [
            "const letters = 'aaa';",
            "const words = 'unicorn unicorn unicorn';",
            "const mixedWhitespace = ' \\t ';",
            "const indentation = '  ';",
            "const a = String.raw`   `;",
            "const a = `   ${b}`;",
            "const o = { '   ': 1 };",
            "type T = '   ';",
            "import a from '   ';",
            "expect(x).toMatchInlineSnapshot('   ');",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
