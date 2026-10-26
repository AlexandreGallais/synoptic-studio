//! `selector-type-no-unknown` of Stylelint 17.16.0 (`lib/rules/selector-type-no-unknown/index.mjs`,
//! REF-STYLELINT-SOURCE): a type selector of a plain rule neither an HTML (`html-tags`, the
//! deprecated and experimental ones), an SVG (`svg-tags`, `hatch`) nor a `MathML` element, keyframe
//! selectors, the arguments of the pseudos naming no element and the types its options ignore
//! left out, reported over it. The project's option, stylelint-config-recommended's: custom
//! elements, Angular's components among them, left out.

use super::reference::{HTML_TYPE_SELECTORS, MATHML_TAGS, SVG_TAGS, has};
use super::selectors::{
    argument_of, comma_list, is_custom_element, is_keyframe_selector, is_standard_type_selector,
    may_hold_a_type,
};
use super::{Context, Matcher, Report, SheetRule, is_standard_rule, matches_any};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct SelectorTypeNoUnknown {
    /// Whether custom elements are left out (`ignore: ["custom-elements"]`).
    pub ignore_custom_elements: bool,
    /// The types left out (`ignoreTypes`).
    pub ignore_types: Vec<Matcher>,
}

impl SelectorTypeNoUnknown {
    /// The project's configuration: custom elements left out.
    pub const PROJECT: Self = Self {
        ignore_custom_elements: true,
        ignore_types: Vec::new(),
    };
}

/// The pseudo-elements and pseudo-classes whose arguments name no element.
const IGNORED_PSEUDOS: &[&str] = &[
    "::highlight",
    "::view-transition-group",
    "::view-transition-image-pair",
    "::view-transition-new",
    "::view-transition-old",
    ":active-view-transition-type",
    ":dir",
    ":-moz-locale-dir",
    ":state",
    ":lang",
];

impl SheetRule for SelectorTypeNoUnknown {
    fn name(&self) -> &'static str {
        "stylelint/selector-type-no-unknown"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unknown type selector \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            if !may_hold_a_type(&node.selector)
                || !is_standard_rule(node)
                || comma_list(&node.selector)
                    .iter()
                    .any(|each| is_keyframe_selector(each))
            {
                continue;
            }
            let Ok(parsed) = selector::parse(sheet.selector(node)) else {
                continue;
            };
            for each in parsed.walk() {
                let tag = parsed.part(each);
                if tag.kind != Part::Tag || !is_standard_type_selector(&parsed, each) {
                    continue;
                }
                let name = &tag.value;
                let lower = name.to_lowercase();
                let ignored_pseudo = argument_of(&parsed, each).is_some_and(|pseudo| {
                    IGNORED_PSEUDOS.contains(&parsed.part(pseudo).value.as_str())
                });
                if (self.ignore_custom_elements && is_custom_element(name))
                    || matches_any(&self.ignore_types, name)
                    || ignored_pseudo
                    || has(HTML_TYPE_SELECTORS, &lower)
                    || SVG_TAGS.contains(&name.as_str())
                    || matches!(name.as_str(), "hatch" | "hatchpath" | "hatchPath")
                    || MATHML_TAGS.contains(&lower.as_str())
                {
                    continue;
                }
                let at = node.start + tag.start;
                reports.push(Report::new(at, at + name.len(), "rejected").with(&[name]));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorTypeNoUnknown;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-type-no-unknown.txt",
            &|config| {
                let options = secondary(config);
                let ignore = options
                    .and_then(|each| each.texts("ignore"))
                    .unwrap_or_default();
                if ignore.iter().any(|each| each == "default-namespace")
                    || options
                        .and_then(|each| each.get("ignoreNamespaces"))
                        .is_some()
                {
                    return Err("namespaces, which the selector reader reads as a combinator");
                }
                Ok(Box::new(SelectorTypeNoUnknown {
                    ignore_custom_elements: ignore.iter().any(|each| each == "custom-elements"),
                    ignore_types: matchers(options, "ignoreTypes")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (56, 20, 16),
            "{:#?}",
            tally.apart
        );
    }
}
