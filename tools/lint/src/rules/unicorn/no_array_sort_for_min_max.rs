//! `unicorn/no-array-sort-for-min-max`: no array sorted by a numeric comparator only to read its
//! first or last element, which `Math.min()` or `Math.max()` gives without sorting
//! (`eslint-plugin-unicorn` 78.0.0). Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{
    Kind, callback, has_optional_chain, is_known_non_indexed, kind_of, method_call, method_named,
    unwrap,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoArraySortForMinMax;

/// Whether an expression is the literal `0`.
fn is_zero(expression: &Expression) -> bool {
    matches!(expression, Expression::NumericLiteral(literal) if literal.value == 0.0)
}

/// Whether an expression is `-1`.
fn is_minus_one(expression: &Expression) -> bool {
    matches!(expression, Expression::UnaryExpression(unary)
        if unary.operator == UnaryOperator::UnaryNegation
            && matches!(&unary.argument, Expression::NumericLiteral(literal) if (literal.value - 1.0).abs() < f64::EPSILON))
}

/// Whether a sort's result, read at an end, comes from a numeric comparator: `true` when it
/// sorts ascending.
fn ascending(source: &Source, sorted: &Expression) -> Option<bool> {
    let sort =
        method_named(sorted, &["sort", "toSorted"]).filter(super::syntax::MethodCall::is_plain)?;
    let Some(&[comparator]) = sort.arguments().as_deref() else {
        return None;
    };
    if has_optional_chain(sort.object) {
        return None;
    }
    let function = callback(comparator)?;
    let generator = matches!(comparator, Expression::FunctionExpression(each) if each.generator);
    let is_async = match comparator {
        Expression::ArrowFunctionExpression(arrow) => arrow.r#async,
        Expression::FunctionExpression(each) => each.r#async,
        _ => false,
    };
    let Some(&[first, second]) = function.parameter_names().as_deref() else {
        return None;
    };
    let Expression::BinaryExpression(body) = unwrap(function.returned()?) else {
        return None;
    };
    if generator || is_async || first == second || body.operator != BinaryOperator::Subtraction {
        return None;
    }
    let is = |expression: &Expression, name: &str| matches!(expression, Expression::Identifier(identifier) if identifier.name == name);
    let direction = if is(&body.left, first) && is(&body.right, second) {
        true
    } else if is(&body.left, second) && is(&body.right, first) {
        false
    } else {
        return None;
    };
    let big = matches!(kind_of(source, sort.object), Some(Kind::TypedArray))
        && matches!(unwrap(sort.object), Expression::NewExpression(new)
            if matches!(&new.callee, Expression::Identifier(callee) if callee.name.starts_with("Big")));
    (!is_known_non_indexed(source, sort.object) && !big).then_some(direction)
}

/// The finding at a read of the first or last element of a sorted array.
fn finding(source: &Source, rule: &'static str, at: u32, ascending: bool, first: bool) -> Finding {
    let method = if ascending == first { "min" } else { "max" };
    let extreme = if method == "min" {
        "minimum"
    } else {
        "maximum"
    };
    source.finding(
        rule,
        at,
        format!("Use `Math.{method}()` instead of sorting to get the {extreme} value."),
    )
}

/// Whether a node is written: assigned, updated, deleted, destructured into or a loop's target.
fn is_written(source: &Source, node: &AstNode) -> bool {
    let span = node.kind().span();
    match source.semantic.nodes().parent_kind(node.id()) {
        AstKind::AssignmentExpression(assignment) => assignment.left.span() == span,
        AstKind::UpdateExpression(_)
        | AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetWithDefault(_)
        | AstKind::AssignmentTargetPropertyProperty(_)
        | AstKind::AssignmentTargetRest(_)
        | AstKind::ForOfStatement(_)
        | AstKind::ForInStatement(_) => true,
        AstKind::UnaryExpression(unary) => unary.operator == UnaryOperator::Delete,
        _ => false,
    }
}

impl Rule for NoArraySortForMinMax {
    fn name(&self) -> &'static str {
        "unicorn/no-array-sort-for-min-max"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::ComputedMemberExpression(member)
                if !member.optional && is_zero(&member.expression) && !is_written(source, node) =>
            {
                if let Some(direction) = ascending(source, &member.object) {
                    findings.push(finding(
                        source,
                        self.name(),
                        member.span.start,
                        direction,
                        true,
                    ));
                }
            }
            AstKind::CallExpression(call) => {
                let Some(at) =
                    method_call(call).filter(|method| method.name == "at" && method.is_plain())
                else {
                    return;
                };
                let first = match at.arguments().as_deref() {
                    Some(&[index]) if is_zero(index) => true,
                    Some(&[index]) if is_minus_one(index) => false,
                    _ => return,
                };
                if let Some(direction) = ascending(source, at.object) {
                    findings.push(finding(
                        source,
                        self.name(),
                        call.span.start,
                        direction,
                        first,
                    ));
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoArraySortForMinMax;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoArraySortForMinMax, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_sorts_for_an_extreme() {
        let minimum = ["Use `Math.min()` instead of sorting to get the minimum value."];
        let maximum = ["Use `Math.max()` instead of sorting to get the maximum value."];

        assert_eq!(
            check("const minimum = array.sort((a, b) => a - b)[0];"),
            minimum
        );
        assert_eq!(
            check("const minimum = array.toSorted((a, b) => a - b).at(0);"),
            minimum
        );
        assert_eq!(
            check("const maximum = array.sort((a, b) => a - b).at(-1);"),
            maximum
        );
        assert_eq!(
            check("const maximum = array.toSorted((a, b) => b - a)[0];"),
            maximum
        );
        assert_eq!(
            check("const minimum = array.toSorted((a, b) => b - a).at(-1);"),
            minimum
        );
        assert_eq!(
            check("const m = array.toSorted(function (a, b) { return a - b; })[0];"),
            minimum
        );
    }

    #[test]
    fn accepts_other_comparators_and_reads() {
        for code in [
            "const minimum = Math.min(...array);",
            "const sorted = array.toSorted((a, b) => a - b);",
            "const first = array.toSorted((a, b) => a.value - b.value)[0];",
            "const first = array.toSorted((a, b) => a - b)[1];",
            "const last = array.toSorted((a, b) => a - b).at(1);",
            "const first = array.toSorted()[0];",
            "const first = array.toSorted((a, a2) => a - a)[0];",
            "const first = array.toSorted(async (a, b) => a - b)[0];",
            "const first = array.toSorted((a, b) => a + b)[0];",
            "const first = array?.toSorted((a, b) => a - b)[0];",
            "const first = array.toSorted((a, b) => a - b)?.[0];",
            "array.toSorted((a, b) => a - b)[0] = 1;",
            "const first = new Set().toSorted((a, b) => a - b)[0];",
            "const first = array.toSorted(compare)[0];",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
