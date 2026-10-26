//! `wcag/h71` of `html-validate` 11.17.0 (`src/rules/wcag/h71.ts`, REF-HTML-VALIDATE-SOURCE): a
//! `<fieldset>` without a `<legend>` child, reported at its name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct WcagH71;

impl TemplateRule for WcagH71 {
    fn name(&self) -> &'static str {
        "html-validate/wcag/h71"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let legend = doc
                .child_elements(index)
                .into_iter()
                .any(|each| doc.el(each).tag == "legend");
            if element.tag == "fieldset" && !legend {
                reports.push(report(
                    doc.name_span(index),
                    "<fieldset> must have a <legend> as the first child".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WcagH71;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("wcag-h71.txt", "wcag/h71", &|_| Box::new(WcagH71));

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 1, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
