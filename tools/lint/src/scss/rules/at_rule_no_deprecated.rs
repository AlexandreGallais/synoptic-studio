//! `at-rule-no-deprecated` of Stylelint 17.16.0 (`lib/rules/at-rule-no-deprecated/index.mjs`,
//! REF-STYLELINT-SOURCE): an at-rule of the deprecated ones (`@document`, `@nest`, `@viewport`)
//! but those its option ignores, reported over its `@name`; `@nest` fixed into a style rule of its
//! selector.

use super::reference::{DEPRECATED_AT_KEYWORDS, has};
use super::{Context, Matcher, Report, SheetRule, is_standard_at_rule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct AtRuleNoDeprecated {
    /// The at-rules left out (`ignoreAtRules`).
    pub ignore_at_rules: Vec<Matcher>,
}

impl AtRuleNoDeprecated {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self {
        ignore_at_rules: Vec::new(),
    };
}

impl SheetRule for AtRuleNoDeprecated {
    fn name(&self) -> &'static str {
        "stylelint/at-rule-no-deprecated"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Deprecated at-rule \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::AtRule) {
            let name = node.name.to_lowercase();
            if !is_standard_at_rule(node)
                || matches_any(&self.ignore_at_rules, &node.name)
                || !has(DEPRECATED_AT_KEYWORDS, &name)
            {
                continue;
            }
            let word = format!("@{}", node.name);
            let report =
                Report::new(node.start, node.start + word.len(), "rejected").with(&[&word]);
            // `@nest a {` becomes `a {`: the at-rule's name and the spaces after it dropped.
            let report = if name == "nest" && node.children.is_some() {
                let end = node.start + word.len() + node.after_name.len();
                let parent = sheet.node(sheet.node(index).parent.unwrap_or(0));
                report
                    .fixed(node.start, end, "")
                    .scoped(parent.start, parent.end.unwrap_or(sheet.text.len()))
            } else {
                report
            };
            reports.push(report);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtRuleNoDeprecated;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/at-rule-no-deprecated.txt",
            &|config| {
                Ok(Box::new(AtRuleNoDeprecated {
                    ignore_at_rules: matchers(secondary(config), "ignoreAtRules")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (7, 6, 0),
            "{:#?}",
            tally.apart
        );
    }
}
