//! Whether a value is a native `URL`, read from the tree alone: `new URL(…)` of the global or of
//! `URL` imported from `url` or `node:url`, a `const` bound to one, an annotation naming `URL`, or
//! a function of the file declared to return one (`eslint-plugin-unicorn` 78.0.0,
//! `utils/is-url.js`, its paths without the compiler).

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, TSType, TSTypeName};
use oxc_semantic::SymbolId;
use oxc_span::GetSpan;

use crate::rules::unicorn::syntax::symbol_of;
use crate::source::Source;

/// What a value is known to be.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UrlKind {
    /// A native `URL`.
    Url,
    /// Something else.
    Other,
    /// Not known.
    Unknown,
}

/// The kind of all of some values: `URL` when all are, something else when one is and none is
/// unknown (`combineTypes`).
fn all(kinds: &[UrlKind]) -> UrlKind {
    if kinds.iter().all(|kind| *kind == UrlKind::Url) {
        UrlKind::Url
    } else if !kinds.contains(&UrlKind::Unknown) && kinds.contains(&UrlKind::Other) {
        UrlKind::Other
    } else {
        UrlKind::Unknown
    }
}

/// The kind of a union: unknown when it mixes `URL` and something else.
fn union(kinds: &[UrlKind]) -> UrlKind {
    if kinds.contains(&UrlKind::Url) && kinds.contains(&UrlKind::Other) {
        UrlKind::Unknown
    } else {
        all(kinds)
    }
}

/// Whether a module is Node.js's `url`.
fn is_url_module(module: &str) -> bool {
    matches!(module, "url" | "node:url")
}

/// Whether a symbol is `URL` imported as a value from `url` or `node:url`.
fn is_url_import(source: &Source, symbol: SymbolId) -> bool {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let AstKind::ImportSpecifier(specifier) = nodes.kind(declaration) else {
        return false;
    };
    let AstKind::ImportDeclaration(import) = nodes.parent_kind(declaration) else {
        return false;
    };
    is_url_module(&import.source.value)
        && specifier.imported.name() == "URL"
        && !specifier.import_kind.is_type()
        && !import.import_kind.is_type()
}

/// Whether a symbol is declared for types only: a type, an interface or a type import.
fn is_type_only(source: &Source, symbol: SymbolId) -> bool {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    match nodes.kind(declaration) {
        AstKind::TSTypeAliasDeclaration(_) | AstKind::TSInterfaceDeclaration(_) => true,
        AstKind::ImportSpecifier(specifier) => {
            specifier.import_kind.is_type()
                || matches!(nodes.parent_kind(declaration), AstKind::ImportDeclaration(import) if import.import_kind.is_type())
        }
        _ => false,
    }
}

/// Whether a callee is the `URL` constructor: the global, or `URL` imported from `url`.
#[must_use]
pub fn is_url_constructor(source: &Source, callee: &Expression) -> bool {
    let Expression::Identifier(name) = callee else {
        return false;
    };
    match symbol_of(source, name) {
        None => name.name == "URL",
        Some(symbol) => {
            is_url_import(source, symbol) || (name.name == "URL" && is_type_only(source, symbol))
        }
    }
}

/// The kind a written type names (`getTypeAnnotationType`).
fn kind_of_type(source: &Source, written: &TSType, depth: u8) -> UrlKind {
    if depth > 8 {
        return UrlKind::Unknown;
    }
    match written {
        TSType::TSParenthesizedType(inner) => {
            kind_of_type(source, &inner.type_annotation, depth + 1)
        }
        TSType::TSTypeReference(reference) => {
            let TSTypeName::IdentifierReference(name) = &reference.type_name else {
                return UrlKind::Unknown;
            };
            let Some(symbol) = symbol_of(source, name) else {
                return if name.name == "URL" {
                    UrlKind::Url
                } else {
                    UrlKind::Unknown
                };
            };
            if is_url_import(source, symbol) {
                return UrlKind::Url;
            }
            let nodes = source.semantic.nodes();
            match nodes.kind(source.semantic.scoping().symbol_declaration(symbol)) {
                AstKind::TSTypeAliasDeclaration(alias) => {
                    kind_of_type(source, &alias.type_annotation, depth + 1)
                }
                AstKind::Class(_) => UrlKind::Other,
                _ => UrlKind::Unknown,
            }
        }
        TSType::TSUnionType(types) => union(
            &types
                .types
                .iter()
                .map(|each| kind_of_type(source, each, depth + 1))
                .collect::<Vec<_>>(),
        ),
        TSType::TSIntersectionType(types) => all(&types
            .types
            .iter()
            .map(|each| kind_of_type(source, each, depth + 1))
            .collect::<Vec<_>>()),
        TSType::TSImportType(import) => {
            let named_url = import
                .qualifier
                .as_ref()
                .is_some_and(|qualifier| matches!(qualifier, oxc_ast::ast::TSImportTypeQualifier::Identifier(name) if name.name == "URL"));
            if is_url_module(&import.source.value) && named_url {
                UrlKind::Url
            } else {
                UrlKind::Other
            }
        }
        _ => UrlKind::Other,
    }
}

