//! `unicorn/prefer-while-loop-condition`: no endless `while (true)`, `for (;;)` or
//! `do … while (true)` whose first statement breaks on a condition: that condition belongs in
//! the `while` (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, Statement, UnaryOperator, VariableDeclarationKind};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{Edit, combined_fix, has_comment, negated_text, statement_removal, text};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferWhileLoopCondition;

/// Whether a test is the literal `true`.
fn is_true(expression: &Expression) -> bool {
    matches!(expression, Expression::BooleanLiteral(literal) if literal.value)
}

/// Whether a statement is a `break` without a label, alone or as the only statement of a block.
fn is_lone_break(statement: &Statement) -> bool {
    match statement {
        Statement::BreakStatement(each) => each.label.is_none(),
        Statement::BlockStatement(block) => {
            matches!(block.body.as_slice(), [Statement::BreakStatement(each)] if each.label.is_none())
        }
        _ => false,
    }
}

/// Whether a statement holds an unlabeled `break` of the loop around it: not inside a nested
/// loop, switch or function.
fn breaks_loop(source: &Source, statement: &Statement) -> bool {
    let id = statement.node_id();
    std::iter::once(source.semantic.nodes().get_node(id))
        .chain(super::syntax::descendants(source, id))
        .any(|node| {
            matches!(node.kind(), AstKind::BreakStatement(each) if each.label.is_none())
                && !super::syntax::path_up(source, node.id(), id)
                    .skip(1)
                    .any(|step| {
                        matches!(
                            step.kind(),
                            AstKind::ForStatement(_)
                                | AstKind::ForInStatement(_)
                                | AstKind::ForOfStatement(_)
                                | AstKind::WhileStatement(_)
                                | AstKind::DoWhileStatement(_)
                                | AstKind::SwitchStatement(_)
                                | AstKind::Function(_)
                                | AstKind::ArrowFunctionExpression(_)
                        )
                    })
        })
}

/// Whether the test reads a `let` or `const` declared in the loop's body, which the `while`
/// head could not see.
fn reads_body_binding(source: &Source, test: &Expression, body: Span) -> bool {
    let test_id = test.node_id();
    std::iter::once(source.semantic.nodes().get_node(test_id))
        .chain(super::syntax::descendants(source, test_id))
        .any(|node| {
            let AstKind::IdentifierReference(identifier) = node.kind() else {
                return false;
            };
            let Some(symbol) = super::syntax::symbol_of(source, identifier) else {
                return false;
            };
            let scoping = source.semantic.scoping();
            let declared = scoping.symbol_span(symbol);
            let var = matches!(
                source.semantic.nodes().parent_kind(scoping.symbol_declaration(symbol)),
                AstKind::VariableDeclaration(declaration) if declaration.kind == VariableDeclarationKind::Var
            );
            !var && declared.start >= body.start && declared.end <= body.end
        })
}

/// The condition the loop keeps running on: the test's operand when it is negated, its negation
/// otherwise.
fn condition(source: &Source, test: &Expression) -> String {
    match test {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            text(source, unary.argument.span()).to_owned()
        }
        _ => negated_text(source, test),
    }
}

