//! `unicorn/no-invalid-property-descriptor`: the descriptors given to `Object.defineProperty()`,
//! `Reflect.defineProperty()`, `Object.defineProperties()` and `Object.create()` are objects,
//! never mix `get` or `set` with `value` or `writable`, hold a function or `undefined` in `get`
//! and `set`, and name no other field (`eslint-plugin-unicorn` 78.0.0). A descriptor is read
//! inline or from a local `const` object used only as one. Its rename of `writeable` is a
//! suggestion in `ESLint`: none here.

use std::collections::HashSet;

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, BindingPattern, CallExpression, Expression, ObjectExpression, ObjectProperty,
    ObjectPropertyKind, PropertyKind, UnaryOperator, VariableDeclarationKind,
};
use oxc_semantic::{NodeId, SymbolId};
use oxc_span::GetSpan;

use super::known_values::{KeyName, expression_key_name, key_name};
use super::syntax::{Value, static_value, symbol_of, text, unwrap};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoInvalidPropertyDescriptor;

/// The fields a descriptor may hold.
const FIELDS: [&str; 6] = [
    "value",
    "writable",
    "get",
    "set",
    "enumerable",
    "configurable",
];

/// The argument a call takes its descriptor, or its map of descriptors, from.
struct DescriptorArgument<'b, 'a> {
    /// The argument.
    node: &'b Expression<'a>,
    /// Whether it maps names to descriptors.
    is_map: bool,
}

impl Rule for NoInvalidPropertyDescriptor {
    fn name(&self) -> &'static str {
        "unicorn/no-invalid-property-descriptor"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut checked = HashSet::new();
        for node in source.semantic.nodes().iter() {
            let AstKind::CallExpression(call) = node.kind() else {
                continue;
            };
            let Some(argument) = descriptor_argument(source, call) else {
                continue;
            };
            if !argument.is_map {
                descriptor_problems(source, argument.node, &mut checked, findings);
                continue;
            }
            let Some(map) = object_expression(source, argument.node, true) else {
                continue;
            };
            let Some(properties) = effective_properties(source, map, true) else {
                continue;
            };
            for (_, property) in properties {
                if property.kind == PropertyKind::Init {
                    descriptor_problems(source, &property.value, &mut checked, findings);
                }
            }
        }
    }
}

/// The descriptor argument of a plain `Object` or `Reflect` call that takes one, no spread
/// before it.
fn descriptor_argument<'b, 'a>(
    source: &Source,
    call: &'b CallExpression<'a>,
) -> Option<DescriptorArgument<'b, 'a>> {
    if call.optional {
        return None;
    }
    let (object, method) = match &call.callee {
        Expression::StaticMemberExpression(member) if !member.optional => (
            &member.object,
            KeyName::Text(member.property.name.to_string()),
        ),
        Expression::ComputedMemberExpression(member) if !member.optional => (
            &member.object,
            expression_key_name(source, &member.expression)?,
        ),
        _ => return None,
    };
    let Expression::Identifier(object) = object else {
        return None;
    };
    if !matches!(object.name.as_str(), "Object" | "Reflect") {
        return None;
    }
    let KeyName::Text(method) = method else {
        return None;
    };
    let is_map =
        object.name == "Object" && matches!(method.as_str(), "defineProperties" | "create");
    if method != "defineProperty" && !is_map {
        return None;
    }
    let index = if is_map { 1 } else { 2 };
    if call.arguments.len() <= index
        || call.arguments[..=index]
            .iter()
            .any(|argument| matches!(argument, Argument::SpreadElement(_)))
    {
        return None;
    }
    Some(DescriptorArgument {
        node: call.arguments[index].as_expression()?,
        is_map,
    })
}

/// The `const` object a name holds, when the file declares it once, unexported, initialized
/// with an object literal.
fn object_variable<'a>(
    source: &Source<'_, 'a>,
    symbol: SymbolId,
) -> Option<&'a ObjectExpression<'a>> {
    let nodes = source.semantic.nodes();
    let scoping = source.semantic.scoping();
    let declaration = scoping.symbol_declaration(symbol);
    let AstKind::VariableDeclarator(declarator) = nodes.kind(declaration) else {
        return None;
    };
    if !matches!(declarator.id, BindingPattern::BindingIdentifier(_)) {
        return None;
    }
    let statement = nodes.parent_id(declaration);
    let AstKind::VariableDeclaration(variables) = nodes.kind(statement) else {
        return None;
    };
    if variables.kind != VariableDeclarationKind::Const
        || matches!(nodes.parent_kind(statement), AstKind::ExportDeclaration(_))
    {
        return None;
    }
    match unwrap(declarator.init.as_ref()?) {
        Expression::ObjectExpression(object) => Some(object),
        _ => None,
    }
}

/// The node above a node and its TypeScript wrappers, and the outermost of them.
fn outer(source: &Source, id: NodeId) -> (NodeId, NodeId) {
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
            _ => return (parent.id(), child),
        }
    }
}

