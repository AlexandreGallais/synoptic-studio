//! `no-dup-id` of `html-validate` 11.17.0 (`src/rules/no-dup-id.ts`, REF-HTML-VALIDATE-SOURCE): a
//! static `id` an earlier element of the same tree has, reported at its value; a `<template>`'s
//! content, and each `<ng-template>`'s, a tree of its own. Two elements in exclusive branches of
//! one `@if`, `@switch` or `@for` and its `@empty` are never rendered together, so may share one
//! (our own reading of Angular's templates). `include` and `exclude` choose the ids checked.

use regex::Regex;

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its options.
pub struct NoDupId {
    /// The ids checked, patterns, all when `None`.
    pub include: Option<Vec<String>>,
    /// The ids not checked, patterns.
    pub exclude: Option<Vec<String>>,
}

impl NoDupId {
    /// The project's configuration: every id.
    pub const PROJECT: Self = Self {
        include: None,
        exclude: None,
    };
}

/// Whether a keyword matches one of a list of patterns (`keywordPatternMatcher`): `/…/` a
/// regular expression, any other text with `*` standing for one character or more, anchored.
fn matches_pattern(list: &[String], keyword: &str) -> bool {
    list.iter().any(|pattern| {
        let source = match pattern
            .strip_prefix('/')
            .and_then(|rest| rest.strip_suffix('/'))
        {
            Some(inner) => format!("^{inner}$"),
            None => format!(
                "^{}$",
                Regex::new(r"\*+").map_or_else(
                    |_| pattern.clone(),
                    |stars| stars.replace_all(pattern, ".+").into_owned()
                )
            ),
        };
        Regex::new(&source).is_ok_and(|regex| regex.is_match(keyword))
    })
}

impl TemplateRule for NoDupId {
    fn name(&self) -> &'static str {
        "html-validate/no-dup-id"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let mut seen: Vec<(Option<usize>, usize, &str, usize)> = Vec::new();
        for index in 0..doc.elements.len() {
            let Some(attr) = doc.attr(index, "id") else {
                continue;
            };
            let Some(id) = attr.text().filter(|id| !id.is_empty()) else {
                continue;
            };
            let ignored = self
                .include
                .as_deref()
                .is_some_and(|list| !matches_pattern(list, id))
                || self
                    .exclude
                    .as_deref()
                    .is_some_and(|list| matches_pattern(list, id));
            if ignored {
                continue;
            }
            let group = doc.closest(index, "template");
            let scope = doc.el(index).scope;
            let duplicate = seen.iter().any(|(each_group, each_scope, each_id, each)| {
                *each_group == group
                    && *each_scope == scope
                    && *each_id == id
                    && !doc.exclusive(*each, index)
            });
            if duplicate {
                reports.push(report(attr.value_span(), format!("Duplicate ID \"{id}\"")));
                continue;
            }
            seen.push((group, scope, id, index));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDupId;
    use crate::template::html::cases::run;
    use crate::template::rules::reports;
    use crate::template::tree::parse;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-dup-id.txt", "no-dup-id", &|options| {
            Box::new(NoDupId {
                include: options.and_then(|options| options.texts("include")),
                exclude: options.and_then(|options| options.texts("exclude")),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (9, 4, 3),
            "{:#?}",
            tally.apart_cases
        );
    }

    #[test]
    fn lets_exclusive_branches_share_an_id() {
        // Our own cases: an `@if` and its `@else`, two cases of a `@switch`, each `<ng-template>`
        // a tree of its own; the same id twice in one branch, or after the chain, is reported.
        let count = |text: &str| {
            let parsed = parse(text);
            reports(&NoDupId::PROJECT, "a.component.html", text, &parsed.nodes).len()
        };
        assert_eq!(
            count("@if (a) {<p id=\"x\"></p>} @else {<p id=\"x\"></p>}"),
            0
        );
        assert_eq!(
            count("@switch (a) { @case (1) {<p id=\"x\"></p>} @default {<p id=\"x\"></p>} }"),
            0
        );
        assert_eq!(
            count("<ng-template><p id=\"x\"></p></ng-template><p id=\"x\"></p>"),
            0
        );
        assert_eq!(count("@if (a) {<p id=\"x\"></p><b id=\"x\"></b>}"), 1);
        assert_eq!(count("@if (a) {<p id=\"x\"></p>} <p id=\"x\"></p>"), 1);
    }
}
