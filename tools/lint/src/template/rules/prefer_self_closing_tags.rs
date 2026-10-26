//! `@angular-eslint/template/prefer-self-closing-tags`: a component, a `<ng-template>` or a
//! `<ng-content>` with nothing between its tags but whitespace is written self-closed
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/prefer-self-closing-tags.ts`);
//! the DOM's elements, `:svg:` and `:math:` ones among them, and any file named `index.html`
//! are left. Reported over the closing tag and fixed into `/>`.

use super::dom::is_dom_element;
use super::r3::{R3, replace, siblings, slice};
use super::{Report, Template, TemplateRule};
use crate::template::lexer::Span;
use crate::template::tags::split_ns_name;
use crate::template::tree::Element;

/// The rule.
pub struct PreferSelfClosingTags;

/// The closing tag of a `<ng-content>`.
const NG_CONTENT_CLOSE: &str = "</ng-content>";

/// Whether an element is the browser's own (`isNativeElement`).
fn is_native(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.starts_with(":svg:") || lower.starts_with(":math:") || is_dom_element(&lower)
}

/// The space written before `/>`: none when the opening tag already ends with whitespace.
fn prefix(last: Option<char>) -> &'static str {
    if last.is_some_and(char::is_whitespace) {
        ""
    } else {
        " "
    }
}

/// An element or a template with an empty body (`processElementOrTemplateNode`).
fn check_element(text: &str, element: &Element, reports: &mut Vec<Report>) {
    let Some(end) = element.end_span else {
        return;
    };
    let start = element.start_span;
    if start.start == end.start && start.end == end.end {
        return;
    }
    let child_content = siblings(&element.children).iter().any(|node| match node {
        R3::Text(each) => !slice(text, each.span.start, each.span.end)
            .trim()
            .is_empty(),
        _ => true,
    });
    let has_content = !slice(text, start.end, end.start).trim().is_empty();
    if has_content || child_content {
        return;
    }
    let last = slice(text, start.start, start.end).chars().rev().nth(1);
    reports.push(Report {
        fix: Some(replace(
            start.end - 1,
            end.end,
            &format!("{}/>", prefix(last)),
        )),
        ..Report::new(end, "preferSelfClosingTags")
    });
}

/// Where the content of a `<ng-content>` starts, past its opening tag's `>` outside quotes
/// (`findStartOfNgContentInnerHTML`).
fn inner_start(source: &str) -> usize {
    let mut quote = None;
    for (index, character) in source.char_indices().skip(11) {
        match quote {
            Some(open) if open == character => quote = None,
            Some(_) => {}
            None => match character {
                '>' => return index + 1,
                '"' | '\'' => quote = Some(character),
                _ => {}
            },
        }
    }
    source.len()
}

/// A `<ng-content>` with an empty body (`processContentNode`).
fn check_content(text: &str, element: &Element, reports: &mut Vec<Report>) {
    let source = slice(text, element.span.start, element.span.end);
    if !source.ends_with(NG_CONTENT_CLOSE) {
        return;
    }
    let start = inner_start(source);
    let close = source.len() - NG_CONTENT_CLOSE.len();
    if start < close && !source[start..close].trim().is_empty() {
        return;
    }
    let last = source
        .get(..start.saturating_sub(1))
        .and_then(|before| before.chars().last());
    let end = element.span.end;
    let at = element.span.start + crate::template::lexer::offset(start) - 1;
    reports.push(Report {
        fix: Some(replace(at, end, &format!("{}/>", prefix(last)))),
        ..Report::new(
            Span::new((end as usize) - NG_CONTENT_CLOSE.len(), end as usize),
            "preferSelfClosingTags",
        )
    });
}

impl TemplateRule for PreferSelfClosingTags {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-self-closing-tags"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferSelfClosingTags",
            "Use self-closing tags for elements with a closing tag but no content.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let file = template.path.rsplit(['/', '\\']).next().unwrap_or_default();
        if file == "index.html" {
            return;
        }
        for (element, _) in template.elements() {
            if element.name.eq_ignore_ascii_case("ng-content") {
                check_content(template.text, element, reports);
            } else if split_ns_name(&element.name).1 == "ng-template" || !is_native(&element.name) {
                check_element(template.text, element, reports);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSelfClosingTags;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-self-closing-tags.txt", &[], &|_| {
            Box::new(PreferSelfClosingTags)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 30 valid and 10 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (30, 10, 0));
    }
}
