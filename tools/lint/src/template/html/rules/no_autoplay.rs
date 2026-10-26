//! `no-autoplay` of `html-validate` 11.17.0 (`src/rules/no-autoplay.ts`,
//! REF-HTML-VALIDATE-SOURCE): an `autoplay` written, not bound, reported at the attribute;
//! `include` and `exclude` choose the elements checked.

use super::super::view::Doc;
use super::super::{MESSAGES, is_keyword_ignored, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its options.
pub struct NoAutoplay {
    /// The elements checked, all when `None`.
    pub include: Option<Vec<String>>,
    /// The elements not checked.
    pub exclude: Option<Vec<String>>,
}

impl NoAutoplay {
    /// The project's configuration: every element.
    pub const PROJECT: Self = Self {
        include: None,
        exclude: None,
    };
}

impl TemplateRule for NoAutoplay {
    fn name(&self) -> &'static str {
        "html-validate/no-autoplay"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for element in &doc.elements {
            if is_keyword_ignored(
                self.include.as_deref(),
                self.exclude.as_deref(),
                &element.tag,
            ) {
                continue;
            }
            for attr in element
                .attrs
                .iter()
                .filter(|attr| attr.key == "autoplay" && !attr.is_dynamic())
            {
                reports.push(report(
                    attr.span(),
                    format!("The autoplay attribute is not allowed on <{}>", element.tag),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAutoplay;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-autoplay.txt", "no-autoplay", &|options| {
            Box::new(NoAutoplay {
                include: options.and_then(|options| options.texts("include")),
                exclude: options.and_then(|options| options.texts("exclude")),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 4, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
