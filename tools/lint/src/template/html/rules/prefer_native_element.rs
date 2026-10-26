//! `prefer-native-element` of `html-validate` 11.17.0 (`src/rules/prefer-native-element.ts`,
//! REF-HTML-VALIDATE-SOURCE): a written `role` a native element has (`role="button"` on a
//! `<div>`), reported at the attribute with the elements to use; the element itself, or one
//! with that role implicitly, is not; `include` and `exclude` choose the roles checked.

use super::super::elements::natural_join;
use super::super::view::Doc;
use super::super::{MESSAGES, is_keyword_ignored, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// Each role and the native elements that have it (`mapping`).
const MAPPING: &[(&str, &[&str])] = &[
    ("article", &["article"]),
    ("banner", &["header"]),
    ("button", &["button"]),
    ("cell", &["td"]),
    ("checkbox", &["input"]),
    ("complementary", &["aside"]),
    ("contentinfo", &["footer"]),
    ("figure", &["figure"]),
    ("form", &["form"]),
    ("heading", &["h1", "h2", "h3", "h4", "h5", "h6"]),
    ("input", &["input"]),
    ("link", &["a"]),
    ("list", &["ul", "ol", "menu"]),
    ("listbox", &["select"]),
    ("listitem", &["li"]),
    ("main", &["main"]),
    ("navigation", &["nav"]),
    ("progressbar", &["progress"]),
    ("radio", &["input"]),
    ("region", &["section"]),
    ("table", &["table"]),
    ("textbox", &["textarea"]),
];

/// The rule and its options.
pub struct PreferNativeElement {
    /// Each role and its native elements, the default mapping when `None`.
    pub mapping: Option<Vec<(String, Vec<String>)>>,
    /// The roles checked, all when `None`.
    pub include: Option<Vec<String>>,
    /// The roles not checked.
    pub exclude: Option<Vec<String>>,
}

impl PreferNativeElement {
    /// The project's configuration: every role of the mapping.
    pub const PROJECT: Self = Self {
        mapping: None,
        include: None,
        exclude: None,
    };
}

impl TemplateRule for PreferNativeElement {
    fn name(&self) -> &'static str {
        "html-validate/prefer-native-element"
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
            let Some(role) = attr
                .text()
                .filter(|role| !role.is_empty())
                .map(str::to_lowercase)
            else {
                continue;
            };
            let natives: Vec<String> = match &self.mapping {
                Some(mapping) => mapping
                    .iter()
                    .find(|(each, _)| *each == role)
                    .map(|(_, natives)| natives.clone())
                    .unwrap_or_default(),
                None => MAPPING
                    .iter()
                    .find(|(each, _)| *each == role)
                    .map(|(_, natives)| natives.iter().map(|native| (*native).to_owned()).collect())
                    .unwrap_or_default(),
            };
            if natives.is_empty() {
                continue;
            }
            if is_keyword_ignored(self.include.as_deref(), self.exclude.as_deref(), &role)
                || natives.iter().any(|native| doc.is(index, native))
                || element.meta.and_then(|meta| meta.role(&doc, index)) == Some(role.as_str())
            {
                continue;
            }
            let named: Vec<String> = natives.iter().map(|native| format!("<{native}>")).collect();
            reports.push(report(
                attr.span(),
                format!("Prefer to use the native {} element", natural_join(&named)),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferNativeElement;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run(
            "prefer-native-element.txt",
            "prefer-native-element",
            &|options| {
                let mapping = options.and_then(|options| match options.get("mapping") {
                    Some(crate::template::corpus::Value::Object(entries)) => Some(
                        entries
                            .iter()
                            .map(|(role, natives)| {
                                let list = match natives {
                                    crate::template::corpus::Value::List(list) => list
                                        .iter()
                                        .map(crate::template::corpus::Value::display)
                                        .collect(),
                                    other => vec![other.display()],
                                };
                                (role.clone(), list)
                            })
                            .collect(),
                    ),
                    _ => None,
                });
                Box::new(PreferNativeElement {
                    mapping,
                    include: options.and_then(|options| options.texts("include")),
                    exclude: options.and_then(|options| options.texts("exclude")),
                })
            },
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (14, 6, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
