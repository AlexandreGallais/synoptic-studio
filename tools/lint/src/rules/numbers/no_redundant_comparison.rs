//! `unicorn/no-redundant-comparison`: in an `&&` chain holding an equality or a disequality of
//! two references, a comparison already implied by another one through it — the same bound
//! read through `a === b`, a stronger numeric bound, or `a === 'x' && a !== b` implying
//! `b !== 'x'` — is reported (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-redundant-comparison.md`). Autofixed by removing it when it runs no code: an
//! ordering comparison may convert its operands, so it is only suggested.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, LogicalOperator};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{
    has_comment_in, is_parenthesized, may_run_code, offset, parent, same_reference, unwrapped,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoRedundantComparison;

/// The rule's message.
const MESSAGE: &str =
    "This comparison is redundant because it is already implied by the other conditions.";

/// A comparison of a subject with a value, `subject operator value`.
#[derive(Clone, Copy)]
struct Predicate<'s, 'a> {
    operator: BinaryOperator,
    value: &'s Expression<'a>,
    node: &'s Expression<'a>,
}

/// A comparison of the chain: its operator, sides and operand.
#[derive(Clone, Copy)]
struct Comparison<'s, 'a> {
    operator: BinaryOperator,
    left: &'s Expression<'a>,
    right: &'s Expression<'a>,
    node: &'s Expression<'a>,
}

/// Whether an expression is a reference: a name, `this`, `super` or a member read.
fn is_reference(expression: &Expression) -> bool {
    matches!(
        unwrapped(expression),
        Expression::Identifier(_)
            | Expression::ThisExpression(_)
            | Expression::Super(_)
            | Expression::StaticMemberExpression(_)
            | Expression::ComputedMemberExpression(_)
            | Expression::PrivateFieldExpression(_)
    )
}

/// The operator reading the comparison the other way round.
fn flipped(operator: BinaryOperator) -> BinaryOperator {
    match operator {
        BinaryOperator::LessThan => BinaryOperator::GreaterThan,
        BinaryOperator::LessEqualThan => BinaryOperator::GreaterEqualThan,
        BinaryOperator::GreaterThan => BinaryOperator::LessThan,
        BinaryOperator::GreaterEqualThan => BinaryOperator::LessEqualThan,
        other => other,
    }
}

/// Whether an operator orders.
fn is_ordering(operator: BinaryOperator) -> bool {
    matches!(
        operator,
        BinaryOperator::LessThan
            | BinaryOperator::LessEqualThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterEqualThan
    )
}

/// The number a literal writes, signed or not.
fn number(expression: &Expression) -> Option<f64> {
    match unwrapped(expression) {
        Expression::NumericLiteral(each) => Some(each.value),
        Expression::UnaryExpression(unary)
            if unary.operator == oxc_ast::ast::UnaryOperator::UnaryNegation =>
        {
            number(&unary.argument).map(|value| -value)
        }
        _ => None,
    }
}

/// Whether one numeric bound implies another, `> 2` implying `> 0`.
fn implies(first: BinaryOperator, a: f64, second: BinaryOperator, b: f64) -> bool {
    let lower = |operator| {
        matches!(
            operator,
            BinaryOperator::GreaterThan | BinaryOperator::GreaterEqualThan
        )
    };
    let inclusive = |operator| {
        matches!(
            operator,
            BinaryOperator::GreaterEqualThan | BinaryOperator::LessEqualThan
        )
    };
    if lower(first) != lower(second) {
        return false;
    }
    let stronger = if lower(first) { a > b } else { a < b };
    let weaker = if lower(first) { a < b } else { a > b };
    if stronger {
        return true;
    }
    if weaker {
        return false;
    }
    !inclusive(first) || inclusive(second)
}

/// Whether one predicate on a subject implies another.
fn entails(first: &Predicate, second: &Predicate) -> bool {
    if first.operator == second.operator && same_reference(first.value, second.value) {
        return true;
    }
    if !is_ordering(first.operator) || !is_ordering(second.operator) {
        return false;
    }
    match (number(first.value), number(second.value)) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => {
            implies(first.operator, a, second.operator, b)
        }
        _ => false,
    }
}

