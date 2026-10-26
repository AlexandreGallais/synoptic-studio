//! `@angular-eslint/template/prefer-control-flow`: `@if`, `@for` and `@switch` rather than the
//! `ngIf`, `ngForOf` and `ngSwitch` directives (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-control-flow.ts`): each bound attribute of those names, an input or a
//! binding of a structural directive's microsyntax, is reported over its source span.

use super::{Report, Template, TemplateRule, bound_attributes};

/// The rule.
pub struct PreferControlFlow;

/// The directives the built-in control flow replaces.
const DIRECTIVES: [&str; 3] = ["ngForOf", "ngIf", "ngSwitch"];

impl TemplateRule for PreferControlFlow {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-control-flow"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferControlFlow",
            "Use built-in control flow instead of directive {{name}}.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            for each in bound_attributes(template.allocator, element) {
                if DIRECTIVES.contains(&each.name.as_str()) {
                    reports.push(Report {
                        data: vec![("name", each.name.clone())],
                        ..Report::new(each.span, "preferControlFlow")
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferControlFlow;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-control-flow.txt", &[], &|_| {
            Box::new(PreferControlFlow)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 5 valid and 10 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (5, 10, 0));
    }
}
