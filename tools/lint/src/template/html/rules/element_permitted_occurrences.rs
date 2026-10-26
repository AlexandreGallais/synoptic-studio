//! `element-permitted-occurrences` of `html-validate` 11.17.0
//! (`src/rules/element-permitted-occurrences.ts`, REF-HTML-VALIDATE-SOURCE): a child its parent
//! permits once (`legend?`) written again is reported, each one past the first.

use super::super::validator::over_limit;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ElementPermittedOccurrences;

impl TemplateRule for ElementPermittedOccurrences {
    fn name(&self) -> &'static str {
        "html-validate/element-permitted-occurrences"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(rules) = element.meta.and_then(|meta| meta.permitted_content) else {
                continue;
            };
            for (child, category) in over_limit(&doc, &doc.child_elements(index), rules) {
                reports.push(report(
                    doc.name_span(child),
                    format!(
                        "Element <{category}> can only appear once under <{}>",
                        element.tag
                    ),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementPermittedOccurrences;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "element-permitted-occurrences.txt",
            "element-permitted-occurrences",
            &|_| Box::new(ElementPermittedOccurrences),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 1, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
