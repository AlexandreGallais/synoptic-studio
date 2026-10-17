//! `unicorn/no-duplicate-logical-operands`: the right operand of `&&` or `||` that repeats the
//! operand on its left, a name, `this` or a member read through them with a static or simple
//! key (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-duplicate-logical-operands.md`). Autofixed
//! for names and `this`, a member read being left to the editor's suggestion: it can run a getter.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, LogicalExpression, LogicalOperator};
use oxc_semantic::AstNode;
use oxc_span::{ContentEq, GetSpan, Span};

use crate::expressions::{has_comment_in, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDuplicateLogicalOperands;

/// The rule's message.
const MESSAGE: &str = "This operand duplicates the left operand.";

/// An expression without its TypeScript wrappers, `as`, `satisfies` and `!`.
fn unwrapped<'s, 'a>(expression: &'s Expression<'a>) -> &'s Expression<'a> {
    match expression {
        Expression::TSAsExpression(inner) => unwrapped(&inner.expression),
        Expression::TSSatisfiesExpression(inner) => unwrapped(&inner.expression),
        Expression::TSNonNullExpression(inner) => unwrapped(&inner.expression),
        Expression::TSTypeAssertion(inner) => unwrapped(&inner.expression),
        other => other,
    }
}

/// Whether an expression is a plain reference: a name, `this`, `super`, or a member read
/// through one with a static key or a name, `this` or a literal as key, no optional chain.
fn is_reference(expression: &Expression) -> bool {
    match unwrapped(expression) {
        Expression::Identifier(_) | Expression::ThisExpression(_) | Expression::Super(_) => true,
        Expression::StaticMemberExpression(member) => {
            !member.optional && is_reference(&member.object)
        }
        Expression::ComputedMemberExpression(member) => {
            let key = unwrapped(&member.expression);
            !member.optional
                && is_reference(&member.object)
                && (matches!(
                    key,
                    Expression::Identifier(_) | Expression::ThisExpression(_)
                ) || key.is_literal())
        }
        _ => false,
    }
}

/// The operand right on the left of a logical expression's right operand.
fn adjacent_left<'s, 'a>(logical: &'s LogicalExpression<'a>) -> &'s Expression<'a> {
    match &logical.left {
        Expression::LogicalExpression(inner) if inner.operator == logical.operator => &inner.right,
        other => other,
    }
}

impl Rule for NoDuplicateLogicalOperands {
    fn name(&self) -> &'static str {
        "unicorn/no-duplicate-logical-operands"
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
        let left = adjacent_left(logical);
        let right = &logical.right;
        if !is_reference(left)
            || !is_reference(right)
            || !unwrapped(left).content_eq(unwrapped(right))
        {
            return;
        }
        let plain = |each: &Expression| {
            matches!(
                each,
                Expression::Identifier(_) | Expression::ThisExpression(_)
            )
        };
        let outside_left = [
            Span::new(logical.span.start, logical.left.span().start),
            Span::new(logical.left.span().end, logical.span.end),
        ];
        let commented = outside_left
            .iter()
            .any(|span| has_comment_in(source, *span));
        if plain(left) && plain(right) && !commented {
            let fix = Fix {
                start: logical.span.start,
                end: logical.span.end,
                text: text(source, logical.left.span()).to_owned(),
            };
            findings.push(source.fixed(self.name(), right.span().start, MESSAGE.to_owned(), fix));
        } else {
            findings.push(source.finding(self.name(), right.span().start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateLogicalOperands;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoDuplicateLogicalOperands, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn reports_and_fixes_a_repeated_name() {
        // The documentation's cases: names fixed, a member read only reported.
        assert_eq!(
            check("if (isReady && isReady) {}\nconst result = value || value;"),
            (
                vec![(1, 16), (2, 25)],
                "if (isReady) {}\nconst result = value;".to_owned()
            )
        );
        let member = "if (object.isReady && object.isReady) {}\nobject.isReady && object.isReady;";
        assert_eq!(check(member), (vec![(1, 23), (2, 19)], member.to_owned()));
        // The operand next to it in a chain; `this`, a computed key, a TypeScript wrapper.
        assert_eq!(
            check("a && b && b;\nthis || this;\nx[0] && x[0];\nx[k] || (x[k] as T);").0,
            [(1, 11), (2, 9), (3, 9), (4, 10)]
        );
    }

    #[test]
    fn leaves_distant_repeats_calls_optional_chains_and_comments() {
        for code in [
            "foo && bar && foo;",
            "f() && f();",
            "a?.b && a?.b;",
            "a ?? a;",
            "a[f()] && a[f()];",
            "a && b;",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let commented = "a /* first */ && a;";
        assert_eq!(check(commented).1, "a /* first */ && a;");
    }
}
