//! `unicorn/no-incorrect-query-selector`: `.querySelectorAll(…)[0]` and its `.at(0)` or
//! `.item(0)` are `.querySelector(…)`; a simple ID selector asks for `.querySelector(…)`; a
//! `NodeList` tested for its truth or compared with `null` or `undefined` is always there, its
//! `.length` is what to test; `.querySelector(…)` gives `null`, never `undefined`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-incorrect-query-selector.md`). The first
//! match and the length check are autofixed unless a comment would be lost.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, CallExpression, Expression, UnaryOperator};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use super::nodes::{is_not_dom_node, plain_method_call, static_string};
use crate::expressions::{const_value, has_comment_in, is_control_flow_test, is_written, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{const_init, symbol_of};
use crate::source::Source;

/// The rule.
pub struct NoIncorrectQuerySelector;

/// The message of a first match read from a `NodeList`.
const FIRST_MATCH: &str = "Prefer `.querySelector()` when only the first match is used.";

/// Whether an expression is a plain call of a query method with one argument, on what may be a
/// DOM node.
fn is_query<'b, 'a>(expression: &'b Expression<'a>, name: &str) -> Option<&'b CallExpression<'a>> {
    let Expression::CallExpression(call) = expression else {
        return None;
    };
    let method = plain_method_call(call, &[name], 1)?;
    (!is_not_dom_node(method.object)).then_some(call)
}

/// Whether an expression is the number `0`.
fn is_zero(expression: &Expression) -> bool {
    matches!(expression, Expression::NumericLiteral(zero) if zero.value == 0.0)
}

/// Whether an expression is a call of a query method, or a `const` bound to one.
fn is_query_or_const(source: &Source, expression: &Expression, name: &str) -> bool {
    match expression {
        Expression::Identifier(_) => {
            const_init(source, expression).is_some_and(|init| is_query(init, name).is_some())
        }
        _ => is_query(expression, name).is_some(),
    }
}

/// The finding of a first match: `querySelectorAll` renamed and the access removed.
fn first_match(
    source: &Source,
    rule: &'static str,
    access: Span,
    call: &CallExpression,
) -> Finding {
    let Expression::StaticMemberExpression(callee) = &call.callee else {
        return source.finding(rule, access.start, FIRST_MATCH.to_owned());
    };
    let removed = Span::new(call.span.end, access.end);
    if has_comment_in(source, removed) {
        return source.finding(rule, access.start, FIRST_MATCH.to_owned());
    }
    let fix = Fix {
        start: callee.property.span.start,
        end: access.end,
        text: format!(
            "querySelector{}",
            text(source, Span::new(callee.property.span.end, call.span.end))
        ),
    };
    source.fixed(rule, access.start, FIRST_MATCH.to_owned(), fix)
}

/// The node a value's truth is read through, `!` and `Boolean(…)` climbed, and whether it is
/// negated (`getBooleanAncestor`).
fn boolean_ancestor(source: &Source, id: NodeId) -> (NodeId, bool) {
    let nodes = source.semantic.nodes();
    let mut current = id;
    let mut negative = false;
    loop {
        let span = nodes.kind(current).span();
        let parent = nodes.parent_id(current);
        match nodes.kind(parent) {
            AstKind::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
                negative = !negative;
            }
            AstKind::CallExpression(call)
                if !call.optional
                    && call.arguments.len() == 1
                    && call.arguments[0].span() == span
                    && matches!(&call.callee, Expression::Identifier(callee)
                        if callee.name == "Boolean" && symbol_of(source, callee).is_none()) => {}
            _ => return (current, negative),
        }
        current = parent;
    }
}

