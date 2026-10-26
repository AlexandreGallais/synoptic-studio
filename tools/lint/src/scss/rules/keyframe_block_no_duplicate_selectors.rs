//! `keyframe-block-no-duplicate-selectors` of Stylelint 17.16.0
//! (`lib/rules/keyframe-block-no-duplicate-selectors/index.mjs`, REF-STYLELINT-SOURCE): a keyframe
//! selector written again in its `@keyframes`, in any case, reported over the repeat.

use super::keyframes_name_pattern::is_keyframes;
use super::{Context, Report, SheetRule, is_standard_rule};
use crate::javascript::JsText;
use crate::scss::selector::{self, Kind as Part, Selectors};
use crate::scss::tree::Kind;

/// The rule.
pub struct KeyframeBlockNoDuplicateSelectors;

/// A selector's text without its comments at its ends, and where it starts in the selector list
/// (`getStrippedSelectorSource`).
#[must_use]
pub fn stripped(parsed: &Selectors, text: &str, selector: usize) -> (String, usize) {
    let part = parsed.part(selector);
    let meaningful: Vec<usize> = part
        .nodes
        .iter()
        .copied()
        .filter(|&each| parsed.part(each).kind != Part::Comment)
        .collect();
    match (meaningful.first(), meaningful.last()) {
        (Some(&first), Some(&last)) => {
            let start = parsed.part(first).start;
            let end = parsed.part(last).end;
            (text[start..end].js_trim().to_owned(), start)
        }
        _ => (text[part.start..part.end].js_trim().to_owned(), part.start),
    }
}

impl SheetRule for KeyframeBlockNoDuplicateSelectors {
    fn name(&self) -> &'static str {
        "stylelint/keyframe-block-no-duplicate-selectors"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Duplicate keyframe selector \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::AtRule) {
            if !is_keyframes(&node.name) {
                continue;
            }
            let mut seen: Vec<String> = Vec::new();
            let rules = sheet
                .all(Kind::Rule)
                .into_iter()
                .filter(|(rule, _)| sheet.sheet.ancestors(*rule).contains(&index));
            for (_, rule) in rules {
                if !is_standard_rule(rule) {
                    continue;
                }
                let text = sheet.selector(rule);
                let Ok(parsed) = selector::parse(text) else {
                    continue;
                };
                for &each in &parsed.top {
                    let (written, start) = stripped(&parsed, text, each);
                    let lower = written.to_lowercase();
                    if seen.contains(&lower) {
                        let at = rule.start + start;
                        reports.push(
                            Report::new(at, at + written.len(), "rejected").with(&[&written]),
                        );
                    } else {
                        seen.push(lower);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KeyframeBlockNoDuplicateSelectors;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/keyframe-block-no-duplicate-selectors.txt",
            &|_| Ok(Box::new(KeyframeBlockNoDuplicateSelectors)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (3, 9, 1),
            "{:#?}",
            tally.apart
        );
    }
}
