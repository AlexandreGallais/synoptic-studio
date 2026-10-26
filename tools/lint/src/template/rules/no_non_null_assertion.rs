//! `@angular-eslint/template/no-non-null-assertion`: no `!` asserting a value is not null in a
//! template (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/no-non-null-assertion.ts`),
//! reported over the asserted expression and its `!`.

use oxc_ast::AstKind;

use super::{Report, Template, TemplateRule, each_node, placed};

/// The rule.
pub struct NoNonNullAssertion;

impl TemplateRule for NoNonNullAssertion {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-non-null-assertion"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "noNonNullAssertion",
            "Avoid using the non-null assertion operator (!) in templates. This bypasses type safety and can lead to runtime errors.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for written in template.expressions() {
            each_node(&written, &mut |node, bound| {
                if let AstKind::TSNonNullExpression(assertion) = node {
                    reports.push(Report::new(
                        placed(bound, assertion.span),
                        "noNonNullAssertion",
                    ));
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoNonNullAssertion;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-non-null-assertion.txt", &[], &|_| {
            Box::new(NoNonNullAssertion)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 35 valid and 46 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (35, 46, 0));
    }
}
