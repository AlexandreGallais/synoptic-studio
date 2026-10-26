//! `attribute-allowed-values` of `html-validate` 11.17.0
//! (`src/rules/attribute-allowed-values.ts`, REF-HTML-VALIDATE-SOURCE): a written value its
//! attribute's metadata does not allow (an enumerated value, a boolean's form), reported at the
//! value, or at the name when the value is missing; a bound value is not read.

use super::super::elements::attribute;
use super::super::validator::valid_attribute;
use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct AttributeAllowedValues;

impl TemplateRule for AttributeAllowedValues {
    fn name(&self) -> &'static str {
        "html-validate/attribute-allowed-values"
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
                let Some(rule) = attribute(meta, &attr.key) else {
                    continue;
                };
                if valid_attribute(&attr.key, &attr.value, rule) {
                    continue;
                }
                let (span, message) = match &attr.value {
                    Value::Static(value) => (
                        attr.value_span(),
                        format!("Attribute \"{}\" has invalid value \"{value}\"", attr.key),
                    ),
                    Value::Omitted | Value::Dynamic => (
                        attr.key_span(),
                        format!("Attribute \"{}\" is missing value", attr.key),
                    ),
                };
                reports.push(report(span, message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AttributeAllowedValues;
    use crate::template::html::cases::{messages, run};

    #[test]
    fn reports_a_value_outside_its_attribute_s_values() {
        // Our own cases: a `type` html5 does not list, a boolean written with another value, a
        // value that must not be omitted; a bound value is not read. `<button type="x"></button>`
        // is 26 characters, `<input disabled="` 17 more: the value at column 44.
        assert_eq!(
            messages(
                &AttributeAllowedValues,
                "<button type=\"x\"></button><input disabled=\"no\"><input name><input [type]=\"t\">"
            ),
            [
                "1:15 Attribute \"type\" has invalid value \"x\"",
                "1:44 Attribute \"disabled\" has invalid value \"no\"",
                "1:55 Attribute \"name\" is missing value",
            ]
        );
    }

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "attribute-allowed-values.txt",
            "attribute-allowed-values",
            &|_| Box::new(AttributeAllowedValues),
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (0, 0, 20),
            "{:#?}",
            tally.apart_cases
        );
    }
}
