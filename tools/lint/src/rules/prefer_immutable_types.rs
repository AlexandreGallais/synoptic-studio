//! `functional/prefer-immutable-types`: in the functional core, each parameter with a written
//! type is deeply read-only (`REF-FUNCTIONAL-PLUGIN`; the project's `parameters` enforcement,
//! `ReadonlyDeep`, inferred types ignored). The plugin asked the compiler for the type's
//! immutability; this rule reads the written type in its strict form (`crate::deep_readonly`):
//! `readonly` or `Readonly<…>` written in place, a type defined elsewhere wrapped in `Readonly<…>`.
//! A type guard's checked parameter is left out, as the plugin left it. The fix writes what is
//! missing.

use oxc_ast::AstKind;
use oxc_ast::ast::{FormalParameters, TSType, TSTypeAnnotation, TSTypePredicateName};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::deep_readonly::{fix, mutable_parts, written_text};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferImmutableTypes;

/// The name a return type's predicate checks, `x` in `x is Point`.
fn guarded_name<'a>(return_type: Option<&TSTypeAnnotation<'a>>) -> Option<&'a str> {
    match &return_type?.type_annotation {
        TSType::TSTypePredicate(predicate) => match &predicate.parameter_name {
            TSTypePredicateName::Identifier(name) => Some(name.name.as_str()),
            TSTypePredicateName::This(_) => None,
        },
        _ => None,
    }
}

/// The parameters of a function or a signature, and its return type.
fn signature<'a>(
    node: &AstNode<'a>,
) -> Option<(&'a FormalParameters<'a>, Option<&'a TSTypeAnnotation<'a>>)> {
    match node.kind() {
        AstKind::Function(function) => Some((&function.params, function.return_type.as_deref())),
        AstKind::ArrowFunctionExpression(arrow) => {
            Some((&arrow.params, arrow.return_type.as_deref()))
        }
        AstKind::TSFunctionType(function) => Some((&function.params, Some(&function.return_type))),
        AstKind::TSMethodSignature(method) => Some((&method.params, method.return_type.as_deref())),
        AstKind::TSCallSignatureDeclaration(call) => {
            Some((&call.params, call.return_type.as_deref()))
        }
        AstKind::TSConstructSignatureDeclaration(construct) => {
            Some((&construct.params, construct.return_type.as_deref()))
        }
        _ => None,
    }
}

impl Rule for PreferImmutableTypes {
    fn name(&self) -> &'static str {
        "functional/prefer-immutable-types"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let Some((parameters, return_type)) = signature(node) else {
            return;
        };
        let guarded = guarded_name(return_type);
        let items = parameters
            .items
            .iter()
            .filter(|parameter| {
                guarded.is_none_or(|name| {
                    parameter
                        .pattern
                        .get_identifier_name()
                        .is_none_or(|own| own.as_str() != name)
                })
            })
            .map(|parameter| (parameter.span.start, parameter.type_annotation.as_deref()));
        let rest = parameters
            .rest
            .iter()
            .map(|rest| (rest.span.start, rest.type_annotation.as_deref()));

