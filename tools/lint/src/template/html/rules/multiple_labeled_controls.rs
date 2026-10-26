//! `multiple-labeled-controls` of `html-validate` 11.17.0
//! (`src/rules/multiple-labeled-controls.ts`, REF-HTML-VALIDATE-SOURCE): a `<label>` wrapping two
//! labelable controls, or one and naming another by `for`, reported at its name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct MultipleLabeledControls;

impl TemplateRule for MultipleLabeledControls {
    fn name(&self) -> &'static str {
        "html-validate/multiple-labeled-controls"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if element.tag != "label" {
                continue;
            }
            let controls: Vec<Option<&str>> = doc
                .descendants(index)
                .into_iter()
                .filter(|each| {
                    doc.el(*each)
                        .meta
                        .is_some_and(|meta| meta.labelable.of(&doc, *each))
                })
                .map(|each| doc.value(each, "id"))
                .collect();
            let named = doc.value(index, "for").filter(|id| !id.is_empty());
            let count = match named {
                Some(id) if !controls.contains(&Some(id)) => controls.len() + 1,
                _ => controls.len(),
            };
            if count > 1 {
                reports.push(report(
                    doc.name_span(index),
                    "<label> is associated with multiple controls".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MultipleLabeledControls;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "multiple-labeled-controls.txt",
            "multiple-labeled-controls",
            &|_| Box::new(MultipleLabeledControls),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 2, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
