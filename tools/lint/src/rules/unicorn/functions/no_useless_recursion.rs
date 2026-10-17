//! `unicorn/no-useless-recursion`: a named function returning a direct call of itself, which a
//! loop could replace without growing the stack (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-useless-recursion.md`). Generators, calls under a `try`, and calls after a
//! `using` declaration are left; so is recursion through a callback. No fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, Statement, VariableDeclarationKind};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::symbol;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUselessRecursion;

/// Whether a statement declares with `using` or `await using`.
fn is_using(statement: &Statement) -> bool {
    matches!(statement, Statement::VariableDeclaration(declaration)
        if matches!(declaration.kind, VariableDeclarationKind::Using | VariableDeclarationKind::AwaitUsing))
}

impl Rule for NoUselessRecursion {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-recursion"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ReturnStatement(statement) = node.kind() else {
            return;
        };
        let Some(Expression::CallExpression(call)) = &statement.argument else {
            return;
        };
        let Expression::Identifier(callee) = &call.callee else {
            return;
        };
        if call.optional {
            return;
        }
        let nodes = source.semantic.nodes();
        let mut previous = node.id();
        for ancestor in nodes.ancestors(node.id()) {
            match ancestor.kind() {
                AstKind::ForStatement(each) if each.init.as_ref().is_some_and(|init| {
                    matches!(init, oxc_ast::ast::ForStatementInit::VariableDeclaration(declaration)
                        if matches!(declaration.kind, VariableDeclarationKind::Using | VariableDeclarationKind::AwaitUsing))
                }) => return,
                AstKind::ForOfStatement(each) if matches!(&each.left, oxc_ast::ast::ForStatementLeft::VariableDeclaration(declaration)
                    if matches!(declaration.kind, VariableDeclarationKind::Using | VariableDeclarationKind::AwaitUsing)) => return,
                AstKind::BlockStatement(_) | AstKind::FunctionBody(_) => {
                    let list = crate::expressions::statements(ancestor.kind()).unwrap_or_default();
                    let before = nodes.kind(previous).span().start;
                    if list.iter().take_while(|each| each.span().start < before).any(is_using) {
                        return;
                    }
                }
                AstKind::Function(function) => {
                    let own = function.id.as_ref().and_then(|id| id.symbol_id.get());
                    if !function.generator && own.is_some() && symbol(source, callee) == own {
                        findings.push(source.finding(self.name(), callee.span.start, "Use a loop instead of this recursive function call.".to_owned()));
                    }
                    return;
                }
                AstKind::ArrowFunctionExpression(_) | AstKind::TryStatement(_) => return,
                _ => {}
            }
            previous = ancestor.id();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUselessRecursion;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoUselessRecursion, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_returned_self_call() {
        // The documentation's case.
        assert_eq!(
            places(
                "function foo(bar) {\n\tif (bar.baz) {\n\t\treturn foo(bar.baz);\n\t}\n\treturn bar;\n}"
            ),
            [(3, 10)]
        );
        assert_eq!(
            places("const f = function g(n) { return g(n - 1); };"),
            [(1, 34)]
        );
    }

    #[test]
    fn leaves_callbacks_other_calls_and_guarded_returns() {
        for code in [
            "function foo(bar) {\n\tif (Array.isArray(bar)) {\n\t\treturn bar.map(baz => foo(baz));\n\t}\n\treturn bar;\n}",
            "function foo(bar) { return 1 + foo(bar); }",
            "function foo(bar) { return other(bar); }",
            "function* foo(bar) { return foo(bar); }",
            "function foo(bar) { try { return foo(bar); } catch { return 1; } }",
            "function foo(bar) { using a = b; return foo(bar); }",
            "function foo(bar) { for (using a of b) { return foo(bar); } }",
            "function foo(bar) { const foo = g; return foo(bar); }",
            "const foo = (bar) => { return foo(bar); };",
            "function foo(bar) { return foo?.(bar); }",
            "function foo(bar) { return; }",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
