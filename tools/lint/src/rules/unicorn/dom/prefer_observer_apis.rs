//! `unicorn/prefer-observer-apis`: a `resize` or `scroll` listener that reads the layout —
//! `getBoundingClientRect()`, `offsetWidth` and their kin, `innerWidth`, the visual viewport's
//! size — is better a `ResizeObserver` or an `IntersectionObserver` (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/prefer-observer-apis.md`). The listener is written inline or is a
//! function of the file never reassigned.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    AssignmentOperator, BindingPattern, CallExpression, Expression, ObjectAssignmentTarget,
    ObjectPattern,
};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use super::events::enclosing_function;
use super::nodes::{member_object, static_property_name, static_string};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{
    FunctionLike, function_like, function_named, symbol_of, unwrap,
};
use crate::source::Source;

/// The rule.
pub struct PreferObserverApis;

/// The layout properties of an element.
const ELEMENT_LAYOUT: [&str; 11] = [
    "clientHeight",
    "clientLeft",
    "clientTop",
    "clientWidth",
    "offsetHeight",
    "offsetLeft",
    "offsetParent",
    "offsetTop",
    "offsetWidth",
    "scrollHeight",
    "scrollWidth",
];

/// Whether an expression is a global name among some, declared nowhere in the file.
fn is_global(source: &Source, expression: &Expression, names: &[&str]) -> bool {
    matches!(expression, Expression::Identifier(name)
        if names.contains(&name.name.as_str()) && symbol_of(source, name).is_none())
}

/// Whether an expression is `window`, `self` or `globalThis`.
fn is_global_object(source: &Source, expression: &Expression) -> bool {
    is_global(source, expression, &["globalThis", "self", "window"])
}

/// Whether an expression is a global by name, or read from the global object.
fn is_global_member(source: &Source, expression: &Expression, name: &str) -> bool {
    let expression = unwrap(expression);
    is_global(source, expression, &[name])
        || (static_property_name(expression).as_deref() == Some(name)
            && member_object(expression)
                .is_some_and(|object| is_global_object(source, unwrap(object))))
}

/// Whether an object is surely no element: a global, an inline literal, or a `const` bound to an
/// object literal.
fn is_not_element(source: &Source, expression: &Expression) -> bool {
    let expression = unwrap(expression);
    is_global_object(source, expression)
        || is_global_member(source, expression, "document")
        || is_global_member(source, expression, "visualViewport")
        || matches!(
            expression,
            Expression::ObjectExpression(_)
                | Expression::ArrayExpression(_)
                | Expression::StringLiteral(_)
                | Expression::NumericLiteral(_)
                | Expression::BooleanLiteral(_)
                | Expression::NullLiteral(_)
                | Expression::RegExpLiteral(_)
                | Expression::BigIntLiteral(_)
        )
        || crate::rules::unicorn::syntax::const_init(source, expression)
            .is_some_and(|init| matches!(init, Expression::ObjectExpression(_)))
}

/// Whether a node is only written: assigned with `=` or destructured into, not updated.
fn is_write_only(source: &Source, node: &AstNode) -> bool {
    let span = node.kind().span();
    match source.semantic.nodes().parent_kind(node.id()) {
        AstKind::AssignmentExpression(assignment) => {
            assignment.left.span() == span && assignment.operator == AssignmentOperator::Assign
        }
        AstKind::ForOfStatement(statement) => statement.left.span() == span,
        AstKind::ForInStatement(statement) => statement.left.span() == span,
        AstKind::AssignmentTargetWithDefault(target) => target.binding.span() == span,
        AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetPropertyProperty(_)
        | AstKind::AssignmentTargetRest(_) => true,
        AstKind::UnaryExpression(unary) => unary.operator == oxc_ast::ast::UnaryOperator::Delete,
        _ => false,
    }
}

/// Whether a member of an object, by its name, reads the layout.
fn is_layout_member(source: &Source, name: &str, object: &Expression) -> bool {
    let object = unwrap(object);
    (matches!(name, "innerHeight" | "innerWidth") && is_global_object(source, object))
        || (matches!(name, "height" | "width")
            && is_global_member(source, object, "visualViewport"))
        || (ELEMENT_LAYOUT.contains(&name) && !is_not_element(source, object))
}

