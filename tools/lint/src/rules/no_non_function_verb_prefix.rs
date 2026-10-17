//! `unicorn/no-non-function-verb-prefix`: a variable or a parameter named like an action —
//! `getX`, `setX`, `unsetX`, `deleteX`, `addX`, `removeX`, `destroyX`, `createX` — holds a function
//! (`REF-UNICORN-NON-FUNCTION-VERB-PREFIX`). The rule asked the compiler whether the value can be
//! called, and reported nothing without types; here the written annotation, or else the
//! initializer, tells it (`crate::written`): a value they leave unknown is not reported, as the
//! rule did not report an unknown type. An import's type is never read; classes and enums, which
//! other rules refuse, are left out.

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, Expression, TSTypeAnnotation};
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;
use crate::written::{Callability, expression_callability, type_callability};

/// The rule.
pub struct NoNonFunctionVerbPrefix;

/// The action verbs, the longest first (the rule's defaults).
const VERBS: [&str; 8] = [
    "destroy", "create", "delete", "remove", "unset", "add", "get", "set",
];

/// The verb a name starts with, followed by a capital letter.
fn verb_of(name: &str) -> Option<&'static str> {
    VERBS.into_iter().find(|verb| {
        name.strip_prefix(verb)
            .and_then(|rest| rest.chars().next())
            .is_some_and(|next| next.is_ascii_uppercase())
    })
}

/// The callability of a name from its written type, or else its initializer.
fn callability(
    annotation: Option<&TSTypeAnnotation>,
    initializer: Option<&Expression>,
) -> Callability {
    match (annotation, initializer) {
        (Some(annotation), _) => type_callability(&annotation.type_annotation),
        (None, Some(initializer)) => expression_callability(initializer),
        (None, None) => Callability::Unknown,
    }
}

/// Whether a node sits in a `declare` statement or a declared module.
fn is_declared(node: &AstNode, source: &Source) -> bool {
    source
        .semantic
        .nodes()
        .ancestors(node.id())
        .any(|ancestor| match ancestor.kind() {
            AstKind::VariableDeclaration(declaration) => declaration.declare,
            AstKind::TSExternalModuleDeclaration(module) => module.declare,
            AstKind::TSNamespaceDeclaration(namespace) => namespace.declare,
            AstKind::TSGlobalDeclaration(global) => global.declare,
            _ => false,
        })
}

impl Rule for NoNonFunctionVerbPrefix {
    fn name(&self) -> &'static str {
        "unicorn/no-non-function-verb-prefix"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (pattern, annotation, initializer) = match node.kind() {
            AstKind::VariableDeclarator(declarator) => (
                &declarator.id,
                declarator.type_annotation.as_deref(),
                declarator.init.as_ref(),
            ),
            AstKind::FormalParameter(parameter)
                if matches!(
                    source
                        .semantic
                        .nodes()
                        .ancestors(node.id())
                        .nth(1)
                        .map(AstNode::kind),
                    Some(
                        AstKind::ArrowFunctionExpression(_)
                            | AstKind::Function(oxc_ast::ast::Function { body: Some(_), .. })
                    )
                ) =>
            {
                (
                    &parameter.pattern,
                    parameter.type_annotation.as_deref(),
                    parameter.initializer.as_deref(),
                )
            }
            _ => return,
        };
        let BindingPattern::BindingIdentifier(identifier) = pattern else {
            return;
        };
        let Some(verb) = verb_of(&identifier.name) else {
            return;
        };
        if callability(annotation, initializer) == Callability::NotCallable
            && !is_declared(node, source)
        {
            findings.push(source.finding(
                self.name(),
                identifier.span.start,
                format!(
                    "`{}` starts with `{verb}`, so it should be a function.",
                    identifier.name
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoNonFunctionVerbPrefix;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&NoNonFunctionVerbPrefix, "src/math/a.ts", code))
    }

    #[test]
    fn reports_a_value_named_like_an_action() {
        assert_eq!(
            found(
                "const getWidth = 3;\nconst createdAt: number[] = [];\nconst removeAll = { a: 1 };"
            ),
            [
                "`getWidth` starts with `get`, so it should be a function.",
                "`removeAll` starts with `remove`, so it should be a function.",
            ]
        );
        assert_eq!(
            found("function f(setSize: number, addOne = \"a\", deleteX: string | null): void {}"),
            [
                "`setSize` starts with `set`, so it should be a function.",
                "`addOne` starts with `add`, so it should be a function.",
                "`deleteX` starts with `delete`, so it should be a function.",
            ]
        );
        // The longest verb wins: `destroy`, not `de…`; `unset`, not `un…`.
        assert_eq!(
            found("const destroyAll = 1 + 2;\nconst unsetFlag = !a;"),
            [
                "`destroyAll` starts with `destroy`, so it should be a function.",
                "`unsetFlag` starts with `unset`, so it should be a function.",
            ]
        );
    }

    #[test]
    fn accepts_functions_and_what_the_code_does_not_tell() {
        for code in [
            "const getWidth = (a: number) => a;",
            "const getWidth: (a: number) => number = make();",
            "const getWidth = make();",
            "const getWidth = a ? b : 1;",
            "const getWidth: Function = f;",
            "const getWidth = null;",
            "const getter = 1;\nconst get = 2;\nconst get_x = 3;",
            "const { getA } = o;",
            "declare const getX: number;",
            "declare module \"m\" { const getX: number; }",
            "type F = (getX: number) => void;",
            "declare function f(getX: number): void;",
            "const getWidth: { (): number } = f;",
            "const getWidth: (() => number) | undefined = f;",
            "const getWidth: Callback & { readonly a: number } = f;",
        ] {
            assert_eq!(found(code), NONE, "{code}");
        }
    }

    #[test]
    fn reads_unions_intersections_and_assertions() {
        assert_eq!(
            found(
                "const getA: number | string = 1;\nconst getB: { readonly a: number } & { readonly b: number } = o;\nconst getC = o as number;\nconst getD = a || 1;"
            ),
            [
                "`getA` starts with `get`, so it should be a function.",
                "`getB` starts with `get`, so it should be a function.",
                "`getC` starts with `get`, so it should be a function.",
            ]
        );
    }
}
