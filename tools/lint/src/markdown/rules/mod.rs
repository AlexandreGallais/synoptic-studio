//! The Markdown rules, one module each with its tests, from the cases of the rule's
//! documentation in `@eslint/markdown` (`REF-ESLINT-MARKDOWN-RULES`).

use super::MarkdownRule;

/// Every Markdown rule, run on every page.
pub const ALL: &[&dyn MarkdownRule] = &[];
