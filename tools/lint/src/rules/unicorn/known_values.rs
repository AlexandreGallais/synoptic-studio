//! What the rules of objects, strings and names share (EN-080), read from the syntax alone: a
//! value known to be a string, a key known statically, a string literal's raw text and the
//! escapes in it (`eslint-plugin-unicorn`'s `utils/`, without type information).

use oxc_ast::ast::{
    BinaryOperator, Expression, TSLiteral, TSType, UnaryOperator, VariableDeclarationKind,
};

use super::syntax::{Binding, Value, binding_of, is_global, static_value, unwrap};
use crate::source::Source;

/// Whether a type annotation is `string` or a string literal type.
#[must_use]
pub fn is_string_type(annotation: &TSType) -> bool {
    match annotation {
        TSType::TSStringKeyword(_) | TSType::TSTemplateLiteralType(_) => true,
        TSType::TSLiteralType(literal) => matches!(literal.literal, TSLiteral::StringLiteral(_)),
        TSType::TSParenthesizedType(inner) => is_string_type(&inner.type_annotation),
        TSType::TSUnionType(union) => union.types.iter().all(is_string_type),
        _ => false,
    }
}

/// Whether a value is known to be a string: a string or a template, `String(…)`,
/// `String.fromCharCode(…)`, `typeof`, a concatenation with a string, a binding annotated or
/// initialized so, or a constant string (`isString`).
#[must_use]
pub fn is_string(source: &Source, expression: &Expression) -> bool {
    is_string_at(source, expression, 0)
}

/// `is_string`, a binding followed at most a few times.
fn is_string_at(source: &Source, expression: &Expression, depth: u8) -> bool {
    if depth > 4 {
        return false;
    }
    match expression {
        Expression::TSSatisfiesExpression(wrapper) if is_string_type(&wrapper.type_annotation) => {
            return true;
        }
        Expression::TSAsExpression(wrapper) => {
            return is_string_type(&wrapper.type_annotation)
                || is_string_at(source, &wrapper.expression, depth);
        }
        _ => {}
    }
    match unwrap(expression) {
        Expression::StringLiteral(_) | Expression::TemplateLiteral(_) => true,
        Expression::UnaryExpression(unary) => unary.operator == UnaryOperator::Typeof,
        Expression::BinaryExpression(binary) => {
            binary.operator == BinaryOperator::Addition
                && (is_string_at(source, &binary.left, depth)
                    || is_string_at(source, &binary.right, depth))
        }
        Expression::CallExpression(call) => {
            (is_global(source, &call.callee, "String") && !call.optional)
                || matches!(&call.callee, Expression::StaticMemberExpression(member)
                    if !call.optional
                        && matches!(member.property.name.as_str(), "fromCharCode" | "fromCodePoint")
                        && is_global(source, &member.object, "String"))
        }
        node @ Expression::Identifier(_) => match binding_of(source, node) {
            Some(Binding::Variable(declarator, kind)) => {
                if let Some(annotation) = &declarator.type_annotation {
                    return is_string_type(&annotation.type_annotation);
                }
                kind == VariableDeclarationKind::Const
                    && declarator
                        .init
                        .as_ref()
                        .is_some_and(|init| is_string_at(source, init, depth + 1))
            }
            Some(Binding::Parameter(parameter)) => parameter
                .type_annotation
                .as_ref()
                .is_some_and(|annotation| is_string_type(&annotation.type_annotation)),
            None => matches!(static_value(source, node), Some(Value::Text(_))),
        },
        _ => false,
    }
}

/// Whether a key is written statically: a literal, a private name, a template without
/// expressions, or a signed number (`isStaticPropertyKey`).
#[must_use]
pub fn is_static_key(expression: &Expression) -> bool {
    match super::syntax::unwrap_chain(expression) {
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::RegExpLiteral(_) => true,
        Expression::TemplateLiteral(template) => template.expressions.is_empty(),
        Expression::UnaryExpression(unary) => {
            matches!(
                unary.operator,
                UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus
            ) && matches!(
                unary.argument,
                Expression::NumericLiteral(_) | Expression::BigIntLiteral(_)
            )
        }
        _ => false,
    }
}

/// What a key is known to name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyName {
    /// A string key, numbers written as `String()` writes them.
    Text(String),
    /// A well-known symbol, `Symbol.iterator`, by its written text.
    Symbol(String),
}

/// The text `String()` gives a constant value.
#[must_use]
pub fn value_text(value: &Value) -> String {
    match value {
        Value::Text(text) => text.clone(),
        Value::Number(number) => number_text(*number),
        Value::Boolean(boolean) => boolean.to_string(),
        Value::Null => "null".to_owned(),
        Value::Undefined => "undefined".to_owned(),
        Value::BigInt(digits) => digits.clone(),
    }
}

/// A number as `String()` writes it, for the integers and the common fractions.
#[must_use]
pub fn number_text(number: f64) -> String {
    if number.is_nan() {
        "NaN".to_owned()
    } else if number.is_infinite() {
        if number > 0.0 {
            "Infinity"
        } else {
            "-Infinity"
        }
        .to_owned()
    } else {
        // `f64`'s shortest form matches ECMAScript's below 1e21, but for `-0`.
        let text = number.to_string();
        if text == "-0" { "0".to_owned() } else { text }
    }
}

