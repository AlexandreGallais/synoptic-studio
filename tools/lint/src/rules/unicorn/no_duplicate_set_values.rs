//! `unicorn/no-duplicate-set-values`: no value written twice in the array literal given to
//! `new Set()`, a static value or the same reference (`eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{ArrayExpressionElement, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{Value, same_reference, static_value, text, unwrap};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDuplicateSetValues;

/// An element read once: the expression (none for a hole) and its static value.
struct Element<'b, 'a> {
    /// The expression, none for a hole.
    expression: Option<&'b Expression<'a>>,
    /// Its value, when it always has one.
    value: Option<Value>,
}

/// Whether two elements hold the same value: equal static values, or the same reference when
/// neither is a literal.
fn is_duplicate(left: &Element, right: &Element) -> bool {
    if let (Some(a), Some(b)) = (&left.value, &right.value) {
        return a.same_value_zero(b);
    }
    match (left.expression, right.expression) {
        (Some(a), Some(b)) => !is_literal(a) && !is_literal(b) && same_reference(a, b),
        _ => false,
    }
}

/// Whether an expression is a literal.
fn is_literal(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
    )
}

impl Rule for NoDuplicateSetValues {
    fn name(&self) -> &'static str {
        "unicorn/no-duplicate-set-values"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::NewExpression(new) = node.kind() else {
            return;
        };
        let set = matches!(&new.callee, Expression::Identifier(callee) if callee.name == "Set");
        let [argument] = new.arguments.as_slice() else {
            return;
        };
        let Some(iterable) = argument.as_expression().filter(|_| set) else {
            return;
        };
        let Expression::ArrayExpression(array) = unwrap(iterable) else {
            return;
        };
        let mut seen: Vec<Element> = Vec::new();
        for element in &array.elements {
            let expression = match element {
                ArrayExpressionElement::SpreadElement(_) => continue,
                ArrayExpressionElement::Elision(_) => None,
                other => other.as_expression(),
            };
            let current = Element {
                expression,
                value: match expression {
                    Some(expression) => static_value(source, expression),
                    None => Some(Value::Undefined),
                },
            };
            if seen.iter().any(|each| is_duplicate(each, &current)) {
                let (at, written) = match expression {
                    Some(expression) => (expression.span().start, text(source, expression.span())),
                    None => (iterable.span().start, "undefined"),
                };
                findings.push(source.finding(
                    self.name(),
                    at,
                    format!("Remove duplicate value `{written}` from the Set."),
                ));
            }
            seen.push(current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateSetValues;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoDuplicateSetValues, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_duplicates() {
        assert_eq!(
            check("const set = new Set([\n\t'foo',\n\t'bar',\n\t'foo',\n]);"),
            ["Remove duplicate value `'foo'` from the Set."]
        );
        assert_eq!(
            check("const foo = 2;\nconst set = new Set([foo, 2]);"),
            ["Remove duplicate value `2` from the Set."]
        );
        assert_eq!(
            check("const set = new Set([foo.bar, foo.bar]);"),
            ["Remove duplicate value `foo.bar` from the Set."]
        );
        // `1` and `1.0`, `NaN`s, a hole and `undefined`, a template and a string, three times.
        assert_eq!(check("new Set([1, 1.0]);").len(), 1);
        assert_eq!(check("new Set([0, -0]);").len(), 1);
        assert_eq!(
            check("new Set([, undefined]);"),
            ["Remove duplicate value `undefined` from the Set."]
        );
        assert_eq!(check("new Set([`a`, 'a']);").len(), 1);
        assert_eq!(check("new Set([a, a, a] as const);").len(), 2);
    }

    #[test]
    fn accepts_distinct_values_and_other_shapes() {
        for code in [
            "const set = new Set(['foo', 'bar']);",
            "const foo = 2;\nconst set = new Set([foo, 3]);",
            "const set = new Set([foo.bar, foo.baz]);",
            "const set = new Set([...a, ...a]);",
            "const set = new Set([f(), f()]);",
            "const set = new Set(['1', 1]);",
            "const array = ['foo', 'foo'];",
            "const map = new Map([['a', 1], ['a', 2]]);",
            "const set = new Set(['a', 'a'], extra);",
            "const set = new Set(items);",
            "const set = new WeakSet([a, a]);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
