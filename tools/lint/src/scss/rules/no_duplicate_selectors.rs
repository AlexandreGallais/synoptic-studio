//! `no-duplicate-selectors` of Stylelint 17.16.0 (`lib/rules/no-duplicate-selectors/index.mjs`,
//! REF-STYLELINT-SOURCE): a selector written twice in one rule's list, or a rule's resolved and
//! normalized list written again in the same at-rule context of the sheet, keyframes and
//! interpolated rules left out, reported over the repeat with the line of the first; with its
//! option `disallowInList`, each selector of a list compared alone.

use super::keyframe_block_no_duplicate_selectors::stripped;
use super::keyframes_name_pattern::is_keyframes;
use super::nested::{Selector, normalize, print, print_list, read, resolve};
use super::{Context, Matcher, Report, SheetRule, is_standard_rule, matches_any};
use crate::scss::selector;
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct NoDuplicateSelectors {
    /// Whether each selector of a list counts alone (`disallowInList`).
    pub disallow_in_list: bool,
    /// The normalized selectors left out (`ignoreSelectors`).
    pub ignore: Vec<Matcher>,
}

impl NoDuplicateSelectors {
    /// The project's configuration.
    pub const PROJECT: Self = Self {
        disallow_in_list: false,
        ignore: Vec::new(),
    };
}

/// A rule's selectors resolved against the rules around it, each with its own; none when a rule
/// around it is not plain or a selector does not read (`resolveNestedSelectorsForRule`).
fn resolved(sheet: &Context, index: usize) -> Option<Vec<(Selector, usize, Vec<Selector>)>> {
    let node = sheet.node(index);
    let own_text = sheet.selector(node);
    let own_parsed = selector::parse(own_text).ok()?;
    let own = read(&own_parsed, own_text, &own_parsed.top);
    let mut context: Option<Vec<Selector>> = None;
    for ancestor in sheet.sheet.ancestors(index).into_iter().rev() {
        let outer = sheet.node(ancestor);
        if outer.is(Kind::AtRule) && outer.name.eq_ignore_ascii_case("scope") {
            let text = ":where(:scope)";
            let parsed = selector::parse(text).ok()?;
            context = Some(read(&parsed, text, &parsed.top));
            continue;
        }
        if !outer.is(Kind::Rule) {
            continue;
        }
        if !is_standard_rule(outer) {
            return None;
        }
        let text = sheet.selector(outer);
        let parsed = selector::parse(text).ok()?;
        let list = read(&parsed, text, &parsed.top);
        context = Some(match context {
            Some(parent) => resolve(&list, &parent),
            None => list,
        });
    }
    Some(
        own.into_iter()
            .zip(own_parsed.top.iter().copied())
            .map(|(selector, part)| {
                let selectors = match &context {
                    Some(parent) => resolve(std::slice::from_ref(&selector), parent),
                    None => vec![selector.clone()],
                };
                (selector, part, selectors)
            })
            .collect(),
    )
}

/// Normalized selectors seen, each with the line it was first seen at.
type Seen = Vec<(String, usize)>;

/// The index of a rule's context, the at-rule it sits in through rules (the root's none), added
/// when new.
fn context_of(sheet: &Context, index: usize, contexts: &mut Vec<(Option<usize>, Seen)>) -> usize {
    let at_rule = sheet
        .sheet
        .ancestors(index)
        .into_iter()
        .find(|&ancestor| !sheet.node(ancestor).is(Kind::Rule))
        .filter(|&ancestor| sheet.node(ancestor).is(Kind::AtRule));
    contexts
        .iter()
        .position(|(key, _)| *key == at_rule)
        .unwrap_or_else(|| {
            contexts.push((at_rule, Vec::new()));
            contexts.len() - 1
        })
}

impl SheetRule for NoDuplicateSelectors {
    fn name(&self) -> &'static str {
        "stylelint/no-duplicate-selectors"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "rejected",
            "Duplicate selector \"{}\", first used at line {}",
        )]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        // Each context, the at-rule a rule sits in through rules (the root's none), with its
        // normalized selectors and the line of each.
        let mut contexts: Vec<(Option<usize>, Seen)> = Vec::new();
        for (index, node) in sheet.all(Kind::Rule) {
            let parent = sheet.node(node.parent.unwrap_or(0));
            if (parent.is(Kind::AtRule) && is_keyframes(&parent.name)) || !is_standard_rule(node) {
                continue;
            }
            let at = context_of(sheet, index, &mut contexts);
            let rule_line = sheet.lines.line(node.start);
            let text = sheet.selector(node);
            let Ok(parsed) = selector::parse(text) else {
                continue;
            };
            let Some(entries) = resolved(sheet, index) else {
                continue;
            };
            // A selector's line is its first token's, a line feed starting it counting the next
            // line, as `postcss-selector-parser` counts it.
            let line_of = |part: usize| {
                let at = node.start + parsed.part(part).start;
                sheet.lines.line(at) + usize::from(sheet.text.as_bytes().get(at) == Some(&b'\n'))
            };
            let mut complain = |start: usize, written: &str, line: usize| {
                reports.push(
                    Report::new(
                        node.start + start,
                        node.start + start + written.len(),
                        "rejected",
                    )
                    .with(&[written, &line.to_string()]),
                );
            };
            if self.disallow_in_list {
                for (_, part, selectors) in &entries {
                    for each in selectors {
                        let normal = print_list(&normalize(std::slice::from_ref(each)));
                        if matches_any(&self.ignore, &normal) {
                            continue;
                        }
                        match contexts[at].1.iter().find(|(key, _)| *key == normal) {
                            Some((_, line)) => {
                                let (written, start) = stripped(&parsed, text, *part);
                                complain(start, &written, *line);
                            }
                            None => contexts[at].1.push((normal, line_of(*part))),
                        }
                    }
                }
                continue;
            }
            let mut in_list: Vec<(String, usize)> = Vec::new();
            for &part in &parsed.top {
                let own = read(&parsed, text, &[part]);
                let normal = print_list(&normalize(&own));
                if matches_any(&self.ignore, &normal) {
                    continue;
                }
                match in_list.iter().find(|(key, _)| *key == normal) {
                    Some((_, line)) => {
                        let (written, start) = stripped(&parsed, text, part);
                        complain(start, &written, *line);
                    }
                    None => in_list.push((normal, line_of(part))),
                }
            }
            let all: Vec<Selector> = entries
                .iter()
                .flat_map(|(_, _, selectors)| selectors.clone())
                .collect();
            if all.is_empty()
                || all.iter().any(|each| {
                    matches_any(
                        &self.ignore,
                        &print(&normalize(std::slice::from_ref(each))[0]),
                    )
                })
            {
                continue;
            }
            let normal = print_list(&normalize(&all));
            match contexts[at].1.iter().find(|(key, _)| *key == normal) {
                Some((_, line)) => {
                    let line = *line;
                    let written = text.trim();
                    complain(0, written, line);
                }
                None => contexts[at].1.push((normal, rule_line)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateSelectors;
    use crate::scss::rules::tester::{Js, matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/no-duplicate-selectors.txt",
            &|config| {
                let options = secondary(config);
                Ok(Box::new(NoDuplicateSelectors {
                    disallow_in_list: options.and_then(|each| each.get("disallowInList"))
                        == Some(&Js::Bool(true)),
                    ignore: matchers(options, "ignoreSelectors")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (26, 34, 10),
            "{:#?}",
            tally.apart
        );
    }
}
