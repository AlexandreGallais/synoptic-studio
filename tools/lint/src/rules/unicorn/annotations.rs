//! What a written type says of a value, without type information: whether it is one of some
//! named types, known not to be, `null` or `undefined`, or unknown (`eslint-plugin-unicorn`
//! 78.0.0, `utils/type-helpers.js`, its paths that read annotations, type aliases, interfaces
//! and imports, the compiler not asked).

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, TSType, TSTypeName, TSTypeOperatorOperator};
use oxc_semantic::{ScopeId, SymbolId};

use crate::source::Source;

/// What a written type says of a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// One of the named types.
    Target,
    /// Known to be something else.
    NonTarget,
    /// `null` or `undefined`.
    Nullish,
    /// Not known.
    Unknown,
}

/// The named types a rule looks for, and how it reads a union.
pub struct Targets<'n> {
    /// The names of the types.
    pub names: &'n [&'n str],
    /// The names a module exports that are the types, by module.
    pub imports: &'n [(&'n str, &'n [&'n str])],
    /// Whether `null` and `undefined` in a union are left out.
    pub allow_nullish_in_mixed_union: bool,
    /// Whether a union holding something else is something else.
    pub treat_mixed_union_as_non_target: bool,
    /// Whether a name declared in the file wins over a known name.
    pub prefer_definitions: bool,
}

/// `null` or `undefined` as something else, unless the union reads them apart.
fn normalized(class: Class) -> Class {
    if class == Class::Nullish {
        Class::NonTarget
    } else {
        class
    }
}

/// What a union of classes is (`combineUnionTypes`).
fn union(classes: Vec<Class>, targets: &Targets) -> Class {
    let classes: Vec<Class> = if targets.allow_nullish_in_mixed_union {
        classes
            .into_iter()
            .filter(|each| *each != Class::Nullish)
            .collect()
    } else {
        classes.into_iter().map(normalized).collect()
    };
    if classes.is_empty() {
        return Class::NonTarget;
    }
    if classes.iter().all(|each| *each == Class::Target) {
        return Class::Target;
    }
    let non_target = if targets.treat_mixed_union_as_non_target {
        classes.contains(&Class::NonTarget)
    } else {
        classes.iter().all(|each| *each == Class::NonTarget)
    };
    if non_target {
        Class::NonTarget
    } else {
        Class::Unknown
    }
}

/// The class of a name known without its declaration (`getKnownTypeReferenceType`).
fn known(name: &str, targets: &Targets) -> Class {
    if targets.names.contains(&name) {
        Class::Target
    } else {
        Class::Unknown
    }
}

/// The dotted name of a type name, `A.B`.
fn type_name(name: &TSTypeName) -> Option<String> {
    match name {
        TSTypeName::IdentifierReference(identifier) => Some(identifier.name.to_string()),
        TSTypeName::QualifiedName(qualified) => Some(format!(
            "{}.{}",
            type_name(&qualified.left)?,
            qualified.right.name
        )),
        TSTypeName::ThisExpression(_) => None,
    }
}

/// The class of a type a name declared in the file stands for, when one is.
fn declared(
    source: &Source,
    scope: ScopeId,
    name: &str,
    targets: &Targets,
    depth: u8,
) -> Option<Class> {
    let scoping = source.semantic.scoping();
    let symbol = scoping.find_binding(scope, name.into())?;
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    let class = match nodes.kind(declaration) {
        AstKind::TSTypeAliasDeclaration(alias) => {
            class_of_type(source, scope, &alias.type_annotation, targets, depth + 1)
        }
        AstKind::TSInterfaceDeclaration(interface) => {
            if interface.extends.is_empty() {
                Class::NonTarget
            } else {
                let classes: Vec<Class> = interface
                    .extends
                    .iter()
                    .map(|heritage| match type_name(&heritage.type_name) {
                        Some(parent) if !parent.contains('.') => {
                            declared(source, scope, &parent, targets, depth + 1)
                                .unwrap_or_else(|| known(&parent, targets))
                        }
                        Some(parent) => known(&parent, targets),
                        None => Class::Unknown,
                    })
                    .map(normalized)
                    .collect();
                if classes.contains(&Class::Unknown) {
                    Class::Unknown
                } else if classes.contains(&Class::Target) {
                    Class::Target
                } else {
                    Class::NonTarget
                }
            }
        }
        AstKind::ImportSpecifier(specifier) => {
            let module = match nodes.parent_kind(declaration) {
                AstKind::ImportDeclaration(import) => Some(import.source.value.as_str()),
                _ => None,
            };
            let imported = specifier.imported.name();
            let named = module.and_then(|module| {
                targets
                    .imports
                    .iter()
                    .find(|(each, _)| *each == module)
                    .map(|(_, names)| names)
            });
            match named {
                Some(exported) if exported.contains(&imported.as_str()) => Class::Target,
                Some(_) => Class::NonTarget,
                None if targets
                    .imports
                    .iter()
                    .any(|(_, names)| names.contains(&imported.as_str()))
                    || known(&imported, targets) != Class::Unknown
                    || known(&specifier.local.name, targets) != Class::Unknown =>
                {
                    Class::NonTarget
                }
                None => Class::Unknown,
            }
        }
        AstKind::ImportDefaultSpecifier(specifier) => {
            if known(&specifier.local.name, targets) == Class::Unknown {
                Class::Unknown
            } else {
                Class::NonTarget
            }
        }
        AstKind::TSEnumDeclaration(_)
        | AstKind::Class(_)
        | AstKind::ImportNamespaceSpecifier(_) => Class::Unknown,
        _ => return None,
    };
    Some(class)
}