/// The groups of references the equalities make equal.
fn classes<'s, 'a>(
    equalities: &[(&'s Expression<'a>, &'s Expression<'a>)],
) -> Vec<Vec<&'s Expression<'a>>> {
    let mut found: Vec<Vec<&Expression>> = Vec::new();
    for (left, right) in equalities {
        let find = |found: &Vec<Vec<&Expression>>, each: &Expression| {
            found
                .iter()
                .position(|class| class.iter().any(|member| same_reference(member, each)))
        };
        match (find(&found, left), find(&found, right)) {
            (Some(a), Some(b)) if a != b => {
                let merged = found.remove(a.max(b));
                found[a.min(b)].extend(merged);
            }
            (Some(_), Some(_)) => {}
            (Some(a), None) => found[a].push(right),
            (None, Some(b)) => found[b].push(left),
            (None, None) => found.push(vec![left, right]),
        }
    }
    found
}

/// A chain's operands sorted: equalities and disequalities of two references, and comparisons
/// of a reference with something else.
struct Sorted<'s, 'a> {
    equalities: Vec<(&'s Expression<'a>, &'s Expression<'a>)>,
    disequalities: Vec<(&'s Expression<'a>, &'s Expression<'a>)>,
    comparisons: Vec<Comparison<'s, 'a>>,
}

/// The operands of a chain, sorted.
fn sorted<'s, 'a>(operands: &[&'s Expression<'a>]) -> Sorted<'s, 'a> {
    let mut found = Sorted {
        equalities: Vec::new(),
        disequalities: Vec::new(),
        comparisons: Vec::new(),
    };
    for operand in operands {
        let Expression::BinaryExpression(binary) = unwrapped(operand) else {
            continue;
        };
        let equality = matches!(
            binary.operator,
            BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
        );
        let chained = matches!(binary.left, Expression::ChainExpression(_))
            || matches!(binary.right, Expression::ChainExpression(_));
        if !(is_ordering(binary.operator) || equality) || chained {
            continue;
        }
        let (left, right) = (is_reference(&binary.left), is_reference(&binary.right));
        if left && right && equality {
            if same_reference(&binary.left, &binary.right) {
                continue;
            }
            let pair = (&binary.left, &binary.right);
            if binary.operator == BinaryOperator::StrictEquality {
                found.equalities.push(pair);
            } else {
                found.disequalities.push(pair);
            }
        } else if left || right {
            found.comparisons.push(Comparison {
                operator: binary.operator,
                left: &binary.left,
                right: &binary.right,
                node: operand,
            });
        }
    }
    found
}

/// A comparison read as a predicate on a subject, when one of its sides is the subject.
fn about<'s, 'a>(
    comparison: &Comparison<'s, 'a>,
    subject: &dyn Fn(&Expression) -> bool,
) -> Option<Predicate<'s, 'a>> {
    match (subject(comparison.left), subject(comparison.right)) {
        (true, false) => Some(Predicate {
            operator: comparison.operator,
            value: comparison.right,
            node: comparison.node,
        }),
        (false, true) => Some(Predicate {
            operator: flipped(comparison.operator),
            value: comparison.left,
            node: comparison.node,
        }),
        _ => None,
    }
}

/// The comparison another one implies through an equality of references.
fn through_equalities<'s, 'a>(found: &Sorted<'s, 'a>) -> Option<&'s Expression<'a>> {
    for class in classes(&found.equalities) {
        let inside = |each: &Expression| class.iter().any(|member| same_reference(member, each));
        let predicates: Vec<Predicate> = found
            .comparisons
            .iter()
            .filter_map(|each| about(each, &inside))
            .collect();
        for (index, first) in predicates.iter().enumerate() {
            for second in &predicates[index + 1..] {
                if entails(first, second) {
                    return Some(second.node);
                }
                if entails(second, first) {
                    return Some(first.node);
                }
            }
        }
    }
    None
}

/// The disequality a comparison implies through a disequality of references:
/// `a === 'x' && a !== b` implies `b !== 'x'`.
fn through_disequalities<'s, 'a>(found: &Sorted<'s, 'a>) -> Option<&'s Expression<'a>> {
    for (left, right) in &found.disequalities {
        for (subject, other) in [(*left, *right), (*right, *left)] {
            let is_subject = |each: &Expression| same_reference(each, subject);
            let is_other = |each: &Expression| same_reference(each, other);
            let Some(value) = found
                .comparisons
                .iter()
                .filter_map(|each| about(each, &is_subject))
                .find(|predicate| predicate.operator == BinaryOperator::StrictEquality)
                .map(|predicate| predicate.value)
            else {
                continue;
            };
            let implied = found.comparisons.iter().find(|each| {
                about(each, &is_other).is_some_and(|predicate| {
                    predicate.operator == BinaryOperator::StrictInequality
                        && same_reference(predicate.value, value)
                })
            });
            if let Some(implied) = implied {
                return Some(implied.node);
            }
        }
    }
    None
}

