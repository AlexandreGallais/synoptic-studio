//! `unicorn/no-confusing-array-splice`: no `splice()` nor `toSpliced()` that replaces one element,
//! which an assignment or `with()` says plainly, nor one that inserts at `-1`
//! (`eslint-plugin-unicorn` 78.0.0). Its fixes are suggestions in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::syntax::{method_call, skips_non_array_receiver, static_number};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoConfusingArraySplice;

/// The message on a call, when it is confusing: one element replaced, or an insertion at `-1`.
fn message(start: Option<f64>, delete_count: Option<f64>, method: &str) -> Option<String> {
    let count = delete_count.map(|value| value.trunc().max(0.0));
    if count == Some(1.0) {
        return Some(format!(
            "Prefer a direct element replacement instead of `{method}()`."
        ));
    }
    let inserts_at_minus_one = start.map(f64::trunc) == Some(-1.0) && count == Some(0.0);
    inserts_at_minus_one.then(|| format!("Avoid using `{method}()` to insert at `-1`."))
}

impl Rule for NoConfusingArraySplice {
    fn name(&self) -> &'static str {
        "unicorn/no-confusing-array-splice"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call)
            .filter(|method| matches!(method.name, "splice" | "toSpliced") && method.is_plain())
        else {
            return;
        };
        let Some(&[start, delete_count, _]) = method.arguments().as_deref() else {
            return;
        };
        let Some(message) = message(
            static_number(start),
            static_number(delete_count),
            method.name,
        ) else {
            return;
        };
        if !skips_non_array_receiver(source, method.object) {
            findings.push(source.finding(self.name(), method.property.start, message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoConfusingArraySplice;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoConfusingArraySplice, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_replacements_and_insertions() {
        assert_eq!(
            check("array.splice(index, 1, element);"),
            ["Prefer a direct element replacement instead of `splice()`."]
        );
        assert_eq!(
            check("const result = array.toSpliced(index, 1, element);"),
            ["Prefer a direct element replacement instead of `toSpliced()`."]
        );
        assert_eq!(
            check("array.splice(-1, 0, element);"),
            ["Avoid using `splice()` to insert at `-1`."]
        );
        assert_eq!(
            check("const result = array.toSpliced(-1, 0, element);"),
            ["Avoid using `toSpliced()` to insert at `-1`."]
        );
        // A count truncated to 1, a start truncated to -1, a negative count read as 0.
        assert_eq!(check("array.splice(i, 1.5 as number, e);").len(), 1);
        assert_eq!(check("array.splice(-1.5, -2, e);").len(), 1);
    }

    #[test]
    fn accepts_the_documented_rewrites_and_other_counts() {
        for code in [
            "array[index] = element;",
            "const result = array.with(index, element);",
            "const insertionIndex = Math.max(array.length - 1, 0); array.splice(insertionIndex, 0, element);",
            "array.splice(index, 2, element);",
            "array.splice(index, count, element);",
            "array.splice(index, 1);",
            "array.splice(index, 1, a, b);",
            "array.splice(-2, 0, element);",
            "array?.splice(index, 1, element);",
            "array.splice?.(index, 1, element);",
            "array.splice(...args, 1, element);",
            "const set = new Set(); set.splice(index, 1, element);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
