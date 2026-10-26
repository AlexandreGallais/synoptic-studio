//! `at-if-no-null` of stylelint-scss 7.3.0 (`src/rules/at-if-no-null/index.js`,
//! REF-STYLELINT-SOURCE): an `@if` comparing to `null` with `==` or `!=` (but `!= null and …`),
//! `null` being false in Sass, reported over its parameters.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct AtIfNoNull;

/// Whether parameters hold `== null` or `!= null` before optional spaces and `)` (`/.* OP null[ \t]*\)?/`).
fn compares(params: &str, operator: &str) -> bool {
    params
        .match_indices(&format!(" {operator} null"))
        .any(|(at, _)| at > 0 || params.starts_with(' '))
}

impl SheetRule for AtIfNoNull {
    fn name(&self) -> &'static str {
        "scss/at-if-no-null"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "equals_null",
                "Expected @if not statement rather than @if statement == null",
            ),
            (
                "not_equals_null",
                "Expected @if statement rather than @if statement != null",
            ),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            let params = &node.params;
            if node.name != "if" || params.contains(" != null and ") {
                continue;
            }
            let key = if compares(params, "==") {
                "equals_null"
            } else if compares(params, "!=") {
                "not_equals_null"
            } else {
                continue;
            };
            let (start, end) = sheet.word_range(node, params);
            reports.push(Report::new(start, end, key));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtIfNoNull;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-if-no-null.txt",
            &|_| Ok(Box::new(AtIfNoNull)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (3, 2, 0),
            "{:#?}",
            tally.apart
        );
    }
}
