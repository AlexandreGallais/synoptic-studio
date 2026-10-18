//! `unicorn/better-dom-traversing`: `.childNodes[0]` is `.firstChild`, `.children[0]` is
//! `.firstElementChild`, a later child is better found by `.querySelector()`, a chain of
//! `.parentElement` by `.closest()`, and chained `.querySelector()` calls of static selectors by
//! one (`eslint-plugin-unicorn` 78.0.0, `docs/rules/better-dom-traversing.md`). Reported only:
//! `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{CallExpression, ComputedMemberExpression, Expression, StaticMemberExpression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::nodes::{is_not_dom_node, plain_method_call, static_string};
use crate::expressions::is_written;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct BetterDomTraversing;

/// The collection a member is read from, `children` or `childNodes`, read without `?.`.
fn collection<'a>(expression: &Expression<'a>) -> Option<&'a str> {
    match expression {
        Expression::StaticMemberExpression(member)
            if !member.optional
                && matches!(member.property.name.as_str(), "children" | "childNodes") =>
        {
            Some(member.property.name.as_str())
        }
        _ => None,
    }
}

/// The index of a member read by a non-negative safe integer, without `?.`.
fn index(member: &ComputedMemberExpression) -> Option<f64> {
    match &member.expression {
        Expression::NumericLiteral(literal)
            if !member.optional
                && literal.value >= 0.0
                && literal.value.fract() == 0.0
                && literal.value <= 9_007_199_254_740_991.0 =>
        {
            Some(literal.value)
        }
        _ => None,
    }
}

/// Whether `children` is a component's, `props.children` or `….props.children`.
fn is_props(member: &Expression) -> bool {
    let Expression::StaticMemberExpression(children) = member else {
        return false;
    };
    match &children.object {
        Expression::Identifier(name) => name.name == "props",
        Expression::StaticMemberExpression(props) => {
            !props.optional && props.property.name == "props"
        }
        _ => false,
    }
}

/// Whether an indexed collection is itself the object of `.children[n]`, which is reported
/// instead.
fn is_nested(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    let AstKind::StaticMemberExpression(children) = parent.kind() else {
        return false;
    };
    children.property.name == "children"
        && !children.optional
        && matches!(nodes.parent_kind(parent.id()),
            AstKind::ComputedMemberExpression(outer) if index(outer).is_some())
}

/// The message of an indexed child read.
fn indexed_message(
    source: &Source,
    node: &AstNode,
    member: &ComputedMemberExpression,
) -> Option<&'static str> {
    let at = index(member)?;
    let name = collection(&member.object)?;
    let Expression::StaticMemberExpression(owner) = &member.object else {
        return None;
    };
    if is_written(source, node)
        || is_not_dom_node(&owner.object)
        || (name == "children" && is_props(&member.object))
        || is_nested(source, node)
    {
        return None;
    }
    match (name, at == 0.0) {
        ("childNodes", true) => Some("Prefer `.firstChild` over `.childNodes[0]`."),
        ("children", true) => Some("Prefer `.firstElementChild` over `.children[0]`."),
        ("children", false) => Some("Consider `.querySelector()` over positional child traversal."),
        _ => None,
    }
}

/// Whether an expression is `.parentElement`, read without `?.`.
fn is_parent_element(expression: &Expression) -> bool {
    matches!(expression, Expression::StaticMemberExpression(member)
        if !member.optional && member.property.name == "parentElement")
}

/// Whether a member is the outermost of a chain of at least two `.parentElement`.
fn is_parent_chain(source: &Source, node: &AstNode, member: &StaticMemberExpression) -> bool {
    if member.optional
        || member.property.name != "parentElement"
        || !is_parent_element(&member.object)
    {
        return false;
    }
    let outer = matches!(source.semantic.nodes().parent_kind(node.id()),
        AstKind::StaticMemberExpression(parent)
            if parent.property.name == "parentElement" && !parent.optional);
    let mut root = &member.object;
    while let Expression::StaticMemberExpression(inner) = root {
        if !is_parent_element(root) {
            break;
        }
        root = &inner.object;
    }
    !outer && !is_not_dom_node(root)
}

/// Whether a call is the inner part of a chain: the object of a static `.querySelector(…)`.
fn is_followed_by_query(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    if !matches!(parent.kind(), AstKind::StaticMemberExpression(_)) {
        return false;
    }
    let AstKind::CallExpression(outer) = nodes.parent_kind(parent.id()) else {
        return false;
    };
    outer.callee.span() == parent.kind().span()
        && plain_method_call(outer, &["querySelector"], 1).is_some()
        && outer.arguments[0]
            .as_expression()
            .and_then(static_string)
            .is_some()
}

