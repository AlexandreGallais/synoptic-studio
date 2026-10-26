//! `color-named` of Stylelint 17.16.0 (`lib/rules/color-named/index.mjs`, REF-STYLELINT-SOURCE)
//! with its option `"never"`: each named colour (`red`) written as a word of a declaration's
//! value, outside the properties taking custom identifiers, the properties and functions its
//! options ignore, nameless parentheses and interpolations, reported over it. The option
//! `"always-where-possible"`, which needs `colord` to name a colour, is not ported. The tokens'
//! files are left out by the project's scope.

use super::reference::{ACCEPT_CUSTOM_IDENTS_PROPERTIES, NAMED_COLORS_KEYWORDS, has};
use super::{Context, Matcher, Report, SheetRule, is_standard_value, matches_any};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its options.
pub struct ColorNamed {
    /// The properties left out (`ignoreProperties`).
    pub ignore_properties: Vec<Matcher>,
    /// The functions whose arguments are left out (`ignoreFunctions`).
    pub ignore_functions: Vec<Matcher>,
    /// Whether every function's arguments are left out (`ignore: ["inside-function"]`).
    pub ignore_inside_function: bool,
}

impl ColorNamed {
    /// The project's configuration: `"never"`, nothing left out.
    pub const PROJECT: Self = Self {
        ignore_properties: Vec::new(),
        ignore_functions: Vec::new(),
        ignore_inside_function: false,
    };
}

/// Whether a part is a standard function, or not a function (`isStandardSyntaxFunction`): a
/// function without a name is a Sass list, `#{`, `${` and a backtick start no CSS.
#[must_use]
pub fn is_standard_function(part: &value::Part) -> bool {
    !(part.value.is_empty()
        || part.value.starts_with("#{")
        || part.value.starts_with("${")
        || part.value.starts_with('`'))
}

impl SheetRule for ColorNamed {
    fn name(&self) -> &'static str {
        "stylelint/color-named"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected \"{}\" to be \"{}\""),
            ("rejected", "Disallowed named color \"{}\""),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if has(ACCEPT_CUSTOM_IDENTS_PROPERTIES, &node.prop)
                || matches_any(&self.ignore_properties, &node.prop)
            {
                continue;
            }
            let start = sheet.value_start(node);
            value::parse(&node.value).walk(&mut |_, part| {
                if part.kind == Part::Function
                    && (self.ignore_inside_function
                        || matches_any(&self.ignore_functions, &part.value))
                {
                    return false;
                }
                if !is_standard_function(part) {
                    return false;
                }
                if is_standard_value(&part.value)
                    && part.kind == Part::Word
                    && has(NAMED_COLORS_KEYWORDS, &part.value.to_lowercase())
                {
                    let at = start + part.start;
                    reports.push(
                        Report::new(at, at + part.value.len(), "rejected").with(&[&part.value]),
                    );
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColorNamed;
    use crate::scss::rules::tester::{Js, matchers, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/color-named.txt",
            &|config| {
                if primary(config) != &Js::Text("never".to_owned()) {
                    return Err("an option the catalogue does not set: always-where-possible");
                }
                let options = secondary(config);
                let ignore = options
                    .and_then(|each| each.texts("ignore"))
                    .unwrap_or_default();
                Ok(Box::new(ColorNamed {
                    ignore_properties: matchers(options, "ignoreProperties")?,
                    ignore_functions: matchers(options, "ignoreFunctions")?,
                    ignore_inside_function: ignore.iter().any(|each| each == "inside-function"),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (34, 15, 45),
            "{:#?}",
            tally.apart
        );
    }
}
