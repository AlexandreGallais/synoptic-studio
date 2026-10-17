//! `functional/functional-parameters`: a function takes at least one parameter
//! (`eslint-plugin-functional` 10.0.1, `docs/rules/functional-parameters.md`), with the project's
//! options: `enforceParameterCount: "atLeastOne"` written as a string, so no IIFE, callback nor
//! accessor is let through; a rest parameter allowed (`allowRestParameter`). The `arguments`
//! part (`allowArgumentsKeyword: false`) is oxlint's `prefer-rest-params`, on every file. A `this`
//! parameter counts, as it does in typescript-eslint's tree; an overload or `declare` signature
//! is a declared function, not one of the tree's functions, and is left. No fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{FormalParameters, FunctionType};
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct FunctionalParameters;

/// The plugin's message.
const MESSAGE: &str = "Functions must have at least one parameter.";

/// The number of parameters a function's tree lists: the named ones and the rest.
fn count(parameters: &FormalParameters) -> usize {
    parameters.items.len() + usize::from(parameters.rest.is_some())
}

impl Rule for FunctionalParameters {
    fn name(&self) -> &'static str {
        "functional/functional-parameters"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (start, parameters) = match node.kind() {
            AstKind::Function(function)
                if matches!(
                    function.r#type,
                    FunctionType::FunctionDeclaration | FunctionType::FunctionExpression
                ) =>
            {
                (
                    function.span.start,
                    count(&function.params) + usize::from(function.this_param.is_some()),
                )
            }
            AstKind::ArrowFunctionExpression(arrow) => (arrow.span.start, count(&arrow.params)),
            _ => return,
        };
        if parameters == 0 {
            findings.push(source.finding(self.name(), start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionalParameters;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&FunctionalParameters, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_function_without_parameter() {
        // The documentation's first case (its `arguments` is oxlint's), then each kind of
        // function: none is exempt with the count written as a string.
        assert_eq!(
            places(
                "function add() {\n  return arguments.reduce((sum, number) => sum + number, 0);\n}"
            ),
            [(1, 1)]
        );
        assert_eq!(places("const f = () => 1;"), [(1, 11)]);
        assert_eq!(
            places("const g = function () {\n  return 1;\n};"),
            [(1, 11)]
        );
        assert_eq!(places("(() => 1)();"), [(1, 2)]);
        assert_eq!(places("xs.map(() => 1);"), [(1, 8)]);
        assert_eq!(places("export async function h() {}"), [(1, 8)]);
    }

    #[test]
    fn lets_a_parameter_a_rest_parameter_and_a_signature_through() {
        // The documentation's correct case, its rest parameter (allowed here), a `this`
        // parameter, an overload and a function type.
        for code in [
            "function add(numbers) {\n  return numbers.reduce((sum, number) => sum + number, 0);\n}",
            "function add(...numbers) {\n  return numbers.reduce((sum, number) => sum + number, 0);\n}",
            "function f(this: Window) {}",
            "function o(): void;\nfunction o(a?: number): void {}",
            "declare function d(): void;",
            "type F = () => void;",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
