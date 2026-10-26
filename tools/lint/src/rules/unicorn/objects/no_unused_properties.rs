//! `unicorn/no-unused-properties`: every property of a local object literal, or of an inline
//! object type of a variable or a parameter, is read somewhere (`eslint-plugin-unicorn`
//! 78.0.0). An object passed whole, exported, called on, assigned to, read by a dynamic key, or
//! holding a method counts as used whole; nested objects are followed through their reads.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    AssignmentTargetProperty, BindingPattern, Expression, ObjectExpression, ObjectPropertyKind,
    PropertyKey, PropertyKind, TSSignature, TSType, TSTypeLiteral,
};
use oxc_semantic::NodeId;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::unwrap;
use crate::source::Source;

/// The rule.
pub struct NoUnusedProperties;

/// What holds properties: an object literal or an inline object type.
#[derive(Clone, Copy)]
enum Container<'a> {
    /// An object literal.
    Object(&'a ObjectExpression<'a>),
    /// An object type literal.
    Type(&'a TSTypeLiteral<'a>),
}

/// A property's static name: a string, or a number written as one.
#[derive(Clone, Debug, PartialEq)]
enum Name {
    /// A name or a string.
    Text(String),
    /// A number.
    Number(f64),
}

/// The name a key gives, when it is a name or a literal (`getPropertyKeyName`).
fn key_name(key: &PropertyKey) -> Option<Name> {
    match key {
        PropertyKey::StaticIdentifier(identifier) => Some(Name::Text(identifier.name.to_string())),
        PropertyKey::Identifier(identifier) => Some(Name::Text(identifier.name.to_string())),
        PropertyKey::StringLiteral(literal) => Some(Name::Text(literal.value.as_str()?.to_owned())),
        PropertyKey::NumericLiteral(literal) => Some(Name::Number(literal.value)),
        _ => None,
    }
}

/// Whether a key is computed from something other than a literal.
fn is_dynamic(key: &PropertyKey, computed: bool) -> bool {
    computed
        && !matches!(
            key,
            PropertyKey::StringLiteral(_)
                | PropertyKey::NumericLiteral(_)
                | PropertyKey::BooleanLiteral(_)
                | PropertyKey::NullLiteral(_)
                | PropertyKey::BigIntLiteral(_)
                | PropertyKey::RegExpLiteral(_)
        )
}

/// The name a literal used as a member's computed key gives, when it is one.
fn literal_name(expression: &Expression) -> Option<Name> {
    match expression {
        Expression::StringLiteral(literal) => Some(Name::Text(literal.value.as_str()?.to_owned())),
        Expression::NumericLiteral(literal) => Some(Name::Number(literal.value)),
        _ => None,
    }
}

/// The text a finding names a property by.
fn display(name: &Name) -> String {
    match name {
        Name::Text(text) => text.clone(),
        Name::Number(number) => crate::rules::unicorn::known_values::number_text(*number),
    }
}

/// A property of a container: where it starts, its key, whether it is computed, and what
/// holds its own properties.
struct Property<'a> {
    /// Where it starts.
    start: u32,
    /// Its key.
    key: &'a PropertyKey<'a>,
    /// Whether its key is computed.
    computed: bool,
    /// The object or object type its value is.
    nested: Option<Container<'a>>,
}

/// The properties of a container, or none when an object literal holds a method, an accessor
/// or a `function`, which `this` may read in unseen ways.
fn properties(container: Container<'_>) -> Option<Vec<Property<'_>>> {
    match container {
        Container::Object(object) => {
            let mut list = Vec::new();
            for each in &object.properties {
                let ObjectPropertyKind::ObjectProperty(property) = each else {
                    continue;
                };
                if property.method
                    || property.kind != PropertyKind::Init
                    || matches!(property.value, Expression::FunctionExpression(_))
                {
                    return None;
                }
                list.push(Property {
                    start: property.span.start,
                    key: &property.key,
                    computed: property.computed,
                    nested: match unwrap(&property.value) {
                        Expression::ObjectExpression(nested) => Some(Container::Object(nested)),
                        _ => None,
                    },
                });
            }
            Some(list)
        }
        Container::Type(literal) => Some(
            literal
                .members
                .iter()
                .filter_map(|member| match member {
                    TSSignature::TSPropertySignature(signature) => Some(Property {
                        start: signature.span.start,
                        key: &signature.key,
                        computed: signature.computed,
                        nested: signature.type_annotation.as_ref().and_then(|annotation| {
                            match &annotation.type_annotation {
                                TSType::TSTypeLiteral(nested) => Some(Container::Type(nested)),
                                _ => None,
                            }
                        }),
                    }),
                    _ => None,
                })
                .collect(),
        ),
    }
}

/// The node holding a node and its TypeScript wrappers.
fn outer_parent(source: &Source, id: NodeId) -> NodeId {
    let nodes = source.semantic.nodes();
    let mut child = id;
    loop {
        let parent = nodes.parent_node(child);
        match parent.kind() {
            AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_) => child = parent.id(),
            _ => return parent.id(),
        }
    }
}

