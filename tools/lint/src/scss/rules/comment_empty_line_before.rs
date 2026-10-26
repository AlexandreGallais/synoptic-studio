//! `comment-empty-line-before` of Stylelint 17.16.0
//! (`lib/rules/comment-empty-line-before/index.mjs`, REF-STYLELINT-SOURCE): a `/* */` comment of
//! its own line, not the root's first, without the empty line before it its options want, or with
//! one they refuse, reported over the comment and fixed by adding or removing it. The project's
//! options, stylelint-config-standard's: `"always"`, except a block's first, Stylelint's own
//! command comments left alone.

use super::empty_lines::{
    Lines, has_empty_line, is_after_comment, is_first_nested, is_first_of_root,
    is_shared_line_comment, report,
};
use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct CommentEmptyLineBefore {
    /// The options.
    pub options: Lines,
}

impl CommentEmptyLineBefore {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        Self {
            options: Lines::new("always", &["stylelint-commands"], &["first-nested"]),
        }
    }
}

impl SheetRule for CommentEmptyLineBefore {
    fn name(&self) -> &'static str {
        "stylelint/comment-empty-line-before"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected empty line before comment"),
            ("rejected", "Expected no empty line before comment"),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let options = &self.options;
        for (index, node) in sheet.all(Kind::Comment) {
            if is_first_of_root(sheet, index)
                || (options.ignores("stylelint-commands") && node.text.starts_with("stylelint-"))
                || (options.ignores("after-comment") && is_after_comment(sheet, index))
                || options.ignores_comment(&node.text)
                || is_shared_line_comment(sheet, index, None)
                || node.inline
            {
                continue;
            }
            let expected = !(options.excepts("first-nested") && is_first_nested(sheet, index))
                && options.primary == "always";
            if expected != has_empty_line(&node.before) {
                reports.push(report(
                    sheet,
                    index,
                    if expected { "expected" } else { "rejected" },
                    expected,
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommentEmptyLineBefore;
    use crate::scss::rules::tester::{empty_lines, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/comment-empty-line-before.txt",
            &|config| {
                Ok(Box::new(CommentEmptyLineBefore {
                    options: empty_lines(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (55, 24, 5),
            "{:#?}",
            tally.apart
        );
    }
}
