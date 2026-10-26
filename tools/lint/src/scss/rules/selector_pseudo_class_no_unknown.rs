//! `selector-pseudo-class-no-unknown` of Stylelint 17.16.0
//! (`lib/rules/selector-pseudo-class-no-unknown/index.mjs`, REF-STYLELINT-SOURCE): a
//! pseudo-class of a plain rule or of `@page` neither known nor prefixed nor custom (`:--`), but
//! those its option ignores; a pseudo-element written with one colon is reported too; a
//! scrollbar state after a `::-webkit-scrollbar…` element, there or in a rule nesting this one,
//! is known. Reported over the pseudo-class.

use super::prefixed::prefix;
use super::reference::{
    AT_RULE_PAGE_PSEUDO_CLASSES, LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS, PSEUDO_CLASSES,
    PSEUDO_ELEMENTS, WEBKIT_SCROLLBAR_PSEUDO_CLASSES, WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS, has,
};
use super::{
    Context, Matcher, Report, SheetRule, is_standard_at_rule, is_standard_rule,
    is_standard_selector, matches_any,
};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct SelectorPseudoClassNoUnknown {
    /// The pseudo-classes left out, without their colon (`ignorePseudoClasses`).
    pub ignore: Vec<Matcher>,
}

impl SelectorPseudoClassNoUnknown {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

/// Whether a rule nesting a node has a `::-webkit-scrollbar…` element in its selector.
fn has_scrollbar_ancestor(sheet: &Context, index: usize) -> bool {
    sheet.ancestors(index).into_iter().any(|node| {
        if !node.is(Kind::Rule) {
            return false;
        }
        let text = sheet.selector(node);
        text.contains("::")
            && selector::parse(text).is_ok_and(|parsed| {
                parsed.walk().into_iter().any(|each| {
                    let part = parsed.part(each);
                    part.kind == Part::Pseudo
                        && part.value.starts_with("::")
                        && has(
                            WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS,
                            &part.value.to_lowercase()[2..],
                        )
                })
            })
    })
}

impl SheetRule for SelectorPseudoClassNoUnknown {
    fn name(&self) -> &'static str {
        "stylelint/selector-pseudo-class-no-unknown"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unknown pseudo-class selector \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            let page = node.is(Kind::AtRule) && node.name == "page" && !node.params.is_empty();
            let (text, offset) = if node.is(Kind::Rule) && is_standard_rule(node) {
                (sheet.selector(node), node.start)
            } else if page && is_standard_at_rule(node) {
                (sheet.params(node), sheet.params_start(node))
            } else {
                continue;
            };
            if !text.contains(':') {
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
                    || value.starts_with(":--")
                    || value.starts_with("::")
                {
                    continue;
                }
                let name = value.trim_start_matches(':').to_lowercase();
                if has(LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS, &name)
                    || matches_any(&self.ignore, &value[1..])
                {
                    continue;
                }
                let prefixed = !prefix(&name).is_empty();
                if page {
                    if has(AT_RULE_PAGE_PSEUDO_CLASSES, &name) {
                        continue;
                    }
                } else if !has(PSEUDO_ELEMENTS, &name) || prefixed {
                    if prefixed || has(PSEUDO_CLASSES, &name) {
                        continue;
                    }
                    let mut previous = parsed.previous(each);
                    let mut crossed = false;
                    let mut nesting = false;
                    let mut element = None;
                    while let Some(at) = previous {
                        let part = parsed.part(at);
                        crossed |= part.kind == Part::Combinator;
                        nesting |= part.kind == Part::Nesting && !crossed;
                        if part.value.starts_with("::") {
                            element = Some(at);
                            break;
                        }
                        previous = parsed.previous(at);
                    }
                    let scrollbar_state = has(WEBKIT_SCROLLBAR_PSEUDO_CLASSES, &name);
                    let known = match element {
                        Some(at) => {
                            has(
                                WEBKIT_SCROLLBAR_PSEUDO_ELEMENTS,
                                &parsed.part(at).value.to_lowercase()[2..],
                            ) && scrollbar_state
                        }
                        None => scrollbar_state && nesting && has_scrollbar_ancestor(sheet, index),
                    };
                    if known {
                        continue;
                    }
                }
                let at = offset + pseudo.start;
                reports.push(Report::new(at, at + value.len(), "rejected").with(&[value]));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SelectorPseudoClassNoUnknown;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/selector-pseudo-class-no-unknown.txt",
            &|config| {
                Ok(Box::new(SelectorPseudoClassNoUnknown {
                    ignore: matchers(secondary(config), "ignorePseudoClasses")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (60, 26, 4),
            "{:#?}",
            tally.apart
        );
    }
}