/// Whether a declaration's object pattern may read a key: it has a rest, a dynamic key, or the
/// key itself.
fn pattern_reads(pattern: &oxc_ast::ast::ObjectPattern, key: &Name) -> bool {
    pattern.rest.is_some()
        || pattern.properties.iter().any(|property| {
            is_dynamic(&property.key, property.computed)
                || key_name(&property.key).as_ref() == Some(key)
        })
}

/// Whether an assignment's object pattern may read a key.
fn target_reads(pattern: &oxc_ast::ast::ObjectAssignmentTarget, key: &Name) -> bool {
    pattern.rest.is_some()
        || pattern.properties.iter().any(|property| match property {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(identifier) => {
                Name::Text(identifier.binding.name.to_string()) == *key
            }
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(property) => {
                is_dynamic(&property.name, property.computed)
                    || key_name(&property.name).as_ref() == Some(key)
            }
        })
}

/// The references that still read a property: a member reading it, or anything reading the
/// object whole; `None` drops a reference that reads another property.
fn next_reference(source: &Source, reference: NodeId, key: &Name) -> Option<NodeId> {
    let nodes = source.semantic.nodes();
    let parent = outer_parent(source, reference);
    let reads_whole_member = |member: NodeId| match nodes.parent_kind(member) {
        AstKind::AssignmentExpression(_) => true,
        AstKind::CallExpression(call) => call.callee.span() == nodes.kind(member).span(),
        _ => false,
    };
    match nodes.kind(parent) {
        AstKind::StaticMemberExpression(member) => (reads_whole_member(parent)
            || Name::Text(member.property.name.to_string()) == *key)
            .then_some(parent),
        AstKind::ComputedMemberExpression(member) => {
            let name = literal_name(&member.expression);
            (reads_whole_member(parent) || name.is_none() || name.as_ref() == Some(key))
                .then_some(parent)
        }
        AstKind::PrivateFieldExpression(_) => reads_whole_member(parent).then_some(parent),
        AstKind::VariableDeclarator(declarator) => match &declarator.id {
            BindingPattern::ObjectPattern(pattern) => pattern_reads(pattern, key).then_some(parent),
            _ => Some(reference),
        },
        AstKind::AssignmentExpression(assignment) => match &assignment.left {
            oxc_ast::ast::AssignmentTarget::ObjectAssignmentTarget(pattern) => {
                target_reads(pattern, key).then_some(parent)
            }
            _ => Some(reference),
        },
        _ => Some(reference),
    }
}

impl NoUnusedProperties {
    /// Reports the unread properties of a container, following the nested objects of those read.
    fn report(
        &self,
        source: &Source,
        container: Container,
        references: &[NodeId],
        findings: &mut Vec<Finding>,
    ) {
        let Some(list) = properties(container) else {
            return;
        };
        for property in list {
            if is_dynamic(property.key, property.computed) {
                continue;
            }
            let Some(name) = key_name(property.key) else {
                continue;
            };
            if name == Name::Text("__proto__".to_owned()) {
                continue;
            }
            let next: Vec<NodeId> = references
                .iter()
                .filter_map(|reference| next_reference(source, *reference, &name))
                .collect();
            if next.is_empty() {
                findings.push(source.finding(
                    self.name(),
                    property.start,
                    format!("Property `{}` is defined but never used.", display(&name)),
                ));
            } else if let Some(nested) = property.nested {
                self.report(source, nested, &next, findings);
            }
        }
    }
}

