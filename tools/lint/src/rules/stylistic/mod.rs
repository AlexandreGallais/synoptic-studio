//! `@stylistic`'s rules that oxfmt leaves to a linter (EN-084), each one module with the cases
//! of its documentation in the plugin's 5.10.0 as tests, with the project's options. On for every
//! TypeScript file; each is autofixed as the plugin fixes it. Not carried: `line-comment-position`,
//! which oxlint's `no-inline-comments` covers, every comment beside code being reported there.

pub mod comments;
pub mod multiline_comment_style;
pub mod padding_line_between_statements;
pub mod spaced_comment;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &multiline_comment_style::MultilineCommentStyle,
    &padding_line_between_statements::PaddingLineBetweenStatements,
    &spaced_comment::SpacedComment,
];

/// Their names, on for every TypeScript file.
pub const NAMES: &[&str] = &[
    "@stylistic/multiline-comment-style",
    "@stylistic/padding-line-between-statements",
    "@stylistic/spaced-comment",
];

#[cfg(test)]
mod tests {
    use super::{NAMES, RULES};

    #[test]
    fn lists_each_rule_once_under_its_name() {
        let names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();

        assert_eq!(names, NAMES);
        assert!(NAMES.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
