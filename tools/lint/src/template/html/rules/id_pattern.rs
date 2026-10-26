//! `id-pattern` of `html-validate` 11.17.0 (`src/rules/id-pattern.ts`, REF-HTML-VALIDATE-SOURCE):
//! a written `id` matching the configured patterns, `kebabcase` for the project, the rule's
//! default, else reported at its value, or at the element when empty.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use super::patterns::{compiled, mismatch};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its option.
pub struct IdPattern {
    /// The patterns, `kebabcase` when `None`.
    pub patterns: Option<Vec<String>>,
}

impl IdPattern {
    /// The project's configuration: `kebabcase`.
    pub const PROJECT: Self = Self { patterns: None };
}

impl TemplateRule for IdPattern {
    fn name(&self) -> &'static str {
        "html-validate/id-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let patterns = compiled(self.patterns.as_deref(), "kebabcase");
        for element in &doc.elements {
            for attr in element.attrs.iter().filter(|attr| attr.key == "id") {
                let Some(value) = attr.text() else {
                    continue;
                };
                if let Some(message) = mismatch("id", value, &patterns) {
                    // An empty value has no place: the report falls back to the attribute.
                    let span = if value.is_empty() {
                        attr.span()
                    } else {
                        attr.value_span()
                    };
                    reports.push(report(span, message));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IdPattern;
    use crate::template::html::cases::run;
    use crate::template::html::rules::patterns::of_option;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("id-pattern.txt", "id-pattern", &|options| {
            Box::new(IdPattern {
                patterns: of_option(options.and_then(|options| options.get("pattern"))),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (5, 3, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
