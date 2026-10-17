//! `unicorn/no-duplicate-loops`: no `.map()` nor `.filter()` written in a `for…of` header, which
//! builds an array only to loop over it; lazy iterator helpers are left alone
//! (`eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::syntax::{call_of, is_iterator, method_call};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDuplicateLoops;

impl Rule for NoDuplicateLoops {
    fn name(&self) -> &'static str {
        "unicorn/no-duplicate-loops"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ForOfStatement(statement) = node.kind() else {
            return;
        };
        let Some(method) = call_of(&statement.right)
            .and_then(method_call)
            .filter(|method| {
                matches!(method.name, "filter" | "map")
                    && method.is_plain()
                    && !method.call.arguments.is_empty()
            })
        else {
            return;
        };
        if is_iterator(source, method.object) {
            return;
        }
        let kind = if statement.r#await {
            "for await…of"
        } else {
            "for…of"
        };
        findings.push(source.finding(
            self.name(),
            method.property.start,
            format!(
                "Do not use `.{}()` directly in a `{kind}` loop header. It creates an intermediate array before the loop.",
                method.name
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateLoops;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoDuplicateLoops, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_headers() {
        assert_eq!(
            check(
                "for (const value of values.map(value => value.property)) {\n\tconsole.log(value);\n}"
            ),
            [
                "Do not use `.map()` directly in a `for…of` loop header. It creates an intermediate array before the loop."
            ]
        );
        assert_eq!(
            check(
                "for (const value of values.filter(value => value.isEnabled)) {\n\tconsole.log(value);\n}"
            ),
            [
                "Do not use `.filter()` directly in a `for…of` loop header. It creates an intermediate array before the loop."
            ]
        );
        assert_eq!(
            check("async function f() { for await (const v of items.map(fetch)) {} }"),
            [
                "Do not use `.map()` directly in a `for await…of` loop header. It creates an intermediate array before the loop."
            ]
        );
        assert_eq!(
            check("for (const v of (values as T[]).map(f) as U[]) {}").len(),
            1
        );
    }

    #[test]
    fn accepts_lazy_helpers_and_other_headers() {
        for code in [
            "for (const value of values) {\n\tconsole.log(value.property);\n}",
            "for (const v of map.values().map(f)) {}",
            "for (const v of Iterator.from(items).filter(f)) {}",
            "for (const v of text.matchAll(re).map(f)) {}",
            "for (const v of set.keys().filter(f).map(g)) {}",
            "function* gen() { yield 1; }\nfor (const v of gen().map(f)) {}",
            "const it = map.entries();\nfor (const v of it.map(f)) {}",
            "for (const v of values.map()) {}",
            "for (const v of values?.map(f)) {}",
            "for (const v of values.flatMap(f)) {}",
            "const mapped = values.map(f);\nfor (const v of mapped) {}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
