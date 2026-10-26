//! `no-implicit-input-type` of `html-validate` 11.17.0 (`src/rules/no-implicit-input-type.ts`,
//! REF-HTML-VALIDATE-SOURCE): an `<input>` without `type`, written or bound, reported at its name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct NoImplicitInputType;

impl TemplateRule for NoImplicitInputType {
    fn name(&self) -> &'static str {
        "html-validate/no-implicit-input-type"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if element.tag == "input" && !doc.has(index, "type") {
                reports.push(report(
                    doc.name_span(index),
                    "<input> is missing recommended \"type\" attribute".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoImplicitInputType;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "no-implicit-input-type.txt",
            "no-implicit-input-type",
            &|_| Box::new(NoImplicitInputType),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (4, 1, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
