//! `wcag/h32` of `html-validate` 11.17.0 (`src/rules/wcag/h32.ts`, REF-HTML-VALIDATE-SOURCE): a
//! `<form>` without a submit button, inside it or naming it by `form`, reported at its name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct WcagH32;

/// Whether an element is a submit button.
fn submits(doc: &Doc, index: usize) -> bool {
    doc.el(index)
        .meta
        .is_some_and(|meta| meta.submit_button.of(doc, index))
}

impl TemplateRule for WcagH32 {
    fn name(&self) -> &'static str {
        "html-validate/wcag/h32"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if !element.meta.is_some_and(|meta| meta.form.of(&doc, index)) {
                continue;
            }
            let nested = doc
                .descendants(index)
                .into_iter()
                .any(|each| submits(&doc, each) && !doc.has(each, "form"));
            let associated = doc
                .value(index, "id")
                .filter(|id| !id.is_empty())
                .is_some_and(|id| {
                    (0..doc.elements.len()).any(|each| {
                        matches!(doc.el(each).tag.as_str(), "button" | "input")
                            && submits(&doc, each)
                            && doc
                                .attr(each, "form")
                                .is_some_and(|attr| attr.is_dynamic() || attr.text() == Some(id))
                    })
                });
            if !nested && !associated {
                reports.push(report(
                    doc.name_span(index),
                    format!("<{}> element must have a submit button", element.tag),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WcagH32;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("wcag-h32.txt", "wcag/h32", &|_| Box::new(WcagH32));

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 4, 5),
            "{:#?}",
            tally.apart_cases
        );
    }
}
