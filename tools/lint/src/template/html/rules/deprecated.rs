//! `deprecated` of `html-validate` 11.17.0 (`src/rules/deprecated.ts`, REF-HTML-VALIDATE-SOURCE):
//! an element its metadata marks deprecated is reported at its name, with the deprecation's
//! message when it gives one; `include` and `exclude` choose the elements checked.

use super::super::view::Doc;
use super::super::{MESSAGES, is_keyword_ignored, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its options.
pub struct Deprecated {
    /// The elements checked, all when `None`.
    pub include: Option<Vec<String>>,
    /// The elements not checked.
    pub exclude: Option<Vec<String>>,
}

impl Deprecated {
    /// The project's configuration: every element.
    pub const PROJECT: Self = Self {
        include: None,
        exclude: None,
    };
}

impl TemplateRule for Deprecated {
    fn name(&self) -> &'static str {
        "html-validate/deprecated"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(deprecated) = element.meta.and_then(|meta| meta.deprecated) else {
                continue;
            };
            if is_keyword_ignored(
                self.include.as_deref(),
                self.exclude.as_deref(),
                &element.tag,
            ) {
                continue;
            }
            let notice = deprecated
                .message
                .map_or_else(String::new, |message| format!(": {message}"));
            reports.push(report(
                doc.name_span(index),
                format!("<{}> is deprecated{notice}", element.tag),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Deprecated;
    use crate::template::html::cases::{messages, run};

    #[test]
    fn reports_a_deprecated_element_with_its_message() {
        // Our own cases: html5's `<center>` has a message, `<keygen>` none; `exclude` leaves
        // `<center>` alone.
        assert_eq!(
            messages(&Deprecated::PROJECT, "<center></center><keygen>"),
            [
                "1:2 <center> is deprecated: use CSS instead",
                "1:19 <keygen> is deprecated"
            ]
        );
        let excluding = Deprecated {
            include: None,
            exclude: Some(vec!["center".to_owned()]),
        };
        assert_eq!(
            messages(&excluding, "<center></center>"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("deprecated.txt", "deprecated", &|options| {
            Box::new(Deprecated {
                include: options.and_then(|options| options.texts("include")),
                exclude: options.and_then(|options| options.texts("exclude")),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (0, 0, 16),
            "{:#?}",
            tally.apart_cases
        );
    }
}
