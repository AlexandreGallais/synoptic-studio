//! `dimension-no-non-numeric-values` of stylelint-scss 7.3.0
//! (`src/rules/dimension-no-non-numeric-values/index.js`, REF-STYLELINT-SOURCE): an interpolation
//! followed by a unit (`#{$a}px`), a string where a number is meant, reported over its first such
//! text in the declaration's value once for each interpolated word.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct DimensionNoNonNumericValues;

/// The units the rule knows, in the original's order.
const UNITS: &[&str] = &[
    "em", "ex", "cap", "ch", "ic", "rem", "lh", "rlh", "vw", "vh", "vi", "vb", "vmin", "vmax",
    "cm", "mm", "Q", "in", "pc", "pt", "px", "deg", "grad", "rad", "turn", "s", "ms", "Hz", "kHz",
    "dpi", "dpcm", "dppx", "x", "fr",
];

/// Whether a character may sit in the interpolation (`[$a-z_0-9 +\-]`).
fn is_simple(byte: u8) -> bool {
    byte == b'$'
        || byte.is_ascii_lowercase()
        || byte == b'_'
        || byte.is_ascii_digit()
        || matches!(byte, b' ' | b'+' | b'-')
}

/// The first interpolation followed by a unit in a text, and the unit
/// (`#\{[$a-z_0-9 +\-]*\}(units);?`).
fn first_match(text: &str) -> Option<(usize, usize, &'static str)> {
    let bytes = text.as_bytes();
    text.match_indices("#{").find_map(|(at, _)| {
        let inner = bytes[at + 2..]
            .iter()
            .take_while(|byte| is_simple(**byte))
            .count();
        let close = at + 2 + inner;
        if bytes.get(close) != Some(&b'}') {
            return None;
        }
        let unit = UNITS
            .iter()
            .find(|unit| text[close + 1..].starts_with(*unit))?;
        let mut end = close + 1 + unit.len();
        if bytes.get(end) == Some(&b';') {
            end += 1;
        }
        Some((at, end, *unit))
    })
}

/// Whether a word is an interpolation with a unit, or a fragment of one (`isInterpolated`).
fn is_interpolated(word: &str) -> bool {
    if let Some(rest) = word.strip_prefix("#{$")
        && rest.bytes().all(|byte| byte.is_ascii_lowercase())
    {
        return true;
    }
    word.strip_prefix("#{").is_some_and(|rest| {
        let inner = rest.bytes().take_while(|byte| is_simple(*byte)).count();
        rest[inner..].strip_prefix('}').is_some_and(|after| {
            let after = after.strip_suffix(';').unwrap_or(after);
            UNITS.contains(&after)
        })
    })
}

impl SheetRule for DimensionNoNonNumericValues {
    fn name(&self) -> &'static str {
        "scss/dimension-no-non-numeric-values"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "rejected",
            "Expected \"$value * 1{}\" instead of \"#{$value}{}\". Consider writing \"value\" in terms of {} originally.",
        )]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            value::parse(&node.value).walk(&mut |_, part| {
                if part.kind != Part::Word || !is_interpolated(&part.value) {
                    return true;
                }
                if let Some((from, to, unit)) = first_match(&node.value) {
                    let (start, end) = sheet.word_range(node, &node.value[from..to]);
                    reports.push(Report::new(start, end, "rejected").with(&[unit, unit, unit]));
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DimensionNoNonNumericValues, first_match, is_interpolated};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/dimension-no-non-numeric-values.txt",
            &|_| Ok(Box::new(DimensionNoNonNumericValues)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (0, 0, 0),
            "{:#?}",
            tally.apart
        );
        assert!(is_interpolated("#{$a}px") && is_interpolated("#{$a") && !is_interpolated("$a"));
        assert_eq!(first_match("1 #{$a}ms"), Some((2, 9, "ms")));
    }
}
