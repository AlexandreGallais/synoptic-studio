//! `selector-max-id` of Stylelint 17.16.0 (`lib/rules/selector-max-id/index.mjs`,
//! REF-STYLELINT-SOURCE): each selector of a plain rule, as written, holding more ids than the
//! limit, those inside a context-functional pseudo-class its option leaves out not counted,
//! reported over the selector without the comments at its ends. The project's limit 0: the
//! components select by class (AUD-060).

use super::keyframe_block_no_duplicate_selectors::stripped;
use super::reference::{
    A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES, LOGICAL_COMBINATIONS_PSEUDO_CLASSES, has,
};
use super::selectors::pseudo_name;
use super::{Context, Matcher, Report, SheetRule, is_standard_rule, matches_any};
use crate::scss::selector::{self, Kind as Part, Selectors};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct SelectorMaxId {
    /// The most ids a selector may hold.
    pub max: usize,
    /// The context-functional pseudo-classes whose ids are not counted
    /// (`ignoreContextFunctionalPseudoClasses`).
    pub ignore: Vec<Matcher>,
}

impl SelectorMaxId {
    /// The project's configuration: no id, nothing left out.
    #[must_use]
    pub const fn project() -> Self {
        Self {
            max: 0,
            ignore: Vec::new(),
        }
    }

    /// Whether a part sits inside a context-functional pseudo-class the option leaves out
    /// (`isInIgnoredContextFunctionalPseudoClass`), the part itself included.
    fn is_ignored(&self, parsed: &Selectors, index: usize) -> bool {
        let mut current = Some(index);
        while let Some(at) = current {
            let part = parsed.part(at);
            if part.kind == Part::Pseudo && matches_any(&self.ignore, &part.value) {
                let name = pseudo_name(&part.value);
                if has(LOGICAL_COMBINATIONS_PSEUDO_CLASSES, &name)
                    || has(A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES, &name)
                {
                    return true;
                }
            }
            current = part.parent;
        }
        false
    }
}

/// Whether a part lies under another, or is it.
fn within(parsed: &Selectors, index: usize, ancestor: usize) -> bool {
    let mut current = Some(index);
    while let Some(at) = current {
        if at == ancestor {
            return true;
        }
        current = parsed.part(at).parent;
    }
    false
}

impl SheetRule for SelectorMaxId {
    fn name(&self) -> &'static str {
        "stylelint/selector-max-id"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Too many ID selectors in \"{}\", maximum {}")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            let text = sheet.selector(node);
            if !text.contains('#') || !is_standard_rule(node) {
                continue;
            }
            let Ok(parsed) = selector::parse(text) else {
                continue;
            };
            let parts = parsed.walk();
            for &each in &parsed.top {
                let count = parts
                    .iter()
                    .filter(|&&part| {
                        parsed.part(part).kind == Part::Id
                            && within(&parsed, part, each)
                            && !self.is_ignored(&parsed, part)
                    })
                    .count();
                if count > self.max {
                    let (written, start) = stripped(&parsed, text, each);
                    let at = node.start + start;
                    reports.push(
                        Report::new(at, at + written.len(), "expected")
                            .with(&[&written, &self.max.to_string()]),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorMaxId;
    use crate::scss::rules::tester::{Js, matchers, primary, reports, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-max-id.txt",
            &|config| {
                let Js::Number(max) = primary(config) else {
                    return Err("an option value not read");
                };
                Ok(Box::new(SelectorMaxId {
                    max: format!("{max}").parse().unwrap_or(0),
                    ignore: matchers(secondary(config), "ignoreContextFunctionalPseudoClasses")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (34, 35, 7),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn f53_ac2_refuses_any_id_and_prints_the_selector_then_the_limit() {
        // Own case: the message's two arguments in the original's order, the selector first.
        assert_eq!(
            reports(&SelectorMaxId::project(), ".a, #b .c {}"),
            [(
                1,
                5,
                "Too many ID selectors in \"#b .c\", maximum 0".to_owned()
            )]
        );
    }
}
