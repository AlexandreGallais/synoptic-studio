//! What a value is known to be, read from the tree without type information, as
//! `createTypeCheckers` of `eslint-plugin-unicorn` 78.0.0 reads it (`utils/type-helpers.js`): its
//! annotation, its `as`, the `const` it is bound to, the return type of the function it calls,
//! its construction, or a node form a kind names. Each kind says which type names, constructors,
//! calls, nodes and annotations stand for it.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, Expression, TSType, TSTypeName, TSTypeOperatorOperator, VariableDeclarationKind,
};
use oxc_semantic::ScopeId;

use crate::rules::unicorn::syntax::{TypeDefinition, symbol_of, type_definition};
use crate::source::Source;

/// What a value is known to be, against one kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Known {
    /// Of the kind.
    Target,
    /// Of another kind.
    Other,
    /// `null` or `undefined`.
    Nullish,
    /// Not known.
    Unknown,
}

/// A kind of value and the forms that tell it.
pub struct Kind {
    /// The type names that stand for it, `Map` and `ReadonlyMap`.
    pub names: &'static [&'static str],
    /// The constructors whose instance it is, `new Name(…)`, by name.
    pub constructors: &'static [&'static str],
    /// The functions whose result it is, `Name(…)`, by name.
    pub calls: &'static [&'static str],
    /// The other nodes it is, read alone.
    pub node: fn(&Source, &Expression) -> bool,
    /// The annotations other than a type name that stand for it.
    pub annotation: fn(&TSType) -> bool,
}

/// How deep bindings and aliases are followed.
const DEPTH: u8 = 6;

/// A union's members combined: all of the kind, all of another, or not known; `null` and
/// `undefined` count as another kind.
fn union(members: impl Iterator<Item = Known>) -> Known {
    let members: Vec<Known> = members
        .map(|each| {
            if each == Known::Nullish {
                Known::Other
            } else {
                each
            }
        })
        .collect();
    if members.is_empty() || members.iter().all(|each| *each == Known::Other) {
        Known::Other
    } else if members.iter().all(|each| *each == Known::Target) {
        Known::Target
    } else {
        Known::Unknown
    }
}

/// What an annotation tells.
#[must_use]
pub fn of_annotation(
    source: &Source,
    scope: ScopeId,
    annotation: &TSType,
    kind: &Kind,
    depth: u8,
) -> Known {
    if depth > DEPTH {
        return Known::Unknown;
    }
    match annotation {
        TSType::TSParenthesizedType(inner) => {
            of_annotation(source, scope, &inner.type_annotation, kind, depth)
        }
        TSType::TSTypeOperatorType(operator) => {
            if operator.operator == TSTypeOperatorOperator::Readonly {
                of_annotation(source, scope, &operator.type_annotation, kind, depth)
            } else {
                Known::Unknown
            }
        }
        TSType::TSTypeReference(reference) => {
            let TSTypeName::IdentifierReference(name) = &reference.type_name else {
                return Known::Unknown;
            };
            if kind.names.contains(&name.name.as_str()) {
                return Known::Target;
            }
            match type_definition(source, scope, &name.name) {
                Some(TypeDefinition::Alias(aliased)) => {
                    of_annotation(source, scope, aliased, kind, depth + 1)
                }
                Some(TypeDefinition::Object) => Known::Other,
                _ => Known::Unknown,
            }
        }
        TSType::TSUnionType(members) => union(
            members
                .types
                .iter()
                .map(|each| of_annotation(source, scope, each, kind, depth)),
        ),
        TSType::TSIntersectionType(members) => {
            let members: Vec<Known> = members
                .types
                .iter()
                .map(|each| of_annotation(source, scope, each, kind, depth))
                .collect();
            if members.contains(&Known::Unknown) {
                Known::Unknown
            } else if members.contains(&Known::Target) {
                Known::Target
            } else {
                Known::Other
            }
        }
        other if (kind.annotation)(other) => Known::Target,
        TSType::TSNullKeyword(_) | TSType::TSUndefinedKeyword(_) => Known::Nullish,
        TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSNeverKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSStringKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSVoidKeyword(_)
        | TSType::TSArrayType(_)
        | TSType::TSLiteralType(_)
        | TSType::TSTupleType(_)
        | TSType::TSTypeLiteral(_)
        | TSType::TSFunctionType(_)
        | TSType::TSConstructorType(_) => Known::Other,
        _ => Known::Unknown,
    }
}

