//! `no-empty-source` of Stylelint 17.16.0 (`lib/rules/no-empty-source/index.mjs`,
//! REF-STYLELINT-SOURCE): a sheet holding nothing but spaces, reported over the whole sheet and one
//! column past it, as `rangeBy` gives the root's end.

use super::{Context, Report, SheetRule};

/// The rule.
pub struct NoEmptySource;

impl SheetRule for NoEmptySource {
    fn name(&self) -> &'static str {
        "stylelint/no-empty-source"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Empty source")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let root = sheet.node(0);
        let notable = !sheet.text.trim().is_empty() && !root.nodes().is_empty();
        if notable || (root.nodes().is_empty() && !root.after.trim().is_empty()) {
            return;
        }
        reports.push(Report::new(0, sheet.text.len() + 1, "rejected"));
    }
}

#[cfg(test)]
mod tests {
    use super::NoEmptySource;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/no-empty-source.txt",
            &|_| Ok(Box::new(NoEmptySource)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (5, 9, 6),
            "{:#?}",
            tally.apart
        );
    }
}