/// Whether a call sits in an optional chain, through the members and calls it is the object or
/// callee of.
fn is_in_chain(source: &Source, node: &AstNode) -> bool {
    source
        .semantic
        .nodes()
        .ancestors(node.id())
        .find(|ancestor| {
            !matches!(
                ancestor.kind(),
                AstKind::StaticMemberExpression(_)
                    | AstKind::ComputedMemberExpression(_)
                    | AstKind::CallExpression(_)
            )
        })
        .is_some_and(|ancestor| matches!(ancestor.kind(), AstKind::ChainExpression(_)))
}

/// Whether a call ends a chain of at least two static `.querySelector(…)` calls on what may be a
/// DOM node, and is neither inside a longer one nor an optional chain.
fn is_query_chain(source: &Source, node: &AstNode, call: &CallExpression) -> bool {
    if is_followed_by_query(source, node) || is_in_chain(source, node) {
        return false;
    }
    let mut count = 0;
    let mut current = call;
    loop {
        let Some(method) = plain_method_call(current, &["querySelector"], 1) else {
            return false;
        };
        if current.arguments[0]
            .as_expression()
            .and_then(static_string)
            .is_none()
        {
            return count >= 2;
        }
        count += 1;
        match method.object {
            Expression::CallExpression(inner)
                if plain_method_call(inner, &["querySelector"], 1).is_some() =>
            {
                current = inner;
            }
            root => {
                return count >= 2
                    && !matches!(root, Expression::ChainExpression(_))
                    && !is_not_dom_node(root);
            }
        }
    }
}

impl Rule for BetterDomTraversing {
    fn name(&self) -> &'static str {
        "unicorn/better-dom-traversing"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let found = match node.kind() {
            AstKind::ComputedMemberExpression(member) => {
                indexed_message(source, node, member).map(|message| (member.span.start, message))
            }
            AstKind::StaticMemberExpression(member) if is_parent_chain(source, node, member) => {
                Some((
                    member.span.start,
                    "Consider `.closest()` over chaining `.parentElement`.",
                ))
            }
            AstKind::CallExpression(call) if is_query_chain(source, node, call) => Some((
                call.span.start,
                "Consider merging chained `.querySelector()` calls.",
            )),
            _ => None,
        };
        if let Some((offset, message)) = found {
            findings.push(source.finding(self.name(), offset, message.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BetterDomTraversing;
    use crate::testing::{lint, messages};

    /// The places and messages of the findings.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&BetterDomTraversing, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn reports_positional_and_chained_traversal() {
        // The documentation's five cases.
        assert_eq!(
            messages(&lint(
                &BetterDomTraversing,
                "playground/a/b.ts",
                "element.childNodes[0];\nelement.children[0];\nelement.children[2];\nelement.parentElement.parentElement;\nelement.querySelector('a').querySelector('b');"
            )),
            [
                "Prefer `.firstChild` over `.childNodes[0]`.",
                "Prefer `.firstElementChild` over `.children[0]`.",
                "Consider `.querySelector()` over positional child traversal.",
                "Consider `.closest()` over chaining `.parentElement`.",
                "Consider merging chained `.querySelector()` calls.",
            ]
        );
        // Once per chain, on the outermost part, at its start.
        assert_eq!(
            check(
                "a.parentElement.parentElement.parentElement;\nx = a.children[0].children[1];\na.querySelector('a').querySelector(`b`).querySelector('c');"
            ),
            [
                (
                    1,
                    1,
                    "Consider `.closest()` over chaining `.parentElement`.".to_owned()
                ),
                (
                    2,
                    5,
                    "Consider `.querySelector()` over positional child traversal.".to_owned()
                ),
                (
                    3,
                    1,
                    "Consider merging chained `.querySelector()` calls.".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn leaves_the_better_forms_and_what_is_not_a_traversal() {
        for code in [
            "element.firstChild;",
            "element.firstElementChild;",
            "element.querySelector(':scope selector');",
            "element.closest('selector');",
            "element.querySelector(':scope a b');",
            "element.childNodes[1];",
            "element.childNodes?.[0];",
            "element.children[-1];",
            "element.children[i];",
            "element.children[0] = node;",
            "props.children[0];",
            "this.props.children[1];",
            "[].children[0];",
            "element.parentElement;",
            "element.parentElement?.parentElement;",
            "element.querySelector(a).querySelector('b');",
            "element.querySelector('a')?.querySelector('b');",
            "element?.querySelector('a').querySelector('b');",
            "'x'.querySelector('a').querySelector('b');",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
        assert_eq!(
            check("a.querySelector(x).querySelector('a').querySelector('b');").len(),
            1
        );
    }
}
