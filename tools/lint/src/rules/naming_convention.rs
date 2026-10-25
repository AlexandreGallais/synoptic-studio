//! `@typescript-eslint/naming-convention`: each name's casing by its role, with the project's
//! options (`the records' conventions, area naming`, `REF-TSESLINT-NAMING-CONVENTION`). The options are
//! sorted and matched as the rule sorts and matches them: a selector before a group of
//! selectors, then the most modifiers and types; the first option that applies is the only one
//! checked. Its `types` option asked the compiler for a name's type; here the type is read from
//! the written annotation, or else the initializer (`crate::written`): a name whose type cannot
//! be read takes the next option. Classes, their members, enums and accessors are left out:
//! `functional/no-classes` and `no-restricted-syntax` refuse them.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingIdentifier, BindingPattern, Expression, ModuleExportName, PropertyKey, PropertyKind,
    TSType, TSTypeAnnotation, VariableDeclarationKind, VariableDeclarator,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::BOOLEAN_PREFIXES;
use crate::source::Source;
use crate::written::{
    is_boolean_expression, is_boolean_type, is_function_expression, is_function_type,
};

/// The rule.
pub struct NamingConvention;

/// A role a name plays, with the rule's value for it (its `Selectors`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Selector {
    /// A variable.
    Variable = 1,
    /// A function's own name.
    Function = 2,
    /// A function's parameter.
    Parameter = 4,
    /// An object literal's method.
    ObjectLiteralMethod = 128,
    /// A type literal's method.
    TypeMethod = 256,
    /// An object literal's property.
    ObjectLiteralProperty = 1024,
    /// A type literal's property.
    TypeProperty = 2048,
    /// A class's name.
    Class = 8192,
    /// An interface's name.
    Interface = 16384,
    /// A type alias's name.
    TypeAlias = 32768,
    /// An enum's name.
    Enum = 65536,
    /// A type parameter.
    TypeParameter = 131_072,
    /// A default or namespace import.
    Import = 262_144,
}

impl Selector {
    /// The name the rule's messages give it: `Object Literal Property`.
    const fn label(self) -> &'static str {
        match self {
            Self::Variable => "Variable",
            Self::Function => "Function",
            Self::Parameter => "Parameter",
            Self::ObjectLiteralMethod => "Object Literal Method",
            Self::TypeMethod => "Type Method",
            Self::ObjectLiteralProperty => "Object Literal Property",
            Self::TypeProperty => "Type Property",
            Self::Class => "Class",
            Self::Interface => "Interface",
            Self::TypeAlias => "Type Alias",
            Self::Enum => "Enum",
            Self::TypeParameter => "Type Parameter",
            Self::Import => "Import",
        }
    }

    /// Whether the rule reads a type for this role (its `SelectorsAllowedToHaveTypes`).
    const fn has_types(self) -> bool {
        matches!(
            self,
            Self::Variable | Self::Parameter | Self::ObjectLiteralProperty | Self::TypeProperty
        )
    }
}

/// What an option selects: one role, or a group of them (the rule's `MetaSelectors`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Selects {
    /// One role.
    One(Selector),
    /// Every role, `default`.
    Default,
    /// The types' names, `typeLike`.
    TypeLike,
}

impl Selects {
    /// The value the rule sorts by: a group's mask, `-1` for `default`.
    const fn value(self) -> i64 {
        match self {
            Self::One(selector) => selector as i64,
            Self::Default => -1,
            Self::TypeLike => 253_952,
        }
    }

    /// Whether it selects a role.
    const fn covers(self, selector: Selector) -> bool {
        match self {
            Self::One(one) => one as i64 == selector as i64,
            Self::Default => true,
            Self::TypeLike => self.value() & selector as i64 != 0,
        }
    }
}

/// A modifier of a name, with the rule's weight for it.
const CONST: u32 = 1;
/// A modifier of a name: destructured from an object.
const DESTRUCTURED: u32 = 256;
/// A modifier of a name: declared at the module's top level.
const GLOBAL: u32 = 512;
/// A modifier of a name: a property key that is not an identifier.
const REQUIRES_QUOTES: u32 = 4096;
/// A type of a name, with the rule's weight for it: a boolean.
const BOOLEAN: u32 = 131_072;
/// A type of a name: a function.
const FUNCTION: u32 = 1_048_576;
/// The weight of a filter.
const FILTERED: u32 = 1 << 30;

