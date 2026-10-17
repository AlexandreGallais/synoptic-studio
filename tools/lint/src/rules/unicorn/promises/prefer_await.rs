//! `unicorn/prefer-await`: a promise chained with `.then()`, `.catch()` or `.finally()` where
//! `await` would do (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-await.md`). Without type
//! information every receiver may be a promise, as the plugin reads it; a chain whose value is
//! discarded with `void` is left. Reported only: the plugin offers a suggestion, never a fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::calls::{is_discarded_with_void, method_name};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferAwait;

impl Rule for PreferAwait {
    fn name(&self) -> &'static str {
        "unicorn/prefer-await"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some((method, at)) = method_name(source, &call.callee) else {
            return;
        };
        if matches!(method.as_str(), "then" | "catch" | "finally")
            && !is_discarded_with_void(source, node)
        {
            findings.push(source.finding(
                self.name(),
                at.start,
                format!("Prefer `await` over promise chaining with `.{method}()`."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferAwait;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferAwait, "playground/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_promise_chains() {
        // The documentation's three wrong cases, then a computed name and a chain.
        assert_eq!(
            places("promise.then(value => {\n\treturn transform(value);\n});"),
            [(1, 9)]
        );
        assert_eq!(
            places("promise.catch(error => {\n\thandleError(error);\n});"),
            [(1, 9)]
        );
        assert_eq!(places("promise.finally(cleanup);"), [(1, 9)]);
        assert_eq!(places("promise['then'](f);"), [(1, 9)]);
        assert_eq!(places("a.then(f).catch(g);"), [(1, 11), (1, 3)]);
        assert_eq!(places("const b = a?.then(f);"), [(1, 14)]);
    }

    #[test]
    fn leaves_awaits_and_chains_discarded_with_void() {
        for code in [
            "const value = await promise;",
            "void (async () => {\n\tconst value = await promise;\n})();",
            "void promise.catch(() => {});",
            "void promise.then(f).catch(g);",
            "void (promise as Promise<number>).then(f);",
            "void a?.then(f);",
            "promise[method](f);",
            "promise.thenable(f);",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
