//! `function-calculation-no-interpolation` of stylelint-scss 7.3.0
//! (`src/rules/function-calculation-no-interpolation/index.js`, REF-STYLELINT-SOURCE): an
//! interpolation as an argument of `calc()`, `max()`, `min()` or `clamp()` outside custom
//! properties, reported over its first.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct FunctionCalculationNoInterpolation;

impl SheetRule for FunctionCalculationNoInterpolation {
    fn name(&self) -> &'static str {
        "scss/function-calculation-no-interpolation"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected interpolation in \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if node.prop.starts_with("--") {
                continue;
            }
            let parsed = value::parse(&node.value);
            parsed.walk(&mut |_, part| {
                if part.kind != Part::Function
                    || !matches!(part.value.as_str(), "calc" | "max" | "min" | "clamp")
                {
                    return true;
                }
                // `/^#\{.*|\s*\}/`: a word starting an interpolation, or holding a `}`.
                let interpolation = part
                    .nodes
                    .iter()
                    .map(|&each| parsed.part(each))
                    .find(|each| {
                        each.kind == Part::Word
                            && (each.value.starts_with("#{") || each.value.contains('}'))
                    });
                if let Some(found) = interpolation {
                    let (start, end) = sheet.word_range(node, &found.value);
                    reports.push(Report::new(start, end, "rejected").with(&[&part.value]));
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionCalculationNoInterpolation;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/function-calculation-no-interpolation.txt",
            &|_| Ok(Box::new(FunctionCalculationNoInterpolation)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (4, 5, 0),
            "{:#?}",
            tally.apart
        );
    }
}
