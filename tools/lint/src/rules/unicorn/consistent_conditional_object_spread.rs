//! `unicorn/consistent-conditional-object-spread`, in its default `logical` style: a conditional
//! spread is written `...(condition && object)`, without an empty branch nor a boolean cast,
//! since every falsy value spreads nothing (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in
//! `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryOperator, ConditionalExpression, Expression, LogicalExpression, LogicalOperator,
    UnaryOperator,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{
    has_comment, is_global, logical_operand_text, negated_text, same_reference, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ConsistentConditionalObjectSpread;

/// The message on a conditional spread written as a ternary.
const LOGICAL: &str = "Prefer logical conditional object spreads.";
/// The message on a boolean cast.
const CAST: &str = "Unnecessary boolean cast. A falsy value spreads nothing.";

/// Whether a branch spreads nothing: `{}`, `undefined` or `null`.
fn is_empty(expression: &Expression) -> bool {
    match expression {
        Expression::ObjectExpression(object) => object.properties.is_empty(),
        Expression::Identifier(identifier) => identifier.name == "undefined",
        Expression::NullLiteral(_) => true,
        _ => false,
    }
}

/// Whether two expressions are the same reference, or the same text with nothing that changes
/// a value (`isSameNode`).
fn same_node(source: &Source, left: &Expression, right: &Expression) -> bool {
    if same_reference(left, right) {
        return true;
    }
    match (left, right) {
        (Expression::AwaitExpression(a), Expression::AwaitExpression(b)) => {
            same_node(source, &a.argument, &b.argument)
        }
        (Expression::LogicalExpression(a), Expression::LogicalExpression(b)) => {
            a.operator == b.operator
                && same_node(source, &a.left, &b.left)
                && same_node(source, &a.right, &b.right)
        }
        (Expression::UnaryExpression(a), Expression::UnaryExpression(b)) => {
            a.operator == b.operator && same_node(source, &a.argument, &b.argument)
        }
        (Expression::UpdateExpression(_), _) => false,
        _ => {
            std::mem::discriminant(left) == std::mem::discriminant(right)
                && text(source, left.span()) == text(source, right.span())
        }
    }
}

/// A test against `null` or `undefined`: what it checks, and whether it holds when that is
/// nullish (`getNullishTest`).
fn nullish_test<'b, 'a>(expression: &'b Expression<'a>) -> Option<(&'b Expression<'a>, bool)> {
    /// One comparison: the reference, its kind (`None` for both, with `==`), whether it holds
    /// when nullish.
    fn comparison<'b, 'a>(
        expression: &'b Expression<'a>,
    ) -> Option<(&'b Expression<'a>, Option<&'static str>, bool)> {
        let Expression::BinaryExpression(binary) = expression else {
            return None;
        };
        let holds = match binary.operator {
            BinaryOperator::Equality | BinaryOperator::StrictEquality => true,
            BinaryOperator::Inequality | BinaryOperator::StrictInequality => false,
            _ => return None,
        };
        let kind = |side: &Expression| match side {
            Expression::NullLiteral(_) => Some("null"),
            Expression::Identifier(identifier) if identifier.name == "undefined" => {
                Some("undefined")
            }
            _ => None,
        };
        let (reference, kind) = match (kind(&binary.left), kind(&binary.right)) {
            (Some(kind), None) => (&binary.right, kind),
            (None, Some(kind)) => (&binary.left, kind),
            _ => return None,
        };
        let loose = matches!(
            binary.operator,
            BinaryOperator::Equality | BinaryOperator::Inequality
        );
        Some((reference, (!loose).then_some(kind), holds))
    }
    if let Some((reference, None, holds)) = comparison(expression) {
        return Some((reference, holds));
    }
    let Expression::LogicalExpression(logical) = expression else {
        return None;
    };
    let (left, Some(left_kind), left_holds) = comparison(&logical.left)? else {
        return None;
    };
    let (right, Some(right_kind), right_holds) = comparison(&logical.right)? else {
        return None;
    };
    let both = left_kind != right_kind && same_reference(left, right);
    match logical.operator {
        LogicalOperator::Or if both && left_holds && right_holds => Some((left, true)),
        LogicalOperator::And if both && !left_holds && !right_holds => Some((left, false)),
        _ => None,
    }
}

/// Whether a node is the argument of a spread in an object literal.
fn is_spread_argument(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    matches!(parent.kind(), AstKind::SpreadElement(_))
        && matches!(nodes.parent_kind(parent.id()), AstKind::ObjectExpression(_))
}

/// The finding on a ternary spread, when one branch spreads nothing.
fn ternary_finding(
    source: &Source,
    rule: &'static str,
    conditional: &ConditionalExpression,
) -> Option<Finding> {
    let alternate_empty = is_empty(&conditional.alternate);
    if alternate_empty == is_empty(&conditional.consequent) {
        return None;
    }
    let (kept, test) = (
        if alternate_empty {
            &conditional.consequent
        } else {
            &conditional.alternate
        },
        &conditional.test,
    );
    let same_as_test = if alternate_empty {
        same_node(source, test, kept)
    } else {
        matches!(test, Expression::UnaryExpression(unary)
            if unary.operator == UnaryOperator::LogicalNot && same_node(source, &unary.argument, kept))
    };
    let nullish = nullish_test(test).is_some_and(|(reference, holds)| {
        holds != alternate_empty && same_node(source, reference, kept)
    });
    if same_as_test || nullish {
        return None;
    }
    if has_comment(source, conditional.span) {
        return Some(source.finding(rule, conditional.span.start, LOGICAL.to_owned()));
    }
    let test_text = if alternate_empty {
        logical_operand_text(source, test, LogicalOperator::And, true)
    } else {
        match test {
            Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
                logical_operand_text(source, &unary.argument, LogicalOperator::And, true)
            }
            _ => negated_text(source, test),
        }
    };
    let kept_text = logical_operand_text(source, kept, LogicalOperator::And, true);
    Some(source.fixed(
        rule,
        conditional.span.start,
        LOGICAL.to_owned(),
        Fix {
            start: conditional.span.start,
            end: conditional.span.end,
            text: format!("{test_text} && {kept_text}"),
        },
    ))
}

