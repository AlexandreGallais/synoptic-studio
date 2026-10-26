//! `@angular-eslint/template/no-distracting-elements`: no `<blink>` nor `<marquee>`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/no-distracting-elements.ts`):
//! each, its name any case, is reported over itself and fixed by its removal.

use super::r3::replace;
use super::{Report, Template, TemplateRule, is_plain_element};

/// The rule.
pub struct NoDistractingElements;

impl TemplateRule for NoDistractingElements {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-distracting-elements"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "noDistractingElements",
            "Do not use <{{element}}> elements as they can create visual accessibility issues and are deprecated",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            let name = element.name.to_lowercase();
            if !is_plain_element(element) || !matches!(name.as_str(), "blink" | "marquee") {
                continue;
            }
            let span = element.span;
            reports.push(Report {
                data: vec![("element", element.name.clone())],
                fix: Some(replace(span.start, span.end, "")),
                ..Report::new(span, "noDistractingElements")
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDistractingElements;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("no-distracting-elements.txt", &[], &|_| {
            Box::new(NoDistractingElements)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 1 valid and 3 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (1, 3, 0));
    }
}