/// A casing of the rule (its `PredefinedFormats`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    /// `camelCase`.
    Camel,
    /// `PascalCase`.
    Pascal,
    /// `UPPER_CASE`.
    Upper,
}

impl Format {
    /// Its name in messages.
    const fn label(self) -> &'static str {
        match self {
            Self::Camel => "camelCase",
            Self::Pascal => "PascalCase",
            Self::Upper => "UPPER_CASE",
        }
    }

    /// Whether a name is written in it, as the rule tells: the first letter's case and the
    /// underscores (`format.js`).
    fn fits(self, name: &str) -> bool {
        let Some(first) = name.chars().next() else {
            return true;
        };
        match self {
            Self::Camel => first.to_lowercase().eq([first]) && !name.contains('_'),
            Self::Pascal => first.to_uppercase().eq([first]) && !name.contains('_'),
            Self::Upper => name.to_uppercase() == name && underscores_between_words(name),
        }
    }
}

/// Whether a name's underscores only separate words: none leading, trailing or doubled.
fn underscores_between_words(name: &str) -> bool {
    !name.starts_with('_') && !name.ends_with('_') && !name.contains("__")
}

/// One option of the project's policy.
#[derive(Clone, Copy, Debug)]
struct Policy {
    /// The roles it selects.
    selects: Selects,
    /// The modifiers a name needs for it to apply.
    modifiers: u32,
    /// The types a name may have for it to apply, none for any.
    types: u32,
    /// The only name it applies to, the project's one filter.
    only: Option<&'static str>,
    /// The casings allowed; none, any.
    formats: &'static [Format],
    /// Whether a leading or trailing underscore is refused.
    no_underscores: bool,
    /// The prefixes one of which the name must start with.
    prefixes: &'static [&'static str],
}

impl Policy {
    /// An option allowing the given casings, no more.
    const fn of(selects: Selects, formats: &'static [Format]) -> Self {
        Self {
            selects,
            modifiers: 0,
            types: 0,
            only: None,
            formats,
            no_underscores: false,
            prefixes: &[],
        }
    }

    /// The weight the rule sorts options of one selector by.
    const fn weight(&self) -> u32 {
        self.modifiers | self.types | if self.only.is_some() { FILTERED } else { 0 }
    }
}

/// The project's options, one per selector (`eslint/settings/identifiers.ts`).
const POLICY: [Policy; 15] = [
    Policy {
        no_underscores: true,
        ..Policy::of(Selects::Default, &[Format::Camel])
    },
    Policy::of(
        Selects::One(Selector::Import),
        &[Format::Camel, Format::Pascal, Format::Upper],
    ),
    Policy {
        modifiers: CONST | GLOBAL,
        ..Policy::of(Selects::One(Selector::Variable), &[Format::Upper])
    },
    Policy {
        modifiers: CONST | GLOBAL,
        types: FUNCTION,
        ..Policy::of(Selects::One(Selector::Variable), &[Format::Camel])
    },
    Policy {
        modifiers: DESTRUCTURED,
        ..Policy::of(Selects::One(Selector::Variable), &[])
    },
    Policy {
        types: BOOLEAN,
        prefixes: &BOOLEAN_PREFIXES,
        ..Policy::of(Selects::One(Selector::Parameter), &[Format::Pascal])
    },
    Policy {
        types: BOOLEAN,
        prefixes: &BOOLEAN_PREFIXES,
        ..Policy::of(Selects::One(Selector::Variable), &[Format::Pascal])
    },
    Policy::of(Selects::TypeLike, &[Format::Pascal]),
    Policy::of(
        Selects::One(Selector::ObjectLiteralProperty),
        &[Format::Camel, Format::Pascal],
    ),
    Policy::of(
        Selects::One(Selector::ObjectLiteralMethod),
        &[Format::Camel, Format::Pascal],
    ),
    Policy {
        modifiers: REQUIRES_QUOTES,
        ..Policy::of(Selects::One(Selector::ObjectLiteralProperty), &[])
    },
    Policy {
        modifiers: REQUIRES_QUOTES,
        ..Policy::of(Selects::One(Selector::ObjectLiteralMethod), &[])
    },
    Policy {
        modifiers: REQUIRES_QUOTES,
        ..Policy::of(Selects::One(Selector::TypeProperty), &[])
    },
    Policy {
        modifiers: REQUIRES_QUOTES,
        ..Policy::of(Selects::One(Selector::TypeMethod), &[])
    },
    Policy {
        only: Some("IIFEs"),
        ..Policy::of(Selects::One(Selector::ObjectLiteralProperty), &[])
    },
];