/// Whether destructuring names some of a set of properties.
fn names_any(names: &[String], wanted: &[&str]) -> bool {
    names.iter().any(|name| wanted.contains(&name.as_str()))
}

/// Whether a destructuring of a value reads the layout.
fn is_layout_destructuring(source: &Source, names: &[String], value: &Expression) -> bool {
    let value = unwrap(value);
    (names_any(names, &["innerHeight", "innerWidth"]) && is_global_object(source, value))
        || (names_any(names, &["height", "width"])
            && is_global_member(source, value, "visualViewport"))
        || (names_any(names, &ELEMENT_LAYOUT) && !is_not_element(source, value))
}

/// The static names of an object pattern's properties.
fn pattern_names(pattern: &ObjectPattern) -> Vec<String> {
    pattern
        .properties
        .iter()
        .filter(|property| !property.computed)
        .filter_map(|property| {
            property
                .key
                .static_name()
                .and_then(|name| name.as_str().map(str::to_owned))
        })
        .collect()
}

/// The static names of an object assignment target's properties.
fn target_names(target: &ObjectAssignmentTarget) -> Vec<String> {
    target
        .properties
        .iter()
        .filter_map(|property| match property {
            oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(name) => {
                Some(name.binding.name.to_string())
            }
            oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyProperty(property)
                if !property.computed =>
            {
                property
                    .name
                    .static_name()
                    .and_then(|name| name.as_str().map(str::to_owned))
            }
            oxc_ast::ast::AssignmentTargetProperty::AssignmentTargetPropertyProperty(_) => None,
        })
        .collect()
}

/// Whether a node reads the layout.
fn reads_layout(source: &Source, node: &AstNode) -> bool {
    match node.kind() {
        AstKind::CallExpression(call) => is_layout_call(source, call),
        AstKind::IdentifierReference(name) => {
            matches!(name.name.as_str(), "innerHeight" | "innerWidth")
                && symbol_of(source, name).is_none()
                && !is_write_only(source, node)
        }
        AstKind::StaticMemberExpression(member) => {
            !is_write_only(source, node)
                && is_layout_member(source, &member.property.name, &member.object)
        }
        AstKind::ComputedMemberExpression(member) => {
            !is_write_only(source, node)
                && static_string(&member.expression)
                    .is_some_and(|name| is_layout_member(source, &name, &member.object))
        }
        _ => false,
    }
}

/// Whether a call reads the layout: `getBoundingClientRect()` or `getClientRects()` of what may
/// be an element.
fn is_layout_call(source: &Source, call: &CallExpression) -> bool {
    matches!(
        &call.callee,
        Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_)
    ) && static_property_name(&call.callee)
        .is_some_and(|name| matches!(name.as_str(), "getBoundingClientRect" | "getClientRects"))
        && member_object(&call.callee).is_some_and(|object| !is_not_element(source, object))
}

/// The function a listener is, with its node and the range of its body.
fn listener(source: &Source, expression: &Expression) -> Option<(NodeId, Span)> {
    let expression = unwrap(expression);
    let function = if let Some(function) = function_like(expression) {
        function
    } else {
        let Expression::Identifier(name) = expression else {
            return None;
        };
        let symbol = symbol_of(source, name)?;
        let scoping = source.semantic.scoping();
        let rewritten = scoping
            .get_resolved_reference_ids(symbol)
            .iter()
            .any(|reference| scoping.get_reference(*reference).is_write());
        if rewritten || scoping.symbol_redeclarations(symbol).len() > 1 {
            return None;
        }
        function_named(source, expression)?
    };
    match function {
        FunctionLike::Function(function) => {
            Some((function.node_id.get(), function.body.as_ref()?.span))
        }
        FunctionLike::Arrow(arrow) => Some((arrow.node_id.get(), arrow.body.span())),
    }
}

