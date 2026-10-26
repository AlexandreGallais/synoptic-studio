//! `no-raw-characters` of `html-validate` 11.17.0 (`src/rules/no-raw-characters.ts`,
//! REF-HTML-VALIDATE-SOURCE): a raw `<`, or an `&` starting no character reference, in an
//! element's text, and a raw `"`, `'`, `<`, `=`, `>`, `` ` `` or `&` in an unquoted value,
//! reported and fixed by its entity; in relaxed mode the `&` is left alone. An interpolation
//! (`{{ a && b }}`) is an expression, not text: Angular's lexer reads it apart, and it is not
//! checked. A text at a template's root belongs to no element, as html-validate's root, and is
//! not checked either.

use regex::Regex;

use super::super::elements::{ValueForm, attribute};
use super::super::view::{Child, Doc};
use super::super::{MESSAGES, fixed};
use crate::finding::Fix;
use crate::template::lexer::{Span, TokenType, offset};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its option.
pub struct NoRawCharacters {
    /// Whether an `&` is left alone.
    pub relaxed: bool,
}

impl NoRawCharacters {
    /// The project's configuration: strict, the rule's default.
    pub const PROJECT: Self = Self { relaxed: false };

    /// Reports each raw character of a text a pattern finds, from its first byte.
    fn find(&self, text: &str, start: usize, pattern: &Regex, reports: &mut Vec<Report>) {
        for found in pattern.find_iter(text) {
            let character = found.as_str();
            // A character reference matched whole is no raw character.
            if character.len() > 1 || (self.relaxed && character == "&") {
                continue;
            }
            let replacement = match character {
                "\"" => "&quot;",
                "&" => "&amp;",
                "'" => "&apos;",
                "<" => "&lt;",
                "=" => "&equals;",
                ">" => "&gt;",
                _ => "&grave;",
            };
            let at = start + found.start();
            reports.push(fixed(
                Span::new(at, at + 1),
                format!("Raw \"{character}\" must be encoded as \"{replacement}\""),
                Fix {
                    start: offset(at),
                    end: offset(at + 1),
                    text: replacement.to_owned(),
                },
            ));
        }
    }
}

/// The text of a pattern, `&` followed by no reference: the `regex` crate has no look-ahead, so
/// the reference is matched and left out by the caller.
fn pattern(characters: &str) -> Option<Regex> {
    Regex::new(&format!(r"(?i)&[\d#A-Z]+;|[{characters}]|&")).ok()
}

impl TemplateRule for NoRawCharacters {
    fn name(&self) -> &'static str {
        "html-validate/no-raw-characters"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let (Some(text_pattern), Some(value_pattern)) = (pattern("<"), pattern("\"'<=>`")) else {
            return;
        };
        let templating = Regex::new(r"(?s)^(?:<%.*?%>|<\?.*?\?>|<\$.*?\$>)$").ok();
        let mut found = Vec::new();
        for element in &doc.elements {
            for child in &element.children {
                let Child::Text(text) = child else {
                    continue;
                };
                let raw = template
                    .text
                    .get(text.span.full_start as usize..text.span.end as usize)
                    .unwrap_or_default();
                if templating.as_ref().is_some_and(|regex| regex.is_match(raw)) {
                    continue;
                }
                let read_as_text = |token: &&crate::template::lexer::Token| {
                    matches!(token.kind, TokenType::Text | TokenType::EscapableRawText)
                };
                for token in text.tokens.iter().filter(read_as_text) {
                    let start = token.span.full_start as usize;
                    let source = template
                        .text
                        .get(start..token.span.end as usize)
                        .unwrap_or_default();
                    self.find(source, start, &text_pattern, &mut found);
                }
            }
            for attr in &element.attrs {
                let Some(value) = attr.text().filter(|value| !value.is_empty()) else {
                    continue;
                };
                let boolean = element
                    .meta
                    .and_then(|meta| attribute(meta, &attr.key))
                    .is_some_and(|rule| rule.form == ValueForm::Boolean);
                if attr.quote(template.text).is_some() || boolean {
                    continue;
                }
                self.find(
                    value,
                    attr.value_span().start as usize,
                    &value_pattern,
                    &mut found,
                );
            }
        }
        reports.extend(found);
    }
}

#[cfg(test)]
mod tests {
    use super::NoRawCharacters;
    use crate::template::html::cases::{fixed, run_except};

    /// The cases html-validate and Angular read apart.
    const KNOWN: &[(&str, &str)] = &[
        (
            "should report error when = are present",
            "Angular's lexer ends an unquoted value at `=`",
        ),
        (
            "<% & %>",
            "html-validate's lexer reads `<% %>` as server templating",
        ),
        (
            "<% & %> (with newlines)",
            "html-validate's lexer reads `<% %>` as server templating",
        ),
        (
            "<$ & $>",
            "html-validate's lexer reads `<$ $>` as server templating",
        ),
        (
            "<$ & $> (with newlines)",
            "html-validate's lexer reads `<$ $>` as server templating",
        ),
    ];

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run_except(
            "no-raw-characters.txt",
            "no-raw-characters",
            KNOWN,
            &|options| {
                Box::new(NoRawCharacters {
                    relaxed: options
                        .and_then(|options| options.flag("relaxed"))
                        .unwrap_or(false),
                })
            },
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (16, 3, 13),
            "{:#?}",
            tally.apart_cases
        );
    }

    #[test]
    fn encodes_a_raw_character_but_not_an_interpolation() {
        // Our own cases: CRLF line ends keep the offsets; an entity is left whole and an
        // interpolation is an expression.
        assert_eq!(
            fixed(
                &NoRawCharacters::PROJECT,
                "<p>\r\n a & b &amp; {{ a && b }}\r\n</p>"
            ),
            "<p>\r\n a &amp; b &amp; {{ a && b }}\r\n</p>"
        );
    }
}
