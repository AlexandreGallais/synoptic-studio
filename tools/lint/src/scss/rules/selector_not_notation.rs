//! `selector-not-notation` of Stylelint 17.16.0 (`lib/rules/selector-not-notation/index.mjs`,
//! REF-STYLELINT-SOURCE): `:not()` of a plain rule written in the notation the option refuses,
//! reported over it: with `"complex"` (the project's, stylelint-config-standard's), a `:not()`
//! following another, fixed by merging the run into the first as one list; with `"simple"`, a
//! `:not()` of a list or of a compound, fixed when each item is simple by one `:not()` each.

use super::{Context, Report, SheetRule, is_standard_rule};
use crate::javascript::JsText;
use crate::scss::selector::{self, Kind as Part, Selectors};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct SelectorNotNotation {
    /// Whether the complex notation is wanted (`"complex"`), else the simple one.
    pub complex: bool,
}

impl SelectorNotNotation {
    /// The project's configuration: `"complex"`.
    pub const PROJECT: Self = Self { complex: true };
}

/// Whether a part is `:not`.
fn is_not(parsed: &Selectors, index: usize) -> bool {
    let part = parsed.part(index);
    part.kind == Part::Pseudo && part.value.eq_ignore_ascii_case(":not")
}

/// Whether a part is a simple selector: a type, `*`, an id, a class, an attribute or a
/// pseudo-class.
fn is_simple(parsed: &Selectors, index: usize) -> bool {
    let part = parsed.part(index);
    match part.kind {
        Part::Tag | Part::Universal | Part::Id | Part::Class | Part::Attribute => true,
        Part::Pseudo => {
            !part.value.starts_with("::")
                && !matches!(
                    part.value.to_lowercase().as_str(),
                    ":before" | ":after" | ":first-line" | ":first-letter"
                )
        }
        _ => false,
    }
}

/// A selector's text, trimmed.
fn written<'t>(parsed: &Selectors, text: &'t str, selector: usize) -> &'t str {
    let part = parsed.part(selector);
    text[part.start..part.end].js_trim()
}

impl SelectorNotNotation {
    /// The complex fix: the run of `:not()`s from `previous` merged into it; its range and text.
    fn merge(parsed: &Selectors, text: &str, previous: usize) -> Option<(usize, usize, String)> {
        let first = parsed.part(previous);
        let head = *first.nodes.first()?;
        if parsed.part(head).nodes.is_empty() {
            return None;
        }
        let mut items: Vec<String> = vec![written(parsed, text, head).to_owned()];
        items.extend(
            first.nodes[1..]
                .iter()
                .map(|&each| format!(" {}", written(parsed, text, each))),
        );
        let mut end = first.end;
        let mut next = parsed.next(previous);
        while let Some(not) = next.filter(|&each| is_not(parsed, each)) {
            items.extend(
                parsed
                    .part(not)
                    .nodes
                    .iter()
                    .map(|&each| format!(" {}", written(parsed, text, each))),
            );
            end = parsed.part(not).end;
            next = parsed.next(not);
        }
        Some((
            first.start,
            end,
            format!("{}({})", first.value, items.join(",")),
        ))
    }

    /// The simple fix: one `:not()` for each simple item, the others after the run of `:not()`s.
    fn split(parsed: &Selectors, text: &str, not: usize) -> Option<(usize, usize, String)> {
        let part = parsed.part(not);
        let simple: Vec<&str> = part
            .nodes
            .iter()
            .filter(|&&each| {
                parsed
                    .part(each)
                    .nodes
                    .first()
                    .is_some_and(|&first| is_simple(parsed, first))
            })
            .map(|&each| written(parsed, text, each))
            .collect();
        let (first, rest) = simple.split_first()?;
        let mut last = not;
        while let Some(next) = parsed.next(last).filter(|&each| is_not(parsed, each)) {
            last = next;
        }
        let name = &parsed.part(last).value;
        let between = &text[part.end..parsed.part(last).end];
        let added: String = rest.iter().fold(String::new(), |mut added, each| {
            added.push_str(name);
            added.push('(');
            added.push_str(each);
            added.push(')');
            added
        });
        Some((
            part.start,
            parsed.part(last).end,
            format!("{}({first}){between}{added}", part.value),
        ))
    }
}

impl SheetRule for SelectorNotNotation {
    fn name(&self) -> &'static str {
        "stylelint/selector-not-notation"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected {} :not() pseudo-class notation")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let option = if self.complex { "complex" } else { "simple" };
        for (_, node) in sheet.all(Kind::Rule) {
            if !node.selector.to_lowercase().contains(":not(") || !is_standard_rule(node) {
                continue;
            }
            let text = sheet.selector(node);
            let Ok(parsed) = selector::parse(text) else {
                continue;
            };
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            for each in parsed.walk() {
                if !is_not(&parsed, each) {
                    continue;
                }
                let part = parsed.part(each);
                let fix = if self.complex {
                    let Some(previous) = parsed
                        .previous(each)
                        .filter(|&previous| is_not(&parsed, previous))
                    else {
                        continue;
                    };
                    Self::merge(&parsed, text, previous)
                } else {
                    let selectors = &part.nodes;
                    let simple = match selectors.as_slice() {
                        [] => true,
                        [only] => match parsed.part(*only).nodes.as_slice() {
                            [] => true,
                            [first] => is_simple(&parsed, *first) && !is_not(&parsed, *first),
                            _ => false,
                        },
                        _ => false,
                    };
                    if simple {
                        continue;
                    }
                    let fixable = selectors.get(1).is_some_and(|&second| {
                        parsed.part(second).nodes.is_empty()
                            || selectors
                                .iter()
                                .all(|&each| parsed.part(each).nodes.len() == 1)
                    });
                    if fixable {
                        Self::split(&parsed, text, each)
                    } else {
                        None
                    }
                };
                let at = node.start + part.start;
                let length = text[part.start..part.end].js_trim().len();
                let report = Report::new(at, at + length, "expected")
                    .with(&[option])
                    .scoped(scope.0, scope.1);
                reports.push(match fix {
                    Some((start, end, replacement)) => {
                        report.fixed(node.start + start, node.start + end, replacement)
                    }
                    None => report,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorNotNotation;
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-not-notation.txt",
            &|config| {
                Ok(Box::new(SelectorNotNotation {
                    complex: primary(config) == &Js::Text("complex".to_owned()),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (11, 14, 2),
            "{:#?}",
            tally.apart
        );
    }
}
