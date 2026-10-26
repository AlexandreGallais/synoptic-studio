//! `font-family-no-missing-generic-family-keyword` of Stylelint 17.16.0
//! (`lib/rules/font-family-no-missing-generic-family-keyword/index.mjs`, REF-STYLELINT-SOURCE): a
//! `font` or `font-family` declaration naming families but no generic one, outside `@font-face`
//! and `@font-palette-values`, a system font, a variable last and the families its option ignores
//! left out, reported over the last family.

use super::font_families::find;
use super::reference::{SYSTEM_FONT_KEYWORDS, has};
use super::selectors::comma_list;
use super::{Context, Matcher, Report, SheetRule, is_standard_value, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct FontFamilyNoMissingGenericFamilyKeyword {
    /// The families whose presence leaves the declaration out (`ignoreFontFamilies`).
    pub ignore: Vec<Matcher>,
}

impl FontFamilyNoMissingGenericFamilyKeyword {
    /// The project's configuration: none.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

impl SheetRule for FontFamilyNoMissingGenericFamilyKeyword {
    fn name(&self) -> &'static str {
        "stylelint/font-family-no-missing-generic-family-keyword"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Missing generic font family")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let property = node.prop.to_ascii_lowercase();
            if property != "font" && property != "font-family" {
                continue;
            }
            let parent = sheet.node(node.parent.unwrap_or(0));
            if parent.is(Kind::AtRule)
                && matches!(
                    parent.name.to_lowercase().as_str(),
                    "font-face" | "font-palette-values"
                )
            {
                continue;
            }
            if node.prop == "font" && has(SYSTEM_FONT_KEYWORDS, &node.value.to_lowercase()) {
                continue;
            }
            let last = comma_list(&node.value).pop();
            if last.is_some_and(|last| {
                last.to_lowercase().starts_with("var(") || !is_standard_value(&last)
            }) {
                continue;
            }
            let families = find(&node.value);
            let Some(last) = families.last() else {
                continue;
            };
            if families
                .iter()
                .any(|family| family.is_keyword() || matches_any(&self.ignore, &family.value))
            {
                continue;
            }
            let start = sheet.value_start(node);
            reports.push(Report::new(
                start + last.start,
                start + last.end,
                "rejected",
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FontFamilyNoMissingGenericFamilyKeyword;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/font-family-no-missing-generic-family-keyword.txt",
            &|config| {
                Ok(Box::new(FontFamilyNoMissingGenericFamilyKeyword {
                    ignore: matchers(secondary(config), "ignoreFontFamilies")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (50, 16, 0),
            "{:#?}",
            tally.apart
        );
    }
}
