//! Whether a written type is deeply read-only, read from the tree alone: the immutability that
//! `eslint-plugin-functional` asked the compiler for (`REF-FUNCTIONAL-PLUGIN`), in the strict form
//! the Product Owner chose on 2026-10-17 — `readonly` or `Readonly<…>` written in place at each
//! level, and a type defined elsewhere never trusted by its name: it passes only wrapped in
//! `Readonly<…>`, its own declaration checked where it is written (EN-076).

use oxc_ast::ast::{
    Expression, TSMappedTypeModifierOperator, TSSignature, TSTupleElement, TSType, TSTypeName,
    TSTypeOperatorOperator, TSTypeReference,
};
use oxc_span::{GetSpan, Span};

use oxc_ast::AstKind;

use crate::finding::Fix;
use crate::source::Source;

/// The read-only generic types of the standard library, whose arguments are read deeply.
const READONLY_GENERICS: [&str; 3] = ["ReadonlyArray", "ReadonlyMap", "ReadonlySet"];

/// The utility types that keep the `readonly` of the type they read (homomorphic mapped types,
/// or filters of a union): their first argument is read.
const MODIFIERS_KEPT: [&str; 7] = [
    "Exclude",
    "Extract",
    "NonNullable",
    "Omit",
    "Partial",
    "Pick",
    "Required",
];

/// Why a part is mutable: an array.
pub const ARRAY: &str = "an array without `readonly`";
/// Why a part is mutable: a tuple.
pub const TUPLE: &str = "a tuple without `readonly`";
/// Why a part is mutable: a property.
pub const PROPERTY: &str = "a property without `readonly`";
/// Why a part is mutable: an index signature.
pub const INDEX: &str = "an index signature without `readonly`";
/// Why a part is mutable: a mapped type.
pub const MAPPED: &str = "a mapped type without `readonly`";
/// Why a part is mutable: a name.
pub const ELSEWHERE: &str = "a type defined elsewhere, not wrapped in `Readonly<…>`";
/// Why a part is mutable: `object` or `any`.
pub const OBJECT: &str = "`object` or `any`, which may hold anything mutable";
/// Why a part is mutable: a member without type.
pub const UNTYPED: &str = "a member without a written type";

/// A part of a written type that is not deeply read-only, why, and the text its fix inserts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mutable {
    /// Where it is written.
    pub span: Span,
    /// What it is, for the message.
    pub reason: &'static str,
    /// The texts its fix inserts, each at an offset; none when no fix is safe.
    pub insertions: Vec<(u32, &'static str)>,
}

/// Every part of a written type that is not deeply read-only, outer parts first.
#[must_use]
pub fn mutable_parts(written: &TSType, source: &Source) -> Vec<Mutable> {
    let mut parts = Vec::new();
    Reader {
        source,
        parts: &mut parts,
    }
    .read(written, false, false);
    parts
}

/// Every part of a signature that is not deeply read-only: an interface's member.
#[must_use]
pub fn mutable_member_parts(member: &TSSignature, source: &Source) -> Vec<Mutable> {
    let mut parts = Vec::new();
    Reader {
        source,
        parts: &mut parts,
    }
    .member(member, false);
    parts
}

/// The fix of a range holding mutable parts: their insertions applied to its text, in order.
#[must_use]
pub fn fix(range: Span, parts: &[Mutable], text: &str) -> Option<Fix> {
    let mut insertions: Vec<(u32, &str)> = parts
        .iter()
        .flat_map(|part| part.insertions.iter().copied())
        .collect();
    if insertions.is_empty() || parts.iter().any(|part| part.insertions.is_empty()) {
        return None;
    }
    // Stable: at one offset, an outer part's text comes before an inner one's.
    insertions.sort_by_key(|(at, _)| *at);
    let mut rewritten = String::new();
    let mut from = range.start;
    for (at, inserted) in insertions {
        rewritten.push_str(text.get(from as usize..at as usize)?);
        rewritten.push_str(inserted);
        from = at;
    }
    rewritten.push_str(text.get(from as usize..range.end as usize)?);

    Some(Fix {
        start: range.start,
        end: range.end,
        text: rewritten,
    })
}

/// The text of a part, for a message.
#[must_use]
pub fn written_text<'t>(part: &Mutable, text: &'t str) -> &'t str {
    text.get(part.span.start as usize..part.span.end as usize)
        .unwrap_or_default()
}

/// The name a type reference is written with, its qualifier left out: `Readonly`, `Point`.
fn reference_name<'a>(reference: &TSTypeReference<'a>) -> Option<&'a str> {
    match &reference.type_name {
        TSTypeName::IdentifierReference(identifier) => Some(identifier.name.as_str()),
        TSTypeName::QualifiedName(_) | TSTypeName::ThisExpression(_) => None,
    }
}

