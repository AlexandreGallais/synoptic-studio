//! `selector-pseudo-element-no-unknown` of Stylelint 17.16.0
//! (`lib/rules/selector-pseudo-element-no-unknown/index.mjs`, REF-STYLELINT-SOURCE): a
//! pseudo-element (`::name`) of a plain rule neither known nor prefixed, but those its option
//! ignores, reported over it; Angular's deprecated `::ng-deep` among them.

use super::prefixed::prefix;
use super::reference::{PSEUDO_ELEMENTS, has};
use super::{
    Context, Matcher, Report, SheetRule, is_standard_rule, is_standard_selector, matches_any,
};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct SelectorPseudoElementNoUnknown {
    /// The pseudo-elements left out, without their colons (`ignorePseudoElements`).
    pub ignore: Vec<Matcher>,
}

impl SelectorPseudoElementNoUnknown {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

impl SheetRule for SelectorPseudoElementNoUnknown {
    fn name(&self) -> &'static str {
        "stylelint/selector-pseudo-element-no-unknown"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unknown pseudo-element selector \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            let text = sheet.selector(node);
            if !node.selector.contains(':') || !is_standard_rule(node) {
                continue;
            }
            let Ok(parsed) = selector::parse(text) else {
                continue;
            };
            for each in parsed.walk() {
                let pseudo = parsed.part(each);
                let value = &pseudo.value;
                if pseudo.kind != Part::Pseudo
                    || !is_standard_selector(value)
                    || !value.starts_with("::")
                {
                    continue;
                }
                let name = &value[2..];
                if matches_any(&self.ignore, name)
                    || !prefix(name).is_empty()
                    || has(PSEUDO_ELEMENTS, &name.to_lowercase())
                {
                    continue;
                }
                let at = node.start + pseudo.start;
                reports.push(Report::new(at, at + value.len(), "rejected").with(&[value]));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorPseudoElementNoUnknown;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-pseudo-element-no-unknown.txt",
            &|config| {
                Ok(Box::new(SelectorPseudoElementNoUnknown {
                    ignore: matchers(secondary(config), "ignorePseudoElements")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (43, 13, 1),
            "{:#?}",
            tally.apart
        );
    }
}
