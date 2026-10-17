//! `unicorn/prefer-dom-node-replace-children`: a node emptied by `node.innerHTML = ''` or by a
//! loop of `removeChild(node.firstChild)` is emptied by `node.replaceChildren()`, and one
//! emptied then filled by `.append(…)` or `.prepend(…)` is filled by one `.replaceChildren(…)`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-dom-node-replace-children.md`). A receiver
//! that may be the document, a template element, no parent node or an optional chain is left.
//! Autofixed when no comment is lost; the merged call only when its arguments are literals and
//! its receiver a plain name that is not the document, as `ESLint` only suggests the others.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, AssignmentOperator, AssignmentTarget, CallExpression, Expression, Statement,
};
use oxc_semantic::{AstNode, ScopeId};
use oxc_span::{GetSpan, Span};

use super::nodes::{is_not_dom_node, plain_method_call, static_property_name, static_string};
use crate::expressions::has_comment_in;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::annotations::{Class, Targets, class_of_expression};
use crate::rules::unicorn::syntax::{
    Value, const_init, has_optional_chain, next_statement, object_text, same_reference,
    single_statement, static_value, symbol_of, text, unwrap,
};
use crate::source::Source;

/// The rule.
pub struct PreferDomNodeReplaceChildren;

/// The types that are no parent node.
const NON_PARENT_NODES: [&str; 10] = [
    "Attr",
    "CDATASection",
    "CharacterData",
    "ChildNode",
    "Comment",
    "DocumentType",
    "Node",
    "Text",
    "Window",
    "WindowProxy",
];

/// Those, and the parent nodes without `innerHTML`.
const NON_INNER_HTML_NODES: [&str; 13] = [
    "Attr",
    "CDATASection",
    "CharacterData",
    "ChildNode",
    "Comment",
    "DocumentType",
    "Node",
    "Text",
    "Window",
    "WindowProxy",
    "Document",
    "DocumentFragment",
    "ParentNode",
];

/// How a receiver of `replaceChildren()` is read.
const RECEIVERS: Targets = Targets {
    names: &[
        "Document",
        "DocumentFragment",
        "Element",
        "HTMLDocument",
        "HTMLElement",
        "ParentNode",
        "SVGElement",
        "ShadowRoot",
    ],
    non_targets: &NON_PARENT_NODES,
    constructors: &[],
    imports: &[],
    allow_nullish_in_mixed_union: true,
    mixed_union: Class::NonTarget,
    prefer_definitions: false,
};

/// How a receiver whose `innerHTML` is emptied is read.
const INNER_HTML_RECEIVERS: Targets = Targets {
    names: &["Element", "HTMLElement", "SVGElement", "ShadowRoot"],
    non_targets: &NON_INNER_HTML_NODES,
    ..RECEIVERS
};

/// How a template element is read.
const TEMPLATES: Targets = Targets {
    names: &["HTMLTemplateElement"],
    non_targets: &[],
    constructors: &[],
    imports: &[],
    allow_nullish_in_mixed_union: true,
    mixed_union: Class::Target,
    prefer_definitions: false,
};

/// How a document is read.
const DOCUMENTS: Targets = Targets {
    names: &["Document", "HTMLDocument", "XMLDocument"],
    non_targets: &[],
    constructors: &[],
    imports: &[],
    allow_nullish_in_mixed_union: false,
    mixed_union: Class::Target,
    prefer_definitions: false,
};

/// The scope a node is read in.
fn scope(source: &Source, span: Span) -> ScopeId {
    source
        .semantic
        .nodes()
        .iter()
        .find(|node| node.kind().span() == span)
        .map_or_else(
            || source.semantic.scoping().root_scope_id(),
            oxc_semantic::AstNode::scope_id,
        )
}

/// Whether a name is a global, declared nowhere in the file.
fn is_global_name(source: &Source, expression: &Expression, names: &[&str]) -> bool {
    matches!(expression, Expression::Identifier(name)
        if names.contains(&name.name.as_str()) && symbol_of(source, name).is_none())
}

