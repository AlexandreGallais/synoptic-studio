//! `unicorn/no-useless-compound-assignment`: no `+= 0`, `-= 0`, `*= 1`, `/= 1` nor `**= 1`,
//! which change nothing (`eslint-plugin-unicorn` 78.0.0). Its removal is a suggestion in
//! `ESLint`, the value possibly changing type: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{AssignmentOperator, Expression};
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::text;
use crate::source::Source;

/// The rule.
pub struct NoUselessCompoundAssignment;

impl Rule for NoUselessCompoundAssignment {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-compound-assignment"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::AssignmentExpression(assignment) = node.kind() else {
            return;
        };
        let (operator, identity) = match assignment.operator {
            AssignmentOperator::Addition => ("+=", 0.0),
            AssignmentOperator::Subtraction => ("-=", 0.0),
            AssignmentOperator::Multiplication => ("*=", 1.0),
            AssignmentOperator::Division => ("/=", 1.0),
            AssignmentOperator::Exponential => ("**=", 1.0),
            _ => return,
        };
        let Expression::NumericLiteral(literal) = &assignment.right else {
            return;
        };
        if literal.value.to_bits() != f64::to_bits(identity) {
            return;
        }
        findings.push(source.finding(
            self.name(),
            assignment.span.start,
            format!(
                "This compound assignment is useless as `{operator} {}` does not change the value.",
                text(source, literal.span)
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUselessCompoundAssignment;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUselessCompoundAssignment, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_identities() {
        assert_eq!(
            check("let total = getTotal();\ntotal += 0;"),
            ["This compound assignment is useless as `+= 0` does not change the value."]
        );
        assert_eq!(
            check("price *= 1;"),
            ["This compound assignment is useless as `*= 1` does not change the value."]
        );
        assert_eq!(
            check("function f() { return count -= 0; }"),
            ["This compound assignment is useless as `-= 0` does not change the value."]
        );
        assert_eq!(check("a /= 1.0; a **= 1;").len(), 2);
    }

    #[test]
    fn accepts_other_values_and_bitwise_idioms() {
        for code in [
            "x += 1;",
            "x |= 0;",
            "x >>>= 0;",
            "x &= -1;",
            "x %= 1;",
            "x += -0;",
            "x *= 1n;",
            "x = 0;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
