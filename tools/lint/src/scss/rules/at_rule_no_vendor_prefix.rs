//! `at-rule-no-vendor-prefix` of Stylelint 17.16.0 (`lib/rules/at-rule-no-vendor-prefix/index.mjs`,
//! REF-STYLELINT-SOURCE): a prefixed at-rule whose standard form Autoprefixer knows
//! (`@-webkit-keyframes`), but those its option ignores, reported over its `@name`; fixed by
//! dropping the prefix.

use super::prefixed::{at_rule_name, unprefix};
use super::{Context, Matcher, Report, SheetRule, is_standard_at_rule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct AtRuleNoVendorPrefix {
    /// The at-rules left out (`ignoreAtRules`).
    pub ignore_at_rules: Vec<Matcher>,
}

impl AtRuleNoVendorPrefix {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self {
        ignore_at_rules: Vec::new(),
    };
}

impl SheetRule for AtRuleNoVendorPrefix {
    fn name(&self) -> &'static str {
        "stylelint/at-rule-no-vendor-prefix"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Vendor-prefixed at-rule \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if !is_standard_at_rule(node)
                || matches_any(&self.ignore_at_rules, &node.name)
                || !node.name.starts_with('-')
                || !at_rule_name(&node.name)
            {
                continue;
            }
            let word = format!("@{}", node.name);
            let (start, end) = sheet.word_range(node, &word);
            let name_start = node.start + 1;
            reports.push(Report::new(start, end, "rejected").with(&[&word]).fixed(
                name_start,
                name_start + node.name.len(),
                unprefix(&node.name),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtRuleNoVendorPrefix;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/at-rule-no-vendor-prefix.txt",
            &|config| {
                Ok(Box::new(AtRuleNoVendorPrefix {
                    ignore_at_rules: matchers(secondary(config), "ignoreAtRules")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (6, 6, 0),
            "{:#?}",
            tally.apart
        );
    }
}
