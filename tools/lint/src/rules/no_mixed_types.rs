//! `functional/no-mixed-types`: a type holds only data or only functions, never both
//! (`REF-FUNCTIONAL-PLUGIN`; interfaces and type literals checked). The plugin asked the compiler
//! whether a property's type is a function; here a member is a function when it is a method, a
//! call or construct signature, or a property or index signature whose written type is a
//! function type (`crate::written`): a property typed by a name is read as data.

use oxc_ast::AstKind;
use oxc_ast::ast::{TSSignature, TSType};
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoMixedTypes;

/// The rule's message.
const MESSAGE: &str = "Only the same kind of members allowed in types.";

/// Whether a written type is a function type, its parentheses left out.
fn is_function(written: &TSType) -> bool {
    match written {
        TSType::TSFunctionType(_) => true,
        TSType::TSParenthesizedType(parenthesized) => is_function(&parenthesized.type_annotation),
        _ => false,
    }
}

/// Whether a member is a function.
fn is_function_member(member: &TSSignature) -> bool {
    match member {
        TSSignature::TSMethodSignature(_)
        | TSSignature::TSCallSignatureDeclaration(_)
        | TSSignature::TSConstructSignatureDeclaration(_) => true,
        TSSignature::TSPropertySignature(property) => property
            .type_annotation
            .as_ref()
            .is_some_and(|annotation| is_function(&annotation.type_annotation)),
        TSSignature::TSIndexSignature(index) => is_function(&index.type_annotation.type_annotation),
    }
}

/// Whether members mix functions and data.
fn is_mixed(members: &[TSSignature]) -> bool {
    let mut kinds = members.iter().map(is_function_member);
    kinds
        .next()
        .is_some_and(|first| kinds.any(|kind| kind != first))
}

/// The members of a type alias the rule reads: a type literal, or one in `Readonly<…>`.
fn alias_members<'a>(written: &'a TSType<'a>) -> Option<&'a [TSSignature<'a>]> {
    match written {
        TSType::TSTypeLiteral(literal) => Some(&literal.members),
        TSType::TSTypeReference(reference) => {
            let arguments = &reference.type_arguments.as_ref()?.params;
            match (&reference.type_name, arguments.as_slice()) {
                (
                    oxc_ast::ast::TSTypeName::IdentifierReference(_),
                    [TSType::TSTypeLiteral(literal)],
                ) => Some(&literal.members),
                _ => None,
            }
        }
        _ => None,
    }
}

impl Rule for NoMixedTypes {
    fn name(&self) -> &'static str {
        "functional/no-mixed-types"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (members, at) = match node.kind() {
            AstKind::TSInterfaceDeclaration(interface) => {
                (Some(interface.body.body.as_slice()), interface.span.start)
            }
            AstKind::TSTypeAliasDeclaration(alias) => {
                (alias_members(&alias.type_annotation), alias.span.start)
            }
            _ => return,
        };
        if members.is_some_and(is_mixed) {
            findings.push(source.finding(self.name(), at, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoMixedTypes;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&NoMixedTypes, "src/math/a.ts", code))
    }

    #[test]
    fn reports_data_and_functions_in_one_type() {
        // [F19.AC2] The members' written annotations tell a function from data.
        for code in [
            "type T = { readonly a: number; readonly f: () => void };",
            "type T = Readonly<{ a: number; m(): void }>;",
            "interface I { a: number; (): void }",
            "type T = { readonly f: (() => void); readonly [k: string]: number };",
            "type T = { new (): T; readonly a: string };",
            // The first two alike, the third not.
            "type T = { readonly a: number; readonly b: string; readonly f: () => void };",
        ] {
            assert_eq!(
                found(code),
                ["Only the same kind of members allowed in types."],
                "{code}"
            );
        }
    }

    #[test]
    fn accepts_a_type_of_one_kind() {
        for code in [
            "type T = { readonly a: number; readonly b: string };",
            "type T = { readonly f: () => void; m(): void; readonly [k: string]: (a: number) => number };",
            "type T = {};",
            "type T = string | { readonly f: () => void; readonly a: number };",
            "type T = Partial<{ a: number; f(): void }, 1>;",
            // A property typed by a name is data, as written.
            "type T = { readonly f: Callback; readonly a: number };",
        ] {
            assert_eq!(found(code), NONE, "{code}");
        }
    }
}