impl Rule for NoUnusedProperties {
    fn name(&self) -> &'static str {
        "unicorn/no-unused-properties"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let scoping = source.semantic.scoping();
        let nodes = source.semantic.nodes();
        for symbol in scoping.symbol_ids() {
            if scoping.symbol_redeclarations(symbol).len() > 1 {
                continue;
            }
            let references: Vec<NodeId> = scoping
                .get_resolved_reference_ids(symbol)
                .iter()
                .map(|id| scoping.get_reference(*id))
                .filter(|reference| !reference.is_type())
                .map(oxc_semantic::Reference::node_id)
                .collect();
            let read = scoping
                .get_resolved_reference_ids(symbol)
                .iter()
                .any(|id| scoping.get_reference(*id).is_read());
            if !read {
                continue;
            }
            let declaration = scoping.symbol_declaration(symbol);
            let container = match nodes.kind(declaration) {
                AstKind::VariableDeclarator(declarator) => {
                    let statement = nodes.parent_id(declaration);
                    if matches!(nodes.parent_kind(statement), AstKind::ExportDeclaration(_)) {
                        continue;
                    }
                    match declarator.init.as_ref().map(unwrap) {
                        Some(Expression::ObjectExpression(object)) => {
                            Some(Container::Object(object))
                        }
                        Some(_) => declarator.type_annotation.as_ref().and_then(|annotation| {
                            match &annotation.type_annotation {
                                TSType::TSTypeLiteral(literal) => Some(Container::Type(literal)),
                                _ => None,
                            }
                        }),
                        None => None,
                    }
                }
                AstKind::FormalParameter(parameter)
                    if matches!(parameter.pattern, BindingPattern::BindingIdentifier(_)) =>
                {
                    parameter.type_annotation.as_ref().and_then(|annotation| {
                        match &annotation.type_annotation {
                            TSType::TSTypeLiteral(literal) => Some(Container::Type(literal)),
                            _ => None,
                        }
                    })
                }
                _ => None,
            };
            if let Some(container) = container {
                self.report(source, container, &references, findings);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnusedProperties;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnusedProperties, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_unused_properties() {
        assert_eq!(
            check(
                "const myEnum = {\n\tused: 1,\n\tunused: 2,\n};\nconsole.log(myEnum.used);\nconst {used} = myEnum;"
            ),
            ["Property `unused` is defined but never used."]
        );
        assert_eq!(
            check("function foo(args: {used: number; unused: number}) {\n\treturn args.used;\n}"),
            ["Property `unused` is defined but never used."]
        );
        assert_eq!(
            check(
                "const styles = {\n\tsuccess: {},\n\tdanger: {}\n};\nf({style: styles.success});"
            ),
            ["Property `danger` is defined but never used."]
        );
        assert_eq!(
            check("const o = {a: {b: 1, c: 2}, 0: 1};\nconsole.log(o.a.b);"),
            [
                "Property `c` is defined but never used.",
                "Property `0` is defined but never used."
            ]
        );
    }

    #[test]
    fn accepts_the_documented_uses() {
        for code in [
            "function foo(args: {used: number}) {\n\treturn args.used;\n}",
            "const myEnum = {\n\tused: 1,\n};\nconsole.log(myEnum.used);\nconst {used} = myEnum;",
            "const myEnum = {\n\tused: 1,\n\tusedToo: 2,\n};\nconsole.log(myEnum);",
            "const myEnum = {\n\tused: 1,\n\tusedToo: 2,\n};\nconsole.log(myEnum.used, myEnum.usedToo);",
            "const myEnum = {\n\tused: 1,\n\tusedToo: 2,\n};\nmyEnum[x];",
            "const foo = {\n\tused: 1,\n\tmethod() {\n\t\treturn this;\n\t}\n};\nfoo.used;",
            "const o = {a: 1, b: 2};",
            "export const o = {a: 1, b: 2}; o.a;",
            "const o = {a: 1, b: 2}; o.a = 3;",
            "const o = {a: 1, b: 2}; o.a();",
            "const o = {a: 1, b: 2}; const {a, ...rest} = o;",
            "const o = {a: 1, b: 2}; const {[k]: v} = o;",
            "const o = {a: 1, [k]: 2, __proto__: p}; o.a;",
            "const o = {a: 1, b: 2}; ({a, b} = o);",
            "const o = {a: 1, b: () => 1}; o.a; o.b;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
