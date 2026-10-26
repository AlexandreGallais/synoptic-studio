//! `at-rule-disallowed-list` of Stylelint 17.16.0 (`lib/rules/at-rule-disallowed-list/index.mjs`,
//! REF-STYLELINT-SOURCE): an at-rule whose name matches one of the option's names or patterns,
//! reported over its `@name`. The project refuses `@import`, for `@use` and `@forward`.

use super::{Context, Matcher, Report, SheetRule, is_standard_at_rule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct AtRuleDisallowedList {
    /// The names refused.
    pub names: Vec<Matcher>,
}

impl AtRuleDisallowedList {
    /// The project's configuration: `["import"]`.
    #[must_use]
    pub fn project() -> Self {
        Self {
            names: vec![Matcher::new("import")],
        }
    }
}

impl SheetRule for AtRuleDisallowedList {
    fn name(&self) -> &'static str {
        "stylelint/at-rule-disallowed-list"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Disallowed at-rule \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if !is_standard_at_rule(node) || !matches_any(&self.names, &node.name) {
                continue;
            }
            let word = format!("@{}", node.name);
            let (start, end) = sheet.word_range(node, &word);
            reports.push(Report::new(start, end, "rejected").with(&[&word]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtRuleDisallowedList;
    use crate::scss::rules::tester::{Js, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/at-rule-disallowed-list.txt",
            &|config| {
                let wrapped = Js::Object(vec![("names".to_owned(), config.clone())]);
                Ok(Box::new(AtRuleDisallowedList {
                    names: crate::scss::rules::tester::matchers(Some(&wrapped), "names")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (4, 10, 1),
            "{:#?}",
            tally.apart
        );
        assert_eq!(AtRuleDisallowedList::project().names.len(), 1);
    }
}
