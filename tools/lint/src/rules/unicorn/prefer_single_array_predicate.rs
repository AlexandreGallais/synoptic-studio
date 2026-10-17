//! `unicorn/prefer-single-array-predicate`: no `array.some(…) || array.some(…)` nor
//! `array.every(…) && array.every(…)`, which one call with the predicates combined does in one
//! pass (`eslint-plugin-unicorn` 78.0.0). Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, Expression, LogicalExpression, LogicalOperator};
use oxc_semantic::AstNode;

use super::syntax::{is_known_non_indexed, method_call, same_reference, text};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSingleArrayPredicate;

/// The operands of a chain of one logical operator, flattened.
fn operands<'b, 'a>(
    expression: &'b Expression<'a>,
    operator: LogicalOperator,
    found: &mut Vec<&'b Expression<'a>>,
) {
    match expression {
        Expression::LogicalExpression(logical) if logical.operator == operator => {
            operands(&logical.left, operator, found);
            operands(&logical.right, operator, found);
        }
        _ => found.push(expression),
    }
}

/// The receiver and the parameter's text of a call of `method` with an arrow of one plain
/// parameter and a concise body.
fn predicate_call<'b, 'a>(
    source: &Source<'_, 'a>,
    expression: &'b Expression<'a>,
    method: &str,
) -> Option<(&'b Expression<'a>, &'a str)> {
    let Expression::CallExpression(call) = expression else {
        return None;
    };
    let found = method_call(call).filter(|each| each.name == method && each.is_plain())?;
    let Some(&[Expression::ArrowFunctionExpression(arrow)]) = found.arguments().as_deref() else {
        return None;
    };
    let plain = !arrow.r#async
        && arrow.return_type.is_none()
        && arrow.type_parameters.is_none()
        && arrow.params.rest.is_none()
        && arrow.get_expression().is_some()
        && call.type_arguments.is_none();
    let [parameter] = arrow.params.items.as_slice() else {
        return None;
    };
    let BindingPattern::BindingIdentifier(_) = parameter.pattern else {
        return None;
    };
    if !plain || parameter.initializer.is_some() || is_known_non_indexed(source, found.object) {
        return None;
    }
    Some((found.object, text(source, parameter.span)))
}

impl Rule for PreferSingleArrayPredicate {
    fn name(&self) -> &'static str {
        "unicorn/prefer-single-array-predicate"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::LogicalExpression(logical) = node.kind() else {
            return;
        };
        let method = match logical.operator {
            LogicalOperator::Or => "some",
            LogicalOperator::And => "every",
            LogicalOperator::Coalesce => return,
        };
        let outermost = !matches!(
            source.semantic.nodes().parent_kind(node.id()),
            AstKind::LogicalExpression(LogicalExpression { operator, .. }) if *operator == logical.operator
        );
        if !outermost {
            return;
        }
        let mut found = Vec::new();
        operands(&logical.left, logical.operator, &mut found);
        operands(&logical.right, logical.operator, &mut found);
        let calls: Option<Vec<_>> = found
            .iter()
            .map(|operand| predicate_call(source, operand, method))
            .collect();
        let Some(calls) = calls else {
            return;
        };
        let Some((object, parameter)) = calls.first() else {
            return;
        };
        let compatible = calls
            .iter()
            .all(|(other, text)| same_reference(object, other) && text == parameter);
        if calls.len() >= 2 && compatible {
            findings.push(source.finding(
                self.name(),
                logical.span.start,
                format!("Prefer a single `Array#{method}()` with a combined predicate."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSingleArrayPredicate;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferSingleArrayPredicate, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_repeated_predicates() {
        let some = ["Prefer a single `Array#some()` with a combined predicate."];

        assert_eq!(
            check(
                "const hasErrors = messages.some(m => m.type === 'error') || messages.some(m => m.severity === 'critical');"
            ),
            some
        );
        assert_eq!(
            check(
                "const allValid = items.every(item => item.valid) && items.every(item => item.complete);"
            ),
            ["Prefer a single `Array#every()` with a combined predicate."]
        );
        assert_eq!(
            check(
                "const m = values.some(value => value === null) || values.some(value => value === undefined) || values.some(value => value === 0);"
            ),
            some
        );
        assert_eq!(
            check("const m = a.b.some((x: number) => x > 1) || a.b.some((x: number) => x < 0);"),
            some
        );
    }

    #[test]
    fn accepts_different_methods_receivers_and_callbacks() {
        for code in [
            "const ok = array.some(x => x > 5) && array.every(x => x < 100);",
            "const ok = array.some(x => x > 5) || someOtherCondition;",
            "const ok = array.some(x => x > 5) || other.some(x => x < 0);",
            "const ok = array.some(x => x > 5) || array.some(y => y < 0);",
            "const ok = array.some(x => x > 5) ?? array.some(x => x < 0);",
            "const ok = array.some(x => { return x > 5; }) || array.some(x => x < 0);",
            "const ok = array.some(async x => x) || array.some(x => x);",
            "const ok = array.some(([x]) => x) || array.some(x => x);",
            "const ok = array.some((x = 1) => x) || array.some(x => x);",
            "const ok = array.some(f) || array.some(g);",
            "const ok = array?.some(x => x) || array.some(x => x);",
            "const ok = array.some<number>(x => x) || array.some(x => x);",
            "const s = new Set(); const ok = s.some(x => x) || s.some(x => !x);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