/// The options that apply to a role, in the rule's order (`validator.js`): one selector before
/// a group, then by modifiers and types, heaviest first; equal options keep their order.
fn ordered(selector: Selector) -> Vec<&'static Policy> {
    let mut options: Vec<&Policy> = POLICY
        .iter()
        .filter(|option| option.selects.covers(selector))
        .collect();
    options.sort_by(|a, b| {
        if a.selects == b.selects {
            return b.weight().cmp(&a.weight());
        }
        let a_group = !matches!(a.selects, Selects::One(_));
        let b_group = !matches!(b.selects, Selects::One(_));
        a_group
            .cmp(&b_group)
            .then_with(|| b.selects.value().cmp(&a.selects.value()))
    });
    options
}

/// The types a name was read to have.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Read {
    /// The modifiers.
    modifiers: u32,
    /// The types, as far as the written code tells them.
    types: u32,
}

/// The message on a name an option refuses, or none.
fn check(selector: Selector, name: &str, read: Read) -> Option<String> {
    let label = selector.label();
    for option in ordered(selector) {
        if option.only.is_some_and(|only| only != name)
            || option.modifiers & !read.modifiers != 0
            || (option.types != 0 && selector.has_types() && option.types & read.types == 0)
        {
            continue;
        }
        if option.no_underscores && name.starts_with('_') {
            return Some(format!(
                "{label} name `{name}` must not have a leading underscore."
            ));
        }
        if option.no_underscores && name.ends_with('_') {
            return Some(format!(
                "{label} name `{name}` must not have a trailing underscore."
            ));
        }
        let trimmed = if option.prefixes.is_empty() {
            name
        } else {
            let Some(rest) = option
                .prefixes
                .iter()
                .find_map(|prefix| name.strip_prefix(prefix))
            else {
                return Some(format!(
                    "{label} name `{name}` must have one of the following prefixes: {}",
                    option.prefixes.join(", ")
                ));
            };
            rest
        };
        let quoted = read.modifiers & REQUIRES_QUOTES != 0;
        if option.formats.is_empty()
            || (!quoted && option.formats.iter().any(|format| format.fits(trimmed)))
        {
            return None;
        }
        let formats: Vec<&str> = option.formats.iter().map(|format| format.label()).collect();
        let trimmed_as = if trimmed == name {
            String::new()
        } else {
            format!(" trimmed as `{trimmed}`")
        };
        return Some(format!(
            "{label} name `{name}`{trimmed_as} must match one of the following formats: {}",
            formats.join(", ")
        ));
    }
    None
}

/// Whether a name is an identifier, so needs no quotes as a property key (`requiresQuoting`).
fn is_identifier_name(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first == '$' || first == '_' || first.is_alphabetic())
        && characters.all(|each| {
            each == '$'
                || each == '_'
                || each == '\u{200c}'
                || each == '\u{200d}'
                || each.is_alphanumeric()
        })
}

/// The types a written annotation or an initializer gives a name.
fn types_of(annotation: Option<&TSTypeAnnotation>, initializer: Option<&Expression>) -> u32 {
    match (annotation, initializer) {
        (Some(annotation), _) => types_of_type(&annotation.type_annotation),
        (None, Some(initializer)) => {
            let function = if is_function_expression(initializer) {
                FUNCTION
            } else {
                0
            };
            let boolean = if is_boolean_expression(initializer) {
                BOOLEAN
            } else {
                0
            };
            function | boolean
        }
        (None, None) => 0,
    }
}

/// The types a written type gives a name.
fn types_of_type(written: &TSType) -> u32 {
    let function = if is_function_type(written) {
        FUNCTION
    } else {
        0
    };
    let boolean = if is_boolean_type(written) { BOOLEAN } else { 0 };
    function | boolean
}

/// A property key's name and whether it needs quotes; none for a computed key.
fn key_name(key: &PropertyKey, computed: bool) -> Option<(String, bool)> {
    if computed {
        return None;
    }
    let name = match key {
        PropertyKey::StaticIdentifier(identifier) => identifier.name.to_string(),
        PropertyKey::StringLiteral(literal) => literal.value.to_string(),
        PropertyKey::NumericLiteral(literal) => literal.value.to_string(),
        _ => return None,
    };
    let quoted = !is_identifier_name(&name);
    Some((name, quoted))
}

