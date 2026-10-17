//! `unicorn/no-useless-logical-operand`: a boolean literal that makes an operand of an `&&` or
//! `||` chain useless — a leading `false &&` or `true ||` that decides the whole chain, or a
//! `true` in `&&` and a `false` in `||` that change nothing (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-useless-logical-operand.md`). A last `&& true` or `|| false` is useless only
//! when the chain is read for its truth or its other operands are booleans; an absorbing literal
//! after other operands is left to `no-constant-binary-expression`. Autofixed when the chain
//! holds no comment.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, LogicalExpression, LogicalOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{
    has_comment_in, is_boolean, is_boolean_context, is_parenthesized, needs_parentheses_as_operand,
    parent, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUselessLogicalOperand;

/// The operands of a chain of one operator, flattened from left to right.
fn operands<'s, 'a>(
    expression: &'s Expression<'a>,
    operator: LogicalOperator,
    into: &mut Vec<&'s Expression<'a>>,
) {
    match expression {
        Expression::LogicalExpression(logical) if logical.operator == operator => {
            operands(&logical.left, operator, into);
            operands(&logical.right, operator, into);
        }
        other => into.push(other),
    }
}

/// Whether an operand is the boolean literal of a value, through TypeScript's wrappers.
fn is_literal(operand: &Expression, value: bool) -> bool {
    matches!(operand.get_inner_expression(), Expression::BooleanLiteral(literal) if literal.value == value)
}

/// Whether an operand needs parentheses inside a chain of an operator.
fn needs_parentheses_in_chain(operand: &Expression, operator: LogicalOperator) -> bool {
    match operand {
        Expression::LogicalExpression(inner) => {
            inner.operator == LogicalOperator::Coalesce
                || (operator == LogicalOperator::And && inner.operator == LogicalOperator::Or)
        }
        Expression::ConditionalExpression(_)
        | Expression::AssignmentExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::SequenceExpression(_)
        | Expression::YieldExpression(_) => true,
        _ => false,
    }
}

/// Whether a chain's place takes any expression without parentheses.
fn takes_any(source: &Source, node: &AstNode) -> bool {
    matches!(
        parent(source, node.id()),
        AstKind::ExpressionStatement(_)
            | AstKind::VariableDeclarator(_)
            | AstKind::IfStatement(_)
            | AstKind::WhileStatement(_)
            | AstKind::ReturnStatement(_)
            | AstKind::CallExpression(_)
            | AstKind::NewExpression(_)
            | AstKind::ArrayExpression(_)
            | AstKind::ObjectProperty(_)
    )
}

impl NoUselessLogicalOperand {
    /// The finding on a chain, the operand reported and the operands kept.
    fn problem<'s, 'a>(
        source: &Source,
        node: &AstNode,
        logical: &'s LogicalExpression<'a>,
        all: &[&'s Expression<'a>],
    ) -> Option<(&'s Expression<'a>, Vec<&'s Expression<'a>>)> {
        let identity = logical.operator == LogicalOperator::And;
        if is_literal(all[0], !identity) {
            return Some((all[0], vec![all[0]]));
        }
        let last = all.len() - 1;
        let removable: Vec<usize> = (0..all.len())
            .filter(|index| {
                is_literal(all[*index], identity)
                    && (*index < last
                        || is_boolean_context(source, node.id())
                        || all[..last].iter().all(|each| is_boolean(source, each)))
            })
            .collect();
        let first = *removable.first()?;
        let kept: Vec<&Expression> = (0..all.len())
            .filter(|index| !removable.contains(index))
            .map(|index| all[index])
            .collect();
        let kept = if kept.is_empty() {
            vec![all[*removable.last()?]]
        } else {
            kept
        };
        Some((all[first], kept))
    }
}

impl Rule for NoUselessLogicalOperand {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-logical-operand"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::LogicalExpression(logical) = node.kind() else {
            return;
        };
        let outermost = !matches!(parent(source, node.id()),
            AstKind::LogicalExpression(outer) if outer.operator == logical.operator);
        if logical.operator == LogicalOperator::Coalesce || !outermost {
            return;
        }
        let mut all = Vec::new();
        operands(&logical.left, logical.operator, &mut all);
        operands(&logical.right, logical.operator, &mut all);
        let Some((target, kept)) = Self::problem(source, node, logical, &all) else {
            return;
        };
        let operator = logical.operator.as_str();
        let message = format!("Simplify this `{operator}` expression.");
        if has_comment_in(source, logical.span) {
            findings.push(source.finding(self.name(), target.span().start, message));
            return;
        }
        let replacement = if let [only] = kept.as_slice() {
            let inner = text(source, only.span());
            if needs_parentheses_as_operand(only)
                && !takes_any(source, node)
                && !is_parenthesized(source, logical.span)
            {
                format!("({inner})")
            } else {
                inner.to_owned()
            }
        } else {
            kept.iter()
                .enumerate()
                .map(|(index, each)| {
                    let inner = text(source, each.span());
                    let leading = index == 0
                        && matches!(
                            each,
                            Expression::FunctionExpression(_)
                                | Expression::ClassExpression(_)
                                | Expression::ObjectExpression(_)
                        );
                    if leading || needs_parentheses_in_chain(each, logical.operator) {
                        format!("({inner})")
                    } else {
                        inner.to_owned()
                    }
                })
                .collect::<Vec<String>>()
                .join(&format!(" {operator} "))
        };
        let fix = Fix {
            start: logical.span.start,
            end: logical.span.end,
            text: replacement,
        };
        findings.push(source.fixed(self.name(), target.span().start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUselessLogicalOperand;
    use crate::testing::{fixed, lint, messages};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoUselessLogicalOperand, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn removes_identity_operands_and_keeps_an_absorbing_lead() {
        // The documentation's cases.
        assert_eq!(
            check("const value = true && input;\nconst other = input && true && fallback;"),
            (
                vec![(1, 15), (2, 24)],
                "const value = input;\nconst other = input && fallback;".to_owned()
            )
        );
        assert_eq!(
            check("if (input && true) {}\nconst value = Boolean(input || false);"),
            (
                vec![(1, 14), (2, 32)],
                "if (input) {}\nconst value = Boolean(input);".to_owned()
            )
        );
        assert_eq!(
            check("const value = false && input;\nconst other = true || input;"),
            (
                vec![(1, 15), (2, 15)],
                "const value = false;\nconst other = true;".to_owned()
            )
        );
        assert_eq!(
            messages(&lint(
                &NoUselessLogicalOperand,
                "scripts/a.ts",
                "a || false || b;"
            )),
            ["Simplify this `||` expression."]
        );
    }

    #[test]
    fn keeps_values_that_would_change_and_parenthesizes_what_needs_it() {
        for code in [
            "const value = input && true;",
            "const other = input || false;",
            "const alwaysFalse = input && false;",
            "const alwaysTrue = input || true;",
            "const a = b ?? true;",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        // The other operands are booleans: the last `&& true` goes.
        assert_eq!(check("const a = b > 1 && true;").1, "const a = b > 1;");
        assert_eq!(
            check("const a = (true && (b || c)) + 1;").1,
            "const a = (b || c) + 1;"
        );
        assert_eq!(
            check("const a = true && (b || c) && d;").1,
            "const a = (b || c) && d;"
        );
        assert_eq!(check("const a = true && true;").1, "const a = true;");
        let commented = "const a = true /* x */ && b;";
        assert_eq!(check(commented), (vec![(1, 11)], commented.to_owned()));
    }
}
