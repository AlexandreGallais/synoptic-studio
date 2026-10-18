//! `unicorn/prefer-location-assign`: navigating by `location.href = url` is
//! `location.assign(url)` (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-location-assign.md`).
//! Autofixed for a plain assignment as a statement to `location`, `window.location` or
//! `globalThis.location`, no comment inside; through a `const` alias, reported only.

use oxc_ast::AstKind;
use oxc_ast::ast::{AssignmentOperator, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::has_comment_in;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{
    const_init, is_value_unused, parenthesized_text, symbol_of, text,
};
use crate::source::Source;

/// The rule.
pub struct PreferLocationAssign;

/// Whether an expression is the global location: `location`, `window.location` or
/// `globalThis.location`.
fn is_direct_location(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(name) => {
            name.name == "location" && symbol_of(source, name).is_none()
        }
        Expression::StaticMemberExpression(member) => {
            member.property.name == "location"
                && matches!(&member.object, Expression::Identifier(object)
                    if matches!(object.name.as_str(), "window" | "globalThis") && symbol_of(source, object).is_none())
        }
        _ => false,
    }
}

impl Rule for PreferLocationAssign {
    fn name(&self) -> &'static str {
        "unicorn/prefer-location-assign"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::AssignmentExpression(assignment) = node.kind() else {
            return;
        };
        let Some(member) = assignment.left.as_member_expression() else {
            return;
        };
        let href = match member {
            oxc_ast::ast::MemberExpression::StaticMemberExpression(member) => {
                (member.property.name == "href").then_some(member.property.span.start)
            }
            oxc_ast::ast::MemberExpression::ComputedMemberExpression(member) => {
                (crate::rules::unicorn::dom::nodes::static_string(&member.expression).as_deref()
                    == Some("href"))
                .then(|| member.expression.span().start)
            }
            oxc_ast::ast::MemberExpression::PrivateFieldExpression(_) => None,
        };
        let object = member.object();
        let direct = is_direct_location(source, object);
        let alias = const_init(source, object).is_some_and(|init| is_direct_location(source, init));
        let Some(start) = href.filter(|_| direct || alias) else {
            return;
        };
        let message = "Prefer `Location#assign()` over assigning to `Location#href`.".to_owned();
        let nodes = source.semantic.nodes();
        let commented = match nodes.parent_kind(node.id()) {
            AstKind::ExpressionStatement(statement) => has_comment_in(source, statement.span),
            _ => has_comment_in(source, assignment.span),
        };
        if assignment.operator != AssignmentOperator::Assign
            || !is_value_unused(source, node.id())
            || !direct
            || commented
        {
            findings.push(source.finding(self.name(), start, message));
            return;
        }
        let value = parenthesized_text(source, &assignment.right);
        let fix = Fix {
            start: assignment.span.start,
            end: assignment.span.end,
            text: format!("{}.assign({value})", text(source, object.span())),
        };
        findings.push(source.fixed(self.name(), start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferLocationAssign;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferLocationAssign, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn navigates_with_assign() {
        // The documentation's case, then `window.`, a computed name, an alias, `+=`, a used value.
        assert_eq!(
            check("location.href = url;\nwindow.location['href'] = (a, b);\nconst l = location;\nl.href = u;\nlocation.href += '#x';\nconst v = (location.href = u);"),
            (
                vec![(1, 10), (2, 17), (4, 3), (5, 10), (6, 21)],
                "location.assign(url);\nwindow.location.assign((a, b));\nconst l = location;\nl.href = u;\nlocation.href += '#x';\nconst v = (location.href = u);"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_other_locations() {
        for code in [
            "location.assign(url);",
            "location.replace('/login');",
            "button.onclick = () => location.assign('/next-page');",
            "other.href = url;",
            "function f(location) { location.href = url; }",
            "const h = location.href;",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
