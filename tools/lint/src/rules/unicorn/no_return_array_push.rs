//! `unicorn/no-return-array-push`: no use of what `Array#push()` or `Array#unshift()` returns,
//! the new length rather than the value or the array (`eslint-plugin-unicorn` 78.0.0). Its fix
//! is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::UnaryOperator;
use oxc_semantic::AstNode;

use super::syntax::{Kind, is_known_non_indexed, is_path, kind_of, method_call, outer_parent};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoReturnArrayPush;

/// The `push` calls of streams and routers left alone, by their receiver's path.
const IGNORED_PUSH_RECEIVERS: [&str; 9] = [
    "stream",
    "router",
    "this",
    "this.router",
    "this.$router",
    "this.stream",
    "process.stdin",
    "process.stdout",
    "process.stderr",
];

impl Rule for NoReturnArrayPush {
    fn name(&self) -> &'static str {
        "unicorn/no-return-array-push"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call).filter(|method| {
            matches!(method.name, "push" | "unshift") && !call.arguments.is_empty()
        }) else {
            return;
        };
        let ignored_receiver = method.name == "push"
            && IGNORED_PUSH_RECEIVERS
                .iter()
                .any(|path| is_path(method.object, path))
            && kind_of(source, method.object) != Some(Kind::Array);
        let (parent, child) = outer_parent(source, node.id());
        let used = match parent {
            AstKind::ExpressionStatement(_) => false,
            AstKind::UnaryExpression(unary) => unary.operator != UnaryOperator::Void,
            AstKind::StaticMemberExpression(member) => member.object.node_id() != child,
            AstKind::ComputedMemberExpression(member) => member.object.node_id() != child,
            _ => true,
        };
        if used && !ignored_receiver && !is_known_non_indexed(source, method.object) {
            findings.push(source.finding(
                self.name(),
                method.property.start,
                format!("Do not use the return value of `.{}(…)`.", method.name),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoReturnArrayPush;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoReturnArrayPush, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_uses_of_the_length() {
        let push = ["Do not use the return value of `.push(…)`."];

        assert_eq!(
            check("function add(item) { return items.push(item); }"),
            push
        );
        assert_eq!(check("const add = item => items.push(item);"), push);
        assert_eq!(
            check("const length = items.unshift(item);"),
            ["Do not use the return value of `.unshift(…)`."]
        );
        assert_eq!(check("source.forEach(item => target.push(item));"), push);
        assert_eq!(check("const n = items?.push(item);"), push);
        assert_eq!(check("const n = (items.push(item) as number);"), push);
        assert_eq!(check("const n = !items.push(item);"), push);
        assert_eq!(check("const a: number[] = []; const n = a.push(1);"), push);
        assert_eq!(
            check("const stream: number[] = []; const n = stream.push(1);"),
            push
        );
        assert_eq!(check("const n = x[items.push(item)];"), push);
    }

    #[test]
    fn accepts_statements_void_chained_results_and_other_receivers() {
        for code in [
            "function add(item) { items.push(item); return; }",
            "const add = item => { items.push(item); };",
            "items.unshift(item); const length = items.length;",
            "function getNextLength(item) { items.push(item); return items.length; }",
            "void items.push(item);",
            "items?.push(item);",
            "items.push(item)!;",
            "const n = items.push();",
            "router.push(to).catch(handle);",
            "const p = router.push(to);",
            "const p = this.$router.push(to);",
            "const p = process.stdout.push(chunk);",
            "const set = new Set(); const n = set.push(1);",
            "const n = items.add(item);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
