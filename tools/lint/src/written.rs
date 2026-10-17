//! What a written type or an initializer says, read from the tree alone: `naming-convention`'s
//! `function` and `boolean` types (`REF-TSESLINT-NAMING-CONVENTION`), which the rule asked the
//! compiler for, read from the written annotation or the initializer instead (EN-076).

use oxc_ast::ast::{
    BinaryExpression, BinaryOperator, Expression, LogicalOperator, TSLiteral, TSType, UnaryOperator,
};

/// Whether a written type is a function's: a function or constructor type, or a union of them
/// with `null` and `undefined`, as `naming-convention`'s `function` type (`REF-TSESLINT-NAMING-CONVENTION`).
#[must_use]
pub fn is_function_type(written: &TSType) -> bool {
    all_but_nullish(written, &|each| {
        matches!(
            each,
            TSType::TSFunctionType(_) | TSType::TSConstructorType(_)
        )
    })
}

/// Whether a written type is boolean: `boolean`, `true` or `false`, or a union of them with
/// `null` and `undefined`, as `naming-convention`'s `boolean` type.
#[must_use]
pub fn is_boolean_type(written: &TSType) -> bool {
    all_but_nullish(written, &|each| match each {
        TSType::TSBooleanKeyword(_) => true,
        TSType::TSLiteralType(literal) => {
            matches!(literal.literal, TSLiteral::BooleanLiteral(_))
        }
        _ => false,
    })
}

/// Whether every part of a type, `null` and `undefined` left out, passes a test; a type of
/// `null` or `undefined` alone does not.
fn all_but_nullish(written: &TSType, test: &dyn Fn(&TSType) -> bool) -> bool {
    let mut parts = Vec::new();
    flatten_union(written, &mut parts);
    let kept: Vec<&TSType> = parts
        .into_iter()
        .filter(|part| {
            !matches!(
                part,
                TSType::TSNullKeyword(_) | TSType::TSUndefinedKeyword(_)
            )
        })
        .collect();

    !kept.is_empty() && kept.iter().all(|part| test(part))
}

/// The members of a union, its parentheses left out.
fn flatten_union<'t, 'a>(written: &'t TSType<'a>, parts: &mut Vec<&'t TSType<'a>>) {
    match written {
        TSType::TSUnionType(union) => {
            for each in &union.types {
                flatten_union(each, parts);
            }
        }
        TSType::TSParenthesizedType(parenthesized) => {
            flatten_union(&parenthesized.type_annotation, parts);
        }
        other => parts.push(other),
    }
}

/// An expression with its parentheses and `satisfies` left out.
fn unwrapped<'t, 'a>(expression: &'t Expression<'a>) -> &'t Expression<'a> {
    match expression {
        Expression::ParenthesizedExpression(parenthesized) => unwrapped(&parenthesized.expression),
        Expression::TSSatisfiesExpression(satisfies) => unwrapped(&satisfies.expression),
        other => other,
    }
}

/// Whether an initializer is a function: an arrow or a function expression, or a value asserted
/// to a function type.
#[must_use]
pub fn is_function_expression(expression: &Expression) -> bool {
    match unwrapped(expression) {
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => true,
        Expression::TSAsExpression(assertion) => is_function_type(&assertion.type_annotation),
        _ => false,
    }
}

/// Whether an initializer is a boolean: `true` or `false`, a negation, a comparison, `in` or
/// `instanceof`, `Boolean(…)`, a logical expression of booleans, or a value asserted to `boolean`.
#[must_use]
pub fn is_boolean_expression(expression: &Expression) -> bool {
    match unwrapped(expression) {
        Expression::BooleanLiteral(_) => true,
        Expression::UnaryExpression(unary) => unary.operator == UnaryOperator::LogicalNot,
        Expression::BinaryExpression(binary) => is_comparison(binary),
        Expression::LogicalExpression(logical) => {
            logical.operator != LogicalOperator::Coalesce
                && is_boolean_expression(&logical.left)
                && is_boolean_expression(&logical.right)
        }
        Expression::CallExpression(call) => {
            matches!(&call.callee, Expression::Identifier(callee) if callee.name == "Boolean")
        }
        Expression::TSAsExpression(assertion) => is_boolean_type(&assertion.type_annotation),
        _ => false,
    }
}

