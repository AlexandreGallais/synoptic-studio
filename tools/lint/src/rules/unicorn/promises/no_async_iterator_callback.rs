//! `unicorn/no-async-iterator-callback`: an `async` callback given to an iterator's `filter`,
//! `forEach`, `some`, `every`, `find` or `flatMap`, which do not wait for the promises it returns
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-async-iterator-callback.md`). The receiver is
//! an iterator as `isIteratorExpression` tells it from the tree; the callback is written there,
//! bound to a `const`, or a function declared once with a body and never reassigned. No fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::calls::{is_async_callback, method_name};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::is_iterator;
use crate::source::Source;

/// The rule.
pub struct NoAsyncIteratorCallback;

/// The helpers that do not wait for their callback's promise.
const METHODS: [&str; 6] = ["filter", "forEach", "some", "every", "find", "flatMap"];

impl Rule for NoAsyncIteratorCallback {
    fn name(&self) -> &'static str {
        "unicorn/no-async-iterator-callback"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(member) = call.callee.as_member_expression() else {
            return;
        };
        let Some((method, _)) = method_name(source, &call.callee) else {
            return;
        };
        let Some(callback) = call.arguments.first().and_then(|each| each.as_expression()) else {
            return;
        };
        if METHODS.contains(&method.as_str())
            && is_iterator(source, member.object())
            && is_async_callback(source, callback, true)
        {
            findings.push(source.finding(
                self.name(),
                callback.span().start,
                format!(
                    "Do not pass an asynchronous callback to `Iterator#{method}()`; returned promises are not awaited."
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAsyncIteratorCallback;
    use crate::testing::lint;

    /// The lines of the findings.
    fn lines(code: &str) -> Vec<usize> {
        lint(&NoAsyncIteratorCallback, "playground/a.ts", code)
            .iter()
            .map(|each| each.line)
            .collect()
    }

    #[test]
    fn reports_async_callbacks_of_iterator_helpers() {
        // The documentation's six wrong cases, then its right ones.
        let code = "Iterator.from(values).filter(async value => isAllowed(value));\nIterator.from(values).some(async value => isAllowed(value));\nIterator.from(values).every(async value => isAllowed(value));\nIterator.from(values).find(async value => isAllowed(value));\nIterator.from(values).forEach(async value => {\n\tawait save(value);\n});\nIterator.from(values).flatMap(async value => getChildren(value));\nIterator.from(values).filter(value => value.isAllowed);\nconst results = await Promise.all(\n\tIterator.from(values).map(async value => transform(value))\n);\nconst total = await Iterator.from(values).reduce(\n\tasync (sum, value) => (await sum) + (await getAmount(value)),\n\t0\n);";

        assert_eq!(lines(code), [1, 2, 3, 4, 5, 8]);
    }

    #[test]
    fn follows_bound_callbacks_and_iterators() {
        let code = "const check = async (value: number) => true;\nasync function save(value: number) {}\nconst items = map.values();\nitems.filter(check);\nmap.keys().forEach(save);\ntext.matchAll(/a/gu).some(async () => true);\nIterator.from(values).filter(async function* () {});\nvalues.filter(async () => true);\nIterator.from(values).filter(other);\nIterator.from(values)['find'](async () => true);\nfunction cleanup(): Promise<void>;\nasync function cleanup() {}\nIterator.from(values).filter(cleanup);";

        assert_eq!(lines(code), [4, 5, 6, 10, 13]);
    }
}
