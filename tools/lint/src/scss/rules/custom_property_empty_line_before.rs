//! `custom-property-empty-line-before` of Stylelint 17.16.0
//! (`lib/rules/custom-property-empty-line-before/index.mjs`, REF-STYLELINT-SOURCE): a plain custom
//! property declaration without the empty line before it its options want, or with one they
//! refuse, reported over the declaration and fixed by adding or removing it. The project's
//! options, stylelint-config-standard's: `"always"`, except after another custom property and a
//! block's first, comments before and one-line blocks left alone.

use super::empty_lines::{
    Lines, has_empty_line, is_after_comment, is_first_nested, previous_non_shared, report,
};
use super::{Context, Report, SheetRule, is_standard_declaration};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct CustomPropertyEmptyLineBefore {
    /// The options.
    pub options: Lines,
}

impl CustomPropertyEmptyLineBefore {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        Self {
            options: Lines::new(
                "always",
                &["after-comment", "inside-single-line-block"],
                &["after-custom-property", "first-nested"],
            ),
        }
    }
}

impl SheetRule for CustomPropertyEmptyLineBefore {
    fn name(&self) -> &'static str {
        "stylelint/custom-property-empty-line-before"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected empty line before custom property"),
            ("rejected", "Expected no empty line before custom property"),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let options = &self.options;
        for (index, node) in sheet.all(Kind::Declaration) {
            if !is_standard_declaration(sheet, index) || !node.prop.starts_with("--") {
                continue;
            }
            let after_custom = previous_non_shared(sheet, index).is_some_and(|at| {
                sheet.node(at).is(Kind::Declaration) && sheet.node(at).prop.starts_with("--")
            });
            let parent = sheet.node(node.parent.unwrap_or(0));
            let single_line_block = (parent.is(Kind::Rule) || parent.is(Kind::AtRule))
                && parent.children.is_some()
                && !sheet.source(parent)[sheet.prelude_length(parent)..].contains(['\n', '\r']);
            let after_block = sheet
                .sheet
                .previous(index)
                .is_some_and(|at| sheet.node(at).is(Kind::Rule) || sheet.node(at).is(Kind::AtRule));
            if (options.ignores("after-comment") && is_after_comment(sheet, index))
                || (options.ignores("after-custom-property") && after_custom)
                || (options.ignores("first-nested") && is_first_nested(sheet, index))
                || (options.ignores("inside-single-line-block") && single_line_block)
            {
                continue;
            }
            let mut expected = options.primary == "always";
            if (options.excepts("first-nested") && is_first_nested(sheet, index))
                || (options.excepts("after-block") && after_block)
                || (options.excepts("after-comment") && is_after_comment(sheet, index))
                || (options.excepts("after-custom-property") && after_custom)
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
    use super::CustomPropertyEmptyLineBefore;
    use crate::scss::rules::tester::{empty_lines, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/custom-property-empty-line-before.txt",
            &|config| {
                Ok(Box::new(CustomPropertyEmptyLineBefore {
                    options: empty_lines(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (64, 51, 1),
            "{:#?}",
            tally.apart
        );
    }
}
