//! `value-keyword-case` of Stylelint 17.16.0 (`lib/rules/value-keyword-case/index.mjs`,
//! REF-STYLELINT-SOURCE): a keyword of a plain declaration's value not in lower case (`"lower"`,
//! the project's) or upper case (`"upper"`), the names a property takes (animation, font, grid
//! area, counter, list style names), system colours, dimensions, operators, values holding `#`,
//! `url()`, `var()`, `counter()`, `attr()` and what its options ignore left out, reported over it
//! and fixed by rewriting it; with `camelCaseSvgKeywords`, `currentColor` and its kin kept so.

use super::reference::{
    ANIMATION_NAME_KEYWORDS, ANIMATION_SHORTHAND_KEYWORDS, CAMEL_CASE_KEYWORDS,
    COUNTER_INCREMENT_KEYWORDS, COUNTER_RESET_KEYWORDS, FONT_FAMILY_KEYWORDS,
    FONT_SHORTHAND_KEYWORDS, GRID_AREA_KEYWORDS, GRID_COLUMN_KEYWORDS, GRID_ROW_KEYWORDS,
    LIST_STYLE_SHORTHAND_KEYWORDS, LIST_STYLE_TYPE_KEYWORDS, SYSTEM_COLORS_KEYWORDS, has,
};
use super::{Context, Matcher, Report, SheetRule, is_standard_value, matches_any};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part, unit};

/// The rule and its options.
pub struct ValueKeywordCase {
    /// Whether lower case is wanted (`"lower"`), else upper case.
    pub lower: bool,
    /// Whether the camel-case SVG keywords keep their case (`camelCaseSvgKeywords`).
    pub camel_case_svg_keywords: bool,
    /// The keywords left out (`ignoreKeywords`).
    pub ignore_keywords: Vec<Matcher>,
    /// The properties left out (`ignoreProperties`).
    pub ignore_properties: Vec<Matcher>,
    /// The functions whose arguments are left out (`ignoreFunctions`).
    pub ignore_functions: Vec<Matcher>,
}

impl ValueKeywordCase {
    /// The project's configuration: `"lower"`.
    pub const PROJECT: Self = Self {
        lower: true,
        camel_case_svg_keywords: false,
        ignore_keywords: Vec::new(),
        ignore_properties: Vec::new(),
        ignore_functions: Vec::new(),
    };

    /// The keyword a word should be.
    fn expected(&self, keyword: &str) -> String {
        if !self.lower {
            return keyword.to_uppercase();
        }
        let lower = keyword.to_lowercase();
        if self.camel_case_svg_keywords
            && let Some(camel) = CAMEL_CASE_KEYWORDS
                .iter()
                .flat_map(|list| list.iter())
                .find(|camel| camel.to_lowercase() == lower)
        {
            return (*camel).to_owned();
        }
        lower
    }
}

/// Whether a value counts as a custom name of a counter property: neither its keywords nor a
/// number.
fn is_counter_name(value: &str, keywords: super::reference::Set) -> bool {
    !(has(keywords, value) || crate::javascript::parse_int(value).is_finite())
}

/// Whether a property's value names things where this word is no keyword.
fn names_something(property: &str, lower: &str) -> bool {
    match property {
        "animation" => {
            !has(ANIMATION_SHORTHAND_KEYWORDS, lower) && !has(ANIMATION_NAME_KEYWORDS, lower)
        }
        "animation-name" => !has(ANIMATION_NAME_KEYWORDS, lower),
        "font" => !has(FONT_SHORTHAND_KEYWORDS, lower) && !has(FONT_FAMILY_KEYWORDS, lower),
        "font-family" => !has(FONT_FAMILY_KEYWORDS, lower),
        "counter-increment" => is_counter_name(lower, COUNTER_INCREMENT_KEYWORDS),
        "counter-reset" => is_counter_name(lower, COUNTER_RESET_KEYWORDS),
        "grid-row" | "grid-row-start" | "grid-row-end" => !has(GRID_ROW_KEYWORDS, lower),
        "grid-column" | "grid-column-start" | "grid-column-end" => {
            !has(GRID_COLUMN_KEYWORDS, lower)
        }
        "grid-area" => !has(GRID_AREA_KEYWORDS, lower),
        "list-style" => {
            !has(LIST_STYLE_SHORTHAND_KEYWORDS, lower) && !has(LIST_STYLE_TYPE_KEYWORDS, lower)
        }
        "list-style-type" => !has(LIST_STYLE_TYPE_KEYWORDS, lower),
        _ => false,
    }
}

impl SheetRule for ValueKeywordCase {
    fn name(&self) -> &'static str {
        "stylelint/value-keyword-case"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to be \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let whole = &node.value;
            if !whole.bytes().any(|byte| byte.is_ascii_alphabetic()) || !is_standard_value(whole) {
                continue;
            }
            let property = node.prop.to_lowercase();
            let start = sheet.value_start(node);
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            value::parse(sheet.value(node)).walk(&mut |_, part| {
                let keyword = &part.value;
                let lower = keyword.to_lowercase();
                if has(SYSTEM_COLORS_KEYWORDS, &lower) {
                    return true;
                }
                if part.kind == Part::Function
                    && (matches!(
                        lower.as_str(),
                        "url" | "var" | "counter" | "counters" | "attr"
                    ) || matches_any(&self.ignore_functions, keyword))
                {
                    return false;
                }
                let dimension = part.kind == Part::Word
                    && is_standard_value(keyword)
                    && !keyword.starts_with('#')
                    && unit(&keyword.replacen("\\0", "", 1).replacen("\\9", "", 1))
                        .is_some_and(|(_, found)| !found.is_empty());
                if part.kind != Part::Word
                    || !is_standard_value(keyword)
                    || whole.contains('#')
                    || matches!(keyword.as_str(), "+" | "-" | "/" | "*" | "%")
                    || dimension
                    || names_something(&property, &lower)
                    || matches_any(&self.ignore_keywords, keyword)
                    || matches_any(&self.ignore_properties, &node.prop)
                {
                    return true;
                }
                let expected = self.expected(keyword);
                if expected != *keyword {
                    let at = start + part.start;
                    reports.push(
                        Report::new(at, at + keyword.len(), "expected")
                            .with(&[keyword, &expected])
                            .fixed(at, at + keyword.len(), expected)
                            .scoped(scope.0, scope.1),
                    );
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ValueKeywordCase;
    use crate::scss::rules::tester::{Js, matchers, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/value-keyword-case.txt",
            &|config| {
                let options = secondary(config);
                Ok(Box::new(ValueKeywordCase {
                    lower: primary(config) == &Js::Text("lower".to_owned()),
                    camel_case_svg_keywords: options
                        .and_then(|each| each.get("camelCaseSvgKeywords"))
                        == Some(&Js::Bool(true)),
                    ignore_keywords: matchers(options, "ignoreKeywords")?,
                    ignore_properties: matchers(options, "ignoreProperties")?,
                    ignore_functions: matchers(options, "ignoreFunctions")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (296, 144, 62),
            "{:#?}",
            tally.apart
        );
    }
}
