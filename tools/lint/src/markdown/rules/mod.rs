//! The Markdown rules, one module each with its tests, from the cases of the rule's
//! documentation in `@eslint/markdown` (`REF-ESLINT-MARKDOWN-RULES`), and the project's own
//! (`local/…`). `markdown/no-reference-like-urls` is dropped (EN-077): the pages link inline,
//! reference definitions appear on one copied page, and a link whose URL is a label is a dead
//! link, which the site's build already refuses in `docs/`.

pub mod fenced_code_language;
pub mod fenced_code_meta;
pub mod heading_increment;
pub mod image_alt_text;
pub mod no_bare_urls;
pub mod no_duplicate_definitions;
pub mod no_duplicate_headings;
pub mod no_empty_definitions;
pub mod no_empty_images;
pub mod no_empty_links;
pub mod no_html;
pub mod no_invalid_label_refs;
pub mod no_missing_atx_heading_space;
pub mod no_missing_label_refs;
pub mod no_missing_link_fragments;
pub mod no_multiple_h1;
pub mod no_reversed_media_syntax;
pub mod no_space_in_emphasis;
pub mod no_unused_definitions;
pub mod require_alt_text;
pub mod table_column_count;

use super::MarkdownRule;

/// Every Markdown rule, run on every page.
pub const ALL: &[&dyn MarkdownRule] = &[
    &fenced_code_language::FencedCodeLanguage,
    &fenced_code_meta::FencedCodeMeta,
    &heading_increment::HeadingIncrement,
    &image_alt_text::ImageAltText,
    &no_bare_urls::NoBareUrls,
    &no_duplicate_definitions::NoDuplicateDefinitions,
    &no_duplicate_headings::NoDuplicateHeadings,
    &no_empty_definitions::NoEmptyDefinitions,
    &no_empty_images::NoEmptyImages,
    &no_empty_links::NoEmptyLinks,
    &no_html::NoHtml,
    &no_invalid_label_refs::NoInvalidLabelRefs,
    &no_missing_atx_heading_space::NoMissingAtxHeadingSpace,
    &no_missing_label_refs::NoMissingLabelRefs,
    &no_missing_link_fragments::NoMissingLinkFragments,
    &no_multiple_h1::NoMultipleH1,
    &no_reversed_media_syntax::NoReversedMediaSyntax,
    &no_space_in_emphasis::NoSpaceInEmphasis,
    &no_unused_definitions::NoUnusedDefinitions,
    &require_alt_text::RequireAltText,
    &table_column_count::TableColumnCount,
];