/// The names a binding pattern declares, each with whether it is destructured by its own name
/// from an object, as the rule's `isDestructured` tells.
fn bound_names<'a>(pattern: &'a BindingPattern<'a>, found: &mut Vec<(&'a str, Span, bool)>) {
    match pattern {
        BindingPattern::BindingIdentifier(identifier) => {
            found.push((identifier.name.as_str(), identifier.span, false));
        }
        BindingPattern::ObjectPattern(object) => {
            for property in &object.properties {
                let own = match &property.value {
                    BindingPattern::BindingIdentifier(identifier) => Some(identifier),
                    BindingPattern::AssignmentPattern(assignment) => match &assignment.left {
                        BindingPattern::BindingIdentifier(identifier) => Some(identifier),
                        _ => None,
                    },
                    _ => None,
                };
                match own.filter(|_| property.shorthand) {
                    Some(identifier) => {
                        found.push((identifier.name.as_str(), identifier.span, true));
                    }
                    None => bound_names(&property.value, found),
                }
            }
            if let Some(rest) = &object.rest {
                bound_names(&rest.argument, found);
            }
        }
        BindingPattern::ArrayPattern(array) => {
            for element in array.elements.iter().flatten() {
                bound_names(element, found);
            }
            if let Some(rest) = &array.rest {
                bound_names(&rest.argument, found);
            }
        }
        BindingPattern::AssignmentPattern(assignment) => bound_names(&assignment.left, found),
    }
}

/// A name a node declares: its role, its text, where it is written and what was read of it.
type Name = (Selector, String, u32, Read);

/// The one name of a declaration, nothing read of it.
fn declared(selector: Selector, id: &BindingIdentifier) -> Vec<Name> {
    vec![(
        selector,
        id.name.to_string(),
        id.span.start,
        Read::default(),
    )]
}

/// The names a pattern binds in a role, with modifiers and, for a single name, its types.
fn pattern_names(
    pattern: &BindingPattern,
    selector: Selector,
    modifiers: u32,
    own_types: u32,
) -> Vec<Name> {
    let mut found = Vec::new();
    bound_names(pattern, &mut found);
    let types = match pattern {
        BindingPattern::BindingIdentifier(_) => own_types,
        _ => 0,
    };
    found
        .into_iter()
        .map(|(name, span, destructured)| {
            let read = Read {
                modifiers: modifiers | if destructured { DESTRUCTURED } else { 0 },
                types,
            };
            (selector, name.to_owned(), span.start, read)
        })
        .collect()
}

/// The name of a property key in a role: a function's role when the member holds one.
fn key_names(
    key: &PropertyKey,
    computed: bool,
    as_function: bool,
    roles: [Selector; 2],
) -> Vec<Name> {
    let Some((name, quoted)) = key_name(key, computed) else {
        return Vec::new();
    };
    let selector = if as_function { roles[1] } else { roles[0] };
    let modifiers = if quoted { REQUIRES_QUOTES } else { 0 };
    vec![(
        selector,
        name,
        key.span().start,
        Read {
            modifiers,
            types: 0,
        },
    )]
}

/// The names a variable declarator binds: `const` and top-level read from its place.
fn declarator_names(node: &AstNode, declarator: &VariableDeclarator, source: &Source) -> Vec<Name> {
    let is_const = matches!(
        source.semantic.nodes().parent_kind(node.id()),
        AstKind::VariableDeclaration(declaration)
            if declaration.kind == VariableDeclarationKind::Const
    );
    let is_global = node.scope_id() == source.semantic.scoping().root_scope_id();
    let modifiers = if is_const { CONST } else { 0 } | if is_global { GLOBAL } else { 0 };
    let types = types_of(
        declarator.type_annotation.as_deref(),
        declarator.init.as_ref(),
    );

    pattern_names(&declarator.id, Selector::Variable, modifiers, types)
}

/// Whether a parameter list is a function's, not a signature's: the rule reads the parameters of
/// function declarations and expressions, arrows and body-less methods.
fn is_function_parameter(node: &AstNode, source: &Source) -> bool {
    matches!(
        source
            .semantic
            .nodes()
            .ancestors(node.id())
            .nth(1)
            .map(AstNode::kind),
        Some(AstKind::Function(_) | AstKind::ArrowFunctionExpression(_))
    )
}