/// Whether a node is a call's descriptor argument of the kind asked.
fn is_argument_of(source: &Source, parent: NodeId, child: NodeId, is_map: bool) -> bool {
    let AstKind::CallExpression(call) = source.semantic.nodes().kind(parent) else {
        return false;
    };
    descriptor_argument(source, call)
        .is_some_and(|argument| argument.is_map == is_map && argument.node.node_id() == child)
}

/// Whether an object literal is a map of descriptors: a call's map argument, or the
/// initializer of a `const` used only as one.
fn is_safe_map_object(source: &Source, id: NodeId) -> bool {
    let (parent, child) = outer(source, id);
    if is_argument_of(source, parent, child, true) {
        return true;
    }
    let AstKind::VariableDeclarator(declarator) = source.semantic.nodes().kind(parent) else {
        return false;
    };
    if declarator.init.as_ref().map(Expression::node_id) != Some(child) {
        return false;
    }
    let BindingPattern::BindingIdentifier(binding) = &declarator.id else {
        return false;
    };
    let symbol = binding.symbol_id();
    object_variable(source, symbol).is_some() && is_safe_variable(source, symbol, true)
}

/// Whether a name is read, and only as a descriptor, or a map of them, or as a field's value
/// in such a map.
fn is_safe_variable(source: &Source, symbol: SymbolId, is_map: bool) -> bool {
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let references = scoping.get_resolved_reference_ids(symbol);
    !references.is_empty()
        && references.iter().all(|id| {
            let reference = scoping.get_reference(*id);
            if !reference.is_read() || reference.is_write() {
                return false;
            }
            let (parent, child) = outer(source, reference.node_id());
            if is_argument_of(source, parent, child, is_map) {
                return true;
            }
            if is_map {
                return false;
            }
            let AstKind::ObjectProperty(property) = nodes.kind(parent) else {
                return false;
            };
            property.kind == PropertyKind::Init
                && !property.method
                && property.value.node_id() == child
                && is_safe_map_object(source, nodes.parent_id(parent))
        })
}

/// The object literal a descriptor, or a map of them, is: written inline, or held by a safe
/// local `const` declared before.
fn object_expression<'a>(
    source: &Source<'_, 'a>,
    expression: &'a Expression<'a>,
    is_map: bool,
) -> Option<&'a ObjectExpression<'a>> {
    match unwrap(expression) {
        Expression::ObjectExpression(object) => Some(object),
        Expression::Identifier(identifier) => {
            let symbol = symbol_of(source, identifier)?;
            let object = object_variable(source, symbol)?;
            let declared = source
                .semantic
                .nodes()
                .kind(source.semantic.scoping().symbol_declaration(symbol));
            (declared.span().end <= identifier.span.start
                && is_safe_variable(source, symbol, is_map))
            .then_some(object)
        }
        _ => None,
    }
}

/// The properties an object literal defines, by name, when none is spread nor computed
/// unknowably; `__proto__: null` is skipped, another prototype gives up on a descriptor.
fn effective_properties<'a>(
    source: &Source<'_, 'a>,
    object: &'a ObjectExpression<'a>,
    is_map: bool,
) -> Option<Vec<(KeyName, &'a ObjectProperty<'a>)>> {
    let mut properties: Vec<(KeyName, &ObjectProperty)> = Vec::new();
    for each in &object.properties {
        let ObjectPropertyKind::ObjectProperty(property) = each else {
            return None;
        };
        let key = key_name(source, &property.key, property.computed)?;
        let prototype = key == KeyName::Text("__proto__".to_owned())
            && !property.computed
            && !property.shorthand
            && !property.method
            && property.kind == PropertyKind::Init;
        if prototype {
            if !is_map && !matches!(unwrap(&property.value), Expression::NullLiteral(_)) {
                return None;
            }
            continue;
        }
        properties.retain(|(name, _)| *name != key);
        properties.push((key, property));
    }
    Some(properties)
}

/// Whether an expression is known not to be an object: a literal but a regular expression, an
/// operation, a template, or a constant primitive.
fn is_non_object(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::BinaryExpression(_)
        | Expression::TemplateLiteral(_)
        | Expression::UnaryExpression(_)
        | Expression::UpdateExpression(_)
        | Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_) => true,
        other => static_value(source, other).is_some(),
    }
}

/// Whether an accessor field holds what is neither a function nor `undefined`.
fn is_non_callable(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::Void => false,
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::ArrayExpression(_)
        | Expression::ObjectExpression(_)
        | Expression::BinaryExpression(_)
        | Expression::TemplateLiteral(_)
        | Expression::UnaryExpression(_)
        | Expression::UpdateExpression(_) => true,
        other => static_value(source, other).is_some_and(|value| value != Value::Undefined),
    }
}

