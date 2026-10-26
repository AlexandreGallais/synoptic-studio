//! `layer-name-pattern` of Stylelint 17.16.0 (`lib/rules/layer-name-pattern/index.mjs`,
//! REF-STYLELINT-SOURCE): each word of a `@layer`'s parameters, and of `layer(…)` in an `@import`,
//! not matching the pattern, reported over it; the project's pattern stylelint-config-standard's,
//! kebab-case with dots between levels.

use super::{Context, Pattern, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its pattern.
pub struct LayerNamePattern {
    /// The pattern.
    pub pattern: Pattern,
}

impl LayerNamePattern {
    /// The project's configuration: `^([a-z][a-z0-9]*)([.-][a-z0-9]+)*$`.
    #[must_use]
    pub fn project() -> Self {
        Self {
            pattern: Pattern::new("^([a-z][a-z0-9]*)([.-][a-z0-9]+)*$"),
        }
    }
}

impl SheetRule for LayerNamePattern {
    fn name(&self) -> &'static str {
        "stylelint/layer-name-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to match pattern \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            let layer = node.name.eq_ignore_ascii_case("layer");
            let import = node.name.eq_ignore_ascii_case("import")
                && node.params.to_ascii_lowercase().contains("layer(");
            if !(layer && !node.params.is_empty() || import) {
                continue;
            }
            let start = sheet.params_start(node);
            let parsed = value::parse(&node.params);
            let mut check = |part: &value::Part| {
                if part.kind == Part::Word && !self.pattern.matches(&part.value) {
                    let at = start + part.start;
                    reports.push(
                        Report::new(at, at + part.value.len(), "expected")
                            .with(&[&part.value, &self.pattern.text]),
                    );
                }
            };
            if layer {
                parsed.walk(&mut |_, part| {
                    check(part);
                    true
                });
            } else {
                parsed.walk(&mut |_, part| {
                    if part.kind == Part::Function && part.value.eq_ignore_ascii_case("layer") {
                        for &child in &part.nodes {
                            check(parsed.part(child));
                        }
                    }
                    true
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LayerNamePattern;
    use crate::scss::rules::tester::{pattern, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/layer-name-pattern.txt",
            &|config| {
                Ok(Box::new(LayerNamePattern {
                    pattern: pattern(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (9, 10, 0),
            "{:#?}",
            tally.apart
        );
        assert!(LayerNamePattern::project().pattern.matches("base.theme"));
    }
}
