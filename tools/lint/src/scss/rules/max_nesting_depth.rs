//! `max-nesting-depth` of Stylelint 17.16.0 (`lib/rules/max-nesting-depth/index.mjs`,
//! REF-STYLELINT-SOURCE): a rule or an at-rule with a block nested deeper than the limit, counted
//! from the root, an at-rule at the root not counting, reported over the whole node; its options
//! leave out at-rules, rules, blockless at-rules and pseudo-class rules. The project's limit 3.

use super::{Context, Matcher, Report, SheetRule, is_standard_rule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct MaxNestingDepth {
    /// The deepest nesting allowed.
    pub depth: usize,
    /// The at-rules left out, and their content not counted (`ignoreAtRules`).
    pub ignore_at_rules: Vec<Matcher>,
    /// The rules not counted (`ignoreRules`).
    pub ignore_rules: Vec<Matcher>,
    /// The pseudo-classes whose `&:…` rules are not counted (`ignorePseudoClasses`).
    pub ignore_pseudo_classes: Option<Vec<Matcher>>,
    /// Whether at-rules holding no declaration are not counted (`ignore: ["blockless-at-rules"]`).
    pub ignore_blockless_at_rules: bool,
    /// Whether `&:…` rules are not counted (`ignore: ["pseudo-classes"]`).
    pub ignore_pseudo_class_rules: bool,
}

impl MaxNestingDepth {
    /// The project's configuration: 3, nothing left out.
    #[must_use]
    pub const fn project() -> Self {
        Self {
            depth: 3,
            ignore_at_rules: Vec::new(),
            ignore_rules: Vec::new(),
            ignore_pseudo_classes: None,
            ignore_blockless_at_rules: false,
            ignore_pseudo_class_rules: false,
        }
    }

    /// Whether a node is an at-rule left out.
    fn is_ignored_at_rule(&self, sheet: &Context, index: usize) -> bool {
        let node = sheet.node(index);
        node.is(Kind::AtRule) && matches_any(&self.ignore_at_rules, &node.name)
    }

    /// Whether a node does not count toward the depth.
    fn is_transparent(&self, sheet: &Context, index: usize) -> bool {
        let node = sheet.node(index);
        let selectors: Vec<&str> = node.selector.split(',').map(str::trim).collect();
        if self.ignore_blockless_at_rules
            && node.is(Kind::AtRule)
            && node
                .nodes()
                .iter()
                .all(|&child| !sheet.node(child).is(Kind::Declaration))
        {
            return true;
        }
        if self.ignore_pseudo_class_rules && node.is(Kind::Rule) {
            let normalized: String = node
                .selector
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let only_pseudo = normalized
                .split(',')
                .filter(|each| !matches_any(&self.ignore_rules, each))
                .all(|each| pseudo_rule(each).is_some());
            if only_pseudo {
                return true;
            }
        }
        node.is(Kind::Rule)
            && (self.ignore_pseudo_classes.is_some() || !self.ignore_rules.is_empty())
            && selectors.iter().all(|each| {
                if matches_any(&self.ignore_rules, each) {
                    return true;
                }
                match (&self.ignore_pseudo_classes, pseudo_rule(each)) {
                    (Some(pseudo), Some(class)) => matches_any(pseudo, class),
                    _ => false,
                }
            })
    }

    /// How deep a node is nested (`nestingDepth`).
    fn nesting(&self, sheet: &Context, mut index: usize) -> usize {
        let mut level = 0;
        loop {
            let Some(parent) = sheet.node(index).parent else {
                return 0;
            };
            if self.is_ignored_at_rule(sheet, parent) {
                return 0;
            }
            let outer = sheet.node(parent);
            let at_root = outer.is(Kind::Root)
                || (outer.is(Kind::AtRule)
                    && outer
                        .parent
                        .is_some_and(|top| sheet.node(top).is(Kind::Root)));
            if at_root {
                return level;
            }
            if !self.is_transparent(sheet, index) {
                level += 1;
            }
            index = parent;
        }
    }
}

/// The pseudo-class of a selector `&:name`, none for any other.
fn pseudo_rule(selector: &str) -> Option<&str> {
    selector
        .strip_prefix("&:")
        .filter(|rest| !rest.starts_with(':'))
}

impl SheetRule for MaxNestingDepth {
    fn name(&self) -> &'static str {
        "stylelint/max-nesting-depth"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Too deep nesting, maximum {}")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut rules = Vec::new();
        let mut at_rules = Vec::new();
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            let is_rule = node.is(Kind::Rule);
            if !(is_rule || node.is(Kind::AtRule))
                || self.is_ignored_at_rule(sheet, index)
                || (is_rule && matches_any(&self.ignore_rules, &node.selector))
                || node.children.is_none()
                || (is_rule && !is_standard_rule(node))
            {
                continue;
            }
            if self.nesting(sheet, index) > self.depth {
                let report =
                    Report::new(node.start, node.end.unwrap_or(sheet.text.len()), "expected")
                        .with(&[&self.depth.to_string()]);
                if is_rule {
                    rules.push(report);
                } else {
                    at_rules.push(report);
                }
            }
        }
        // The rules are walked before the at-rules, as the original walks them.
        reports.extend(rules);
        reports.extend(at_rules);
    }
}

#[cfg(test)]
mod tests {
    use super::MaxNestingDepth;
    use crate::scss::rules::tester::{Js, matchers, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/max-nesting-depth.txt",
            &|config| {
                let Js::Number(depth) = primary(config) else {
                    return Err("an option value not read");
                };
                let options = secondary(config);
                let ignore = options
                    .and_then(|each| each.texts("ignore"))
                    .unwrap_or_default();
                let pseudo = options
                    .and_then(|each| each.get("ignorePseudoClasses"))
                    .is_some();
                Ok(Box::new(MaxNestingDepth {
                    depth: format!("{depth}").parse().unwrap_or(0),
                    ignore_at_rules: matchers(options, "ignoreAtRules")?,
                    ignore_rules: matchers(options, "ignoreRules")?,
                    ignore_pseudo_classes: if pseudo {
                        Some(matchers(options, "ignorePseudoClasses")?)
                    } else {
                        None
                    },
                    ignore_blockless_at_rules: ignore
                        .iter()
                        .any(|each| each == "blockless-at-rules"),
                    ignore_pseudo_class_rules: ignore.iter().any(|each| each == "pseudo-classes"),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (37, 29, 1),
            "{:#?}",
            tally.apart
        );
    }
}
