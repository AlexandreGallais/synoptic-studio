//! `functional/no-expression-statements`: no statement made of an expression, whose value is
//! thrown away (`eslint-plugin-functional` 10.0.1, `docs/rules/no-expression-statements.md`), with
//! the project's option `ignoreVoid` off, so no type is read: a directive starting with `use `
//! and a `yield` are left, an `await` is not. No fix.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoExpressionStatements;

/// The plugin's message.
const MESSAGE: &str = "Using expressions to cause side-effects not allowed.";

/// Whether a string is a directive the plugin lets through, `"use strict"`.
fn is_directive(value: &str) -> bool {
    value.starts_with("use ")
}

impl Rule for NoExpressionStatements {
    fn name(&self) -> &'static str {
        "functional/no-expression-statements"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let start = match node.kind() {
            AstKind::ExpressionStatement(statement) => match &statement.expression {
                Expression::YieldExpression(_) => return,
                Expression::StringLiteral(literal) if is_directive(&literal.value) => return,
                _ => statement.span.start,
            },
            AstKind::Directive(directive) if !is_directive(&directive.expression.value) => {
                directive.span.start
            }
            _ => return,
        };
        findings.push(source.finding(self.name(), start, MESSAGE.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::NoExpressionStatements;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoExpressionStatements, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_an_expression_whose_value_is_thrown_away() {
        // The documentation's cases, its `ignoreVoid` one (off here), an awaited call and a
        // string that is no `use` directive.
        assert_eq!(places("console.log(\"Hello world!\");"), [(1, 1)]);
        assert_eq!(places("array.push(3);"), [(1, 1)]);
        assert_eq!(places("foo(bar);"), [(1, 1)]);
        assert_eq!(places("console.log(\"hello world\");"), [(1, 1)]);
        assert_eq!(
            places("async function f(x) {\n  await g(x);\n  return 1;\n}"),
            [(2, 3)]
        );
        assert_eq!(places("\"foo\";\nconst a = 1;"), [(1, 1)]);
    }

    #[test]
    fn lets_a_value_kept_a_use_directive_and_a_yield_through() {
        for code in [
            "const baz = foo(bar);",
            "\"use strict\";\nconst a = 1;",
            "function* g(x) {\n  yield x;\n}",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
