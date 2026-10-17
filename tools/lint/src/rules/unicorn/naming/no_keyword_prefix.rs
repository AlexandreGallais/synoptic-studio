//! `unicorn/no-keyword-prefix`: no name starting with the keyword `new` or `class` followed by a
//! capital, `newFoo` reading like `new Foo` (`eslint-plugin-unicorn` 78.0.0, with the project's
//! `checkProperties: false`: a property, a member read and a key are left alone, third-party
//! option names such as `newIsCap` not being the project's to rename). A name passed
//! to or called as a function is left alone, as in `ESLint`.

use std::collections::HashSet;

use oxc_ast::AstKind;
use oxc_semantic::NodeId;
use oxc_span::{GetSpan, Span};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoKeywordPrefix;

/// The keywords a name may not start with, the plugin's default.
const PREFIXES: [&str; 2] = ["new", "class"];

/// The keyword a name starts with, followed by a capital (`onlyCamelCase`).
fn keyword_prefix(name: &str) -> Option<&'static str> {
    PREFIXES.iter().copied().find(|keyword| {
        name.strip_prefix(keyword)
            .and_then(|rest| rest.chars().next())
            .is_some_and(|next| next.is_ascii_uppercase())
    })
}

/// Whether a name at a node is reported, by where it stands (`checkProperties: false`).
fn is_reported(source: &Source, id: NodeId, span: Span) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(id);
    match parent.kind() {
        AstKind::StaticMemberExpression(_)
        | AstKind::ComputedMemberExpression(_)
        | AstKind::PrivateFieldExpression(_)
        | AstKind::ObjectProperty(_)
        | AstKind::AssignmentTargetPropertyIdentifier(_)
        | AstKind::CallExpression(_)
        | AstKind::NewExpression(_) => false,
        AstKind::BindingProperty(property) => {
            if property.key.span() == span {
                property.computed
            } else {
                !property.shorthand
            }
        }
        AstKind::AssignmentTargetPropertyProperty(property) => {
            if property.name.span() == span {
                property.computed
            } else {
                true
            }
        }
        AstKind::AssignmentPattern(pattern) => {
            pattern.left.span() == span
                && matches!(nodes.parent_kind(parent.id()), AstKind::BindingProperty(property) if !property.shorthand)
        }
        AstKind::AssignmentTargetWithDefault(_) => matches!(
            nodes.parent_kind(parent.id()),
            AstKind::AssignmentTargetPropertyProperty(_)
        ),
        AstKind::FormalParameter(parameter) => parameter.initializer.is_none(),
        _ => true,
    }
}

impl Rule for NoKeywordPrefix {
    fn name(&self) -> &'static str {
        "unicorn/no-keyword-prefix"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut reported = HashSet::new();
        for node in source.semantic.nodes().iter() {
            let (name, span) = match node.kind() {
                AstKind::IdentifierName(identifier) => (identifier.name.as_str(), identifier.span),
                AstKind::IdentifierReference(identifier) => {
                    (identifier.name.as_str(), identifier.span)
                }
                AstKind::BindingIdentifier(identifier) => {
                    (identifier.name.as_str(), identifier.span)
                }
                AstKind::LabelIdentifier(identifier) => (identifier.name.as_str(), identifier.span),
                _ => continue,
            };
            let Some(keyword) = keyword_prefix(name) else {
                continue;
            };
            let import_name = match source.semantic.nodes().parent_kind(node.id()) {
                AstKind::ImportSpecifier(specifier) => Some(specifier.local.name == name),
                AstKind::ImportDefaultSpecifier(_) | AstKind::ImportNamespaceSpecifier(_) => {
                    Some(true)
                }
                _ => None,
            };
            let report = import_name.unwrap_or_else(|| is_reported(source, node.id(), span));
            if report && reported.insert(span.start) {
                findings.push(source.finding(
                    self.name(),
                    span.start,
                    format!("Do not prefix identifiers with keyword `{keyword}`."),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoKeywordPrefix;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoKeywordPrefix, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_prefixed_names() {
        assert_eq!(
            check("const newFoo = 'foo';"),
            ["Do not prefix identifiers with keyword `new`."]
        );
        assert_eq!(
            check("const classFoo = 'foo';"),
            ["Do not prefix identifiers with keyword `class`."]
        );
        assert_eq!(
            check("function newThing() {}"),
            ["Do not prefix identifiers with keyword `new`."]
        );
        assert_eq!(
            check("const {a: newA} = object;"),
            ["Do not prefix identifiers with keyword `new`."]
        );
        assert_eq!(
            check("import {newFoo} from 'x';"),
            ["Do not prefix identifiers with keyword `new`."]
        );
        assert_eq!(check("type NewFoo = 1; type X = newFoo;").len(), 1);
    }

    #[test]
    fn accepts_the_documented_and_property_names() {
        for code in [
            "const foo = 'foo';",
            "const _newFoo = 'foo';",
            "const new_foo = 'foo';",
            "const fooNew = 'foo';",
            "const news = 1;",
            "const object = {newFoo: 1};",
            "foo.newFoo = 2;",
            "const {newFoo} = object;",
            "f(newFoo);",
            "newFoo();",
            "function f(newFoo = 1) {}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
