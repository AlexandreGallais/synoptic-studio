//! `no-abstract-role` of `html-validate` 11.17.0 (`src/rules/no-abstract-role.ts`,
//! REF-HTML-VALIDATE-SOURCE): each abstract WAI-ARIA role written in a `role`, reported at it.

use super::super::view::Doc;
use super::super::{MESSAGES, report, tokens};
use crate::template::rules::{Report, Template, TemplateRule};

/// The abstract roles.
const ABSTRACT: [&str; 12] = [
    "command",
    "composite",
    "input",
    "landmark",
    "range",
    "roletype",
    "section",
    "sectionhead",
    "select",
    "structure",
    "widget",
    "window",
];

/// The rule.
pub struct NoAbstractRole;

impl TemplateRule for NoAbstractRole {
    fn name(&self) -> &'static str {
        "html-validate/no-abstract-role"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for element in &doc.elements {
            for attr in element.attrs.iter().filter(|attr| attr.key == "role") {
                let Some(value) = attr.text() else {
                    continue;
                };
                for (role, span) in tokens(value, attr.value_span()) {
                    if ABSTRACT.contains(&role) {
                        reports.push(report(
                            span,
                            format!("Role \"{role}\" is abstract and must not be used"),
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAbstractRole;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-abstract-role.txt", "no-abstract-role", &|_| {
            Box::new(NoAbstractRole)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (4, 2, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
