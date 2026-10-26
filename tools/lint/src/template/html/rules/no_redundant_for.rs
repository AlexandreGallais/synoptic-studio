//! `no-redundant-for` of `html-validate` 11.17.0 (`src/rules/no-redundant-for.ts`,
//! REF-HTML-VALIDATE-SOURCE): a `<label for>` naming a control it wraps is reported at `for`. The
//! rule's fixes are suggestions, not carried.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct NoRedundantFor;

impl TemplateRule for NoRedundantFor {
    fn name(&self) -> &'static str {
        "html-validate/no-redundant-for"
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
            let wraps = doc
                .descendants(index)
                .into_iter()
                .any(|each| doc.value(each, "id") == Some(id));
            if wraps {
                reports.push(report(
                    attr.key_span(),
                    "Redundant \"for\" attribute".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoRedundantFor;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-redundant-for.txt", "no-redundant-for", &|_| {
            Box::new(NoRedundantFor)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 2, 4),
            "{:#?}",
            tally.apart_cases
        );
    }
}