/// Whether a receiver is the global document, through `const` bindings (`isGlobalDocument`).
fn is_global_document(source: &Source, expression: &Expression, depth: u8) -> bool {
    let expression = unwrap(expression);
    if depth < 8
        && let Some(init) = const_init(source, expression)
    {
        return is_global_document(source, init, depth + 1);
    }
    match expression {
        Expression::Identifier(_) => is_global_name(source, expression, &["document"]),
        Expression::StaticMemberExpression(member) => {
            !member.optional
                && member.property.name == "document"
                && is_global_name(
                    source,
                    &member.object,
                    &["frames", "globalThis", "parent", "self", "top", "window"],
                )
        }
        _ => false,
    }
}

/// The string an expression always is, through `const` bindings.
fn string_value(source: &Source, expression: &Expression) -> Option<String> {
    let expression = unwrap(expression);
    static_string(expression).or_else(|| match static_value(source, expression) {
        Some(Value::Text(text)) => Some(text),
        _ => None,
    })
}

/// Whether a receiver may be created as a `<template>` (`mayCreateHtmlTemplateElement`).
fn may_create_template(source: &Source, expression: &Expression, depth: u8) -> bool {
    let expression = unwrap(expression);
    if depth < 8
        && let Some(init) = const_init(source, expression)
    {
        return may_create_template(source, init, depth + 1);
    }
    let Expression::CallExpression(call) = expression else {
        return false;
    };
    let static_call = |name: &str, low: usize, high: usize| {
        !call.optional
            && matches!(&call.callee, Expression::StaticMemberExpression(member) if !member.optional)
            && static_property_name(&call.callee).as_deref() == Some(name)
            && (low..=high).contains(&call.arguments.len())
            && !call.arguments.iter().any(Argument::is_spread)
    };
    let argument = |index: usize| call.arguments[index].as_expression();
    if static_call("createElement", 1, 2) {
        let Some(tag) = argument(0) else {
            return false;
        };
        return match string_value(source, tag) {
            Some(name) => name.eq_ignore_ascii_case("template"),
            None => has_member(unwrap(tag)),
        };
    }
    if !static_call("createElementNS", 2, 3)
        || !argument(1)
            .and_then(|tag| string_value(source, tag))
            .is_some_and(|name| name.eq_ignore_ascii_case("template"))
    {
        return false;
    }
    argument(0)
        .and_then(|namespace| string_value(source, namespace))
        .is_none_or(|namespace| namespace == "http://www.w3.org/1999/xhtml")
}

/// Whether an expression reads a member, whose value may change.
fn has_member(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::StaticMemberExpression(_)
            | Expression::ComputedMemberExpression(_)
            | Expression::ChainExpression(_)
    )
}

/// Whether a receiver is left: no DOM node, known to be no parent node, or read through `?.`.
fn is_skipped(source: &Source, receiver: &Expression, targets: &Targets) -> bool {
    let class = class_of_expression(source, scope(source, receiver.span()), receiver, targets, 0);
    is_not_dom_node(receiver)
        || matches!(class, Class::NonTarget | Class::Nullish)
        || is_global_name(source, receiver, &["globalThis", "self", "window"])
        || has_optional_chain(receiver)
}

/// The statement emptying a receiver.
fn replacement(source: &Source, receiver: &Expression) -> String {
    format!("{}.replaceChildren()", object_text(source, receiver))
}

/// The message of an emptied receiver.
fn message(source: &Source, receiver: &Expression) -> String {
    format!(
        "Prefer `{}` over manually emptying DOM children.",
        replacement(source, receiver)
    )
}

/// Whether a range holds a comment outside an inner range.
fn has_comment_around(source: &Source, outer: Span, inner: Span) -> bool {
    has_comment_in(source, Span::new(outer.start, inner.start))
        || has_comment_in(source, Span::new(inner.end, outer.end))
}

