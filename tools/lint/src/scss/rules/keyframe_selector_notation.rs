//! `keyframe-selector-notation` of Stylelint 17.16.0
//! (`lib/rules/keyframe-selector-notation/index.mjs`, REF-STYLELINT-SOURCE): a keyframe selector
//! written `from` or `to` where its option wants `0%` and `100%`, or the reverse, reported over it
//! and fixed by rewriting it. The project's option, stylelint-config-standard's,
//! `"percentage-unless-within-keyword-only-block"`: percentages, unless every selector of the
//! `@keyframes` is a keyword.

use super::keyframes_name_pattern::is_keyframes;
use super::{Context, Report, SheetRule};
use crate::javascript::JsText;
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The notation wanted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notation {
    /// `from` and `to`.
    Keyword,
    /// `0%` and `100%`.
    Percentage,
    /// Percentages, unless every selector of the block is a keyword.
    PercentageUnlessKeywordOnly,
}

/// The rule and its option.
pub struct KeyframeSelectorNotation {
    /// The notation wanted.
    pub notation: Notation,
}

impl KeyframeSelectorNotation {
    /// The project's configuration: `"percentage-unless-within-keyword-only-block"`.
    pub const PROJECT: Self = Self {
        notation: Notation::PercentageUnlessKeywordOnly,
    };
}

/// Whether a selector is a keyword, `from` or `to`.
fn is_keyword(selector: &str) -> bool {
    selector == "from" || selector == "to"
}

impl SheetRule for KeyframeSelectorNotation {
    fn name(&self) -> &'static str {
        "stylelint/keyframe-selector-notation"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to be \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::AtRule) {
            if !is_keyframes(&node.name) {
                continue;
            }
            let rules: Vec<_> = sheet
                .all(Kind::Rule)
                .into_iter()
                .filter(|(rule, _)| sheet.sheet.ancestors(*rule).contains(&index))
                .collect();
            let keyword_only = rules.iter().all(|(_, rule)| {
                rule.selector
                    .split(',')
                    .all(|each| is_keyword(&each.js_trim().to_lowercase()))
            });
            for (_, rule) in &rules {
                let text = sheet.selector(rule);
                let Ok(parsed) = selector::parse(text) else {
                    continue;
                };
                for &each in &parsed.top {
                    let relevant: Vec<usize> = parsed
                        .part(each)
                        .nodes
                        .iter()
                        .copied()
                        .filter(|&part| {
                            let part = parsed.part(part);
                            part.kind != Part::Comment
                                && !(part.kind == Part::Combinator
                                    && part.value.js_trim().is_empty())
                        })
                        .collect();
                    let [only] = relevant.as_slice() else {
                        continue;
                    };
                    let tag = parsed.part(*only);
                    let lower = tag.value.to_lowercase();
                    if tag.kind != Part::Tag
                        || !(is_keyword(&lower) || lower == "0%" || lower == "100%")
                    {
                        continue;
                    }
                    let fine = match self.notation {
                        Notation::Keyword => is_keyword(&lower),
                        Notation::Percentage => !is_keyword(&lower),
                        Notation::PercentageUnlessKeywordOnly => {
                            keyword_only || !is_keyword(&lower)
                        }
                    };
                    if fine {
                        continue;
                    }
                    let fixed = match lower.as_str() {
                        "from" => "0%",
                        "to" => "100%",
                        "0%" => "from",
                        _ => "to",
                    };
                    let at = rule.start + tag.start;
                    reports.push(
                        Report::new(at, at + lower.len(), "expected")
                            .with(&[&tag.value, fixed])
                            .fixed(at, at + tag.value.len(), fixed)
                            .scoped(rule.start, rule.end.unwrap_or(sheet.text.len())),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyframeSelectorNotation, Notation};
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/keyframe-selector-notation.txt",
            &|config| {
                let notation = match primary(config) {
                    Js::Text(text) if text == "keyword" => Notation::Keyword,
                    Js::Text(text) if text == "percentage" => Notation::Percentage,
                    _ => Notation::PercentageUnlessKeywordOnly,
                };
                Ok(Box::new(KeyframeSelectorNotation { notation }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (41, 21, 1),
            "{:#?}",
            tally.apart
        );
    }
}