/// The finding on a boolean cast, `!!value` or `Boolean(value)`.
fn cast_finding(source: &Source, rule: &'static str, operand: &Expression) -> Option<Finding> {
    let (argument, keep_written) = match operand {
        Expression::UnaryExpression(outer) if outer.operator == UnaryOperator::LogicalNot => {
            match &outer.argument {
                Expression::UnaryExpression(inner)
                    if inner.operator == UnaryOperator::LogicalNot =>
                {
                    (&inner.argument, true)
                }
                _ => return None,
            }
        }
        Expression::CallExpression(call)
            if !call.optional
                && is_global(source, &call.callee, "Boolean")
                && call.arguments.len() == 1 =>
        {
            (call.arguments[0].as_expression()?, false)
        }
        _ => return None,
    };
    let span: Span = operand.span();
    if has_comment(source, span) {
        return Some(source.finding(rule, span.start, CAST.to_owned()));
    }
    let replacement = logical_operand_text(source, argument, LogicalOperator::And, keep_written);
    Some(source.fixed(
        rule,
        span.start,
        CAST.to_owned(),
        Fix {
            start: span.start,
            end: span.end,
            text: replacement,
        },
    ))
}

/// The findings on the boolean casts of an `&&` spread, its last operand aside.
fn cast_findings(source: &Source, rule: &'static str, logical: &LogicalExpression) -> Vec<Finding> {
    let mut findings = Vec::new();
    if logical.operator != LogicalOperator::And {
        return findings;
    }
    let mut node = &logical.left;
    while let Expression::LogicalExpression(inner) = node {
        if inner.operator != LogicalOperator::And {
            break;
        }
        findings.extend(cast_finding(source, rule, &inner.right));
        node = &inner.left;
    }
    findings.extend(cast_finding(source, rule, node));
    findings
}

