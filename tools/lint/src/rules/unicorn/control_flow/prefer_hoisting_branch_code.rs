//! `unicorn/prefer-hoisting-branch-code`: the statements every branch of a complete
//! `if`/`else if`/`else` chain starts or ends with, compared token by token, belong before or
//! after it (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-hoisting-branch-code.md`). Each
//! branch keeps a statement of its own; fully identical branches are left to
//! `no-duplicate-if-branches`. Shared trailing code is autofixed; shared leading code only when
//! it and the tests run nothing, as it would then run before the tests. No fix moves a
//! declaration, a multi-line token, code a comment stands by, or code reading a branch's own
//! names.

use oxc_ast::AstKind;
use oxc_ast::ast::{BlockStatement, IfStatement, Statement, VariableDeclarationKind};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{
    edited, has_comment_in, has_side_effect, line_indent, parent, reindent, same_tokens,
    statement_names, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferHoistingBranchCode;

/// The message for shared leading code.
const START: &str = "These branches start with the same code. Move it before the `if` statement.";

/// The message for shared trailing code.
const END: &str = "These branches end with the same code. Move it after the `if` statement.";

/// The blocks of a complete chain, or `None` when it has no last `else` or a branch is no block.
fn blocks<'s, 'a>(first: &'s IfStatement<'a>) -> Option<Vec<&'s BlockStatement<'a>>> {
    let mut found = Vec::new();
    let mut current = first;
    loop {
        let Statement::BlockStatement(block) = &current.consequent else {
            return None;
        };
        found.push(&**block);
        match current.alternate.as_ref()? {
            Statement::IfStatement(next) => current = next,
            Statement::BlockStatement(last) => {
                found.push(last);
                return Some(found);
            }
            _ => return None,
        }
    }
}

/// Whether a statement declares a block-scoped name.
fn declares(statement: &Statement) -> bool {
    match statement {
        Statement::VariableDeclaration(declaration) => {
            declaration.kind != VariableDeclarationKind::Var
        }
        other => other.as_declaration().is_some(),
    }
}

/// The names a branch declares at its top level.
fn own_names<'a>(block: &BlockStatement<'a>) -> Vec<&'a str> {
    block
        .body
        .iter()
        .filter_map(|statement| match statement {
            Statement::VariableDeclaration(declaration) => Some(declaration),
            _ => None,
        })
        .flat_map(|declaration| declaration.declarations.iter())
        .filter_map(|declarator| declarator.id.get_identifier_name())
        .map(|name| name.as_str())
        .collect()
}

/// The shared statements of one side, and their fix when safe.
struct Side<'s, 'a> {
    chain: &'s IfStatement<'a>,
    blocks: &'s [&'s BlockStatement<'a>],
    count: usize,
    leading: bool,
}

impl Side<'_, '_> {
    /// The shared statements of a block.
    fn shared<'b, 'a>(&self, block: &'b BlockStatement<'a>) -> &'b [Statement<'a>] {
        if self.leading {
            &block.body[..self.count]
        } else {
            &block.body[block.body.len() - self.count..]
        }
    }

    /// The range of a block the shared statements are removed from.
    fn removal(&self, block: &BlockStatement) -> Span {
        let body = &block.body;
        if self.leading {
            Span::new(body[0].span().start, body[self.count].span().start)
        } else {
            Span::new(
                body[body.len() - self.count - 1].span().end,
                body[body.len() - 1].span().end,
            )
        }
    }

    /// The fix, when moving is safe.
    fn fix(&self, source: &Source, in_list: bool) -> Option<Fix> {
        let moved = self.shared(self.blocks[0]);
        let multi_line = source.tokens.iter().any(|token| {
            moved
                .iter()
                .any(|each| token.start() >= each.span().start && token.end() <= each.span().end)
                && text(source, token.span()).contains('\n')
        });
        let reads_own = self.blocks.iter().any(|block| {
            let names = own_names(block);
            self.shared(block).iter().any(|each| {
                !names.is_empty()
                    && statement_names(each)
                        .iter()
                        .any(|read| names.contains(read))
            })
        });
        if !in_list
            || moved.iter().any(declares)
            || multi_line
            || reads_own
            || has_comment_in(source, self.chain.span)
        {
            return None;
        }
        if self.leading {
            let mut test_runs = false;
            let mut current = Some(self.chain);
            while let Some(link) = current {
                test_runs |= has_side_effect(&link.test);
                current = match &link.alternate {
                    Some(Statement::IfStatement(next)) => Some(next),
                    _ => None,
                };
            }
            let quiet = moved.iter().all(|each| {
                matches!(each, Statement::ExpressionStatement(statement) if !has_side_effect(&statement.expression))
            });
            if test_runs || !quiet {
                return None;
            }
        }
        let if_indent = line_indent(source, self.chain.span.start);
        let first = moved[0].span();
        let span = Span::new(first.start, moved[moved.len() - 1].span().end);
        let hoisted = reindent(
            text(source, span),
            line_indent(source, first.start),
            if_indent,
        );
        let mut edits: Vec<(u32, u32, String)> = self
            .blocks
            .iter()
            .map(|block| {
                let removed = self.removal(block);
                (removed.start, removed.end, String::new())
            })
            .collect();
        let body = edited(source, self.chain.span, &mut edits);
        let text = if self.leading {
            format!("{hoisted}\n{if_indent}{body}")
        } else {
            format!("{body}\n{if_indent}{hoisted}")
        };
        Some(Fix {
            start: self.chain.span.start,
            end: self.chain.span.end,
            text,
        })
    }
}

