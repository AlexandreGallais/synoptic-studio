//! `unicorn/no-impossible-length-comparison`: no comparison of a `.length` or a `.size` with a
//! negative bound, nor `>= 0`, always `false` or always `true` since a count is never negative
//! (`eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, LogicalOperator, ObjectPropertyKind};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{const_init, same_reference, static_number, unwrap};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoImpossibleLengthComparison;

/// The properties of a shape that make `length` a dimension rather than a count.
const SHAPE_PROPERTIES: [&str; 3] = ["depth", "height", "width"];

/// The object and the property of a `.length` or `.size` read without `?.`.
fn length_or_size<'b, 'a>(expression: &'b Expression<'a>) -> Option<(&'b Expression<'a>, &'a str)> {
    match unwrap(expression) {
        Expression::StaticMemberExpression(member)
            if !member.optional && matches!(member.property.name.as_str(), "length" | "size") =>
        {
            Some((&member.object, member.property.name.as_str()))
        }
        _ => None,
    }
}

/// The number a side is: a literal, or a `const` initialized with one.
fn bound(source: &Source, expression: &Expression) -> Option<f64> {
    static_number(expression).or_else(|| static_number(const_init(source, unwrap(expression))?))
}

/// The operator read with its sides swapped.
fn flipped(operator: BinaryOperator) -> BinaryOperator {
    match operator {
        BinaryOperator::LessThan => BinaryOperator::GreaterThan,
        BinaryOperator::LessEqualThan => BinaryOperator::GreaterEqualThan,
        BinaryOperator::GreaterThan => BinaryOperator::LessThan,
        BinaryOperator::GreaterEqualThan => BinaryOperator::LessEqualThan,
        other => other,
    }
}

/// What `count <operator> value` always gives, for a count that is never negative.
fn constant_result(operator: BinaryOperator, value: f64) -> Option<bool> {
    if !value.is_finite() {
        return None;
    }
    match operator {
        BinaryOperator::LessThan => (value <= 0.0).then_some(false),
        BinaryOperator::LessEqualThan
        | BinaryOperator::StrictEquality
        | BinaryOperator::Equality => (value < 0.0).then_some(false),
        BinaryOperator::GreaterThan
        | BinaryOperator::StrictInequality
        | BinaryOperator::Inequality => (value < 0.0).then_some(true),
        BinaryOperator::GreaterEqualThan => (value <= 0.0).then_some(true),
        _ => None,
    }
}

/// Whether the object is a `const` bound to an object literal whose property is not a count:
/// a value that is not a non-negative integer, an accessor, or one the literal cannot tell.
fn is_custom_shape(source: &Source, object: &Expression, property: &str) -> bool {
    let Some(Expression::ObjectExpression(literal)) = const_init(source, object) else {
        return false;
    };
    let value = literal.properties.iter().rev().find_map(|each| match each {
        ObjectPropertyKind::ObjectProperty(entry)
            if entry.key.is_specific_static_name(property) =>
        {
            Some(
                static_number(&entry.value)
                    .filter(|_| !entry.method && entry.kind == oxc_ast::ast::PropertyKind::Init),
            )
        }
        _ => None,
    });
    !matches!(value, Some(Some(number)) if number >= 0.0 && number.fract() == 0.0)
}

/// The operands of an `&&` chain, flattened.
fn and_operands<'b, 'a>(expression: &'b Expression<'a>, operands: &mut Vec<&'b Expression<'a>>) {
    match expression {
        Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::And => {
            and_operands(&logical.left, operands);
            and_operands(&logical.right, operands);
        }
        _ => operands.push(expression),
    }
}

/// Whether the comparison sits in an `&&` chain that reads the object's width, height or depth.
fn has_shape_check(source: &Source, node: &AstNode, object: &Expression) -> bool {
    let nodes = source.semantic.nodes();
    let mut root = None;
    for ancestor in nodes.ancestors(node.id()) {
        match ancestor.kind() {
            AstKind::LogicalExpression(logical) if logical.operator == LogicalOperator::And => {
                root = Some(logical);
            }
            _ => break,
        }
    }
    let Some(root) = root else {
        return false;
    };
    let mut operands = Vec::new();
    and_operands(&root.left, &mut operands);
    and_operands(&root.right, &mut operands);
    operands.into_iter().any(|operand| {
        matches!(unwrap(operand), Expression::StaticMemberExpression(member)
            if !member.optional
                && SHAPE_PROPERTIES.contains(&member.property.name.as_str())
                && same_reference(&member.object, object))
    })
}

impl Rule for NoImpossibleLengthComparison {
    fn name(&self) -> &'static str {
        "unicorn/no-impossible-length-comparison"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::BinaryExpression(binary) = node.kind() else {
            return;
        };
        let (count, operator, other) = if let Some(count) = length_or_size(&binary.left) {
            (count, binary.operator, &binary.right)
        } else if let Some(count) = length_or_size(&binary.right) {
            (count, flipped(binary.operator), &binary.left)
        } else {
            return;
        };
        let (object, property) = count;
        let skipped = matches!(
            unwrap(object),
            Expression::ThisExpression(_) | Expression::Super(_)
        ) || matches!(object, Expression::ChainExpression(_))
            || is_custom_shape(source, unwrap(object), property)
            || has_shape_check(source, node, object);
        let Some(result) = bound(source, other).and_then(|value| constant_result(operator, value))
        else {
            return;
        };
        if !skipped {
            findings.push(source.finding(
                self.name(),
                binary.span().start,
                format!(
                    "This comparison is always {result} because `.{property}` is always a non-negative integer."
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoImpossibleLengthComparison;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoImpossibleLengthComparison, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_impossible_comparisons() {
        let never = |property: &str| {
            format!(
                "This comparison is always false because `.{property}` is always a non-negative integer."
            )
        };
        let always =
            "This comparison is always true because `.length` is always a non-negative integer.";

        assert_eq!(check("if (array.length < 0) {}"), [never("length")]);
        assert_eq!(check("if (string.length === -1) {}"), [never("length")]);
        assert_eq!(check("if (set.size <= -1) {}"), [never("size")]);
        assert_eq!(check("if (array.length >= 0) {}"), [always]);
        // Flipped, through a constant, a `!=`, `>` and `<=` against a negative bound.
        assert_eq!(check("if (0 > array.length) {}"), [never("length")]);
        assert_eq!(
            check("const NONE = -1; if (array.length == NONE) {}"),
            [never("length")]
        );
        assert_eq!(check("if (array.length !== -1) {}").len(), 1);
        assert_eq!(check("if (array.length > -2) {}").len(), 1);
        assert_eq!(
            check("const o = { length: 3 }; if (o.length < 0) {}").len(),
            1
        );
    }

    #[test]
    fn accepts_possible_comparisons_and_custom_shapes() {
        for code in [
            "if (array.length === 0) {}",
            "if (array.length > 0) {}",
            "if (set.size >= 1) {}",
            "if (array.length <= 0) {}",
            "if (array.length < 1) {}",
            "if (array.length < n) {}",
            "if (array.length < Infinity) {}",
            "if (array.count < 0) {}",
            "if (array?.length < 0) {}",
            "if (this.length < 0) {}",
            "if (dimensions.width && dimensions.length < 0) {}",
            "if (box.height > 1 && box.depth && box.length < 0) {}",
            "const o = { length: -1 }; if (o.length < 0) {}",
            "const o = { get length() { return 1; } }; if (o.length < 0) {}",
            "const o = { size: n }; if (o.size < 0) {}",
            "if (a.length + b < 0) {}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
