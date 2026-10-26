//! `media-feature-name-no-unknown` of Stylelint 17.16.0
//! (`lib/rules/media-feature-name-no-unknown/index.mjs`, REF-STYLELINT-SOURCE): a media feature of
//! an `@media` neither known nor prefixed nor custom (`--`), but those its option ignores,
//! reported over its name.

use super::media::feature_names;
use super::prefixed::prefix;
use super::reference::{MEDIA_FEATURE_NAMES, has};
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct MediaFeatureNameNoUnknown {
    /// The features left out (`ignoreMediaFeatureNames`).
    pub ignore: Vec<Matcher>,
}

impl MediaFeatureNameNoUnknown {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

impl SheetRule for MediaFeatureNameNoUnknown {
    fn name(&self) -> &'static str {
        "stylelint/media-feature-name-no-unknown"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unknown media feature name \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if !node.name.eq_ignore_ascii_case("media") {
                continue;
            }
            let start = sheet.params_start(node);
            for named in feature_names(sheet.params(node)) {
                let name = &named.name;
                if name.starts_with("--")
                    || matches_any(&self.ignore, name)
                    || !prefix(name).is_empty()
                    || has(MEDIA_FEATURE_NAMES, &name.to_lowercase())
                {
                    continue;
                }
                let at = start + named.start;
                reports.push(Report::new(at, at + name.len(), "rejected").with(&[name]));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MediaFeatureNameNoUnknown;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/media-feature-name-no-unknown.txt",
            &|config| {
                Ok(Box::new(MediaFeatureNameNoUnknown {
                    ignore: matchers(secondary(config), "ignoreMediaFeatureNames")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (45, 22, 16),
            "{:#?}",
            tally.apart
        );
    }
}
