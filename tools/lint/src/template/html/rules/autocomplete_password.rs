//! `autocomplete-password` of `html-validate` 11.17.0 (`src/rules/autocomplete-password.ts`,
//! REF-HTML-VALIDATE-SOURCE): a password field without `autocomplete`, reported at the element;
//! or with `off` (its first token past a section or a hint), or another value than the preferred
//! one when configured, reported at the token. The project prefers none.

use super::super::view::Doc;
use super::super::{MESSAGES, report, tokens};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its option.
pub struct AutocompletePassword {
    /// The value required, any when `None`.
    pub preferred: Option<String>,
}

impl AutocompletePassword {
    /// The project's configuration: no value preferred.
    pub const PROJECT: Self = Self { preferred: None };
}

impl TemplateRule for AutocompletePassword {
    fn name(&self) -> &'static str {
        "html-validate/autocomplete-password"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let password = doc
                .value(index, "type")
                .is_some_and(|kind| kind.to_lowercase() == "password");
            if element.tag != "input" || !password {
                continue;
            }
            let Some(attr) = doc.attr(index, "autocomplete") else {
                reports.push(report(
                    doc.name_span(index),
                    "<input type=\"password\"> is missing required \"autocomplete\" attribute"
                        .to_owned(),
                ));
                continue;
            };
            let Some(value) = attr.text().filter(|value| !value.is_empty()) else {
                continue;
            };
            let lower = value.to_lowercase();
            let list = tokens(&lower, attr.value_span());
            let found = list.iter().find(|(token, _)| {
                !(token.starts_with("section-") || matches!(*token, "shipping" | "billing"))
            });
            let Some((token, span)) = found else {
                continue;
            };
            if *token == "off" {
                reports.push(report(
                    *span,
                    "<input type=\"password\"> should not use autocomplete=\"off\"".to_owned(),
                ));
            } else if let Some(preferred) = self.preferred.as_deref().map(str::to_lowercase)
                && *token != preferred
            {
                reports.push(report(
                    *span,
                    format!("<input type=\"password\"> should use autocomplete=\"{preferred}\""),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AutocompletePassword;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "autocomplete-password.txt",
            "autocomplete-password",
            &|options| {
                Box::new(AutocompletePassword {
                    preferred: options
                        .and_then(|options| options.text("preferred"))
                        .map(ToOwned::to_owned),
                })
            },
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (11, 10, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
