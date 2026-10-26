//! `custom-property-pattern` of Stylelint 17.16.0 (`lib/rules/custom-property-pattern/index.mjs`,
//! REF-STYLELINT-SOURCE): a custom property not matching the pattern once its `--` dropped, where
//! it is declared, read by `var()` or named by `@property`, reported over its name; the project's
//! pattern stylelint-config-standard's kebab-case.

use super::{Context, Pattern, Report, SheetRule, is_standard_property};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its pattern.
pub struct CustomPropertyPattern {
    /// The pattern.
    pub pattern: Pattern,
}

impl CustomPropertyPattern {
    /// The project's configuration: kebab-case.
    #[must_use]
    pub fn project() -> Self {
        Self {
            pattern: Pattern::kebab(),
        }
    }

    /// Whether a property passes: not a custom property, not plain CSS, or matching.
    fn passes(&self, property: &str) -> bool {
        !property.starts_with("--")
            || !is_standard_property(property)
            || self.pattern.matches(&property[2..])
    }
}

impl SheetRule for CustomPropertyPattern {
    fn name(&self) -> &'static str {
        "stylelint/custom-property-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to match pattern \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut report = |at: usize, name: &str| {
            reports.push(
                Report::new(at, at + name.len(), "expected").with(&[name, &self.pattern.text]),
            );
        };
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            if node.is(Kind::Declaration) {
                if node.value.to_ascii_lowercase().contains("var(") {
                    let parsed = value::parse(&node.value);
                    let start = sheet.value_start(node);
                    parsed.walk(&mut |_, part| {
                        if part.kind == Part::Function
                            && part.value.eq_ignore_ascii_case("var")
                            && let Some(&first) = part.nodes.first()
                        {
                            let first = parsed.part(first);
                            if !self.passes(&first.value) {
                                report(start + first.start, &first.value);
                            }
                        }
                        true
                    });
                }
                if !self.passes(&node.prop) {
                    report(node.start, &node.prop);
                }
            } else if node.is(Kind::AtRule)
                && node.name.eq_ignore_ascii_case("property")
                && !self.passes(&node.params)
            {
                report(sheet.params_start(node), &node.params);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CustomPropertyPattern;
    use crate::scss::rules::tester::{pattern, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/custom-property-pattern.txt",
            &|config| {
                Ok(Box::new(CustomPropertyPattern {
                    pattern: pattern(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (15, 13, 0),
            "{:#?}",
            tally.apart
        );
        assert!(CustomPropertyPattern::project().passes("--surface-panel"));
    }
}
