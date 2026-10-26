//! `selector-no-deprecated` of Stylelint 17.16.0 (`lib/rules/selector-no-deprecated/index.mjs`,
//! REF-STYLELINT-SOURCE): a deprecated pseudo-class, pseudo-element or type selector of a plain
//! rule, but those its option ignores, reported over it; one with a standard form
//! (`:-moz-any` for `:is`) fixed by rewriting it.

use super::reference::{
    DEPRECATED_HTML_TYPE_SELECTORS, DEPRECATED_PSEUDO_CLASSES, DEPRECATED_PSEUDO_ELEMENTS,
    DEPRECATED_SVG_TYPE_SELECTORS, has,
};
use super::selectors::is_standard_type_selector;
use super::{
    Context, Matcher, Report, SheetRule, is_standard_rule, is_standard_selector, matches_any,
};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct SelectorNoDeprecated {
    /// The selectors left out, without their colons (`ignoreSelectors`).
    pub ignore: Vec<Matcher>,
}

impl SelectorNoDeprecated {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

/// The standard form of a deprecated pseudo-class.
fn class_remap(name: &str) -> Option<&'static str> {
    match name {
        "focus-ring" => Some("focus-visible"),
        "matches" | "-webkit-any" | "-moz-any" => Some("is"),
        "top-layer" | "popup-open" => Some("open"),
        "user-error" => Some("user-invalid"),
        _ => None,
    }
}

impl SheetRule for SelectorNoDeprecated {
    fn name(&self) -> &'static str {
        "stylelint/selector-no-deprecated"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected \"{}\" to be \"{}\""),
            ("rejected", "Deprecated selector \"{}\""),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            if !is_standard_rule(node) {
                continue;
            }
            let text = sheet.selector(node);
            let Ok(parsed) = selector::parse(text) else {
                continue;
            };
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            let mut pseudos = Vec::new();
            let mut tags = Vec::new();
            for each in parsed.walk() {
                let part = parsed.part(each);
                let at = node.start + part.start;
                let value = &part.value;
                if part.kind == Part::Pseudo {
                    if !is_standard_selector(value) || value.starts_with(":--") {
                        continue;
                    }
                    let element = value.starts_with("::");
                    let named = &value[if element { 2 } else { 1 }..];
                    let normalized = named.to_lowercase();
                    let mapped = if element {
                        if !has(DEPRECATED_PSEUDO_ELEMENTS, &normalized) {
                            continue;
                        }
                        (normalized == "content").then(|| "::slotted".to_owned())
                    } else {
                        if !has(DEPRECATED_PSEUDO_CLASSES, &normalized) {
                            continue;
                        }
                        class_remap(&normalized).map(|remap| format!(":{remap}"))
                    };
                    if matches_any(&self.ignore, named) {
                        continue;
                    }
                    let report =
                        Report::new(at, at + value.len(), "rejected").scoped(scope.0, scope.1);
                    pseudos.push(match mapped {
                        Some(mapped) => Report {
                            key: "expected",
                            ..report
                                .with(&[value, &mapped])
                                .fixed(at, at + value.len(), mapped)
                        },
                        None => report.with(&[value]),
                    });
                } else if part.kind == Part::Tag
                    && is_standard_type_selector(&parsed, each)
                    && !matches_any(&self.ignore, value)
                    && (has(DEPRECATED_HTML_TYPE_SELECTORS, &value.to_lowercase())
                        || has(DEPRECATED_SVG_TYPE_SELECTORS, value))
                {
                    tags.push(Report::new(at, at + value.len(), "rejected").with(&[value]));
                }
            }
            reports.extend(pseudos);
            reports.extend(tags);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorNoDeprecated;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-no-deprecated.txt",
            &|config| {
                Ok(Box::new(SelectorNoDeprecated {
                    ignore: matchers(secondary(config), "ignoreSelectors")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (12, 17, 0),
            "{:#?}",
            tally.apart
        );
    }
}
