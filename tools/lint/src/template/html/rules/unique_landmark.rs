//! `unique-landmark` of `html-validate` 11.17.0 (`src/rules/unique-landmark.ts`,
//! REF-HTML-VALIDATE-SOURCE): two landmarks of one role, each without a name or with the same, are
//! reported at their name's attribute or at the element; an unnamed `<form>` or `<section>` is no
//! landmark; a bound name is unknown, never reported.

use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report};
use crate::template::lexer::Span;
use crate::template::rules::{Report, Template, TemplateRule};

/// The landmark roles.
const ROLES: [&str; 7] = [
    "complementary",
    "contentinfo",
    "form",
    "banner",
    "main",
    "navigation",
    "region",
];

/// A landmark's name.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Name {
    /// None.
    None,
    /// Bound.
    Dynamic,
    /// Written, or the text of the element it names.
    Text(String),
}

/// The rule.
pub struct UniqueLandmark;

/// A landmark's role: its `role` written, else its implicit one.
fn role_of(doc: &Doc, index: usize) -> Option<String> {
    match doc.attr(index, "role").map(|attr| &attr.value) {
        Some(Value::Static(role)) => Some(role.clone()),
        Some(Value::Dynamic | Value::Omitted) => None,
        None => doc
            .el(index)
            .meta
            .and_then(|meta| meta.role(doc, index))
            .map(ToOwned::to_owned),
    }
}

/// A landmark's name and where it is reported (`getTextEntryFromElement`).
fn name_of(doc: &Doc, index: usize) -> (Name, Span) {
    let name = |value: &Value| match value {
        Value::Static(text) => Name::Text(text.clone()),
        Value::Dynamic => Name::Dynamic,
        Value::Omitted => Name::None,
    };
    if let Some(attr) = doc.attr(index, "aria-label") {
        return (name(&attr.value), attr.key_span());
    }
    if let Some(attr) = doc.attr(index, "aria-labelledby") {
        let text = match &attr.value {
            Value::Static(id) if !id.is_empty() => {
                let found = (0..doc.elements.len())
                    .find(|each| doc.value(*each, "id") == Some(id.as_str()));
                match found.map(|each| doc.text_content(each)) {
                    // An interpolated text is known at run time only.
                    Some((_, true)) => Name::Dynamic,
                    Some((text, false)) => Name::Text(text),
                    None => Name::Text(format!("#{id}")),
                }
            }
            other => name(other),
        };
        return (text, attr.key_span());
    }
    (Name::None, doc.name_span(index))
}

impl TemplateRule for UniqueLandmark {
    fn name(&self) -> &'static str {
        "html-validate/unique-landmark"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let candidate = "aside, footer, form, header, main, nav, section, [role]";
        let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
        for index in (0..doc.elements.len()).filter(|index| doc.matches(*index, candidate)) {
            let Some(role) = role_of(&doc, index).filter(|role| ROLES.contains(&role.as_str()))
            else {
                continue;
            };
            match groups.iter_mut().find(|(each, _)| *each == role) {
                Some((_, members)) => members.push(index),
                None => groups.push((role, vec![index])),
            }
        }
        for (_, members) in groups.iter().filter(|(_, members)| members.len() > 1) {
            let entries: Vec<(usize, Name, Span)> = members
                .iter()
                .map(|index| {
                    let (name, span) = name_of(&doc, *index);
                    (*index, name, span)
                })
                .collect();
            for (index, name, span) in &entries {
                let unnamed_region = *name == Name::None
                    && matches!(doc.el(*index).tag.as_str(), "form" | "section");
                if unnamed_region || *name == Name::Dynamic {
                    continue;
                }
                let duplicate = entries.iter().filter(|(_, other, _)| other == name).count() > 1;
                let named = matches!(name, Name::Text(text) if !text.is_empty());
                if named && !duplicate {
                    continue;
                }
                reports.push(report(
                    *span,
                    "Landmarks must have a non-empty and unique accessible name (aria-label or aria-labelledby)".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueLandmark;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("unique-landmark.txt", "unique-landmark", &|_| {
            Box::new(UniqueLandmark)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (14, 18, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
