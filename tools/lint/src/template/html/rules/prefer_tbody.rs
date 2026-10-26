//! `prefer-tbody` of `html-validate` 11.17.0 (`src/rules/prefer-tbody.ts`,
//! REF-HTML-VALIDATE-SOURCE): a `<table>` with `<tr>` children and no `<tbody>`, its first row
//! reported at its name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct PreferTbody;

impl TemplateRule for PreferTbody {
    fn name(&self) -> &'static str {
        "html-validate/prefer-tbody"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if element.tag != "table" {
                continue;
            }
            let children = doc.child_elements(index);
            if children.iter().any(|each| doc.el(*each).tag == "tbody") {
                continue;
            }
            if let Some(row) = children.iter().find(|each| doc.el(**each).tag == "tr") {
                reports.push(report(
                    doc.name_span(*row),
                    "Prefer to wrap <tr> elements in <tbody>".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferTbody;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("prefer-tbody.txt", "prefer-tbody", &|_| {
            Box::new(PreferTbody)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (2, 1, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
