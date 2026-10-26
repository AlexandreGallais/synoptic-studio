//! The rules of stylelint-scss 7.3.0 on the nesting selector `&` (REF-STYLELINT-SOURCE):
//! `at-mixin-no-risky-nesting-selector` (`src/rules/at-mixin-no-risky-nesting-selector/index.js`),
//! a `&` after another selector in a rule nested inside a mixin's rule, which takes the selector
//! where the mixin is included; `selector-no-redundant-nesting-selector`
//! (`src/rules/selector-no-redundant-nesting-selector/index.js` and `src/utils/hasNestedSibling.js`),
//! a `&` followed by a combinator, where the nesting alone means it; `selector-no-union-class-name`
//! (`src/rules/selector-no-union-class-name/index.js`), a `&` after a class glued to a name
//! (`&-suffix`), building a class no search finds whole.

use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::javascript::JsText;
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// Which of the three rules.
#[derive(Clone, Debug)]
pub enum Ampersand {
    /// `at-mixin-no-risky-nesting-selector`.
    RiskyInMixin,
    /// `selector-no-redundant-nesting-selector`, with the type selectors after the combinator it
    /// leaves (`ignoreKeywords`).
    Redundant(Vec<Matcher>),
    /// `selector-no-union-class-name`.
    UnionClassName,
}

/// The rule.
pub struct AmpersandRules {
    /// Which.
    pub which: Ampersand,
}

/// Whether a selector of a list has a `&` after something else (`hasNestedParentSelector`).
fn has_nested_parent(selectors: &str) -> bool {
    selectors.split(',').any(|selector| {
        selector
            .match_indices('&')
            .any(|(at, _)| !selector[..at].js_trim_end().is_empty())
    })
}

impl AmpersandRules {
    /// `at-mixin-no-risky-nesting-selector`.
    fn risky(sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Rule) {
            let in_mixin = sheet
                .ancestors(index)
                .iter()
                .any(|each| each.is(Kind::AtRule) && each.name == "mixin");
            let parent = sheet.node(node.parent.unwrap_or(0));
            if node.selector.contains('&')
                && in_mixin
                && has_nested_parent(&node.selector)
                && parent.is(Kind::Rule)
                && !parent.selector.is_empty()
            {
                reports.push(Report::new(
                    node.start,
                    node.end.unwrap_or(sheet.text.len()),
                    "rejected",
                ));
            }
        }
    }

    /// `selector-no-redundant-nesting-selector`.
    fn redundant(sheet: &Context, ignored: &[Matcher], reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            if !node.selector.contains('&') || node.selector == "&" {
                continue;
            }
            let Ok(parsed) = selector::parse(&node.selector) else {
                continue;
            };
            for each in parsed.walk() {
                let part = parsed.part(each);
                if part.kind != Part::Nesting || parsed.previous(each).is_some() {
                    continue;
                }
                let holder = part.parent.map(|at| parsed.part(at));
                let siblings = holder.map_or(&[][..], |holder| &holder.nodes[..]);
                if siblings
                    .iter()
                    .any(|at| *at != each && parsed.part(*at).kind == Part::Nesting)
                {
                    continue;
                }
                let list = holder
                    .and_then(|holder| holder.parent)
                    .map(|at| parsed.part(at));
                let next = parsed.next(each);
                if next.is_none()
                    && (list.map_or(parsed.top.len(), |list| list.nodes.len()) > 1
                        || list.is_some_and(|list| list.kind == Part::Pseudo))
                {
                    continue;
                }
                if next.is_some_and(|at| parsed.part(at).kind != Part::Combinator) {
                    continue;
                }
                let after = next
                    .and_then(|at| parsed.next(at))
                    .map(|at| parsed.part(at));
                if after.is_some_and(|after| {
                    (after.kind == Part::Tag && matches_any(ignored, after.value.js_trim()))
                        || after.kind == Part::Combinator
                }) {
                    continue;
                }
                let at = node.start + part.start;
                reports.push(Report::new(at, at + part.value.len(), "rejected"));
            }
        }
    }

    /// `selector-no-union-class-name`.
    fn union(sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Rule) {
            if !node.selector.contains('&') {
                continue;
            }
            let Some(outer) = sheet
                .ancestors(index)
                .into_iter()
                .find(|each| each.is(Kind::Rule))
            else {
                continue;
            };
            let Ok(parent) = selector::parse(&outer.selector) else {
                continue;
            };
            if parent
                .walk()
                .last()
                .is_none_or(|last| parent.part(*last).kind != Part::Class)
            {
                continue;
            }
            let Ok(parsed) = selector::parse(&node.selector) else {
                continue;
            };
            for each in parsed.walk() {
                let part = parsed.part(each);
                let Some(next) = parsed.next(each) else {
                    continue;
                };
                let glued = matches!(
                    parsed.part(next).kind,
                    Part::Tag | Part::Nesting | Part::Universal | Part::String | Part::Comment
                );
                if part.kind == Part::Nesting && glued {
                    let at = node.start + part.start;
                    reports.push(Report::new(at, at + node.selector.len(), "rejected"));
                }
            }
        }
    }
}

impl SheetRule for AmpersandRules {
    fn name(&self) -> &'static str {
        match self.which {
            Ampersand::RiskyInMixin => "scss/at-mixin-no-risky-nesting-selector",
            Ampersand::Redundant(_) => "scss/selector-no-redundant-nesting-selector",
            Ampersand::UnionClassName => "scss/selector-no-union-class-name",
        }
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        match self.which {
            Ampersand::RiskyInMixin => &[(
                "rejected",
                "Unexpected nested parent selector in @mixin rule",
            )],
            Ampersand::Redundant(_) => &[("rejected", "Unnecessary nesting selector (&)")],
            Ampersand::UnionClassName => &[(
                "rejected",
                "Unexpected union class name with the parent selector (&)",
            )],
        }
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        match &self.which {
            Ampersand::RiskyInMixin => Self::risky(sheet, reports),
            Ampersand::Redundant(ignored) => Self::redundant(sheet, ignored, reports),
            Ampersand::UnionClassName => Self::union(sheet, reports),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Ampersand, AmpersandRules, has_nested_parent};
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_risky() {
        let tally = run(
            "stylelint-scss/at-mixin-no-risky-nesting-selector.txt",
            &|_| {
                Ok(Box::new(AmpersandRules {
                    which: Ampersand::RiskyInMixin,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (12, 3, 0),
            "{:#?}",
            tally.apart
        );
        assert!(has_nested_parent(".a, .b &") && !has_nested_parent("& .a"));
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_redundant() {
        let tally = run(
            "stylelint-scss/selector-no-redundant-nesting-selector.txt",
            &|config| {
                Ok(Box::new(AmpersandRules {
                    which: Ampersand::Redundant(matchers(secondary(config), "ignoreKeywords")?),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (23, 18, 0),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_union() {
        let tally = run(
            "stylelint-scss/selector-no-union-class-name.txt",
            &|_| {
                Ok(Box::new(AmpersandRules {
                    which: Ampersand::UnionClassName,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (11, 3, 1),
            "{:#?}",
            tally.apart
        );
    }
}