/// The kind of the variable a name reads, declared before it (`getTypeFromVariable`).
fn kind_of_name(source: &Source, name: &oxc_ast::ast::IdentifierReference, depth: u8) -> UrlKind {
    let Some(symbol) = symbol_of(source, name) else {
        return UrlKind::Unknown;
    };
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    if scoping.symbol_redeclarations(symbol).len() > 1
        || nodes.kind(declaration).span().start > name.span.start
    {
        return UrlKind::Unknown;
    }
    match nodes.kind(declaration) {
        AstKind::VariableDeclarator(declarator) => {
            let is_const = matches!(nodes.parent_kind(declaration),
                AstKind::VariableDeclaration(statement) if statement.kind.is_const());
            let from_init = match &declarator.init {
                Some(init) if is_const && declarator.id.get_binding_identifier().is_some() => {
                    url_kind(source, init, depth + 1)
                }
                _ => UrlKind::Unknown,
            };
            if from_init != UrlKind::Unknown {
                return from_init;
            }
            declarator
                .type_annotation
                .as_ref()
                .map_or(UrlKind::Unknown, |annotation| {
                    kind_of_type(source, &annotation.type_annotation, depth + 1)
                })
        }
        AstKind::FormalParameter(parameter) => parameter
            .type_annotation
            .as_ref()
            .map_or(UrlKind::Unknown, |annotation| {
                kind_of_type(source, &annotation.type_annotation, depth + 1)
            }),
        _ => UrlKind::Unknown,
    }
}

/// The kind of a value (`getUrlType`).
#[must_use]
pub fn url_kind(source: &Source, expression: &Expression, depth: u8) -> UrlKind {
    if depth > 8 {
        return UrlKind::Unknown;
    }
    match expression {
        Expression::Identifier(name) => {
            let kind = kind_of_name(source, name, depth);
            if kind != UrlKind::Unknown {
                return kind;
            }
        }
        Expression::TSAsExpression(assertion) => {
            return match kind_of_type(source, &assertion.type_annotation, depth) {
                UrlKind::Unknown => url_kind(source, &assertion.expression, depth + 1),
                kind => kind,
            };
        }
        Expression::TSTypeAssertion(assertion) => {
            return match kind_of_type(source, &assertion.type_annotation, depth) {
                UrlKind::Unknown => url_kind(source, &assertion.expression, depth + 1),
                kind => kind,
            };
        }
        Expression::TSSatisfiesExpression(inner) => {
            return url_kind(source, &inner.expression, depth + 1);
        }
        Expression::TSNonNullExpression(inner) => {
            return url_kind(source, &inner.expression, depth + 1);
        }
        Expression::SequenceExpression(sequence) => {
            return sequence
                .expressions
                .last()
                .map_or(UrlKind::Unknown, |last| url_kind(source, last, depth + 1));
        }
        Expression::ConditionalExpression(conditional) => {
            return all(&[
                url_kind(source, &conditional.consequent, depth + 1),
                url_kind(source, &conditional.alternate, depth + 1),
            ]);
        }
        Expression::NewExpression(new) => {
            if is_url_constructor(source, &new.callee) {
                return UrlKind::Url;
            }
            return UrlKind::Other;
        }
        Expression::CallExpression(call) => {
            if let Expression::Identifier(callee) = &call.callee
                && let Some(symbol) = symbol_of(source, callee)
                && source
                    .semantic
                    .scoping()
                    .symbol_redeclarations(symbol)
                    .len()
                    <= 1
                && let AstKind::Function(function) = source
                    .semantic
                    .nodes()
                    .kind(source.semantic.scoping().symbol_declaration(symbol))
            {
                return function
                    .return_type
                    .as_ref()
                    .map_or(UrlKind::Unknown, |annotation| {
                        kind_of_type(source, &annotation.type_annotation, depth + 1)
                    });
            }
        }
        _ => {}
    }
    UrlKind::Unknown
}

/// Whether a value is a native `URL` (`isUrl`).
#[must_use]
pub fn is_url(source: &Source, expression: &Expression) -> bool {
    url_kind(source, expression, 0) == UrlKind::Url
}

