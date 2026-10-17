//! `unicorn/no-unreadable-for-of-expression`: no complex iterable in a `for…of` header — a name,
//! a property chain, an array literal or a simple call keep it readable; anything else, lazy
//! iterator chains included, goes to a named variable first (`eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, ArrayExpressionElement, CallExpression, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{call_of, is_iterator, is_lazy_helper, method_call, unwrap_chain};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUnreadableForOfExpression;

/// Whether an expression is a literal.
fn is_literal(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
    )
}

/// Whether a member reads a plain property of a simple object: `a.b`, `this.#c`.
fn is_simple_member(expression: &Expression) -> bool {
    match expression {
        Expression::StaticMemberExpression(member) => {
            !member.optional && is_simple_iterable(&member.object)
        }
        Expression::PrivateFieldExpression(member) => {
            !member.optional && is_simple_iterable(&member.object)
        }
        _ => false,
    }
}

/// Whether a call's argument is simple: a name, a literal, a template, a simple member, or a
/// call without arguments of a name or a simple member.
fn is_simple_argument(expression: &Expression) -> bool {
    let expression = unwrap_chain(expression);
    match expression {
        Expression::Identifier(_)
        | Expression::ThisExpression(_)
        | Expression::Super(_)
        | Expression::TemplateLiteral(_) => true,
        Expression::CallExpression(call) => {
            !call.optional
                && call.arguments.is_empty()
                && (matches!(call.callee, Expression::Identifier(_))
                    || is_simple_member(&call.callee))
        }
        other => is_literal(other) || is_simple_member(other),
    }
}

/// Whether a call is `filter(…)` or `map(…)`, which `no-duplicate-loops` reports instead.
fn is_array_iteration(call: &CallExpression) -> bool {
    method_call(call).is_some_and(|method| {
        matches!(method.name, "filter" | "map") && method.is_plain() && !call.arguments.is_empty()
    })
}

/// Whether a call is simple: of a name or a simple member, its arguments simple or arrays of
/// simple elements.
fn is_simple_call(call: &CallExpression) -> bool {
    let callee_simple =
        matches!(call.callee, Expression::Identifier(_)) || is_simple_member(&call.callee);
    let arguments_simple = call.arguments.iter().all(|argument| match argument {
        Argument::SpreadElement(_) => false,
        other => other.as_expression().is_some_and(|argument| {
            let argument = unwrap_chain(argument);
            is_simple_argument(argument)
                || matches!(argument, Expression::ArrayExpression(array) if array.elements.iter().all(|element| match element {
                    ArrayExpressionElement::SpreadElement(_) | ArrayExpressionElement::Elision(_) => false,
                    other => other.as_expression().is_some_and(is_simple_argument),
                }))
        }),
    });
    !call.optional && !is_array_iteration(call) && callee_simple && arguments_simple
}

/// Whether an iterable reads simply in a loop header.
fn is_simple_iterable(expression: &Expression) -> bool {
    let expression = unwrap_chain(expression);
    match expression {
        Expression::Identifier(_)
        | Expression::ThisExpression(_)
        | Expression::Super(_)
        | Expression::ArrayExpression(_) => true,
        Expression::CallExpression(call) => is_simple_call(call),
        other => is_literal(other) || is_simple_member(other),
    }
}

impl Rule for NoUnreadableForOfExpression {
    fn name(&self) -> &'static str {
        "unicorn/no-unreadable-for-of-expression"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ForOfStatement(statement) = node.kind() else {
            return;
        };
        let right = unwrap_chain(&statement.right);
        let call = call_of(right);
        let lazy = call.is_some_and(|call| is_lazy_helper(source, call, 0));
        let duplicate = call.is_some_and(|call| {
            is_array_iteration(call)
                && method_call(call).is_some_and(|method| !is_iterator(source, method.object))
        });
        if !lazy && (duplicate || is_simple_iterable(right)) {
            return;
        }
        let kind = if statement.r#await {
            "for await…of"
        } else {
            "for…of"
        };
        findings.push(source.finding(
            self.name(),
            statement.right.span().start,
            format!("Move the complex iterable expression out of the `{kind}` loop header."),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnreadableForOfExpression;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnreadableForOfExpression, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_complex_headers() {
        let message = ["Move the complex iterable expression out of the `for…of` loop header."];

        assert_eq!(
            check("for (const item of getItems(createArgument(seed))) {\n\tconsole.log(item);\n}"),
            message
        );
        assert_eq!(
            check(
                "for (const key of Object.keys(Object.fromEntries(entries))) {\n\tconsole.log(key);\n}"
            ),
            message
        );
        assert_eq!(check("for (const v of map.values().map(f)) {}"), message);
        assert_eq!(check("for (const v of a ? b : c) {}"), message);
        assert_eq!(check("for (const v of f(...args)) {}"), message);
        assert_eq!(check("for (const v of f(x + 1)) {}"), message);
        assert_eq!(check("for (const v of a?.()) {}"), message);
        assert_eq!(
            check("async function f() { for await (const v of g(h(1))) {} }"),
            ["Move the complex iterable expression out of the `for await…of` loop header."]
        );
    }

    #[test]
    fn accepts_the_documented_simple_headers() {
        for code in [
            "const argument = createArgument(seed);\nfor (const item of getItems(argument)) {\n\tconsole.log(item);\n}",
            "for (const item of getItems(argument)) {\n\tconsole.log(item);\n}",
            "for (const item of items) {\n\tconsole.log(item);\n}",
            "for (const value of Object.values(object)) {\n\tconsole.log(value);\n}",
            "for (const v of [a, b]) {}",
            "for (const v of this.items) {}",
            "for (const v of a.b.c) {}",
            "for (const v of 'text') {}",
            "for (const v of f(a, 'b', `c`, d.e, g(), h.i(), [j, 1])) {}",
            "for (const v of items.map(f)) {}",
            "for (const v of items.filter(f)) {}",
            "for (const v of map.entries()) {}",
            "for (const v of items as T[]) {}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
