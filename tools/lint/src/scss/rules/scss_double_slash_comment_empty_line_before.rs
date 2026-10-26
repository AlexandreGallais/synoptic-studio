//! `double-slash-comment-empty-line-before` of stylelint-scss 7.3.0
//! (`src/rules/double-slash-comment-empty-line-before/index.js` and `src/utils/isInlineComment.js`,
//! REF-STYLELINT-SOURCE): a `//` comment of its own line, not the sheet's first, without the empty
//! line before it its options want, or with one they refuse, reported over it and fixed by adding
//! or removing it. The project's options, stylelint-config-standard-scss's: `"always"`, except a
//! block's first, Stylelint's commands and comments following comments left alone.

use super::empty_lines::{Lines, report};
use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct DoubleSlashCommentEmptyLineBefore {
    /// The options.
    pub options: Lines,
}

impl DoubleSlashCommentEmptyLineBefore {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        Self {
            options: Lines::new(
                "always",
                &["between-comments", "stylelint-commands"],
                &["first-nested"],
            ),
        }
    }
}

/// Whether a comment shares its line with a node before or after it (`isInlineComment`).
fn is_inline(sheet: &Context, index: usize) -> bool {
    let node = sheet.node(index);
    let before_something = sheet.sheet.next(index).is_some_and(|next| {
        let next = sheet.node(next);
        !next.is(Kind::Comment)
            && sheet.lines.line(
                node.end
                    .unwrap_or(sheet.text.len())
                    .saturating_sub(1)
                    .max(node.start),
            ) == sheet.lines.line(next.start)
    });
    !node.before.contains('\n') || before_something
}

impl SheetRule for DoubleSlashCommentEmptyLineBefore {
    fn name(&self) -> &'static str {
        "scss/double-slash-comment-empty-line-before"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected empty line before comment"),
            ("rejected", "Unexpected empty line before comment"),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let options = &self.options;
        for (index, node) in sheet.all(Kind::Comment) {
            let nested = node.parent != Some(0);
            if !node.inline
                || is_inline(sheet, index)
                || sheet.node(0).nodes().first() == Some(&index)
                || (node.text.starts_with("stylelint-") && options.ignores("stylelint-commands"))
                || (nested && options.ignores("inside-block"))
                || (options.ignores("between-comments")
                    && sheet
                        .sheet
                        .previous(index)
                        .is_some_and(|at| sheet.node(at).is(Kind::Comment)))
            {
                continue;
            }
            let first = sheet.node(node.parent.unwrap_or(0)).nodes().first() == Some(&index);
            let expected = if options.excepts("first-nested") && nested && first {
                false
            } else if nested && options.excepts("inside-block") {
                options.primary == "never"
            } else {
                options.primary == "always"
            };
            let has_empty = node.before.match_indices('\n').any(|(at, _)| {
                node.before[at + 1..].find('\n').is_some_and(|next| {
                    node.before[at + 1..at + 1 + next]
                        .chars()
                        .all(char::is_whitespace)
                })
            });
            if expected != has_empty {
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
    use super::DoubleSlashCommentEmptyLineBefore;
    use crate::scss::rules::tester::{empty_lines, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/double-slash-comment-empty-line-before.txt",
            &|config| {
                Ok(Box::new(DoubleSlashCommentEmptyLineBefore {
                    options: empty_lines(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (15, 14, 0),
            "{:#?}",
            tally.apart
        );
    }
}
