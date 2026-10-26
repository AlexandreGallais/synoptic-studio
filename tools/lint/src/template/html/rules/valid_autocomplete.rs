//! `valid-autocomplete` of `html-validate` 11.17.0 (`src/rules/valid-autocomplete.ts`,
//! REF-HTML-VALIDATE-SOURCE): a written `autocomplete` checked as the HTML standard reads it: on a
//! `<form>`, `on` or `off`; on a control, `on` or `off` alone (never on a hidden field), else its
//! tokens in order (a section, `shipping` or `billing`, a contact kind, one field name, then
//! `webauthn`), the field allowed by the input's type.

use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report, tokens};
use crate::template::lexer::Span;
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct ValidAutocomplete;

/// The kinds of token, in the order they must appear (`expectedOrder`).
const ORDER: [&str; 6] = ["section", "hint", "contact", "field1", "field2", "webauthn"];

/// The field names without a contact token (`fieldNames1`), each with its control group.
const FIELDS_1: &[(&str, &str)] = &[
    ("name", "text"),
    ("honorific-prefix", "text"),
    ("given-name", "text"),
    ("additional-name", "text"),
    ("family-name", "text"),
    ("honorific-suffix", "text"),
    ("nickname", "text"),
    ("username", "username"),
    ("new-password", "password"),
    ("current-password", "password"),
    ("one-time-code", "password"),
    ("organization-title", "text"),
    ("organization", "text"),
    ("street-address", "multiline"),
    ("address-line1", "text"),
    ("address-line2", "text"),
    ("address-line3", "text"),
    ("address-level4", "text"),
    ("address-level3", "text"),
    ("address-level2", "text"),
    ("address-level1", "text"),
    ("country", "text"),
    ("country-name", "text"),
    ("postal-code", "text"),
    ("cc-name", "text"),
    ("cc-given-name", "text"),
    ("cc-additional-name", "text"),
    ("cc-family-name", "text"),
    ("cc-number", "text"),
    ("cc-exp", "month"),
    ("cc-exp-month", "numeric"),
    ("cc-exp-year", "numeric"),
    ("cc-csc", "text"),
    ("cc-type", "text"),
    ("transaction-currency", "text"),
    ("transaction-amount", "numeric"),
    ("language", "text"),
    ("bday", "date"),
    ("bday-day", "numeric"),
    ("bday-month", "numeric"),
    ("bday-year", "numeric"),
    ("sex", "text"),
    ("url", "url"),
    ("photo", "url"),
];

/// The field names that take a contact token (`fieldNames2`), each with its control group.
const FIELDS_2: &[(&str, &str)] = &[
    ("tel", "tel"),
    ("tel-country-code", "text"),
    ("tel-national", "text"),
    ("tel-area-code", "text"),
    ("tel-local", "text"),
    ("tel-local-prefix", "text"),
    ("tel-local-suffix", "text"),
    ("tel-extension", "text"),
    ("email", "username"),
    ("impp", "url"),
];

/// The input types that take no `autocomplete`.
const DISALLOWED: [&str; 7] = [
    "checkbox", "radio", "file", "submit", "image", "reset", "button",
];

/// A token's kind (`matchToken`).
fn kind(token: &str) -> Option<&'static str> {
    if token.starts_with("section-") {
        Some("section")
    } else if matches!(token, "shipping" | "billing") {
        Some("hint")
    } else if FIELDS_1.iter().any(|(field, _)| *field == token) {
        Some("field1")
    } else if FIELDS_2.iter().any(|(field, _)| *field == token) {
        Some("field2")
    } else if matches!(token, "home" | "work" | "mobile" | "fax" | "pager") {
        Some("contact")
    } else if token == "webauthn" {
        Some("webauthn")
    } else {
        None
    }
}

/// The control groups an input type allows (`getControlGroups`).
fn control_groups(kind: &str) -> &'static [&'static str] {
    const ALL: &[&str] = &[
        "text",
        "multiline",
        "password",
        "url",
        "username",
        "tel",
        "numeric",
        "month",
        "date",
    ];
    const SINGLE_LINE: &[&str] = &[
        "text", "password", "url", "username", "tel", "numeric", "month", "date",
    ];
    match kind {
        "hidden" => ALL,
        "text" | "search" => SINGLE_LINE,
        "password" => &["password"],
        "url" => &["url"],
        "email" => &["username"],
        "tel" => &["tel"],
        "number" => &["numeric"],
        "month" => &["month"],
        "date" => &["date"],
        _ => &[],
    }
}