/// The name a property key gives, when it is written statically or, computed, holds a constant
/// value or a well-known symbol (`getPropertyName`, `getStaticValue`).
#[must_use]
pub fn key_name(
    source: &Source,
    key: &oxc_ast::ast::PropertyKey,
    computed: bool,
) -> Option<KeyName> {
    if let Some(name) = key.static_name()
        && (!computed || !matches!(key, oxc_ast::ast::PropertyKey::StaticIdentifier(_)))
    {
        return Some(KeyName::Text(match key {
            oxc_ast::ast::PropertyKey::NumericLiteral(literal) => number_text(literal.value),
            _ => name.into_cow_str()?.into_owned(),
        }));
    }
    let expression = key.as_expression()?;
    expression_key_name(source, expression)
}

/// The name a computed key's expression gives: a constant value, or `Symbol.name`.
#[must_use]
pub fn expression_key_name(source: &Source, expression: &Expression) -> Option<KeyName> {
    if let Expression::StaticMemberExpression(member) = unwrap(expression)
        && is_global(source, &member.object, "Symbol")
    {
        return Some(KeyName::Symbol(format!("Symbol.{}", member.property.name)));
    }
    static_value(source, expression).map(|value| KeyName::Text(value_text(&value)))
}

/// The value an expression always has, as `static_value` reads it, numbers also computed through
/// `/`, `%` and `**` (`getStaticValue`).
#[must_use]
pub fn constant_value(source: &Source, expression: &Expression) -> Option<Value> {
    if let Some(value) = static_value(source, expression) {
        return Some(value);
    }
    let Expression::BinaryExpression(binary) = unwrap(expression) else {
        return None;
    };
    let (Some(Value::Number(left)), Some(Value::Number(right))) = (
        constant_value(source, &binary.left),
        constant_value(source, &binary.right),
    ) else {
        return None;
    };
    match binary.operator {
        BinaryOperator::Addition => Some(Value::Number(left + right)),
        BinaryOperator::Subtraction => Some(Value::Number(left - right)),
        BinaryOperator::Multiplication => Some(Value::Number(left * right)),
        BinaryOperator::Division => Some(Value::Number(left / right)),
        BinaryOperator::Remainder => Some(Value::Number(left % right)),
        BinaryOperator::Exponential => Some(Value::Number(left.powf(right))),
        _ => None,
    }
}

/// Whether a type annotation is known to describe no string: a keyword other than `string`, an
/// array, a tuple, an object type, a function, a literal other than a string, or a union of them
/// with `null` and `undefined` (`isKnownNonString`, from the annotation).
#[must_use]
pub fn is_non_string_type(annotation: &TSType) -> bool {
    match annotation {
        TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSNeverKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSVoidKeyword(_)
        | TSType::TSArrayType(_)
        | TSType::TSTupleType(_)
        | TSType::TSTypeLiteral(_)
        | TSType::TSFunctionType(_)
        | TSType::TSConstructorType(_)
        | TSType::TSNullKeyword(_)
        | TSType::TSUndefinedKeyword(_) => true,
        TSType::TSLiteralType(literal) => !matches!(literal.literal, TSLiteral::StringLiteral(_)),
        TSType::TSParenthesizedType(inner) => is_non_string_type(&inner.type_annotation),
        TSType::TSTypeOperatorType(operator) => {
            operator.operator == oxc_ast::ast::TSTypeOperatorOperator::Readonly
                && is_non_string_type(&operator.type_annotation)
        }
        TSType::TSUnionType(union) => union.types.iter().all(is_non_string_type),
        _ => false,
    }
}

/// Whether a value is known not to be a string: by its binding's annotation, or written as an
/// array, an object, a function, a class or a constant other than a string.
#[must_use]
pub fn is_known_non_string(source: &Source, expression: &Expression) -> bool {
    let expression = unwrap(expression);
    match expression {
        Expression::ArrayExpression(_)
        | Expression::ObjectExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::ClassExpression(_) => true,
        Expression::Identifier(_) => match binding_of(source, expression) {
            Some(Binding::Variable(declarator, kind)) => {
                if let Some(annotation) = &declarator.type_annotation {
                    return is_non_string_type(&annotation.type_annotation);
                }
                kind == VariableDeclarationKind::Const
                    && declarator
                        .init
                        .as_ref()
                        .is_some_and(|init| is_known_non_string(source, init))
            }
            Some(Binding::Parameter(parameter)) => {
                !parameter.optional
                    && parameter
                        .type_annotation
                        .as_ref()
                        .is_some_and(|annotation| is_non_string_type(&annotation.type_annotation))
            }
            None => static_value(source, expression)
                .is_some_and(|value| !matches!(value, Value::Text(_))),
        },
        _ => static_value(source, expression).is_some_and(|value| !matches!(value, Value::Text(_))),
    }
}
