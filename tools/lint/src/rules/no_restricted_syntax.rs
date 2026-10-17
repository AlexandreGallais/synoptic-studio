//! `no-restricted-syntax`: the syntax outside the project's style, as the seven selectors of
//! `ESLint`'s rule named it (`REF-ESLINT-RESTRICTED-SYNTAX`) — no `enum`, no `for…in`, no getter or
//! setter, no namespace re-export, no dynamic import, and in the library no default nor optional
//! parameter (ADR-0016). The eighth selector, a spread in a fold, is oxlint's
//! `oxc/no-accumulating-spread` now.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, FormalParameter, FunctionType, MethodDefinitionKind, PropertyKind,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::glob::matches_any;
use crate::rules::Rule;
use crate::scopes::LIBRARY_FILES;
use crate::source::Source;

/// The rule.
pub struct NoRestrictedSyntax;

/// The message on an `enum`.
const ENUM: &str = "Use a union of string literals instead of an enum.";
/// The message on a `for…in`.
const FOR_IN: &str = "Iterate over Object.keys/values/entries instead.";
/// The message on a getter or a setter.
const ACCESSOR: &str = "No accessors: data is plain and readonly.";
/// The message on `export * as name from`.
const NAMESPACE_EXPORT: &str = "Re-export without namespace: `export * from`.";
/// The message on `import()`.
const DYNAMIC_IMPORT: &str = "The dependency graph stays static: no dynamic import.";
/// The message on a default parameter.
const DEFAULT_PARAMETER: &str =
    "No default parameter: every input is explicit; defaults live in the model (ADR-0016).";
/// The message on an optional parameter.
const OPTIONAL_PARAMETER: &str = "No optional parameter: every input is explicit (ADR-0016).";

/// Whether a node is what the selectors call `:function`: a function declaration or expression,
/// or an arrow — not a declared signature, a body-less method nor a function type.
fn is_function(node: &AstNode) -> bool {
    match node.kind() {
        AstKind::Function(function) => matches!(
            function.r#type,
            FunctionType::FunctionDeclaration | FunctionType::FunctionExpression
        ),
        AstKind::ArrowFunctionExpression(_) => true,
        _ => false,
    }
}

/// The parameter of a function a node sits in, with the parameter's owner checked.
fn parameter_of<'a>(
    node: &AstNode<'a>,
    source: &Source<'_, 'a>,
) -> Option<&'a FormalParameter<'a>> {
    let mut ancestors = source.semantic.nodes().ancestors(node.id());
    let parameter = ancestors.find_map(|ancestor| match ancestor.kind() {
        AstKind::FormalParameter(parameter) => Some(parameter),
        _ => None,
    })?;
    // Past the parameter come its list, then its owner.
    ancestors
        .nth(1)
        .filter(|owner| is_function(owner))
        .map(|_| parameter)
}

/// The library's finding on a parameter: a default, a default inside its destructured object,
/// an optional name.
fn parameter_finding(node: &AstNode, source: &Source) -> Option<&'static str> {
    match node.kind() {
        AstKind::FormalParameter(parameter) => {
            let owner = source.semantic.nodes().ancestors(node.id()).nth(1)?;
            if !is_function(owner) {
                None
            } else if parameter.initializer.is_some() {
                Some(DEFAULT_PARAMETER)
            } else if parameter.optional {
                Some(OPTIONAL_PARAMETER)
            } else {
                None
            }
        }
        AstKind::AssignmentPattern(_) => {
            let parameter = parameter_of(node, source)?;
            let in_object = parameter.initializer.is_none()
                && matches!(parameter.pattern, BindingPattern::ObjectPattern(_));
            in_object.then_some(DEFAULT_PARAMETER)
        }
        _ => None,
    }
}

