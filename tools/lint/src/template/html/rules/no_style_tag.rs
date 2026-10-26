//! `no-style-tag` of `html-validate` 11.17.0 (`src/rules/no-style-tag.ts`,
//! REF-HTML-VALIDATE-SOURCE): a `<style>`, but one a `<template>` holds when allowed (the rule's
//! default), reported at its name: a component's styles belong to its `.scss` file.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its option.
pub struct NoStyleTag {
    /// Whether a `<style>` a `<template>` holds is allowed.
    pub allow_template: bool,
}

impl NoStyleTag {
    /// The project's configuration: the rule's default.
    pub const PROJECT: Self = Self {
        allow_template: true,
    };
}

impl TemplateRule for NoStyleTag {
    fn name(&self) -> &'static str {
        "html-validate/no-style-tag"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let in_template = element
                .parent
                .is_some_and(|parent| doc.el(parent).tag == "template");
            if element.tag == "style" && !(self.allow_template && in_template) {
                // The report falls back to its event's place, the start tag's `<style`.
                let name = doc.name_span(index);
                reports.push(report(
                    crate::template::lexer::Span::new(name.start as usize - 1, name.end as usize),
                    "Use external stylesheet with <link> instead of <style> tag".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoStyleTag;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-style-tag.txt", "no-style-tag", &|options| {
            Box::new(NoStyleTag {
                allow_template: options
                    .and_then(|options| options.flag("allowTemplate"))
                    .unwrap_or(true),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (2, 3, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