/// What a name bound once tells: its annotation, else the `const` value it holds.
fn of_binding(
    source: &Source,
    scope: ScopeId,
    name: &oxc_ast::ast::IdentifierReference,
    kind: &Kind,
    depth: u8,
) -> Known {
    let Some(symbol) = symbol_of(source, name) else {
        return Known::Unknown;
    };
    let scoping = source.semantic.scoping();
    if scoping.symbol_redeclarations(symbol).len() > 1 {
        return Known::Unknown;
    }
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    match nodes.kind(declaration) {
        AstKind::VariableDeclarator(declarator) => {
            if let Some(annotation) = &declarator.type_annotation {
                let known =
                    of_annotation(source, scope, &annotation.type_annotation, kind, depth + 1);
                if known != Known::Unknown {
                    return known;
                }
            }
            let constant = matches!(nodes.parent_kind(declaration),
                AstKind::VariableDeclaration(statement) if statement.kind == VariableDeclarationKind::Const);
            match (&declarator.id, &declarator.init) {
                (BindingPattern::BindingIdentifier(_), Some(init)) if constant => {
                    of_value(source, scope, init, kind, depth + 1)
                }
                _ => Known::Unknown,
            }
        }
        AstKind::FormalParameter(parameter) => {
            let Some(annotation) = &parameter.type_annotation else {
                return Known::Unknown;
            };
            let known = of_annotation(source, scope, &annotation.type_annotation, kind, depth + 1);
            if parameter.optional {
                union([known, Known::Nullish].into_iter())
            } else {
                known
            }
        }
        _ => Known::Unknown,
    }
}

/// What the return type of a function a call names, declared once and never reassigned, tells.
fn of_call(source: &Source, scope: ScopeId, callee: &Expression, kind: &Kind, depth: u8) -> Known {
    let Expression::Identifier(name) = callee else {
        return Known::Unknown;
    };
    let Some(symbol) = symbol_of(source, name) else {
        return Known::Unknown;
    };
    let scoping = source.semantic.scoping();
    let written = scoping
        .get_resolved_references(symbol)
        .any(oxc_semantic::Reference::is_write);
    if written || scoping.symbol_redeclarations(symbol).len() > 1 {
        return Known::Unknown;
    }
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    let returns = match nodes.kind(declaration) {
        AstKind::Function(function) if function.type_parameters.is_none() => {
            function.return_type.as_deref()
        }
        AstKind::VariableDeclarator(declarator) => {
            let constant = matches!(nodes.parent_kind(declaration),
                AstKind::VariableDeclaration(statement) if statement.kind == VariableDeclarationKind::Const);
            let mut init = declarator.init.as_ref();
            while let Some(
                Expression::TSSatisfiesExpression(_) | Expression::TSNonNullExpression(_),
            ) = init
            {
                init = match init {
                    Some(Expression::TSSatisfiesExpression(inner)) => Some(&inner.expression),
                    Some(Expression::TSNonNullExpression(inner)) => Some(&inner.expression),
                    _ => None,
                };
            }
            match init {
                Some(Expression::ArrowFunctionExpression(arrow))
                    if constant && arrow.type_parameters.is_none() =>
                {
                    arrow.return_type.as_deref()
                }
                Some(Expression::FunctionExpression(function))
                    if constant && function.type_parameters.is_none() =>
                {
                    function.return_type.as_deref()
                }
                _ => None,
            }
        }
        _ => None,
    };
    returns.map_or(Known::Unknown, |annotation| {
        of_annotation(source, scope, &annotation.type_annotation, kind, depth + 1)
    })
}

/// What a value is known to be.
#[must_use]
pub fn of_value(
    source: &Source,
    scope: ScopeId,
    expression: &Expression,
    kind: &Kind,
    depth: u8,
) -> Known {
    if depth > DEPTH {
        return Known::Unknown;
    }
    let known = match expression {
        Expression::TSSatisfiesExpression(inner) => {
            return of_value(source, scope, &inner.expression, kind, depth);
        }
        Expression::TSAsExpression(inner) => {
            match of_annotation(source, scope, &inner.type_annotation, kind, depth) {
                Known::Unknown => of_value(source, scope, &inner.expression, kind, depth),
                known => known,
            }
        }
        Expression::TSTypeAssertion(inner) => {
            match of_annotation(source, scope, &inner.type_annotation, kind, depth) {
                Known::Unknown => of_value(source, scope, &inner.expression, kind, depth),
                known => known,
            }
        }
        Expression::TSNonNullExpression(inner) => {
            of_value(source, scope, &inner.expression, kind, depth)
        }
        Expression::ParenthesizedExpression(inner) => {
            of_value(source, scope, &inner.expression, kind, depth)
        }
        Expression::SequenceExpression(sequence) => {
            sequence.expressions.last().map_or(Known::Unknown, |last| {
                of_value(source, scope, last, kind, depth)
            })
        }
        Expression::ConditionalExpression(conditional) => union(
            [
                of_value(source, scope, &conditional.consequent, kind, depth),
                of_value(source, scope, &conditional.alternate, kind, depth),
            ]
            .into_iter(),
        ),
        Expression::Identifier(name) => of_binding(source, scope, name, kind, depth),
        Expression::CallExpression(call) => of_call(source, scope, &call.callee, kind, depth),
        _ => Known::Unknown,
    };
    if known != Known::Unknown {
        return known;
    }
    let made = match expression {
        Expression::NewExpression(new) => {
            matches!(&new.callee, Expression::Identifier(name) if kind.constructors.contains(&name.name.as_str()))
        }
        Expression::CallExpression(call) => {
            matches!(&call.callee, Expression::Identifier(name) if !call.optional && kind.calls.contains(&name.name.as_str()))
        }
        _ => false,
    };
    if made || (kind.node)(source, expression) {
        Known::Target
    } else {
        Known::Unknown
    }
}
