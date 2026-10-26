//! `selector-id-pattern` of Stylelint 17.16.0 (`lib/rules/selector-id-pattern/index.mjs`,
//! REF-STYLELINT-SOURCE): each id of a plain rule's selector not matching the pattern, reported
//! over its `#name`; the project's pattern stylelint-config-standard's kebab-case.

use super::{Context, Pattern, Report, SheetRule, is_standard_rule};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its pattern.
pub struct SelectorIdPattern {
    /// The pattern.
    pub pattern: Pattern,
}

impl SelectorIdPattern {
    /// The project's configuration: kebab-case.
    #[must_use]
    pub fn project() -> Self {
        Self {
            pattern: Pattern::kebab(),
        }
    }
}

impl SheetRule for SelectorIdPattern {
    fn name(&self) -> &'static str {
        "stylelint/selector-id-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to match pattern \"{}\"")]
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
            for index in parsed.walk() {
                let part = parsed.part(index);
                if part.kind != Part::Id || self.pattern.matches(&part.value) {
                    continue;
                }
                let written = parsed.text(text, index).trim();
                let at = node.start + part.start;
                reports.push(
                    Report::new(at, at + written.len(), "expected")
                        .with(&[written, &self.pattern.text]),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorIdPattern;
    use crate::scss::rules::tester::{pattern, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-id-pattern.txt",
            &|config| {
                Ok(Box::new(SelectorIdPattern {
                    pattern: pattern(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (24, 6, 2),
            "{:#?}",
            tally.apart
        );
        assert!(SelectorIdPattern::project().pattern.matches("color-plane"));
    }
}
