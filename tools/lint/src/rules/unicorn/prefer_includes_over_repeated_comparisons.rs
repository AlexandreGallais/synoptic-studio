//! `unicorn/prefer-includes-over-repeated-comparisons`: no chain of three or more `===` joined by
//! `||` that compares one subject with several values, a membership `includes()` says
//! (`eslint-plugin-unicorn` 78.0.0, its default `minimumComparisons` of 3).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryExpression, BinaryOperator, Expression, LogicalExpression, LogicalOperator,
};
use oxc_semantic::AstNode;

use super::syntax::{
    Value, has_optional_chain, has_side_effect, is_reference, same_reference, static_value, unwrap,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferIncludesOverRepeatedComparisons;

/// The fewest comparisons reported.
const MINIMUM_COMPARISONS: usize = 3;

/// The operands of an `||` chain, flattened.
fn operands<'b, 'a>(expression: &'b Expression<'a>, found: &mut Vec<&'b Expression<'a>>) {
    match expression {
        Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::Or => {
            operands(&logical.left, found);
            operands(&logical.right, found);
        }
        _ => found.push(expression),
    }
}

/// Whether a value is `NaN`.
fn is_nan(source: &Source, expression: &Expression) -> bool {
    matches!(static_value(source, expression), Some(Value::Number(number)) if number.is_nan())
}

/// The one reference every comparison reads, `undefined` aside.
fn shared_subject<'b, 'a>(comparisons: &[&'b BinaryExpression<'a>]) -> Option<&'b Expression<'a>> {
    let (first, rest) = comparisons.split_first()?;
    let mut candidates: Vec<&Expression> = [&first.left, &first.right]
        .into_iter()
        .filter(|side| {
            is_reference(side)
                && !matches!(unwrap(side), Expression::Identifier(identifier) if identifier.name == "undefined")
        })
        .collect();
    for comparison in rest {
        candidates.retain(|candidate| {
            same_reference(candidate, &comparison.left)
                || same_reference(candidate, &comparison.right)
        });
    }
    match candidates.as_slice() {
        [only] => Some(only),
        _ => None,
    }
}

/// The value a comparison compares the subject with, when exactly one side is the subject.
fn compared<'b, 'a>(
    comparison: &'b BinaryExpression<'a>,
    subject: &Expression,
) -> Option<&'b Expression<'a>> {
    match (
        same_reference(&comparison.left, subject),
        same_reference(&comparison.right, subject),
    ) {
        (true, false) => Some(&comparison.right),
        (false, true) => Some(&comparison.left),
        _ => None,
    }
}

impl Rule for PreferIncludesOverRepeatedComparisons {
    fn name(&self) -> &'static str {
        "unicorn/prefer-includes-over-repeated-comparisons"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::LogicalExpression(logical) = node.kind() else {
            return;
        };
        let outermost = !matches!(
            source.semantic.nodes().parent_kind(node.id()),
            AstKind::LogicalExpression(LogicalExpression {
                operator: LogicalOperator::Or,
                ..
            })
        );
        if logical.operator != LogicalOperator::Or || !outermost {
            return;
        }
        let mut found = Vec::new();
        operands(&logical.left, &mut found);
        operands(&logical.right, &mut found);
        let comparisons: Option<Vec<&BinaryExpression>> = found
            .iter()
            .map(|operand| match operand {
                Expression::BinaryExpression(binary)
                    if binary.operator == BinaryOperator::StrictEquality =>
                {
                    Some(&**binary)
                }
                _ => None,
            })
            .collect();
        let Some(comparisons) = comparisons else {
            return;
        };
        let optional = comparisons
            .iter()
            .any(|each| has_optional_chain(&each.left) || has_optional_chain(&each.right));
        if comparisons.len() < MINIMUM_COMPARISONS || optional {
            return;
        }
        let Some(subject) = shared_subject(&comparisons) else {
            return;
        };
        let values_plain = comparisons.iter().all(|comparison| {
            compared(comparison, subject)
                .is_some_and(|value| !has_side_effect(value) && !is_nan(source, value))
        });
        if values_plain && !is_nan(source, subject) {
            findings.push(source.finding(
                self.name(),
                logical.span.start,
                "Use `.includes()` instead of repeated equality checks.".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferIncludesOverRepeatedComparisons;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &PreferIncludesOverRepeatedComparisons,
            "src/a.ts",
            code,
        ))
    }

    #[test]
    fn reports_the_documented_repeated_comparisons() {
        let message = ["Use `.includes()` instead of repeated equality checks."];

        assert_eq!(
            check("value === 'a' || value === 'b' || value === 'c';"),
            message
        );
        assert_eq!(
            check("args[0] === '-h' || args[0] === '--help' || args[0] === '--version';"),
            message
        );
        assert_eq!(
            check("'a' === value || value === 'b' || c === value || value === d.e;"),
            message
        );
        assert_eq!(
            check("const four = value === 'a' || value === 'b' || value === 'c' || value === 'd';"),
            message
        );
    }

    #[test]
    fn accepts_two_comparisons_mixed_chains_and_unsafe_values() {
        for code in [
            "value === 'a' || value === 'b';",
            "value !== 'a' && value !== 'b' && value !== 'c';",
            "state.a === undefined || state.b === undefined || state.c === undefined;",
            "value === 'a' || value === 'b' || value == 'c';",
            "value === 'a' || value === 'b' || other;",
            "value === 'a' || value === 'b' || other === 'c';",
            "value?.x === 'a' || value?.x === 'b' || value?.x === 'c';",
            "value === f() || value === 'b' || value === 'c';",
            "value === NaN || value === 'b' || value === 'c';",
            "value === value || value === 'b' || value === 'c';",
            "a === b || a === b || b === a;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
