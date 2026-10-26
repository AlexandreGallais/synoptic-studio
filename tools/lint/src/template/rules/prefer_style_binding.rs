//! `@angular-eslint/template/prefer-style-binding`: `[style]` and `[style.x]` rather than
//! `[ngStyle]` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-style-binding.ts`), each bound `ngStyle` reported over its source span.

use super::{Report, Template, TemplateRule, bound_attributes};

/// The rule.
pub struct PreferStyleBinding;

impl TemplateRule for PreferStyleBinding {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-style-binding"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferStyleBinding",
            "Consider using [style] or [style.property] bindings instead of [ngStyle] where applicable.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            for each in bound_attributes(template.allocator, element) {
                if each.name == "ngStyle" {
                    reports.push(Report::new(each.span, "preferStyleBinding"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferStyleBinding;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-style-binding.txt", &[], &|_| {
            Box::new(PreferStyleBinding)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 14 valid and 10 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (14, 10, 0));
    }
}
