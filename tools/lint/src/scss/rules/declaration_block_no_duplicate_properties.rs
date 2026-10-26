//! `declaration-block-no-duplicate-properties` of Stylelint 17.16.0
//! (`lib/rules/declaration-block-no-duplicate-properties/index.mjs`, REF-STYLELINT-SOURCE): a
//! property declared again in its block (custom properties, `src` and the properties its option
//! ignores left out), reported over the name of the one that loses, the earlier unless only it is
//! `!important`, and fixed by removing it. Its options `ignore: ["consecutive-duplicates"]`,
//! `["consecutive-duplicates-with-different-values"]` and
//! `["consecutive-duplicates-with-same-prefixless-values"]` are ported; the project sets none.
//! `["consecutive-duplicates-with-different-syntaxes"]`, which needs `CSSTree`, is not.

use super::prefixed::prefix;
use super::{Context, Matcher, Report, SheetRule, is_standard_property, matches_any};
use crate::scss::tree::Node;

/// What consecutive duplicates the rule lets pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ignore {
    /// None: every duplicate reported.
    Nothing,
    /// Any consecutive duplicate (`consecutive-duplicates`).
    Consecutive,
    /// Consecutive duplicates of different values (`…-with-different-values`).
    DifferentValues,
    /// Consecutive duplicates whose values are equal once unprefixed
    /// (`…-with-same-prefixless-values`).
    SamePrefixlessValues,
}

/// The rule and its options.
pub struct DeclarationBlockNoDuplicateProperties {
    /// The consecutive duplicates let pass.
    pub ignore: Ignore,
    /// The properties left out (`ignoreProperties`).
    pub ignore_properties: Vec<Matcher>,
}

impl DeclarationBlockNoDuplicateProperties {
    /// The project's configuration: every duplicate reported.
    pub const PROJECT: Self = Self {
        ignore: Ignore::Nothing,
        ignore_properties: Vec::new(),
    };
}

/// A value without its leading vendor prefix (`vendor.unprefixed`).
fn unprefixed(value: &str) -> &str {
    &value[prefix(value).len()..]
}

impl SheetRule for DeclarationBlockNoDuplicateProperties {
    fn name(&self) -> &'static str {
        "stylelint/declaration-block-no-duplicate-properties"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Duplicate property \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let report = |node: &Node, reports: &mut Vec<Report>| {
            let (start, end) = sheet.word_range(node, &node.prop);
            let (from, to) = sheet.removal(node);
            let parent = sheet.node(node.parent.unwrap_or(0));
            reports.push(
                Report::new(start, end, "rejected")
                    .with(&[&node.prop])
                    .fixed(from, to, "")
                    .scoped(parent.start, parent.end.unwrap_or(sheet.text.len())),
            );
        };
        for block in sheet.declaration_blocks() {
            let mut kept: Vec<usize> = Vec::new();
            let mut previous = String::new();
            for index in block {
                let node = sheet.node(index);
                let lower = node.prop.to_lowercase();
                if !is_standard_property(&node.prop)
                    || node.prop.starts_with("--")
                    || matches_any(&self.ignore_properties, &node.prop)
                    || lower == "src"
                {
                    continue;
                }
                let duplicate = kept
                    .iter()
                    .position(|&each| sheet.node(each).prop.to_lowercase() == lower);
                let consecutive = previous == lower;
                previous.clone_from(&lower);
                let Some(at) = duplicate else {
                    kept.push(index);
                    continue;
                };
                let earlier = sheet.node(kept[at]);
                let earlier_wins = !node.important && earlier.important;
                let fix_or_report = |kept: &mut Vec<usize>, reports: &mut Vec<Report>| {
                    if earlier_wins {
                        report(node, reports);
                    } else {
                        report(earlier, reports);
                        kept[at] = index;
                    }
                };
                match self.ignore {
                    Ignore::DifferentValues | Ignore::SamePrefixlessValues => {
                        let same_prefixless = unprefixed(&node.value) == unprefixed(&earlier.value);
                        if !consecutive
                            || (self.ignore == Ignore::SamePrefixlessValues && !same_prefixless)
                        {
                            fix_or_report(&mut kept, reports);
                            continue;
                        }
                        if node.value != earlier.value {
                            continue;
                        }
                        kept[at] = index;
                        report(earlier, reports);
                    }
                    Ignore::Consecutive if consecutive => {}
                    Ignore::Nothing | Ignore::Consecutive => fix_or_report(&mut kept, reports),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeclarationBlockNoDuplicateProperties, Ignore};
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/declaration-block-no-duplicate-properties.txt",
            &|config| {
                let options = secondary(config);
                let ignore = options
                    .and_then(|each| each.texts("ignore"))
                    .unwrap_or_default();
                let ignore = match ignore.first().map(String::as_str) {
                    None => Ignore::Nothing,
                    Some("consecutive-duplicates") => Ignore::Consecutive,
                    Some("consecutive-duplicates-with-different-values") => Ignore::DifferentValues,
                    Some("consecutive-duplicates-with-same-prefixless-values") => {
                        Ignore::SamePrefixlessValues
                    }
                    Some(_) => {
                        return Err(
                            "an option needing CSSTree: consecutive duplicates of different syntaxes",
                        );
                    }
                };
                Ok(Box::new(DeclarationBlockNoDuplicateProperties {
                    ignore,
                    ignore_properties: matchers(options, "ignoreProperties")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (28, 52, 44),
            "{:#?}",
            tally.apart
        );
    }
}
