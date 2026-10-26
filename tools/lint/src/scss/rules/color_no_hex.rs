//! `color-no-hex` of Stylelint 17.16.0 (`lib/rules/color-no-hex/index.mjs`,
//! REF-STYLELINT-SOURCE): each word of a declaration's value written `#` and letters or digits,
//! outside `url()` and the functions its option ignores, reported over its text. The tokens'
//! files are left out by the project's scope (`scss::scopes`).

use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its option.
pub struct ColorNoHex {
    /// The functions whose arguments are left out (`ignoreFunctions`).
    pub ignore_functions: Vec<Matcher>,
}

impl ColorNoHex {
    /// The project's configuration: no function left out.
    pub const PROJECT: Self = Self {
        ignore_functions: Vec::new(),
    };
}

/// Whether a word is `#` and letters or digits (`/^#[\da-z]+$/i`).
fn is_hex(word: &str) -> bool {
    word.len() > 1
        && word.starts_with('#')
        && word[1..].bytes().all(|byte| byte.is_ascii_alphanumeric())
}

impl SheetRule for ColorNoHex {
    fn name(&self) -> &'static str {
        "stylelint/color-no-hex"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Disallowed hex color \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let text = sheet.value(node);
            if !text.contains('#') {
                continue;
            }
            let start = sheet.value_start(node);
            let parsed = value::parse(text);
            parsed.walk(&mut |_, part| {
                if part.kind == Part::Function
                    && (part.value.eq_ignore_ascii_case("url")
                        || matches_any(&self.ignore_functions, &part.value))
                {
                    return false;
                }
                if part.kind == Part::Word && is_hex(&part.value) {
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
    use super::ColorNoHex;
    use crate::scss::rules::tester::{matchers, reports, run, secondary};

    #[test]
    fn refuses_a_hex_colour_of_one_digit() {
        // Own case: `#a` matches `/^#[\da-z]+$/i`, one character after `#` enough.
        assert_eq!(
            reports(&ColorNoHex::PROJECT, "a { color: #a; }"),
            [(1, 12, "Disallowed hex color \"#a\"".to_owned())]
        );
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/color-no-hex.txt",
            &|config| {
                Ok(Box::new(ColorNoHex {
                    ignore_functions: matchers(secondary(config), "ignoreFunctions")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (10, 9, 8),
            "{:#?}",
            tally.apart
        );
    }
}