impl PreferDomNodeReplaceChildren {
    /// `node.innerHTML = ''`.
    fn inner_html(&self, source: &Source, node: &AstNode, findings: &mut Vec<Finding>) {
        let AstKind::AssignmentExpression(assignment) = node.kind() else {
            return;
        };
        if assignment.operator != AssignmentOperator::Assign
            || static_string(unwrap(&assignment.right)).as_deref() != Some("")
        {
            return;
        }
        let (receiver, property) = match &assignment.left {
            AssignmentTarget::StaticMemberExpression(member)
                if member.property.name == "innerHTML" =>
            {
                (&member.object, member.property.span.start)
            }
            AssignmentTarget::ComputedMemberExpression(member)
                if static_string(&member.expression).as_deref() == Some("innerHTML") =>
            {
                (&member.object, member.expression.span().start)
            }
            _ => return,
        };
        let scope = scope(source, receiver.span());
        if is_skipped(source, receiver, &INNER_HTML_RECEIVERS)
            || is_global_document(source, receiver, 0)
            || may_create_template(source, receiver, 0)
            || class_of_expression(source, scope, receiver, &TEMPLATES, 0) == Class::Target
        {
            return;
        }
        let message = message(source, receiver);
        let nodes = source.semantic.nodes();
        if let AstKind::ExpressionStatement(statement) = nodes.parent_kind(node.id())
            && !has_comment_around(source, statement.span, receiver.span())
        {
            let fix = Fix {
                start: statement.span.start,
                end: statement.span.end,
                text: format!("{};", replacement(source, receiver)),
            };
            findings.push(source.fixed(self.name(), property, message, fix));
            return;
        }
        findings.push(source.finding(self.name(), property, message));
    }

    /// `while (node.firstChild) node.removeChild(node.firstChild);`.
    fn remove_loop(&self, source: &Source, node: &AstNode, findings: &mut Vec<Finding>) {
        let AstKind::WhileStatement(statement) = node.kind() else {
            return;
        };
        let Expression::StaticMemberExpression(child) = &statement.test else {
            return;
        };
        if child.optional || !matches!(child.property.name.as_str(), "firstChild" | "lastChild") {
            return;
        }
        let Some(Statement::ExpressionStatement(only)) = single_statement(&statement.body) else {
            return;
        };
        let Expression::CallExpression(call) = &only.expression else {
            return;
        };
        let Some(method) = plain_method_call(call, &["removeChild"], 1) else {
            return;
        };
        let receiver = &child.object;
        let same_child = call.arguments[0]
            .as_expression()
            .is_some_and(|argument| same_reference(argument, &statement.test));
        if !same_reference(receiver, method.object)
            || !same_child
            || is_skipped(source, receiver, &RECEIVERS)
        {
            return;
        }
        let message = message(source, receiver);
        if has_comment_around(source, statement.span, receiver.span()) {
            findings.push(source.finding(self.name(), statement.span.start, message));
            return;
        }
        let fix = Fix {
            start: statement.span.start,
            end: statement.span.end,
            text: format!("{};", replacement(source, receiver)),
        };
        findings.push(source.fixed(self.name(), statement.span.start, message, fix));
    }

    /// `node.replaceChildren();` then `node.append(…);`.
    fn replace_and_add(
        &self,
        source: &Source,
        node: &AstNode,
        call: &CallExpression,
        findings: &mut Vec<Finding>,
    ) {
        let Some(method) = plain_method_call(call, &["replaceChildren"], 0) else {
            return;
        };
        let nodes = source.semantic.nodes();
        let AstKind::ExpressionStatement(statement) = nodes.parent_kind(node.id()) else {
            return;
        };
        let Some(Statement::ExpressionStatement(next)) =
            next_statement(source, nodes.parent_id(node.id()))
        else {
            return;
        };
        let Expression::CallExpression(add) = &next.expression else {
            return;
        };
        let Some(added) = super::nodes::plain_method_call_at_least(add, &["append", "prepend"], 1)
        else {
            return;
        };
        let receiver = method.object;
        if !same_reference(receiver, added.object) || is_skipped(source, receiver, &RECEIVERS) {
            return;
        }
        let message =
            "Prefer one `.replaceChildren(…)` call over emptying and then adding DOM children."
                .to_owned();
        let scope = scope(source, receiver.span());
        let safe_arguments = add.arguments.iter().all(|argument| {
            argument.as_expression().map(unwrap).is_some_and(|value| {
                matches!(
                    value,
                    Expression::StringLiteral(_)
                        | Expression::NumericLiteral(_)
                        | Expression::BooleanLiteral(_)
                        | Expression::NullLiteral(_)
                        | Expression::BigIntLiteral(_)
                ) || matches!(value, Expression::TemplateLiteral(template) if template.expressions.is_empty())
            })
        });
        let plain_receiver = matches!(
            unwrap(receiver),
            Expression::Identifier(_) | Expression::ThisExpression(_)
        );
        let is_document = is_global_document(source, receiver, 0)
            || class_of_expression(source, scope, receiver, &DOCUMENTS, 0) == Class::Target;
        if has_comment_in(source, statement.span)
            || !safe_arguments
            || !plain_receiver
            || is_document
        {
            findings.push(source.finding(self.name(), call.span.start, message));
            return;
        }
        let between = text(source, Span::new(statement.span.end, next.span.start));
        let removed_end = if between.trim().is_empty() {
            next.span.start
        } else {
            statement.span.end
        };
        let fix = Fix {
            start: statement.span.start,
            end: added.property.end,
            text: format!(
                "{}replaceChildren",
                text(source, Span::new(removed_end, added.property.start))
            ),
        };
        findings.push(source.fixed(self.name(), call.span.start, message, fix));
    }
}