        for (at, annotation) in items.chain(rest) {
            let Some(written) = annotation.map(|annotation| &annotation.type_annotation) else {
                continue;
            };
            let parts = mutable_parts(written, source);
            let Some(first) = parts.first() else {
                continue;
            };
            let message = format!(
                "Parameter should be deeply read-only, written in place: `{}` is {}.",
                written_text(first, source.text),
                first.reason
            );
            findings.push(match fix(written.span(), &parts, source.text) {
                Some(fixed) => source.fixed(self.name(), at, message, fixed),
                None => source.finding(self.name(), at, message),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferImmutableTypes;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of the functional core.
    fn core(code: &str) -> Vec<String> {
        messages(&lint(&PreferImmutableTypes, "src/math/a.ts", code))
    }

    /// A piece of the functional core with the rule's fixes applied.
    fn fixed_core(code: &str) -> String {
        fixed(code, &lint(&PreferImmutableTypes, "src/math/a.ts", code))
    }

    #[test]
    fn refuses_a_mutable_array_tuple_or_property_written_in_place() {
        assert_eq!(
            core("function f(a: number[]): number { return 1; }"),
            [
                "Parameter should be deeply read-only, written in place: `number[]` is an array without `readonly`."
            ]
        );
        assert_eq!(
            core("const f = (a: [number, number]): number => 1;"),
            [
                "Parameter should be deeply read-only, written in place: `[number, number]` is a tuple without `readonly`."
            ]
        );
        assert_eq!(
            core("type F = (a: { x: number }) => number;"),
            [
                "Parameter should be deeply read-only, written in place: `x: number` is a property without `readonly`."
            ]
        );
        // Deep: the outer array is read-only, its rows are not.
        assert_eq!(
            core("function f(a: readonly number[][]): number { return 1; }"),
            [
                "Parameter should be deeply read-only, written in place: `number[]` is an array without `readonly`."
            ]
        );
    }

    #[test]
    fn refuses_a_type_defined_elsewhere_unless_wrapped() {
        // [F19.AC2] A type of the standard library or of a package is never trusted by its name.
        let code = "import type { Glyph } from \"fontkit\";\nfunction f(a: DataView, b: Glyph): number { return 1; }";
        assert_eq!(
            core(code),
            [
                "Parameter should be deeply read-only, written in place: `DataView` is a type defined elsewhere, not wrapped in `Readonly<…>`.",
                "Parameter should be deeply read-only, written in place: `Glyph` is a type defined elsewhere, not wrapped in `Readonly<…>`.",
            ]
        );
        assert_eq!(
            core(
                "import type { Glyph } from \"fontkit\";\nfunction f(a: Readonly<DataView>, b: Readonly<Glyph>): number { return 1; }"
            ),
            NONE
        );
    }

    #[test]
    fn accepts_a_type_proven_where_it_is_declared() {
        for code in [
            // Declared in the file, checked there by `type-declaration-immutability`.
            "type P = { readonly x: number };\nfunction f(a: P): number { return a.x; }",
            // Imported from the core by a relative path.
            "import type { Point } from \"../geometry\";\nfunction f(a: Point, b: readonly Point[]): number { return 1; }",
            // Utility types that keep `readonly`, a type parameter, the read-only generics.
            "import type { Point } from \"./point\";\nfunction f<T>(a: Pick<Point, \"x\">, b: T, c: ReadonlyMap<string, readonly T[]>): number { return 1; }",
            // Primitives, literals, functions, `readonly` and `Readonly<…>` in place.
            "function f(a: number, b: \"x\" | 1, c: (x: number) => number, d: Readonly<{ x: readonly string[] }>, e: readonly [x: number, ...rest: number[]]): number { return 1; }",
            "const f = function (a: { readonly [key: string]: number }, b: { readonly [K in \"a\"]: number }): number { return 1; };",
        ] {
            assert_eq!(core(code), NONE, "{code}");
        }
    }

    #[test]
    fn leaves_inferred_parameters_and_a_type_guard_s_checked_one() {
        for code in [
            "const f = [1].map((a) => a);",
            "function isPoint(a: object): a is object { return true; }",
        ] {
            assert_eq!(core(code), NONE, "{code}");
        }
        // The guard leaves out only the parameter it checks.
        assert_eq!(
            core("function g(a: object, b: number[]): a is object { return true; }").len(),
            1
        );
    }

    #[test]
    fn checks_signatures_and_the_rest_parameter() {
        let code = "type T = { m(a: number[]): void; (a: number[]): void; new (a: number[]): T };\ndeclare function f(...a: number[]): void;";
        assert_eq!(core(code).len(), 4);
    }

    #[test]
    fn writes_what_is_missing() {
        assert_eq!(
            fixed_core("function f(a: Point[][], b: { x: DataView }): void {}"),
            "function f(a: readonly (readonly Readonly<Point>[])[], b: { readonly x: Readonly<DataView> }): void {}"
        );
        assert_eq!(
            fixed_core(
                "function f(a: { [k: string]: number }, b: { [K in \"a\"]: number }, c: [number]): void {}"
            ),
            "function f(a: { readonly [k: string]: number }, b: { readonly [K in \"a\"]: number }, c: readonly [number]): void {}"
        );
        // `any` and `object` have no fix: the finding stays.
        let code = "function f(a: object[]): void {}";
        assert_eq!(fixed_core(code), code);
    }
}
