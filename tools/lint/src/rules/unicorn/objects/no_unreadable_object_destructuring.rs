//! `unicorn/no-unreadable-object-destructuring`: an object destructuring has no computed key
//! (unless the same pattern collects a rest and the key is dynamic), no array pattern inside
//! it, no third level of object patterns, and assigns to no property (`eslint-plugin-unicorn`
//! 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, PropertyKey};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{is_global, static_value};
use crate::source::Source;

/// The rule.
pub struct NoUnreadableObjectDestructuring;

/// Whether a node is an object pattern, in a declaration or an assignment.
fn is_object_pattern(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::ObjectPattern(_) | AstKind::ObjectAssignmentTarget(_)
    )
}

/// The pattern directly holding a pattern node, through defaults, rests, properties and
/// TypeScript's wrappers (`getParentPattern`).
fn parent_pattern<'s, 'a>(source: &Source<'s, 'a>, id: NodeId) -> Option<&'s AstNode<'a>> {
    let nodes = source.semantic.nodes();
    let mut child = id;
    loop {
        let parent = nodes.parent_node(child);
        match parent.kind() {
            AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_)
            | AstKind::AssignmentPattern(_)
            | AstKind::AssignmentTargetWithDefault(_)
            | AstKind::BindingRestElement(_)
            | AstKind::AssignmentTargetRest(_) => child = parent.id(),
            AstKind::BindingProperty(property) => {
                return (property.value.span() == nodes.kind(child).span())
                    .then(|| nodes.parent_node(parent.id()));
            }
            AstKind::AssignmentTargetPropertyProperty(property) => {
                return (property.binding.span() == nodes.kind(child).span())
                    .then(|| nodes.parent_node(parent.id()));
            }
            AstKind::ObjectPattern(_)
            | AstKind::ArrayPattern(_)
            | AstKind::ObjectAssignmentTarget(_)
            | AstKind::ArrayAssignmentTarget(_) => return Some(parent),
            _ => return None,
        }
    }
}

/// How many object patterns a pattern sits in, itself included.
fn object_depth(source: &Source, node: &AstNode) -> usize {
    let mut depth = usize::from(is_object_pattern(&node.kind()));
    let mut current = node.id();
    while let Some(pattern) = parent_pattern(source, current) {
        depth += usize::from(is_object_pattern(&pattern.kind()));
        current = pattern.id();
    }
    depth
}

/// The static properties of `Math` and `String` a key may read.
fn is_static_global(source: &Source, expression: &Expression) -> bool {
    let tag = match expression {
        Expression::TaggedTemplateExpression(tagged) => &tagged.tag,
        other => other,
    };
    let Expression::StaticMemberExpression(member) = tag else {
        return false;
    };
    let name = member.property.name.as_str();
    (is_global(source, &member.object, "Math")
        && matches!(
            name,
            "E" | "LN2" | "LN10" | "LOG2E" | "LOG10E" | "PI" | "SQRT1_2" | "SQRT2"
        ))
        || (is_global(source, &member.object, "String") && name == "raw")
}

/// Whether a computed key has a value known without running anything.
fn is_static_key(source: &Source, key: &PropertyKey) -> bool {
    key.as_expression().is_some_and(|expression| {
        static_value(source, expression).is_some() || is_static_global(source, expression)
    })
}

impl Rule for NoUnreadableObjectDestructuring {
    fn name(&self) -> &'static str {
        "unicorn/no-unreadable-object-destructuring"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let nodes = source.semantic.nodes();
        let reported = match node.kind() {
            AstKind::BindingProperty(property) if property.computed => {
                let has_rest = matches!(nodes.parent_kind(node.id()),
                    AstKind::ObjectPattern(pattern) if pattern.rest.is_some());
                (!has_rest || is_static_key(source, &property.key)).then_some((
                    property.span.start,
                    "Do not use computed keys in object destructuring.",
                ))
            }
            AstKind::AssignmentTargetPropertyProperty(property) if property.computed => {
                let has_rest = matches!(nodes.parent_kind(node.id()),
                    AstKind::ObjectAssignmentTarget(pattern) if pattern.rest.is_some());
                (!has_rest || is_static_key(source, &property.name)).then_some((
                    property.span.start,
                    "Do not use computed keys in object destructuring.",
                ))
            }
            AstKind::ArrayPattern(_) | AstKind::ArrayAssignmentTarget(_) => {
                (object_depth(source, node) > 0).then_some((
                    node.kind().span().start,
                    "Do not use array destructuring inside object destructuring.",
                ))
            }
            AstKind::ObjectPattern(_) | AstKind::ObjectAssignmentTarget(_) => {
                (object_depth(source, node) == 3).then_some((
                    node.kind().span().start,
                    "Do not use object destructuring deeper than two levels.",
                ))
            }
            AstKind::StaticMemberExpression(_)
            | AstKind::ComputedMemberExpression(_)
            | AstKind::PrivateFieldExpression(_) => parent_pattern(source, node.id())
                .filter(|pattern| is_object_pattern(&pattern.kind()))
                .map(|_| {
                    (
                        node.kind().span().start,
                        "Do not assign destructured values to object properties.",
                    )
                }),
            _ => None,
        };
        if let Some((offset, message)) = reported {
            findings.push(source.finding(self.name(), offset, message.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnreadableObjectDestructuring;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &NoUnreadableObjectDestructuring,
            "playground/a.ts",
            code,
        ))
    }

    #[test]
    fn reports_the_documented_unreadable_patterns() {
        assert_eq!(
            check("const {[key]: value} = object;"),
            ["Do not use computed keys in object destructuring."]
        );
        assert_eq!(
            check("const {foo: [bar]} = object;"),
            ["Do not use array destructuring inside object destructuring."]
        );
        assert_eq!(
            check("const {foo: {bar: {baz}}} = object;"),
            ["Do not use object destructuring deeper than two levels."]
        );
        assert_eq!(
            check("({foo: object.property} = object);"),
            ["Do not assign destructured values to object properties."]
        );
        assert_eq!(
            check("const {['a']: value, ...rest} = object;"),
            ["Do not use computed keys in object destructuring."]
        );
        assert_eq!(
            check("function f({a: [b] = []}) {}"),
            ["Do not use array destructuring inside object destructuring."]
        );
        assert_eq!(
            check("({a: {b: {c}}} = object);"),
            ["Do not use object destructuring deeper than two levels."]
        );
    }

    #[test]
    fn accepts_the_documented_readable_patterns() {
        for code in [
            "const {foo} = object;",
            "const {foo: bar = defaultValue} = object;",
            "const {foo: {bar}} = object;",
            "const {[key]: value, ...rest} = object;",
            "const [bar] = object.foo;",
            "const {baz} = object.foo.bar;",
            "({foo} = object);\nobject.property = foo;",
            "const [{a: {b}}] = list;",
            "[object.a] = list;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
