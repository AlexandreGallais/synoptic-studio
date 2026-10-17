//! `unicorn/prefer-promise-static-methods`: `new Promise(…)` whose executor only resolves or only
//! rejects with no value or a simple one, written `Promise.resolve(…)` or `Promise.reject(…)`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-promise-static-methods.md`). A simple value
//! is a name, a literal or a template without expressions, not a parameter of the executor.
//! Autofixed unless a type is written on the executor, `resolve` follows type arguments, or a
//! comment would go.

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, Expression, FormalParameters};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::calls::{Body, promise_executor};
use crate::expressions::{has_comment_in, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferPromiseStaticMethods;

/// An executor read as the rule reads it.
struct Executor<'s, 'a> {
    /// Its parameters.
    params: &'s FormalParameters<'a>,
    /// Its body.
    body: Body<'s, 'a>,
    /// Whether a type is written on it: a parameter's, its return's, or type parameters.
    typed: bool,
    /// Its own name, for a named function.
    name: Option<&'a str>,
    /// Whether it is a `function`, where `arguments` is its own.
    is_function: bool,
}

/// The executor, when it is a plain function or arrow, neither `async` nor a generator.
fn executor<'s, 'a>(expression: &'s Expression<'a>) -> Option<Executor<'s, 'a>> {
    match expression {
        Expression::ArrowFunctionExpression(arrow) if !arrow.r#async => Some(Executor {
            params: &arrow.params,
            body: Body::of_arrow(&arrow.body)?,
            typed: arrow.return_type.is_some()
                || arrow.type_parameters.is_some()
                || arrow
                    .params
                    .items
                    .iter()
                    .any(|each| each.type_annotation.is_some()),
            name: None,
            is_function: false,
        }),
        Expression::FunctionExpression(function)
            if !function.r#async && !function.generator && function.this_param.is_none() =>
        {
            Some(Executor {
                params: &function.params,
                body: Body::Block(function.body.as_ref()?),
                typed: function.return_type.is_some()
                    || function.type_parameters.is_some()
                    || function
                        .params
                        .items
                        .iter()
                        .any(|each| each.type_annotation.is_some()),
                name: function.id.as_ref().map(|id| id.name.as_str()),
                is_function: true,
            })
        }
        _ => None,
    }
}

/// The names of the executor's parameters, one or two plain names, different.
fn parameter_names<'a>(params: &FormalParameters<'a>) -> Option<Vec<&'a str>> {
    if params.rest.is_some() || params.items.is_empty() || params.items.len() > 2 {
        return None;
    }
    let names: Vec<&str> = params
        .items
        .iter()
        .map(|parameter| match &parameter.pattern {
            BindingPattern::BindingIdentifier(name) if parameter.initializer.is_none() => {
                Some(name.name.as_str())
            }
            _ => None,
        })
        .collect::<Option<_>>()?;
    (names.len() == 1 || names[0] != names[1]).then_some(names)
}

/// Whether a value is simple: a name, a literal, or a template without expressions.
fn is_simple(expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(_)
        | Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_) => true,
        Expression::TemplateLiteral(template) => template.expressions.is_empty(),
        _ => false,
    }
}

/// Whether comments sit inside a range outside a part kept.
fn loses_comments(source: &Source, range: Span, kept: Option<Span>) -> bool {
    match kept {
        Some(kept) => {
            has_comment_in(source, Span::new(range.start, kept.start))
                || has_comment_in(source, Span::new(kept.end, range.end))
        }
        None => has_comment_in(source, range),
    }
}

impl Rule for PreferPromiseStaticMethods {
    fn name(&self) -> &'static str {
        "unicorn/prefer-promise-static-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::NewExpression(new) = node.kind() else {
            return;
        };
        let Some(executor) = promise_executor(source, new).and_then(executor) else {
            return;
        };
        let Some(names) = parameter_names(executor.params) else {
            return;
        };
        let Some(Expression::CallExpression(call)) = executor.body.only_expression() else {
            return;
        };
        let Expression::Identifier(callee) = &call.callee else {
            return;
        };
        if call.optional || call.arguments.len() > 1 {
            return;
        }
        let method = if callee.name == names[0] {
            "resolve"
        } else if names.get(1) == Some(&callee.name.as_str()) {
            "reject"
        } else {
            return;
        };
        let value = match call.arguments.first() {
            Some(argument) => match argument.as_expression() {
                Some(value) => Some(value),
                None => return,
            },
            None => None,
        };
        if let Some(value) = value {
            let own_name = matches!(value, Expression::Identifier(name)
                if (name.name == "arguments" && executor.is_function)
                    || executor.name == Some(name.name.as_str())
                    || names.contains(&name.name.as_str()));
            if !is_simple(value) || own_name {
                return;
            }
        }
        let message = format!("Prefer `Promise.{method}()` over a trivial `new Promise()`.");
        let type_arguments = new.type_arguments.as_ref();
        if executor.typed
            || (method == "resolve" && type_arguments.is_some())
            || loses_comments(source, new.span, value.map(GetSpan::span))
        {
            findings.push(source.finding(self.name(), new.span.start, message));
            return;
        }
        let fix = Fix {
            start: new.span.start,
            end: new.span.end,
            text: format!(
                "Promise.{method}{}({})",
                type_arguments.map_or("", |each| text(source, each.span)),
                value.map_or("", |each| text(source, each.span()))
            ),
        };
        findings.push(source.fixed(self.name(), new.span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferPromiseStaticMethods;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferPromiseStaticMethods, "playground/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn writes_the_static_methods() {
        // The documentation's two cases, then the other simple values.
        assert_eq!(
            check("new Promise(resolve => resolve(value));"),
            (vec![(1, 1)], "Promise.resolve(value);".to_owned())
        );
        assert_eq!(
            check("new Promise((resolve, reject) => reject(error));").1,
            "Promise.reject(error);"
        );
        assert_eq!(
            check("const a = new Promise(function (resolve) { resolve(); });").1,
            "const a = Promise.resolve();"
        );
        assert_eq!(
            check("new Promise(r => r(`a`));").1,
            "Promise.resolve(`a`);"
        );
        assert_eq!(
            check("new Promise<never>((_, reject) => reject(1));").1,
            "Promise.reject<never>(1);"
        );
    }

    #[test]
    fn reports_without_a_fix_when_types_or_comments_would_change() {
        for code in [
            "new Promise<number>(resolve => resolve(1));",
            "new Promise((resolve: (a: number) => void) => resolve(1));",
            "new Promise((resolve): void => resolve(1));",
            "new Promise(resolve => /* a */ resolve(1));",
        ] {
            assert_eq!(check(code), (vec![(1, 1)], code.to_owned()), "{code}");
        }
    }

    #[test]
    fn leaves_executors_that_do_more() {
        for code in [
            "new Promise(resolve => resolve(compute()));",
            "new Promise(resolve => resolve(a.b));",
            "new Promise(resolve => { setup(); resolve(1); });",
            "new Promise(async resolve => resolve(1));",
            "new Promise((resolve, reject) => resolve(reject));",
            "new Promise(function f(resolve) { resolve(f); });",
            "new Promise(function (resolve) { resolve(arguments); });",
            "new Promise((resolve, resolve2) => other(1));",
            "new Promise(({ resolve }) => resolve(1));",
            "new Promise((resolve = f) => resolve(1));",
            "new Promise((...all) => all[0](1));",
            "new Promise(resolve => resolve(1, 2));",
            "new Promise(resolve => resolve?.(1));",
            "new Promise(executor);",
            "const Promise = f;\nnew Promise(resolve => resolve(1));",
            "new Other(resolve => resolve(1));",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
