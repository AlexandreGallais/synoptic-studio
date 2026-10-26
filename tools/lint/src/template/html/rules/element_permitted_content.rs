//! `element-permitted-content` of `html-validate` 11.17.0
//! (`src/rules/element-permitted-content.ts`, REF-HTML-VALIDATE-SOURCE): an element its parent's
//! content model does not permit, a transparent element's children checked against the parent
//! above it; else an element an ancestor's permitted descendants exclude, up to a `<template>`.
//! An element at a root has no known parent: a template's root is inside its component's host.

use super::super::elements::Transparent;
use super::super::validator::{matches_category, permitted};
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ElementPermittedContent;

/// Reports an element its parent does not permit, or a transparent element's child; whether one
/// was.
fn check_content(doc: &Doc, index: usize, parent: usize, reports: &mut Vec<Report>) -> bool {
    let rules = doc.el(parent).meta.and_then(|meta| meta.permitted_content);
    if !permitted(doc, index, rules) {
        reports.push(report(
            doc.name_span(index),
            format!(
                "<{}> element is not permitted as content under <{}>",
                doc.el(index).tag,
                doc.el(parent).tag
            ),
        ));
        return true;
    }
    let transparent = doc
        .el(index)
        .meta
        .map_or(Transparent::No, |meta| meta.transparent);
    let children: Vec<usize> = match transparent {
        Transparent::No => return false,
        Transparent::Yes => doc.child_elements(index),
        Transparent::Only(categories) => doc
            .child_elements(index)
            .into_iter()
            .filter(|child| {
                categories
                    .iter()
                    .any(|category| matches_category(doc, *child, category, false))
            })
            .collect(),
    };
    let mut any = false;
    for child in children {
        any |= check_content(doc, child, parent, reports);
    }
    any
}

/// Reports an element an ancestor's permitted descendants exclude; whether one did.
fn check_descendant(doc: &Doc, index: usize, reports: &mut Vec<Report>) -> bool {
    for ancestor in doc.ancestors(index) {
        let element = doc.el(ancestor);
        let Some(meta) = element.meta else {
            continue;
        };
        if element.tag == "template"
            && meta.template_root.of(doc, ancestor)
            && meta.script_supporting.of(doc, ancestor)
        {
            return false;
        }
        let Some(rules) = meta.permitted_descendants else {
            continue;
        };
        if permitted(doc, index, Some(rules)) {
            continue;
        }
        reports.push(report(
            doc.name_span(index),
            format!(
                "<{}> element is not permitted as a descendant of <{}>",
                doc.el(index).tag,
                element.tag
            ),
        ));
        return true;
    }
    false
}

impl TemplateRule for ElementPermittedContent {
    fn name(&self) -> &'static str {
        "html-validate/element-permitted-content"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for index in 0..doc.elements.len() {
            let Some(parent) = doc.el(index).parent else {
                continue;
            };
            if doc.el(parent).meta.is_some() && check_content(&doc, index, parent, reports) {
                continue;
            }
            check_descendant(&doc, index, reports);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementPermittedContent;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "element-permitted-content.txt",
            "element-permitted-content",
            &|_| Box::new(ElementPermittedContent),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 6, 12),
            "{:#?}",
            tally.apart_cases
        );
    }
}