/// Whether a value constructs a fresh `URL` (`isFreshUrl`).
#[must_use]
pub fn is_fresh_url(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::TSAsExpression(inner) => is_fresh_url(source, &inner.expression),
        Expression::TSSatisfiesExpression(inner) => is_fresh_url(source, &inner.expression),
        Expression::TSTypeAssertion(inner) => is_fresh_url(source, &inner.expression),
        Expression::TSNonNullExpression(inner) => is_fresh_url(source, &inner.expression),
        Expression::SequenceExpression(sequence) => sequence
            .expressions
            .last()
            .is_some_and(|last| is_fresh_url(source, last)),
        Expression::ConditionalExpression(conditional) => {
            is_fresh_url(source, &conditional.consequent)
                && is_fresh_url(source, &conditional.alternate)
        }
        Expression::NewExpression(new) => is_url_constructor(source, &new.callee),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use oxc_ast::AstKind;
    use oxc_semantic::AstNode;

    use super::{is_fresh_url, url_kind};
    use crate::finding::Finding;
    use crate::rules::Rule;
    use crate::source::Source;
    use crate::testing::{lint, messages};

    /// Reports the kind of each value given to `probe(…)`, and whether it is a fresh `URL`.
    struct Probe;

    impl Rule for Probe {
        fn name(&self) -> &'static str {
            "probe/url"
        }

        fn checks_nodes(&self) -> bool {
            true
        }

        fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
            if let AstKind::CallExpression(call) = node.kind()
                && call.callee.is_specific_id("probe")
                && let Some(value) = call.arguments[0].as_expression()
            {
                let fresh = if is_fresh_url(source, value) {
                    " fresh"
                } else {
                    ""
                };
                let message = format!("{:?}{fresh}", url_kind(source, value, 0));
                findings.push(source.finding(self.name(), call.span.start, message));
            }
        }
    }

    /// The kinds of the probes of a piece of code.
    fn kinds(code: &str) -> Vec<String> {
        messages(&lint(&Probe, "scripts/a.ts", code))
    }

    #[test]
    fn tells_urls_by_their_construction() {
        assert_eq!(
            kinds(
                "probe(new URL(a));\nprobe(new Other(a));\nprobe((new URL(a), new URL(b)));\nprobe(c ? new URL(a) : new URL(b));\nprobe(c ? new URL(a) : u);\nprobe(new URL(a)!);\nprobe(new URL(a) satisfies URL);\nprobe(<URL>u);\nprobe(u as unknown as URL);"
            ),
            [
                "Url fresh",
                "Other",
                "Url fresh",
                "Url fresh",
                "Unknown",
                "Url fresh",
                "Url fresh",
                "Url",
                "Url"
            ]
        );
        assert_eq!(
            kinds(
                "import { URL as NodeUrl } from 'node:url';\nimport type { URL as TypeUrl } from 'url';\nprobe(new NodeUrl(a));\nprobe(new TypeUrl(a));\nconst URL = class {};\nprobe(new URL(a));"
            ),
            ["Url fresh", "Other", "Other"]
        );
        assert_eq!(
            kinds("type URL = string;\nprobe(new URL(a));"),
            ["Url fresh"]
        );
    }

    #[test]
    fn tells_urls_by_their_annotations() {
        assert_eq!(
            kinds(
                "type Alias = URL;\nclass Mine {}\nfunction f(a: URL, b: Alias, c: Mine, d: URL | string, e: URL & Other, g: (URL), h: import('node:url').URL, i: import('x').Y, j: string, k: Unknown, l: Q.R) {\n  probe(a); probe(b); probe(c); probe(d); probe(e); probe(g); probe(h); probe(i); probe(j); probe(k); probe(l);\n}"
            ),
            [
                "Url", "Url", "Other", "Unknown", "Unknown", "Url", "Url", "Other", "Other",
                "Unknown", "Unknown"
            ]
        );
        assert_eq!(
            kinds(
                "import { URL as Imported } from 'url';\nconst a = new URL(x);\nlet b: URL = y;\nlet c = new URL(x);\nfunction made(): URL { return a; }\nconst made2 = (): URL => a;\nfunction f(d: Imported) { probe(d); }\nprobe(a); probe(b); probe(c); probe(made()); probe(made2()); probe(later); probe(undeclared);\nconst later = new URL(x);"
            ),
            [
                "Url", "Url", "Url", "Unknown", "Url", "Unknown", "Unknown", "Unknown"
            ]
        );
    }
}
