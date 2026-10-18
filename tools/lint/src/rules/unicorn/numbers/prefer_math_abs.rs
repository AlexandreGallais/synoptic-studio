//! `unicorn/prefer-math-abs`: an absolute value written by hand, `value < 0 ? -value : value`,
//! and a symmetric range check, `n > MAX || n < -MAX`, are written with `Math.abs()`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-math-abs.md`). Big integers are left:
//! `Math.abs()` refuses them. The rewrite turns `-0` into `0`, so the plugin only suggests it:
//! no fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryExpression, BinaryOperator, Expression, TSType, UnaryOperator};
use oxc_semantic::AstNode;

use super::prefer_unary_minus::is_negative_one;
use crate::expressions::{same_reference, symbol, written_type};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferMathAbs;

/// The rule's message.
const MESSAGE: &str = "Prefer `Math.abs()` to simplify this expression.";

/// Whether an expression is the literal `0`.
fn is_zero(expression: &Expression) -> bool {
    matches!(expression, Expression::NumericLiteral(each) if each.value == 0.0)
}

/// Whether a written type is or holds `bigint`.
fn is_bigint_type(written: &TSType) -> bool {
    match written {
        TSType::TSBigIntKeyword(_) => true,
        TSType::TSLiteralType(literal) => {
            matches!(literal.literal, oxc_ast::ast::TSLiteral::BigIntLiteral(_))
        }
        TSType::TSUnionType(union) => union.types.iter().any(is_bigint_type),
        TSType::TSIntersectionType(intersection) => intersection.types.iter().any(is_bigint_type),
        _ => false,
    }
}

/// Whether an expression is a big integer: a literal, `BigInt(…)`, its negation, a value
/// asserted to `bigint`, or a variable written or initialized so.
fn is_bigint(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::BigIntLiteral(_) => true,
        Expression::CallExpression(call) => {
            call.callee.is_specific_id("BigInt") && call.arguments.len() == 1 && !call.optional
        }
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::UnaryNegation && is_bigint(source, &unary.argument)
        }
        Expression::TSAsExpression(inner) => is_bigint_type(&inner.type_annotation),
        Expression::TSSatisfiesExpression(inner) => is_bigint_type(&inner.type_annotation),
        Expression::TSTypeAssertion(inner) => is_bigint_type(&inner.type_annotation),
        Expression::TSNonNullExpression(inner) => is_bigint(source, &inner.expression),
        Expression::Identifier(name) => symbol(source, name).is_some_and(|found| {
            let nodes = source.semantic.nodes();
            let declaration = source.semantic.scoping().symbol_declaration(found);
            let written = written_type(source, found)
                .is_some_and(|each| is_bigint_type(&each.type_annotation));
            let initialized = match nodes.kind(declaration) {
                AstKind::VariableDeclarator(declarator) => declarator
                    .init
                    .as_ref()
                    .is_some_and(|value| is_bigint(source, value)),
                AstKind::FormalParameter(parameter) => parameter
                    .initializer
                    .as_deref()
                    .is_some_and(|value| is_bigint(source, value)),
                _ => false,
            };
            written || initialized
        }),
        _ => false,
    }
}

/// Whether an expression negates a value: `-value`, `0 - value`, `value * -1` or `-1 * value`.
fn negates(expression: &Expression, value: &Expression) -> bool {
    match expression {
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::UnaryNegation && same_reference(&unary.argument, value)
        }
        Expression::BinaryExpression(binary) => match binary.operator {
            BinaryOperator::Subtraction => {
                is_zero(&binary.left) && same_reference(&binary.right, value)
            }
            BinaryOperator::Multiplication => {
                (same_reference(&binary.left, value) && is_negative_one(&binary.right))
                    || (is_negative_one(&binary.left) && same_reference(&binary.right, value))
            }
            _ => false,
        },
        _ => false,
    }
}

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

/// An ordering comparison, neither side a big integer.
fn ordering<'s, 'a>(
    source: &Source,
    expression: &'s Expression<'a>,
) -> Option<&'s BinaryExpression<'a>> {
    let Expression::BinaryExpression(binary) = expression else {
        return None;
    };
    let orders = matches!(
        binary.operator,
        BinaryOperator::GreaterThan
            | BinaryOperator::GreaterEqualThan
            | BinaryOperator::LessThan
            | BinaryOperator::LessEqualThan
    );
    (orders && !is_bigint(source, &binary.left) && !is_bigint(source, &binary.right))
        .then_some(binary)
}

