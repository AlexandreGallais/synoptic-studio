//! `hidden-focusable` of `html-validate` 11.17.0 (`src/rules/hidden-focusable.ts`,
//! REF-HTML-VALIDATE-SOURCE): a focusable element hidden by `aria-hidden="true"`, its own
//! reported at the attribute, an ancestor's at the element.

use super::super::a11y::{aria_hidden, is_focusable};
use super::super::elements::Cat;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct HiddenFocusable;

impl TemplateRule for HiddenFocusable {
    fn name(&self) -> &'static str {
        "html-validate/hidden-focusable"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let candidate = doc.has(index, "tabindex")
                || element
                    .meta
                    .is_some_and(|meta| !matches!(meta.focusable, Cat::No));
            if !candidate || !is_focusable(&doc, index) || !aria_hidden(&doc, index).any() {
                continue;
            }
            match doc.attr(index, "aria-hidden") {
                Some(attr) => reports.push(report(
                    attr.key_span(),
                    "aria-hidden cannot be used on focusable elements".to_owned(),
                )),
                None => reports.push(report(
                    doc.name_span(index),
                    "aria-hidden cannot be used on focusable elements (hidden by ancestor element)"
                        .to_owned(),
                )),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HiddenFocusable;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("hidden-focusable.txt", "hidden-focusable", &|_| {
            Box::new(HiddenFocusable)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (7, 6, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
