//! `length-zero-no-unit` of Stylelint 17.16.0 (`lib/rules/length-zero-no-unit/index.mjs`,
//! REF-STYLELINT-SOURCE): a zero length written with its unit (`0px`) in a plain declaration's
//! value or an at-rule's parameters, outside math functions, `line-height`, `flex`, a `font`'s
//! line height and `fr`, reported over the unit and fixed by dropping it. The project's options,
//! stylelint-config-standard-scss's: custom properties and the preludes of `@function` and
//! `@mixin` left as written.

use super::reference::{LENGTH_UNITS, MATH_FUNCTIONS, has};
use super::{
    Context, Matcher, Report, SheetRule, is_standard_at_rule, is_standard_declaration, matches_any,
};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part, unit};

/// The rule and its options.
pub struct LengthZeroNoUnit {
    /// Whether custom properties are left out (`ignore: ["custom-properties"]`).
    pub ignore_custom_properties: bool,
    /// The functions whose arguments are left out (`ignoreFunctions`).
    pub ignore_functions: Vec<Matcher>,
    /// The at-rules whose preludes are left out (`ignorePreludeOfAtRules`).
    pub ignore_prelude_of_at_rules: Vec<Matcher>,
}

impl LengthZeroNoUnit {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        Self {
            ignore_custom_properties: true,
            ignore_functions: Vec::new(),
            ignore_prelude_of_at_rules: vec![Matcher::new("function"), Matcher::new("mixin")],
        }
    }

    /// Checks a value or parameters starting at a byte, for a declaration of a property, the
    /// fixes scoped to the node.
    fn check(
        &self,
        text: &str,
        start: usize,
        property: Option<&str>,
        scope: (usize, usize),
        reports: &mut Vec<Report>,
    ) {
        let parsed = value::parse(text);
        let font = property.is_some_and(|property| property.eq_ignore_ascii_case("font"));
        let mut previous_slash: Vec<(usize, bool)> = Vec::new();
        parsed.walk(&mut |index, part| {
            let after_slash = font && previous_slash.iter().any(|(next, _)| *next == index);
            previous_slash.retain(|(next, _)| *next != index);
            if part.kind == Part::Div && part.value == "/" {
                previous_slash.push((index + 1, true));
            }
            if after_slash {
                return true;
            }
            if part.kind == Part::Function
                && (has(MATH_FUNCTIONS, &part.value.to_lowercase())
                    || matches_any(&self.ignore_functions, &part.value))
            {
                return false;
            }
            if part.kind != Part::Word {
                return true;
            }
            let Some((number, found)) = unit(&part.value) else {
                return true;
            };
            let zero = number.parse::<f64>().is_ok_and(|value| value == 0.0)
                || crate::javascript::parse_float(number) == 0.0;
            if !zero
                || found.is_empty()
                || !has(LENGTH_UNITS, &found.to_lowercase())
                || found.eq_ignore_ascii_case("fr")
            {
                return true;
            }
            let at = start + part.start + number.len();
            let kept = number.strip_prefix('.').unwrap_or(number);
            reports.push(
                Report::new(at, at + found.len(), "rejected")
                    .fixed(start + part.start, start + part.end, kept)
                    .scoped(scope.0, scope.1),
            );
            true
        });
    }
}

impl SheetRule for LengthZeroNoUnit {
    fn name(&self) -> &'static str {
        "stylelint/length-zero-no-unit"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Disallowed unit")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut at_rules = Vec::new();
        let mut declarations = Vec::new();
        for (index, node) in sheet.all(Kind::AtRule) {
            if is_standard_at_rule(node)
                && !matches_any(&self.ignore_prelude_of_at_rules, &node.name)
            {
                let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
                self.check(
                    sheet.params(node),
                    sheet.params_start(node),
                    None,
                    scope,
                    &mut at_rules,
                );
            }
            let _ = index;
        }
        for (index, node) in sheet.all(Kind::Declaration) {
            let property = node.prop.to_lowercase();
            if !is_standard_declaration(sheet, index)
                || property == "line-height"
                || property == "flex"
                || (self.ignore_custom_properties && node.prop.starts_with("--"))
            {
                continue;
            }
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            self.check(
                sheet.value(node),
                sheet.value_start(node),
                Some(&node.prop),
                scope,
                &mut declarations,
            );
        }
        // The at-rules are walked before the declarations, as the original walks them.
        reports.extend(at_rules);
        reports.extend(declarations);
    }
}

#[cfg(test)]
mod tests {
    use super::LengthZeroNoUnit;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/length-zero-no-unit.txt",
            &|config| {
                let options = secondary(config);
                let ignore = options
                    .and_then(|each| each.texts("ignore"))
                    .unwrap_or_default();
                Ok(Box::new(LengthZeroNoUnit {
                    ignore_custom_properties: ignore.iter().any(|each| each == "custom-properties"),
                    ignore_functions: matchers(options, "ignoreFunctions")?,
                    ignore_prelude_of_at_rules: matchers(options, "ignorePreludeOfAtRules")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (77, 41, 6),
            "{:#?}",
            tally.apart
        );
    }
}
