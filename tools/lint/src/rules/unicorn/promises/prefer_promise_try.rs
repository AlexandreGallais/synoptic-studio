//! `unicorn/prefer-promise-try`: the boilerplate that wraps a call in a promise —
//! `new Promise(resolve => resolve(fn()))` and `Promise.resolve().then(fn)` — where
//! `Promise.try()` would do (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-promise-try.md`).
//! The first is autofixed, the call kept in an arrow, unless a type is written on the executor,
//! the expression is the callee of a `new`, or a comment would go; the second is only reported,
//! its timing changing.

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::calls::{Body, parenthesized_span, promise_executor};
use crate::expressions::{has_comment_in, is_global, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{call_of, concise_body_text, unwrap};
use crate::source::Source;

/// The rule.
pub struct PreferPromiseTry;

/// The rule's message.
const MESSAGE: &str = "Prefer `Promise.try()` over promise-wrapping boilerplate.";

/// Whether a range of the file reads a name.
fn reads(source: &Source, symbol: oxc_semantic::SymbolId, span: oxc_span::Span) -> bool {
    source
        .semantic
        .scoping()
        .get_resolved_references(symbol)
        .any(|reference| {
            let at = source.semantic.nodes().kind(reference.node_id()).span();
            at.start >= span.start && at.end <= span.end
        })
}

/// Whether a node is the callee of a `new`, through the members read on it.
fn is_new_callee(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = node.id();
    loop {
        let parent = nodes.parent_node(child);
        let child_span = nodes.kind(child).span();
        match parent.kind() {
            AstKind::StaticMemberExpression(member) if member.object.span() == child_span => {
                child = parent.id();
            }
            AstKind::ComputedMemberExpression(member) if member.object.span() == child_span => {
                child = parent.id();
            }
            AstKind::NewExpression(new) => return new.callee.span() == child_span,
            _ => return false,
        }
    }
}

/// Whether a `.then()` callback may be one: a function, a member or a name other than
/// `undefined`.
fn is_then_callback(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => true,
        Expression::Identifier(name) => name.name != "undefined",
        _ => false,
    }
}

impl PreferPromiseTry {
    /// The finding on `new Promise(resolve => resolve(fn()))`.
    fn wrapped_call(&self, node: &AstNode, source: &Source) -> Option<Finding> {
        let AstKind::NewExpression(new) = node.kind() else {
            return None;
        };
        let Expression::ArrowFunctionExpression(arrow) = promise_executor(source, new)? else {
            return None;
        };
        let [parameter] = arrow.params.items.as_slice() else {
            return None;
        };
        let BindingPattern::BindingIdentifier(name) = &parameter.pattern else {
            return None;
        };
        if arrow.r#async || arrow.params.rest.is_some() || parameter.initializer.is_some() {
            return None;
        }
        let Expression::CallExpression(resolve) = Body::of_arrow(&arrow.body)?.only_expression()?
        else {
            return None;
        };
        let [value] = resolve.arguments.as_slice() else {
            return None;
        };
        let value = value.as_expression()?;
        let calls_resolve =
            matches!(&resolve.callee, Expression::Identifier(callee) if callee.name == name.name);
        if resolve.optional
            || !calls_resolve
            || call_of(value).is_none()
            || reads(source, name.symbol_id.get()?, value.span())
        {
            return None;
        }
        let typed = parameter.type_annotation.is_some()
            || arrow.return_type.is_some()
            || arrow.type_parameters.is_some();
        let range = parenthesized_span(source, new.span);
        if typed || is_new_callee(source, node) || has_comment_in(source, range) {
            return Some(source.finding(self.name(), new.span.start, MESSAGE.to_owned()));
        }
        let fix = Fix {
            start: range.start,
            end: range.end,
            text: format!(
                "Promise.try{}(() => {})",
                new.type_arguments
                    .as_ref()
                    .map_or("", |each| text(source, each.span)),
                concise_body_text(source, value)
            ),
        };
        Some(source.fixed(self.name(), new.span.start, MESSAGE.to_owned(), fix))
    }

    /// The finding on `Promise.resolve().then(fn)`.
    fn resolved_then(&self, node: &AstNode, source: &Source) -> Option<Finding> {
        let AstKind::CallExpression(call) = node.kind() else {
            return None;
        };
        let Expression::StaticMemberExpression(then) = &call.callee else {
            return None;
        };
        let [callback] = call.arguments.as_slice() else {
            return None;
        };
        let Expression::CallExpression(resolved) = &then.object else {
            return None;
        };
        let Expression::StaticMemberExpression(resolve) = &resolved.callee else {
            return None;
        };
        let from_promise = matches!(&resolve.object, Expression::Identifier(promise)
            if promise.name == "Promise" && is_global(source, promise));
        let plain = !call.optional && !then.optional && !resolved.optional && !resolve.optional;
        (plain
            && then.property.name == "then"
            && resolve.property.name == "resolve"
            && from_promise
            && resolved.arguments.is_empty()
            && callback.as_expression().is_some_and(is_then_callback))
        .then(|| source.finding(self.name(), call.span.start, MESSAGE.to_owned()))
    }
}

impl Rule for PreferPromiseTry {
    fn name(&self) -> &'static str {
        "unicorn/prefer-promise-try"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        findings.extend(
            self.wrapped_call(node, source)
                .or_else(|| self.resolved_then(node, source)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::PreferPromiseTry;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferPromiseTry, "playground/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn writes_promise_try_for_a_wrapped_call() {
        // The documentation's three wrong cases.
        assert_eq!(
            check("new Promise(resolve => resolve(fn()));"),
            (vec![(1, 1)], "Promise.try(() => fn());".to_owned())
        );
        assert_eq!(
            check("new Promise(resolve => resolve(fn(argument)));").1,
            "Promise.try(() => fn(argument));"
        );
        let then = "Promise.resolve().then(fn);";
        assert_eq!(check(then), (vec![(1, 1)], then.to_owned()));
        assert_eq!(
            check("const a = (new Promise<number>(resolve => resolve(a?.b())));").1,
            "const a = Promise.try<number>(() => a?.b());"
        );
        assert_eq!(
            check("const a = new Promise(resolve => { resolve(f()); });").1,
            "const a = Promise.try(() => f());"
        );
    }

    #[test]
    fn reports_without_a_fix_when_types_comments_or_new_would_change() {
        for code in [
            "new Promise((resolve: (a: number) => void) => resolve(fn()));",
            "new Promise(resolve => resolve(/* a */ fn()));",
            "new (new Promise(resolve => resolve(fn())).constructor)();",
        ] {
            let found = check(code);
            assert_eq!((found.0.len(), found.1), (1, code.to_owned()), "{code}");
        }
    }

    #[test]
    fn leaves_executors_that_do_other_work() {
        for code in [
            "new Promise(resolve => {\n\tsetup();\n\tresolve(fn());\n});",
            "new Promise(resolve => resolve(value));",
            "new Promise(resolve => resolve(fn(resolve)));",
            "new Promise((resolve, reject) => resolve(fn()));",
            "new Promise(function (resolve) { resolve(fn()); });",
            "new Promise(async resolve => resolve(fn()));",
            "Promise.resolve(1).then(fn);",
            "Promise.resolve().then(undefined);",
            "Promise.resolve().then(fn, other);",
            "Promise.resolve().then(1);",
            "Promise.resolve()?.then(fn);",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