/// Whether a binary expression compares, and so is a boolean.
fn is_comparison(binary: &BinaryExpression) -> bool {
    binary.operator.is_equality()
        || binary.operator.is_compare()
        || matches!(
            binary.operator,
            BinaryOperator::In | BinaryOperator::Instanceof
        )
}

/// Whether a value can be called, as `unicorn/no-non-function-verb-prefix` tells it
/// (`REF-UNICORN-NON-FUNCTION-VERB-PREFIX`): `null`, `undefined`, `never`, `any`, `unknown` and
/// what the written code does not tell are unknown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Callability {
    /// A function or a constructor.
    Callable,
    /// A value that cannot be called.
    NotCallable,
    /// Not told by the written code.
    Unknown,
}

/// The callability of the members of a union: unknown when one is, callable when all are.
fn union_callability(parts: impl IntoIterator<Item = Callability>) -> Callability {
    let parts: Vec<Callability> = parts.into_iter().collect();
    if parts.is_empty() || parts.contains(&Callability::Unknown) {
        Callability::Unknown
    } else if parts.iter().all(|part| *part == Callability::Callable) {
        Callability::Callable
    } else {
        Callability::NotCallable
    }
}

/// The callability a written type tells.
#[must_use]
pub fn type_callability(written: &TSType) -> Callability {
    match written {
        TSType::TSFunctionType(_) | TSType::TSConstructorType(_) => Callability::Callable,
        TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSObjectKeyword(_)
        | TSType::TSStringKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSVoidKeyword(_)
        | TSType::TSLiteralType(_)
        | TSType::TSTemplateLiteralType(_)
        | TSType::TSArrayType(_)
        | TSType::TSTupleType(_)
        | TSType::TSTypeOperatorType(_)
        | TSType::TSMappedType(_) => Callability::NotCallable,
        TSType::TSTypeLiteral(literal) => {
            let has_signature = literal.members.iter().any(|member| {
                matches!(
                    member,
                    oxc_ast::ast::TSSignature::TSCallSignatureDeclaration(_)
                        | oxc_ast::ast::TSSignature::TSConstructSignatureDeclaration(_)
                )
            });
            if has_signature {
                Callability::Callable
            } else {
                Callability::NotCallable
            }
        }
        TSType::TSUnionType(_) | TSType::TSParenthesizedType(_) => {
            let mut parts = Vec::new();
            flatten_union(written, &mut parts);
            union_callability(
                parts
                    .into_iter()
                    .filter(|part| {
                        !matches!(
                            part,
                            TSType::TSNullKeyword(_) | TSType::TSUndefinedKeyword(_)
                        )
                    })
                    .map(type_callability),
            )
        }
        TSType::TSIntersectionType(intersection) => {
            let parts: Vec<Callability> = intersection.types.iter().map(type_callability).collect();
            if parts.contains(&Callability::Callable) {
                Callability::Callable
            } else if parts.contains(&Callability::Unknown) {
                Callability::Unknown
            } else {
                Callability::NotCallable
            }
        }
        TSType::TSTypeReference(reference) => match &reference.type_name {
            oxc_ast::ast::TSTypeName::IdentifierReference(name)
                if matches!(
                    name.name.as_str(),
                    "CallableFunction" | "Function" | "NewableFunction"
                ) =>
            {
                Callability::Callable
            }
            _ => Callability::Unknown,
        },
        _ => Callability::Unknown,
    }
}

/// The callability an initializer tells.
#[must_use]
pub fn expression_callability(expression: &Expression) -> Callability {
    match unwrapped(expression) {
        Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::ClassExpression(_) => Callability::Callable,
        Expression::BooleanLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::TemplateLiteral(_)
        | Expression::ArrayExpression(_)
        | Expression::ObjectExpression(_)
        | Expression::BinaryExpression(_)
        | Expression::UpdateExpression(_) => Callability::NotCallable,
        Expression::UnaryExpression(unary) if unary.operator != UnaryOperator::Void => {
            Callability::NotCallable
        }
        Expression::LogicalExpression(logical) => union_callability([
            expression_callability(&logical.left),
            expression_callability(&logical.right),
        ]),
        Expression::ConditionalExpression(conditional) => union_callability([
            expression_callability(&conditional.consequent),
            expression_callability(&conditional.alternate),
        ]),
        Expression::TSAsExpression(assertion) => type_callability(&assertion.type_annotation),
        _ => Callability::Unknown,
    }
}