impl Rule for PreferWhileLoopCondition {
    fn name(&self) -> &'static str {
        "unicorn/prefer-while-loop-condition"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (body, span, endless) = match node.kind() {
            AstKind::WhileStatement(statement) => {
                (&statement.body, statement.span, is_true(&statement.test))
            }
            AstKind::DoWhileStatement(statement) => {
                (&statement.body, statement.span, is_true(&statement.test))
            }
            AstKind::ForStatement(statement) => (
                &statement.body,
                statement.span,
                statement.init.is_none()
                    && statement.update.is_none()
                    && statement.test.as_ref().is_none_or(is_true),
            ),
            _ => return,
        };
        let labeled = matches!(
            source.semantic.nodes().parent_kind(node.id()),
            AstKind::LabeledStatement(_)
        );
        let Statement::BlockStatement(block) = body else {
            return;
        };
        if !endless || labeled {
            return;
        }
        let Some((Statement::IfStatement(first), rest)) = block.body.split_first() else {
            return;
        };
        if first.alternate.is_some()
            || !is_lone_break(&first.consequent)
            || rest.iter().any(|each| breaks_loop(source, each))
            || reads_body_binding(source, &first.test, block.span)
        {
            return;
        }
        let message = "Prefer putting the condition in the `while` statement.".to_owned();
        let lossy = has_comment(source, Span::new(span.start, first.span.end))
            || has_comment(source, Span::new(block.span.end, span.end));
        let condition = condition(source, &first.test);
        let mut edits = vec![Edit {
            span: statement_removal(source, first.span),
            text: String::new(),
        }];
        match node.kind() {
            AstKind::WhileStatement(statement) => edits.push(Edit {
                span: statement.test.span(),
                text: condition,
            }),
            AstKind::ForStatement(_) => edits.push(Edit {
                span: Span::new(span.start, block.span.start),
                text: format!("while ({condition}) "),
            }),
            _ => {
                edits.push(Edit {
                    span: Span::new(span.start, span.start + 2),
                    text: format!("while ({condition})"),
                });
                edits.push(Edit {
                    span: Span::new(block.span.end, span.end),
                    text: String::new(),
                });
            }
        }
        let fix = (!lossy).then(|| combined_fix(source, edits)).flatten();
        findings.push(match fix {
            Some(fix) => source.fixed(self.name(), first.span.start, message, fix),
            None => source.finding(self.name(), first.span.start, message),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::PreferWhileLoopCondition;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferWhileLoopCondition, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferWhileLoopCondition, "playground/a.ts", code),
        )
    }

    #[test]
    fn reports_and_fixes_the_documented_loops() {
        assert_eq!(
            check("while (true) {\n\tif (!hasMore()) {\n\t\tbreak;\n\t}\n\tprocessNext();\n}"),
            ["Prefer putting the condition in the `while` statement."]
        );
        let cases = [
            (
                "while (true) {\n\tif (!hasMore()) {\n\t\tbreak;\n\t}\n\tprocessNext();\n}",
                "while (hasMore()) {\n\tprocessNext();\n}",
            ),
            (
                "do {\n\tif (!hasMore()) {\n\t\tbreak;\n\t}\n\tprocessNext();\n} while (true);",
                "while (hasMore()) {\n\tprocessNext();\n}",
            ),
            (
                "while (true) {\n\tif (done) {\n\t\tbreak;\n\t}\n\tprocessNext();\n}",
                "while (!done) {\n\tprocessNext();\n}",
            ),
            (
                "for (;;) {\n\tif (!hasMore()) {\n\t\tbreak;\n\t}\n\tprocessNext();\n}",
                "while (hasMore()) {\n\tprocessNext();\n}",
            ),
            (
                "for (; true;) {\n\tif (a || b) break;\n\tstep();\n}",
                "while (!(a || b)) {\n\tstep();\n}",
            ),
        ];
        for (code, after) in cases {
            assert_eq!(fix(code), after, "{code}");
        }
    }

    #[test]
    fn accepts_loops_it_cannot_lift_the_condition_of() {
        for code in [
            "while (hasMore()) {\n\tprocessNext();\n}",
            "while (true) { step(); if (done) break; }",
            "while (true) { if (done) break; else step(); }",
            "while (true) { if (done) { log(); break; } }",
            "while (true) { if (done) break; if (other) break; }",
            "while (true) { if (x) break; const x = next(); }",
            "label: while (true) { if (done) break; }",
            "while (true) { if (done) break label; }",
            "while (x) { if (done) break; }",
            "for (let i = 0;;) { if (done) break; }",
            "do { if (done) break; } while (x);",
            "while (true) if (done) break;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
        // A break inside a nested loop or a function leaves the rule its say.
        assert_eq!(
            check("while (true) { if (done) break; for (const a of b) { break; } items.forEach(() => {}); }").len(),
            1
        );
    }
}
