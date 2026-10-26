//! `input-missing-label` of `html-validate` 11.17.0 (`src/rules/input-missing-label.ts`,
//! REF-HTML-VALIDATE-SOURCE): a field in the accessibility tree, not a hidden field nor a button,
//! without an accessible name: its `<label for>` or the `<label>` around it hidden or without
//! text, or none at all.

use super::super::a11y::{has_accessible_name, in_accessibility_tree};
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct InputMissingLabel;

/// The labels of a field: those naming its `id`, else the one around it.
fn labels_of(doc: &Doc, index: usize) -> Vec<usize> {
    if let Some(id) = doc.value(index, "id").filter(|id| !id.is_empty()) {
        let by_id: Vec<usize> = (0..doc.elements.len())
            .filter(|each| doc.el(*each).tag == "label" && doc.value(*each, "for") == Some(id))
            .collect();
        if !by_id.is_empty() {
            return by_id;
        }
    }
    doc.ancestors(index)
        .into_iter()
        .find(|each| doc.el(*each).tag == "label")
        .into_iter()
        .collect()
}

impl TemplateRule for InputMissingLabel {
    fn name(&self) -> &'static str {
        "html-validate/input-missing-label"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let tag = element.tag.as_str();
            if !matches!(tag, "input" | "textarea" | "select")
                || !in_accessibility_tree(&doc, index)
            {
                continue;
            }
            let ignored = tag == "input"
                && doc.value(index, "type").is_some_and(|kind| {
                    matches!(
                        kind.to_lowercase().as_str(),
                        "hidden" | "submit" | "reset" | "button"
                    )
                });
            if ignored || has_accessible_name(&doc, index) {
                continue;
            }
            let labels = labels_of(&doc, index);
            if let Some(first) = labels.first() {
                if !labels
                    .iter()
                    .any(|label| in_accessibility_tree(&doc, *label))
                {
                    reports.push(report(
                        doc.name_span(*first),
                        format!("<{tag}> element has <label> but <label> element is hidden"),
                    ));
                } else if !labels.iter().any(|label| has_accessible_name(&doc, *label)) {
                    reports.push(report(
                        doc.name_span(*first),
                        format!("<{tag}> element has <label> but <label> has no text"),
                    ));
                }
                continue;
            }
            let message = if doc.has(index, "aria-label") {
                format!("<{tag}> element has aria-label but label has no text")
            } else if doc.has(index, "aria-labelledby") {
                format!("<{tag}> element has aria-labelledby but referenced element has no text")
            } else {
                format!("<{tag}> element does not have a <label>")
            };
            reports.push(report(doc.name_span(index), message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InputMissingLabel;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("input-missing-label.txt", "input-missing-label", &|_| {
            Box::new(InputMissingLabel)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (17, 14, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
