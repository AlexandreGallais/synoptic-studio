//! `block-no-redundant-nested-style-rules` of Stylelint 17.16.0
//! (`lib/rules/block-no-redundant-nested-style-rules/index.mjs`, REF-STYLELINT-SOURCE): a nested
//! rule whose selector is `&` alone, reported over it.

use super::{Context, Report, SheetRule, is_standard_rule};
use crate::scss::tree::Kind;

/// The rule.
pub struct BlockNoRedundantNestedStyleRules;

impl SheetRule for BlockNoRedundantNestedStyleRules {
    fn name(&self) -> &'static str {
        "stylelint/block-no-redundant-nested-style-rules"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Redundant nested style rule")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            if is_standard_rule(node)
                && node.selector == "&"
                && node.parent.is_some_and(|parent| parent != 0)
            {
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
    use super::BlockNoRedundantNestedStyleRules;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/block-no-redundant-nested-style-rules.txt",
            &|_| Ok(Box::new(BlockNoRedundantNestedStyleRules)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (15, 7, 0),
            "{:#?}",
            tally.apart
        );
    }
}
