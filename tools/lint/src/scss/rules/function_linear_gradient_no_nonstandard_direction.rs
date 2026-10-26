//! `function-linear-gradient-no-nonstandard-direction` of Stylelint 17.16.0
//! (`lib/rules/function-linear-gradient-no-nonstandard-direction/index.mjs`, REF-STYLELINT-SOURCE):
//! a `linear-gradient()` (prefixed or not) whose first argument is a direction written without
//! `to` (with `to` when prefixed), a side repeated, or a number that is no angle, reported over
//! that argument.

use super::prefixed::prefix;
use super::{Context, Report, SheetRule, function_arguments, is_standard_value};
use crate::javascript::JsText;
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct FunctionLinearGradientNoNonstandardDirection;

/// Whether a name is `linear-gradient`, prefixed or not.
fn is_linear_gradient(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    ["", "-webkit-", "-moz-", "-o-", "-ms-"]
        .iter()
        .any(|prefix| lower.strip_prefix(prefix) == Some("linear-gradient"))
}

/// Whether a word is a side (`/top|left|bottom|right/i` matched whole).
fn is_side(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "top" | "left" | "bottom" | "right"
    )
}

/// Whether a text holds a side anywhere (`/top|left|bottom|right/i`).
fn holds_side(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    ["top", "left", "bottom", "right"]
        .iter()
        .any(|side| lower.contains(side))
}

/// Whether a direction is standard: `to` and one or two different sides, or without `to` for a
/// prefixed function (`isStandardDirection`).
fn is_standard_direction(source: &str, with_to: bool) -> bool {
    let rest = if with_to {
        match source.get(..3) {
            Some(to) if to.eq_ignore_ascii_case("to ") => &source[3..],
            _ => return false,
        }
    } else {
        source
    };
    let words: Vec<&str> = rest.split(' ').collect();
    match words.as_slice() {
        [side] => is_side(side),
        [first, second] => is_side(first) && is_side(second) && first != second,
        _ => false,
    }
}

/// Whether a number is an angle (`/^[\d.]+(?:deg|grad|rad|turn)$/`).
fn is_angle(text: &str) -> bool {
    let digits = text
        .bytes()
        .take_while(|byte| byte.is_ascii_digit() || *byte == b'.')
        .count();
    digits > 0 && matches!(&text[digits..], "deg" | "grad" | "rad" | "turn")
}

/// Whether a text holds the word `in` (`/\bin\b/i`).
fn holds_in(text: &str) -> bool {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .any(|word| word.eq_ignore_ascii_case("in"))
}

impl SheetRule for FunctionLinearGradientNoNonstandardDirection {
    fn name(&self) -> &'static str {
        "stylelint/function-linear-gradient-no-nonstandard-direction"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Nonstandard direction")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.value.to_ascii_lowercase().contains("linear-gradient(") {
                continue;
            }
            let start = sheet.value_start(node);
            let parsed = value::parse(&node.value);
            parsed.walk(&mut |_, part| {
                if part.kind != Part::Function {
                    return true;
                }
                let printed = node.value[part.start..part.end].to_lowercase();
                for (expression, at, _) in function_arguments(&printed, &is_linear_gradient) {
                    let first_raw = expression.split(',').next().unwrap_or_default();
                    let first = first_raw.js_trim();
                    if !is_standard_value(first) || first.starts_with("var(") || holds_in(first) {
                        continue;
                    }
                    let bad = if first.starts_with(|c: char| c.is_ascii_digit() || c == '.') {
                        !is_angle(first)
                    } else {
                        holds_side(first)
                            && !is_standard_direction(first, prefix(&part.value).is_empty())
                    };
                    if bad {
                        let from = start + part.start + at;
                        reports.push(Report::new(
                            from,
                            from + first_raw.js_trim_end().len(),
                            "rejected",
                        ));
                    }
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionLinearGradientNoNonstandardDirection;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/function-linear-gradient-no-nonstandard-direction.txt",
            &|_| Ok(Box::new(FunctionLinearGradientNoNonstandardDirection)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (39, 27, 0),
            "{:#?}",
            tally.apart
        );
    }
}
