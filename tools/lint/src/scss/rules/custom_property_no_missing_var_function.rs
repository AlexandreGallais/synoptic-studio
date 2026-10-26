//! `custom-property-no-missing-var-function` of Stylelint 17.16.0
//! (`lib/rules/custom-property-no-missing-var-function/index.mjs`, REF-STYLELINT-SOURCE): a word of
//! a declaration's value naming a custom property the sheet declares or registers by `@property`,
//! outside `var()`'s first argument, the properties taking dashed identifiers, `running()`
//! without `var()`, `anchor()`'s anchor name and `style()`'s property name, reported over it.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part, Value};

/// The rule.
pub struct CustomPropertyNoMissingVarFunction;

/// The properties whose values name things with dashed identifiers.
const IGNORED_PROPERTIES: &[&str] = &[
    "anchor-name",
    "anchor-scope",
    "animation",
    "animation-name",
    "animation-timeline",
    "container-name",
    "counter-increment",
    "counter-reset",
    "counter-set",
    "grid-column",
    "grid-column-end",
    "grid-column-start",
    "grid-row",
    "grid-row-end",
    "grid-row-start",
    "list-style",
    "list-style-type",
    "position-anchor",
    "position-try",
    "position-try-fallbacks",
    "timeline-scope",
    "transition",
    "transition-property",
    "view-transition-name",
    "will-change",
];

/// Whether a part is a word starting `--`.
fn is_dashed(part: &value::Part) -> bool {
    part.kind == Part::Word && part.value.starts_with("--")
}

impl CustomPropertyNoMissingVarFunction {
    /// Checks a part of a value, its arguments as the original drills into them; each name found
    /// with its offset in the value.
    fn check(parsed: &Value, index: usize, known: &[String], found: &mut Vec<(usize, String)>) {
        let part = parsed.part(index);
        if part.kind == Part::Function {
            let name = part.value.to_lowercase();
            let nodes = &part.nodes;
            let arguments: Vec<usize> = match name.as_str() {
                "var" => nodes.iter().skip(1).copied().collect(),
                "running" => match nodes.first().map(|&first| parsed.part(first)) {
                    Some(child)
                        if child.kind == Part::Function
                            && child.value.eq_ignore_ascii_case("var") =>
                    {
                        child.nodes.iter().skip(1).copied().collect()
                    }
                    _ => return,
                },
                "anchor" | "anchor-size" => {
                    let comma = nodes.iter().position(|&each| {
                        let each = parsed.part(each);
                        each.kind == Part::Div && each.value == ","
                    });
                    nodes
                        .iter()
                        .enumerate()
                        .filter(|(at, each)| {
                            !(is_dashed(parsed.part(**each))
                                && comma.is_none_or(|comma| *at < comma))
                        })
                        .map(|(_, each)| *each)
                        .collect()
                }
                "style" => {
                    let mut colon = false;
                    for &each in nodes {
                        let argument = parsed.part(each);
                        if argument.kind == Part::Div && argument.value == ":" {
                            colon = true;
                        } else if colon {
                            Self::check(parsed, each, known, found);
                        }
                    }
                    return;
                }
                _ => nodes.clone(),
            };
            for argument in arguments {
                Self::check(parsed, argument, known, found);
            }
            return;
        }
        if !is_dashed(part) {
            return;
        }
        let clean = part.value.trim_end_matches(';');
        if known.iter().any(|each| each == clean) {
            found.push((part.start, clean.to_owned()));
        }
    }
}

impl SheetRule for CustomPropertyNoMissingVarFunction {
    fn name(&self) -> &'static str {
        "stylelint/custom-property-no-missing-var-function"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Missing var function for \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut known: Vec<String> = Vec::new();
        for (_, node) in sheet.all(Kind::AtRule) {
            if node.name.eq_ignore_ascii_case("property") {
                known.push(node.params.clone());
            }
        }
        for (_, node) in sheet.all(Kind::Declaration) {
            if node.prop.starts_with("--") {
                known.push(node.prop.clone());
            }
        }
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.value.contains("--")
                || IGNORED_PROPERTIES.contains(&node.prop.to_lowercase().as_str())
            {
                continue;
            }
            let parsed = value::parse(&node.value);
            let mut found = Vec::new();
            for &top in &parsed.nodes {
                Self::check(&parsed, top, &known, &mut found);
            }
            let start = sheet.value_start(node);
            for (at, name) in found {
                let at = start + at;
                reports.push(Report::new(at, at + name.len(), "rejected").with(&[&name]));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CustomPropertyNoMissingVarFunction;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/custom-property-no-missing-var-function.txt",
            &|_| Ok(Box::new(CustomPropertyNoMissingVarFunction)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (30, 16, 0),
            "{:#?}",
            tally.apart
        );
    }
}
