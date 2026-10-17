//! `unicorn/no-object-methods-with-collections`: no `Object.keys()`, `Object.values()` nor
//! `Object.entries()` on a `Map` or a `Set`, whose contents are no own properties: the result is
//! empty (`eslint-plugin-unicorn` 78.0.0). Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::syntax::{Kind, is_global, kind_of, method_call};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoObjectMethodsWithCollections;

impl Rule for NoObjectMethodsWithCollections {
    fn name(&self) -> &'static str {
        "unicorn/no-object-methods-with-collections"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call).filter(|method| {
            matches!(method.name, "keys" | "values" | "entries")
                && method.is_plain()
                && is_global(source, method.object, "Object")
        }) else {
            return;
        };
        let Some(&[argument]) = method.arguments().as_deref() else {
            return;
        };
        let collection = match kind_of(source, argument) {
            Some(Kind::Map) => "Map",
            Some(Kind::Set) => "Set",
            _ => return,
        };
        findings.push(source.finding(
            self.name(),
            method.property.start,
            format!(
                "`Object.{}()` does not return {collection} contents.",
                method.name
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoObjectMethodsWithCollections;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoObjectMethodsWithCollections, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_collections() {
        assert_eq!(
            check("const map = new Map([\n\t['a', 1],\n\t['b', 2],\n]);\nObject.keys(map);"),
            ["`Object.keys()` does not return Map contents."]
        );
        assert_eq!(
            check("const map = new Map([['a', 1]]);\nObject.values(map);"),
            ["`Object.values()` does not return Map contents."]
        );
        assert_eq!(
            check("const set = new Set(['a', 'b']);\nObject.entries(set);"),
            ["`Object.entries()` does not return Set contents."]
        );
        assert_eq!(
            check("function f(s: ReadonlySet<string>) { return Object.keys(s); }").len(),
            1
        );
        assert_eq!(check("Object.keys(new Map());").len(), 1);
        assert_eq!(check("Object.keys(m as Map<string, number>);").len(), 1);
        assert_eq!(check("Object.keys(a ? new Set() : new Set());").len(), 1);
    }

    #[test]
    fn accepts_weak_unknown_and_mixed_values() {
        for code in [
            "Array.from(map.keys());",
            "const map = new WeakMap(); Object.keys(map);",
            "const set = new WeakSet(); Object.keys(set);",
            "Object.keys(object);",
            "Object.keys(a ? new Set() : new Map());",
            "Object.keys(new Map(), extra);",
            "Object?.keys(new Map());",
            "Object.assign(new Map());",
            "const Object = { keys: (x: unknown) => x }; Object.keys(new Map());",
            "let map = new Map(); Object.keys(map);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
