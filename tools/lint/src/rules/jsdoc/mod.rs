//! The rules of `eslint-plugin-jsdoc` 65.3.1 that oxlint lacks (EN-083), each one module run on
//! the cases of the plugin's own tests with the project's options, read from the blocks of
//! `crate::jsdoc`. No scope turns one off: they run on every TypeScript file, as in `ESLint`;
//! `require-jsdoc` asks for the type members' blocks in the library only.

pub mod check_alignment;
pub mod check_indentation;
pub mod check_line_alignment;
pub mod check_param_names;
pub mod check_syntax;
pub mod check_template_names;
pub mod escape_inline_tags;
pub mod informative_docs;
pub mod lines_before_block;
pub mod match_description;
pub mod multiline_blocks;
pub mod no_bad_blocks;
pub mod no_blank_block_descriptions;
pub mod no_multi_asterisks;
pub mod no_restricted_syntax;
pub mod no_types;
pub mod normalize_see_links;
pub mod require_asterisk_prefix;
pub mod require_description;
pub mod require_hyphen_before_param_description;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &check_alignment::CheckAlignment,
    &check_indentation::CheckIndentation,
    &check_line_alignment::CheckLineAlignment,
    &check_param_names::CheckParamNames,
    &check_syntax::CheckSyntax,
    &check_template_names::CheckTemplateNames,
    &escape_inline_tags::EscapeInlineTags,
    &informative_docs::InformativeDocs,
    &lines_before_block::LinesBeforeBlock,
    &match_description::MatchDescription,
    &multiline_blocks::MultilineBlocks,
    &no_bad_blocks::NoBadBlocks,
    &no_blank_block_descriptions::NoBlankBlockDescriptions,
    &no_multi_asterisks::NoMultiAsterisks,
    &no_restricted_syntax::NoRestrictedSyntax,
    &no_types::NoTypes,
    &normalize_see_links::NormalizeSeeLinks,
    &require_asterisk_prefix::RequireAsteriskPrefix,
    &require_description::RequireDescription,
    &require_hyphen_before_param_description::RequireHyphenBeforeParamDescription,
];

/// Their names, on for every TypeScript file.
pub const NAMES: &[&str] = &[
    "jsdoc/check-alignment",
    "jsdoc/check-indentation",
    "jsdoc/check-line-alignment",
    "jsdoc/check-param-names",
    "jsdoc/check-syntax",
    "jsdoc/check-template-names",
    "jsdoc/escape-inline-tags",
    "jsdoc/informative-docs",
    "jsdoc/lines-before-block",
    "jsdoc/match-description",
    "jsdoc/multiline-blocks",
    "jsdoc/no-bad-blocks",
    "jsdoc/no-blank-block-descriptions",
    "jsdoc/no-multi-asterisks",
    "jsdoc/no-restricted-syntax",
    "jsdoc/no-types",
    "jsdoc/normalize-see-links",
    "jsdoc/require-asterisk-prefix",
    "jsdoc/require-description",
    "jsdoc/require-hyphen-before-param-description",
];

#[cfg(test)]
mod tests {
    use super::{NAMES, RULES};

    #[test]
    fn names_each_rule_once_in_order() {
        let names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(names, NAMES);
        assert_eq!(sorted, NAMES);
    }
}
