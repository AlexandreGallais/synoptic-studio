//! `unicorn/prefer-short-arrow-method`: an object literal's method whose body is a lone
//! `return` becomes an arrow property (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-short-arrow-method.md`, `'always'`). Left when it is a generator, named
//! `__proto__`, takes `this`, or reads `this`, `arguments`, `super`, `new.target` or `eval`,
//! which an arrow would change. Autofixed unless it is generic or holds a comment.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, Function, ObjectProperty, PropertyKind, Statement};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{has_comment_in, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferShortArrowMethod;

/// The rule's message.
const MESSAGE: &str = "Prefer an arrow function property over a method with a single return.";

/// The search of a body for what an arrow would read differently.
#[derive(Default)]
struct Context {
    found: bool,
}

impl<'a> Visit<'a> for Context {
    fn visit_expression(&mut self, expression: &Expression<'a>) {
        self.found |= match expression {
            Expression::ThisExpression(_) | Expression::Super(_) | Expression::NewTarget(_) => true,
            Expression::Identifier(name) => name.name == "arguments" || name.name == "eval",
            _ => false,
        };
        walk::walk_expression(self, expression);
    }

    fn visit_function(&mut self, _function: &Function<'a>, _flags: oxc_semantic::ScopeFlags) {}
}

/// The value a method returns, when it can be an arrow.
fn convertible<'s, 'a>(
    property: &'s ObjectProperty<'a>,
) -> Option<(&'s Function<'a>, &'s Expression<'a>)> {
    let Expression::FunctionExpression(function) = &property.value else {
        return None;
    };
    let prototype = !property.computed
        && property
            .key
            .static_name()
            .is_some_and(|name| name == "__proto__");
    if !property.method
        || property.kind != PropertyKind::Init
        || function.generator
        || prototype
        || function.this_param.is_some()
    {
        return None;
    }
    let body = function.body.as_ref()?;
    let mut search = Context::default();
    search.visit_function_body(body);
    if search.found {
        return None;
    }
    match body.statements.as_slice() {
        [Statement::ReturnStatement(statement)] => Some((function, statement.argument.as_ref()?)),
        _ => None,
    }
}

impl Rule for PreferShortArrowMethod {
    fn name(&self) -> &'static str {
        "unicorn/prefer-short-arrow-method"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ObjectProperty(property) = node.kind() else {
            return;
        };
        let Some((function, value)) = convertible(property) else {
            return;
        };
        if function.type_parameters.is_some() || has_comment_in(source, property.span) {
            findings.push(source.finding(self.name(), property.span.start, MESSAGE.to_owned()));
            return;
        }
        let key = text(source, property.key.span());
        let key = if property.computed {
            format!("[{key}]")
        } else {
            key.to_owned()
        };
        let parameters = &function.params;
        let inside = Span::new(parameters.span.start + 1, parameters.span.end - 1);
        let returns = function
            .return_type
            .as_ref()
            .map_or("", |written| text(source, written.span));
        let body = text(source, value.span());
        let wrapped = body.starts_with('{')
            || matches!(
                value,
                Expression::SequenceExpression(_) | Expression::AssignmentExpression(_)
            );
        let body = if wrapped {
            format!("({body})")
        } else {
            body.to_owned()
        };
        let fix = Fix {
            start: property.span.start,
            end: property.span.end,
            text: format!(
                "{key}: {}({}){returns} => {body}",
                if function.r#async { "async " } else { "" },
                text(source, inside)
            ),
        };
        findings.push(source.fixed(self.name(), property.span.start, MESSAGE.to_owned(), fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferShortArrowMethod;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferShortArrowMethod, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn turns_a_returning_method_into_an_arrow() {
        // The documentation's three cases.
        assert_eq!(
            check("const api = {\n\tgetUrl() {\n\t\treturn 'https://api.example.com';\n\t},\n};"),
            (
                vec![(2, 2)],
                "const api = {\n\tgetUrl: () => 'https://api.example.com',\n};".to_owned()
            )
        );
        assert_eq!(
            check("const calculator = {\n\tadd(a, b) {\n\t\treturn a + b;\n\t},\n};").1,
            "const calculator = {\n\tadd: (a, b) => a + b,\n};"
        );
        assert_eq!(
            check("const asyncApi = {\n\tasync fetchData(url) {\n\t\treturn fetch(url);\n\t},\n};")
                .1,
            "const asyncApi = {\n\tfetchData: async (url) => fetch(url),\n};"
        );
        // A computed key, a return type, an object returned.
        assert_eq!(
            check("const o = { [k](a: number): T { return { a }; } };").1,
            "const o = { [k]: (a: number): T => ({ a }) };"
        );
    }

    #[test]
    fn leaves_methods_an_arrow_would_change() {
        for code in [
            "const user = {\n\tgetName() {\n\t\treturn this.firstName + ' ' + this.lastName;\n\t},\n\tgetRole() {\n\t\treturn this.role;\n\t},\n};",
            "const o = { a() { g(); return 1; } };",
            "const o = { *a() { return 1; } };",
            "const o = { a() { return arguments; } };",
            "const o = { a() { return super.b; } };",
            "const o = { a() { return new.target; } };",
            "const o = { a(this: T) { return 1; } };",
            "const o = { __proto__() { return 1; } };",
            "const o = { a: function () { return 1; } };",
            "const o = { get a() { return 1; } };",
            "const o = { a() { return; } };",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        for code in [
            "const o = { a<T>(b: T) { return b; } };",
            "const o = { a() { return 1; /* one */ } };",
        ] {
            assert_eq!(check(code), (vec![(1, 13)], code.to_owned()), "{code}");
        }
    }
}