impl Rule for PreferDomNodeReplaceChildren {
    fn name(&self) -> &'static str {
        "unicorn/prefer-dom-node-replace-children"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::AssignmentExpression(_) => self.inner_html(source, node, findings),
            AstKind::WhileStatement(_) => self.remove_loop(source, node, findings),
            AstKind::CallExpression(call) => self.replace_and_add(source, node, call, findings),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferDomNodeReplaceChildren;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferDomNodeReplaceChildren, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn empties_and_fills_with_replace_children() {
        // The documentation's five cases; the fixed text is ESLint's, which only suggests merging
        // a call whose arguments are not literals.
        assert_eq!(
            check("element.innerHTML = '';\nfirst.replaceChildren();\nfirst.append(child);\nsecond.replaceChildren();\nsecond.prepend(firstChild, secondChild);\nwhile (element.firstChild) {\n\telement.removeChild(element.firstChild);\n}\nwhile (element.lastChild) {\n\telement.removeChild(element.lastChild);\n}"),
            (
                vec![(1, 9), (2, 1), (4, 1), (6, 1), (9, 1)],
                "element.replaceChildren();\nfirst.replaceChildren();\nfirst.append(child);\nsecond.replaceChildren();\nsecond.prepend(firstChild, secondChild);\nelement.replaceChildren();\nelement.replaceChildren();"
                    .to_owned()
            )
        );
        // Literal arguments merged; a member receiver, the document or a comment: reported only.
        assert_eq!(
            check("a.replaceChildren();\na.append('x', 1);\nb.c.replaceChildren();\nb.c.append('x');\ndocument.replaceChildren();\ndocument.append('a');\nq.replaceChildren();\n// note\nq.append(z);\n(a || b).innerHTML = '';\nconst v = (e.innerHTML = '');"),
            (
                vec![(1, 1), (3, 1), (5, 1), (7, 1), (10, 10), (11, 14)],
                "a.replaceChildren('x', 1);\nb.c.replaceChildren();\nb.c.append('x');\ndocument.replaceChildren();\ndocument.append('a');\nq.replaceChildren();\n// note\nq.append(z);\n(a || b).replaceChildren();\nconst v = (e.innerHTML = '');"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_templates_documents_and_other_nodes() {
        for code in [
            "element.replaceChildren();",
            "element.replaceChildren(child);",
            "document.innerHTML = '';",
            "const t = document.createElement('template');\nt.innerHTML = '';",
            "const t = document.createElementNS('http://www.w3.org/1999/xhtml', 'template');\nt.innerHTML = '';",
            "const t = document.createElement(names.tag);\nt.innerHTML = '';",
            "function f(n: Text, m: HTMLTemplateElement) { n.innerHTML = ''; m.innerHTML = ''; }",
            "window.innerHTML = '';",
            "e.innerHTML = 'x';",
            "e.innerHTML += '';",
            "while (p.firstChild) p.removeChild(p.lastChild);",
            "while (p?.firstChild) p.removeChild(p.firstChild);",
            "a.replaceChildren();\nb.append(c);",
            "[].innerHTML = '';",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
        assert_eq!(check("e['innerHTML'] = \"\";").1, "e.replaceChildren();");
        assert_eq!(
            check("function f(k: HTMLElement | null) { k.innerHTML = ''; }")
                .0
                .len(),
            1
        );
    }
}
