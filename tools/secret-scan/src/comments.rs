//! `@secretlint/secretlint-rule-filter-comments` of `@secretlint/secretlint-rule-preset-recommend`
//! 13.0.7, and the filter of `@secretlint/core` 13.0.7 that drops what it ignores: a
//! `secretlint-disable` comment ignores what follows it until a `secretlint-enable`, or the end of
//! the file; `secretlint-disable-line` its own line, `secretlint-disable-next-line` the next. Each
//! names the rules it ignores, separated by commas, before an optional `--` and a reason; none
//! names every rule (`*`). A finding is dropped when both ends of its range lie within an ignored
//! range of its rule.
//!
//! The rule computes with JavaScript numbers, and its behaviour at the edges is kept: a line past
//! the last has no index (`NaN`), so a `secretlint-disable-line` on the file's last line, with no
//! line break after it, ignores nothing; and an end counts as unset when it is `null`, 0 or
//! `NaN`, as JavaScript's `!` reads it.

use std::sync::LazyLock;

use crate::js::DOT;
use crate::pattern::Pattern;
use crate::source::{Finding, Source, position};

/// The comments, their kind the group `type` and the rest of their line the group `options`.
static COMMENT: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::plain(&format!(
        "(?P<type>secretlint-(?:disable-next-line|disable-line|disable|enable))(?P<options>{DOT}*)"
    ))
});

/// A JavaScript number of the rule: a UTF-16 index, `None` for `NaN`.
type Number = Option<usize>;

/// An ignored range: its ends, which may be `NaN`, and the rule it ignores, `*` for every rule.
#[derive(Debug, Clone)]
pub struct Ignored {
    /// Its start, a UTF-16 index.
    start: Number,
    /// Its end, included.
    end: Number,
    /// The rule's id, `*` for every rule.
    rule: String,
}

/// One range being built: its end unset until a `secretlint-enable`.
#[derive(Debug, Clone)]
struct Reporting {
    /// Its start.
    start: Number,
    /// Its end, `None` until set, JavaScript's `null`.
    end: Option<Number>,
    /// The rule's id, `*` for every rule.
    rule: String,
}

/// Whether a JavaScript number, or `null`, is falsy: `null`, 0 or `NaN`.
fn falsy(number: Option<Number>) -> bool {
    matches!(number, None | Some(None | Some(0)))
}

/// The rule ids a comment names, before `--`, trimmed, the empty ones dropped.
fn rule_ids(options: &str) -> Vec<String> {
    let names = options.find("--").map_or(options, |at| &options[..at]);

    names
        .split(',')
        .map(|name| name.trim_matches(is_js_white_space))
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect()
}

/// JavaScript's white space and line terminators, as `String#trim` removes them.
fn is_js_white_space(character: char) -> bool {
    matches!(
        character,
        '\t' | '\u{0B}'
            | '\u{0C}'
            | ' '
            | '\u{A0}'
            | '\u{FEFF}'
            | '\n'
            | '\r'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
    )
}

/// The rules a comment names, or `*`.
fn targets(options: &str) -> Vec<String> {
    let ids = rule_ids(options);

    if ids.is_empty() {
        vec!["*".to_owned()]
    } else {
        ids
    }
}

/// The ranges the comments of a file ignore.
#[must_use]
pub fn ignored_ranges(source: &Source<'_>) -> Vec<Ignored> {
    let comments = COMMENT.find_all(&source.content);

    if comments.is_empty() {
        return Vec::new();
    }
    let starts = source.line_starts();
    // `positionToIndex`: a line's first index, `NaN` past the last line.
    let line_index = |line: usize| line.checked_sub(1).and_then(|at| starts.get(at)).copied();
    let mut reporting: Vec<Reporting> = Vec::new();

    for comment in comments {
        let kind = comment.group("type").unwrap_or_default();
        let options = comment.group("options").unwrap_or_default();
        let index = source.index(comment.start);

        match kind {
            "secretlint-disable-line" | "secretlint-disable-next-line" => {
                let line = position(&starts, index).0 + usize::from(kind.ends_with("next-line"));
                let start = line_index(line);
                let end = line_index(line + 1).and_then(|next| next.checked_sub(1));

                for rule in targets(options) {
                    reporting.push(Reporting {
                        start,
                        end: Some(end),
                        rule,
                    });
                }
            }
            "secretlint-disable" => {
                for rule in targets(options) {
                    reporting.push(Reporting {
                        start: Some(index),
                        end: None,
                        rule,
                    });
                }
            }
            _ => enable(&mut reporting, Some(index), &rule_ids(options)),
        }
    }
    let length = Some(source.index(source.content.len()));

    reporting
        .into_iter()
        .map(|range| Ignored {
            start: range.start,
            end: range.end.unwrap_or(length),
            rule: range.rule,
        })
        .collect()
}

