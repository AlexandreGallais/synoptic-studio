//! `no-irregular-whitespace` of Stylelint 17.16.0 (`lib/rules/no-irregular-whitespace/index.mjs`,
//! REF-STYLELINT-SOURCE): each run of irregular spaces (no-break, zero-width, the Unicode spaces,
//! vertical tab, form feed) in a node's text outside strings and comments (and the attribute
//! values of a selector), reported over the run, each part of a node placed as the original
//! places it.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct NoIrregularWhitespace;

/// Whether a character is an irregular space.
fn is_irregular(character: char) -> bool {
    matches!(
        character,
        '\u{b}' | '\u{c}' | '\u{a0}' | '\u{85}' | '\u{1680}' | '\u{180e}' | '\u{feff}' | '\u{2000}'
            ..='\u{200b}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}'
    )
}

/// Each run of irregular spaces of a text: its byte and its length.
fn runs(text: &str) -> Vec<(usize, usize)> {
    let mut found: Vec<(usize, usize)> = Vec::new();
    for (at, character) in text.char_indices() {
        if !is_irregular(character) {
            continue;
        }
        match found.last_mut() {
            Some((start, length)) if *start + *length == at => *length += character.len_utf8(),
            _ => found.push((at, character.len_utf8())),
        }
    }
    found
}

/// A text with its irregular spaces made plain spaces in the given ranges.
fn blanked(text: &str, ranges: &[(usize, usize)]) -> String {
    text.char_indices()
        .map(|(at, character)| {
            if is_irregular(character)
                && ranges.iter().any(|(start, end)| *start <= at && at < *end)
            {
                " ".repeat(character.len_utf8())
            } else {
                character.to_string()
            }
        })
        .collect()
}

/// A value with its strings and comments blanked (`normalizeValue`), strings kept when told.
fn normalized_value(text: &str, strings: bool, comments: bool) -> String {
    let parsed = value::parse(text);
    let mut ranges = Vec::new();
    parsed.walk(&mut |_, part| {
        if (strings && part.kind == Part::String) || (comments && part.kind == Part::Comment) {
            ranges.push((part.start, part.end));
        }
        true
    });
    blanked(text, &ranges)
}

/// A selector with its attributes' and comments' insides blanked (`normalizeSelector`).
fn normalized_selector(text: &str) -> String {
    let Ok(parsed) = crate::scss::selector::parse(text) else {
        return text.to_owned();
    };
    let ranges: Vec<(usize, usize)> = parsed
        .walk()
        .into_iter()
        .map(|index| parsed.part(index))
        .filter(|part| {
            matches!(
                part.kind,
                crate::scss::selector::Kind::Attribute | crate::scss::selector::Kind::Comment
            )
        })
        .map(|part| (part.start, part.end))
        .collect();
    blanked(text, &ranges)
}

impl SheetRule for NoIrregularWhitespace {
    fn name(&self) -> &'static str {
        "stylelint/no-irregular-whitespace"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Irregular whitespace")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        if !sheet.text.chars().any(is_irregular) {
            return;
        }
        let mut validate = |start: usize, text: &str| {
            for (at, length) in runs(text) {
                reports.push(Report::new(start + at, start + at + length, "rejected"));
            }
        };
        let after = |node: &crate::scss::tree::Node| {
            let end = node.end.unwrap_or(0);
            if end == 0 {
                0
            } else if node.after.is_empty() {
                end - 1
            } else {
                end - (node.after.len() + 1)
            }
        };
        for (_, node) in sheet.all(Kind::AtRule) {
            let params_start = 1 + node.name.len() + node.after_name.len();
            let params = sheet.params(node);
            validate(node.start, &node.before);
            validate(node.start + 1, &node.name);
            validate(node.start + 1 + node.name.len(), &node.after_name);
            validate(
                node.start + params_start,
                &normalized_value(params, true, false),
            );
            validate(node.start + params_start + params.len(), &node.between);
            validate(node.start + after(node), &node.after);
        }
        for (_, node) in sheet.all(Kind::Rule) {
            let selector = sheet.selector(node);
            validate(node.start, &node.before);
            validate(node.start, &normalized_selector(selector));
            validate(node.start + selector.len(), &node.between);
            validate(node.start + after(node), &node.after);
        }
        for (_, node) in sheet.all(Kind::Declaration) {
            let strings = !matches!(
                node.prop.to_lowercase().as_str(),
                "grid" | "grid-template" | "grid-template-areas"
            );
            validate(node.start, &node.before);
            validate(node.start, &normalized_value(&node.prop, true, true));
            validate(
                node.start + node.prop.len(),
                &normalized_value(&node.between, true, true),
            );
            validate(
                sheet.value_start(node),
                &normalized_value(sheet.value(node), strings, true),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoIrregularWhitespace;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/no-irregular-whitespace.txt",
            &|_| Ok(Box::new(NoIrregularWhitespace)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (0, 16, 0),
            "{:#?}",
            tally.apart
        );
    }
}
