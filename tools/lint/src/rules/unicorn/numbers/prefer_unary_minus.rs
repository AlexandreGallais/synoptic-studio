//! `unicorn/prefer-unary-minus`: `x * -1`, `-1 * x` and `x / -1` negate `x`, written `-x`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-unary-minus.md`); `-1 / x` is left.
//! Autofixed unless a comment is inside.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{has_comment_in, needs_parentheses_as_operand, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferUnaryMinus;

/// Whether an expression is `-1`.
#[must_use]
pub fn is_negative_one(expression: &Expression) -> bool {
    matches!(expression, Expression::UnaryExpression(unary)
        if unary.operator == UnaryOperator::UnaryNegation
            && matches!(&unary.argument, Expression::NumericLiteral(one) if one.value.to_bits() == 1.0_f64.to_bits()))
}

impl Rule for PreferUnaryMinus {
    fn name(&self) -> &'static str {
        "unicorn/prefer-unary-minus"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::BinaryExpression(binary) = node.kind() else {
            return;
        };
        let (left, right) = (
            is_negative_one(&binary.left),
            is_negative_one(&binary.right),
        );
        let operand = match binary.operator {
            BinaryOperator::Multiplication | BinaryOperator::Division if right && !left => {
                &binary.left
            }
            BinaryOperator::Multiplication if left && !right => &binary.right,
            _ => return,
        };
        let operation = if binary.operator == BinaryOperator::Multiplication {
            "multiplying"
        } else {
            "dividing"
        };
        let message = format!("Prefer the unary minus operator over {operation} by `-1`.");
        if has_comment_in(source, binary.span) {
            findings.push(source.finding(self.name(), binary.span.start, message));
            return;
        }
        let inner = text(source, operand.span());
        let negative = matches!(operand, Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::UnaryNegation);
        let mut replacement = if needs_parentheses_as_operand(operand) || negative {
            format!("-({inner})")
        } else {
            format!("-{inner}")
        };
        if source.text[..binary.span.start as usize].ends_with('-') {
            replacement = format!(" {replacement}");
        }
        let fix = Fix {
            start: binary.span.start,
            end: binary.span.end,
            text: replacement,
        };
        findings.push(source.fixed(self.name(), binary.span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferUnaryMinus;
    use crate::testing::{fixed, lint, messages};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferUnaryMinus, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn negates_with_the_unary_minus() {
        // The documentation's three cases.
        assert_eq!(
            check(
                "const inverted = value * -1;\nconst flipped = -1 * someNumber;\nconst divided = value / -1;"
            ),
            (
                vec![(1, 18), (2, 17), (3, 17)],
                "const inverted = -value;\nconst flipped = -someNumber;\nconst divided = -value;"
                    .to_owned()
            )
        );
        assert_eq!(
            messages(&lint(&PreferUnaryMinus, "scripts/a.ts", "a / -1;")),
            ["Prefer the unary minus operator over dividing by `-1`."]
        );
        // Parentheses where the operand needs them, and a space after a minus.
        assert_eq!(check("const a = (b + c) * -1;").1, "const a = -(b + c);");
        assert_eq!(check("const a = -b * -1;").1, "const a = -(-b);");
    }

    #[test]
    fn leaves_other_operations() {
        for code in [
            "const a = -1 / x;",
            "const a = -1 * -1;",
            "const a = x * 1;",
            "const a = x * -2;",
            "const a = x - -1;",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let commented = "const a = x /* why */ * -1;";
        assert_eq!(check(commented), (vec![(1, 11)], commented.to_owned()));
    }
}