/// The class of a written type (`getTypeAnnotationType`).
#[must_use]
pub fn class_of_type(
    source: &Source,
    scope: ScopeId,
    written: &TSType,
    targets: &Targets,
    depth: u8,
) -> Class {
    if depth > 8 {
        return Class::Unknown;
    }
    match written {
        TSType::TSParenthesizedType(inner) => {
            class_of_type(source, scope, &inner.type_annotation, targets, depth)
        }
        TSType::TSTypeOperatorType(operator) => {
            if operator.operator == TSTypeOperatorOperator::Readonly {
                class_of_type(source, scope, &operator.type_annotation, targets, depth)
            } else {
                Class::Unknown
            }
        }
        TSType::TSTypeReference(reference) => {
            let Some(name) = type_name(&reference.type_name) else {
                return Class::Unknown;
            };
            if !targets.prefer_definitions {
                let class = known(&name, targets);
                if class != Class::Unknown {
                    return class;
                }
            }
            if name.contains('.') {
                return known(&name, targets);
            }
            declared(source, scope, &name, targets, depth).unwrap_or_else(|| known(&name, targets))
        }
        TSType::TSUnionType(types) => union(
            types
                .types
                .iter()
                .map(|each| class_of_type(source, scope, each, targets, depth + 1))
                .collect(),
            targets,
        ),
        TSType::TSIntersectionType(types) => {
            let classes: Vec<Class> = types
                .types
                .iter()
                .map(|each| normalized(class_of_type(source, scope, each, targets, depth + 1)))
                .collect();
            if classes.contains(&Class::Unknown) {
                Class::Unknown
            } else if classes.contains(&Class::Target) {
                Class::Target
            } else {
                Class::NonTarget
            }
        }
        TSType::TSNullKeyword(_) | TSType::TSUndefinedKeyword(_) => Class::Nullish,
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
        | TSType::TSConstructorType(_) => Class::NonTarget,
        _ => Class::Unknown,
    }
}

/// The class of the variable or parameter a symbol is, from its annotation, then from the
/// value a `const` is bound to (`getTypeFromVariable`).
#[must_use]
pub fn class_of_symbol(source: &Source, symbol: SymbolId, targets: &Targets, depth: u8) -> Class {
    if depth > 8 {
        return Class::Unknown;
    }
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    let scope = scoping.symbol_scope_id(symbol);
    match nodes.kind(declaration) {
        AstKind::FormalParameter(parameter) => {
            let class = parameter
                .type_annotation
                .as_ref()
                .map_or(Class::Unknown, |annotation| {
                    class_of_type(source, scope, &annotation.type_annotation, targets, depth)
                });
            if parameter.optional {
                union(vec![class, Class::Nullish], targets)
            } else {
                class
            }
        }
        AstKind::VariableDeclarator(declarator) => {
            let class = declarator
                .type_annotation
                .as_ref()
                .map_or(Class::Unknown, |annotation| {
                    class_of_type(source, scope, &annotation.type_annotation, targets, depth)
                });
            if class != Class::Unknown {
                return class;
            }
            let is_const = matches!(nodes.parent_kind(declaration),
                AstKind::VariableDeclaration(statement) if statement.kind.is_const());
            match &declarator.init {
                Some(init) if is_const && declarator.id.get_binding_identifier().is_some() => {
                    class_of_expression(source, scope, init, targets, depth + 1)
                }
                _ => Class::Unknown,
            }
        }
        _ => Class::Unknown,
    }
}

/// The class of an expression from what is written: a name's, an assertion's, a conditional's
/// (`getType` without the compiler).
#[must_use]
pub fn class_of_expression(
    source: &Source,
    scope: ScopeId,
    expression: &Expression,
    targets: &Targets,
    depth: u8,
) -> Class {
    if depth > 8 {
        return Class::Unknown;
    }
    match expression {
        Expression::Identifier(name) => name
            .reference_id
            .get()
            .and_then(|reference| {
                source
                    .semantic
                    .scoping()
                    .get_reference(reference)
                    .symbol_id()
            })
            .map_or(Class::Unknown, |symbol| {
                class_of_symbol(source, symbol, targets, depth + 1)
            }),
        Expression::TSAsExpression(assertion) => {
            match class_of_type(source, scope, &assertion.type_annotation, targets, depth) {
                Class::Unknown => {
                    class_of_expression(source, scope, &assertion.expression, targets, depth + 1)
                }
                class => class,
            }
        }
        Expression::TSTypeAssertion(assertion) => {
            match class_of_type(source, scope, &assertion.type_annotation, targets, depth) {
                Class::Unknown => {
                    class_of_expression(source, scope, &assertion.expression, targets, depth + 1)
                }
                class => class,
            }
        }
        Expression::TSNonNullExpression(inner) => {
            class_of_expression(source, scope, &inner.expression, targets, depth + 1)
        }
        Expression::TSSatisfiesExpression(inner) => {
            class_of_expression(source, scope, &inner.expression, targets, depth + 1)
        }
        Expression::ParenthesizedExpression(inner) => {
            class_of_expression(source, scope, &inner.expression, targets, depth + 1)
        }
        Expression::SequenceExpression(sequence) => {
            sequence.expressions.last().map_or(Class::Unknown, |last| {
                class_of_expression(source, scope, last, targets, depth + 1)
            })
        }
        Expression::ConditionalExpression(conditional) => union(
            vec![
                class_of_expression(source, scope, &conditional.consequent, targets, depth + 1),
                class_of_expression(source, scope, &conditional.alternate, targets, depth + 1),
            ],
            targets,
        ),
        _ => Class::Unknown,
    }
}
