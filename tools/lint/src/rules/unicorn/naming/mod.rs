//! Unicorn's rules on names, comments and types that oxlint lacks (EN-080), each one module with
//! the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, with the project's
//! options. No scope turns one off: they run on every TypeScript file.
//! `single-line-block-comment-style` is not carried: `@stylistic/multiline-comment-style`
//! already refuses what it reports.

pub mod comment_content;
pub mod consistent_compound_words;
pub mod consistent_tuple_labels;
pub mod name_replacements;
pub mod no_keyword_prefix;
pub mod prefer_type_literal_last;
