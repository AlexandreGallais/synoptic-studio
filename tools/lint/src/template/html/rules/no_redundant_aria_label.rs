//! `no-redundant-aria-label` of `html-validate` 11.17.0
//! (`src/rules/no-redundant-aria-label.ts`, REF-HTML-VALIDATE-SOURCE): a field's `aria-label`
//! repeating the text of the `<label for>` naming it, reported at the attribute's name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::javascript::JsText;
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct NoRedundantAriaLabel;

impl TemplateRule for NoRedundantAriaLabel {
    fn name(&self) -> &'static str {
        "html-validate/no-redundant-aria-label"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if !matches!(element.tag.as_str(), "input" | "textarea" | "select") {
                continue;
            }
            let (Some(attr), Some(id)) = (doc.attr(index, "aria-label"), doc.value(index, "id"))
            else {
                continue;
            };
            if id.is_empty() {
                continue;
            }
            let label = (0..doc.elements.len())
                .find(|each| doc.el(*each).tag == "label" && doc.value(*each, "for") == Some(id));
            let Some(label) = label else {
                continue;
            };
            let (text, _) = doc.text_content(label);
            if attr.text() == Some(text.js_trim()) {
                reports.push(report(
                    attr.key_span(),
                    "aria-label is redundant when label containing same text exists".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoRedundantAriaLabel;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "no-redundant-aria-label.txt",
            "no-redundant-aria-label",
            &|_| Box::new(NoRedundantAriaLabel),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 3, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