/// The finding of a `NodeList` tested for its truth.
fn length_check(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    is_list: bool,
    span: Span,
) -> Option<Finding> {
    let nodes = source.semantic.nodes();
    if !matches!(
        nodes.parent_kind(node.id()),
        AstKind::IfStatement(_)
            | AstKind::ConditionalExpression(_)
            | AstKind::WhileStatement(_)
            | AstKind::DoWhileStatement(_)
            | AstKind::ForStatement(_)
            | AstKind::LogicalExpression(_)
            | AstKind::UnaryExpression(_)
            | AstKind::CallExpression(_)
    ) {
        return None;
    }
    let (ancestor, negative) = boolean_ancestor(source, node.id());
    if !is_control_flow_test(source, ancestor) {
        return None;
    }
    if !is_list {
        return None;
    }
    let message = "Check `.length` instead of the `NodeList` object itself.".to_owned();
    let whole = nodes.kind(ancestor).span();
    let after = source.text[whole.end as usize..].trim_start();
    if has_comment_in(source, Span::new(whole.start, span.start))
        || has_comment_in(source, Span::new(span.end, whole.end))
        || after.starts_with("//")
        || after.starts_with("/*")
    {
        return Some(source.finding(rule, span.start, message));
    }
    let fix = Fix {
        start: whole.start,
        end: whole.end,
        text: format!(
            "{}.length {}",
            text(source, span),
            if negative { "=== 0" } else { "> 0" }
        ),
    };
    Some(source.fixed(rule, span.start, message, fix))
}

/// Whether an expression is `null`, or `undefined` declared nowhere.
fn is_nullish(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::NullLiteral(_) => true,
        Expression::Identifier(name) => {
            name.name == "undefined" && symbol_of(source, name).is_none()
        }
        _ => false,
    }
}

/// The message of a query compared with `null` or `undefined`.
fn comparison(
    source: &Source,
    left: &Expression,
    right: &Expression,
    strict: bool,
) -> Option<&'static str> {
    let ordered = [(left, right), (right, left)];
    for name in ["querySelectorAll", "querySelector"] {
        let compared = ordered.iter().find(|(query, value)| {
            is_query_or_const(source, query, name) && is_nullish(source, value)
        });
        match (name, compared) {
            ("querySelectorAll", Some(_)) => {
                return Some(
                    "Check `.length` instead of comparing the `NodeList` object with `null` or `undefined`.",
                );
            }
            ("querySelector", Some((_, value)))
                if strict && matches!(value, Expression::Identifier(_)) =>
            {
                return Some(
                    "Compare the result of `.querySelector()` with `null` instead of `undefined`.",
                );
            }
            _ => {}
        }
    }
    None
}

impl Rule for NoIncorrectQuerySelector {
    fn name(&self) -> &'static str {
        "unicorn/no-incorrect-query-selector"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::ComputedMemberExpression(member)
                if !member.optional && is_zero(&member.expression) && !is_written(source, node) =>
            {
                if let Some(call) = is_query(&member.object, "querySelectorAll") {
                    findings.push(first_match(source, self.name(), member.span, call));
                }
            }
            AstKind::CallExpression(call) => {
                if let Some(method) = plain_method_call(call, &["at", "item"], 1)
                    && let Some(list) = is_query(method.object, "querySelectorAll")
                    && call.arguments[0].as_expression().is_some_and(is_zero)
                {
                    findings.push(first_match(source, self.name(), call.span, list));
                    return;
                }
                let is_list = plain_method_call(call, &["querySelectorAll"], 1)
                    .is_some_and(|method| !is_not_dom_node(method.object));
                if let Some(finding) = length_check(source, self.name(), node, is_list, call.span) {
                    findings.push(finding);
                    return;
                }
                let Some(method) = plain_method_call(call, &["querySelectorAll"], 1) else {
                    return;
                };
                if is_not_dom_node(method.object) || is_first_access(source, node) {
                    return;
                }
                let simple_id = call.arguments[0]
                    .as_expression()
                    .and_then(static_string)
                    .is_some_and(|selector| is_simple_id(&selector));
                if simple_id {
                    findings.push(source.finding(
                        self.name(),
                        method.property.start,
                        "Prefer `.querySelector()` for a simple ID selector.".to_owned(),
                    ));
                }
            }
            AstKind::BinaryExpression(binary) => {
                let strict = matches!(
                    binary.operator,
                    BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
                );
                let equality = strict
                    || matches!(
                        binary.operator,
                        BinaryOperator::Equality | BinaryOperator::Inequality
                    );
                if equality
                    && let Some(message) = comparison(source, &binary.left, &binary.right, strict)
                {
                    findings.push(source.finding(
                        self.name(),
                        binary.span.start,
                        message.to_owned(),
                    ));
                }
            }
            AstKind::IdentifierReference(name) => {
                let is_list = symbol_of(source, name)
                    .and_then(|symbol| const_value(source, symbol))
                    .is_some_and(|init| is_query(init, "querySelectorAll").is_some());
                if let Some(finding) = length_check(source, self.name(), node, is_list, name.span) {
                    findings.push(finding);
                }
            }
            _ => {}
        }
    }
}

