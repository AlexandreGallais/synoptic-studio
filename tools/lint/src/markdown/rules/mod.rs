//! The Markdown rules, one module each with its tests, from the cases of the rule's
//! documentation in `@eslint/markdown` (`REF-ESLINT-MARKDOWN-RULES`).

pub mod fenced_code_language;
pub mod fenced_code_meta;
pub mod heading_increment;
pub mod no_bare_urls;
pub mod no_duplicate_definitions;
pub mod no_duplicate_headings;
pub mod no_empty_definitions;
pub mod no_empty_images;
pub mod no_empty_links;
pub mod no_html;
pub mod no_invalid_label_refs;

use super::MarkdownRule;

/// Every Markdown rule, run on every page.
pub const ALL: &[&dyn MarkdownRule] = &[
    &fenced_code_language::FencedCodeLanguage,
    &fenced_code_meta::FencedCodeMeta,
    &heading_increment::HeadingIncrement,
    &no_bare_urls::NoBareUrls,
    &no_duplicate_definitions::NoDuplicateDefinitions,
    &no_duplicate_headings::NoDuplicateHeadings,
    &no_empty_definitions::NoEmptyDefinitions,
    &no_empty_images::NoEmptyImages,
    &no_empty_links::NoEmptyLinks,
    &no_html::NoHtml,
    &no_invalid_label_refs::NoInvalidLabelRefs,
];
