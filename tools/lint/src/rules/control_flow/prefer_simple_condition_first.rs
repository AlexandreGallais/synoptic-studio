//! `unicorn/prefer-simple-condition-first`: in an `&&` or `||` chain read for its truth, a simple
//! condition — a name, its negation, a strict comparison of names, `typeof name` and literals, or
//! `name == null` — written after a complex one is reported once per chain
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-simple-condition-first.md`). A name guarded
//! before by `typeof name !== 'undefined'` stays. The simple ones move first, in their order,
//! autofixed only when every complex condition they cross is a conditional expression of simple
//! parts and no comment stands in or around the chain.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, LogicalExpression, LogicalOperator, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{has_comment_in, is_boolean_context, text, unwrapped};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSimpleConditionFirst;

/// Whether an expression is a name or `typeof name`.
fn is_name_or_typeof(expression: &Expression) -> bool {
    match unwrapped(expression) {
        Expression::Identifier(_) => true,
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::Typeof
                && matches!(unwrapped(&unary.argument), Expression::Identifier(_))
        }
        _ => false,
    }
}

/// Whether an expression is a simple operand of a strict comparison: a name, `typeof name`, a
/// literal, or a signed number.
fn is_simple_operand(expression: &Expression) -> bool {
    let expression = unwrapped(expression);
    if is_name_or_typeof(expression) || expression.is_literal() {
        return true;
    }
    let Expression::UnaryExpression(unary) = expression else {
        return false;
    };
    match unwrapped(&unary.argument) {
        Expression::NumericLiteral(_) => {
            matches!(
                unary.operator,
                UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus
            )
        }
        Expression::BigIntLiteral(_) => unary.operator == UnaryOperator::UnaryNegation,
        _ => false,
    }
}

/// Whether a condition is simple.
fn is_simple(expression: &Expression) -> bool {
    match unwrapped(expression) {
        Expression::Identifier(_) => true,
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::LogicalNot && is_simple(&unary.argument)
        }
        Expression::BinaryExpression(binary) => {
            let (left, right) = (unwrapped(&binary.left), unwrapped(&binary.right));
            let loose = matches!(
                binary.operator,
                BinaryOperator::Equality | BinaryOperator::Inequality
            ) && ((matches!(left, Expression::Identifier(_)) && right.is_null())
                || (left.is_null() && matches!(right, Expression::Identifier(_))));
            loose
                || (matches!(
                    binary.operator,
                    BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
                ) && is_simple_operand(left)
                    && is_simple_operand(right)
                    && (is_name_or_typeof(left) || is_name_or_typeof(right)))
        }
        _ => false,
    }
}

/// Whether a condition is `typeof name !== 'undefined'`, either way round.
fn guards_name(expression: &Expression, name: &str) -> bool {
    let Expression::BinaryExpression(binary) = unwrapped(expression) else {
        return false;
    };
    if binary.operator != BinaryOperator::StrictInequality {
        return false;
    }
    let (left, right) = (unwrapped(&binary.left), unwrapped(&binary.right));
    let (check, literal) = if matches!(left, Expression::UnaryExpression(_)) {
        (left, right)
    } else {
        (right, left)
    };
    matches!(check, Expression::UnaryExpression(unary)
        if unary.operator == UnaryOperator::Typeof
            && matches!(unwrapped(&unary.argument), Expression::Identifier(each) if each.name == name))
        && matches!(literal, Expression::StringLiteral(each) if each.value == "undefined")
}

/// Whether a conditional expression is made of simple parts only, and so can move.
fn is_safe_conditional(expression: &Expression) -> bool {
    matches!(unwrapped(expression), Expression::ConditionalExpression(conditional)
        if [&conditional.test, &conditional.consequent, &conditional.alternate]
            .iter()
            .all(|part| is_simple(part) || is_simple_operand(part) || is_safe_conditional(part)))
}

/// The operands of a chain of one operator, through TypeScript's wrappers.
fn operands<'s, 'a>(
    expression: &'s Expression<'a>,
    operator: LogicalOperator,
    into: &mut Vec<&'s Expression<'a>>,
    wrapped: &mut bool,
) {
    let inner = unwrapped(expression);
    match inner {
        Expression::LogicalExpression(logical) if logical.operator == operator => {
            *wrapped |= !std::ptr::eq(inner, expression);
            operands(&logical.left, operator, into, wrapped);
            operands(&logical.right, operator, into, wrapped);
        }
        _ => into.push(expression),
    }
}

/// The text of an operand of the reordered chain.
fn operand_text(source: &Source, operand: &Expression, operator: LogicalOperator) -> String {
    let inner = text(source, operand.span());
    let loose = match operand {
        Expression::LogicalExpression(logical) => {
            logical.operator == LogicalOperator::Coalesce
                || (operator == LogicalOperator::And && logical.operator == LogicalOperator::Or)
        }
        Expression::ConditionalExpression(_)
        | Expression::AssignmentExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::SequenceExpression(_)
        | Expression::YieldExpression(_) => true,
        _ => false,
    };
    if loose {
        format!("({inner})")
    } else {
        inner.to_owned()
    }
}

impl PreferSimpleConditionFirst {
    /// The finding on a chain.
    fn check(&self, source: &Source, logical: &LogicalExpression) -> Option<Finding> {
        let mut list = Vec::new();
        let mut wrapped = false;
        operands(&logical.left, logical.operator, &mut list, &mut wrapped);
        operands(&logical.right, logical.operator, &mut list, &mut wrapped);
        let simple: Vec<bool> = list.iter().map(|each| is_simple(each)).collect();
        let first_complex = simple.iter().position(|each| !each)?;
        let misplaced = (first_complex + 1..list.len()).find(|index| {
            simple[*index]
                && !(logical.operator == LogicalOperator::And
                    && matches!(unwrapped(list[*index]), Expression::Identifier(name)
                        if list[..*index].iter().any(|each| guards_name(each, &name.name))))
        })?;
        let last_simple = simple.iter().rposition(|each| *each).unwrap_or(0);
        let safe = list
            .iter()
            .enumerate()
            .all(|(index, each)| simple[index] || index > last_simple || is_safe_conditional(each));
        let at = list[misplaced].span().start;
        let operator = logical.operator.as_str();
        let message = if safe {
            format!("Prefer this simple condition first in the `{operator}` expression.")
        } else {
            "Consider moving this simple condition first after verifying short-circuit behavior."
                .to_owned()
        };
        let before = source.text[..logical.span.start as usize].trim_end();
        let after = source.text[logical.span.end as usize..].trim_start();
        let commented = has_comment_in(source, logical.span)
            || before.ends_with("*/")
            || after.starts_with("/*")
            || after.starts_with("//");
        if !safe || wrapped || commented {
            return Some(source.finding(self.name(), at, message));
        }
        let reordered: Vec<String> = list
            .iter()
            .zip(&simple)
            .filter(|(_, each)| **each)
            .chain(list.iter().zip(&simple).filter(|(_, each)| !**each))
            .map(|(each, _)| operand_text(source, each, logical.operator))
            .collect();
        let fix = Fix {
            start: logical.span.start,
            end: logical.span.end,
            text: reordered.join(&format!(" {operator} ")),
        };
        Some(source.fixed(self.name(), at, message, fix))
    }
}

impl Rule for PreferSimpleConditionFirst {
    fn name(&self) -> &'static str {
        "unicorn/prefer-simple-condition-first"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::LogicalExpression(logical) = node.kind() else {
            return;
        };
        if logical.operator == LogicalOperator::Coalesce {
            return;
        }
        let nodes = source.semantic.nodes();
        let mut outermost = node.id();
        while matches!(
            nodes.parent_kind(outermost),
            AstKind::TSAsExpression(_)
                | AstKind::TSSatisfiesExpression(_)
                | AstKind::TSNonNullExpression(_)
                | AstKind::TSTypeAssertion(_)
        ) {
            outermost = nodes.parent_id(outermost);
        }
        let same_operator_above = matches!(nodes.parent_kind(outermost),
            AstKind::LogicalExpression(outer) if outer.operator == logical.operator);
        if same_operator_above || !is_boolean_context(source, outermost) {
            return;
        }
        findings.extend(self.check(source, logical));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSimpleConditionFirst;
    use crate::testing::{fixed, lint, messages};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferSimpleConditionFirst, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn moves_simple_conditions_first() {
        // The documentation's cases.
        assert_eq!(
            check("if ((foo ? bar : baz) && ready && enabled);"),
            (
                vec![(1, 26)],
                "if (ready && enabled && (foo ? bar : baz));".to_owned()
            )
        );
        assert_eq!(
            check("if ((foo ? bar : baz) || ready || enabled);").1,
            "if (ready || enabled || (foo ? bar : baz));"
        );
        let unsafe_call = "if (check(foo) && ready);";
        assert_eq!(check(unsafe_call), (vec![(1, 19)], unsafe_call.to_owned()));
        assert_eq!(
            messages(&lint(
                &PreferSimpleConditionFirst,
                "scripts/a.ts",
                unsafe_call
            )),
            ["Consider moving this simple condition first after verifying short-circuit behavior."]
        );
        // `== null`, a signed literal, `typeof`; the chain under `!` or `Boolean()`.
        assert_eq!(
            check("const a = !(f() && b == null && c === -1);").0,
            [(1, 20)]
        );
        assert_eq!(
            check("const a = Boolean(o.p || typeof t === 'string');").0,
            [(1, 26)]
        );
    }

    #[test]
    fn leaves_values_guards_and_ordered_chains() {
        for code in [
            "if (typeof FLAG !== 'undefined' && check() && FLAG);",
            "const value = foo() || fallback;",
            "if (ready && enabled && (foo ? bar : baz));",
            "if (a.b && c.d);",
            "if (a ?? b);",
            "if (f() && a.b === c.d);",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_without_fix_around_comments_or_wrappers() {
        for code in [
            "if ((t ? a : b) /* c */ && ready);",
            "if (((t ? a : b) && ready) as boolean && go);",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }
}