/// The names a node declares.
fn names(node: &AstNode, source: &Source) -> Vec<Name> {
    match node.kind() {
        AstKind::Function(function) => function
            .id
            .as_ref()
            .map_or_else(Vec::new, |id| declared(Selector::Function, id)),
        AstKind::ImportDefaultSpecifier(import) => declared(Selector::Import, &import.local),
        AstKind::ImportNamespaceSpecifier(import) => declared(Selector::Import, &import.local),
        AstKind::ImportSpecifier(import) if matches!(&import.imported, ModuleExportName::IdentifierName(name) if name.name == "default") => {
            declared(Selector::Import, &import.local)
        }
        AstKind::VariableDeclarator(declarator) => declarator_names(node, declarator, source),
        AstKind::FormalParameter(parameter) if is_function_parameter(node, source) => {
            let types = types_of(
                parameter.type_annotation.as_deref(),
                parameter.initializer.as_deref(),
            );
            pattern_names(&parameter.pattern, Selector::Parameter, 0, types)
        }
        AstKind::FormalParameterRest(rest) if is_function_parameter(node, source) => {
            pattern_names(&rest.rest.argument, Selector::Parameter, 0, 0)
        }
        AstKind::ObjectProperty(property) if property.kind == PropertyKind::Init => {
            let holds_function = matches!(
                property.value,
                Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
            );
            key_names(
                &property.key,
                property.computed,
                holds_function,
                [
                    Selector::ObjectLiteralProperty,
                    Selector::ObjectLiteralMethod,
                ],
            )
        }
        AstKind::TSPropertySignature(property) => {
            let holds_function = property.type_annotation.as_ref().is_some_and(|annotation| {
                matches!(annotation.type_annotation, TSType::TSFunctionType(_))
            });
            key_names(
                &property.key,
                property.computed,
                holds_function,
                [Selector::TypeProperty, Selector::TypeMethod],
            )
        }
        AstKind::TSMethodSignature(method) => key_names(
            &method.key,
            method.computed,
            true,
            [Selector::TypeProperty, Selector::TypeMethod],
        ),
        AstKind::Class(class) => class
            .id
            .as_ref()
            .map_or_else(Vec::new, |id| declared(Selector::Class, id)),
        AstKind::TSInterfaceDeclaration(interface) => declared(Selector::Interface, &interface.id),
        AstKind::TSTypeAliasDeclaration(alias) => declared(Selector::TypeAlias, &alias.id),
        AstKind::TSEnumDeclaration(declaration) => declared(Selector::Enum, &declaration.id),
        AstKind::TSTypeParameter(parameter)
            if matches!(
                source.semantic.nodes().parent_kind(node.id()),
                AstKind::TSTypeParameterDeclaration(_)
            ) =>
        {
            declared(Selector::TypeParameter, &parameter.name)
        }
        _ => Vec::new(),
    }
}

