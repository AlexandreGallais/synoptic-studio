//! `value-no-vendor-prefix` of Stylelint 17.16.0 (`lib/rules/value-no-vendor-prefix/index.mjs`,
//! REF-STYLELINT-SOURCE): a prefixed value or function of a plain declaration whose standard form
//! Autoprefixer knows (`-webkit-sticky`, `-moz-calc()`), but those its option ignores, reported
//! over it and fixed by dropping the prefix. The project's option, stylelint-config-standard's:
//! `box` and `inline-box` left out.

use super::prefixed::{property_value, unprefix};
use super::{
    Context, Matcher, Report, SheetRule, is_standard_declaration, is_standard_property, matches_any,
};
use crate::scss::tree::Kind;
use crate::scss::value;

/// The rule and its option.
pub struct ValueNoVendorPrefix {
    /// The values left out (`ignoreValues`).
    pub ignore: Vec<Matcher>,
}

impl ValueNoVendorPrefix {
    /// The project's configuration: `box` and `inline-box` left out.
    #[must_use]
    pub fn project() -> Self {
        Self {
            ignore: vec![Matcher::new("box"), Matcher::new("inline-box")],
        }
    }
}

impl SheetRule for ValueNoVendorPrefix {
    fn name(&self) -> &'static str {
        "stylelint/value-no-vendor-prefix"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Vendor-prefixed value \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Declaration) {
            let lower = node.value.to_lowercase();
            let prefixed = [
                "-webkit-", "-moz-", "-ms-", "-o-", "-xv-", "-apple-", "-wap-", "-khtml-",
            ]
            .iter()
            .any(|prefix| lower.contains(prefix));
            if !prefixed
                || !is_standard_declaration(sheet, index)
                || !is_standard_property(&node.prop)
            {
                continue;
            }
            let start = sheet.value_start(node);
            let end = node.end.unwrap_or(sheet.text.len());
            value::parse(&node.value).walk(&mut |_, part| {
                if property_value(&part.value) && !matches_any(&self.ignore, &part.value) {
                    let at = start + part.start;
                    reports.push(
                        Report::new(at, at + part.value.len(), "rejected")
                            .with(&[&part.value])
                            .fixed(at, at + part.value.len(), unprefix(&part.value))
                            .scoped(node.start, end),
                    );
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ValueNoVendorPrefix;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/value-no-vendor-prefix.txt",
            &|config| {
                Ok(Box::new(ValueNoVendorPrefix {
                    ignore: matchers(secondary(config), "ignoreValues")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (21, 16, 2),
            "{:#?}",
            tally.apart
        );
    }
}
