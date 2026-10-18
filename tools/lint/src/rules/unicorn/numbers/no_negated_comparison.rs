//! `unicorn/no-negated-comparison`: a negated equality, `!(a === b)`, is written with the
//! opposite operator, `a !== b` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-negated-comparison.md`, `checkLogicalExpressions: false`). Relational
//! comparisons stay negated: `!(a > b)` also holds for `NaN`. Autofixed, the parentheses kept
//! where the comparison's place needs them.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{has_comment_in, is_parenthesized, parent, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoNegatedComparison;

/// The rule's message.
const MESSAGE: &str = "Prefer the opposite comparison instead of negating the whole comparison.";

/// The opposite of an equality operator.
fn opposite(operator: BinaryOperator) -> Option<BinaryOperator> {
    match operator {
        BinaryOperator::StrictEquality => Some(BinaryOperator::StrictInequality),
        BinaryOperator::StrictInequality => Some(BinaryOperator::StrictEquality),
        BinaryOperator::Equality => Some(BinaryOperator::Inequality),
        BinaryOperator::Inequality => Some(BinaryOperator::Equality),
        _ => None,
    }
}

impl Rule for NoNegatedComparison {
    fn name(&self) -> &'static str {
        "unicorn/no-negated-comparison"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::UnaryExpression(negation) = node.kind() else {
            return;
        };
        let up = parent(source, node.id());
        let doubled = matches!(up, AstKind::UnaryExpression(outer) if outer.operator == UnaryOperator::LogicalNot);
        if negation.operator != UnaryOperator::LogicalNot || doubled {
            return;
        }
        let Expression::BinaryExpression(comparison) = &negation.argument else {
            return;
        };
        let Some(replaced) = opposite(comparison.operator) else {
            return;
        };
        let operator_span = Span::new(comparison.left.span().end, comparison.right.span().start);
        if has_comment_in(
            source,
            Span::new(negation.span.start, comparison.span.start),
        ) {
            findings.push(source.finding(self.name(), negation.span.start, MESSAGE.to_owned()));
            return;
        }
        let between = text(source, operator_span).replacen(
            comparison.operator.as_str(),
            replaced.as_str(),
            1,
        );
        let rewritten = format!(
            "{}{between}{}",
            text(source, comparison.left.span()),
            text(source, comparison.right.span())
        );
        let grouped = matches!(
            up,
            AstKind::AwaitExpression(_)
                | AstKind::BinaryExpression(_)
                | AstKind::SpreadElement(_)
                | AstKind::TSAsExpression(_)
                | AstKind::TSNonNullExpression(_)
                | AstKind::TSSatisfiesExpression(_)
                | AstKind::TSTypeAssertion(_)
                | AstKind::UnaryExpression(_)
                | AstKind::YieldExpression(_)
        ) && !is_parenthesized(source, negation.span);
        let fix = Fix {
            start: negation.span.start,
            end: negation.span.end,
            text: if grouped {
                format!("({rewritten})")
            } else {
                rewritten
            },
        };
        findings.push(source.fixed(self.name(), negation.span.start, MESSAGE.to_owned(), fix));
    }
}

#[cfg(test)]
mod tests {
    use super::NoNegatedComparison;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoNegatedComparison, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn writes_the_opposite_equality() {
        // The documentation's two reported cases.
        assert_eq!(
            check("const isDifferent = !(a === b);"),
            (vec![(1, 21)], "const isDifferent = a !== b;".to_owned())
        );
        assert_eq!(
            check("if (!(typeof value === 'undefined')) {}").1,
            "if (typeof value !== 'undefined') {}"
        );
        // Loose equality; a place that needs the group.
        assert_eq!(check("f(!(a != b));").1, "f(a == b);");
        assert_eq!(
            check("const c = 1 + !(a == b);").1,
            "const c = 1 + (a != b);"
        );
    }

    #[test]
    fn leaves_relational_and_logical_negations() {
        for code in [
            "if (!(a === b && c === d)) {}",
            "if (!(a > b)) {}",
            "if (!!(a === b)) {}",
            "if (a !== b) {}",
            "const c = -(a === b);",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let commented = "const c = !/* why */(a === b);";
        assert_eq!(check(commented), (vec![(1, 11)], commented.to_owned()));
    }
}
