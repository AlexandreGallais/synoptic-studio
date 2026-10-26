//! `selector-pseudo-element-colon-notation` of Stylelint 17.16.0
//! (`lib/rules/selector-pseudo-element-colon-notation/index.mjs`, REF-STYLELINT-SOURCE): a
//! pseudo-element of CSS 1 and 2 (`before`, `after`, `first-line`, `first-letter`) written with one
//! colon where the option wants two (`"double"`, the project's), or the reverse, reported over its
//! colons and fixed by rewriting them.

use super::reference::{LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS, has};
use super::{Context, Report, SheetRule, is_standard_rule};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct SelectorPseudoElementColonNotation {
    /// Whether two colons are wanted (`"double"`), else one (`"single"`).
    pub double: bool,
}

impl SelectorPseudoElementColonNotation {
    /// The project's configuration: `"double"`.
    pub const PROJECT: Self = Self { double: true };
}

impl SheetRule for SelectorPseudoElementColonNotation {
    fn name(&self) -> &'static str {
        "stylelint/selector-pseudo-element-colon-notation"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected {} colon pseudo-element notation")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            if !node.selector.contains(':') || !is_standard_rule(node) {
                continue;
            }
            let text = sheet.selector(node);
            let Ok(parsed) = selector::parse(text) else {
                continue;
            };
            for each in parsed.walk() {
                let pseudo = parsed.part(each);
                if pseudo.kind != Part::Pseudo {
                    continue;
                }
                let element = pseudo.value.replace(':', "");
                if !has(LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS, &element.to_lowercase()) {
                    continue;
                }
                let is_double = pseudo.value.starts_with("::");
                if is_double == self.double {
                    continue;
                }
                let at = node.start + pseudo.start;
                let colons = if self.double { "::" } else { ":" };
                let option = if self.double { "double" } else { "single" };
                reports.push(
                    Report::new(at, at + if is_double { 2 } else { 1 }, "expected")
                        .with(&[option])
                        .fixed(at, at + pseudo.value.len(), format!("{colons}{element}"))
                        .scoped(node.start, node.end.unwrap_or(sheet.text.len())),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorPseudoElementColonNotation;
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-pseudo-element-colon-notation.txt",
            &|config| {
                Ok(Box::new(SelectorPseudoElementColonNotation {
                    double: primary(config) == &Js::Text("double".to_owned()),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (27, 14, 1),
            "{:#?}",
            tally.apart
        );
    }
}
