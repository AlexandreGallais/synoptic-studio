//! `unit-no-unknown` of Stylelint 17.16.0 (`lib/rules/unit-no-unknown/index.mjs`,
//! REF-STYLELINT-SOURCE): a dimension of a plain declaration's value or an `@media`'s parameters
//! whose unit CSS does not know (`10pixels`), `x` known only in `image-set()`, `image-resolution`
//! and a resolution media feature, `url()` and the functions and units its options ignore left
//! out, reported over the unit. The dimensions are read by a reading of ours of the CSS tokens.

use super::numbers::numbers;
use super::prefixed::prefix;
use super::reference::{UNITS, has};
use super::{
    Context, Matcher, Report, SheetRule, is_standard_at_rule, is_standard_declaration,
    is_standard_value, matches_any,
};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct UnitNoUnknown {
    /// The units left out (`ignoreUnits`).
    pub ignore_units: Vec<Matcher>,
    /// The functions whose arguments are left out (`ignoreFunctions`).
    pub ignore_functions: Vec<Matcher>,
}

impl UnitNoUnknown {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self {
        ignore_units: Vec::new(),
        ignore_functions: Vec::new(),
    };

    /// Checks a text starting at a byte; `x` known from the start when `allow_x`, or from a
    /// resolution media feature on.
    fn check_text(
        &self,
        text: &str,
        start: usize,
        allow_x: bool,
        media: bool,
        reports: &mut Vec<Report>,
    ) {
        let has_dimension = text.as_bytes().windows(2).any(|pair| {
            pair[0].is_ascii_digit()
                && (pair[1] == b'%'
                    || pair[1] == b'-'
                    || pair[1].is_ascii_alphanumeric()
                    || pair[1] == b'_')
        });
        if !has_dimension {
            return;
        }
        // `x` is a resolution inside the parentheses of a resolution media feature.
        let resolution = |before: usize| {
            let mut depth = 0;
            let open = text[..before]
                .char_indices()
                .rev()
                .find_map(|(at, character)| {
                    match character {
                        ')' => depth += 1,
                        '(' if depth == 0 => return Some(at),
                        '(' => depth -= 1,
                        _ => {}
                    }
                    None
                });
            open.is_some_and(|open| text[open..before].to_lowercase().contains("resolution"))
        };
        for number in numbers(text) {
            let unit = number.unit.to_lowercase();
            if unit.is_empty() || unit == "%" {
                continue;
            }
            if number.functions.iter().any(|name| {
                name.eq_ignore_ascii_case("url") || matches_any(&self.ignore_functions, name)
            }) || matches_any(&self.ignore_units, &number.unit)
            {
                continue;
            }
            let image_set = number.functions.iter().any(|name| {
                let lower = name.to_lowercase();
                lower[prefix(&lower).len()..] == *"image-set"
            });
            let x_known = allow_x || image_set || (media && resolution(number.start));
            if (unit == "x" && x_known) || (has(UNITS, &unit) && unit != "x") {
                continue;
            }
            let at = start + number.end;
            reports.push(Report::new(at, at + unit.len(), "rejected").with(&[&number.unit]));
        }
    }
}

impl SheetRule for UnitNoUnknown {
    fn name(&self) -> &'static str {
        "stylelint/unit-no-unknown"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unknown unit \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut at_rules = Vec::new();
        for (_, node) in sheet.all(Kind::AtRule) {
            if node.name.eq_ignore_ascii_case("media") && is_standard_at_rule(node) {
                self.check_text(
                    sheet.params(node),
                    sheet.params_start(node),
                    false,
                    true,
                    &mut at_rules,
                );
            }
        }
        let mut declarations = Vec::new();
        for (index, node) in sheet.all(Kind::Declaration) {
            let parent = sheet.node(node.parent.unwrap_or(0));
            let unicode_range = node.prop.eq_ignore_ascii_case("unicode-range")
                && parent.is(Kind::AtRule)
                && parent.name.eq_ignore_ascii_case("font-face");
            let value = sheet.value(node);
            if !is_standard_declaration(sheet, index) || unicode_range || !is_standard_value(value)
            {
                continue;
            }
            let image_resolution = node.prop.eq_ignore_ascii_case("image-resolution");
            self.check_text(
                value,
                sheet.value_start(node),
                image_resolution,
                false,
                &mut declarations,
            );
        }
        reports.extend(at_rules);
        reports.extend(declarations);
    }
}

#[cfg(test)]
mod tests {
    use super::UnitNoUnknown;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/unit-no-unknown.txt",
            &|config| {
                let options = secondary(config);
                Ok(Box::new(UnitNoUnknown {
                    ignore_units: matchers(options, "ignoreUnits")?,
                    ignore_functions: matchers(options, "ignoreFunctions")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (87, 36, 4),
            "{:#?}",
            tally.apart
        );
    }
}