impl Rule for NoRestrictedSyntax {
    fn name(&self) -> &'static str {
        "no-restricted-syntax"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let message = match node.kind() {
            AstKind::TSEnumDeclaration(_) => Some(ENUM),
            AstKind::ForInStatement(_) => Some(FOR_IN),
            AstKind::MethodDefinition(method)
                if matches!(
                    method.kind,
                    MethodDefinitionKind::Get | MethodDefinitionKind::Set
                ) =>
            {
                Some(ACCESSOR)
            }
            AstKind::ObjectProperty(property)
                if matches!(property.kind, PropertyKind::Get | PropertyKind::Set) =>
            {
                Some(ACCESSOR)
            }
            AstKind::ExportAllDeclaration(export) if export.exported.is_some() => {
                Some(NAMESPACE_EXPORT)
            }
            AstKind::ImportExpression(_) => Some(DYNAMIC_IMPORT),
            AstKind::FormalParameter(_) | AstKind::AssignmentPattern(_)
                if matches_any(LIBRARY_FILES, source.path) =>
            {
                parameter_finding(node, source)
            }
            _ => None,
        };
        if let Some(message) = message {
            findings.push(source.finding(self.name(), node.span().start, message.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoRestrictedSyntax;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code of the library.
    fn library(code: &str) -> Vec<String> {
        messages(&lint(&NoRestrictedSyntax, "src/math/a.ts", code))
    }

    /// The messages on a piece of code of the tools.
    fn tools(code: &str) -> Vec<String> {
        messages(&lint(&NoRestrictedSyntax, "scripts/a.ts", code))
    }

    #[test]
    fn reports_at_the_node_it_refuses() {
        // A `lint-disable-next-line` directive is matched by line: the place must be the node's.
        let places: Vec<(usize, usize)> = lint(
            &NoRestrictedSyntax,
            "scripts/a.ts",
            "const x = 1;\n  enum Color { Red }",
        )
        .iter()
        .map(|finding| (finding.line, finding.column))
        .collect();

        assert_eq!(places, [(2, 3)]);
    }

    #[test]
    fn refuses_enums_for_in_and_accessors_everywhere() {
        // [F19.AC3] Each selector of `no-restricted-syntax`, reported with its message.
        assert_eq!(
            tools("enum Color { Red }"),
            ["Use a union of string literals instead of an enum."]
        );
        assert_eq!(
            tools("for (const k in {}) {}"),
            ["Iterate over Object.keys/values/entries instead."]
        );
        assert_eq!(
            tools("const o = { get a() { return 1; }, set a(v) {} };"),
            [
                "No accessors: data is plain and readonly.",
                "No accessors: data is plain and readonly."
            ]
        );
        assert_eq!(
            tools("class C { get a(): number { return 1; } static set b(v: number) {} }"),
            [
                "No accessors: data is plain and readonly.",
                "No accessors: data is plain and readonly."
            ]
        );
    }

    #[test]
    fn refuses_namespace_reexports_and_dynamic_imports_everywhere() {
        assert_eq!(
            tools("export * as a from \"./a\";"),
            ["Re-export without namespace: `export * from`."]
        );
        assert_eq!(
            tools("const a = await import(\"./a\");"),
            ["The dependency graph stays static: no dynamic import."]
        );
    }

    #[test]
    fn accepts_plain_data_static_imports_and_star_reexports() {
        let code = "import a from \"./a\";\nexport * from \"./b\";\nconst o = { a, b() { return 1; }, c: 2 };\nfor (const k of []) {}\ntype T = { get x(): number };";
        assert_eq!(tools(code), NONE);
        assert_eq!(library(code), NONE);
    }

    #[test]
    fn refuses_default_and_optional_parameters_in_the_library_only() {
        let code = "function f(a = 1, b?: number): void {}";

        assert_eq!(
            library(code),
            [
                "No default parameter: every input is explicit; defaults live in the model (ADR-0016).",
                "No optional parameter: every input is explicit (ADR-0016).",
            ]
        );
        assert_eq!(tools(code), NONE);
    }

    #[test]
    fn refuses_defaults_destructured_from_an_object_parameter() {
        assert_eq!(
            library("const f = ({ a = 1, b: { c = 2 } }: T): number => a;"),
            [
                "No default parameter: every input is explicit; defaults live in the model (ADR-0016).",
                "No default parameter: every input is explicit; defaults live in the model (ADR-0016).",
            ]
        );
        // The whole object defaulted: one finding, on the parameter.
        assert_eq!(
            library("const f = ({ a = 1 } = {}): number => a;"),
            [
                "No default parameter: every input is explicit; defaults live in the model (ADR-0016)."
            ]
        );
    }

    #[test]
    fn leaves_what_the_selectors_did_not_match() {
        // An array pattern's default, a declared signature, a function type, a variable's default.
        for code in [
            "function f([a = 1]: number[]): number { return a; }",
            "declare function f(a?: number): void;",
            "type F = (a?: number) => void;",
            "const { a = 1 } = { a: 2 };",
            "function f({ a }: { a?: number }): void {}",
        ] {
            assert_eq!(library(code), NONE, "{code}");
        }
    }
}
