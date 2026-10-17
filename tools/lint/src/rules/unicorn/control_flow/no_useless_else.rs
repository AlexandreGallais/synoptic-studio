//! `unicorn/no-useless-else`: an `else` after a branch that always leaves — by `return`,
//! `throw`, `break`, `continue` or `process.exit()`, through nested blocks, `if`s, `switch`es and
//! `try`s (`crate::exits`) — is reported at its keyword, `else if` included
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-useless-else.md`). Autofixed when the `else`
//! branch, unwrapped and re-indented to the `if`'s line, declares nothing block-scoped, holds no
//! multi-line token, has no comment before it nor anything after it on its last line, and the
//! `if`'s branch is a block.

use oxc_ast::AstKind;
use oxc_ast::ast::{Declaration, Statement, VariableDeclarationKind};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::exits::always_exits;
use crate::expressions::{has_comment_in, line_indent, offset, parent, reindent, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUselessElse;

/// The rule's message.
const MESSAGE: &str = "Unexpected `else` after a statement that exits.";

/// Whether a statement declares a block-scoped name: `let`, `const`, a class, a function or a
/// TypeScript declaration.
fn is_block_scoped(statement: &Statement) -> bool {
    match statement {
        Statement::VariableDeclaration(declaration) => {
            declaration.kind != VariableDeclarationKind::Var
        }
        other => other
            .as_declaration()
            .is_some_and(|declaration| !matches!(declaration, Declaration::VariableDeclaration(_))),
    }
}

/// The text of the `else` branch moved after the `if`, or `None` when the move is unsafe.
fn moved_text(
    source: &Source,
    consequent: &Statement,
    alternate: &Statement,
    if_start: u32,
) -> Option<String> {
    let Statement::BlockStatement(_) = consequent else {
        return None;
    };
    let after = text(
        source,
        Span::new(alternate.span().end, offset(source.text.len())),
    );
    let rest_of_line = after.split('\n').next().unwrap_or_default();
    if !rest_of_line.trim().is_empty()
        || has_comment_in(
            source,
            Span::new(consequent.span().end, alternate.span().start),
        )
    {
        return None;
    }
    let if_indent = line_indent(source, if_start);
    let Statement::BlockStatement(block) = alternate else {
        return (!is_block_scoped(alternate))
            .then(|| format!("{if_indent}{}", text(source, alternate.span())));
    };
    let multi_line_token = source
        .tokens
        .iter()
        .filter(|token| token.start() >= block.span.start && token.end() <= block.span.end)
        .any(|token| text(source, token.span()).contains('\n'));
    if block.body.iter().any(is_block_scoped) || multi_line_token {
        return None;
    }
    let inside = Span::new(block.span.start + 1, block.span.end - 1);
    let body = text(source, inside);
    let first = body.len() - body.trim_start().len();
    let body_indent = if body.trim().is_empty() {
        ""
    } else {
        line_indent(source, inside.start + offset(first))
    };
    let moved = reindent(body, body_indent, if_indent);
    let moved = moved.trim();
    Some(if moved.is_empty() {
        String::new()
    } else {
        format!("{if_indent}{moved}")
    })
}

impl Rule for NoUselessElse {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-else"
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
        let Some(alternate) = &statement.alternate else {
            return;
        };
        let in_list = matches!(
            parent(source, node.id()),
            AstKind::Program(_)
                | AstKind::BlockStatement(_)
                | AstKind::FunctionBody(_)
                | AstKind::StaticBlock(_)
                | AstKind::SwitchCase(_)
        );
        if !in_list || !always_exits(&statement.consequent) {
            return;
        }
        let between = Span::new(statement.consequent.span().end, alternate.span().start);
        let at = text(source, between)
            .find("else")
            .map_or(between.start, |at| {
                between.start + crate::expressions::offset(at)
            });
        match moved_text(
            source,
            &statement.consequent,
            alternate,
            statement.span.start,
        ) {
            Some(moved) => {
                let fix = Fix {
                    start: between.start,
                    end: alternate.span().end,
                    text: if moved.is_empty() {
                        moved
                    } else {
                        format!("\n{moved}")
                    },
                };
                findings.push(source.fixed(self.name(), at, MESSAGE.to_owned(), fix));
            }
            None => findings.push(source.finding(self.name(), at, MESSAGE.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUselessElse;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoUselessElse, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn moves_the_else_branch_after_an_exit() {
        // The documentation's two cases.
        assert_eq!(
            check("function f() {\n\tif (foo) {\n\t\treturn;\n\t} else {\n\t\tbar();\n\t}\n}"),
            (
                vec![(4, 4)],
                "function f() {\n\tif (foo) {\n\t\treturn;\n\t}\n\tbar();\n}".to_owned()
            )
        );
        assert_eq!(
            check("if (foo) {\n\tthrow new Error();\n} else if (bar) {\n\tbaz();\n}"),
            (
                vec![(3, 3)],
                "if (foo) {\n\tthrow new Error();\n}\nif (bar) {\n\tbaz();\n}".to_owned()
            )
        );
        // A nested exit, an empty `else`, a `switch` case, a loop's `break`.
        assert_eq!(
            check("for (const a of b) {\n  if (a) {\n    if (c) { break; } else { continue; }\n  } else {}\n}").1,
            "for (const a of b) {\n  if (a) {\n    if (c) { break; }\n    continue;\n  }\n}"
        );
        assert_eq!(
            check("function f() {\nswitch (a) {\n  case 1:\n    if (b) { return; } else { c(); }\n}\n}").0,
            [(4, 24)]
        );
    }

    #[test]
    fn reports_without_fix_when_moving_is_unsafe() {
        for code in [
            "function f() {\n  if (a) return; else b();\n}",
            "function f() {\n  if (a) { return; } else { const b = 1; g(b); }\n}",
            "function f() {\n  if (a) { return; } /* why */ else { g(); }\n}",
            "function f() {\n  if (a) { return; } else { g(`x\ny`); }\n}",
            "function f() {\n  if (a) { return; } else { g(); } h();\n}",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }

    #[test]
    fn leaves_branches_that_go_on_and_nested_else_ifs() {
        for code in [
            "if (a) { b(); } else { c(); }",
            "function f() { if (a) { return; } }",
            "const f = () => { if (a) { if (b) { return; } } else { c(); } };",
            "function f() { label: if (a) { return; } else { c(); } }",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        assert_eq!(
            check("function f() {\n  if (a) { return 1; } else if (b) { return 2; } else { return 3; }\n}").0,
            [(2, 24)]
        );
    }
}
