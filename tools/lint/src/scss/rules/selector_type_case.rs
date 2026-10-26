//! `selector-type-case` of Stylelint 17.16.0 (`lib/rules/selector-type-case/index.mjs`,
//! REF-STYLELINT-SOURCE): a type selector of a plain rule not in lower case (`"lower"`, the
//! project's) or upper case (`"upper"`), the camel-case SVG elements, keyframe selectors and the
//! types its option ignores left out, reported over it and fixed by rewriting it.

use super::reference::{MIXED_CASE_SVG_TYPE_SELECTORS, has};
use super::selectors::{
    comma_list, is_keyframe_selector, is_standard_type_selector, may_hold_a_type,
};
use super::{Context, Matcher, Report, SheetRule, is_standard_rule, matches_any};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct SelectorTypeCase {
    /// Whether lower case is wanted (`"lower"`), else upper case.
    pub lower: bool,
    /// The types left out (`ignoreTypes`).
    pub ignore_types: Vec<Matcher>,
}

impl SelectorTypeCase {
    /// The project's configuration: `"lower"`.
    pub const PROJECT: Self = Self {
        lower: true,
        ignore_types: Vec::new(),
    };
}

impl SheetRule for SelectorTypeCase {
    fn name(&self) -> &'static str {
        "stylelint/selector-type-case"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to be \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            let text = &node.selector;
            let wrong_case = if self.lower {
                text.bytes().any(|byte| byte.is_ascii_uppercase())
            } else {
                text.bytes().any(|byte| byte.is_ascii_lowercase())
            };
            if !may_hold_a_type(text)
                || !wrong_case
                || !is_standard_rule(node)
                || comma_list(text)
                    .iter()
                    .any(|each| is_keyframe_selector(each))
            {
                continue;
            }
            let raw = sheet.selector(node);
            let Ok(parsed) = selector::parse(raw) else {
                continue;
            };
            for each in parsed.walk() {
                let tag = parsed.part(each);
                if tag.kind != Part::Tag
                    || !is_standard_type_selector(&parsed, each)
                    || has(MIXED_CASE_SVG_TYPE_SELECTORS, &tag.value)
                    || matches_any(&self.ignore_types, &tag.value)
                {
                    continue;
                }
                let expected = if self.lower {
                    tag.value.to_lowercase()
                } else {
                    tag.value.to_uppercase()
                };
                if expected == tag.value {
                    continue;
                }
                let at = node.start + tag.start;
                reports.push(
                    Report::new(at, at + tag.value.len(), "expected")
                        .with(&[&tag.value, &expected])
                        .fixed(at, at + tag.value.len(), expected)
                        .scoped(node.start, node.end.unwrap_or(sheet.text.len())),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorTypeCase;
    use crate::scss::rules::tester::{Js, matchers, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-type-case.txt",
            &|config| {
                Ok(Box::new(SelectorTypeCase {
                    lower: primary(config) == &Js::Text("lower".to_owned()),
                    ignore_types: matchers(secondary(config), "ignoreTypes")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (59, 19, 1),
            "{:#?}",
            tally.apart
        );
    }
}
