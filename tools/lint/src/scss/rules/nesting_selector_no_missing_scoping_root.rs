//! `nesting-selector-no-missing-scoping-root` of Stylelint 17.16.0
//! (`lib/rules/nesting-selector-no-missing-scoping-root/index.mjs`, REF-STYLELINT-SOURCE): each `&`
//! of a plain rule's selector, or of an `@scope`'s parameters, with no rule, `@scope` nor at-rule
//! its option names around it, reported over it. The project's option,
//! stylelint-config-recommended-scss's: `@mixin` a root, a mixin being included in a rule.

use super::{
    Context, Matcher, Report, SheetRule, is_standard_at_rule, is_standard_rule, matches_any,
};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct NestingSelectorNoMissingScopingRoot {
    /// The at-rules that are roots (`ignoreAtRules`).
    pub ignore_at_rules: Vec<Matcher>,
}

impl NestingSelectorNoMissingScopingRoot {
    /// The project's configuration: `@mixin` a root.
    #[must_use]
    pub fn project() -> Self {
        Self {
            ignore_at_rules: vec![Matcher::new("mixin")],
        }
    }

    /// Whether a node has a rule, an `@scope` or an at-rule the option names around it.
    fn has_root(&self, sheet: &Context, index: usize) -> bool {
        sheet.ancestors(index).into_iter().any(|node| {
            node.is(Kind::Rule)
                || (node.is(Kind::AtRule)
                    && (node.name == "scope" || matches_any(&self.ignore_at_rules, &node.name)))
        })
    }
}

impl SheetRule for NestingSelectorNoMissingScopingRoot {
    fn name(&self) -> &'static str {
        "stylelint/nesting-selector-no-missing-scoping-root"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Missing scoping root")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut rules = Vec::new();
        let mut scopes = Vec::new();
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            let (text, offset, found) =
                if node.is(Kind::Rule) && node.selector.contains('&') && is_standard_rule(node) {
                    (sheet.selector(node), node.start, &mut rules)
                } else if node.is(Kind::AtRule)
                    && node.name.eq_ignore_ascii_case("scope")
                    && is_standard_at_rule(node)
                    && node.params.contains('&')
                {
                    (sheet.params(node), sheet.params_start(node), &mut scopes)
                } else {
                    continue;
                };
            if self.has_root(sheet, index) {
                continue;
            }
            let parsed = if node.is(Kind::Rule) {
                selector::parse(text).ok()
            } else {
                scope_selectors(text)
            };
            for (part_start, kind) in parsed.iter().flat_map(|each| {
                each.walk()
                    .into_iter()
                    .map(|at| (each.part(at).start, each.part(at).kind))
            }) {
                if kind == Part::Nesting {
                    found.push(Report::new(
                        offset + part_start,
                        offset + part_start + 1,
                        "rejected",
                    ));
                }
            }
        }
        reports.extend(rules);
        reports.extend(scopes);
    }
}

/// An `@scope`'s parameters read as its selectors: `(start) to (end)`, each read in its
/// parentheses, places kept by blanking the rest.
fn scope_selectors(params: &str) -> Option<selector::Selectors> {
    let mut blanked = String::with_capacity(params.len());
    let mut depth = 0_i32;
    for character in params.chars() {
        match character {
            '(' => {
                depth += 1;
                blanked.push(if depth == 1 { ' ' } else { character });
            }
            ')' => {
                depth -= 1;
                blanked.push(if depth == 0 { ',' } else { character });
            }
            _ if depth > 0 => blanked.push(character),
            _ => blanked.push_str(&" ".repeat(character.len_utf8())),
        }
    }
    selector::parse(blanked.trim_end_matches([',', ' '])).ok()
}

#[cfg(test)]
mod tests {
    use super::NestingSelectorNoMissingScopingRoot;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/nesting-selector-no-missing-scoping-root.txt",
            &|config| {
                Ok(Box::new(NestingSelectorNoMissingScopingRoot {
                    ignore_at_rules: matchers(secondary(config), "ignoreAtRules")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (18, 16, 3),
            "{:#?}",
            tally.apart
        );
    }
}
