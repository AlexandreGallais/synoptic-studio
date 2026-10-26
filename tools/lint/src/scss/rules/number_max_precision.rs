//! `number-max-precision` of Stylelint 17.16.0 (`lib/rules/number-max-precision/index.mjs`,
//! REF-STYLELINT-SOURCE): a number of a declaration's value or an at-rule's parameters (but
//! `@import`'s) with more decimals than the limit, outside `url()`, strings, comments and
//! identifiers, its units and properties the options ignore left out, reported over the number.
//! The numbers are read by a reading of ours of the CSS tokens `@csstools/css-tokenizer` gives
//! Stylelint. The project's limit, stylelint-config-standard's, 4.

use super::numbers::numbers;
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct NumberMaxPrecision {
    /// The most decimals.
    pub precision: usize,
    /// The properties left out (`ignoreProperties`).
    pub ignore_properties: Vec<Matcher>,
    /// The units left out (`ignoreUnits`).
    pub ignore_units: Vec<Matcher>,
}

impl NumberMaxPrecision {
    /// The project's configuration: 4.
    pub const PROJECT: Self = Self {
        precision: 4,
        ignore_properties: Vec::new(),
        ignore_units: Vec::new(),
    };
}

/// A number as JavaScript prints it.
fn printed(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e21 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

impl NumberMaxPrecision {
    /// Checks a text starting at a byte.
    fn check_text(&self, text: &str, start: usize, reports: &mut Vec<Report>) {
        for number in numbers(text) {
            if number
                .functions
                .iter()
                .any(|name| name.eq_ignore_ascii_case("url"))
            {
                continue;
            }
            if !number.unit.is_empty() && matches_any(&self.ignore_units, &number.unit) {
                continue;
            }
            let raw = &text[number.start..number.end];
            let decimals = raw.split_once('.').map_or(0, |(_, after)| {
                after.bytes().take_while(u8::is_ascii_digit).count()
            });
            if decimals <= self.precision {
                continue;
            }
            let value = crate::javascript::to_number(raw);
            let fixed = format!("{value:.precision$}", precision = self.precision);
            reports.push(
                Report::new(start + number.start, start + number.end, "expected")
                    .with(&[&printed(value), &fixed]),
            );
        }
    }
}

impl SheetRule for NumberMaxPrecision {
    fn name(&self) -> &'static str {
        "stylelint/number-max-precision"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to be \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut at_rules = Vec::new();
        for (_, node) in sheet.all(Kind::AtRule) {
            let params = sheet.params(node);
            if !node.name.eq_ignore_ascii_case("import") && params.contains('.') {
                self.check_text(params, sheet.params_start(node), &mut at_rules);
            }
        }
        let mut declarations = Vec::new();
        for (_, node) in sheet.all(Kind::Declaration) {
            let value = sheet.value(node);
            if value.contains('.') && !matches_any(&self.ignore_properties, &node.prop) {
                self.check_text(value, sheet.value_start(node), &mut declarations);
            }
        }
        reports.extend(at_rules);
        reports.extend(declarations);
    }
}

#[cfg(test)]
mod tests {
    use super::NumberMaxPrecision;
    use crate::scss::rules::tester::{Js, matchers, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/number-max-precision.txt",
            &|config| {
                let Js::Number(precision) = primary(config) else {
                    return Err("an option value not read");
                };
                let options = secondary(config);
                if options
                    .and_then(|each| each.get("insideFunctions"))
                    .is_some()
                {
                    return Err("an option the catalogue does not set: insideFunctions");
                }
                Ok(Box::new(NumberMaxPrecision {
                    precision: format!("{precision}").parse().unwrap_or(0),
                    ignore_properties: matchers(options, "ignoreProperties")?,
                    ignore_units: matchers(options, "ignoreUnits")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (37, 18, 9),
            "{:#?}",
            tally.apart
        );
    }
}
