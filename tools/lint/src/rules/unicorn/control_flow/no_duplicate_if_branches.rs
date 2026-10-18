//! `unicorn/no-duplicate-if-branches`: two adjacent branches of an `if` chain with the same
//! body, compared token by token, comments, layout, empty statements and a last `;` left out
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-duplicate-if-branches.md`). Empty branches
//! are never the same.

use oxc_ast::AstKind;
use oxc_ast::ast::{IfStatement, Statement};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{parent, same_tokens};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDuplicateIfBranches;

/// A branch's statements, empty statements left out.
fn branch_statements<'s, 'a>(branch: &'s Statement<'a>) -> Vec<&'s Statement<'a>> {
    match branch {
        Statement::BlockStatement(block) => block
            .body
            .iter()
            .filter(|each| !matches!(each, Statement::EmptyStatement(_)))
            .collect(),
        Statement::EmptyStatement(_) => Vec::new(),
        other => vec![other],
    }
}

/// The branches of an `if` chain, in order.
fn branches<'s, 'a>(first: &'s IfStatement<'a>) -> Vec<&'s Statement<'a>> {
    let mut found = Vec::new();
    let mut current = first;
    loop {
        found.push(&current.consequent);
        match &current.alternate {
            Some(Statement::IfStatement(next)) => current = next,
            Some(last) => {
                found.push(last);
                return found;
            }
            None => return found,
        }
    }
}

impl Rule for NoDuplicateIfBranches {
    fn name(&self) -> &'static str {
        "unicorn/no-duplicate-if-branches"
    }

    fn needs_tokens(&self) -> bool {
        true
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::IfStatement(statement) = node.kind() else {
            return;
        };
        if matches!(parent(source, node.id()), AstKind::IfStatement(outer)
            if outer.alternate.as_ref().is_some_and(|alternate| alternate.span() == statement.span))
        {
            return;
        }
        let chain = branches(statement);
        for pair in chain.windows(2) {
            let (left, right) = (branch_statements(pair[0]), branch_statements(pair[1]));
            let same = !left.is_empty()
                && left.len() == right.len()
                && left
                    .iter()
                    .zip(&right)
                    .all(|(a, b)| same_tokens(source, a.span(), b.span()));
            if same {
                let line = source.line(pair[0].span().start);
                findings.push(source.finding(
                    self.name(),
                    pair[1].span().start,
                    format!("This branch has the same body as the branch on line {line}."),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateIfBranches;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoDuplicateIfBranches, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_adjacent_branches_with_the_same_body() {
        // The documentation's two cases.
        let code = "if (isAdmin) {\n\tshowDashboard();\n\tloadStats();\n} else {\n\tshowDashboard();\n\tloadStats();\n}\nif (value === 'a') {\n\tdoThing();\n} else if (value === 'b') {\n\tdoThing();\n}\n";
        assert_eq!(places(code), [(4, 8), (10, 27)]);
        assert_eq!(
            messages(&lint(&NoDuplicateIfBranches, "scripts/a.ts", code))[0],
            "This branch has the same body as the branch on line 1."
        );
        // Layout, comments, empty statements and a last `;` do not count; an unbraced branch does.
        assert_eq!(
            places(
                "if (a) { b( 1 ) /* x */; ; } else b(1)\nif (a) b(); else if (c) d(); else d();"
            ),
            [(1, 35), (2, 35)]
        );
    }

    #[test]
    fn accepts_different_empty_or_distant_branches() {
        for code in [
            "if (isAdmin) {\n\tshowDashboard();\n\tloadStats();\n} else {\n\tshowLogin();\n}",
            "if (value === 'a' || value === 'b') {\n\tdoThing();\n}",
            "if (a) {} else {}",
            "if (a) ; else ;",
            "if (a) { b(); } else if (c) { d(); } else { b(); }",
            "if (a) { b(); c(); } else { b(); }",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
