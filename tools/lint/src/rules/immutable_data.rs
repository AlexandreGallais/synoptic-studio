//! `functional/immutable-data`: in the functional core, no existing object or array is changed
//! (`REF-FUNCTIONAL-PLUGIN`; the project's options: classes, immediate mutations and `let`
//! declarations not ignored). The plugin asked the compiler whether a method's object is an
//! array, a map or a set; without types, every assignment to a member, every `delete` of one,
//! every `++` or `--` on one, the mutating methods of arrays, maps and sets called by name on any
//! object, and `Object.assign`, `defineProperty`, `defineProperties` and `setPrototypeOf` on an
//! existing object are reported.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, AssignmentTarget, Expression, SimpleAssignmentTarget, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ImmutableData;

/// The message on a member changed.
const GENERIC: &str = "Modifying an existing object/array is not allowed.";
/// The message on an array's mutating method.
const ARRAY: &str = "Modifying an array is not allowed.";
/// The message on a map's or a set's mutating method.
const COLLECTION: &str = "Modifying a map or a set is not allowed.";
/// The message on `Object`'s mutating functions.
const OBJECT: &str = "Modifying properties of existing object not allowed.";

/// The methods that change an array (the plugin's `arrayMutatorMethods`).
const ARRAY_MUTATORS: [&str; 9] = [
    "copyWithin",
    "fill",
    "pop",
    "push",
    "reverse",
    "shift",
    "sort",
    "splice",
    "unshift",
];

/// The methods that change a map or a set (the plugin's `mapMutatorMethods` and
/// `setMutatorMethods`).
const COLLECTION_MUTATORS: [&str; 4] = ["add", "clear", "delete", "set"];

/// The functions of `Object` that change their first argument.
const OBJECT_MUTATORS: [&str; 4] = [
    "assign",
    "defineProperties",
    "defineProperty",
    "setPrototypeOf",
];

/// Whether an expression is a member, its parentheses left out.
fn is_member(expression: &Expression) -> bool {
    match expression {
        Expression::ParenthesizedExpression(parenthesized) => is_member(&parenthesized.expression),
        other => other.is_member_expression(),
    }
}

/// The message on a call that changes its object, by the method's name.
fn call_message(call: &oxc_ast::ast::CallExpression) -> Option<&'static str> {
    let Expression::StaticMemberExpression(member) = call.callee.without_parentheses() else {
        return None;
    };
    let method = member.property.name.as_str();
    if ARRAY_MUTATORS.contains(&method) {
        return Some(ARRAY);
    }
    if COLLECTION_MUTATORS.contains(&method) {
        return Some(COLLECTION);
    }
    let on_object =
        matches!(&member.object, Expression::Identifier(object) if object.name == "Object");
    let changes_existing = matches!(
        call.arguments.first(),
        Some(
            Argument::Identifier(_)
                | Argument::StaticMemberExpression(_)
                | Argument::ComputedMemberExpression(_)
        )
    );
    (on_object
        && OBJECT_MUTATORS.contains(&method)
        && call.arguments.len() >= 2
        && changes_existing)
        .then_some(OBJECT)
}

impl Rule for ImmutableData {
    fn name(&self) -> &'static str {
        "functional/immutable-data"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let message = match node.kind() {
            AstKind::AssignmentExpression(assignment) => matches!(
                assignment.left,
                AssignmentTarget::ComputedMemberExpression(_)
                    | AssignmentTarget::StaticMemberExpression(_)
                    | AssignmentTarget::PrivateFieldExpression(_)
            )
            .then_some(GENERIC),
            AstKind::UpdateExpression(update) => matches!(
                update.argument,
                SimpleAssignmentTarget::ComputedMemberExpression(_)
                    | SimpleAssignmentTarget::StaticMemberExpression(_)
                    | SimpleAssignmentTarget::PrivateFieldExpression(_)
            )
            .then_some(GENERIC),
            AstKind::UnaryExpression(unary) => (unary.operator == UnaryOperator::Delete
                && is_member(&unary.argument))
            .then_some(GENERIC),
            AstKind::CallExpression(call) => call_message(call),
            _ => None,
        };
        if let Some(message) = message {
            findings.push(source.finding(self.name(), node.span().start, message.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ImmutableData;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of the functional core.
    fn core(code: &str) -> Vec<String> {
        messages(&lint(&ImmutableData, "src/math/a.ts", code))
    }

    #[test]
    fn reports_a_member_assigned_deleted_or_counted() {
        // [F19.AC2] Any assignment to a member and any `delete`, whatever the object's type.
        assert_eq!(
            core("const o = { a: 1 };\no.a = 2;\no[\"b\"] += 1;\ndelete o.a;\no.a++;\n--o[0];"),
            [
                "Modifying an existing object/array is not allowed.",
                "Modifying an existing object/array is not allowed.",
                "Modifying an existing object/array is not allowed.",
                "Modifying an existing object/array is not allowed.",
                "Modifying an existing object/array is not allowed.",
            ]
        );
    }

    #[test]
    fn reports_the_mutating_methods_by_name() {
        // [F19.AC2] The array methods of the plugin, on any object, even a new array.
        assert_eq!(
            core(
                "a.push(1);\n[3, 1].sort();\nb.c.splice(0, 1);\nd?.reverse();\nm.set(\"k\", 1);\ns.add(1);\nm.delete(\"k\");"
            ),
            [
                "Modifying an array is not allowed.",
                "Modifying an array is not allowed.",
                "Modifying an array is not allowed.",
                "Modifying an array is not allowed.",
                "Modifying a map or a set is not allowed.",
                "Modifying a map or a set is not allowed.",
                "Modifying a map or a set is not allowed.",
            ]
        );
        assert_eq!(
            core("Object.assign(target, { a: 1 });\nObject.defineProperty(o.p, \"a\", {});"),
            [
                "Modifying properties of existing object not allowed.",
                "Modifying properties of existing object not allowed.",
            ]
        );
    }

    #[test]
    fn accepts_new_values_and_local_variables() {
        for code in [
            "const a = [1].toSorted();",
            "const b = Object.assign({}, a);",
            "const c = Object.assign(a);",
            "const d = Object.freeze(a);",
            "const e = [...a, 1];",
            "let f = 1; f = 2; f++;",
            "const g = a.concat(b).slice(1);",
            "const h = push(a);",
            "delete x;",
            "const k = o[\"push\"](1);",
        ] {
            assert_eq!(core(code), NONE, "{code}");
        }
    }
}
