//! `element-required-attributes` of `html-validate` 11.17.0
//! (`src/rules/element-required-attributes.ts`, REF-HTML-VALIDATE-SOURCE): an element without an
//! attribute its metadata requires, bound or written, reported at its name.

use super::super::elements::Required;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule, interpolate};

/// The rule.
pub struct ElementRequiredAttributes;

impl TemplateRule for ElementRequiredAttributes {
    fn name(&self) -> &'static str {
        "html-validate/element-required-attributes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(meta) = element.meta else {
                continue;
            };
            for attribute in meta.attributes {
                let message = match attribute.required {
                    Required::No => continue,
                    Required::Yes => "{{ tagName }} is missing required \"{{ attr }}\" attribute",
                    Required::When(function) => match function(&doc, index) {
                        Some(message) => message,
                        None => continue,
                    },
                };
                if doc.has(index, attribute.name) {
                    continue;
                }
                let data = [
                    ("tagName", format!("<{}>", element.tag)),
                    ("attr", attribute.name.to_owned()),
                ];
                reports.push(report(doc.name_span(index), interpolate(message, &data)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementRequiredAttributes;
    use crate::template::html::cases::{messages, run};

    #[test]
    fn reports_a_missing_required_attribute() {
        // Our own cases: `<img>` needs `src`, bound or written; `<link>` an `href` or an
        // `imagesrcset`.
        assert_eq!(
            messages(&ElementRequiredAttributes, "<img><img [src]=\"s\"><link>"),
            [
                "1:2 <img> is missing required \"src\" attribute",
                "1:22 <link> is missing required \"href\" or \"imagesrcset\" attribute",
            ]
        );
    }

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "element-required-attributes.txt",
            "element-required-attributes",
            &|_| Box::new(ElementRequiredAttributes),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 1, 4),
            "{:#?}",
            tally.apart_cases
        );
    }
}
