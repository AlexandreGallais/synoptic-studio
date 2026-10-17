//! `functional/type-declaration-immutability`: in the functional core, every declared type is
//! deeply read-only (`REF-FUNCTIONAL-PLUGIN`; the project's one rule: every name, `ReadonlyDeep`
//! at least). The plugin asked the compiler for the type's immutability; this rule reads the
//! declaration in its strict form (`crate::deep_readonly`): every property, index signature, array
//! and tuple marked `readonly` in place, a type defined elsewhere wrapped in `Readonly<…>`. The
//! fix writes what is missing in a type alias; an interface, which `consistent-type-definitions`
//! refuses, is reported without one.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::deep_readonly::{
    ELSEWHERE, Mutable, fix, mutable_member_parts, mutable_parts, written_text,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct TypeDeclarationImmutability;

impl Rule for TypeDeclarationImmutability {
    fn name(&self) -> &'static str {
        "functional/type-declaration-immutability"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (name, at, parts, fixed) = match node.kind() {
            AstKind::TSTypeAliasDeclaration(alias) => {
                let parts = mutable_parts(&alias.type_annotation, source);
                let fixed = fix(alias.type_annotation.span(), &parts, source.text);
                (alias.id.name.as_str(), alias.span.start, parts, fixed)
            }
            AstKind::TSInterfaceDeclaration(interface) => {
                let heritage = interface.extends.iter().map(|heritage| Mutable {
                    span: heritage.span,
                    reason: ELSEWHERE,
                    insertions: Vec::new(),
                });
                let members = interface
                    .body
                    .body
                    .iter()
                    .flat_map(|member| mutable_member_parts(member, source));
                let parts: Vec<Mutable> = heritage.chain(members).collect();
                (
                    interface.id.name.as_str(),
                    interface.span.start,
                    parts,
                    None,
                )
            }
            _ => return,
        };
        let Some(first) = parts.first() else {
            return;
        };
        let message = format!(
            "`{name}` should be deeply read-only, written in place: `{}` is {}.",
            written_text(first, source.text),
            first.reason
        );
        findings.push(match fixed {
            Some(fixed) => source.fixed(self.name(), at, message, fixed),
            None => source.finding(self.name(), at, message),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::TypeDeclarationImmutability;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of the functional core.
    fn core(code: &str) -> Vec<String> {
        messages(&lint(&TypeDeclarationImmutability, "src/math/a.ts", code))
    }

    #[test]
    fn refuses_a_declaration_mutable_in_place() {
        assert_eq!(
            core("type P = { readonly x: number; y: number };"),
            [
                "`P` should be deeply read-only, written in place: `y: number` is a property without `readonly`."
            ]
        );
        assert_eq!(
            core("type L = { readonly items: string[] };"),
            [
                "`L` should be deeply read-only, written in place: `string[]` is an array without `readonly`."
            ]
        );
        assert_eq!(
            core("interface I extends Base { readonly x: number }\ninterface J { x: number }"),
            [
                "`I` should be deeply read-only, written in place: `Base` is a type defined elsewhere, not wrapped in `Readonly<…>`.",
                "`J` should be deeply read-only, written in place: `x: number` is a property without `readonly`.",
            ]
        );
        assert_eq!(
            core("type O = { readonly a: object; readonly b };"),
            [
                "`O` should be deeply read-only, written in place: `object` is `object` or `any`, which may hold anything mutable."
            ]
        );
    }

    #[test]
    fn refuses_a_type_defined_elsewhere_unless_wrapped() {
        // [F19.AC2] `Temporal.PlainDate` and a package's type are defined elsewhere.
        assert_eq!(
            core(
                "import type { Font } from \"fontkit\";\ntype D = { readonly at: Temporal.PlainDate; readonly font: Font };"
            ),
            [
                "`D` should be deeply read-only, written in place: `Temporal.PlainDate` is a type defined elsewhere, not wrapped in `Readonly<…>`."
            ]
        );
        assert_eq!(core("type K = (typeof import(\"./k\"))[number];").len(), 1);
        assert_eq!(
            core(
                "import type { Font } from \"fontkit\";\ntype D = { readonly at: Readonly<Temporal.PlainDate>; readonly font: Readonly<Font> };"
            ),
            NONE
        );
        assert_eq!(
            core(
                "import type { KINDS } from \"./kinds\";\ntype K = Readonly<(typeof KINDS)[number]>;"
            ),
            NONE
        );
    }

    #[test]
    fn accepts_what_is_read_only_where_it_is_written() {
        for code in [
            "type Id = string;",
            "type F = (a: number) => number;",
            "type K = \"a\" | \"b\";",
            "type P = Readonly<{ x: number; y: readonly number[] }>;",
            "import type { Point } from \"./point\";\ntype S = { readonly start: Point; readonly all: ReadonlySet<Point> };",
            "type C<T> = T extends string ? readonly T[] : never;",
            "const KINDS = [\"a\", \"b\"] as const;\ntype Kind = (typeof KINDS)[number];",
            "type T = { readonly [key: string]: number; m(): number };",
        ] {
            assert_eq!(core(code), NONE, "{code}");
        }
        // A constant without `as const` is not read-only where it is written.
        assert_eq!(
            core("const KINDS = [\"a\"];\ntype Kind = (typeof KINDS)[number];").len(),
            1
        );
    }

    #[test]
    fn writes_what_is_missing_in_a_type_alias_only() {
        let code = "type P = { x: Point[] };\ninterface I { x: number }";
        let findings = lint(&TypeDeclarationImmutability, "src/math/a.ts", code);

        assert_eq!(
            fixed(code, &findings),
            "type P = { readonly x: readonly Readonly<Point>[] };\ninterface I { x: number }"
        );
    }
}