/// Checks a control's tokens (`validateTokens`).
fn check_tokens(
    doc: &Doc,
    index: usize,
    tokens: &[(&str, Span)],
    key: Span,
    reports: &mut Vec<Report>,
) {
    let mut order = Vec::new();
    for (token, span) in tokens {
        let Some(found) = kind(token) else {
            reports.push(report(
                *span,
                format!("\"{token}\" is not a valid autocomplete token or field name"),
            ));
            return;
        };
        order.push(found);
    }
    let fields: Vec<usize> = (0..order.len())
        .filter(|at| order[*at].starts_with("field"))
        .collect();
    let combination =
        |first: &str, second: &str| format!("\"{second}\" cannot be combined with \"{first}\"");
    match fields.as_slice() {
        [] => reports.push(report(
            key,
            "autocomplete attribute is missing field name".to_owned(),
        )),
        [_] => {}
        [first, .., last] => reports.push(report(
            tokens[*last].1,
            combination(tokens[*first].0, tokens[*last].0),
        )),
    }
    if let (Some(field), Some(contact)) = (
        order.iter().position(|each| *each == "field1"),
        order.iter().position(|each| *each == "contact"),
    ) {
        reports.push(report(
            tokens[contact].1,
            combination(tokens[field].0, tokens[contact].0),
        ));
    }
    let ranks: Vec<usize> = order
        .iter()
        .map(|each| {
            ORDER
                .iter()
                .position(|kind| kind == each)
                .unwrap_or_default()
        })
        .collect();
    for at in 0..ranks.len().saturating_sub(1) {
        if ranks[0] > ranks[at + 1] {
            reports.push(report(
                tokens[at + 1].1,
                format!(
                    "\"{}\" must appear before \"{}\"",
                    tokens[at + 1].0,
                    tokens[at].0
                ),
            ));
        }
    }
    let Some(field) = fields.first() else {
        return;
    };
    if doc.el(index).tag != "input" {
        return;
    }
    let kind = match doc.attr(index, "type").map(|attr| &attr.value) {
        Some(Value::Dynamic) => return,
        Some(Value::Static(kind)) => kind.as_str(),
        Some(Value::Omitted) | None => "text",
    };
    let token = tokens[*field].0;
    let group = FIELDS_1
        .iter()
        .chain(FIELDS_2)
        .find(|(each, _)| *each == token)
        .map(|(_, group)| *group);
    if group.is_some_and(|group| control_groups(kind).contains(&group)) {
        return;
    }
    reports.push(report(
        tokens[*field].1,
        format!("\"{token}\" cannot be used on <input type=\"{kind}\">"),
    ));
}

impl TemplateRule for ValidAutocomplete {
    fn name(&self) -> &'static str {
        "html-validate/valid-autocomplete"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(attr) = doc.attr(index, "autocomplete") else {
                continue;
            };
            let Some(value) = attr.text() else {
                continue;
            };
            let lower = value.to_lowercase();
            let list = tokens(&lower, attr.value_span());
            if list.is_empty() {
                continue;
            }
            match element.tag.as_str() {
                "form" => {
                    let trimmed = lower.trim();
                    if !matches!(trimmed, "on" | "off") {
                        reports.push(report(
                            attr.value_span(),
                            format!("\"{trimmed}\" cannot be used on <form>"),
                        ));
                    }
                }
                "input" | "textarea" | "select" => {
                    let kind = match doc.attr(index, "type").map(|attr| &attr.value) {
                        Some(Value::Static(kind)) => kind.clone(),
                        Some(Value::Dynamic) => String::new(),
                        Some(Value::Omitted) | None => "text".to_owned(),
                    };
                    if element.tag == "input" && DISALLOWED.contains(&kind.as_str()) {
                        reports.push(report(
                            attr.key_span(),
                            format!(
                                "autocomplete attribute cannot be used on <input type=\"{kind}\">"
                            ),
                        ));
                        continue;
                    }
                    let Some(at) = list
                        .iter()
                        .position(|(token, _)| matches!(*token, "on" | "off"))
                    else {
                        check_tokens(&doc, index, &list, attr.key_span(), reports);
                        continue;
                    };
                    let (value, span) = list[at];
                    if list.len() > 1 {
                        let first = list[usize::from(at == 0)].0;
                        reports.push(report(
                            span,
                            format!("\"{value}\" cannot be combined with \"{first}\""),
                        ));
                    }
                    if kind == "hidden" {
                        reports.push(report(
                            list[0].1,
                            format!("\"{value}\" cannot be used on <input type=\"hidden\">"),
                        ));
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ValidAutocomplete;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("valid-autocomplete.txt", "valid-autocomplete", &|_| {
            Box::new(ValidAutocomplete)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (18, 29, 10),
            "{:#?}",
            tally.apart_cases
        );
    }
}
