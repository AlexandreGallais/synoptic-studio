//! `class-pattern` of `html-validate` 11.17.0 (`src/rules/class-pattern.ts`,
//! REF-HTML-VALIDATE-SOURCE): each written class matching the configured patterns, `kebabcase`
//! for the project, the rule's default, else reported at the class.

use super::super::view::Doc;
use super::super::{MESSAGES, report, tokens};
use super::patterns::{compiled, mismatch};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its option.
pub struct ClassPattern {
    /// The patterns, `kebabcase` when `None`.
    pub patterns: Option<Vec<String>>,
}

impl ClassPattern {
    /// The project's configuration: `kebabcase`.
    pub const PROJECT: Self = Self { patterns: None };
}

impl TemplateRule for ClassPattern {
    fn name(&self) -> &'static str {
        "html-validate/class-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let patterns = compiled(self.patterns.as_deref(), "kebabcase");
        for element in &doc.elements {
            for attr in element.attrs.iter().filter(|attr| attr.key == "class") {
                let Some(value) = attr.text() else {
                    continue;
                };
                for (class, span) in tokens(value, attr.value_span()) {
                    if let Some(message) = mismatch("class", class, &patterns) {
                        reports.push(report(span, message));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClassPattern;
    use crate::template::html::cases::run;
    use crate::template::html::rules::patterns::of_option;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("class-pattern.txt", "class-pattern", &|options| {
            Box::new(ClassPattern {
                patterns: of_option(options.and_then(|options| options.get("pattern"))),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 2, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