/// Whether a selector is a simple ID selector, `#name`.
fn is_simple_id(selector: &str) -> bool {
    let Some(name) = selector.strip_prefix('#') else {
        return false;
    };
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first == '-' || first == '_' || first.is_ascii_alphabetic())
        && characters.all(|each| each == '-' || each == '_' || each.is_ascii_alphanumeric())
}

/// Whether a `.querySelectorAll(…)` call is read only for its first element.
fn is_first_access(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    match parent.kind() {
        AstKind::ComputedMemberExpression(member) => {
            !member.optional && is_zero(&member.expression)
        }
        AstKind::StaticMemberExpression(member)
            if !member.optional && matches!(member.property.name.as_str(), "at" | "item") =>
        {
            matches!(nodes.parent_kind(parent.id()), AstKind::CallExpression(call)
                if plain_method_call(call, &["at", "item"], 1).is_some()
                    && call.callee.span() == member.span
                    && call.arguments[0].as_expression().is_some_and(is_zero))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::NoIncorrectQuerySelector;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoIncorrectQuerySelector, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn reports_the_documented_cases_and_fixes_them_as_eslint() {
        // The documentation's seven cases, then planted ones; the fixed text is ESLint's.
        let code = "document.querySelectorAll('form')[0];\ndocument.querySelectorAll('form').at(0);\ndocument.querySelectorAll('#foo');\nif (document.querySelectorAll('.item')) {}\nconst elements = document.querySelectorAll('.item');\nif (elements) {}\ndocument.querySelectorAll('.item') === null;\ndocument.querySelector('.item') === undefined;\nif (!document.querySelectorAll('.item')) {}\nwhile (Boolean(elements) && x) {}\nconst y = elements ? 1 : 2;\nelements == undefined;\nundefined !== document.querySelector('a');\ndocument.querySelectorAll('#a-b_c').item(0);\ndocument.querySelectorAll(`#ok`);\ndocument.querySelectorAll('form')[/* c */ 0];";
        assert_eq!(
            check(code),
            (
                vec![
                    (1, 1),
                    (2, 1),
                    (3, 10),
                    (4, 5),
                    (6, 5),
                    (7, 1),
                    (8, 1),
                    (9, 6),
                    (10, 16),
                    (11, 11),
                    (12, 1),
                    (13, 1),
                    (14, 1),
                    (15, 10),
                    (16, 1),
                ],
                "document.querySelector('form');\ndocument.querySelector('form');\ndocument.querySelectorAll('#foo');\nif (document.querySelectorAll('.item').length > 0) {}\nconst elements = document.querySelectorAll('.item');\nif (elements.length > 0) {}\ndocument.querySelectorAll('.item') === null;\ndocument.querySelector('.item') === undefined;\nif (document.querySelectorAll('.item').length === 0) {}\nwhile (elements.length > 0 && x) {}\nconst y = elements.length > 0 ? 1 : 2;\nelements == undefined;\nundefined !== document.querySelector('a');\ndocument.querySelector('#a-b_c');\ndocument.querySelectorAll(`#ok`);\ndocument.querySelectorAll('form')[/* c */ 0];"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_the_right_uses() {
        for code in [
            "document.querySelector('form');",
            "document.querySelector('#foo');",
            "if (document.querySelectorAll('.item').length > 0) {}",
            "document.querySelectorAll('.item').length === 0;",
            "document.querySelector('.item') === null;",
            "document.querySelector('a') == undefined;",
            "document.querySelectorAll('#1a');",
            "document.querySelectorAll('form')[0] = 1;",
            "document.querySelectorAll('form')?.[0];",
            "document.querySelectorAll('form')[1];",
            "[].querySelectorAll('#foo');",
            "const e = document.querySelectorAll('a');\nfoo(e);\nif (e.length) {}",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
