//! `at-extend-no-missing-placeholder` of stylelint-scss 7.3.0
//! (`src/rules/at-extend-no-missing-placeholder/index.js`, REF-STYLELINT-SOURCE): an `@extend` of
//! anything but a `%placeholder` or an interpolation, reported over its parameters.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct AtExtendNoMissingPlaceholder;

impl SheetRule for AtExtendNoMissingPlaceholder {
    fn name(&self) -> &'static str {
        "scss/at-extend-no-missing-placeholder"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "rejected",
            "Expected a placeholder selector (e.g. %placeholder) to be used in @extend",
        )]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            let param = node.params.trim();
            let interpolated = param.starts_with("#{") && param[2..].contains('}');
            if node.name != "extend" || param.starts_with('%') || interpolated {
                continue;
            }
            let (start, end) = sheet.word_range(node, param);
            reports.push(Report::new(start, end, "rejected"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtExtendNoMissingPlaceholder;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-extend-no-missing-placeholder.txt",
            &|_| Ok(Box::new(AtExtendNoMissingPlaceholder)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (3, 4, 0),
            "{:#?}",
            tally.apart
        );
    }
}
