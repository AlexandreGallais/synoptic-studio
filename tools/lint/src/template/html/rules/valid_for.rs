//! `valid-for` of `html-validate` 11.17.0 (`src/rules/valid-for.ts`, REF-HTML-VALIDATE-SOURCE): a
//! `<label for>` naming an element of the template that is not labelable, reported at its value;
//! an element of another component's template is not seen, so not checked.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ValidFor;

impl TemplateRule for ValidFor {
    fn name(&self) -> &'static str {
        "html-validate/valid-for"
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
            let Some(attr) = doc.attr(index, "for") else {
                continue;
            };
            let Some(id) = attr.text().filter(|id| !id.is_empty()) else {
                continue;
            };
            let Some(target) =
                (0..doc.elements.len()).find(|each| doc.value(*each, "id") == Some(id))
            else {
                continue;
            };
            let labelable = doc
                .el(target)
                .meta
                .is_none_or(|meta| meta.labelable.of(&doc, target));
            if !labelable {
                reports.push(report(
                    attr.value_span(),
                    "<label> \"for\" attribute must reference a labelable form control".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ValidFor;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("valid-for.txt", "valid-for", &|_| Box::new(ValidFor));

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (8, 3, 7),
            "{:#?}",
            tally.apart_cases
        );
    }
}
