//! `prefer-button` of `html-validate` 11.17.0 (`src/rules/prefer-button.ts`,
//! REF-HTML-VALIDATE-SOURCE): an `<input>` of type `button`, `submit`, `reset` or `image`,
//! reported at its type's value; `include` and `exclude` choose the types checked.

use super::super::view::Doc;
use super::super::{MESSAGES, is_keyword_ignored, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its options.
pub struct PreferButton {
    /// The types checked, all when `None`.
    pub include: Option<Vec<String>>,
    /// The types not checked.
    pub exclude: Option<Vec<String>>,
}

impl PreferButton {
    /// The project's configuration: every type.
    pub const PROJECT: Self = Self {
        include: None,
        exclude: None,
    };
}

impl TemplateRule for PreferButton {
    fn name(&self) -> &'static str {
        "html-validate/prefer-button"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for element in doc.elements.iter().filter(|element| element.tag == "input") {
            for attr in element.attrs.iter().filter(|attr| attr.key == "type") {
                let Some(kind) = attr
                    .text()
                    .filter(|kind| !kind.is_empty())
                    .map(str::to_lowercase)
                else {
                    continue;
                };
                if is_keyword_ignored(self.include.as_deref(), self.exclude.as_deref(), &kind)
                    || !matches!(kind.as_str(), "button" | "submit" | "reset" | "image")
                {
                    continue;
                }
                reports.push(report(
                    attr.value_span(),
                    format!("Prefer to use <button> instead of <input type=\"{kind}\"> when adding buttons"),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferButton;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("prefer-button.txt", "prefer-button", &|options| {
            Box::new(PreferButton {
                include: options.and_then(|options| options.texts("include")),
                exclude: options.and_then(|options| options.texts("exclude")),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (5, 7, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
