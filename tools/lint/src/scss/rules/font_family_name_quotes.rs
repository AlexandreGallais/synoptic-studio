//! `font-family-name-quotes` of Stylelint 17.16.0 (`lib/rules/font-family-name-quotes/index.mjs`,
//! REF-STYLELINT-SOURCE): a family of a `font` or `font-family` declaration quoted or not against
//! its option, generic and system families never quoted, reported over its name and fixed by
//! adding or removing the quotes. The project's option, stylelint-config-standard's:
//! `"always-where-recommended"`, a name quoted when it holds anything but letters and `-`.

use super::font_families::find;
use super::reference::{FONT_FAMILY_KEYWORDS, PREFIXED_SYSTEM_FONTS, has};
use super::{Context, Report, SheetRule, is_standard_value};
use crate::javascript::JsText;
use crate::scss::tree::Kind;

/// When quotes are wanted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quotes {
    /// Always, but for keywords.
    AlwaysUnlessKeyword,
    /// Where a name holds anything but letters and `-`.
    AlwaysWhereRecommended,
    /// Where a name could not be read unquoted.
    AlwaysWhereRequired,
}

/// The rule and its option.
pub struct FontFamilyNameQuotes {
    /// When quotes are wanted.
    pub quotes: Quotes,
}

impl FontFamilyNameQuotes {
    /// The project's configuration: `"always-where-recommended"`.
    pub const PROJECT: Self = Self {
        quotes: Quotes::AlwaysWhereRecommended,
    };
}

/// Whether quotes are recommended: anything but letters and `-` (`quotesRecommended`).
fn recommended(family: &str) -> bool {
    family.is_empty()
        || !family
            .bytes()
            .all(|byte| byte.is_ascii_alphabetic() || byte == b'-')
}

/// Whether quotes are required: a word starting with a digit or `--`, or holding a character no
/// identifier holds (`quotesRequired`).
fn required(family: &str) -> bool {
    family.js_split_whitespace().any(|word| {
        let digit = word
            .strip_prefix('-')
            .unwrap_or(word)
            .starts_with(|c: char| c.is_ascii_digit());
        digit
            || word.starts_with("--")
            || !word
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c >= '\u{a0}')
    })
}

impl SheetRule for FontFamilyNameQuotes {
    fn name(&self) -> &'static str {
        "stylelint/font-family-name-quotes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected quotes around \"{}\""),
            ("rejected", "Expected no quotes around \"{}\""),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let property = node.prop.to_ascii_lowercase();
            if (property != "font" && property != "font-family") || !is_standard_value(&node.value)
            {
                continue;
            }
            let start = sheet.value_start(node);
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            for family in find(&node.value) {
                let name = &family.value;
                let quoted = family.quote.is_some();
                let raw = family
                    .quote
                    .map_or_else(|| name.clone(), |quote| format!("{quote}{name}{quote}"));
                if raw.to_lowercase().starts_with("var(") {
                    continue;
                }
                let keyword = has(FONT_FAMILY_KEYWORDS, &name.to_lowercase())
                    || has(PREFIXED_SYSTEM_FONTS, name)
                    || name == "BlinkMacSystemFont";
                let key = if keyword {
                    quoted.then_some("rejected")
                } else {
                    let wanted = match self.quotes {
                        Quotes::AlwaysUnlessKeyword => true,
                        Quotes::AlwaysWhereRecommended => recommended(name),
                        Quotes::AlwaysWhereRequired => required(name),
                    };
                    match (wanted, quoted) {
                        (true, false) => Some("expected"),
                        (false, true) if self.quotes != Quotes::AlwaysUnlessKeyword => {
                            Some("rejected")
                        }
                        _ => None,
                    }
                };
                let Some(key) = key else {
                    continue;
                };
                let (from, to) = sheet.word_range(node, &raw);
                let at = start + family.start;
                let (fix_end, text) = if key == "expected" {
                    (at + name.len(), format!("\"{name}\""))
                } else {
                    (at + name.len() + 2, name.clone())
                };
                reports.push(
                    Report::new(from, to, key)
                        .with(&[name])
                        .fixed(at, fix_end, text)
                        .scoped(scope.0, scope.1),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FontFamilyNameQuotes, Quotes, recommended, required};
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/font-family-name-quotes.txt",
            &|config| {
                let quotes = match primary(config) {
                    Js::Text(text) if text == "always-unless-keyword" => {
                        Quotes::AlwaysUnlessKeyword
                    }
                    Js::Text(text) if text == "always-where-required" => {
                        Quotes::AlwaysWhereRequired
                    }
                    _ => Quotes::AlwaysWhereRecommended,
                };
                Ok(Box::new(FontFamilyNameQuotes { quotes }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (65, 80, 0),
            "{:#?}",
            tally.apart
        );
        assert!(
            recommended("Times New Roman")
                && !recommended("Arial")
                && required("Font 1")
                && !required("Arial")
        );
    }
}
