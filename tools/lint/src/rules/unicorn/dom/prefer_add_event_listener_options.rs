//! `unicorn/prefer-add-event-listener-options`: `addEventListener(type, listener, true)` says
//! `{capture: true}` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-add-event-listener-options.md`). Autofixed.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::nodes::plain_method_call;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferAddEventListenerOptions;

impl Rule for PreferAddEventListenerOptions {
    fn name(&self) -> &'static str {
        "unicorn/prefer-add-event-listener-options"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        if plain_method_call(call, &["addEventListener"], 3).is_none() {
            return;
        }
        let Some(Expression::BooleanLiteral(capture)) = call.arguments[2].as_expression() else {
            return;
        };
        let replacement = format!("{{capture: {}}}", capture.value);
        let message = format!("Prefer `{replacement}` over `{}`.", capture.value);
        let fix = Fix {
            start: capture.span.start,
            end: capture.span.end,
            text: replacement,
        };
        findings.push(source.fixed(self.name(), capture.span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferAddEventListenerOptions;
    use crate::testing::{fixed, lint, messages};

    #[test]
    fn writes_the_options_object() {
        // The documentation's two cases.
        let code = "element.addEventListener('click', handleClick, true);\nelement.addEventListener('scroll', handleScroll, false);";
        let findings = lint(&PreferAddEventListenerOptions, "playground/a/b.ts", code);
        assert_eq!(
            messages(&findings),
            [
                "Prefer `{capture: true}` over `true`.",
                "Prefer `{capture: false}` over `false`.",
            ]
        );
        assert_eq!(
            fixed(code, &findings),
            "element.addEventListener('click', handleClick, {capture: true});\nelement.addEventListener('scroll', handleScroll, {capture: false});"
        );
    }

    #[test]
    fn leaves_options_objects_and_other_calls() {
        for code in [
            "element.addEventListener('touchmove', handleScroll, {passive: true, capture: true});",
            "element.addEventListener('load', initializeOnce, {once: true});",
            "element?.addEventListener('click', f, true);",
            "element.addEventListener('click', f, true, x);",
            "element.removeEventListener('click', f, true);",
            "element.addEventListener('click', f, capture);",
        ] {
            assert!(
                lint(&PreferAddEventListenerOptions, "playground/a/b.ts", code).is_empty(),
                "{code}"
            );
        }
    }
}
