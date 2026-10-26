//! `element-permitted-parent` of `html-validate` 11.17.0
//! (`src/rules/element-permitted-parent.ts`, REF-HTML-VALIDATE-SOURCE): an element whose parent
//! its metadata does not permit; neither at a root, nor under an element of its own name, which
//! `element-permitted-content` reports, nor (our own case) under a component, a custom element
//! whose content Angular projects elsewhere.

use super::super::elements::{Entry, natural_join};
use super::super::validator::permitted;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ElementPermittedParent;

/// The message (`formatMessage`): the parents permitted when each is a category or a tag.
fn message(child: &str, parent: &str, rules: &[Entry]) -> String {
    let named: Option<Vec<String>> = rules
        .iter()
        .map(|entry| match entry {
            Entry::Item(item) => Some(
                item.strip_prefix('@')
                    .map_or_else(|| format!("<{item}>"), ToOwned::to_owned),
            ),
            Entry::Exclude(_) => None,
        })
        .collect();
    match named.filter(|named| !named.is_empty()) {
        Some(named) => format!(
            "<{child}> element requires a {} element as parent",
            natural_join(&named)
        ),
        None => format!("<{child}> element cannot have <{parent}> element as parent"),
    }
}

impl TemplateRule for ElementPermittedParent {
    fn name(&self) -> &'static str {
        "html-validate/element-permitted-parent"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(parent) = element.parent else {
                continue;
            };
            // Our own case: Angular projects content: an element written inside a component (a
            // custom element, its name holding a dash) lands where the component's template puts
            // it, which this template does not show.
            if doc.el(parent).tag == element.tag || doc.el(parent).tag.contains('-') {
                continue;
            }
            let Some(rules) = element.meta.and_then(|meta| meta.permitted_parent) else {
                continue;
            };
            if !permitted(&doc, parent, Some(rules)) {
                reports.push(report(
                    doc.name_span(index),
                    message(&element.tag, &doc.el(parent).tag, rules),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementPermittedParent;
    use crate::template::html::cases::{messages, run};

    #[test]
    fn leaves_an_element_projected_into_a_component() {
        // Our own case: Angular projects content: `<li>` written inside `<app-list>` lands where
        // the component's template puts its `<ng-content>`, a list; under `<div>` it is reported.
        assert_eq!(
            messages(
                &ElementPermittedParent,
                "<app-list><li>a</li></app-list><div><li>b</li></div>"
            ),
            ["1:38 <li> element requires a <ul>, <ol>, <menu> or <template> element as parent"]
        );
    }

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "element-permitted-parent.txt",
            "element-permitted-parent",
            &|_| Box::new(ElementPermittedParent),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 2, 8),
            "{:#?}",
            tally.apart_cases
        );
    }
}
