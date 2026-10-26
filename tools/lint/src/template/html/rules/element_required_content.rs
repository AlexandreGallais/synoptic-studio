//! `element-required-content` of `html-validate` 11.17.0
//! (`src/rules/element-required-content.ts`, REF-HTML-VALIDATE-SOURCE): an element without a
//! child its metadata requires, once for each missing.

use super::super::validator::missing_content;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ElementRequiredContent;

impl TemplateRule for ElementRequiredContent {
    fn name(&self) -> &'static str {
        "html-validate/element-required-content"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(rules) = element.meta.and_then(|meta| meta.required_content) else {
                continue;
            };
            for missing in missing_content(&doc, index, rules) {
                let tag = missing.strip_prefix('@').map_or_else(
                    || format!("<{missing}>"),
                    |category| format!("{category} element"),
                );
                reports.push(report(
                    doc.name_span(index),
                    format!("<{}> element must have {tag} as content", element.tag),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementRequiredContent;
    use crate::template::html::cases::{messages, run};

    #[test]
    fn reports_missing_required_content() {
        // Our own cases: `<details>` needs its `<summary>`, `<hgroup>` a heading; a `<summary>`
        // under an `@if` counts, the block read through.
        assert_eq!(
            messages(
                &ElementRequiredContent,
                "<details></details><hgroup><p></p></hgroup><details>@if (a) {<summary></summary>}</details>"
            ),
            [
                "1:2 <details> element must have <summary> as content",
                "1:21 <hgroup> element must have heading element as content",
            ]
        );
    }

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "element-required-content.txt",
            "element-required-content",
            &|_| Box::new(ElementRequiredContent),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (0, 0, 9),
            "{:#?}",
            tally.apart_cases
        );
    }
}
