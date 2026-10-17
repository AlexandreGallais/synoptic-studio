//! `unicorn/no-subtraction-comparison`: a difference compared with `0`, `a - b > 0`, compares
//! the values themselves, `a > b` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-subtraction-comparison.md`). The rewrite holds for numbers only: `>` and `<`
//! are autofixed when both sides are known numbers, the other operators when both are finite
//! number literals; the rest is only suggested, so reported without fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, TSType, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{const_value, has_comment_in, symbol, text, unwrapped, written_type};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoSubtractionComparison;

/// The rule's message.
const MESSAGE: &str =
    "Prefer comparing the values directly over comparing the difference with `0`.";

/// The comparison read the other way round.
fn inverted(operator: BinaryOperator) -> BinaryOperator {
    match operator {
        BinaryOperator::GreaterThan => BinaryOperator::LessThan,
        BinaryOperator::GreaterEqualThan => BinaryOperator::LessEqualThan,
        BinaryOperator::LessThan => BinaryOperator::GreaterThan,
        BinaryOperator::LessEqualThan => BinaryOperator::GreaterEqualThan,
        other => other,
    }
}

/// Whether an expression is the literal `0`.
fn is_zero(expression: &Expression) -> bool {
    matches!(expression, Expression::NumericLiteral(each) if each.value == 0.0)
}

/// The finite number an expression writes, signed or not.
fn finite_number(expression: &Expression) -> Option<f64> {
    match unwrapped(expression) {
        Expression::NumericLiteral(each) => Some(each.value).filter(|value| value.is_finite()),
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::UnaryNegation => {
            finite_number(&unary.argument).map(|value| -value)
        }
        _ => None,
    }
}

/// Whether an expression is known to be a number: a number literal, arithmetic, a value written
/// `number`, or a `const` holding a number.
fn is_number(source: &Source, expression: &Expression, depth: usize) -> bool {
    if depth > 8 {
        return false;
    }
    match unwrapped(expression) {
        Expression::NumericLiteral(_) => true,
        Expression::UnaryExpression(unary) => {
            matches!(
                unary.operator,
                UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus
            ) && is_number(source, &unary.argument, depth + 1)
        }
        Expression::BinaryExpression(binary) => {
            matches!(
                binary.operator,
                BinaryOperator::Subtraction
                    | BinaryOperator::Multiplication
                    | BinaryOperator::Division
                    | BinaryOperator::Remainder
                    | BinaryOperator::Exponential
                    | BinaryOperator::Addition
            ) && is_number(source, &binary.left, depth + 1)
                && is_number(source, &binary.right, depth + 1)
        }
        Expression::Identifier(name) => {
            symbol(source, name).is_some_and(|found| match written_type(source, found) {
                Some(written) => matches!(written.type_annotation, TSType::TSNumberKeyword(_)),
                None => const_value(source, found)
                    .is_some_and(|value| is_number(source, value, depth + 1)),
            })
        }
        _ => false,
    }
}

impl Rule for NoSubtractionComparison {
    fn name(&self) -> &'static str {
        "unicorn/no-subtraction-comparison"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::BinaryExpression(comparison) = node.kind() else {
            return;
        };
        let compares = comparison.operator.is_compare() || comparison.operator.is_equality();
        if !compares {
            return;
        }
        let subtraction = |each: &Expression| matches!(each, Expression::BinaryExpression(inner) if inner.operator == BinaryOperator::Subtraction);
        let (difference, operator) = if is_zero(&comparison.right) && subtraction(&comparison.left)
        {
            (&comparison.left, comparison.operator)
        } else if is_zero(&comparison.left) && subtraction(&comparison.right) {
            (&comparison.right, inverted(comparison.operator))
        } else {
            return;
        };
        let Expression::BinaryExpression(difference) = difference else {
            return;
        };
        let strict = matches!(
            operator,
            BinaryOperator::GreaterThan | BinaryOperator::LessThan
        );
        let fixable = if strict {
            is_number(source, &difference.left, 0) && is_number(source, &difference.right, 0)
        } else {
            finite_number(&difference.left).is_some() && finite_number(&difference.right).is_some()
        };
        if !fixable || has_comment_in(source, comparison.span) {
            findings.push(source.finding(self.name(), comparison.span.start, MESSAGE.to_owned()));
            return;
        }
        let fix = Fix {
            start: comparison.span.start,
            end: comparison.span.end,
            text: format!(
                "{} {} {}",
                text(source, difference.left.span()),
                operator.as_str(),
                text(source, difference.right.span())
            ),
        };
        findings.push(source.fixed(self.name(), comparison.span.start, MESSAGE.to_owned(), fix));
    }
}

#[cfg(test)]
mod tests {
    use super::NoSubtractionComparison;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoSubtractionComparison, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn compares_the_values_themselves() {
        // The documentation's four cases, reported without fix: their operands' types are unknown.
        let code = "if (a - b > 0) {}\nif (a - b <= 0) {}\nif (0 < a - b) {}\nif (a - b === 0) {}";
        assert_eq!(
            check(code),
            (vec![(1, 5), (2, 5), (3, 5), (4, 5)], code.to_owned())
        );
        // Known numbers are fixed; non-strict operators need literals.
        assert_eq!(
            check("function f(a: number, b: number) { return a - b > 0; }").1,
            "function f(a: number, b: number) { return a > b; }"
        );
        assert_eq!(check("const c = 0 < 2 * x - -1;").0, [(1, 11)]);
        assert_eq!(check("const c = 3 - 2 >= 0;").1, "const c = 3 >= 2;");
        assert_eq!(
            check("const n = 1;\nconst c = 0 > n - 2;").1,
            "const n = 1;\nconst c = n < 2;"
        );
    }

    #[test]
    fn leaves_other_comparisons() {
        for code in [
            "if (a - b > 1) {}",
            "if (a + b > 0) {}",
            "if (a > b) {}",
            "if (a - b) {}",
            "const c = a - b - 0;",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
