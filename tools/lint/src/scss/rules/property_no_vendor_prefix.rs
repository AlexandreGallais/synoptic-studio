//! `property-no-vendor-prefix` of Stylelint 17.16.0 (`lib/rules/property-no-vendor-prefix/index.mjs`,
//! REF-STYLELINT-SOURCE): a prefixed property whose standard form Autoprefixer knows
//! (`-webkit-transform`), but those its option ignores and a `-webkit-background-size` whose
//! values the standard form reads otherwise, reported over the property and fixed by dropping
//! the prefix. A prefix still needed (Safari's `-webkit-user-select`) takes a justified directive.

use super::prefixed::{property as autoprefixable, unprefix};
use super::reference::{BASIC_KEYWORDS, has};
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct PropertyNoVendorPrefix {
    /// The properties left out (`ignoreProperties`).
    pub ignore: Vec<Matcher>,
}

impl PropertyNoVendorPrefix {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

/// Whether `-webkit-background-size`'s values mean the same unprefixed: each one of two values,
/// or `auto` or a CSS-wide keyword alone.
fn is_safe_background_size(value: &str) -> bool {
    value.split(',').all(|background| {
        let values: Vec<&str> = background.split_whitespace().collect();
        match values.as_slice() {
            [_, _] => true,
            [first] => *first == "auto" || has(BASIC_KEYWORDS, first),
            _ => false,
        }
    })
}

impl SheetRule for PropertyNoVendorPrefix {
    fn name(&self) -> &'static str {
        "stylelint/property-no-vendor-prefix"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Vendor-prefixed property \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let prop = &node.prop;
            if matches_any(&self.ignore, prop)
                || !prop.starts_with('-')
                || prop.starts_with("--")
                || !autoprefixable(prop)
                || (prop == "-webkit-background-size" && !is_safe_background_size(&node.value))
            {
                continue;
            }
            let (start, end) = sheet.word_range(node, prop);
            reports.push(
                Report::new(start, end, "rejected")
                    .with(&[prop])
                    .fixed(node.start, node.start + prop.len(), unprefix(prop))
                    .scoped(node.start, node.end.unwrap_or(sheet.text.len())),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PropertyNoVendorPrefix;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/property-no-vendor-prefix.txt",
            &|config| {
                Ok(Box::new(PropertyNoVendorPrefix {
                    ignore: matchers(secondary(config), "ignoreProperties")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (16, 19, 0),
            "{:#?}",
            tally.apart
        );
    }
}
