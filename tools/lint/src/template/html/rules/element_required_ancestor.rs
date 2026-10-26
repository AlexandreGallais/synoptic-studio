//! `element-required-ancestor` of `html-validate` 11.17.0
//! (`src/rules/element-required-ancestor.ts`, REF-HTML-VALIDATE-SOURCE): an element without one
//! of the ancestors its metadata requires. Inside an `<ng-template>`, whose content renders where
//! it is instantiated, the ancestors are not known: such an element is not checked (our own
//! reading of Angular's templates).

use regex::Regex;

use super::super::elements::natural_join;
use super::super::validator::has_ancestors;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ElementRequiredAncestor;

impl TemplateRule for ElementRequiredAncestor {
    fn name(&self) -> &'static str {
        "html-validate/element-required-ancestor"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let tag_only = Regex::new(r"(?i)^[\dA-Z-]+$").ok();
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(rules) = element.meta.and_then(|meta| meta.required_ancestors) else {
                continue;
            };
            if element.in_template || has_ancestors(&doc, index, rules) {
                continue;
            }
            let named: Vec<String> = rules
                .iter()
                .map(|rule| {
                    if tag_only.as_ref().is_some_and(|regex| regex.is_match(rule)) {
                        format!("<{rule}>")
                    } else {
                        format!("\"{rule}\"")
                    }
                })
                .collect();
            reports.push(report(
                doc.name_span(index),
                format!(
                    "<{}> element requires a {} ancestor",
                    element.tag,
                    natural_join(&named)
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementRequiredAncestor;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "element-required-ancestor.txt",
            "element-required-ancestor",
            &|_| Box::new(ElementRequiredAncestor),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (1, 0, 4),
            "{:#?}",
            tally.apart_cases
        );
    }
}