/// The walk over a written type that gathers its mutable parts.
struct Reader<'r, 's, 'a, 'p> {
    /// The file.
    source: &'r Source<'s, 'a>,
    /// The parts found.
    parts: &'p mut Vec<Mutable>,
}

impl Reader<'_, '_, '_, '_> {
    /// Records a part.
    fn found(&mut self, span: Span, reason: &'static str, insertions: Vec<(u32, &'static str)>) {
        self.parts.push(Mutable {
            span,
            reason,
            insertions,
        });
    }

    /// Reads a type, `readonly` already written around it when `in_readonly`; an array element
    /// needs parentheses around an inserted `readonly`.
    fn read(&mut self, written: &TSType, in_readonly: bool, in_array: bool) {
        let span = written.span();
        let readonly = if in_array {
            vec![(span.start, "(readonly "), (span.end, ")")]
        } else {
            vec![(span.start, "readonly ")]
        };
        match written {
            TSType::TSAnyKeyword(_) | TSType::TSObjectKeyword(_) => {
                self.found(span, OBJECT, Vec::new());
            }
            TSType::TSArrayType(array) => {
                if !in_readonly {
                    self.found(span, ARRAY, readonly);
                }
                self.read(&array.element_type, false, true);
            }
            TSType::TSTupleType(tuple) => {
                if !in_readonly {
                    self.found(span, TUPLE, readonly);
                }
                for element in &tuple.element_types {
                    self.element(element);
                }
            }
            TSType::TSTypeOperatorType(operator) => {
                if operator.operator == TSTypeOperatorOperator::Readonly {
                    self.read(&operator.type_annotation, true, false);
                }
            }
            TSType::TSTypeLiteral(literal) => {
                for member in &literal.members {
                    self.member(member, in_readonly);
                }
            }
            TSType::TSMappedType(mapped) => {
                let written_readonly = matches!(
                    mapped.readonly,
                    Some(TSMappedTypeModifierOperator::True | TSMappedTypeModifierOperator::Plus)
                );
                if !(in_readonly || written_readonly) {
                    let bracket = self
                        .source
                        .text
                        .get(span.start as usize..span.end as usize)
                        .and_then(|inside| inside.find('['))
                        .and_then(|at| u32::try_from(at).ok());
                    let insertions = bracket
                        .map(|at| vec![(span.start + at, "readonly ")])
                        .unwrap_or_default();
                    self.found(span, MAPPED, insertions);
                }
                if let Some(value) = &mapped.type_annotation {
                    self.read(value, false, false);
                }
            }
            TSType::TSUnionType(union) => {
                for each in &union.types {
                    self.read(each, false, false);
                }
            }
            TSType::TSIntersectionType(intersection) => {
                for each in &intersection.types {
                    self.read(each, false, false);
                }
            }
            TSType::TSParenthesizedType(parenthesized) => {
                self.read(&parenthesized.type_annotation, in_readonly, false);
            }
            TSType::TSConditionalType(conditional) => {
                self.read(&conditional.true_type, false, false);
                self.read(&conditional.false_type, false, false);
            }
            TSType::TSTypeReference(reference) => self.reference(reference, span),
            TSType::TSIndexedAccessType(_) | TSType::TSTypeQuery(_)
                if is_constant_query(written, self.source) => {}
            TSType::TSImportType(_) | TSType::TSIndexedAccessType(_) | TSType::TSTypeQuery(_) => {
                self.found(
                    span,
                    ELSEWHERE,
                    vec![(span.start, "Readonly<"), (span.end, ">")],
                );
            }
            TSType::TSNamedTupleMember(member) => match member.element_type.as_ts_type() {
                Some(element) => self.read(element, in_readonly, false),
                None => self.element(&member.element_type),
            },
            _ => {}
        }
    }

    /// Reads a tuple's element, inside a tuple.
    fn element(&mut self, element: &TSTupleElement) {
        match element {
            TSTupleElement::TSOptionalType(optional) => {
                self.read(&optional.type_annotation, false, false);
            }
            // The rest of a read-only tuple is read-only: its array's elements are read.
            TSTupleElement::TSRestType(rest) => self.read(&rest.type_annotation, true, false),
            _ => {
                if let Some(written) = element.as_ts_type() {
                    self.read(written, false, false);
                }
            }
        }
    }

    /// Reads a type literal's member, `readonly` implied when `in_readonly`.
    fn member(&mut self, member: &TSSignature, in_readonly: bool) {
        match member {
            TSSignature::TSPropertySignature(property) => {
                if !(property.readonly || in_readonly) {
                    self.found(
                        property.span,
                        PROPERTY,
                        vec![(property.span.start, "readonly ")],
                    );
                }
                match &property.type_annotation {
                    Some(annotation) => self.read(&annotation.type_annotation, false, false),
                    None => self.found(property.span, UNTYPED, Vec::new()),
                }
            }
            TSSignature::TSIndexSignature(index) => {
                if !(index.readonly || in_readonly) {
                    self.found(index.span, INDEX, vec![(index.span.start, "readonly ")]);
                }
                self.read(&index.type_annotation.type_annotation, false, false);
            }
            // A function's signature holds no data; `functional/prefer-property-signatures`
            // refuses a method signature.
            TSSignature::TSCallSignatureDeclaration(_)
            | TSSignature::TSConstructSignatureDeclaration(_)
            | TSSignature::TSMethodSignature(_) => {}
        }
    }

    /// Reads a type reference: `Readonly<…>` and the read-only generics are read deeply, any
    /// other name is a type defined elsewhere, to wrap in `Readonly<…>`; type arguments are read.
    fn reference(&mut self, reference: &TSTypeReference, span: Span) {
        let name = reference_name(reference);
        let arguments = reference
            .type_arguments
            .as_ref()
            .map(|arguments| arguments.params.as_slice())
            .unwrap_or_default();

        if name == Some("Readonly") {
            match arguments.first() {
                Some(TSType::TSTypeReference(wrapped)) => {
                    for argument in wrapped
                        .type_arguments
                        .iter()
                        .flat_map(|inner| inner.params.iter())
                    {
                        self.read(argument, false, false);
                    }
                }
                // A type read from a value or another type, wrapped in place; or nothing wrapped.
                Some(
                    TSType::TSImportType(_)
                    | TSType::TSIndexedAccessType(_)
                    | TSType::TSTypeQuery(_),
                )
                | None => {}
                Some(wrapped) => self.read(wrapped, true, false),
            }
            return;
        }
        if name.is_some_and(|name| MODIFIERS_KEPT.contains(&name)) {
            if let Some(object) = arguments.first() {
                self.read(object, false, false);
            }
            return;
        }
        let read_only_generic = name.is_some_and(|name| READONLY_GENERICS.contains(&name));
        if !(read_only_generic || proven_by_declaration(reference, self.source)) {
            self.found(
                span,
                ELSEWHERE,
                vec![(span.start, "Readonly<"), (span.end, ">")],
            );
        }
        for argument in arguments {
            self.read(argument, false, false);
        }
    }
}

