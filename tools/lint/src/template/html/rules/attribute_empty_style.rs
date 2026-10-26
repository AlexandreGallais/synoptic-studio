//! `attribute-empty-style` of `html-validate` 11.17.0 (`src/rules/attribute-empty-style.ts`,
//! REF-HTML-VALIDATE-SOURCE): an attribute whose value may be omitted, written empty or omitted
//! in another style than the one asked, `omit` for the project, the rule's default, reported at
//! its name; a bound value is never empty.

use super::super::elements::{ValueForm, attribute};
use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its option.
pub struct AttributeEmptyStyle {
    /// Whether `x=""` is asked rather than `x`.
    pub empty: bool,
}

impl AttributeEmptyStyle {
    /// The project's configuration: `omit`, the rule's default.
    pub const PROJECT: Self = Self { empty: false };
}

impl TemplateRule for AttributeEmptyStyle {
    fn name(&self) -> &'static str {
        "html-validate/attribute-empty-style"
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
                let omissible = attribute(meta, &attr.key)
                    .is_some_and(|rule| rule.form == ValueForm::Omissible);
                let empty = matches!(&attr.value, Value::Omitted) || attr.text() == Some("");
                if !omissible || !empty {
                    continue;
                }
                let key = &attr.key;
                if self.empty && attr.value == Value::Omitted {
                    reports.push(report(
                        attr.key_span(),
                        format!("Attribute \"{key}\" value should be empty string"),
                    ));
                } else if !self.empty && attr.value != Value::Omitted {
                    reports.push(report(
                        attr.key_span(),
                        format!("Attribute \"{key}\" should omit value"),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AttributeEmptyStyle;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "attribute-empty-style.txt",
            "attribute-empty-style",
            &|options| {
                Box::new(AttributeEmptyStyle {
                    empty: options.and_then(|options| options.text("style")) == Some("empty"),
                })
            },
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (12, 2, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
