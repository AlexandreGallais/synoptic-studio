//! `selector-no-vendor-prefix` of Stylelint 17.16.0 (`lib/rules/selector-no-vendor-prefix/index.mjs`,
//! REF-STYLELINT-SOURCE): a prefixed pseudo of a plain rule whose standard form Autoprefixer knows
//! (`::-moz-selection`), but those its option ignores, reported over it and fixed by dropping the
//! prefix.

use super::prefixed::{selector as autoprefixable, unprefix};
use super::{Context, Matcher, Report, SheetRule, is_standard_rule, matches_any};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct SelectorNoVendorPrefix {
    /// The selectors left out (`ignoreSelectors`).
    pub ignore: Vec<Matcher>,
}

impl SelectorNoVendorPrefix {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

impl SheetRule for SelectorNoVendorPrefix {
    fn name(&self) -> &'static str {
        "stylelint/selector-no-vendor-prefix"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Vendor-prefixed selector \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            let lower = node.selector.to_lowercase();
            let prefixed = [
                "-webkit-", "-moz-", "-ms-", "-o-", "-xv-", "-apple-", "-wap-", "-khtml-",
            ]
            .iter()
            .any(|prefix| lower.contains(prefix));
            if !prefixed || !is_standard_rule(node) {
                continue;
            }
            let Ok(parsed) = selector::parse(sheet.selector(node)) else {
                continue;
            };
            for each in parsed.walk() {
                let pseudo = parsed.part(each);
                if pseudo.kind != Part::Pseudo
                    || !autoprefixable(&pseudo.value)
                    || matches_any(&self.ignore, &pseudo.value)
                {
                    continue;
                }
                let at = node.start + pseudo.start;
                let end = at + pseudo.value.len();
                reports.push(
                    Report::new(at, end, "rejected")
                        .with(&[&pseudo.value])
                        .fixed(at, end, unprefix(&pseudo.value))
                        .scoped(node.start, node.end.unwrap_or(sheet.text.len())),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorNoVendorPrefix;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-no-vendor-prefix.txt",
            &|config| {
                Ok(Box::new(SelectorNoVendorPrefix {
                    ignore: matchers(secondary(config), "ignoreSelectors")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (12, 10, 1),
            "{:#?}",
            tally.apart
        );
    }
}
