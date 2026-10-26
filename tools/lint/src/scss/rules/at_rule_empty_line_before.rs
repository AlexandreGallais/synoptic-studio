//! `at-rule-empty-line-before` of Stylelint 17.16.0 (`lib/rules/at-rule-empty-line-before/index.mjs`,
//! REF-STYLELINT-SOURCE): a plain at-rule, not the root's first, without the empty line before it
//! its options want, or with one they refuse, reported over the at-rule and fixed by adding or
//! removing it. The project's options, stylelint-config-standard-scss's: `"always"`, except a
//! blockless at-rule after another and a block's first, comments before and `@else` left alone.

use super::empty_lines::{
    Lines, has_empty_line, is_after_comment, is_blockless_after_blockless,
    is_blockless_after_same_name, is_first_nested, is_first_of_root, previous_non_shared, report,
};
use super::{Context, Matcher, Report, SheetRule, is_standard_at_rule};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct AtRuleEmptyLineBefore {
    /// The options.
    pub options: Lines,
}

impl AtRuleEmptyLineBefore {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        let mut options = Lines::new(
            "always",
            &["after-comment"],
            &["blockless-after-blockless", "first-nested"],
        );
        options.ignore_at_rules = vec![Matcher::new("else")];
        Self { options }
    }
}

impl SheetRule for AtRuleEmptyLineBefore {
    fn name(&self) -> &'static str {
        "stylelint/at-rule-empty-line-before"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected empty line before at-rule"),
            ("rejected", "Expected no empty line before at-rule"),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let options = &self.options;
        for (index, node) in sheet.all(Kind::AtRule) {
            let nested = node.parent.is_some_and(|parent| parent != 0);
            if is_first_of_root(sheet, index)
                || !is_standard_at_rule(node)
                || options.ignores_at_rule(&node.name)
                || (options.ignores("blockless-after-blockless")
                    && is_blockless_after_blockless(sheet, index))
                || (options.ignores("first-nested") && is_first_nested(sheet, index))
                || (options.ignores("blockless-after-same-name-blockless")
                    && is_blockless_after_same_name(sheet, index))
                || (options.ignores("inside-block") && nested)
                || (options.ignores("after-comment") && is_after_comment(sheet, index))
            {
                continue;
            }
            let after_same_name = previous_non_shared(sheet, index).is_some_and(|at| {
                sheet.node(at).is(Kind::AtRule) && sheet.node(at).name == node.name
            });
            let mut expected = options.primary == "always";
            if (options.excepts("after-same-name") && after_same_name)
                || (options.excepts("inside-block") && nested)
                || (options.excepts("first-nested") && is_first_nested(sheet, index))
                || (options.excepts("blockless-after-blockless")
                    && is_blockless_after_blockless(sheet, index))
                || (options.excepts("blockless-after-same-name-blockless")
                    && is_blockless_after_same_name(sheet, index))
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
    use super::AtRuleEmptyLineBefore;
    use crate::scss::rules::tester::{empty_lines, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/at-rule-empty-line-before.txt",
            &|config| {
                Ok(Box::new(AtRuleEmptyLineBefore {
                    options: empty_lines(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (206, 152, 2),
            "{:#?}",
            tally.apart
        );
    }
}