/// The redundant comparison of a chain's operands.
fn redundant<'s, 'a>(operands: &[&'s Expression<'a>]) -> Option<&'s Expression<'a>> {
    let found = sorted(operands);
    if found.comparisons.is_empty()
        || (found.equalities.is_empty() && found.disequalities.is_empty())
    {
        return None;
    }
    through_equalities(&found).or_else(|| through_disequalities(&found))
}

/// The operands of an `&&` chain.
fn operands<'s, 'a>(expression: &'s Expression<'a>, into: &mut Vec<&'s Expression<'a>>) {
    match expression {
        Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::And => {
            operands(&logical.left, into);
            operands(&logical.right, into);
        }
        other => into.push(other),
    }
}

/// A range with the parentheses written around it.
fn with_parentheses(source: &Source, span: Span) -> Span {
    if is_parenthesized(source, span) {
        let start = offset(source.text[..span.start as usize].trim_end().len()) - 1;
        let after = &source.text[span.end as usize..];
        let end = span.end + offset(after.len() - after.trim_start().len()) + 1;
        Span::new(start, end)
    } else {
        span
    }
}

impl Rule for NoRedundantComparison {
    fn name(&self) -> &'static str {
        "unicorn/no-redundant-comparison"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::LogicalExpression(logical) = node.kind() else {
            return;
        };
        let outermost = !matches!(parent(source, node.id()), AstKind::LogicalExpression(outer) if outer.operator == logical.operator);
        if logical.operator != LogicalOperator::And || !outermost {
            return;
        }
        let mut list = Vec::new();
        operands(&logical.left, &mut list);
        operands(&logical.right, &mut list);
        let Some(found) = redundant(&list) else {
            return;
        };
        let at = found.span().start;
        let holder = source
            .semantic
            .nodes()
            .iter()
            .find_map(|each| match each.kind() {
                AstKind::LogicalExpression(chain)
                    if chain.left.span() == found.span() || chain.right.span() == found.span() =>
                {
                    Some(chain)
                }
                _ => None,
            });
        let Some(holder) = holder else {
            return;
        };
        let (left, right) = (
            with_parentheses(source, holder.left.span()),
            with_parentheses(source, holder.right.span()),
        );
        let range = if holder.left.span() == found.span() {
            Span::new(left.start, right.start)
        } else {
            Span::new(left.end, right.end)
        };
        if has_comment_in(source, range) || may_run_code(found) {
            findings.push(source.finding(self.name(), at, MESSAGE.to_owned()));
            return;
        }
        let fix = Fix {
            start: range.start,
            end: range.end,
            text: String::new(),
        };
        findings.push(source.fixed(self.name(), at, MESSAGE.to_owned(), fix));
    }
}

#[cfg(test)]
mod tests {
    use super::NoRedundantComparison;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoRedundantComparison, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn reports_a_comparison_implied_through_an_equality() {
        // The documentation's four cases: ordering ones only suggested, strict ones fixed.
        let ordered = "if (a > 0 && b > 0 && a === b) {}";
        assert_eq!(check(ordered), (vec![(1, 14)], ordered.to_owned()));
        assert_eq!(check("if (a > 0 && b > 2 && a === b) {}").0, [(1, 5)]);
        assert_eq!(
            check("if (a !== 'test' && b !== 'test' && a === b) {}"),
            (vec![(1, 21)], "if (a !== 'test' && a === b) {}".to_owned())
        );
        assert_eq!(
            check("if (a === 'test' && b !== 'test' && a !== b) {}").1,
            "if (a === 'test' && a !== b) {}"
        );
        // A weaker inclusive bound is the redundant one.
        assert_eq!(check("if (b >= 0 && a > 0 && a === b) {}").0, [(1, 5)]);
        // The first operand removed with what follows it.
        assert_eq!(
            check("if (x.y === z && x.y === 1 && z === 1) {}").0,
            [(1, 31)]
        );
        assert_eq!(
            check("if (b !== 'test' && a !== 'test' && a === b) {}").1,
            "if (b !== 'test' && a === b) {}"
        );
    }

    #[test]
    fn leaves_independent_or_alternative_comparisons() {
        for code in [
            "if (a > 0 && a === b) {}",
            "if (a > 0 || b > 0 || a === b) {}",
            "if (a > 0 && b < 0 && a === b) {}",
            "if (a > 0 && b > 0) {}",
            "if (a >= 0 && b < 1 && a === b) {}",
            "if (a?.c > 0 && b > 0 && a === b) {}",
            "if (a > x && b > y && a === b) {}",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
