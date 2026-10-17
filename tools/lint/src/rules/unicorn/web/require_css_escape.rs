//! `unicorn/require-css-escape`: a value interpolated into an attribute selector of
//! `querySelector(…)`, `querySelectorAll(…)`, `closest(…)` or `matches(…)` goes through
//! `CSS.escape(…)`, or a quote or bracket in it breaks the selector (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/require-css-escape.md`, its default `checkAllSelectors: false`, as
//! configured here). Autofixed.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, TemplateLiteral};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::dom::nodes::is_not_dom_node;
use crate::rules::unicorn::syntax::method_call;
use crate::source::Source;

/// The rule.
pub struct RequireCssEscape;

/// The text of a template's pieces, cooked.
fn cooked(template: &TemplateLiteral, range: std::ops::Range<usize>) -> String {
    template.quasis[range]
        .iter()
        .filter_map(|quasi| quasi.value.cooked.map(|text| text.to_string()))
        .collect()
}

/// Whether the expression at an index sits inside an attribute selector: a `[` before it left
/// open, and a `]` after it.
fn is_in_attribute(template: &TemplateLiteral, index: usize) -> bool {
    let before = cooked(template, 0..index + 1);
    let open = before
        .rfind('[')
        .is_some_and(|open| before.rfind(']').is_none_or(|close| open > close));
    open && template.quasis[index + 1..]
        .iter()
        .any(|quasi| quasi.value.cooked.is_some_and(|text| text.contains(']')))
}

/// Whether an expression is already `CSS.escape(…)`.
fn is_escaped(expression: &Expression) -> bool {
    let Expression::CallExpression(call) = expression else {
        return false;
    };
    method_call(call).is_some_and(|method| {
        method.is_plain()
            && method.name == "escape"
            && call.arguments.len() == 1
            && !call.arguments[0].is_spread()
            && matches!(method.object, Expression::Identifier(object) if object.name == "CSS")
    })
}

impl Rule for RequireCssEscape {
    fn name(&self) -> &'static str {
        "unicorn/require-css-escape"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call) else {
            return;
        };
        if !matches!(
            method.name,
            "closest" | "matches" | "querySelector" | "querySelectorAll"
        ) || is_not_dom_node(method.object)
        {
            return;
        }
        let Some(Expression::TemplateLiteral(template)) = call
            .arguments
            .first()
            .and_then(|first| first.as_expression())
        else {
            return;
        };
        for (index, expression) in template.expressions.iter().enumerate() {
            if is_escaped(expression) || !is_in_attribute(template, index) {
                continue;
            }
            let span = expression.span();
            let inner = &source.text[span.start as usize..span.end as usize];
            let text = if matches!(expression, Expression::SequenceExpression(_)) {
                format!("CSS.escape(({inner}))")
            } else {
                format!("CSS.escape({inner})")
            };
            let fix = Fix {
                start: span.start,
                end: span.end,
                text,
            };
            findings.push(source.fixed(
                self.name(),
                span.start,
                "Use `CSS.escape()` for interpolated values in CSS selectors.".to_owned(),
                fix,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireCssEscape;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&RequireCssEscape, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn escapes_values_in_attribute_selectors() {
        // The documentation's two cases, then a sequence, its written parentheses kept around the
        // call as ESLint keeps them, and two values in one selector.
        assert_eq!(
            check("document.querySelector(`[data-id=\"${id}\"]`);\nelement.querySelectorAll(`a[href^=\"#${hash}\"]`);\nel.closest(`[a=\"${(x, y)}\"]`);\nel.matches(`[a=\"${b}${CSS.escape(c)}${d}\"]`);"),
            (
                vec![(1, 37), (2, 39), (3, 20), (4, 19), (4, 39)],
                "document.querySelector(`[data-id=\"${CSS.escape(id)}\"]`);\nelement.querySelectorAll(`a[href^=\"#${CSS.escape(hash)}\"]`);\nel.closest(`[a=\"${(CSS.escape((x, y)))}\"]`);\nel.matches(`[a=\"${CSS.escape(b)}${CSS.escape(c)}${CSS.escape(d)}\"]`);"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_escaped_values_and_other_selectors() {
        for code in [
            "document.querySelector(`[data-id=\"${CSS.escape(id)}\"]`);",
            "document.querySelector(cssEscape`#${id}`);",
            "document.querySelector(`#${id}`);",
            "document.querySelector(`[a] ${b}`);",
            "document.querySelector(`[a=${b}`);",
            "document.querySelector('[a]');",
            "document.getElementById(`[a=${b}]`);",
            "'x'.matches(`[a=${b}]`);",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
