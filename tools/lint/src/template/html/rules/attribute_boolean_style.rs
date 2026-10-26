//! `attribute-boolean-style` of `html-validate` 11.17.0 (`src/rules/attribute-boolean-style.ts`,
//! REF-HTML-VALIDATE-SOURCE): a boolean attribute written in another style than the one asked,
//! `omit` (`disabled`) for the project, the rule's default, reported at its name; a property
//! binding, an alias of the attribute, is not.

use super::super::elements::{ValueForm, attribute};
use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// How a boolean attribute is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    /// `disabled`.
    Omit,
    /// `disabled=""`.
    Empty,
    /// `disabled="disabled"`.
    Name,
}

/// The rule and its option.
pub struct AttributeBooleanStyle {
    /// The style asked.
    pub style: Style,
}

impl AttributeBooleanStyle {
    /// The project's configuration: `omit`, the rule's default.
    pub const PROJECT: Self = Self { style: Style::Omit };
}

impl TemplateRule for AttributeBooleanStyle {
    fn name(&self) -> &'static str {
        "html-validate/attribute-boolean-style"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for element in &doc.elements {
            let Some(meta) = element.meta else {
                continue;
            };
            for attr in &element.attrs {
                let boolean =
                    attribute(meta, &attr.key).is_some_and(|rule| rule.form == ValueForm::Boolean);
                if !boolean || attr.bound {
                    continue;
                }
                let key = &attr.key;
                let (invalid, message) = match self.style {
                    Style::Omit => (
                        attr.value != Value::Omitted,
                        format!("Attribute \"{key}\" should omit value"),
                    ),
                    Style::Empty => (
                        attr.text() != Some(""),
                        format!("Attribute \"{key}\" value should be empty string"),
                    ),
                    Style::Name => (
                        attr.text() != Some(key.as_str()),
                        format!("Attribute \"{key}\" should be set to {key}=\"{key}\""),
                    ),
                };
                if invalid {
                    reports.push(report(attr.key_span(), message));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AttributeBooleanStyle, Style};
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "attribute-boolean-style.txt",
            "attribute-boolean-style",
            &|options| {
                let style = match options.and_then(|options| options.text("style")) {
                    Some("empty") => Style::Empty,
                    Some("name") => Style::Name,
                    _ => Style::Omit,
                };
                Box::new(AttributeBooleanStyle { style })
            },
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (8, 9, 5),
            "{:#?}",
            tally.apart_cases
        );
    }
}
