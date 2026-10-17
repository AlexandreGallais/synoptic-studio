//! `unicorn/prefer-early-return`: a function body whose last statement is an `if` without
//! `else` wrapping more than one statement is turned into a guard that returns early, the
//! wrapped statements following it (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-early-return.md`, `maximumStatements: 1`). The condition is negated with
//! `!`, a negation unwrapped. Autofixed when the wrapper is a block declaring nothing, holding no
//! multi-line token, and no comment lies in it or after it in the function.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, FunctionBody, IfStatement, Statement, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{
    has_comment_in, is_parenthesized, line_indent, negated_text, reindent, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferEarlyReturn;

/// The rule's message.
const MESSAGE: &str =
    "Prefer an early return over wrapping the remainder of the function body in an `if` statement.";

/// The number of statements a branch holds, empty ones left out.
fn statement_count(branch: &Statement) -> usize {
    match branch {
        Statement::EmptyStatement(_) => 0,
        Statement::BlockStatement(block) => block
            .body
            .iter()
            .filter(|each| !matches!(each, Statement::EmptyStatement(_)))
            .count(),
        _ => 1,
    }
}

/// The condition of the guard: the test negated, a negation unwrapped.
fn guard_condition(source: &Source, test: &Expression) -> String {
    match test {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            let argument = unary.argument.span();
            if is_parenthesized(source, argument) {
                format!("({})", text(source, argument))
            } else {
                text(source, argument).to_owned()
            }
        }
        other => negated_text(source, other),
    }
}

/// The guard and the unwrapped statements replacing a wrapper, when the move is safe.
fn rewritten(source: &Source, wrapper: &IfStatement, body: &FunctionBody) -> Option<String> {
    let Statement::BlockStatement(block) = &wrapper.consequent else {
        return None;
    };
    let declares = block.body.iter().any(|each| {
        matches!(each, Statement::VariableDeclaration(_)) || each.as_declaration().is_some()
    });
    let multi_line = source
        .tokens
        .iter()
        .filter(|token| token.start() >= block.span.start && token.end() <= block.span.end)
        .any(|token| text(source, token.span()).contains('\n'));
    let after = Span::new(wrapper.span.start, body.span.end);
    if declares || multi_line || has_comment_in(source, after) {
        return None;
    }
    let if_indent = line_indent(source, wrapper.span.start);
    let inside = Span::new(block.span.start + 1, block.span.end - 1);
    let first = block.body.first()?.span().start;
    let body_indent = line_indent(source, first);
    let unit = body_indent
        .strip_prefix(if_indent)
        .filter(|unit| !unit.is_empty())
        .unwrap_or("\t");
    let moved = reindent(text(source, inside), body_indent, if_indent);
    Some(format!(
        "if ({}) {{\n{if_indent}{unit}return;\n{if_indent}}}\n\n{if_indent}{}",
        guard_condition(source, &wrapper.test),
        moved.trim()
    ))
}

impl Rule for PreferEarlyReturn {
    fn name(&self) -> &'static str {
        "unicorn/prefer-early-return"
    }

    fn needs_tokens(&self) -> bool {
        true
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let body = match node.kind() {
            AstKind::Function(function) => function.body.as_deref(),
            AstKind::ArrowFunctionExpression(arrow) => arrow.get_function_body(),
            _ => None,
        };
        let Some(body) = body else {
            return;
        };
        let Some(Statement::IfStatement(wrapper)) = body.statements.last() else {
            return;
        };
        if wrapper.alternate.is_some() || statement_count(&wrapper.consequent) <= 1 {
            return;
        }
        let finding = match rewritten(source, wrapper, body) {
            Some(replacement) => source.fixed(
                self.name(),
                wrapper.span.start,
                MESSAGE.to_owned(),
                Fix {
                    start: wrapper.span.start,
                    end: wrapper.span.end,
                    text: replacement,
                },
            ),
            None => source.finding(self.name(), wrapper.span.start, MESSAGE.to_owned()),
        };
        findings.push(finding);
    }
}

#[cfg(test)]
mod tests {
    use super::PreferEarlyReturn;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferEarlyReturn, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn turns_a_last_wrapper_into_a_guard() {
        // The documentation's case.
        assert_eq!(
            check("function foo() {\n\tdoSomethingBefore();\n\tif (condition) {\n\t\tdoSomething();\n\t\tdoSomethingElse();\n\t}\n}"),
            (
                vec![(3, 2)],
                "function foo() {\n\tdoSomethingBefore();\n\tif (!condition) {\n\t\treturn;\n\t}\n\n\tdoSomething();\n\tdoSomethingElse();\n}".to_owned()
            )
        );
        // A negation unwrapped, a comparison negated whole, an arrow's body.
        assert_eq!(
            check("const f = () => {\n  if (!a) {\n    b();\n    c();\n  }\n};").1,
            "const f = () => {\n  if (a) {\n    return;\n  }\n\n  b();\n  c();\n};"
        );
        assert_eq!(
            check("function f() {\n  if (a < b) {\n    c();\n    d();\n  }\n}").1,
            "function f() {\n  if (!(a < b)) {\n    return;\n  }\n\n  c();\n  d();\n}"
        );
    }

    #[test]
    fn leaves_short_wrappers_else_branches_and_code_after() {
        for code in [
            "function foo() {\n\tif (condition) {\n\t\tdoSomething();\n\t}\n}",
            "function foo() {\n\tif (condition) {\n\t\tdoSomething();\n\t\tdoSomethingElse();\n\t}\n\tfinish();\n}",
            "function foo() { if (a) { b(); c(); } else { d(); } }",
            "function foo() { if (a) { b(); ; } }",
            "const f = () => a;",
            "declare function f(): void;",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_without_fix_when_moving_is_unsafe() {
        for code in [
            "function f() {\n  if (a) {\n    const b = 1;\n    c(b);\n  }\n}",
            "function f() {\n  if (a) {\n    b(); // why\n    c();\n  }\n}",
            "function f() {\n  if (a) {\n    b(`x\ny`);\n    c();\n  }\n}",
            "function f() {\n  if (a) {\n    b();\n    c();\n  }\n  // end\n}",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }
}
