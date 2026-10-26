//! `function-disallowed-list` of stylelint-scss 7.3.0 (`src/rules/function-disallowed-list/index.js`,
//! REF-STYLELINT-SOURCE): a Sass function called in a declaration's value or an at-rule's params,
//! not CSS's own (`isNativeCssFunction`), whose name without its module matches one of the
//! option's names or patterns, reported over the first appearance of that name in the node; the
//! name a `@function`, `@mixin` or `@include` declares or calls is no call. The project refuses
//! Sass's colour functions outside the tokens' files, as Stylelint's rule refuses CSS's (AUD-060).

use super::scss_dollar_variable_no_missing_interpolation::is_native;
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::scss::tree::{Kind, Node};
use crate::scss::value::{self, Kind as Part};

/// The rule and its option.
pub struct FunctionDisallowedList {
    /// The functions refused, by their names without their module.
    pub names: Vec<Matcher>,
}

impl FunctionDisallowedList {
    /// The project's configuration: Sass's functions making a colour, those of `sass:color`
    /// (`color.adjust`, `color.mix`…) and the global ones (`mix`, `lighten`…); CSS's own
    /// (`saturate()`, `invert()`, `grayscale()` of `filter`) are left to Stylelint's rule.
    #[must_use]
    pub fn project() -> Self {
        Self {
            names: vec![Matcher::new(
                "/^(adjust|change|scale|mix|complement|grayscale|invert|to-space|to-gamut|adjust-color|change-color|scale-color|adjust-hue|lighten|darken|saturate|desaturate|opacify|fade-in|transparentize|fade-out)$/",
            )],
        }
    }

    /// The reports of a value or params (`checkValue`), the first function skipped when it is the
    /// name a `@function`, `@mixin` or `@include` declares or calls.
    fn check_value(
        &self,
        sheet: &Context,
        node: &Node,
        text: &str,
        skip_first: bool,
        reports: &mut Vec<Report>,
    ) {
        if text.is_empty() {
            return;
        }
        let mut skipped = !skip_first;
        value::parse(text).walk(&mut |_, part| {
            if part.kind != Part::Function {
                return true;
            }
            if !skipped {
                skipped = true;
                return true;
            }
            if is_native(&part.value) || part.value.is_empty() {
                return true;
            }
            let bare = part
                .value
                .find('.')
                .map_or(part.value.as_str(), |dot| &part.value[dot + 1..]);
            for matcher in &self.names {
                if matches_any(std::slice::from_ref(matcher), bare) {
                    let (start, end) = sheet.word_range(node, bare);
                    reports.push(Report::new(start, end, "rejected").with(&[bare]));
                }
            }
            true
        });
    }
}

impl SheetRule for FunctionDisallowedList {
    fn name(&self) -> &'static str {
        "scss/function-disallowed-list"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected function \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            self.check_value(sheet, node, &node.value, false, reports);
        }
        for (_, node) in sheet.all(Kind::AtRule) {
            let named =
                ["function", "include", "mixin"].contains(&node.name.to_lowercase().as_str());
            self.check_value(sheet, node, sheet.params(node), named, reports);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionDisallowedList;
    use crate::scss::rules::tester::{Js, matchers, primary, reports, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/function-disallowed-list.txt",
            &|config| {
                let wrapped = Js::Object(vec![("names".to_owned(), primary(config).clone())]);
                Ok(Box::new(FunctionDisallowedList {
                    names: matchers(Some(&wrapped), "names")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (6, 13, 0),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn refuses_sass_s_colour_functions_and_leaves_css_s_own() {
        let code = "@use \"sass:color\";\na {\n  color: color.adjust($a, $lightness: 10%);\n  background: mix($a, $b);\n  border-color: lighten($a, 5%);\n  filter: saturate(2) invert(1);\n  transform: scale(2);\n  width: math.div(1px, 2);\n}\n";

        // Own case: `color.adjust`, `mix` and `lighten` make colours (lines 3 to 5, each name
        // at its first appearance in its declaration); `saturate()`, `invert()` and `scale()`
        // are CSS's own, `math.div` no colour.
        assert_eq!(
            reports(&FunctionDisallowedList::project(), code),
            [
                (3, 16, "Unexpected function \"adjust\"".to_owned()),
                (4, 15, "Unexpected function \"mix\"".to_owned()),
                (5, 17, "Unexpected function \"lighten\"".to_owned()),
            ]
        );
    }
}