/// A `secretlint-enable` at `end`: closes the last open range of each rule it names; naming none,
/// the last open ranges that started at the same place.
fn enable(reporting: &mut [Reporting], end: Number, rules: &[String]) {
    if rules.is_empty() {
        let mut previous_start: Option<Number> = None;

        for range in reporting.iter_mut().rev() {
            // `prevStart && prevStart !== startIndex`: `NaN` is falsy, and differs from itself.
            if previous_start.is_some_and(|start| !falsy(Some(start)) && start != range.start) {
                break;
            }
            if falsy(range.end) {
                range.end = Some(end);
                previous_start = Some(range.start);
            }
        }
    } else {
        for rule in rules {
            if let Some(range) = reporting
                .iter_mut()
                .rev()
                .find(|range| falsy(range.end) && range.rule == *rule)
            {
                range.end = Some(end);
            }
        }
    }
}

/// Whether a finding lies, both ends, within a range ignoring its rule.
#[must_use]
pub fn is_ignored(finding: &Finding, ignored: &[Ignored]) -> bool {
    let contains = |range: &Ignored, index: usize| {
        range.start.is_some_and(|start| start <= index) && range.end.is_some_and(|end| index <= end)
    };

    ignored.iter().any(|range| {
        contains(range, finding.range.0)
            && contains(range, finding.range.1)
            && (range.rule == "*" || range.rule == finding.rule_id)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_comment_names_its_rules_before_its_reason() {
        assert_eq!(rule_ids(" a, b ,c"), ["a", "b", "c"]);
        assert_eq!(rule_ids(" -- the reason, a"), Vec::<String>::new());
        assert_eq!(rule_ids(" a -- why"), ["a"]);
        assert_eq!(targets(""), ["*"]);
    }

    #[test]
    fn a_falsy_number_is_null_zero_or_nan() {
        assert!(falsy(None));
        assert!(falsy(Some(Some(0))));
        assert!(falsy(Some(None)));
        assert!(!falsy(Some(Some(3))));
    }

    #[test]
    fn enable_without_rules_closes_the_ranges_opened_together() {
        // disable a,b at 5, then disable c at 9: enable at 20 closes c's alone, the last open
        // range; a second enable at 30 closes a's and b's, opened together at 5.
        let open = |start: usize, rule: &str| Reporting {
            start: Some(start),
            end: None,
            rule: rule.to_owned(),
        };
        let mut reporting = vec![open(5, "a"), open(5, "b"), open(9, "c")];

        enable(&mut reporting, Some(20), &[]);
        assert_eq!(
            reporting.iter().map(|range| range.end).collect::<Vec<_>>(),
            [None, None, Some(Some(20))]
        );
        enable(&mut reporting, Some(30), &[]);
        assert_eq!(
            reporting.iter().map(|range| range.end).collect::<Vec<_>>(),
            [Some(Some(30)), Some(Some(30)), Some(Some(20))]
        );
    }

    #[test]
    fn enable_with_a_rule_closes_its_last_open_range() {
        let mut reporting = vec![
            Reporting {
                start: Some(1),
                end: None,
                rule: "a".to_owned(),
            },
            Reporting {
                start: Some(2),
                end: None,
                rule: "b".to_owned(),
            },
        ];

        enable(&mut reporting, Some(7), &["a".to_owned(), "z".to_owned()]);
        assert_eq!(reporting[0].end, Some(Some(7)));
        assert_eq!(reporting[1].end, None);
    }
}
