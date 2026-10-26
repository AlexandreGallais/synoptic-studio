//! `font-family-no-duplicate-names` of Stylelint 17.16.0
//! (`lib/rules/font-family-no-duplicate-names/index.mjs`, REF-STYLELINT-SOURCE): a family named
//! again in a `font` or `font-family` declaration (a generic keyword in any case), but those its
//! option ignores, reported over the repeat.

use super::font_families::find;
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::javascript::JsText;
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct FontFamilyNoDuplicateNames {
    /// The families left out (`ignoreFontFamilyNames`).
    pub ignore: Vec<Matcher>,
}

impl FontFamilyNoDuplicateNames {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

impl SheetRule for FontFamilyNoDuplicateNames {
    fn name(&self) -> &'static str {
        "stylelint/font-family-no-duplicate-names"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Duplicate font-family name \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let property = node.prop.to_ascii_lowercase();
            if property != "font" && property != "font-family" {
                continue;
            }
            let start = sheet.value_start(node);
            let mut keywords: Vec<String> = Vec::new();
            let mut names: Vec<String> = Vec::new();
            for family in find(&node.value) {
                let name = family.value.js_trim().to_owned();
                if matches_any(&self.ignore, &name) {
                    continue;
                }
                let raw = family.quote.map_or(name.len(), |_| name.len() + 2);
                let seen = if family.is_keyword() {
                    let found = keywords.contains(&name.to_lowercase());
                    keywords.push(name.clone());
                    found
                } else {
                    let found = names.contains(&name);
                    names.push(name.clone());
                    found
                };
                if seen {
                    let at = start + family.start;
                    reports.push(Report::new(at, at + raw, "rejected").with(&[&name]));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FontFamilyNoDuplicateNames;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/font-family-no-duplicate-names.txt",
            &|config| {
                Ok(Box::new(FontFamilyNoDuplicateNames {
                    ignore: matchers(secondary(config), "ignoreFontFamilyNames")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (11, 9, 0),
            "{:#?}",
            tally.apart
        );
    }
}
