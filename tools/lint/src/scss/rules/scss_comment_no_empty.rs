//! `comment-no-empty` of stylelint-scss 7.3.0 (`src/rules/comment-no-empty/index.js`,
//! REF-STYLELINT-SOURCE): a comment, `/* */` or `//`, with no text, reported over it.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct CommentNoEmpty;

impl SheetRule for CommentNoEmpty {
    fn name(&self) -> &'static str {
        "scss/comment-no-empty"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected empty comment")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Comment) {
            if node.text.is_empty() {
                reports.push(Report::new(
                    node.start,
                    node.end.unwrap_or(sheet.text.len()),
                    "rejected",
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommentNoEmpty;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/comment-no-empty.txt",
            &|_| Ok(Box::new(CommentNoEmpty)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (3, 9, 0),
            "{:#?}",
            tally.apart
        );
    }
}
