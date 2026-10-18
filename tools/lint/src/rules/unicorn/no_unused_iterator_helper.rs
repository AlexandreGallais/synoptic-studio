//! `unicorn/no-unused-iterator-helper`: no lazy iterator helper whose result is thrown away —
//! `.map()`, `.filter()`, `.flatMap()`, `.take()` or `.drop()` on an iterator read nothing until
//! consumed (`eslint-plugin-unicorn` 78.0.0). Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::UnaryOperator;
use oxc_semantic::AstNode;

use super::syntax::{is_lazy_helper, method_call};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUnusedIteratorHelper;

/// Whether a call's result is thrown away: a statement, `void`, a `for` header's init or update,
/// through `await`, `?.` and TypeScript's wrappers.
fn is_discarded(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = node.id();
    loop {
        let parent = nodes.parent_node(child);
        match parent.kind() {
            AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_)
            | AstKind::AwaitExpression(_)
            | AstKind::ChainExpression(_) => child = parent.id(),
            AstKind::ExpressionStatement(_) => return true,
            AstKind::UnaryExpression(unary) => return unary.operator == UnaryOperator::Void,
            AstKind::ForStatement(statement) => {
                let init = statement
                    .init
                    .as_ref()
                    .and_then(|init| init.as_expression())
                    .is_some_and(|init| init.node_id() == child);
                let update = statement
                    .update
                    .as_ref()
                    .is_some_and(|update| update.node_id() == child);
                return init || update;
            }
            _ => return false,
        }
    }
}

impl Rule for NoUnusedIteratorHelper {
    fn name(&self) -> &'static str {
        "unicorn/no-unused-iterator-helper"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        if !is_discarded(source, node) || !is_lazy_helper(source, call, 0) {
            return;
        }
        let Some(method) = method_call(call) else {
            return;
        };
        findings.push(source.finding(
            self.name(),
            method.property.start,
            format!(
                "Consume the iterator returned by `.{}(…)`; iterator helpers are lazy.",
                method.name
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnusedIteratorHelper;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnusedIteratorHelper, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_discarded_helpers() {
        assert_eq!(
            check("items.values().map(transform);"),
            ["Consume the iterator returned by `.map(…)`; iterator helpers are lazy."]
        );
        assert_eq!(
            check("void items.values().filter(predicate);"),
            ["Consume the iterator returned by `.filter(…)`; iterator helpers are lazy."]
        );
        assert_eq!(
            check("async function f() { await items.values().flatMap(transform); }"),
            ["Consume the iterator returned by `.flatMap(…)`; iterator helpers are lazy."]
        );
        assert_eq!(check("items.keys().drop(1) as unknown;").len(), 1);
        assert_eq!(
            check("for (items.entries().take(1); ;) { break; }").len(),
            1
        );
        assert_eq!(check("const it = text.matchAll(re);\nit.map(f);").len(), 1);
        assert_eq!(check("items?.values().map(f);").len(), 1);
    }

    #[test]
    fn accepts_consumed_and_stored_helpers() {
        for code in [
            "const transformed = items.values().map(transform).toArray();",
            "const filtered = items.values().filter(predicate);\nfor (const item of filtered) {\n\tconsume(item);\n}",
            "items.map(transform);",
            "items.values().map();",
            "items.values().forEach(f);",
            "use(items.values().map(f));",
            "items.values().map(f) || x;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
