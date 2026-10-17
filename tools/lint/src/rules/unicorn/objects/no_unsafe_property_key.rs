//! `unicorn/no-unsafe-property-key`: no object, array, function, class, `BigInt` nor unsafe or
//! non-finite number as a property key, which JavaScript turns into a surprising string
//! (`eslint-plugin-unicorn` 78.0.0). A key is read from its syntax, its `const` binding, its
//! annotation or the file's type declarations; a value of unknown kind is left alone.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Expression, PropertyKey, TSLiteral, TSType, TSTypeName, TSTypeOperatorOperator, UnaryOperator,
    VariableDeclarationKind,
};
use oxc_semantic::{AstNode, ScopeId};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{
    Binding, Kind, Value, binding_of, is_global, kind_of, static_value, symbol_of,
};
use crate::source::Source;

/// The rule.
pub struct NoUnsafePropertyKey;

/// What the rule reports.
const MESSAGE: &str =
    "Do not use an unsafe value as a property key. Use an explicit string or symbol key instead.";

/// The globals that are objects or functions, unsafe as keys.
const UNSAFE_GLOBALS: [&str; 59] = [
    "BigInt",
    "Boolean",
    "Number",
    "String",
    "Symbol",
    "Object",
    "Array",
    "ArrayBuffer",
    "DataView",
    "Date",
    "Function",
    "Map",
    "WeakMap",
    "Set",
    "WeakSet",
    "Promise",
    "RegExp",
    "SharedArrayBuffer",
    "Proxy",
    "WeakRef",
    "FinalizationRegistry",
    "DisposableStack",
    "AsyncDisposableStack",
    "Error",
    "EvalError",
    "RangeError",
    "ReferenceError",
    "SyntaxError",
    "TypeError",
    "URIError",
    "AggregateError",
    "SuppressedError",
    "Int8Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Int16Array",
    "Uint16Array",
    "Int32Array",
    "Uint32Array",
    "Float16Array",
    "Float32Array",
    "Float64Array",
    "BigInt64Array",
    "BigUint64Array",
    "Atomics",
    "decodeURI",
    "decodeURIComponent",
    "encodeURI",
    "encodeURIComponent",
    "eval",
    "globalThis",
    "Infinity",
    "Intl",
    "isFinite",
    "isNaN",
    "JSON",
    "Math",
    "NaN",
    "parseFloat",
];

/// The globals not in `UNSAFE_GLOBALS` for their length: `parseInt`, `Reflect`, `WebAssembly`.
const MORE_UNSAFE_GLOBALS: [&str; 3] = ["parseInt", "Reflect", "WebAssembly"];

/// What a key is known to be.
#[derive(Clone, Copy, PartialEq, Eq)]
enum KeyType {
    /// Unsafe.
    Unsafe,
    /// Safe.
    Safe,
    /// Not known.
    Unknown,
}

/// Whether a number is unsafe as a key: not a safe integer, and an integer or not finite.
fn is_unsafe_number(value: f64) -> bool {
    const MAX_SAFE: f64 = 9_007_199_254_740_991.0;
    let integer = value.trunc().to_bits() == value.to_bits();
    let safe_integer = value.is_finite() && integer && value.abs() <= MAX_SAFE;
    !safe_integer && (!value.is_finite() || integer)
}