impl Rule for NamingConvention {
    fn name(&self) -> &'static str {
        "@typescript-eslint/naming-convention"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        for (selector, name, at, read) in names(node, source) {
            if let Some(message) = check(selector, &name, read) {
                findings.push(source.finding(self.name(), at, message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NamingConvention;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&NamingConvention, "src/math/a.ts", code))
    }

    #[test]
    fn asks_upper_case_of_module_constants_and_camel_case_of_functions() {
        assert_eq!(
            found("const maxSize = 1;\nconst MAX_SIZE = 2;"),
            ["Variable name `maxSize` must match one of the following formats: UPPER_CASE"]
        );
        // Two underscores in a row join no words: `A__B` is not UPPER_CASE.
        assert_eq!(
            found("const A__B = 1;"),
            ["Variable name `A__B` must match one of the following formats: UPPER_CASE"]
        );
        // [F19.AC2] A global constant holding a function, read from its initializer or its type.
        assert_eq!(
            found(
                "const twice = (a: number) => 2 * a;\nconst half = function (a: number) { return a / 2; };\nconst third: (a: number) => number = divider(3);"
            ),
            NONE
        );
        assert_eq!(
            found("const TWICE = (a: number) => 2 * a;"),
            ["Variable name `TWICE` must match one of the following formats: camelCase"]
        );
        // A call's result is not read: without a written type, the constant is a value.
        assert_eq!(
            found("const third = divider(3);"),
            ["Variable name `third` must match one of the following formats: UPPER_CASE"]
        );
        assert_eq!(
            found("function Twice(a: number): number { return a; }"),
            ["Function name `Twice` must match one of the following formats: camelCase"]
        );
    }

    #[test]
    fn asks_camel_case_inside_a_function_and_lets_destructured_names_be() {
        assert_eq!(
            found(
                "function f(a: number): number { const MAX = a; const { Width, height: Tall } = g(); return MAX; }"
            ),
            [
                "Variable name `MAX` must match one of the following formats: camelCase",
                "Variable name `Tall` must match one of the following formats: camelCase",
            ]
        );
        // At the top level, a destructured name still takes the global constant's option first.
        assert_eq!(
            found("const { a, B_C } = g();\nconst [first] = h();"),
            [
                "Variable name `a` must match one of the following formats: UPPER_CASE",
                "Variable name `first` must match one of the following formats: UPPER_CASE",
            ]
        );
    }

    #[test]
    fn asks_a_prefix_of_booleans_read_from_their_type_or_initializer() {
        // [F19.AC2] The boolean prefixes, the type read from the annotation or the initializer.
        assert_eq!(
            found(
                "function f(open: boolean, isShut = false, hasA: true | undefined): boolean { const ready = a < 1 && b > 2; const visible = !open; const both = open && isShut; return ready; }"
            ),
            [
                "Parameter name `open` must have one of the following prefixes: are, can, did, does, has, is, should, was, will",
                "Variable name `ready` must have one of the following prefixes: are, can, did, does, has, is, should, was, will",
                "Variable name `visible` must have one of the following prefixes: are, can, did, does, has, is, should, was, will",
            ]
        );
        assert_eq!(
            found("function f(island: boolean): boolean { const IS_X = a < 1; return island; }"),
            [
                "Parameter name `island` trimmed as `land` must match one of the following formats: PascalCase",
                "Variable name `IS_X` must have one of the following prefixes: are, can, did, does, has, is, should, was, will",
            ]
        );
        // A module constant: the boolean option weighs more than the constant's.
        assert_eq!(found("const IS_ON = true;").len(), 1);
        assert_eq!(
            found(
                "const isOn = true;\nconst hasNone = a in b;\nconst canX = Boolean(a);\nconst wasY = a instanceof B;\nconst isZ = (a as boolean);"
            ),
            NONE
        );
        // An unread type is not a boolean: `a ?? b` and a call.
        assert_eq!(
            found("function f(): void { const ready = a ?? b; const done = g(); }"),
            NONE
        );
    }

    #[test]
    fn refuses_leading_and_trailing_underscores() {
        assert_eq!(
            found("function f(_a: number, b_: number): void {}"),
            [
                "Parameter name `_a` must not have a leading underscore.",
                "Parameter name `b_` must not have a trailing underscore.",
            ]
        );
    }

    #[test]
    fn asks_pascal_case_of_types_and_reads_imports_properties_and_quotes() {
        assert_eq!(
            found(
                "type point = { readonly X: number; readonly \"a-b\": number; m(): void; readonly F: () => void };\ninterface shape {}\nfunction f<t>(): void {}\nclass widget {}\nenum color {}"
            ),
            [
                "Type Alias name `point` must match one of the following formats: PascalCase",
                "Type Property name `X` must match one of the following formats: camelCase",
                "Type Method name `F` must match one of the following formats: camelCase",
                "Interface name `shape` must match one of the following formats: PascalCase",
                "Type Parameter name `t` must match one of the following formats: PascalCase",
                "Class name `widget` must match one of the following formats: PascalCase",
                "Enum name `color` must match one of the following formats: PascalCase",
            ]
        );
        assert_eq!(
            found(
                "import fs_x from \"a\";\nimport * as Path from \"b\";\nimport { default as BIG_ONE, snake_named } from \"c\";"
            ),
            [
                "Import name `fs_x` must match one of the following formats: camelCase, PascalCase, UPPER_CASE"
            ]
        );
        assert_eq!(
            found(
                "const O = { Ab: 1, cd: 2, \"e-f\": 3, 1: 4, IIFEs: 5, G_H: 6, Run() {}, run_it: () => 1, [k]: 7 };"
            ),
            [
                "Object Literal Property name `G_H` must match one of the following formats: camelCase, PascalCase",
                "Object Literal Method name `run_it` must match one of the following formats: camelCase, PascalCase",
            ]
        );
    }

    #[test]
    fn reads_the_names_of_patterns_and_rest_parameters() {
        // A name destructured from a parameter takes no option of its own; a function type's
        // parameters are not read.
        assert_eq!(
            found(
                "function f({ a: { B } }: T, [C]: U, ...D: V): void {}\ntype G = (Wide: number) => void;"
            ),
            [
                "Parameter name `B` must match one of the following formats: camelCase",
                "Parameter name `C` must match one of the following formats: camelCase",
                "Parameter name `D` must match one of the following formats: camelCase",
            ]
        );
    }
}