/// The problems of one descriptor, each object literal checked once.
fn descriptor_problems(
    source: &Source,
    expression: &Expression,
    checked: &mut HashSet<u32>,
    findings: &mut Vec<Finding>,
) {
    let rule = "unicorn/no-invalid-property-descriptor";
    let Some(descriptor) = object_expression(source, expression, false) else {
        if is_non_object(source, expression) {
            findings.push(source.finding(
                rule,
                expression.span().start,
                "A property descriptor must be an object.".to_owned(),
            ));
        }
        return;
    };
    if !checked.insert(descriptor.span.start) {
        return;
    }
    let Some(properties) = effective_properties(source, descriptor, false) else {
        return;
    };
    let has = |name: &str| {
        properties
            .iter()
            .any(|(key, _)| *key == KeyName::Text(name.to_owned()))
    };
    if (has("get") || has("set")) && (has("value") || has("writable")) {
        findings.push(
            source.finding(
                rule,
                descriptor.span.start,
                "A property descriptor cannot combine `get` or `set` with `value` or `writable`."
                    .to_owned(),
            ),
        );
    }
    for (key, property) in &properties {
        let name = match key {
            KeyName::Text(name) if FIELDS.contains(&name.as_str()) => name.as_str(),
            KeyName::Text(name) => {
                findings.push(source.finding(
                    rule,
                    property.key.span().start,
                    format!("Unknown property descriptor field `{name}`."),
                ));
                continue;
            }
            KeyName::Symbol(_) => {
                findings.push(source.finding(
                    rule,
                    property.key.span().start,
                    format!(
                        "Unknown property descriptor field `{}`.",
                        text(source, property.key.span())
                    ),
                ));
                continue;
            }
        };
        if matches!(name, "get" | "set")
            && property.kind == PropertyKind::Init
            && !property.method
            && is_non_callable(source, &property.value)
        {
            findings.push(source.finding(
                rule,
                property.value.span().start,
                format!("The `{name}` field must be a function or `undefined`."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidPropertyDescriptor;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoInvalidPropertyDescriptor, "playground/a.ts", code))
    }

    const MIXED: &str =
        "A property descriptor cannot combine `get` or `set` with `value` or `writable`.";

    #[test]
    fn reports_the_documented_invalid_descriptors() {
        assert_eq!(
            check(
                "Object.defineProperty(object, 'property', {\n\tget: getter,\n\twritable: false,\n});"
            ),
            [MIXED]
        );
        assert_eq!(
            check("Reflect.defineProperty(object, 'property', {get: null});"),
            ["The `get` field must be a function or `undefined`."]
        );
        assert_eq!(
            check(
                "Object.defineProperties(object, {\n\tproperty: {value: 1, writeable: true},\n});"
            ),
            ["Unknown property descriptor field `writeable`."]
        );
        assert_eq!(
            check("Object.create(prototype, {\n\tproperty: 42,\n});"),
            ["A property descriptor must be an object."]
        );
        assert_eq!(
            check(
                "const descriptor = {get: getter, value: undefined};\nObject.defineProperty(object, 'property', descriptor);"
            ),
            [MIXED]
        );
        assert_eq!(
            check(
                "Object.defineProperty({}, 'property', {\n\tget: () => 1,\n\twritable: false,\n} satisfies PropertyDescriptor);"
            ),
            [MIXED]
        );
        assert_eq!(
            check("Object.defineProperty(o, 'p', {[Symbol.iterator]: 1, set: 'x'});"),
            [
                "Unknown property descriptor field `Symbol.iterator`.",
                "The `set` field must be a function or `undefined`."
            ]
        );
        assert_eq!(
            check("Object.defineProperty(o, 'p', undefined);"),
            ["A property descriptor must be an object."]
        );
    }

    #[test]
    fn accepts_the_documented_valid_descriptors_and_unknown_ones() {
        for code in [
            "Object.defineProperty(object, 'property', {get: getter});",
            "Reflect.defineProperty(object, 'property', {get: undefined});",
            "Object.defineProperties(object, {\n\tproperty: {value: 1, writable: true},\n});",
            "Object.create(prototype, {\n\tproperty: {value: 42},\n});",
            "const descriptor = {get: getter};\nconst descriptors = {property: descriptor};\nObject.create(prototype, descriptors);",
            "Object.defineProperty({}, 'property', {\n\tget: () => 1,\n} satisfies PropertyDescriptor);",
            "Object.defineProperty(o, 'p', {});",
            "Object.defineProperty(o, 'p', {get: void 0, get set() { return 1; }});",
            "Object.defineProperty(o, 'p', {...rest, writeable: 1});",
            "Object.defineProperty(o, 'p', descriptor);",
            "Object?.defineProperty(o, 'p', {writeable: 1});",
            "Object.freeze(o, 'p', {writeable: 1});",
            "const d = {writeable: 1}; d.x; Object.defineProperty(o, 'p', d);",
            "export const d = {writeable: 1}; Object.defineProperty(o, 'p', d);",
            "Object.defineProperty(o, 'p', {__proto__: base, writeable: 1});",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
