//! What the signal rules read of a value, from the tree alone where `@angular-eslint/eslint-plugin`
//! 22.6.0 asks TypeScript's checker (`utils/signals.ts`, `KNOWN_SIGNAL_TYPES`): a value is a
//! signal when the file shows it, by a written type of a signal's name or by the call of one of
//! `@angular/core`'s signal factories; a type only TypeScript infers, an object's member typed
//! elsewhere and a store's state are not seen (`lint_rules`).

use oxc_ast::AstKind;
use oxc_ast::ast::{ClassElement, Expression, PropertyKey, TSType, TSTypeAnnotation, TSTypeName};
use oxc_semantic::NodeId;

use crate::expressions::{const_value, symbol, written_type};
use crate::rules::unicorn::syntax::unwrap;
use crate::source::Source;

/// The types of a signal (`KNOWN_SIGNAL_TYPES`).
pub const SIGNAL_TYPES: [&str; 5] = [
    "InputSignal",
    "InputSignalWithTransform",
    "ModelSignal",
    "Signal",
    "WritableSignal",
];

/// The functions of `@angular/core` and its `rxjs-interop` that return a signal.
const SIGNAL_FACTORIES: [&str; 10] = [
    "computed",
    "contentChild",
    "contentChildren",
    "input",
    "linkedSignal",
    "model",
    "signal",
    "toSignal",
    "viewChild",
    "viewChildren",
];

/// How deep a value is followed through the constants that hold it.
const DEPTH: usize = 4;

/// Whether a written type is a signal's: a reference to one of their names, not a union.
#[must_use]
pub fn is_signal_type(annotation: Option<&TSTypeAnnotation>) -> bool {
    matches!(annotation.map(|written| &written.type_annotation),
        Some(TSType::TSTypeReference(reference))
            if matches!(&reference.type_name, TSTypeName::IdentifierReference(name)
                if SIGNAL_TYPES.contains(&name.name.as_str())))
}

/// Whether a name is imported from one of Angular's packages under the same name.
#[must_use]
pub fn is_angular_import(source: &Source, name: &oxc_ast::ast::IdentifierReference) -> bool {
    let Some(symbol) = symbol(source, name) else {
        return false;
    };
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let AstKind::ImportSpecifier(specifier) = nodes.kind(declaration) else {
        return false;
    };
    let from_angular = nodes.ancestors(declaration).any(|ancestor| {
        matches!(ancestor.kind(), AstKind::ImportDeclaration(import)
            if import.source.value.starts_with("@angular/"))
    });
    from_angular && specifier.imported.name() == name.name.as_str()
}

/// Whether an expression calls one of Angular's signal factories, `signal(0)`,
/// `input.required()`, or makes a read-only signal, `count.asReadonly()`.
#[must_use]
pub fn is_signal_factory_call(source: &Source, expression: &Expression) -> bool {
    let Expression::CallExpression(call) = unwrap(expression) else {
        return false;
    };
    let factory = match &call.callee {
        Expression::Identifier(callee) => Some(callee),
        Expression::StaticMemberExpression(member) if member.property.name == "required" => {
            match &member.object {
                Expression::Identifier(callee) => Some(callee),
                _ => None,
            }
        }
        Expression::StaticMemberExpression(member) if member.property.name == "asReadonly" => {
            return true;
        }
        _ => None,
    };
    factory.is_some_and(|callee| {
        SIGNAL_FACTORIES.contains(&callee.name.as_str()) && is_angular_import(source, callee)
    })
}

/// Whether the class around a node declares a member of a name as a signal: a property with a
/// signal's written type, or set to a signal factory's call.
fn is_signal_member(source: &Source, id: NodeId, name: &str) -> bool {
    class_members(source, id)
        .iter()
        .any(|element| match element {
            ClassElement::PropertyDefinition(property) => {
                property.key.static_name().is_some_and(|key| key == name)
                    && (is_signal_type(property.type_annotation.as_deref())
                        || property
                            .value
                            .as_ref()
                            .is_some_and(|value| is_signal_factory_call(source, value)))
            }
            _ => false,
        })
}

/// The members of the nearest class around a node.
#[must_use]
pub fn class_members<'s, 'a>(source: &'s Source<'_, 'a>, id: NodeId) -> &'s [ClassElement<'a>] {
    source
        .semantic
        .nodes()
        .ancestors(id)
        .find_map(|ancestor| match ancestor.kind() {
            AstKind::Class(class) => Some(class.body.body.as_slice()),
            _ => None,
        })
        .unwrap_or_default()
}

/// Whether the nearest class around a node declares a getter of a name.
#[must_use]
pub fn is_getter(source: &Source, id: NodeId, name: &str) -> bool {
    class_members(source, id).iter().any(|element| {
        matches!(element, ClassElement::MethodDefinition(method)
            if method.kind == oxc_ast::ast::MethodDefinitionKind::Get
                && method.key.static_name().is_some_and(|key| key == name))
    })
}

/// Whether the file shows an expression at a node to be a signal: a name written with a
/// signal's type or bound to a signal, `this.<member>` of such a member, or the call of a
/// function of the file whose written return type is a signal's.
#[must_use]
pub fn is_signal(source: &Source, id: NodeId, expression: &Expression) -> bool {
    is_signal_within(source, id, expression, DEPTH)
}

/// [`is_signal`], following at most `depth` constants.
fn is_signal_within(source: &Source, id: NodeId, expression: &Expression, depth: usize) -> bool {
    if depth == 0 {
        return false;
    }
    match unwrap(expression) {
        Expression::Identifier(name) => {
            let Some(symbol) = symbol(source, name) else {
                return false;
            };
            is_signal_type(written_type(source, symbol))
                || const_value(source, symbol).is_some_and(|value| {
                    is_signal_factory_call(source, value)
                        || is_signal_within(source, id, value, depth - 1)
                })
        }
        Expression::StaticMemberExpression(member) => {
            matches!(member.object, Expression::ThisExpression(_))
                && is_signal_member(source, id, &member.property.name)
        }
        Expression::CallExpression(call) => match &call.callee {
            Expression::Identifier(callee) => symbol(source, callee).is_some_and(|symbol| {
                let declaration = source.semantic.scoping().symbol_declaration(symbol);
                matches!(source.semantic.nodes().kind(declaration), AstKind::Function(function)
                    if function.is_declaration()
                        && is_signal_type(function.return_type.as_deref()))
            }),
            _ => false,
        },
        _ => false,
    }
}

/// Whether a key names a member: `name` in `{ name: … }`.
#[must_use]
pub fn key_is(key: &PropertyKey, name: &str) -> bool {
    matches!(key, PropertyKey::StaticIdentifier(key) if key.name == name)
}