/// Whether a named type is proven deeply read-only where it is declared, in the functional core
/// whose files the two immutability rules check: a type alias or an interface of the same file,
/// or a type imported by a relative path — the core imports only from the core (its layers,
/// `import/no-restricted-paths`); or a type parameter, whose argument is the caller's type, written
/// and checked where the caller is. A global of the standard library or a package's type is not:
/// it passes only wrapped in `Readonly<…>`.
#[must_use]
pub fn proven_by_declaration(reference: &TSTypeReference, source: &Source) -> bool {
    let TSTypeName::IdentifierReference(identifier) = &reference.type_name else {
        return false;
    };
    let scoping = source.semantic.scoping();
    let Some(symbol) = identifier
        .reference_id
        .get()
        .and_then(|id| scoping.get_reference(id).symbol_id())
    else {
        return false;
    };
    let flags = scoping.symbol_flags(symbol);
    if flags.is_type_alias() || flags.is_interface() || flags.is_type_parameter() {
        return true;
    }
    let declaration = source.semantic.symbol_declaration(symbol);
    flags.is_import()
        && source
            .semantic
            .nodes()
            .ancestors(declaration.id())
            .find_map(|ancestor| match ancestor.kind() {
                AstKind::ImportDeclaration(import) => Some(import.source.value.starts_with('.')),
                _ => None,
            })
            .unwrap_or(false)
}

/// Whether a type reads a constant of the same file declared `as const`, deeply read-only where
/// it is written: `typeof ALIGNMENTS`, `(typeof ALIGNMENTS)[number]`.
fn is_constant_query(written: &TSType, source: &Source) -> bool {
    match written {
        TSType::TSParenthesizedType(parenthesized) => {
            is_constant_query(&parenthesized.type_annotation, source)
        }
        TSType::TSIndexedAccessType(access) => is_constant_query(&access.object_type, source),
        TSType::TSTypeQuery(query) => {
            let Some(TSTypeName::IdentifierReference(identifier)) =
                query.expr_name.as_ts_type_name()
            else {
                return false;
            };
            let scoping = source.semantic.scoping();
            identifier
                .reference_id
                .get()
                .and_then(|id| scoping.get_reference(id).symbol_id())
                .is_some_and(|symbol| {
                    match source.semantic.symbol_declaration(symbol).kind() {
                        AstKind::VariableDeclarator(declarator) => {
                            matches!(&declarator.init, Some(Expression::TSAsExpression(assertion))
                                if matches!(&assertion.type_annotation, TSType::TSTypeReference(constant)
                                    if reference_name(constant) == Some("const")))
                        }
                        _ => false,
                    }
                })
        }
        _ => false,
    }
}
