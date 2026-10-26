//! `rule-empty-line-before` of Stylelint 17.16.0 (`lib/rules/rule-empty-line-before/index.mjs`,
//! REF-STYLELINT-SOURCE): a plain rule, not the root's first, without the empty line before it
//! its options want, or with one they refuse, reported over the rule and fixed by adding or
//! removing it. The project's options, stylelint-config-standard's: `"always-multi-line"`,
//! except a block's first, comments before left alone.

use super::empty_lines::{
    Lines, has_empty_line, is_after_single_line_comment, is_first_nested, is_first_of_root,
    previous_non_shared, report,
};
use super::{Context, Report, SheetRule, is_standard_rule};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct RuleEmptyLineBefore {
    /// The options.
    pub options: Lines,
}

impl RuleEmptyLineBefore {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        Self {
            options: Lines::new("always-multi-line", &["after-comment"], &["first-nested"]),
        }
    }
}

impl SheetRule for RuleEmptyLineBefore {
    fn name(&self) -> &'static str {
        "stylelint/rule-empty-line-before"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected empty line before rule"),
            ("rejected", "Expected no empty line before rule"),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let options = &self.options;
        for (index, node) in sheet.all(Kind::Rule) {
            if !is_standard_rule(node) || is_first_of_root(sheet, index) {
                continue;
            }
            let previous = sheet.sheet.previous(index);
            if options.ignores("after-comment")
                && previous.is_some_and(|at| sheet.node(at).is(Kind::Comment))
            {
                continue;
            }
            let nested = node.parent.is_some_and(|parent| parent != 0);
            if (options.ignores("first-nested") && is_first_nested(sheet, index))
                || (options.ignores("inside-block") && nested)
            {
                continue;
            }
            if options.primary.contains("multi-line") && !sheet.source(node).contains(['\n', '\r'])
            {
                continue;
            }
            let after_rule =
                previous_non_shared(sheet, index).is_some_and(|at| sheet.node(at).is(Kind::Rule));
            let mut expected = options.primary.contains("always");
            if (options.excepts("first-nested") && is_first_nested(sheet, index))
                || (options.excepts("after-rule") && after_rule)
                || (options.excepts("inside-block-and-after-rule") && nested && after_rule)
                || (options.excepts("after-single-line-comment")
                    && is_after_single_line_comment(sheet, index))
                || (options.excepts("inside-block") && nested)
            {
                expected = !expected;
            }
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
    use super::RuleEmptyLineBefore;
    use crate::scss::rules::tester::{empty_lines, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/rule-empty-line-before.txt",
            &|config| {
                Ok(Box::new(RuleEmptyLineBefore {
                    options: empty_lines(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (68, 65, 2),
            "{:#?}",
            tally.apart
        );
    }
}
