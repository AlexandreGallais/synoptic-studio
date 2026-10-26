//! `name-pattern` of `html-validate` 11.17.0 (`src/rules/name-pattern.ts`,
//! REF-HTML-VALIDATE-SOURCE): a written `name` of a listed form control, its `[]` aside, matching
//! the configured patterns, `camelcase` for the project, the rule's default, else reported at its
//! value, or at the element when empty.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use super::patterns::{compiled, mismatch};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its option.
pub struct NamePattern {
    /// The patterns, `camelcase` when `None`.
    pub patterns: Option<Vec<String>>,
}

impl NamePattern {
    /// The project's configuration: `camelcase`.
    pub const PROJECT: Self = Self { patterns: None };
}

impl TemplateRule for NamePattern {
    fn name(&self) -> &'static str {
        "html-validate/name-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let patterns = compiled(self.patterns.as_deref(), "camelcase");
        for element in &doc.elements {
            let listed = element
                .meta
                .and_then(|meta| meta.form_associated)
                .is_some_and(|associated| associated.listed);
            if !listed {
                continue;
            }
            for attr in element.attrs.iter().filter(|attr| attr.key == "name") {
                let Some(value) = attr.text() else {
                    continue;
                };
                let name = value.strip_suffix("[]").unwrap_or(value);
                if let Some(message) = mismatch("name", name, &patterns) {
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
    use super::NamePattern;
    use crate::template::html::cases::run;
    use crate::template::html::rules::patterns::of_option;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("name-pattern.txt", "name-pattern", &|options| {
            Box::new(NamePattern {
                patterns: of_option(options.and_then(|options| options.get("pattern"))),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 4, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
