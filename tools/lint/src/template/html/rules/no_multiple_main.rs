//! `no-multiple-main` of `html-validate` 11.17.0 (`src/rules/no-multiple-main.ts`,
//! REF-HTML-VALIDATE-SOURCE): each `<main>` without `hidden` after the first is reported.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct NoMultipleMain;

impl TemplateRule for NoMultipleMain {
    fn name(&self) -> &'static str {
        "html-validate/no-multiple-main"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let visible = (0..doc.elements.len())
            .filter(|index| doc.el(*index).tag == "main" && !doc.has(*index, "hidden"))
            .skip(1);
        for index in visible {
            reports.push(report(
                doc.name_span(index),
                "Multiple <main> elements present in document".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoMultipleMain;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-multiple-main.txt", "no-multiple-main", &|_| {
            Box::new(NoMultipleMain)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (4, 1, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
