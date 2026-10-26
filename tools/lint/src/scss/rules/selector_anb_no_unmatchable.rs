//! `selector-anb-no-unmatchable` of Stylelint 17.16.0
//! (`lib/rules/selector-anb-no-unmatchable/index.mjs`, REF-STYLELINT-SOURCE): an `An+B` pseudo-class
//! (`:nth-child()` and its kin) of a plain rule whose `A` and `B` are both zero or absent
//! (`0n+0`), matching no element, reported over the pseudo-class. Its argument is read by a
//! reading of ours of the `An+B` microsyntax `CSSTree` gives Stylelint.

use super::reference::{
    A_NPLUS_BNOTATION_PSEUDO_CLASSES, A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES, has,
};
use super::{Context, Report, SheetRule, is_standard_rule};
use crate::javascript::is_js_space;
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule.
pub struct SelectorAnbNoUnmatchable;

/// Whether an `An+B` argument (its `of S` left out) has no nonzero coefficient; none for one not
/// read as `An+B` (`odd`, `even`, a variable).
fn is_unmatchable(argument: &str) -> Option<bool> {
    let mut lower = argument.to_lowercase();
    while let Some(open) = lower.find("/*") {
        let close = lower[open..]
            .find("*/")
            .map_or(lower.len(), |end| open + end + 2);
        lower.replace_range(open..close, " ");
    }
    let formula = lower.split(" of ").next().unwrap_or_default();
    let compact: String = formula.chars().filter(|c| !is_js_space(*c)).collect();
    if compact.is_empty() || compact == "odd" || compact == "even" {
        return None;
    }
    let zero = |text: &str| -> Option<bool> {
        let digits = text.trim_start_matches(['+', '-']);
        (!digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()))
            .then(|| digits.bytes().all(|byte| byte == b'0'))
    };
    match compact.split_once('n') {
        Some((a, b)) => {
            let a_zero = match a {
                "" | "+" | "-" => false,
                other => zero(other)?,
            };
            let b_zero = if b.is_empty() { true } else { zero(b)? };
            Some(a_zero && b_zero)
        }
        None => zero(&compact),
    }
}

impl SheetRule for SelectorAnbNoUnmatchable {
    fn name(&self) -> &'static str {
        "stylelint/selector-anb-no-unmatchable"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unmatchable An+B selector \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            if !node.selector.to_lowercase().contains(":nth") || !is_standard_rule(node) {
                continue;
            }
            let text = sheet.selector(node);
            let Ok(parsed) = selector::parse(text) else {
                continue;
            };
            for each in parsed.walk() {
                let pseudo = parsed.part(each);
                let name = pseudo.value.trim_start_matches(':');
                if pseudo.kind != Part::Pseudo
                    || pseudo.value.starts_with("::")
                    || !(has(A_NPLUS_BNOTATION_PSEUDO_CLASSES, name)
                        || has(A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES, name))
                    || pseudo.nodes.is_empty()
                {
                    continue;
                }
                let written = &text[pseudo.start..pseudo.end];
                let argument = written
                    .find('(')
                    .map(|open| written[open + 1..].trim_end_matches(')'))
                    .unwrap_or_default();
                if is_unmatchable(argument) == Some(true) {
                    let at = node.start + pseudo.start;
                    reports.push(Report::new(at, at + written.len(), "rejected").with(&[written]));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectorAnbNoUnmatchable, is_unmatchable};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-anb-no-unmatchable.txt",
            &|_| Ok(Box::new(SelectorAnbNoUnmatchable)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (12, 14, 1),
            "{:#?}",
            tally.apart
        );
        assert_eq!(
            [
                is_unmatchable("0n + 0"),
                is_unmatchable("-0"),
                is_unmatchable("2n"),
                is_unmatchable("odd"),
                is_unmatchable("$a")
            ],
            [Some(true), Some(true), Some(false), None, None]
        );
    }
}
