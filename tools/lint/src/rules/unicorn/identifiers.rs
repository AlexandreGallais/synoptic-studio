//! What Unicorn's naming rules share (EN-080): the declared names they check, a free name for a
//! rename, and the edits renaming a binding everywhere (`eslint-plugin-unicorn`'s
//! `shared/identifier-checks`, `utils/get-available-variable-name`, `fix/rename-variable`).

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, Expression, ModuleExportName};
use oxc_semantic::{ScopeId, SymbolId};
use oxc_span::{GetSpan, Span};

use crate::rules::unicorn::modules::globals::{ES_GLOBALS, is_identifier_name};
use crate::rules::unicorn::syntax::Edit;
use crate::source::Source;

/// The words TypeScript reserves, which no new name may be.
const RESERVED_WORDS: [&str; 60] = [
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "as",
    "implements",
    "interface",
    "let",
    "package",
    "private",
    "protected",
    "public",
    "static",
    "yield",
    "any",
    "boolean",
    "constructor",
    "declare",
    "get",
    "module",
    "require",
    "number",
    "set",
    "string",
    "symbol",
    "type",
    "from",
    "of",
];

/// The first letter in upper case.
#[must_use]
pub fn upper_first(text: &str) -> String {
    let mut characters = text.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

/// The first letter in lower case.
#[must_use]
pub fn lower_first(text: &str) -> String {
    let mut characters = text.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_lowercase().chain(characters).collect()
    })
}

/// Whether a text reads the same in upper case: a constant's name, a digit, `_`.
#[must_use]
pub fn is_upper_case(text: &str) -> bool {
    text == text.to_uppercase()
}

/// Whether a text's first character reads the same in upper case.
#[must_use]
pub fn is_upper_first(text: &str) -> bool {
    text.chars()
        .next()
        .is_some_and(|first| first.to_uppercase().eq(std::iter::once(first)))
}

/// Whether an import's source is a module of the project: a relative or absolute path.
fn is_internal_source(source: &str) -> bool {
    !source.contains("node_modules") && (source.starts_with('.') || source.starts_with('/'))
}