/// The number a numeric literal, signed with `-` or not, is.
fn number_literal(expression: &Expression) -> Option<f64> {
    match expression {
        Expression::NumericLiteral(literal) => Some(literal.value),
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::UnaryNegation => {
            match &unary.argument {
                Expression::NumericLiteral(literal) => Some(-literal.value),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Whether an expression is a `BigInt` literal, signed with `-` or not.
fn is_bigint_literal(expression: &Expression) -> bool {
    match expression {
        Expression::BigIntLiteral(_) => true,
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::UnaryNegation
                && matches!(unary.argument, Expression::BigIntLiteral(_))
        }
        _ => false,
    }
}

/// Whether an expression is written as an unsafe key: an object, an array, a function, a class,
/// a `new`, a `BigInt` or an unsafe number.
fn is_unsafe_node(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::ObjectExpression(_)
            | Expression::ArrayExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::ClassExpression(_)
            | Expression::NewExpression(_)
    ) || is_bigint_literal(expression)
        || number_literal(expression).is_some_and(is_unsafe_number)
}

/// Whether a type annotation describes an unsafe key: `bigint`, `object`, an array, a tuple, a
/// function, an object type with members, or a type of the file declaring one.
fn is_unsafe_annotation(source: &Source, annotation: &TSType, scope: ScopeId, depth: u8) -> bool {
    if depth > 6 {
        return false;
    }
    match annotation {
        TSType::TSBigIntKeyword(_)
        | TSType::TSObjectKeyword(_)
        | TSType::TSArrayType(_)
        | TSType::TSTupleType(_)
        | TSType::TSFunctionType(_)
        | TSType::TSConstructorType(_) => true,
        TSType::TSLiteralType(literal) => match &literal.literal {
            TSLiteral::BigIntLiteral(_) => true,
            TSLiteral::NumericLiteral(number) => is_unsafe_number(number.value),
            TSLiteral::UnaryExpression(unary) => {
                matches!(&unary.argument, Expression::BigIntLiteral(_))
                    || matches!(&unary.argument,
                Expression::NumericLiteral(number) if is_unsafe_number(-number.value))
            }
            _ => false,
        },
        TSType::TSParenthesizedType(inner) => {
            is_unsafe_annotation(source, &inner.type_annotation, scope, depth)
        }
        TSType::TSTypeOperatorType(operator) => {
            operator.operator == TSTypeOperatorOperator::Readonly
                && is_unsafe_annotation(source, &operator.type_annotation, scope, depth)
        }
        TSType::TSTypeLiteral(literal) => !literal.members.is_empty(),
        TSType::TSUnionType(union) => union
            .types
            .iter()
            .any(|each| is_unsafe_annotation(source, each, scope, depth)),
        TSType::TSIntersectionType(intersection) => intersection
            .types
            .iter()
            .all(|each| is_unsafe_annotation(source, each, scope, depth)),
        TSType::TSTypeReference(reference) => {
            let TSTypeName::IdentifierReference(name) = &reference.type_name else {
                return false;
            };
            if matches!(name.name.as_str(), "Array" | "ReadonlyArray") {
                return true;
            }
            is_unsafe_type_name(source, scope, name.name.as_str(), depth)
        }
        _ => false,
    }
}

/// Whether a type name the file declares is unsafe: an alias of an unsafe type, an interface
/// with members or a heritage, or a class.
fn is_unsafe_type_name(source: &Source, scope: ScopeId, name: &str, depth: u8) -> bool {
    let scoping = source.semantic.scoping();
    let Some(symbol) = scoping.find_binding(scope, oxc_ast::ast::Ident::from(name)) else {
        return false;
    };
    let declaration = scoping.symbol_declaration(symbol);
    let declared_scope = source.semantic.nodes().get_node(declaration).scope_id();
    match source.semantic.nodes().kind(declaration) {
        AstKind::TSTypeAliasDeclaration(alias) => {
            is_unsafe_annotation(source, &alias.type_annotation, declared_scope, depth + 1)
        }
        AstKind::TSInterfaceDeclaration(interface) => {
            !interface.body.body.is_empty() || !interface.extends.is_empty()
        }
        AstKind::Class(_) => true,
        _ => false,
    }
}

/// What a constant value makes of a key.
fn value_type(value: &Value) -> KeyType {
    match value {
        Value::BigInt(_) => KeyType::Unsafe,
        Value::Number(number) if is_unsafe_number(*number) => KeyType::Unsafe,
        _ => KeyType::Safe,
    }
}

/// What a key is, from its syntax, its binding and its annotations (`getStaticPropertyKeyType`,
/// then `isTarget`).
fn key_type(source: &Source, scope: ScopeId, expression: &Expression, depth: u8) -> KeyType {
    if depth > 6 {
        return KeyType::Unknown;
    }
    match expression {
        Expression::TemplateLiteral(_) => KeyType::Safe,
        Expression::Identifier(identifier) => {
            if symbol_of(source, identifier).is_none() {
                let name = identifier.name.as_str();
                return if UNSAFE_GLOBALS.contains(&name) || MORE_UNSAFE_GLOBALS.contains(&name) {
                    KeyType::Unsafe
                } else {
                    static_value(source, expression)
                        .map_or(KeyType::Unknown, |value| value_type(&value))
                };
            }
            identifier_type(source, scope, expression, depth)
        }
        Expression::StaticMemberExpression(member) => {
            if is_enum_member(source, &member.object) {
                KeyType::Safe
            } else {
                expression_type(source, expression)
            }
        }
        Expression::TSAsExpression(wrapper) => wrapped_type(
            source,
            scope,
            &wrapper.expression,
            &wrapper.type_annotation,
            depth,
        ),
        Expression::TSSatisfiesExpression(wrapper) => wrapped_type(
            source,
            scope,
            &wrapper.expression,
            &wrapper.type_annotation,
            depth,
        ),
        Expression::TSTypeAssertion(wrapper) => wrapped_type(
            source,
            scope,
            &wrapper.expression,
            &wrapper.type_annotation,
            depth,
        ),
        Expression::TSNonNullExpression(wrapper) => {
            key_type(source, scope, &wrapper.expression, depth + 1)
        }
        Expression::SequenceExpression(sequence) => sequence
            .expressions
            .last()
            .map_or(KeyType::Unknown, |last| {
                key_type(source, scope, last, depth + 1)
            }),
        Expression::ConditionalExpression(conditional) => {
            let consequent = key_type(source, scope, &conditional.consequent, depth + 1);
            let alternate = key_type(source, scope, &conditional.alternate, depth + 1);
            if consequent == KeyType::Unsafe || alternate == KeyType::Unsafe {
                KeyType::Unsafe
            } else if consequent == KeyType::Safe && alternate == KeyType::Safe {
                KeyType::Safe
            } else {
                KeyType::Unknown
            }
        }
        Expression::CallExpression(call) => {
            let from_value = expression_type(source, expression);
            if from_value != KeyType::Unknown {
                return from_value;
            }
            if call_returns_unsafe(source, &call.callee) {
                KeyType::Unsafe
            } else {
                KeyType::Unknown
            }
        }
        _ => expression_type(source, expression),
    }
}

/// What a wrapped key is: its expression's, or else its annotation's.
fn wrapped_type(
    source: &Source,
    scope: ScopeId,
    expression: &Expression,
    annotation: &TSType,
    depth: u8,
) -> KeyType {
    match key_type(source, scope, expression, depth + 1) {
        KeyType::Unknown if is_unsafe_annotation(source, annotation, scope, 0) => KeyType::Unsafe,
        other => other,
    }
}

/// What a name's binding makes of a key: a function, a class or an enum is unsafe; a `const`
/// is what its value is; an annotation decides otherwise.
fn identifier_type(source: &Source, scope: ScopeId, expression: &Expression, depth: u8) -> KeyType {
    let Expression::Identifier(identifier) = expression else {
        return KeyType::Unknown;
    };
    let Some(symbol) = symbol_of(source, identifier) else {
        return KeyType::Unknown;
    };
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    match nodes.kind(declaration) {
        AstKind::Function(_) | AstKind::Class(_) | AstKind::TSEnumDeclaration(_) => {
            return KeyType::Unsafe;
        }
        _ => {}
    }
    let declared_scope = nodes.get_node(declaration).scope_id();
    match binding_of(source, expression) {
        Some(Binding::Variable(declarator, kind)) => {
            if kind == VariableDeclarationKind::Const
                && let Some(init) = &declarator.init
            {
                let from_value = key_type(source, scope, init, depth + 1);
                if from_value != KeyType::Unknown {
                    return from_value;
                }
            }
            declarator
                .type_annotation
                .as_ref()
                .map_or(KeyType::Unknown, |annotation| {
                    if is_unsafe_annotation(source, &annotation.type_annotation, declared_scope, 0)
                    {
                        KeyType::Unsafe
                    } else {
                        KeyType::Unknown
                    }
                })
        }
        Some(Binding::Parameter(parameter)) => {
            parameter
                .type_annotation
                .as_ref()
                .map_or(KeyType::Unknown, |annotation| {
                    if is_unsafe_annotation(source, &annotation.type_annotation, declared_scope, 0)
                    {
                        KeyType::Unsafe
                    } else {
                        KeyType::Unknown
                    }
                })
        }
        None => KeyType::Unknown,
    }
}

/// Whether an object names an enum of the file.
fn is_enum_member(source: &Source, object: &Expression) -> bool {
    let Expression::Identifier(identifier) = object else {
        return false;
    };
    symbol_of(source, identifier).is_some_and(|symbol| {
        matches!(
            source
                .semantic
                .nodes()
                .kind(source.semantic.scoping().symbol_declaration(symbol)),
            AstKind::TSEnumDeclaration(_)
        )
    })
}

/// What an expression's syntax or constant value makes of a key.
fn expression_type(source: &Source, expression: &Expression) -> KeyType {
    let unsafe_global_property = matches!(expression, Expression::StaticMemberExpression(member)
        if ["globalThis", "self", "window"].iter().any(|name| is_global(source, &member.object, name))
            && (UNSAFE_GLOBALS.contains(&member.property.name.as_str())
                || MORE_UNSAFE_GLOBALS.contains(&member.property.name.as_str())));
    let value = crate::rules::unicorn::known_values::constant_value(source, expression);
    if is_unsafe_node(expression)
        || unsafe_global_property
        || kind_of(source, expression) == Some(Kind::Array)
        || value
            .as_ref()
            .is_some_and(|value| value_type(value) == KeyType::Unsafe)
    {
        return KeyType::Unsafe;
    }
    value.map_or(KeyType::Unknown, |_| KeyType::Safe)
}

/// Whether a callee is a function of the file whose return type is written unsafe.
fn call_returns_unsafe(source: &Source, callee: &Expression) -> bool {
    let Some(function) = crate::rules::unicorn::syntax::function_named(source, callee) else {
        return false;
    };
    let (return_type, typed) = match function {
        crate::rules::unicorn::syntax::FunctionLike::Function(function) => (
            function.return_type.as_ref(),
            function.type_parameters.is_some(),
        ),
        crate::rules::unicorn::syntax::FunctionLike::Arrow(arrow) => {
            (arrow.return_type.as_ref(), arrow.type_parameters.is_some())
        }
    };
    !typed
        && return_type.is_some_and(|annotation| {
            is_unsafe_annotation(
                source,
                &annotation.type_annotation,
                source.semantic.scoping().root_scope_id(),
                0,
            )
        })
}

impl Rule for NoUnsafePropertyKey {
    fn name(&self) -> &'static str {
        "unicorn/no-unsafe-property-key"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let key: Option<&Expression> = match node.kind() {
            AstKind::ComputedMemberExpression(member) => Some(&member.expression),
            AstKind::ObjectProperty(property) => checked_key(&property.key, property.computed),
            AstKind::MethodDefinition(method) => checked_key(&method.key, method.computed),
            AstKind::PropertyDefinition(property) => checked_key(&property.key, property.computed),
            AstKind::AccessorProperty(property) => checked_key(&property.key, property.computed),
            AstKind::BindingProperty(property) => checked_key(&property.key, property.computed),
            AstKind::AssignmentTargetPropertyProperty(property) => {
                checked_key(&property.name, property.computed)
            }
            _ => None,
        };
        let Some(key) = key else {
            return;
        };
        if key_type(source, node.scope_id(), key, 0) == KeyType::Unsafe {
            findings.push(source.finding(
                self.name(),
                oxc_span::GetSpan::span(key).start,
                MESSAGE.to_owned(),
            ));
        }
    }
}

/// The key a definition's check reads: a computed one, or a `BigInt` or unsafe number literal.
fn checked_key<'b, 'a>(key: &'b PropertyKey<'a>, computed: bool) -> Option<&'b Expression<'a>> {
    let expression = key.as_expression()?;
    (computed
        || is_bigint_literal(expression)
        || number_literal(expression).is_some_and(is_unsafe_number))
    .then_some(expression)
}

#[cfg(test)]
mod tests {
    use super::{MESSAGE, NoUnsafePropertyKey};
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnsafePropertyKey, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_unsafe_keys() {
        for code in [
            "const key = {};\nobject[key] = value;",
            "const key = [];\nconst object = {\n\t[key]: value,\n};",
            "array[4n] = value;",
            "const object = {\n\t9007199254740992: value,\n};",
            "const object = {\n\t[{}]: value,\n};",
            "object[Math] = 1;",
            "object[globalThis.JSON] = 1;",
            "function f() {} object[f] = 1;",
            "function g(key: object) { return map[key]; }",
            "function g(key: number[]) { return map[key]; }",
            "object[Infinity] = 1;",
            "object[ok ? {} : 'a'] = 1;",
            "object[key as bigint] = 1;",
            "const o = { [-1n]: 1 };",
            "type Point = { x: number }; function g(key: Point) { return map[key]; }",
            "function make(): number[] { return []; } object[make()] = 1;",
        ] {
            assert_eq!(check(code), [MESSAGE], "{code}");
        }
    }

    #[test]
    fn accepts_the_documented_safe_keys() {
        for code in [
            "object[String(key)] = value;",
            "const object = {\n\t[key.join('')]: value,\n};",
            "array[Number(4n)] = value;",
            "const object = {\n\t'9007199254740992': value,\n};",
            "const map = new Map([\n\t[{}, value],\n]);",
            "array[0] = value;",
            "array[index] = value;",
            "array[i + 1] = value;",
            "object[`key`] = 1;",
            "object[Symbol.iterator] = 1;",
            "function g(key: string) { return map[key]; }",
            "const o = { 1.5: 1, 10: 2 };",
            "enum E { A } object[E.A] = 1;",
            "object[ok ? 'a' : 'b'] = 1;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
