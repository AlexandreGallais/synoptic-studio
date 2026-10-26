//! `wcag/h67` of `html-validate` 11.17.0 (`src/rules/wcag/h67.ts`, REF-HTML-VALIDATE-SOURCE): an
//! `<img>` with a `title` and an empty or missing `alt`, reported at the `title`.

use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct WcagH67;

impl TemplateRule for WcagH67 {
    fn name(&self) -> &'static str {
        "html-validate/wcag/h67"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if element.tag != "img" {
                continue;
            }
            let Some(title) = doc
                .attr(index, "title")
                .filter(|attr| attr.text() != Some(""))
            else {
                continue;
            };
            let alt = doc
                .attr(index, "alt")
                .is_some_and(|attr| match &attr.value {
                    Value::Dynamic => true,
                    Value::Static(text) => !text.is_empty(),
                    Value::Omitted => false,
                });
            if !alt {
                reports.push(report(
                    title.key_span(),
                    "<img> with empty alt text cannot have title attribute".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WcagH67;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("wcag-h67.txt", "wcag/h67", &|_| Box::new(WcagH67));

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (4, 2, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
