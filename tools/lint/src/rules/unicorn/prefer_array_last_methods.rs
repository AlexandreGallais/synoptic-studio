//! `unicorn/prefer-array-last-methods`: no `.reverse()` nor `.toReversed()` followed by `find()`,
//! `findIndex()`, `indexOf()` or `reduce()`, which `findLast()`, `findLastIndex()`,
//! `lastIndexOf()` and `reduceRight()` say directly (`eslint-plugin-unicorn` 78.0.0). Its fix is
//! a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::syntax::{method_call, method_named};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferArrayLastMethods;

/// Each forward method and its last-oriented one.
const REPLACEMENTS: [(&str, &str); 4] = [
    ("find", "findLast"),
    ("findIndex", "findLastIndex"),
    ("indexOf", "lastIndexOf"),
    ("reduce", "reduceRight"),
];

impl Rule for PreferArrayLastMethods {
    fn name(&self) -> &'static str {
        "unicorn/prefer-array-last-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call).filter(super::syntax::MethodCall::is_plain) else {
            return;
        };
        let Some((_, replacement)) = REPLACEMENTS.iter().find(|(name, _)| *name == method.name)
        else {
            return;
        };
        let Some(reversing) = method_named(method.object, &["reverse", "toReversed"])
            .filter(|reversing| reversing.is_plain() && reversing.call.arguments.is_empty())
        else {
            return;
        };
        findings.push(source.finding(
            self.name(),
            method.property.start,
            format!(
                "Prefer `Array#{replacement}()` over `Array#{}().{}()`.",
                reversing.name, method.name
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferArrayLastMethods;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferArrayLastMethods, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_reversed_searches_and_folds() {
        assert_eq!(
            check("const result = array.reverse().find(isUnicorn);"),
            ["Prefer `Array#findLast()` over `Array#reverse().find()`."]
        );
        assert_eq!(
            check("const result = array.toReversed().reduce(reducer, initialValue);"),
            ["Prefer `Array#reduceRight()` over `Array#toReversed().reduce()`."]
        );
        assert_eq!(
            check("const i = array.toReversed().findIndex(isUnicorn);"),
            ["Prefer `Array#findLastIndex()` over `Array#toReversed().findIndex()`."]
        );
        assert_eq!(
            check("const i = array.toReversed().indexOf(1);"),
            ["Prefer `Array#lastIndexOf()` over `Array#toReversed().indexOf()`."]
        );
    }

    #[test]
    fn accepts_the_last_methods_and_other_chains() {
        for code in [
            "const result = array.findLast(isUnicorn);",
            "const result = array.reduceRight(reducer, initialValue);",
            "const result = array.toReversed().map(f);",
            "const result = array.toSorted().find(f);",
            "const result = array.reverse(x).find(f);",
            "const result = array?.reverse().find(f);",
            "const result = array.reverse()?.find(f);",
            "const result = array.reverse?.().find(f);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
