//! `tel-non-breaking` of `html-validate` 11.17.0 (`src/rules/tel-non-breaking.ts`,
//! REF-HTML-VALIDATE-SOURCE): a space or a hyphen in the text of a `tel:` link, reported and fixed
//! by its non-breaking entity; a link of an ignored class, or styled `white-space: nowrap` or
//! `pre`, left alone. The text of an interpolation is an expression, never checked.

use super::super::a11y::css_declarations;
use super::super::view::{Child, Doc};
use super::super::{MESSAGES, fixed, tokens};
use crate::finding::Fix;
use crate::javascript::JsText;
use crate::template::lexer::{Span, TokenType, offset};
use crate::template::rules::{Report, Template, TemplateRule};
use crate::template::tree::Text;

/// The characters reported, their entity and its description.
const CHARACTERS: [(char, &str, &str); 2] = [
    (' ', "&nbsp;", "non-breaking space"),
    ('-', "&#8209;", "non-breaking hyphen"),
];

/// The rule and its options.
pub struct TelNonBreaking {
    /// The classes of a link left alone.
    pub ignore_classes: Vec<String>,
    /// Whether a link styled `nowrap` or `pre` is left alone.
    pub ignore_style: bool,
}

impl TelNonBreaking {
    /// The project's configuration: the rule's defaults.
    pub const PROJECT: Self = Self {
        ignore_classes: Vec::new(),
        ignore_style: true,
    };

    /// Whether a link is left alone.
    fn is_ignored(&self, doc: &Doc, index: usize) -> bool {
        let classes = doc
            .value(index, "class")
            .map(|value| {
                tokens(value, Span::new(0, 0))
                    .into_iter()
                    .map(|(each, _)| each.to_owned())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let by_class = self
            .ignore_classes
            .iter()
            .any(|each| classes.contains(each));
        let by_style = self.ignore_style
            && doc.value(index, "style").is_some_and(|style| {
                css_declarations(style).iter().any(|(property, value)| {
                    property == "white-space" && matches!(value.as_str(), "nowrap" | "pre")
                })
            });
        by_class || by_style
    }
}

/// Reports each character of a text, its leading and trailing whitespace left out (`getText`,
/// whose `/^(\s*)(\S.*)$/` reads nothing of a text with a line end after its first character).
fn check_text(source: &str, text: &Text, reports: &mut Vec<Report>) {
    let start = text.span.full_start as usize;
    let raw = source
        .get(start..text.span.end as usize)
        .unwrap_or_default();
    let leading = raw.len() - raw.js_trim_start().len();
    let rest = &raw[leading..];
    if rest.contains(['\n', '\r', '\u{2028}', '\u{2029}']) {
        return;
    }
    let trimmed = rest.js_trim_end();
    let expressions: Vec<(usize, usize)> = text
        .tokens
        .iter()
        .filter(|token| token.kind == TokenType::Interpolation)
        .map(|token| (token.span.full_start as usize, token.span.end as usize))
        .collect();
    for (at, character) in trimmed.char_indices() {
        let Some((_, replacement, description)) =
            CHARACTERS.iter().find(|(each, _, _)| *each == character)
        else {
            continue;
        };
        let begin = start + leading + at;
        if expressions
            .iter()
            .any(|(from, to)| (*from..*to).contains(&begin))
        {
            continue;
        }
        reports.push(fixed(
            Span::new(begin, begin + 1),
            format!("\"{character}\" should be replaced with \"{replacement}\" ({description}) in telephone number"),
            Fix {
                start: offset(begin),
                end: offset(begin + 1),
                text: (*replacement).to_owned(),
            },
        ));
    }
}

/// Reports the characters of every text under an element.
fn walk(source: &str, doc: &Doc, index: usize, reports: &mut Vec<Report>) {
    for child in &doc.el(index).children {
        match child {
            Child::Text(text) => check_text(source, text, reports),
            Child::Element(each) => walk(source, doc, *each, reports),
        }
    }
}

impl TemplateRule for TelNonBreaking {
    fn name(&self) -> &'static str {
        "html-validate/tel-non-breaking"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let tel = doc
                .value(index, "href")
                .is_some_and(|href| href.starts_with("tel:"));
            if element.tag != "a" || !tel || self.is_ignored(&doc, index) {
                continue;
            }
            walk(template.text, &doc, index, reports);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TelNonBreaking;
    use crate::template::html::cases::{fixed, run};

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("tel-non-breaking.txt", "tel-non-breaking", &|options| {
            Box::new(TelNonBreaking {
                ignore_classes: options
                    .and_then(|options| options.texts("ignoreClasses"))
                    .unwrap_or_default(),
                ignore_style: options
                    .and_then(|options| options.flag("ignoreStyle"))
                    .unwrap_or(true),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (5, 6, 4),
            "{:#?}",
            tally.apart_cases
        );
    }

    #[test]
    fn fixes_a_telephone_number() {
        // Our own cases: CRLF line ends and an entity before the number keep the offsets; a text
        // with a line end after its first character is not read, as `getText` reads none.
        assert_eq!(
            fixed(
                &TelNonBreaking::PROJECT,
                "<p>\r\n</p>\r\n<a href=\"tel:1\">&amp;1 2-3 </a>"
            ),
            "<p>\r\n</p>\r\n<a href=\"tel:1\">&amp;1&nbsp;2&#8209;3 </a>"
        );
        assert_eq!(
            fixed(&TelNonBreaking::PROJECT, "<a href=\"tel:1\">1 2\r\n</a>"),
            "<a href=\"tel:1\">1 2\r\n</a>"
        );
    }
}
