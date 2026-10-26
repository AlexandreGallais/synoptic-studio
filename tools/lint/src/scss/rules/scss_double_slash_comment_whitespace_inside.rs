//! `double-slash-comment-whitespace-inside` of stylelint-scss 7.3.0
//! (`src/rules/double-slash-comment-whitespace-inside/index.js`, REF-STYLELINT-SOURCE): a `//`
//! comment with text but no space after its slashes where the option wants one (`"always"`, the
//! project's), or with one where it refuses it, reported after the slashes and fixed by adding or
//! removing the space.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct DoubleSlashCommentWhitespaceInside {
    /// Whether a space is wanted (`"always"`), else refused.
    pub always: bool,
}

impl DoubleSlashCommentWhitespaceInside {
    /// The project's configuration: `"always"`.
    pub const PROJECT: Self = Self { always: true };
}

impl SheetRule for DoubleSlashCommentWhitespaceInside {
    fn name(&self) -> &'static str {
        "scss/double-slash-comment-whitespace-inside"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected a space after //"),
            ("rejected", "Unexpected space after //"),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Comment) {
            if !node.inline || node.text.trim().is_empty() {
                continue;
            }
            // The comment's own text: its spaces before (`left`) and its text as written.
            let inside = &sheet.source(node)[2..];
            let left = inside.len() - inside.trim_start().len();
            let text = inside.trim();
            let slashes = text.len() - text.trim_start_matches('/').len();
            let after_slashes = &text[slashes..];
            let key = if !self.always
                && (left > 0 || (slashes > 0 && after_slashes.starts_with(char::is_whitespace)))
            {
                "rejected"
            } else if self.always
                && left == 0
                && !(after_slashes.is_empty() || after_slashes.starts_with(char::is_whitespace))
            {
                "expected"
            } else {
                continue;
            };
            let at = node.start + 2 + slashes;
            let body = node.start + 2 + left;
            let report = Report::new(at, at, key);
            reports.push(if self.always {
                report.fixed(body + slashes, body + slashes, " ")
            } else {
                let spaces = after_slashes.len() - after_slashes.trim_start().len();
                let removed = if slashes > 0 { spaces } else { 0 };
                report.fixed(node.start + 2, body + slashes + removed, &text[..slashes])
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DoubleSlashCommentWhitespaceInside;
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/double-slash-comment-whitespace-inside.txt",
            &|config| {
                Ok(Box::new(DoubleSlashCommentWhitespaceInside {
                    always: primary(config) == &Js::Text("always".to_owned()),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (12, 9, 4),
            "{:#?}",
            tally.apart
        );
    }
}