/// Whether a conditional expression writes an absolute value by hand.
fn is_hand_absolute(
    source: &Source,
    test: &Expression,
    consequent: &Expression,
    alternate: &Expression,
) -> bool {
    let Some(comparison) = ordering(source, test) else {
        return false;
    };
    let (value, operator) = if is_zero(&comparison.right) {
        (&comparison.left, comparison.operator)
    } else if is_zero(&comparison.left) {
        (&comparison.right, inverted(comparison.operator))
    } else {
        return false;
    };
    match operator {
        BinaryOperator::LessThan | BinaryOperator::LessEqualThan => {
            negates(consequent, value) && same_reference(alternate, value)
        }
        _ => same_reference(consequent, value) && negates(alternate, value),
    }
}

/// A comparison read as `value > threshold` or `value < threshold`, by its kind.
fn bound<'s, 'a>(
    source: &Source,
    expression: &'s Expression<'a>,
    upper: bool,
) -> Option<(&'s Expression<'a>, BinaryOperator, &'s Expression<'a>)> {
    let comparison = ordering(source, expression)?;
    let greater = matches!(
        comparison.operator,
        BinaryOperator::GreaterThan | BinaryOperator::GreaterEqualThan
    );
    let read = if greater == upper {
        (&comparison.left, comparison.operator, &comparison.right)
    } else {
        (
            &comparison.right,
            inverted(comparison.operator),
            &comparison.left,
        )
    };
    Some(read)
}

/// Whether two comparisons joined by `||` check a symmetric range, `n > MAX || n < -MAX`.
fn is_symmetric_range(source: &Source, greater: &Expression, less: &Expression) -> bool {
    let (Some((value, operator, threshold)), Some((other, less_operator, negative))) =
        (bound(source, greater, true), bound(source, less, false))
    else {
        return false;
    };
    let matching = matches!(
        (operator, less_operator),
        (BinaryOperator::GreaterThan, BinaryOperator::LessThan)
            | (
                BinaryOperator::GreaterEqualThan,
                BinaryOperator::LessEqualThan
            )
    );
    same_reference(value, other) && matching && negates(negative, threshold)
}

impl Rule for PreferMathAbs {
    fn name(&self) -> &'static str {
        "unicorn/prefer-math-abs"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (found, at) = match node.kind() {
            AstKind::ConditionalExpression(conditional) => (
                is_hand_absolute(
                    source,
                    &conditional.test,
                    &conditional.consequent,
                    &conditional.alternate,
                ),
                conditional.span.start,
            ),
            AstKind::LogicalExpression(logical)
                if logical.operator == oxc_ast::ast::LogicalOperator::Or =>
            {
                (
                    is_symmetric_range(source, &logical.left, &logical.right)
                        || is_symmetric_range(source, &logical.right, &logical.left),
                    logical.span.start,
                )
            }
            _ => return,
        };
        if found {
            findings.push(source.finding(self.name(), at, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferMathAbs;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferMathAbs, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_hand_written_absolute_values_and_ranges() {
        // The documentation's four cases.
        assert_eq!(
            places("const absolute = value < 0 ? -value : value;"),
            [(1, 18)]
        );
        assert_eq!(
            places("const absolute = value <= 0 ? 0 - value : value;"),
            [(1, 18)]
        );
        assert_eq!(
            places("const absolute = value > 0 ? value : -value;"),
            [(1, 18)]
        );
        assert_eq!(
            places("if (number > MAXIMUM || number < -MAXIMUM) {\n}"),
            [(1, 5)]
        );
        // Zero on the left, a product by `-1`, the range written the other way.
        for code in [
            "const a = 0 > v ? v * -1 : v;",
            "const a = 0 <= v ? v : -1 * v;",
            "if (-M >= n || M <= n) {}",
            "if (n < -M || n > M) {}",
        ] {
            assert_eq!(places(code).len(), 1, "{code}");
        }
    }

    #[test]
    fn leaves_other_shapes_and_big_integers() {
        for code in [
            "const a = value < 0 ? value : -value;",
            "const a = value < 1 ? -value : value;",
            "const a = value < 0 ? -other : value;",
            "if (n > M || n < -K) {}",
            "if (n > M || n <= -M) {}",
            "if (n > M && n < -M) {}",
            "const a = 0n > v ? -v : v;",
            "const v = 2n;\nconst a = v < 0 ? -v : v;",
            "function f(v: bigint) { return v < 0 ? -v : v; }",
            "const a = (v as bigint) < 0 ? -v : v;",
            "const a = BigInt(x) < 0 ? -x : x;",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
