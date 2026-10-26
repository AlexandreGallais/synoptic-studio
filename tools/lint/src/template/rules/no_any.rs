//! `@angular-eslint/template/no-any`: no `$any(…)` cast in a template, called on the component
//! (`$any(x)`, `this.$any(x)`), never on another object (`@angular-eslint/eslint-plugin-template`
//! 22.6.0, `src/rules/no-any.ts`), reported over the call. The plugin's removal of the cast is
//! a suggestion, not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;

use super::{Report, Template, TemplateRule, each_node, placed};

/// The rule.
pub struct NoAny;

/// Whether a callee is the component's `$any`.
fn is_any(callee: &Expression) -> bool {
    match callee {
        Expression::Identifier(name) => name.name == "$any",
        Expression::StaticMemberExpression(member) => {
            member.property.name == "$any"
                && matches!(member.object, Expression::ThisExpression(_))
                && !member.optional
        }
        _ => false,
    }
}

impl TemplateRule for NoAny {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-any"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("noAny", "Avoid using \"$any\" in templates"),
            ("suggestRemoveAny", "Remove $any"),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for written in template.expressions() {
            each_node(&written, &mut |node, bound| {
                // A `Call`, never a `SafeCall` (`$any?.(x)`).
                if let AstKind::CallExpression(call) = node
                    && !call.optional
                    && is_any(&call.callee)
                {
                    reports.push(Report::new(placed(bound, call.span), "noAny"));
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAny;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-any.txt", &[], &|_| Box::new(NoAny));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 12 valid and 8 invalid cases, all run; their 10 suggestions, not carried, unchecked.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (12, 8, 10));
    }
}
