//! `unicorn/prefer-block-statement-over-iife`: a function called at once, as a statement, with
//! no argument, only to scope its body, becomes a block (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-block-statement-over-iife.md`). Left when it is named, async, a generator,
//! takes parameters, holds a directive, returns, declares with `var`, calls `eval`, or — a
//! `function` — reads `this`, `arguments` or `new.target`. Autofixed unless a comment stands
//! outside its body.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, FunctionBody};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::AstNode;
use oxc_span::Span;

use crate::expressions::{has_comment_in, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferBlockStatementOverIife;

/// The rule's message.
const MESSAGE: &str = "Prefer a block statement over an IIFE used only for scoping.";

/// What a body holds that only a function gives: a `return`, a `var`, a direct `eval`, and, for
/// a `function`, `this`, `arguments` or `new.target`.
#[derive(Default)]
struct FunctionOnly {
    depth: usize,
    arrow: bool,
    returns: bool,
    context: bool,
}

impl<'a> Visit<'a> for FunctionOnly {
    fn visit_return_statement(&mut self, statement: &oxc_ast::ast::ReturnStatement<'a>) {
        self.returns |= self.depth == 0;
        walk::walk_return_statement(self, statement);
    }

    fn visit_variable_declaration(&mut self, declaration: &oxc_ast::ast::VariableDeclaration<'a>) {
        self.returns |=
            self.depth == 0 && declaration.kind == oxc_ast::ast::VariableDeclarationKind::Var;
        walk::walk_variable_declaration(self, declaration);
    }

    fn visit_expression(&mut self, expression: &Expression<'a>) {
        let eval = matches!(expression, Expression::CallExpression(call)
            if call.callee.is_specific_id("eval") && self.depth == 0);
        let context = match expression {
            Expression::ThisExpression(_) | Expression::NewTarget(_) => true,
            Expression::Identifier(name) => name.name == "arguments",
            _ => false,
        };
        self.returns |= eval;
        self.context |= context && !self.arrow;
        walk::walk_expression(self, expression);
    }

    fn visit_function(
        &mut self,
        _function: &oxc_ast::ast::Function<'a>,
        _flags: oxc_semantic::ScopeFlags,
    ) {
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        self.depth += 1;
        walk::walk_arrow_function_expression(self, arrow);
        self.depth -= 1;
    }
}

/// The block body of a function called at once with no argument, when it only scopes.
fn scoping_body<'a>(callee: &'a Expression<'a>) -> Option<&'a FunctionBody<'a>> {
    let (body, arrow) = match callee {
        Expression::ArrowFunctionExpression(each)
            if !each.r#async && each.params.items.is_empty() && each.params.rest.is_none() =>
        {
            (each.get_function_body()?, true)
        }
        Expression::FunctionExpression(each)
            if each.id.is_none()
                && !each.r#async
                && !each.generator
                && each.params.items.is_empty()
                && each.params.rest.is_none() =>
        {
            (each.body.as_deref()?, false)
        }
        _ => return None,
    };
    if !body.directives.is_empty() {
        return None;
    }
    let mut search = FunctionOnly {
        arrow,
        ..FunctionOnly::default()
    };
    search.visit_function_body(body);
    (!search.returns && !search.context).then_some(body)
}

impl Rule for PreferBlockStatementOverIife {
    fn name(&self) -> &'static str {
        "unicorn/prefer-block-statement-over-iife"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ExpressionStatement(statement) = node.kind() else {
            return;
        };
        let Expression::CallExpression(call) = &statement.expression else {
            return;
        };
        if call.optional || !call.arguments.is_empty() {
            return;
        }
        let Some(body) = scoping_body(&call.callee) else {
            return;
        };
        let outside = [
            Span::new(statement.span.start, body.span.start),
            Span::new(body.span.end, statement.span.end),
        ];
        if outside.iter().any(|span| has_comment_in(source, *span)) {
            findings.push(source.finding(self.name(), call.span.start, MESSAGE.to_owned()));
            return;
        }
        let fix = Fix {
            start: statement.span.start,
            end: statement.span.end,
            text: text(source, body.span).to_owned(),
        };
        findings.push(source.fixed(self.name(), call.span.start, MESSAGE.to_owned(), fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferBlockStatementOverIife;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferBlockStatementOverIife, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn turns_a_scoping_iife_into_a_block() {
        // The documentation's two cases.
        assert_eq!(
            check("(() => {\n\tconst value = getValue();\n\trun(value);\n})();"),
            (
                vec![(1, 1)],
                "{\n\tconst value = getValue();\n\trun(value);\n}".to_owned()
            )
        );
        assert_eq!(
            check("(function () {\n\tconst value = getValue();\n\trun(value);\n})();").1,
            "{\n\tconst value = getValue();\n\trun(value);\n}"
        );
        // An arrow may read `this`; a nested function may return.
        assert_eq!(
            check("(() => { g(this); const f = function () { return 1; }; })();").0,
            [(1, 1)]
        );
    }

    #[test]
    fn leaves_iifes_that_need_their_function() {
        for code in [
            "const value = (() => {\n\treturn getValue();\n})();",
            "(async () => {\n\tawait run();\n})();",
            "(() => { if (a) { return; } })();",
            "((a) => { g(a); })(1);",
            "(() => { g(); })(1);",
            "(function named() { g(); })();",
            "(function () { g(this); })();",
            "(function () { g(arguments); })();",
            "(function () { g(new.target); })();",
            "(function* () { g(); })();",
            "(() => { var a = 1; })();",
            "(() => { eval('a'); })();",
            "(function () { 'use strict'; g(); })();",
            "(() => g())();",
            "(() => { g(); })?.();",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let commented = "(() => { g(); })(/* now */);";
        assert_eq!(check(commented), (vec![(1, 1)], commented.to_owned()));
    }
}
