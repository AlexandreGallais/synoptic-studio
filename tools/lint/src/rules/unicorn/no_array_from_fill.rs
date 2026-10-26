//! `unicorn/no-array-from-fill`: no `.fill()` after `Array.from({length: …})`, whose mapping
//! argument makes each value (`eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKind};
use oxc_semantic::AstNode;

use super::syntax::{is_global, method_call, method_named};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoArrayFromFill;

/// Its message.
const MESSAGE: &str =
    "Use the `Array.from(…, mapFunction)` argument instead of chaining `.fill()`.";

/// Whether an expression is `Array.from({length: …})`, its object holding `length` only.
fn is_array_from_length(source: &Source, expression: &Expression) -> bool {
    let Some(from) = method_named(expression, &["from"]) else {
        return false;
    };
    let Some([Expression::ObjectExpression(object)]) = from.arguments().as_deref() else {
        return false;
    };
    let length_only = match object.properties.as_slice() {
        [ObjectPropertyKind::ObjectProperty(property)] => {
            !property.computed
                && !property.method
                && property.kind == PropertyKind::Init
                && property.key.is_specific_static_name("length")
        }
        _ => false,
    };
    from.is_plain() && is_global(source, from.object, "Array") && length_only
}

impl Rule for NoArrayFromFill {
    fn name(&self) -> &'static str {
        "unicorn/no-array-from-fill"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(fill) = method_call(call).filter(|method| method.name == "fill") else {
            return;
        };
        let reported = fill.is_plain()
            && fill
                .arguments()
                .is_some_and(|arguments| arguments.len() <= 1)
            && is_array_from_length(source, fill.object);
        if reported {
            findings.push(source.finding(self.name(), fill.property.start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoArrayFromFill;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoArrayFromFill, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_fill_after_array_from() {
        let message =
            "Use the `Array.from(…, mapFunction)` argument instead of chaining `.fill()`.";

        for code in [
            "Array.from({length: 3}).fill().map((_, index) => index);",
            "Array.from({length: 3}).fill({});",
            "Array.from({'length': n}).fill(0);",
            "Array.from({length}).fill(0);",
        ] {
            assert_eq!(check(code), [message], "{code}");
        }
        // The finding sits on `fill`: `Array.from({length: 3}).` is 24 characters.
        let findings = lint(
            &NoArrayFromFill,
            "src/a.ts",
            "Array.from({length: 3}).fill({});",
        );
        assert_eq!(findings[0].column, 25);
    }

    #[test]
    fn accepts_the_mapping_argument_and_other_shapes() {
        for code in [
            "Array.from({length: 3}, (_, index) => index);",
            "Array.from({length: 3}, () => ({}));",
            "Array.from({length: 3, other: 1}).fill(0);",
            "Array.from({[length]: 3}).fill(0);",
            "Array.from({length: 3}).fill(0, 1);",
            "Array.from({length: 3})?.fill(0);",
            "Array.from(items).fill(0);",
            "const Array = { from: (x: unknown) => [x] }; Array.from({length: 3}).fill(0);",
            "Array.of({length: 3}).fill(0);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
