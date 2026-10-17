//! `unicorn/no-double-comparison`: two comparisons of the same pair of operands joined by `||` or
//! `&&` that say what one comparison says, `x === y || x < y` for `x <= y`, are reported
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-double-comparison.md`). The operands may be
//! flipped; only `===` and `!==` count as equalities; a regular expression operand is left, since
//! `<=` compares its text and `===` its identity. The plugin only suggests the combined
//! comparison, which may differ for mixed types: no fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryExpression, BinaryOperator, Expression, LogicalOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::super::syntax::{
    const_init, has_optional_chain, parenthesized_text, same_reference, unwrap,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDoubleComparison;

/// A comparison the rule reasons about, its operands no optional chain.
fn comparison<'s, 'a>(expression: &'s Expression<'a>) -> Option<&'s BinaryExpression<'a>> {
    let Expression::BinaryExpression(binary) = unwrap(expression) else {
        return None;
    };
    let compares = matches!(
        binary.operator,
        BinaryOperator::LessThan
            | BinaryOperator::LessEqualThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterEqualThan
            | BinaryOperator::StrictEquality
            | BinaryOperator::StrictInequality
    );
    (compares && !has_optional_chain(&binary.left) && !has_optional_chain(&binary.right))
        .then_some(binary)
}

/// The operator reading the comparison with its operands swapped.
fn flipped(operator: BinaryOperator) -> BinaryOperator {
    match operator {
        BinaryOperator::LessThan => BinaryOperator::GreaterThan,
        BinaryOperator::LessEqualThan => BinaryOperator::GreaterEqualThan,
        BinaryOperator::GreaterThan => BinaryOperator::LessThan,
        BinaryOperator::GreaterEqualThan => BinaryOperator::LessEqualThan,
        other => other,
    }
}

/// The one comparison two comparisons joined by a logical operator say, in either order: the
/// plugin's six reductions.
fn reduction(
    logical: LogicalOperator,
    first: BinaryOperator,
    second: BinaryOperator,
) -> Option<&'static str> {
    use BinaryOperator::{
        GreaterEqualThan, GreaterThan, LessEqualThan, LessThan, StrictEquality, StrictInequality,
    };
    let pair = |a, b| (first == a && second == b) || (first == b && second == a);
    match logical {
        LogicalOperator::Or if pair(StrictEquality, LessThan) => Some("<="),
        LogicalOperator::Or if pair(StrictEquality, GreaterThan) => Some(">="),
        LogicalOperator::Or if pair(LessThan, GreaterThan) => Some("!=="),
        LogicalOperator::And if pair(LessEqualThan, GreaterEqualThan) => Some("==="),
        LogicalOperator::And if pair(LessEqualThan, StrictInequality) => Some("<"),
        LogicalOperator::And if pair(GreaterEqualThan, StrictInequality) => Some(">"),
        _ => None,
    }
}

/// Whether an operand is a regular expression: its literal, or a `const` bound to one.
fn is_regexp(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::RegExpLiteral(_) => true,
        node @ Expression::Identifier(_) => const_init(source, node)
            .is_some_and(|init| matches!(unwrap(init), Expression::RegExpLiteral(_))),
        _ => false,
    }
}

impl Rule for NoDoubleComparison {
    fn name(&self) -> &'static str {
        "unicorn/no-double-comparison"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::LogicalExpression(logical) = node.kind() else {
            return;
        };
        if logical.operator == LogicalOperator::Coalesce {
            return;
        }
        let (Some(left), Some(right)) = (comparison(&logical.left), comparison(&logical.right))
        else {
            return;
        };
        let right_operator = if same_reference(&left.left, &right.left)
            && same_reference(&left.right, &right.right)
        {
            right.operator
        } else if same_reference(&left.left, &right.right)
            && same_reference(&left.right, &right.left)
        {
            flipped(right.operator)
        } else {
            return;
        };
        let Some(operator) = reduction(logical.operator, left.operator, right_operator) else {
            return;
        };
        if is_regexp(source, &left.left) || is_regexp(source, &left.right) {
            return;
        }
        let replacement = format!(
            "{} {operator} {}",
            parenthesized_text(source, &left.left),
            parenthesized_text(source, &left.right)
        );
        findings.push(source.finding(
            self.name(),
            logical.span().start,
            format!("These comparisons can be simplified to `{replacement}`."),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoDoubleComparison;
    use crate::testing::{lint, messages};

    /// The messages of the findings.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&NoDoubleComparison, "scripts/a.ts", code))
    }

    #[test]
    fn reports_the_documented_pairs() {
        // The documentation's five examples and its table of reductions.
        for (code, combined) in [
            ("x === y || x < y;", "x <= y"),
            ("x === y || x > y;", "x >= y"),
            ("x <= y && x >= y;", "x === y"),
            ("x <= y && x !== y;", "x < y"),
            ("x === y || y < x;", "x >= y"),
            ("x < y || x > y;", "x !== y"),
            ("x >= y && x !== y;", "x > y"),
        ] {
            assert_eq!(
                found(code),
                [format!(
                    "These comparisons can be simplified to `{combined}`."
                )],
                "{code}"
            );
        }
    }

    #[test]
    fn reports_at_the_logical_expression_with_the_left_operands() {
        let findings = lint(
            &NoDoubleComparison,
            "scripts/a.ts",
            "const a = (o.p as number) === b || o.p < b;",
        );

        assert_eq!((findings[0].line, findings[0].column), (1, 11));
        assert_eq!(
            messages(&findings),
            ["These comparisons can be simplified to `(o.p as number) <= b`."]
        );
    }

    #[test]
    fn leaves_implied_flipped_loose_and_unrelated_pairs() {
        // The documentation's caveats: implied, flipped without equality, loose equality.
        for code in [
            "x <= y || x < y;",
            "x < y || y > x;",
            "x == y || x < y;",
            "x === y || x < z;",
            "x === y && x < y;",
            "x === y ?? x < y;",
            "a?.b === y || a?.b < y;",
            "x === /a/ || x < /a/;",
            "const r = /a/;\nr === y || r < y;",
            "const r = /a/;\ny === r || y < r;",
            "x === y || x < y || z;",
        ] {
            let expected: &[&str] = if code.ends_with("|| z;") {
                &["These comparisons can be simplified to `x <= y`."]
            } else {
                &[]
            };
            assert_eq!(found(code), expected, "{code}");
        }
    }
}
