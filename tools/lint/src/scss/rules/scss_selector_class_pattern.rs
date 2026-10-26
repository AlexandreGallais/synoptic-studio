//! `selector-class-pattern` of stylelint-scss 7.3.0 (`src/rules/selector-class-pattern/index.js`,
//! REF-STYLELINT-SOURCE): a class not matching the pattern, reported over it; with
//! `resolveNestedSelectors` (the project's), a selector gluing `&` to a name (`&-icon`) resolved
//! against the rules around it first. The project's pattern stylelint-config-standard's
//! kebab-case.

use super::scss_selectors::{
    has_interpolating_ampersand, is_standard_rule, is_standard_syntax_selector, resolve,
};
use super::{Context, Pattern, Report, SheetRule};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct SelectorClassPattern {
    /// The pattern.
    pub pattern: Pattern,
    /// Whether nested selectors are resolved (`resolveNestedSelectors`).
    pub resolve_nested: bool,
}

impl SelectorClassPattern {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        Self {
            pattern: Pattern::kebab(),
            resolve_nested: true,
        }
    }

    /// Reports the classes of a selector.
    fn check_selector(&self, start: usize, text: &str, reports: &mut Vec<Report>) {
        let Ok(parsed) = selector::parse(text) else {
            return;
        };
        for each in parsed.walk() {
            let part = parsed.part(each);
            if part.kind != Part::Class || self.pattern.matches(&part.value) {
                continue;
            }
            let shown = parsed.text(text, each).trim();
            let at = start + part.start;
            reports.push(
                Report::new(at, at + shown.len(), "expected").with(&[shown, &self.pattern.text]),
            );
        }
    }
}

impl SheetRule for SelectorClassPattern {
    fn name(&self) -> &'static str {
        "scss/selector-class-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to match pattern \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Rule) {
            let parent = sheet.node(node.parent.unwrap_or(0));
            if !is_standard_rule(sheet, index)
                || (parent.is(Kind::AtRule) && parent.name == "keyframes")
            {
                continue;
            }
            let written = sheet.selector(node);
            if self.resolve_nested && has_interpolating_ampersand(written) {
                for nested in resolve(sheet, written, index) {
                    if is_standard_syntax_selector(&nested) {
                        self.check_selector(node.start, &nested, reports);
                    }
                }
            } else {
                self.check_selector(node.start, written, reports);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorClassPattern;
    use crate::scss::rules::tester::{Js, pattern, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/selector-class-pattern.txt",
            &|config| {
                Ok(Box::new(SelectorClassPattern {
                    pattern: pattern(config)?,
                    resolve_nested: secondary(config)
                        .and_then(|each| each.get("resolveNestedSelectors"))
                        == Some(&Js::Bool(true)),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (42, 14, 9),
            "{:#?}",
            tally.apart
        );
    }
}
