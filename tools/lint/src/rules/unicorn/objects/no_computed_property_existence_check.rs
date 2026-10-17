//! `unicorn/no-computed-property-existence-check`: no `object[key]` read for its truth and no
//! `key in object` with a dynamic key, which find inherited properties too
//! (`eslint-plugin-unicorn` 78.0.0); `Object.hasOwn()` checks an own property. A `Map`, a `Set`,
//! their weak kinds or a string is left alone. Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression};
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::known_values::{is_static_key, is_string};
use crate::rules::unicorn::syntax::{Kind, is_boolean_context, kind_of, outer_parent};
use crate::source::Source;

/// The rule.
pub struct NoComputedPropertyExistenceCheck;

/// What the rule reports.
const MESSAGE: &str = "Do not use a dynamic object property existence check.";

/// Whether a receiver is known to be a collection or a string, for which `Object.hasOwn()`
/// replaces nothing.
fn is_known_non_object(source: &Source, expression: &Expression) -> bool {
    matches!(
        kind_of(source, expression),
        Some(Kind::Map | Kind::Set | Kind::WeakMap | Kind::WeakSet)
    ) || is_string(source, expression)
}

impl Rule for NoComputedPropertyExistenceCheck {
    fn name(&self) -> &'static str {
        "unicorn/no-computed-property-existence-check"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let reported = match node.kind() {
            AstKind::ComputedMemberExpression(member) => {
                let (_, outermost) = outer_parent(source, node.id());
                (!is_static_key(&member.expression)
                    && is_boolean_context(source, outermost)
                    && !is_known_non_object(source, &member.object))
                .then_some(member.span.start)
            }
            AstKind::BinaryExpression(binary) => (binary.operator == BinaryOperator::In
                && !is_static_key(&binary.left)
                && !is_known_non_object(source, &binary.right))
            .then_some(binary.span.start),
            _ => None,
        };
        if let Some(offset) = reported {
            findings.push(source.finding(self.name(), offset, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MESSAGE, NoComputedPropertyExistenceCheck};
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &NoComputedPropertyExistenceCheck,
            "playground/a.ts",
            code,
        ))
    }

    #[test]
    fn reports_the_documented_dynamic_checks() {
        assert_eq!(check("if (object[key]) {}"), [MESSAGE]);
        assert_eq!(check("if (key in object) {}"), [MESSAGE]);
        assert_eq!(check("const a = !object[key];"), [MESSAGE]);
        assert_eq!(check("const a = Boolean(object?.[key]);"), [MESSAGE]);
        assert_eq!(check("while (ok && (object[key] as T)) {}"), [MESSAGE]);
        assert_eq!(check("const a = key in object;"), [MESSAGE]);
    }

    #[test]
    fn accepts_values_static_keys_and_known_receivers() {
        for code in [
            "if (object[key] === true) {}",
            "if (Object.hasOwn(object, key)) {}",
            "const value = object[key];",
            "if ('key' in object) {}",
            "if (object['key']) {}",
            "if (object[0]) {}",
            "if (object[-1]) {}",
            "if (object[`key`]) {}",
            "const map = new Map(); if (map[key]) {}",
            "const set: Set<string> = new Set(); if (key in set) {}",
            "if ('text'[index]) {}",
            "function f(text: string) { if (text[index]) {} }",
            "const a = object[key] ?? fallback;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
