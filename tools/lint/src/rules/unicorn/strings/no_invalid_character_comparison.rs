//! `unicorn/no-invalid-character-comparison`: one character read from a string, by `charAt()`,
//! `at()` or a static index, compared with `===`, `!==`, `==` or `!=` to a string of two
//! characters or more, a comparison always false or always true, is reported
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-invalid-character-comparison.md`). The
//! receiver must be known to be a string, as `known_values::is_string` reads it; the empty
//! string, out of bounds, passes. Reported only, as the plugin does.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression};
use oxc_semantic::AstNode;

use super::super::known_values::is_string;
use super::super::syntax::{method_call, unwrap};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoInvalidCharacterComparison;

/// The text of a string literal, or of a template without expressions.
fn static_string(expression: &Expression) -> Option<String> {
    match expression {
        Expression::StringLiteral(literal) => Some(literal.value.to_string()),
        Expression::TemplateLiteral(template) if template.expressions.is_empty() => template
            .quasis
            .first()
            .and_then(|quasi| quasi.value.cooked)
            .map(|cooked| cooked.to_string()),
        _ => None,
    }
}

/// Whether an index is written statically: a number, or a string writing a whole number as
/// `String()` does, `'0'` but not `'01'`.
fn is_static_index(expression: &Expression) -> bool {
    match expression {
        Expression::NumericLiteral(_) => true,
        Expression::StringLiteral(literal) => literal
            .value
            .parse::<u64>()
            .is_ok_and(|index| index < (1 << 53) && index.to_string() == literal.value.as_str()),
        _ => false,
    }
}

/// Whether an expression reads one character of a string: `charAt()` or `at()` with at most one
/// argument, or a static index, on a string; an optional read or call is a chain, never one of
/// these.
fn is_single_character(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::CallExpression(call) => method_call(call).is_some_and(|method| {
            matches!(method.name, "charAt" | "at")
                && method
                    .arguments()
                    .is_some_and(|arguments| arguments.len() <= 1)
                && is_string(source, method.object)
        }),
        Expression::ComputedMemberExpression(member) => {
            is_static_index(&member.expression) && is_string(source, &member.object)
        }
        _ => false,
    }
}

impl Rule for NoInvalidCharacterComparison {
    fn name(&self) -> &'static str {
        "unicorn/no-invalid-character-comparison"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::BinaryExpression(binary) = node.kind() else {
            return;
        };
        let negated = match binary.operator {
            BinaryOperator::StrictEquality | BinaryOperator::Equality => false,
            BinaryOperator::StrictInequality | BinaryOperator::Inequality => true,
            _ => return,
        };
        for (access, other) in [(&binary.left, &binary.right), (&binary.right, &binary.left)] {
            let Some(string) = static_string(other) else {
                continue;
            };
            if string.encode_utf16().count() > 1 && is_single_character(source, unwrap(access)) {
                findings.push(source.finding(
                    self.name(),
                    binary.span.start,
                    format!(
                        "A single character can never equal the multi-character string \
                         `{string}`, so this comparison is always {negated}."
                    ),
                ));
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidCharacterComparison;
    use crate::testing::{lint, messages};

    /// The messages of the findings.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&NoInvalidCharacterComparison, "scripts/a.ts", code))
    }

    #[test]
    fn reports_the_documented_comparisons() {
        // The documentation's four cases.
        for (code, string) in [
            ("'unicorn'.charAt(0) === 'ab';", "ab"),
            ("'unicorn'.charAt(3) === '/n';", "/n"),
            ("'unicorn'.at(0) === 'ab';", "ab"),
            ("'unicorn'[0] === 'ab';", "ab"),
        ] {
            assert_eq!(
                found(code),
                [format!(
                    "A single character can never equal the multi-character string `{string}`, \
                     so this comparison is always false."
                )],
                "{code}"
            );
        }
    }

    #[test]
    fn reports_the_other_side_negations_and_known_strings() {
        assert_eq!(
            found("'ab' !== 'unicorn'['1'];"),
            [
                "A single character can never equal the multi-character string `ab`, so this \
              comparison is always true."
            ]
        );
        for code in [
            "function f(text: string) { return text.charAt(i) == `ab`; }",
            "const word = 'unicorn';\nword.at(-1) != 'ab';",
            "String(x)[0] === 'ab';",
            "'a'.charAt(0) === '😀';",
        ] {
            assert_eq!(found(code).len(), 1, "{code}");
        }
    }

    #[test]
    fn leaves_single_characters_empty_strings_and_unknown_receivers() {
        // The documentation's valid cases, then receivers not known to be strings, dynamic or
        // non-canonical indices, optional reads and other operators.
        for code in [
            "'unicorn'.charAt(0) === 'u';",
            "'unicorn'.charAt(3) === '\\n';",
            "'unicorn'.at(0) === 'a';",
            "'unicorn'[0] === 'a';",
            "'unicorn'.charAt(100) === '';",
            "array.at(0) === 'ab';",
            "value[0] === 'ab';",
            "'unicorn'[i] === 'ab';",
            "'unicorn'['01'] === 'ab';",
            "'unicorn'?.[0] === 'ab';",
            "'unicorn'.at?.(0) === 'ab';",
            "'unicorn'.charAt(0, 1) === 'ab';",
            "'unicorn'.charAt(0) < 'ab';",
            "'unicorn'.charAt(0) === `a${b}`;",
        ] {
            assert_eq!(found(code), Vec::<String>::new(), "{code}");
        }
    }
}
