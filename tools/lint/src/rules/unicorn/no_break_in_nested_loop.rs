//! `unicorn/no-break-in-nested-loop`: no unlabeled `break` in a loop or a `switch` that sits in
//! another loop, where its target is easy to misread (`eslint-plugin-unicorn` 78.0.0, its
//! `continue` check left off as configured; `no-continue` refuses every `continue`).

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoBreakInNestedLoop;

/// Whether a node is a loop.
fn is_loop(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::ForStatement(_)
            | AstKind::ForInStatement(_)
            | AstKind::ForOfStatement(_)
            | AstKind::WhileStatement(_)
            | AstKind::DoWhileStatement(_)
    )
}

impl Rule for NoBreakInNestedLoop {
    fn name(&self) -> &'static str {
        "unicorn/no-break-in-nested-loop"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::BreakStatement(statement) = node.kind() else {
            return;
        };
        if statement.label.is_some() {
            return;
        }
        let mut has_target = false;
        for ancestor in source.semantic.nodes().ancestors(node.id()) {
            let kind = ancestor.kind();
            if matches!(
                kind,
                AstKind::Function(_) | AstKind::ArrowFunctionExpression(_)
            ) {
                return;
            }
            if has_target && is_loop(&kind) {
                findings.push(source.finding(
                    self.name(),
                    statement.span.start,
                    "Move this nested loop or switch into a function instead of using `break` here."
                        .to_owned(),
                ));
                return;
            }
            if is_loop(&kind) || matches!(kind, AstKind::SwitchStatement(_)) {
                has_target = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoBreakInNestedLoop;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoBreakInNestedLoop, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_nested_breaks() {
        let message =
            ["Move this nested loop or switch into a function instead of using `break` here."];

        assert_eq!(
            check(
                "for (const item of items) {\n\tfor (const child of item.children) {\n\t\tif (child.done) {\n\t\t\tbreak;\n\t\t}\n\t}\n}"
            ),
            message
        );
        assert_eq!(
            check(
                "for (const item of items) {\n\tswitch (item.type) {\n\t\tcase 'hidden':\n\t\t\tbreak;\n\t\tcase 'visible':\n\t\t\tprocess(item);\n\t\t\tbreak;\n\t}\n}"
            ),
            [message[0], message[0]]
        );
        assert_eq!(check("while (a) { do { break; } while (b); }"), message);
        assert_eq!(
            check("for (;;) { switch (x) { default: for (const y of z) { break; } } }"),
            message
        );
    }

    #[test]
    fn accepts_single_loops_functions_and_labels() {
        for code in [
            "function processChildren(item) {\n\tfor (const child of item.children) {\n\t\tif (child.done) {\n\t\t\tbreak;\n\t\t}\n\t}\n}\nfor (const item of items) {\n\tprocessChildren(item);\n}",
            "for (const item of items) {\n\tfor (const child of item.children) {\n\t\tcheck(child);\n\t}\n}",
            "for (const item of items) { if (item) break; }",
            "switch (x) { case 1: break; }",
            "outer: for (const a of b) { for (const c of d) { break outer; } }",
            "for (const a of b) { items.forEach(() => { for (const c of d) { break; } }); }",
            "for (const a of b) { function f() { switch (x) { case 1: break; } } }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
