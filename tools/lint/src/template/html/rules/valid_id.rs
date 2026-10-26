//! `valid-id` of `html-validate` 11.17.0 (`src/rules/valid-id.ts`, REF-HTML-VALIDATE-SOURCE): an
//! `id` is not empty, holds no whitespace and, unless relaxed, starts with a letter and holds only
//! letters, digits, `-` and `_`. The project runs it strict, the rule's default.

use regex::Regex;

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule, relaxed or not.
pub struct ValidId {
    /// Whether only emptiness and whitespace are checked.
    pub relaxed: bool,
}

impl ValidId {
    /// The project's configuration: strict.
    pub const PROJECT: Self = Self { relaxed: false };
}

impl TemplateRule for ValidId {
    fn name(&self) -> &'static str {
        "html-validate/valid-id"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let leading = Regex::new(r"^\P{L}").ok();
        let disallowed = Regex::new(r"[^\p{L}\p{N}_-]").ok();
        let matches = |regex: &Option<Regex>, value: &str| {
            regex.as_ref().is_some_and(|regex| regex.is_match(value))
        };
        for element in &doc.elements {
            for attr in element.attrs.iter().filter(|attr| attr.key == "id") {
                let Some(value) = attr.text() else {
                    continue;
                };
                let (span, problem) = if value.is_empty() {
                    (attr.span(), "must not be empty")
                } else if value.chars().any(char::is_whitespace) {
                    (attr.value_span(), "must not contain whitespace")
                } else if self.relaxed {
                    continue;
                } else if matches(&leading, value) {
                    (attr.value_span(), "must begin with a letter")
                } else if matches(&disallowed, value) {
                    (
                        attr.value_span(),
                        "must only contain letters, digits, dash and underscore characters",
                    )
                } else {
                    continue;
                };
                reports.push(report(span, format!("element id \"{value}\" {problem}")));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ValidId;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("valid-id.txt", "valid-id", &|options| {
            Box::new(ValidId {
                relaxed: options
                    .and_then(|options| options.flag("relaxed"))
                    .unwrap_or(false),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 10, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
