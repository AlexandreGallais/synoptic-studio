//! `media-feature-name-no-vendor-prefix` of Stylelint 17.16.0
//! (`lib/rules/media-feature-name-no-vendor-prefix/index.mjs`, REF-STYLELINT-SOURCE): a prefixed
//! `device-pixel-ratio` feature of an `@media`, but those its option ignores, reported over its
//! first occurrence in the at-rule and fixed by dropping the prefix.

use super::prefixed::media_feature_name;
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct MediaFeatureNameNoVendorPrefix {
    /// The features left out (`ignoreMediaFeatureNames`).
    pub ignore: Vec<Matcher>,
}

impl MediaFeatureNameNoVendorPrefix {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

/// The prefixed features the original looks for.
const FEATURES: &[&str] = &[
    "-webkit-device-pixel-ratio",
    "-webkit-min-device-pixel-ratio",
    "-webkit-max-device-pixel-ratio",
    "-o-device-pixel-ratio",
    "-o-min-device-pixel-ratio",
    "-o-max-device-pixel-ratio",
    "-moz-device-pixel-ratio",
    "min--moz-device-pixel-ratio",
    "max--moz-device-pixel-ratio",
];

/// Each match of the features in a text, left to right, as a global regular expression of
/// their alternation finds them in any case.
fn matches(text: &str) -> Vec<(usize, &str)> {
    let lower = text.to_ascii_lowercase();
    let mut found = Vec::new();
    let mut at = 0;
    while at < lower.len() {
        let hit = FEATURES
            .iter()
            .find(|feature| lower[at..].starts_with(*feature));
        if let Some(feature) = hit {
            found.push((at, &text[at..at + feature.len()]));
            at += feature.len();
        } else {
            at += lower[at..].chars().next().map_or(1, char::len_utf8);
        }
    }
    found
}

/// A feature without its vendor prefix (`replace(/-moz-|-o-|-webkit-/i, '')`).
fn unprefixed(feature: &str) -> String {
    let lower = feature.to_ascii_lowercase();
    let first = ["-moz-", "-o-", "-webkit-"]
        .iter()
        .filter_map(|prefix| lower.find(prefix).map(|at| (at, prefix.len())))
        .min();
    first.map_or_else(
        || feature.to_owned(),
        |(at, length)| format!("{}{}", &feature[..at], &feature[at + length..]),
    )
}

impl SheetRule for MediaFeatureNameNoVendorPrefix {
    fn name(&self) -> &'static str {
        "stylelint/media-feature-name-no-vendor-prefix"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Vendor-prefixed media feature name")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if !node.name.eq_ignore_ascii_case("media") || !media_feature_name(&node.params) {
                continue;
            }
            let params_start = sheet.params_start(node);
            let params = sheet.params(node);
            for (_, feature) in matches(sheet.source(node)) {
                if matches_any(&self.ignore, feature) {
                    continue;
                }
                let (start, end) = sheet.word_range(node, feature);
                let report = Report::new(start, end, "rejected")
                    .scoped(node.start, node.end.unwrap_or(sheet.text.len()));
                let report = match params.find(feature) {
                    Some(at) => {
                        let from = params_start + at;
                        report.fixed(from, from + feature.len(), unprefixed(feature))
                    }
                    None => report,
                };
                reports.push(report);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MediaFeatureNameNoVendorPrefix, unprefixed};
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/media-feature-name-no-vendor-prefix.txt",
            &|config| {
                Ok(Box::new(MediaFeatureNameNoVendorPrefix {
                    ignore: matchers(secondary(config), "ignoreMediaFeatureNames")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (8, 12, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(
            unprefixed("min--moz-device-pixel-ratio"),
            "min-device-pixel-ratio"
        );
    }
}
