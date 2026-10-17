//! `functional/no-conditional-statements`: in the functional core, a choice is an expression; an
//! `if` or a `switch` is kept only when each of its branches leaves — returns, throws, breaks or
//! continues (`REF-FUNCTIONAL-PLUGIN`; the project's `allowReturningBranches: true`). The plugin
//! also asked the compiler whether a branch's last call returns `never`; without types, a branch
//! is read from the tree alone: a call to a function that never returns does not end it.

use oxc_ast::AstKind;
use oxc_ast::ast::{Statement, SwitchCase};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoConditionalStatements;

/// The rule's message.
const MESSAGE: &str =
    "Incomplete branch, every branch in a conditional statement must contain a return statement.";

/// Whether a statement ends an `if`'s branch; a nested `if` is checked on its own.
fn leaves_if(statement: &Statement) -> bool {
    matches!(
        statement,
        Statement::IfStatement(_)
            | Statement::ReturnStatement(_)
            | Statement::ThrowStatement(_)
            | Statement::BreakStatement(_)
            | Statement::ContinueStatement(_)
    )
}

/// Whether an `if`'s branch leaves: it is such a statement, or a block holding one.
fn if_branch_leaves(branch: &Statement) -> bool {
    match branch {
        Statement::BlockStatement(block) => block.body.iter().any(leaves_if),
        other => leaves_if(other),
    }
}

/// Whether a statement ends a `switch`'s case: a `break` only when it leaves another statement
/// than this `switch`, by its label; a nested `switch` is checked on its own.
fn leaves_switch(statement: &Statement, label: Option<&str>) -> bool {
    match statement {
        Statement::SwitchStatement(_)
        | Statement::ReturnStatement(_)
        | Statement::ThrowStatement(_)
        | Statement::ContinueStatement(_) => true,
        Statement::BreakStatement(stop) => stop
            .label
            .as_ref()
            .is_some_and(|own| Some(own.name.as_str()) != label),
        _ => false,
    }
}

/// Whether a `switch`'s case leaves, or falls through having no statement.
fn case_leaves(case: &SwitchCase, label: Option<&str>) -> bool {
    let statements = &case.consequent;
    if statements.is_empty() || statements.iter().any(|each| leaves_switch(each, label)) {
        return true;
    }
    let all_blocks = statements
        .iter()
        .all(|each| matches!(each, Statement::BlockStatement(_)));
    all_blocks
        && matches!(statements.last(), Some(Statement::BlockStatement(last))
            if last.body.iter().any(|each| leaves_switch(each, label)))
}

impl Rule for NoConditionalStatements {
    fn name(&self) -> &'static str {
        "functional/no-conditional-statements"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let mut report = |at: u32| {
            findings.push(source.finding(self.name(), at, MESSAGE.to_owned()));
        };
        match node.kind() {
            AstKind::IfStatement(statement) => {
                for branch in std::iter::once(&statement.consequent).chain(&statement.alternate) {
                    if !if_branch_leaves(branch) {
                        report(branch.span().start);
                    }
                }
            }
            AstKind::SwitchStatement(statement) => {
                let label = match source.semantic.nodes().parent_kind(node.id()) {
                    AstKind::LabeledStatement(labeled) => Some(labeled.label.name.as_str()),
                    _ => None,
                };
                for case in &statement.cases {
                    if !case_leaves(case, label) {
                        report(case.span.start);
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoConditionalStatements;
    use crate::testing::lint;

    /// The places of the findings on a piece of the functional core.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoConditionalStatements, "src/math/a.ts", code)
            .iter()
            .map(|finding| (finding.line, finding.column))
            .collect()
    }

    #[test]
    fn reports_a_branch_that_does_not_leave() {
        // [F19.AC2] Each branch returns or throws, read from the tree.
        assert_eq!(
            places(
                "function f(a: number): number {\n  if (a > 0) { g(a); }\n  if (a < 0) return 1;\n  else g(a);\n  return 0;\n}"
            ),
            [(2, 14), (4, 8)]
        );
        // A call that never returns is not read: its branch is incomplete.
        assert_eq!(
            places("function f(a: number): number {\n  if (a) { fail(a); }\n  return 0;\n}"),
            [(2, 10)]
        );
    }

    #[test]
    fn accepts_branches_that_return_throw_break_or_nest() {
        for code in [
            "function f(a: number): number { if (a) { return 1; } else { throw new Error(); } }",
            "function f(a: number): number { if (a) return 1; return 2; }",
            "function f(a: number): number { if (a) { if (a > 1) { return 1; } } return 2; }",
            "function f(a: number): number { if (a) { const b = a; return b; } else if (a > 2) { return 3; } return 2; }",
            "function f(a: number[]): void { for (const b of a) { if (b) { continue; } if (b > 1) { break; } } }",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_a_case_that_neither_leaves_nor_falls_through() {
        let code = "function f(a: string): number {\n  outer: switch (a) {\n    case \"a\":\n    case \"b\":\n      return 1;\n    case \"c\": { g(); return 2; }\n    case \"d\": g(); break;\n    case \"e\": break outer;\n    case \"f\": { g(); } { return 3; }\n    case \"g\": throw new Error();\n    default: { g(); }\n  }\n  return 0;\n}";

        assert_eq!(places(code), [(7, 5), (8, 5), (11, 5)]);
        // A statement, then a block that leaves: the plugin reads blocks only when every statement is one.
        assert_eq!(
            places(
                "function f(a: string): number {\n  switch (a) {\n    case \"x\": g(); { return 1; }\n    default: return 0;\n  }\n}"
            ),
            [(3, 5)]
        );
        assert_eq!(
            places(
                "function f(a: string): number { x: for (;;) { switch (a) { case \"a\": break x; default: return 1; } } }"
            ),
            Vec::<(usize, usize)>::new()
        );
    }
}