impl Rule for ConsistentConditionalObjectSpread {
    fn name(&self) -> &'static str {
        "unicorn/consistent-conditional-object-spread"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::ConditionalExpression(conditional) if is_spread_argument(source, node) => {
                findings.extend(ternary_finding(source, self.name(), conditional));
            }
            AstKind::LogicalExpression(logical) if is_spread_argument(source, node) => {
                findings.extend(cast_findings(source, self.name(), logical));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsistentConditionalObjectSpread;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&ConsistentConditionalObjectSpread, "src/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&ConsistentConditionalObjectSpread, "src/a.ts", code),
        )
    }

    #[test]
    fn reports_and_fixes_the_documented_ternaries() {
        assert_eq!(
            check("const object = {...(condition ? {property} : {})};"),
            ["Prefer logical conditional object spreads."]
        );
        assert_eq!(
            fix("const object = {...(condition ? {property} : {})};"),
            "const object = {...(condition && {property})};"
        );
        assert_eq!(
            fix("const object = {...(condition ? {property} : undefined)};"),
            "const object = {...(condition && {property})};"
        );
        assert_eq!(
            fix("const object = {...(condition ? null : {property})};"),
            "const object = {...(!condition && {property})};"
        );
        assert_eq!(
            fix("const object = {...(!condition ? {} : {property})};"),
            "const object = {...(condition && {property})};"
        );
        assert_eq!(
            fix("const object = {...(a || b ? {property} : {})};"),
            "const object = {...((a || b) && {property})};"
        );
        assert_eq!(
            fix("const object = {...(a > b ? {} : x ? y : z)};"),
            "const object = {...(!(a > b) && (x ? y : z))};"
        );
    }

    #[test]
    fn reports_and_fixes_the_documented_boolean_casts() {
        let cast = "Unnecessary boolean cast. A falsy value spreads nothing.";

        assert_eq!(
            check("const object = {...(!!condition && {property})};"),
            [cast]
        );
        assert_eq!(
            fix("const object = {...(!!condition && {property})};"),
            "const object = {...(condition && {property})};"
        );
        assert_eq!(
            fix("const object = {...(Boolean(condition) && {property})};"),
            "const object = {...(condition && {property})};"
        );
        assert_eq!(
            check("const object = {...(!!first && !!second && {property})};"),
            [cast, cast]
        );
        assert_eq!(
            fix("const object = {...(!!first && !!second && {property})};"),
            "const object = {...(first && second && {property})};"
        );
        assert_eq!(
            fix("const object = {...(Boolean(a || b) && {property})};"),
            "const object = {...((a || b) && {property})};"
        );
    }

    #[test]
    fn reports_without_a_fix_around_a_comment() {
        for code in [
            "const object = {...(condition /* c */ ? {property} : {})};",
            "const object = {...(!!/* c */condition && {property})};",
        ] {
            let findings = lint(&ConsistentConditionalObjectSpread, "src/a.ts", code);
            assert_eq!(findings.len(), 1, "{code}");
            assert_eq!(fixed(code, &findings), code, "{code}");
        }
    }

    #[test]
    fn accepts_logical_spreads_and_other_shapes() {
        for code in [
            "const object = {...(condition && {property})};",
            "const object = {...(first && second && {property})};",
            "const object = {...(condition ? {a} : {b})};",
            "const object = {...(condition ? {} : undefined)};",
            "const object = {...(options ? options : {})};",
            "const object = {...(!options ? {} : options)};",
            "const object = {...(options != null ? options : {})};",
            "const object = {...(options == null ? {} : options)};",
            "const object = {...(options === null || options === undefined ? {} : options)};",
            "const object = {...(options !== null && options !== undefined ? options : {})};",
            "const array = [...(condition ? [a] : [])];",
            "const object = {...(x || {})};",
            "const value = condition ? {property} : {};",
            "const object = {...(!!a || {property})};",
            "const object = {...(Boolean(...a) && {property})};",
            "const Boolean = (x: unknown) => x; const object = {...(Boolean(a) && {property})};",
            "const object = {...(!a && {property})};",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
