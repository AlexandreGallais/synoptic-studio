//! `unicorn/prefer-then-catch`: a rejection handler passed as `.then()`'s second argument, which
//! misses the errors of the first, where `.then(…).catch(…)` handles both
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-then-catch.md`). Only a plain
//! `.then(a, b)` with two arguments is read, neither of them `null`, `undefined` nor `void …`.
//! Reported only: the plugin offers a suggestion, never a fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, UnaryOperator};
use oxc_semantic::AstNode;

use crate::expressions::is_global;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::unwrap;
use crate::source::Source;

/// The rule.
pub struct PreferThenCatch;

/// The rule's message.
const MESSAGE: &str = "Prefer `.then(…).catch(…)` over passing a rejection handler to `.then()`.";

/// Whether a handler is `null`, `void …` or the global `undefined`.
fn is_nullish(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::NullLiteral(_) => true,
        Expression::UnaryExpression(unary) => unary.operator == UnaryOperator::Void,
        Expression::Identifier(name) => name.name == "undefined" && is_global(source, name),
        _ => false,
    }
}

impl Rule for PreferThenCatch {
    fn name(&self) -> &'static str {
        "unicorn/prefer-then-catch"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Expression::StaticMemberExpression(member) = &call.callee else {
            return;
        };
        if member.property.name != "then" || member.optional || call.optional {
            return;
        }
        let [fulfilled, rejected] = call.arguments.as_slice() else {
            return;
        };
        let (Some(fulfilled), Some(rejected)) =
            (fulfilled.as_expression(), rejected.as_expression())
        else {
            return;
        };
        if !is_nullish(source, fulfilled) && !is_nullish(source, rejected) {
            findings.push(source.finding(
                self.name(),
                member.property.span.start,
                MESSAGE.to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferThenCatch;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferThenCatch, "playground/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_rejection_handler_given_to_then() {
        // The documentation's wrong case, then a call discarded with `void`.
        assert_eq!(
            places(
                "fetch(url).then(\n\tresponse => JSON.parse(response),\n\terror => {\n\t\tlog(error);\n\t},\n);"
            ),
            [(1, 12)]
        );
        assert_eq!(places("void promise.then(show, report);"), [(1, 14)]);
    }

    #[test]
    fn leaves_catch_nullish_handlers_and_other_calls() {
        for code in [
            "fetch(url)\n\t.then(response => JSON.parse(response))\n\t.catch(error => {\n\t\tlog(error);\n\t});",
            "promise.then(undefined, handleError);",
            "promise.then(onFulfilled, undefined);",
            "promise.then(null, handleError);",
            "promise.then(void 0, handleError);",
            "promise?.then(a, b);",
            "promise.then?.(a, b);",
            "promise['then'](a, b);",
            "promise.then(a, b, c);",
            "promise.then(...handlers, b);",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
