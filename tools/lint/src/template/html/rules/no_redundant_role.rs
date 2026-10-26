//! `no-redundant-role` of `html-validate` 11.17.0 (`src/rules/no-redundant-role.ts`,
//! REF-HTML-VALIDATE-SOURCE): a written `role` the element already has implicitly, reported at
//! its value; `include` and `exclude` choose the roles checked.

use super::super::view::Doc;
use super::super::{MESSAGES, is_keyword_ignored, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its options.
pub struct NoRedundantRole {
    /// The roles checked, all when `None`.
    pub include: Option<Vec<String>>,
    /// The roles not checked.
    pub exclude: Option<Vec<String>>,
}

impl NoRedundantRole {
    /// The project's configuration: every role.
    pub const PROJECT: Self = Self {
        include: None,
        exclude: None,
    };
}

impl TemplateRule for NoRedundantRole {
    fn name(&self) -> &'static str {
        "html-validate/no-redundant-role"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(attr) = doc.attr(index, "role") else {
                continue;
            };
            let Some(role) = attr.text().filter(|role| !role.is_empty()) else {
                continue;
            };
            let implicit = element.meta.and_then(|meta| meta.role(&doc, index));
            if implicit != Some(role)
                || is_keyword_ignored(self.include.as_deref(), self.exclude.as_deref(), role)
            {
                continue;
            }
            reports.push(report(
                attr.value_span(),
                format!("Redundant role \"{role}\" on <{}>", element.tag),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoRedundantRole;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-redundant-role.txt", "no-redundant-role", &|options| {
            Box::new(NoRedundantRole {
                include: options.and_then(|options| options.texts("include")),
                exclude: options.and_then(|options| options.texts("exclude")),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (8, 4, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
