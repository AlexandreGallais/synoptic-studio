//! The font families of a `font` or `font-family` value, as `findFontFamily` of Stylelint 17.16.0
//! finds them (REF-STYLELINT-SOURCE, MIT, under ADR-0076): its words and strings, the font's
//! size, line height, style, weight and stretch keywords and numbers left out, the words of a
//! family name written with spaces joined.

use super::is_standard_value;
use super::reference::{
    BASIC_KEYWORDS, FONT_FAMILY_KEYWORDS, FONT_SHORTHAND_KEYWORDS, FONT_SIZE_KEYWORDS,
    LENGTH_UNITS, has,
};
use crate::scss::value::{self, Kind as Part, Value};

/// A family found: its text, joined, where its first part starts and ends, its quote, and
/// whether its first part is a word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Family {
    /// The family's text, its words joined by what separated them.
    pub value: String,
    /// Its first part's first byte in the value.
    pub start: usize,
    /// Its first part's end in the value.
    pub end: usize,
    /// The quote of a string.
    pub quote: Option<char>,
    /// Whether its first part is a word, not a string.
    pub word: bool,
}

impl Family {
    /// Whether it is a generic family or a CSS-wide keyword (`isFamilyNameKeyword`).
    #[must_use]
    pub fn is_keyword(&self) -> bool {
        self.word && has(FONT_FAMILY_KEYWORDS, &self.value.to_lowercase())
    }
}

/// Whether a word is a font size: a keyword, a percentage or a length (`isValidFontSize`).
#[must_use]
pub fn is_font_size(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    if has(FONT_SIZE_KEYWORDS, word) {
        return true;
    }
    value::unit(word)
        .is_some_and(|(_, unit)| unit == "%" || has(LENGTH_UNITS, &unit.to_lowercase()))
}

/// Whether a text is a number as `Number(value) == value` reads it (`isNumbery`).
fn is_numbery(text: &str) -> bool {
    !text.trim().is_empty() && !crate::javascript::to_number(text).is_nan()
}

/// The families of a value.
#[must_use]
pub fn find(text: &str) -> Vec<Family> {
    let parsed = value::parse(text);
    if let [only] = parsed.nodes.as_slice() {
        let part = parsed.part(*only);
        if has(BASIC_KEYWORDS, &part.value.to_lowercase()) {
            return vec![family(part)];
        }
    }
    let mut families: Vec<Family> = Vec::new();
    let mut merge: Option<String> = None;
    walk_siblings(&parsed, &parsed.nodes, &mut families, &mut merge);
    families
}

/// A family of one part.
fn family(part: &value::Part) -> Family {
    Family {
        value: part.value.clone(),
        start: part.start,
        end: part.end,
        quote: part.quote,
        word: part.kind == Part::Word,
    }
}

/// Reads a list of sibling parts, in order, a function's arguments left out.
fn walk_siblings(
    parsed: &Value,
    nodes: &[usize],
    families: &mut Vec<Family>,
    merge: &mut Option<String>,
) {
    for (index, &at) in nodes.iter().enumerate() {
        let part = parsed.part(at);
        if part.kind == Part::Function
            || !matches!(
                part.kind,
                Part::Word | Part::String | Part::Space | Part::Div
            )
        {
            continue;
        }
        let lower = part.value.to_lowercase();
        if !is_standard_value(&lower) || lower.starts_with("var(") {
            continue;
        }
        let keyword = has(FONT_FAMILY_KEYWORDS, &lower);
        if !keyword && (has(FONT_SHORTHAND_KEYWORDS, &lower) || is_font_size(&part.value)) {
            continue;
        }
        let next = nodes.get(index + 1).map(|&each| parsed.part(each));
        let previous = index
            .checked_sub(1)
            .map(|before| parsed.part(nodes[before]));
        let before_previous = index
            .checked_sub(2)
            .map(|before| parsed.part(nodes[before]));
        let is_comma = |each: &value::Part| each.kind == Part::Div && each.value == ",";
        if keyword
            && has(FONT_SIZE_KEYWORDS, &lower)
            && !(next.is_none_or(is_comma)
                || nodes[..index]
                    .iter()
                    .any(|&each| is_comma(parsed.part(each))))
        {
            continue;
        }
        if previous.is_some_and(|previous| previous.value == "/")
            && before_previous.is_some_and(|before| is_font_size(&before.value))
        {
            continue;
        }
        if is_numbery(&lower) {
            continue;
        }
        let separator = part.kind == Part::Space || (part.kind == Part::Div && part.value != ",");
        if separator && !families.is_empty() {
            *merge = Some(part.value.clone());
            continue;
        }
        if part.kind == Part::Space || part.kind == Part::Div {
            continue;
        }
        match (merge.take(), families.last_mut()) {
            (Some(between), Some(last)) => {
                last.value = format!("{}{between}{}", last.value, part.value);
            }
            _ => families.push(family(part)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{find, is_font_size};

    #[test]
    fn finds_the_families_of_a_font_value() {
        let names: Vec<String> = find("italic bold 12px/30px Times New Roman, \"Georgia\", serif")
            .into_iter()
            .map(|family| family.value)
            .collect();
        assert_eq!(names, ["Times New Roman", "Georgia", "serif"]);
        assert_eq!(find("inherit").len(), 1);
        assert!(
            is_font_size("small")
                && is_font_size("10%")
                && !is_font_size("bold")
                && !is_font_size("")
        );
    }
}
