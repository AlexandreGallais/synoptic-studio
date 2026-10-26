//! `property-no-deprecated` of Stylelint 17.16.0 (`lib/rules/property-no-deprecated/index.mjs`,
//! REF-STYLELINT-SOURCE): a deprecated property of a plain declaration (`clip`, `word-wrap`, the
//! old flexbox's `-*-box-*`…), but those its option ignores and `-webkit-box-orient: vertical`,
//! reported over the property; one with a standard form fixed by renaming it, `page-break-*`'s
//! `always` made `page`.

use super::{Context, Matcher, Report, SheetRule, is_standard_declaration, matches_any};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its option.
pub struct PropertyNoDeprecated {
    /// The properties left out (`ignoreProperties`).
    pub ignore: Vec<Matcher>,
}

impl PropertyNoDeprecated {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

/// The deprecated properties and their standard forms, none for those without one.
const DEPRECATED: &[(&str, Option<&str>)] = &[
    ("-khtml-box-align", Some("align-items")),
    ("-khtml-box-direction", None),
    ("-khtml-box-flex", Some("flex-grow")),
    ("-khtml-box-lines", None),
    ("-khtml-box-ordinal-group", Some("order")),
    ("-khtml-box-orient", None),
    ("-khtml-box-pack", None),
    ("-khtml-user-modify", None),
    ("-moz-box-align", Some("align-items")),
    ("-moz-box-direction", None),
    ("-moz-box-flex", Some("flex-grow")),
    ("-moz-box-lines", None),
    ("-moz-box-ordinal-group", Some("order")),
    ("-moz-box-orient", None),
    ("-moz-box-pack", None),
    ("-moz-user-modify", None),
    ("-ms-box-align", Some("align-items")),
    ("-ms-box-direction", None),
    ("-ms-box-flex", Some("flex-grow")),
    ("-ms-box-lines", None),
    ("-ms-box-ordinal-group", Some("order")),
    ("-ms-box-orient", None),
    ("-ms-box-pack", None),
    ("-webkit-box-align", Some("align-items")),
    ("-webkit-box-direction", None),
    ("-webkit-box-flex", Some("flex-grow")),
    ("-webkit-box-lines", None),
    ("-webkit-box-ordinal-group", Some("order")),
    ("-webkit-box-orient", None),
    ("-webkit-box-pack", None),
    ("-webkit-user-modify", None),
    ("grid-column-gap", Some("column-gap")),
    ("grid-gap", Some("gap")),
    ("grid-row-gap", Some("row-gap")),
    ("ime-mode", None),
    ("page-break-after", Some("break-after")),
    ("page-break-before", Some("break-before")),
    ("page-break-inside", Some("break-inside")),
    ("position-try-options", Some("position-try-fallbacks")),
    ("scroll-snap-coordinate", None),
    ("scroll-snap-destination", None),
    ("scroll-snap-margin-bottom", Some("scroll-margin-bottom")),
    ("scroll-snap-margin-left", Some("scroll-margin-left")),
    ("scroll-snap-margin-right", Some("scroll-margin-right")),
    ("scroll-snap-margin-top", Some("scroll-margin-top")),
    ("scroll-snap-margin", Some("scroll-margin")),
    ("scroll-snap-points-x", None),
    ("scroll-snap-points-y", None),
    ("scroll-snap-type-x", None),
    ("scroll-snap-type-y", None),
    ("word-wrap", Some("overflow-wrap")),
    ("clip", None),
];

impl SheetRule for PropertyNoDeprecated {
    fn name(&self) -> &'static str {
        "stylelint/property-no-deprecated"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected \"{}\" to be \"{}\""),
            ("rejected", "Deprecated property \"{}\""),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Declaration) {
            let prop = &node.prop;
            let lower = prop.to_lowercase();
            if !is_standard_declaration(sheet, index) || matches_any(&self.ignore, prop) {
                continue;
            }
            let Some((_, mapped)) = DEPRECATED.iter().find(|(name, _)| *name == lower) else {
                continue;
            };
            if lower == "-webkit-box-orient" && node.value.eq_ignore_ascii_case("vertical") {
                continue;
            }
            let report = Report::new(node.start, node.start + prop.len(), "rejected").with(&[prop]);
            let Some(mapped) = mapped else {
                reports.push(report);
                continue;
            };
            let raw = sheet.value(node);
            let value_start = sheet.value_start(node);
            let mut value_text = raw.to_owned();
            if lower == "page-break-before" || lower == "page-break-after" {
                let parsed = value::parse(raw);
                let mut edits: Vec<(usize, usize)> = Vec::new();
                parsed.walk(&mut |_, part| {
                    if part.kind == Part::Word && part.value.eq_ignore_ascii_case("always") {
                        edits.push((part.start, part.end));
                    }
                    true
                });
                for (from, to) in edits.into_iter().rev() {
                    value_text.replace_range(from..to, "page");
                }
            }
            let between = &sheet.text[node.start + prop.len()..value_start];
            reports.push(Report {
                key: "expected",
                ..report.with(&[prop, mapped]).fixed(
                    node.start,
                    value_start + raw.len(),
                    format!("{mapped}{between}{value_text}"),
                )
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PropertyNoDeprecated;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/property-no-deprecated.txt",
            &|config| {
                Ok(Box::new(PropertyNoDeprecated {
                    ignore: matchers(secondary(config), "ignoreProperties")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (6, 7, 0),
            "{:#?}",
            tally.apart
        );
    }
}
