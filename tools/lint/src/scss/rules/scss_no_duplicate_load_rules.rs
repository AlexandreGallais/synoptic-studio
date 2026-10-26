//! `no-duplicate-load-rules` of stylelint-scss 7.3.0 (`src/rules/no-duplicate-load-rules/index.js`,
//! REF-STYLELINT-SOURCE): an `@use`, `@forward` or `@import` of a URL the same at-rule loaded
//! before (under the same media and conditions for `@import`), reported over the whole at-rule.

use super::{Context, Report, SheetRule};
use crate::javascript::{JsText, is_js_space};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part, Value};

/// The rule.
pub struct NoDuplicateLoadRules;

/// Parameters without their namespace or visibility clauses (`/\s+(as|with|show|hide)\s[^;]*/g`).
fn without_clauses(params: &str) -> String {
    let mut result = String::new();
    let mut rest = params;
    loop {
        let found = rest.char_indices().find_map(|(at, character)| {
            if !is_js_space(character) || (at > 0 && rest[..at].ends_with(is_js_space)) {
                return None;
            }
            let spaces = rest[at..].len() - rest[at..].js_trim_start().len();
            let word = &rest[at + spaces..];
            ["as", "with", "show", "hide"]
                .iter()
                .find(|keyword| {
                    word.starts_with(**keyword) && word[keyword.len()..].starts_with(is_js_space)
                })
                .map(|_| at)
        });
        let Some(at) = found else {
            result.push_str(rest);
            return result;
        };
        result.push_str(&rest[..at]);
        let end = rest[at..]
            .find(';')
            .map_or(rest.len(), |semicolon| at + semicolon);
        rest = &rest[end..];
    }
}

/// The text of parts with no white space (`stringifyCondition`).
fn condition(parsed: &Value, text: &str, parts: &[usize]) -> String {
    parts
        .iter()
        .map(|each| {
            let part = parsed.part(*each);
            &text[part.start..part.end]
        })
        .collect::<String>()
        .js_split_whitespace()
        .collect()
}

/// An `@import`'s conditions, one for each media query (`listImportConditions`).
fn conditions(parsed: &Value, text: &str, parts: &[usize]) -> Vec<String> {
    let mut shared = Vec::new();
    let mut media = Vec::new();
    let mut last = Vec::new();
    for &each in parts {
        let part = parsed.part(each);
        if matches!(part.kind, Part::Space | Part::Comment) {
            continue;
        }
        let layer = (part.kind == Part::Function
            && matches!(part.value.as_str(), "supports" | "layer"))
            || (part.kind == Part::Word && part.value == "layer");
        if media.is_empty() && layer {
            shared.push(condition(parsed, text, &[each]));
            continue;
        }
        if part.kind == Part::Div && part.value == "," {
            media.push(condition(parsed, text, &last));
            last.clear();
            continue;
        }
        last.push(each);
    }
    if !last.is_empty() {
        media.push(condition(parsed, text, &last));
    }
    let joined = shared.join(" ");
    match (media.is_empty(), shared.is_empty()) {
        (false | true, true) => media,
        (true, false) => vec![joined],
        (false, false) => media
            .iter()
            .map(|each| format!("{joined} {each}"))
            .collect(),
    }
}

impl SheetRule for NoDuplicateLoadRules {
    fn name(&self) -> &'static str {
        "scss/no-duplicate-load-rules"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected duplicate load rule {}")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        // Each at-rule's name with its URLs and their conditions.
        let mut loaded: Vec<(String, String, Vec<String>)> = Vec::new();
        for (_, node) in sheet.all(Kind::AtRule) {
            let name = node.name.to_lowercase();
            if !matches!(name.as_str(), "forward" | "import" | "use") {
                continue;
            }
            let params = if name == "forward" {
                sheet.params(node).to_owned()
            } else {
                without_clauses(sheet.params(node))
            };
            let parsed = value::parse(&params);
            let Some((&first, rest)) = parsed.nodes.split_first() else {
                continue;
            };
            let part = parsed.part(first);
            let uri = match part.nodes.first() {
                Some(&inner) if part.kind == Part::Function && part.value == "url" => {
                    parsed.part(inner).value.clone()
                }
                _ => part.value.clone(),
            };
            let media = conditions(&parsed, &params, rest);
            let found = loaded
                .iter()
                .position(|(each, url, _)| *each == name && *url == uri);
            let duplicate = match found {
                Some(at) if !media.is_empty() => {
                    media.iter().any(|each| loaded[at].2.contains(each))
                }
                Some(_) => true,
                None => false,
            };
            if duplicate {
                let length = 1
                    + node.name.len()
                    + node.after_name.len()
                    + sheet.params(node).len()
                    + node.between.len();
                reports
                    .push(Report::new(node.start, node.start + length, "rejected").with(&[&uri]));
                continue;
            }
            match found {
                Some(at) => loaded[at].2.extend(media),
                None => loaded.push((name, uri, media)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NoDuplicateLoadRules, without_clauses};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/no-duplicate-load-rules.txt",
            &|_| Ok(Box::new(NoDuplicateLoadRules)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (27, 31, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(without_clauses("\"a\" as b"), "\"a\"");
    }
}
