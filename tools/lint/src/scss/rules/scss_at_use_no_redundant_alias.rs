//! `at-use-no-redundant-alias` of stylelint-scss 7.3.0
//! (`src/rules/at-use-no-redundant-alias/index.js`, REF-STYLELINT-SOURCE): an `@use` whose `as`
//! names the namespace Sass gives the module anyway, reported over `as name` and fixed by
//! dropping it. The module's default namespace is read as the original reads it, its first `.`
//! and the character after it dropped.

use super::{Context, Report, SheetRule};
use crate::javascript::{JsText, is_js_space};
use crate::scss::tree::Kind;

/// The rule.
pub struct AtUseNoRedundantAlias;

/// The namespace Sass gives a module (`getDefaultNamespace`): its last path segment, the first
/// `.` and the character after it, or its trailing quotes, removed.
fn default_namespace(module: &str) -> String {
    let segment = module.rsplit(['/', ':']).next().unwrap_or_default();
    let bytes = segment.as_bytes();
    for at in 0..bytes.len() {
        if bytes[at] == b'.' && bytes.get(at + 1).is_some_and(|next| *next != b'.') {
            let width = segment[at + 1..].chars().next().map_or(1, char::len_utf8);
            return format!("{}{}", &segment[..at], &segment[at + 1 + width..]);
        }
        if bytes[at] == b'"' && bytes[at..].iter().all(|byte| *byte == b'"') {
            return segment[..at].to_owned();
        }
    }
    segment.to_owned()
}

/// The parameters without quotes, split at ` as ` and ` with ` (`separateEachParams`).
fn split(params: &str) -> Vec<String> {
    let bare: String = params.chars().filter(|c| *c != '"' && *c != '\'').collect();
    let mut parts = Vec::new();
    let mut rest = bare.as_str();
    loop {
        let found = rest.char_indices().find_map(|(at, c)| {
            if !is_js_space(c) {
                return None;
            }
            let after = rest[at..].js_trim_start();
            ["as", "with"].iter().find_map(|word| {
                after
                    .strip_prefix(word)
                    .filter(|tail| tail.starts_with(is_js_space))
                    .map(|tail| (at, rest.len() - tail.js_trim_start().len()))
            })
        });
        if let Some((cut, next)) = found {
            parts.push(rest[..cut].to_owned());
            rest = &rest[next..];
        } else {
            parts.push(rest.to_owned());
            return parts;
        }
    }
}

impl SheetRule for AtUseNoRedundantAlias {
    fn name(&self) -> &'static str {
        "scss/at-use-no-redundant-alias"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected redundant namespace.")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            let params = &node.params;
            let parts = split(params);
            if node.name != "use" || parts.len() < 2 || default_namespace(&parts[0]) != parts[1] {
                continue;
            }
            let Some(at) = params
                .match_indices("as")
                .find(|(at, _)| {
                    params[at + 2..].starts_with(is_js_space)
                        && !params[at + 2..].js_trim_start().is_empty()
                })
                .map(|(at, _)| at)
            else {
                continue;
            };
            let spaces = params[at + 2..].len() - params[at + 2..].js_trim_start().len();
            let name = params[at + 2 + spaces..]
                .js_split_whitespace()
                .next()
                .unwrap_or_default();
            let start = sheet.params_start(node) + at;
            let end = start + 2 + spaces + name.len();
            // `\s*as\s* [^\s*]+\s*` made one space, in the at-rule's text before its `;`.
            let before = params[..at].len() - params[..at].js_trim_end().len();
            let after = params[at + 2 + spaces + name.len()..].len()
                - params[at + 2 + spaces + name.len()..].js_trim_start().len();
            let fix_start = start - before;
            let fix_end = end + after;
            // The new at-rule takes the old one's spaces: a space left at its end is dropped.
            let at_end = params[at + 2 + spaces + name.len()..].js_trim().is_empty();
            let replacement = if at_end { "" } else { " " };
            reports.push(Report::new(start, end, "rejected").fixed(
                fix_start,
                fix_end,
                replacement,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AtUseNoRedundantAlias, default_namespace};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-use-no-redundant-alias.txt",
            &|_| Ok(Box::new(AtUseNoRedundantAlias)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (6, 14, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(
            (
                default_namespace("sass:math"),
                default_namespace("a/b.scss")
            ),
            ("math".to_owned(), "bcss".to_owned())
        );
    }
}
