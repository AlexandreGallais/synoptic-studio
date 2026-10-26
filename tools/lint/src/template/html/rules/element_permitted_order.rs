//! `element-permitted-order` of `html-validate` 11.17.0
//! (`src/rules/element-permitted-order.ts`, REF-HTML-VALIDATE-SOURCE): the first child written
//! after one its parent's order puts later is reported.

use super::super::validator::out_of_order;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ElementPermittedOrder;

impl TemplateRule for ElementPermittedOrder {
    fn name(&self) -> &'static str {
        "html-validate/element-permitted-order"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(rules) = element.meta.and_then(|meta| meta.permitted_order) else {
                continue;
            };
            if let Some((child, previous)) = out_of_order(&doc, &doc.child_elements(index), rules) {
                reports.push(report(
                    doc.name_span(child),
                    format!(
                        "Element <{}> must be used before <{}> in this context",
                        doc.el(child).tag,
                        doc.el(previous).tag
                    ),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementPermittedOrder;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "element-permitted-order.txt",
            "element-permitted-order",
            &|_| Box::new(ElementPermittedOrder),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 1, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
