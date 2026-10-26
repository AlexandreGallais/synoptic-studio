//! `media-type-no-deprecated` of Stylelint 17.16.0 (`lib/rules/media-type-no-deprecated/index.mjs`,
//! REF-STYLELINT-SOURCE): a deprecated media type (`tv`, `handheld`…) of a plain `@media`, but
//! those its option ignores, reported over it.

use super::media::media_types;
use super::reference::{DEPRECATED_MEDIA_TYPES_NAMES, has};
use super::{Context, Matcher, Report, SheetRule, is_standard_at_rule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct MediaTypeNoDeprecated {
    /// The media types left out (`ignoreMediaTypes`).
    pub ignore: Vec<Matcher>,
}

impl MediaTypeNoDeprecated {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

impl SheetRule for MediaTypeNoDeprecated {
    fn name(&self) -> &'static str {
        "stylelint/media-type-no-deprecated"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Deprecated media type \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if !node.name.eq_ignore_ascii_case("media") || !is_standard_at_rule(node) {
                continue;
            }
            let start = sheet.params_start(node);
            for named in media_types(sheet.params(node)) {
                let name = &named.name;
                if !has(DEPRECATED_MEDIA_TYPES_NAMES, &name.to_lowercase())
                    || matches_any(&self.ignore, name)
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
    use super::MediaTypeNoDeprecated;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/media-type-no-deprecated.txt",
            &|config| {
                Ok(Box::new(MediaTypeNoDeprecated {
                    ignore: matchers(secondary(config), "ignoreMediaTypes")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (10, 12, 0),
            "{:#?}",
            tally.apart
        );
    }
}
