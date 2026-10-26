//! `@angular-eslint/template/no-negated-async`: neither the result of the `async` pipe negated,
//! `!(value | async)`, which flips while the value is still `null`, nor a negated value given
//! to it, `!value | async` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/no-negated-async.ts`): the first reported over the `!` and its parentheses, unless
//! negated twice (`!!(value | async)`), its pipe the last of the parentheses; the second over the
//! value and the pipe. The plugin's comparisons with `false`, `null` and `undefined` are
//! suggestions, not carried.

use std::collections::BTreeSet;

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, UnaryOperator};

use super::{Report, Template, TemplateRule, each_node, every_bound, nested_at, pipe_span, placed};

/// The rule.
pub struct NoNegatedAsync;

/// The `!` of a unary expression.
fn negation<'r, 'a>(expression: &'r Expression<'a>) -> Option<&'r Expression<'a>> {
    match expression {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            Some(&unary.argument)
        }
        _ => None,
    }
}

impl TemplateRule for NoNegatedAsync {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-negated-async"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "noNegatedAsync",
                "Async pipe results should not be negated. Use `(observable | async) === false`, `(observable | async) === null`, or `(observable | async) === undefined` to check its value instead",
            ),
            (
                "noNegatedValueForAsync",
                "Values used with the async pipe should not be negated.",
            ),
            ("suggestFalseComparison", "Compare with `false`"),
            ("suggestNullComparison", "Compare with `null`"),
            ("suggestUndefinedComparison", "Compare with `undefined`"),
            ("suggestUsingNonNegatedValue", "Use non-negated value"),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for written in template.expressions() {
            for bound in every_bound(&written.bound) {
                let negated = bound.expression.as_ref().and_then(negation).is_some();
                if negated && bound.pipes.first().is_some_and(|pipe| pipe.name == "async") {
                    reports.push(Report::new(
                        pipe_span(template.text, bound, true),
                        "noNegatedValueForAsync",
                    ));
                }
            }
            // A `!` directly under another is not reported; the outer one is met first.
            let mut doubled = BTreeSet::new();
            each_node(&written, &mut |node, bound| {
                let AstKind::UnaryExpression(unary) = node else {
                    return;
                };
                if unary.operator != UnaryOperator::LogicalNot {
                    return;
                }
                if let Expression::UnaryExpression(inner) = &unary.argument {
                    doubled.insert(placed(bound, inner.span).start);
                }
                let span = placed(bound, unary.span);
                let Expression::ParenthesizedExpression(parenthesized) = &unary.argument else {
                    return;
                };
                let Expression::Identifier(masked) = &parenthesized.expression else {
                    return;
                };
                let last_is_async = nested_at(bound, masked.span)
                    .and_then(|nested| nested.pipes.last())
                    .is_some_and(|pipe| pipe.name == "async");
                if last_is_async && !doubled.contains(&span.start) {
                    reports.push(Report::new(span, "noNegatedAsync"));
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoNegatedAsync;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-negated-async.txt", &[], &|_| Box::new(NoNegatedAsync));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 6 valid and 5 invalid cases, all run; their 13 suggestions, not carried, unchecked.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (6, 5, 13));
    }
}
