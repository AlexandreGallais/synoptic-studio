//! `unicorn/no-async-promise-finally`: an `async` function given to `.finally()`, whose
//! rejection would replace the promise's own (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-async-promise-finally.md`). The function is written there, bound to a `const`,
//! or a function declared once. No fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::calls::{is_async_callback, method_name};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoAsyncPromiseFinally;

/// The rule's message.
const MESSAGE: &str = "Do not pass an async function to `Promise#finally()`.";

impl Rule for NoAsyncPromiseFinally {
    fn name(&self) -> &'static str {
        "unicorn/no-async-promise-finally"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        if method_name(source, &call.callee).is_none_or(|(name, _)| name != "finally") {
            return;
        }
        let Some(callback) = call.arguments.first().and_then(|each| each.as_expression()) else {
            return;
        };
        if is_async_callback(source, callback, false) {
            findings.push(source.finding(self.name(), callback.span().start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAsyncPromiseFinally;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoAsyncPromiseFinally, "playground/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_an_async_finalizer() {
        // The documentation's wrong cases, then a `const` and a computed name.
        assert_eq!(
            places("promise.finally(async () => {\n\tawait cleanup();\n});"),
            [(1, 17)]
        );
        assert_eq!(
            places("async function cleanup() {}\n\npromise.finally(cleanup);"),
            [(3, 17)]
        );
        assert_eq!(
            places("const cleanup = async function () {};\npromise['finally'](cleanup as never);"),
            [(2, 20)]
        );
        assert_eq!(places("void promise.finally(async () => {});"), [(1, 22)]);
    }

    #[test]
    fn leaves_synchronous_finalizers_and_unknown_callbacks() {
        for code in [
            "promise.finally(() => {\n\tcleanupSynchronously();\n});",
            "function cleanup() {}\n\npromise.finally(cleanup);",
            "promise.finally(() => {\n\treturn cleanup().catch(error => {\n\t\tlogCleanupError(error);\n\t});\n});",
            "import { cleanup } from './a';\npromise.finally(cleanup);",
            "promise.finally(object.cleanup);",
            "promise.finally(async function* () {});",
            "promise.then(async () => {});",
            "promise.finally();",
            "function cleanup(): Promise<void>;\nasync function cleanup() {}\npromise.finally(cleanup);",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
