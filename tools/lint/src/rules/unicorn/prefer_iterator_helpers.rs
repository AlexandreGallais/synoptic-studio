//! `unicorn/prefer-iterator-helpers`: no array built from an iterator only to call `every()`,
//! `find()`, `forEach()`, `some()` or `reduce()` on it, nor to slice it by fixed bounds — the
//! iterator's own helpers do it lazily (`eslint-plugin-unicorn` 78.0.0). Its fixes are
//! suggestions in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{ArrayExpressionElement, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{
    FunctionLike, Value, function_like, function_named, has_optional_chain, is_iterator,
    method_call, method_named, static_value, unwrap_chain,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferIteratorHelpers;

/// The iterator a temporary array is built from: `[...iterator]` or `Array.from(iterator)`.
fn spread_or_from<'b, 'a>(
    source: &Source,
    expression: &'b Expression<'a>,
) -> Option<&'b Expression<'a>> {
    match expression {
        Expression::ArrayExpression(array) => match array.elements.as_slice() {
            [ArrayExpressionElement::SpreadElement(spread)]
                if is_iterator(source, &spread.argument) =>
            {
                Some(&spread.argument)
            }
            _ => None,
        },
        Expression::CallExpression(call) if call.type_arguments.is_none() => {
            let from = method_call(call).filter(|method| {
                method.name == "from"
                    && method.is_plain()
                    && matches!(method.object, Expression::Identifier(object) if object.name == "Array")
            })?;
            match from.arguments().as_deref() {
                Some(&[iterator]) if is_iterator(source, iterator) => Some(iterator),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Whether a callback may read the array argument at an index: a function of the file that
/// declares it, or one this file does not show.
fn may_read_array(source: &Source, callback: &Expression, index: usize) -> bool {
    let function: Option<FunctionLike> = match unwrap_chain(callback) {
        Expression::Identifier(_) => function_named(source, callback),
        other => function_like(other),
    };
    function.is_none_or(|function| function.may_read_argument(source, index))
}

/// The `drop(…).take(…)` a `slice(start, end)` is, when its bounds are fixed whole numbers.
fn slice_helpers(source: &Source, arguments: &[&Expression]) -> Option<String> {
    let bounds: Option<Vec<u64>> = arguments
        .iter()
        .map(|argument| match static_value(source, argument)? {
            Value::Number(number)
                if number >= 0.0 && number.fract() == 0.0 && number <= 9_007_199_254_740_991.0 =>
            {
                number.to_string().parse::<u64>().ok()
            }
            _ => None,
        })
        .collect();
    let bounds = bounds?;
    let (start, end) = (bounds.first().copied()?, bounds.get(1).copied());
    if end.is_some_and(|end| end <= start) {
        return Some("take(0)".to_owned());
    }
    let mut calls = Vec::new();
    if start > 0 {
        calls.push(format!("drop({start})"));
    }
    if let Some(end) = end {
        calls.push(format!("take({})", end - start));
    }
    (!calls.is_empty()).then(|| calls.join("."))
}

impl Rule for PreferIteratorHelpers {
    fn name(&self) -> &'static str {
        "unicorn/prefer-iterator-helpers"
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
        let Some(arguments) = method.arguments() else {
            return;
        };
        if method.name == "slice"
            && (1..=2).contains(&arguments.len())
            && call.type_arguments.is_none()
        {
            let to_array = method_named(method.object, &["toArray"])
                .filter(|inner| {
                    inner.is_plain()
                        && inner.call.arguments.is_empty()
                        && inner.call.type_arguments.is_none()
                        && !has_optional_chain(inner.object)
                })
                .map(|inner| inner.object);
            let iterator = to_array.or_else(|| spread_or_from(source, method.object));
            if iterator.is_some_and(|each| !matches!(each, Expression::Super(_)))
                && let Some(replacement) = slice_helpers(source, &arguments)
            {
                findings.push(source.finding(
                    self.name(),
                    method.property.start,
                    format!("Prefer `{replacement}` before materializing the iterator instead of slicing the array."),
                ));
                return;
            }
        }
        let index = match (method.name, arguments.len()) {
            ("every" | "find" | "forEach" | "some", 1) => 2,
            ("reduce", 1 | 2) => 3,
            _ => return,
        };
        if may_read_array(source, arguments[0], index) {
            return;
        }
        if spread_or_from(source, method.object).is_some() {
            findings.push(source.finding(
                self.name(),
                method.object.span().start,
                format!(
                    "Prefer `Iterator#{}()` over materializing an array.",
                    method.name
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferIteratorHelpers;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferIteratorHelpers, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_temporary_arrays() {
        assert_eq!(
            check("[...map.values()].find(value => value.id === id);"),
            ["Prefer `Iterator#find()` over materializing an array."]
        );
        assert_eq!(
            check("iterator.toArray().slice(20, 30);"),
            [
                "Prefer `drop(20).take(10)` before materializing the iterator instead of slicing the array."
            ]
        );
        assert_eq!(
            check("[...text.matchAll(pattern)].slice(0, 10);"),
            ["Prefer `take(10)` before materializing the iterator instead of slicing the array."]
        );
        assert_eq!(check("Array.from(map.keys()).some(k => k > 1);").len(), 1);
        assert_eq!(
            check("Array.from(map.keys()).reduce((a, k) => a + k, 0);").len(),
            1
        );
        assert_eq!(check("const n = 2;\nit.toArray().slice(n);").len(), 1);
        assert_eq!(
            check("it.toArray().slice(5, 2);"),
            ["Prefer `take(0)` before materializing the iterator instead of slicing the array."]
        );
        assert_eq!(
            check("function f(x: number) { return x > 1; }\n[...set.values()].every(f);").len(),
            1
        );
    }

    #[test]
    fn accepts_callbacks_that_may_read_the_array_and_other_arrays() {
        for code in [
            "map.values().find(value => value.id === id);",
            "iterator.drop(20).take(10).toArray();",
            "[...map.values()].find((value, index, array) => array.length > 1);",
            "[...map.values()].find(callback);",
            "[...map.values()].find(function (value) { return arguments.length > 1; });",
            "[...map.values()].reduce((a, v, i, all) => a + all.length, 0);",
            "[...map.values()].map(f);",
            "[...items].find(v => v);",
            "[...map.values(), 1].find(v => v);",
            "it.toArray().slice();",
            "it.toArray().slice(0);",
            "it.toArray().slice(-1);",
            "it.toArray().slice(n);",
            "it?.toArray().slice(1);",
            "Array.from<number>(map.keys()).some(k => k);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