impl Rule for PreferHoistingBranchCode {
    fn name(&self) -> &'static str {
        "unicorn/prefer-hoisting-branch-code"
    }

    fn needs_tokens(&self) -> bool {
        true
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::IfStatement(chain) = node.kind() else {
            return;
        };
        let up = parent(source, node.id());
        if matches!(up, AstKind::IfStatement(outer) if outer.alternate.as_ref().is_some_and(|each| each.span() == chain.span))
        {
            return;
        }
        let Some(blocks) = blocks(chain) else {
            return;
        };
        let shortest = blocks
            .iter()
            .map(|block| block.body.len())
            .min()
            .unwrap_or(0);
        let same_at = |index: &dyn Fn(&BlockStatement) -> usize| {
            let first = &blocks[0].body[index(blocks[0])];
            blocks
                .iter()
                .all(|block| same_tokens(source, block.body[index(block)].span(), first.span()))
        };
        let leading = (0..shortest).take_while(|at| same_at(&|_| *at)).count();
        let trailing = (0..shortest - leading)
            .take_while(|back| same_at(&|block: &BlockStatement| block.body.len() - 1 - back))
            .count();
        if (leading == 0 && trailing == 0) || leading + trailing >= shortest {
            return;
        }
        let in_list = matches!(
            up,
            AstKind::Program(_)
                | AstKind::BlockStatement(_)
                | AstKind::FunctionBody(_)
                | AstKind::StaticBlock(_)
                | AstKind::SwitchCase(_)
        );
        for (count, is_leading) in [(leading, true), (trailing, false)] {
            if count == 0 {
                continue;
            }
            let side = Side {
                chain,
                blocks: &blocks,
                count,
                leading: is_leading,
            };
            let at = side.shared(blocks[0])[0].span().start;
            let message = if is_leading { START } else { END };
            findings.push(match side.fix(source, in_list) {
                Some(fix) => source.fixed(self.name(), at, message.to_owned(), fix),
                None => source.finding(self.name(), at, message.to_owned()),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferHoistingBranchCode;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferHoistingBranchCode, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn moves_shared_code_out_of_the_branches() {
        // The documentation's two cases: leading code that calls is reported only.
        let leading = "if (isAdmin) {\n\tlog('access');\n\tshowDashboard();\n} else {\n\tlog('access');\n\tshowLogin();\n}";
        assert_eq!(check(leading), (vec![(2, 2)], leading.to_owned()));
        assert_eq!(
            check(
                "if (isAdmin) {\n\tshowDashboard();\n\tcleanup();\n} else {\n\tshowLogin();\n\tcleanup();\n}"
            ),
            (
                vec![(3, 2)],
                "if (isAdmin) {\n\tshowDashboard();\n} else {\n\tshowLogin();\n}\ncleanup();"
                    .to_owned()
            )
        );
        // Quiet leading code is moved; an `else if` chain is read whole.
        assert_eq!(
            check(
                "if (a) {\n  x;\n  b();\n} else if (c) {\n  x;\n  d();\n} else {\n  x;\n  e();\n}"
            )
            .1,
            "x;\nif (a) {\n  b();\n} else if (c) {\n  d();\n} else {\n  e();\n}"
        );
    }

    #[test]
    fn leaves_incomplete_identical_or_unbraced_chains() {
        for code in [
            "if (a) { b(); c(); } else if (d) { b(); e(); }",
            "if (a) { b(); } else { b(); }",
            "if (a) { b(); c(); } else { b(); }",
            "if (a) b(); else { b(); c(); }",
            "if (a) { b(); c(); } else { d(); e(); }",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_without_fix_when_moving_is_unsafe() {
        for code in [
            "if (a) {\n  b();\n  const c = 1;\n} else {\n  d();\n  const c = 1;\n}",
            "if (a) {\n  const x = 1;\n  b(x);\n  g(x);\n} else {\n  const x = 2;\n  d(x);\n  g(x);\n}",
            "if (a) {\n  b(); // first\n  c();\n} else {\n  d();\n  c();\n}",
            "if (a) {\n  b();\n  c(`x\ny`);\n} else {\n  d();\n  c(`x\ny`);\n}",
            "if (f()) {\n  x;\n  b();\n} else {\n  x;\n  d();\n}",
            "label: if (a) {\n  b();\n  c();\n} else {\n  d();\n  c();\n}",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }
}
