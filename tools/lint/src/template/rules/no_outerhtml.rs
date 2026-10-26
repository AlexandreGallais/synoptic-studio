//! `@angular-eslint/template/no-outerhtml`: no `outerHTML` on a DOM element, bound or plain,
//! which replaces the very node Angular updates (`@angular-eslint/eslint-plugin-template`
//! 22.6.0, `src/rules/no-outerhtml.ts`); a component's input of that name is left alone.
//! Reported over the name bound.

use super::dom::is_dom_element;
use super::{Report, Role, Template, TemplateRule, bound3, key_span};

/// The rule.
pub struct NoOuterhtml;

impl TemplateRule for NoOuterhtml {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-outerhtml"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "noOuterHtml",
            "Do not use `outerHTML`. Binding to `[outerHTML]` throws on update because setting `outerHTML` replaces the host element itself, so Angular loses its reference to the node (\"This element has no parent node\"); a static `outerHTML` attribute is not a real DOM attribute and has no effect. Use `[innerHTML]` (being mindful of sanitization) or restructure the template instead.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !is_dom_element(&element.name) {
                continue;
            }
            for each in bound3(element) {
                if each.role != Role::Output && each.name.eq_ignore_ascii_case("outerhtml") {
                    reports.push(Report::new(key_span(each.attribute), "noOuterHtml"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoOuterhtml;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-outerhtml.txt", &[], &|_| Box::new(NoOuterhtml));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 4 valid and 4 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (4, 4, 0));
    }
}
