//! `unicorn/prefer-iterator-to-array`: `iterator.toArray()` rather than `[...iterator]` where an
//! array is wanted (`eslint-plugin-unicorn` 78.0.0). Autofixed for `Iterator.from()` and
//! `Iterator.concat()` chains, as in `ESLint`; the `.values()`, `.keys()`, `.entries()` and
//! `.matchAll()` cases are suggestions there, reported only here.

use oxc_ast::AstKind;
use oxc_ast::ast::{ArrayExpressionElement, Expression};
use oxc_semantic::AstNode;

use super::syntax::{has_comment, method_call, parenthesized_text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferIteratorToArray;

/// The typed arrays' names.
const TYPED_ARRAYS: [&str; 12] = [
    "Int8Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Int16Array",
    "Uint16Array",
    "Int32Array",
    "Uint32Array",
    "Float16Array",
    "Float32Array",
    "Float64Array",
    "BigInt64Array",
    "BigUint64Array",
];

/// How a spread iterator is rewritten: fixed, or only reported.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Rewrite {
    /// A fix.
    Fix,
    /// A report.
    Report,
}

/// How a spread argument may be rewritten, when it is an iterator.
fn rewrite(expression: &Expression) -> Option<Rewrite> {
    let Expression::CallExpression(call) = expression else {
        return None;
    };
    let method = method_call(call).filter(super::syntax::MethodCall::is_plain)?;
    let on_iterator =
        matches!(method.object, Expression::Identifier(object) if object.name == "Iterator");
    match method.name {
        "from" if on_iterator && call.arguments.len() == 1 => Some(Rewrite::Fix),
        "concat" if on_iterator => Some(Rewrite::Fix),
        "entries" | "keys" | "values" if call.arguments.is_empty() => Some(Rewrite::Report),
        "matchAll" if call.arguments.len() == 1 => Some(Rewrite::Report),
        "drop" | "filter" | "flatMap" | "map" | "take" if call.arguments.len() == 1 => {
            rewrite(method.object)
        }
        _ => None,
    }
}

/// Whether the array's place accepts any iterable, needing no array.
fn accepts_iterable(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    let is_first = |arguments: &[oxc_ast::ast::Argument]| {
        arguments
            .first()
            .and_then(|argument| argument.as_expression())
            .is_some_and(|argument| argument.node_id() == node.id())
    };
    match parent.kind() {
        AstKind::ForOfStatement(statement) => statement.right.node_id() == node.id(),
        AstKind::YieldExpression(expression) => expression.delegate,
        AstKind::SpreadElement(_) => matches!(
            nodes.parent_kind(parent.id()),
            AstKind::ArrayExpression(_) | AstKind::CallExpression(_) | AstKind::NewExpression(_)
        ),
        AstKind::NewExpression(new) => {
            let Expression::Identifier(callee) = &new.callee else {
                return false;
            };
            let collection = matches!(callee.name.as_str(), "Map" | "WeakMap" | "Set" | "WeakSet")
                && new.arguments.len() == 1;
            let typed = TYPED_ARRAYS.contains(&callee.name.as_str()) && !new.arguments.is_empty();
            (collection || typed) && is_first(&new.arguments)
        }
        AstKind::CallExpression(call) => {
            let Some(method) = method_call(call).filter(super::syntax::MethodCall::is_plain) else {
                return false;
            };
            let Expression::Identifier(object) = method.object else {
                return false;
            };
            let object = object.name.as_str();
            let consumes = (object == "Promise"
                && matches!(method.name, "all" | "allSettled" | "any" | "race")
                && call.arguments.len() == 1)
                || ((object == "Array" || TYPED_ARRAYS.contains(&object)) && method.name == "from")
                || (object == "Object" && method.name == "fromEntries");
            consumes && is_first(&call.arguments)
        }
        _ => false,
    }
}

impl Rule for PreferIteratorToArray {
    fn name(&self) -> &'static str {
        "unicorn/prefer-iterator-to-array"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ArrayExpression(array) = node.kind() else {
            return;
        };
        let [ArrayExpressionElement::SpreadElement(spread)] = array.elements.as_slice() else {
            return;
        };
        if accepts_iterable(source, node) {
            return;
        }
        let Some(kind) = rewrite(&spread.argument) else {
            return;
        };
        let message = "Prefer `Iterator#toArray()` over a temporary spread array.".to_owned();
        if kind == Rewrite::Report || has_comment(source, array.span) {
            findings.push(source.finding(self.name(), array.span.start, message));
            return;
        }
        findings.push(source.fixed(
            self.name(),
            array.span.start,
            message,
            Fix {
                start: array.span.start,
                end: array.span.end,
                text: format!("{}.toArray()", parenthesized_text(source, &spread.argument)),
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferIteratorToArray;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferIteratorToArray, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferIteratorToArray, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_spreads() {
        let message = ["Prefer `Iterator#toArray()` over a temporary spread array."];

        assert_eq!(check("const values = [...map.values()];"), message);
        assert_eq!(
            check("const values = [...map.values().map(value => value * 2)];"),
            message
        );
        // Reported, not fixed: custom `values()` may return other iterables.
        assert_eq!(
            fix("const values = [...map.values()];"),
            "const values = [...map.values()];"
        );
        assert_eq!(
            fix("const values = [...Iterator.from(items).map(f)];"),
            "const values = Iterator.from(items).map(f).toArray();"
        );
        assert_eq!(check("const m = [...text.matchAll(re)];"), message);
    }

    #[test]
    fn accepts_iterables_mixed_arrays_and_iterable_places() {
        for code in [
            "const values = [...set];",
            "new Set([...map.values()]);",
            "const values = [first, ...map.values()];",
            "const values = [...foo, ...bar];",
            "for (const v of [...map.values()]) {}",
            "function* g() { yield* [...map.values()]; }",
            "use(...[...map.values()]);",
            "Array.from([...map.values()]);",
            "Promise.all([...map.values()]);",
            "Object.fromEntries([...map.entries()]);",
            "new Uint8Array([...map.values()]);",
            "const values = [...map.values(1)];",
            "const values = [...items.map(f)];",
            "const values = [...map?.values()];",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