/// Whether a listener's body reads the layout, its nested functions left out.
fn listener_reads_layout(source: &Source, function: NodeId, body: Span) -> bool {
    source.semantic.nodes().iter().any(|node| {
        let span = node.kind().span();
        span.start >= body.start
            && span.end <= body.end
            && enclosing_function(source, node.id()) == Some(function)
            && match node.kind() {
                AstKind::VariableDeclarator(declarator) => match (&declarator.id, &declarator.init)
                {
                    (BindingPattern::ObjectPattern(pattern), Some(init)) => {
                        is_layout_destructuring(source, &pattern_names(pattern), init)
                    }
                    _ => false,
                },
                AstKind::AssignmentExpression(assignment) => match &assignment.left {
                    oxc_ast::ast::AssignmentTarget::ObjectAssignmentTarget(target) => {
                        is_layout_destructuring(source, &target_names(target), &assignment.right)
                    }
                    _ => false,
                },
                _ => reads_layout(source, node),
            }
    })
}

impl Rule for PreferObserverApis {
    fn name(&self) -> &'static str {
        "unicorn/prefer-observer-apis"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let count = (2..=3).contains(&call.arguments.len())
            && !call.arguments.iter().any(oxc_ast::ast::Argument::is_spread);
        let is_add = match &call.callee {
            Expression::StaticMemberExpression(member) => {
                member.property.name == "addEventListener" && !member.optional
            }
            callee => is_global(source, callee, &["addEventListener"]),
        };
        if call.optional || !count || !is_add {
            return;
        }
        let Some(event) = call.arguments[0].as_expression() else {
            return;
        };
        let message = match static_string(unwrap(event)).as_deref() {
            Some("resize") => "Prefer ResizeObserver over a resize listener with layout reads.",
            Some("scroll") => {
                "Prefer IntersectionObserver over a scroll listener with layout reads."
            }
            _ => return,
        };
        let Some((function, body)) = call.arguments[1]
            .as_expression()
            .and_then(|handler| listener(source, handler))
        else {
            return;
        };
        if listener_reads_layout(source, function, body) {
            findings.push(source.finding(self.name(), event.span().start, message.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferObserverApis;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferObserverApis, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_listeners_reading_the_layout() {
        // The documentation's two cases.
        assert_eq!(
            messages(&lint(
                &PreferObserverApis,
                "playground/a/b.ts",
                "window.addEventListener('resize', () => {\n\telement.classList.toggle('is-small', element.offsetWidth < 500);\n});\nwindow.addEventListener('scroll', () => {\n\telement.classList.toggle('is-visible', element.getBoundingClientRect().top < window.innerHeight);\n});"
            )),
            [
                "Prefer ResizeObserver over a resize listener with layout reads.",
                "Prefer IntersectionObserver over a scroll listener with layout reads.",
            ]
        );
        // A named function, a `const` arrow, a viewport name, a computed and a visual viewport read.
        assert_eq!(
            check(
                "function onResize() { const { clientWidth } = box; use(clientWidth); }\naddEventListener('resize', onResize);\nconst h = () => { ({ width } = visualViewport); };\nwindow.addEventListener('resize', h);\nwindow.addEventListener('resize', () => { use(innerWidth); });\nwindow.addEventListener('resize', () => { el['scrollHeight']; });"
            ),
            [(2, 18), (4, 25), (5, 25), (6, 25)]
        );
    }

    #[test]
    fn leaves_listeners_without_layout_reads() {
        for code in [
            "window.addEventListener('scroll', () => {\n\tupdateScrollPosition(window.scrollY);\n});",
            "window.addEventListener('resize', () => { innerWidth = 3; });",
            "window.addEventListener('resize', () => { setTimeout(() => el.offsetWidth); });",
            "window.addEventListener('resize', () => { document.clientWidth; ({a: 1}).offsetWidth; });",
            "const o = { offsetWidth: 1 }; window.addEventListener('scroll', () => o.offsetWidth);",
            "let g = () => el.offsetTop; g = other; window.addEventListener('scroll', g);",
            "window.addEventListener('click', () => el.offsetTop);",
            "window.addEventListener(name, () => el.offsetTop);",
            "el?.addEventListener('resize', () => el.offsetTop);",
            "window.addEventListener('resize', () => { el.offsetWidth = 1; });",
            "window.addEventListener('resize', handler);",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