/// The module a declaration of a name imports or requires, when it is a default or namespace
/// import, a shorthand named import, or a `require()` bound to the name; with whether it is the
/// shorthand named kind.
fn imported_module<'a>(source: &Source<'_, 'a>, symbol: SymbolId) -> Option<(&'a str, bool)> {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let import_source = || {
        nodes
            .ancestors(declaration)
            .find_map(|node| match node.kind() {
                AstKind::ImportDeclaration(import) => Some(import.source.value.as_str()),
                _ => None,
            })
    };
    match nodes.kind(declaration) {
        AstKind::ImportDefaultSpecifier(_) | AstKind::ImportNamespaceSpecifier(_) => {
            import_source().map(|module| (module, false))
        }
        AstKind::ImportSpecifier(specifier) => {
            let default = matches!(&specifier.imported, ModuleExportName::IdentifierName(name) if name.name == "default");
            let shorthand = specifier.imported.span() == specifier.local.span;
            (default || shorthand)
                .then(import_source)
                .flatten()
                .map(|module| (module, shorthand && !default))
        }
        AstKind::VariableDeclarator(declarator) => {
            if !matches!(declarator.id, BindingPattern::BindingIdentifier(_)) {
                return None;
            }
            let Some(Expression::CallExpression(call)) = &declarator.init else {
                return None;
            };
            match (&call.callee, call.arguments.as_slice()) {
                (Expression::Identifier(callee), [oxc_ast::ast::Argument::StringLiteral(name)])
                    if callee.name == "require" && !call.optional =>
                {
                    Some((name.value.as_str(), false))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// The symbols the naming rules check by default: every declared name but an enum member, a
/// default, namespace or shorthand import of a package, a `require()` of one, and a shorthand
/// destructured name (`checkShorthandProperties: false`).
#[must_use]
pub fn checked_symbols(source: &Source) -> Vec<SymbolId> {
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let shorthand: std::collections::HashSet<SymbolId> = nodes
        .iter()
        .filter_map(|node| match node.kind() {
            AstKind::BindingIdentifier(binding) => {
                let shorthand_value = matches!(nodes.parent_kind(node.id()),
                    AstKind::BindingProperty(property)
                        if property.shorthand && matches!(property.value, BindingPattern::BindingIdentifier(_)));
                shorthand_value.then(|| binding.symbol_id.get()).flatten()
            }
            _ => None,
        })
        .collect();
    scoping
        .symbol_ids()
        .filter(|symbol| {
            let declaration = scoping.symbol_declaration(*symbol);
            if matches!(nodes.kind(declaration), AstKind::TSEnumMember(_)) {
                return false;
            }
            if let Some((module, _)) = imported_module(source, *symbol) {
                return is_internal_source(module);
            }
            !shorthand.contains(symbol)
        })
        .collect()
}

/// The scopes a name is used in: those of its references, then its own.
#[must_use]
pub fn scopes_of(source: &Source, symbol: SymbolId) -> Vec<ScopeId> {
    let scoping = source.semantic.scoping();
    scoping
        .get_resolved_reference_ids(symbol)
        .iter()
        .map(|id| scoping.get_reference(*id).scope_id())
        .chain(std::iter::once(scoping.symbol_scope_id(symbol)))
        .collect()
}

/// The names fixes renamed to, each with the scopes it was given in.
#[derive(Default)]
pub struct GeneratedNames(Vec<(String, ScopeId)>);

impl GeneratedNames {
    /// Records a name a fix renames to.
    pub fn add(&mut self, name: &str, scopes: &[ScopeId]) {
        for scope in scopes {
            self.0.push((name.to_owned(), *scope));
        }
    }

    /// Whether a fix already renamed to a name in one of the scopes.
    fn taken(&self, name: &str, scopes: &[ScopeId]) -> bool {
        self.0
            .iter()
            .any(|(each, scope)| each == name && scopes.contains(scope))
    }
}

/// Whether a name is free in a scope: no binding there or above, no global of `ESLint`, no
/// `arguments` of a function around, and no undeclared name read inside it.
fn is_free(source: &Source, name: &str, scope: ScopeId) -> bool {
    let scoping = source.semantic.scoping();
    if scoping
        .find_binding(scope, oxc_ast::ast::Ident::from(name))
        .is_some()
        || ES_GLOBALS.contains(&name)
    {
        return false;
    }
    if name == "arguments"
        && scoping.scope_ancestors(scope).any(|each| {
            let flags = scoping.scope_flags(each);
            flags.is_function() && !flags.is_arrow()
        })
    {
        return false;
    }
    !scoping
        .root_unresolved_references()
        .iter()
        .filter(|(key, _)| key.as_str() == name)
        .flat_map(|(_, references)| references.iter())
        .any(|reference| {
            scoping
                .scope_ancestors(scoping.get_reference(*reference).scope_id())
                .any(|each| each == scope)
        })
}

/// The wanted name, or it followed by as many `_` as it takes to be a valid identifier, free in
/// every scope and not given by another fix; `None` when no `_` makes it valid.
#[must_use]
pub fn available_name(
    source: &Source,
    wanted: &str,
    scopes: &[ScopeId],
    generated: &GeneratedNames,
) -> Option<String> {
    let valid = |name: &str| is_identifier_name(name) && !RESERVED_WORDS.contains(&name);
    let mut name = wanted.to_owned();
    if !valid(&name) {
        name.push('_');
        if !valid(&name) {
            return None;
        }
    }
    while !scopes.iter().all(|scope| is_free(source, &name, *scope))
        || generated.taken(&name, scopes)
    {
        name.push('_');
    }
    Some(name)
}

/// Whether a name can be renamed: none of its declarations is exported.
#[must_use]
pub fn is_renamable(source: &Source, symbol: SymbolId) -> bool {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let statement = match nodes.kind(declaration) {
        AstKind::VariableDeclarator(declarator) => {
            if !matches!(declarator.id, BindingPattern::BindingIdentifier(_)) {
                return true;
            }
            nodes.parent_id(declaration)
        }
        AstKind::Function(_)
        | AstKind::Class(_)
        | AstKind::TSTypeAliasDeclaration(_)
        | AstKind::TSInterfaceDeclaration(_)
        | AstKind::TSEnumDeclaration(_) => declaration,
        _ => return true,
    };
    !matches!(nodes.parent_kind(statement), AstKind::ExportDeclaration(_))
}

/// The edits renaming a symbol at its declaration and every reference, a shorthand property,
/// import or export kept by writing both names (`renameVariable`).
#[must_use]
pub fn rename_edits(source: &Source, symbol: SymbolId, replacement: &str) -> Vec<Edit> {
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let name = scoping.symbol_name(symbol);
    let mut edits = Vec::new();
    let declared = scoping.symbol_span(symbol);
    for node in nodes.iter() {
        let (span, parent) = match node.kind() {
            AstKind::BindingIdentifier(binding) if binding.span == declared => {
                (binding.span, nodes.parent_node(node.id()))
            }
            AstKind::IdentifierReference(reference)
                if reference
                    .reference_id
                    .get()
                    .is_some_and(|id| scoping.get_reference(id).symbol_id() == Some(symbol)) =>
            {
                (reference.span, nodes.parent_node(node.id()))
            }
            _ => continue,
        };
        let shorthand_left = matches!(parent.kind(), AstKind::AssignmentPattern(_))
            && matches!(nodes.parent_kind(parent.id()), AstKind::BindingProperty(property) if property.shorthand);
        let text = match parent.kind() {
            AstKind::ObjectProperty(property) if property.shorthand => {
                format!("{name}: {replacement}")
            }
            AstKind::BindingProperty(property) if property.shorthand => {
                format!("{name}: {replacement}")
            }
            AstKind::AssignmentTargetPropertyIdentifier(_) => format!("{name}: {replacement}"),
            AstKind::ImportSpecifier(specifier)
                if specifier.imported.span() == specifier.local.span =>
            {
                format!("{name} as {replacement}")
            }
            AstKind::ExportSpecifier(specifier)
                if specifier.local.span() == specifier.exported.span() =>
            {
                format!("{replacement} as {name}")
            }
            _ if shorthand_left => format!("{name}: {replacement}"),
            _ => replacement.to_owned(),
        };
        edits.push(Edit { span, text });
    }
    edits
}

/// The span of a symbol's declared name.
#[must_use]
pub fn declared_span(source: &Source, symbol: SymbolId) -> Span {
    source.semantic.scoping().symbol_span(symbol)
}
