//! `unicorn/no-array-front-mutation`: no `Array#shift()` nor `Array#unshift()`, which consume or
//! grow an array from its front (`eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::syntax::{is_path, method_call, skips_non_array_receiver};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoArrayFrontMutation;

/// The stream-style `unshift` calls left alone, by their path.
const STREAM_PATHS: [&str; 6] = [
    "stream",
    "this",
    "this.stream",
    "process.stdin",
    "process.stdout",
    "process.stderr",
];

impl Rule for NoArrayFrontMutation {
    fn name(&self) -> &'static str {
        "unicorn/no-array-front-mutation"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call)
            .filter(|method| matches!(method.name, "shift" | "unshift") && !method.call.optional)
        else {
            return;
        };
        let stream = method.name == "unshift"
            && STREAM_PATHS.iter().any(|path| is_path(method.object, path));
        if stream || skips_non_array_receiver(source, method.object) {
            return;
        }
        findings.push(source.finding(
            self.name(),
            method.property.start,
            format!(
                "Avoid front-of-array mutation with `Array#{}()`.",
                method.name
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoArrayFrontMutation;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoArrayFrontMutation, "playground/a.ts", code))
    }

    #[test]
    fn reports_shift_and_unshift() {
        assert_eq!(
            check("array.shift();"),
            ["Avoid front-of-array mutation with `Array#shift()`."]
        );
        assert_eq!(
            check("array.unshift(item);"),
            ["Avoid front-of-array mutation with `Array#unshift()`."]
        );
        assert_eq!(check("array?.shift();").len(), 1);
        assert_eq!(check("const a = [1]; a.shift();").len(), 1);
        assert_eq!(check("[1, 2].shift();").len(), 1);
        assert_eq!(check("this.stream.shift();").len(), 1);
    }

    #[test]
    fn accepts_iteration_queues_streams_and_other_receivers() {
        for code in [
            "for (const item of array) { process(item); }",
            "queue.enqueue(item); queue.dequeue();",
            "array.shift?.();",
            "array[\"shift\"]();",
            "stream.unshift(chunk);",
            "this.unshift(chunk);",
            "this.stream.unshift(chunk);",
            "process.stdin.unshift(chunk);",
            "process.stdout.unshift(chunk);",
            "process.stderr.unshift(chunk);",
            "const set = new Set(); set.shift();",
            "const queue = new Queue(); queue.shift();",
            "function f(items: ReadonlyMap<string, number>) { items.shift(); }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
