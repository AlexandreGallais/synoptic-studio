//! `aria-label-misuse` of `html-validate` 11.17.0 (`src/rules/aria-label-misuse.ts`,
//! REF-HTML-VALIDATE-SOURCE): an `aria-label` or `aria-labelledby` with a value on an element
//! that is neither a landmark nor one of the elements allowed, nor given a role, a `tabindex`, nor
//! interactive or labelable, reported at its name; strictly allowed when the element may be
//! named, which `allowAnyNamable` accepts.

use super::super::a11y::naming_allowed;
use super::super::view::{Doc, Value};
use super::super::{MESSAGES, is_pattern_ignored, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The elements an `aria-label` is valid on (`allowlist`).
const ALLOWED: [&str; 21] = [
    "article", "aside", "footer", "form", "header", "main", "nav", "search", "section", "area",
    "dialog", "fieldset", "figure", "iframe", "img", "summary", "table", "ul", "ol", "td", "th",
];

/// The rule and its options.
pub struct AriaLabelMisuse {
    /// Whether an element that may be named is accepted.
    pub allow_any_namable: bool,
    /// The elements checked, patterns, all when `None`.
    pub include: Option<Vec<String>>,
    /// The elements not checked, patterns.
    pub exclude: Option<Vec<String>>,
}

impl AriaLabelMisuse {
    /// The project's configuration: the rule's defaults.
    pub const PROJECT: Self = Self {
        allow_any_namable: false,
        include: None,
        exclude: None,
    };
}

/// Whether the element may carry the label (`isValidUsage`).
fn is_valid_usage(doc: &Doc, index: usize) -> bool {
    let element = doc.el(index);
    let Some(meta) = element.meta else {
        return true;
    };
    ALLOWED.contains(&element.tag.as_str())
        || doc.has(index, "role")
        || doc.has(index, "tabindex")
        || meta.interactive.of(doc, index)
        || meta.labelable.of(doc, index)
}

impl TemplateRule for AriaLabelMisuse {
    fn name(&self) -> &'static str {
        "html-validate/aria-label-misuse"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let ignored = is_pattern_ignored(
                self.include.as_deref(),
                self.exclude.as_deref(),
                &element.tag,
            );
            if element.meta.is_none() || ignored || is_valid_usage(&doc, index) {
                continue;
            }
            for key in ["aria-label", "aria-labelledby"] {
                let Some(attr) = doc.attr(index, key) else {
                    continue;
                };
                if matches!(&attr.value, Value::Omitted) || attr.text() == Some("") {
                    continue;
                }
                let allows = naming_allowed(&doc, index);
                if allows && self.allow_any_namable {
                    continue;
                }
                let message = if allows {
                    format!(
                        "\"{key}\" is strictly allowed but is not recommended to be used on this element"
                    )
                } else {
                    format!("\"{key}\" cannot be used on this element")
                };
                reports.push(report(attr.key_span(), message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AriaLabelMisuse;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("aria-label-misuse.txt", "aria-label-misuse", &|options| {
            let elements = options.and_then(|options| options.get("elements"));
            Box::new(AriaLabelMisuse {
                allow_any_namable: options
                    .and_then(|options| options.flag("allowAnyNamable"))
                    .unwrap_or(false),
                include: elements.and_then(|elements| elements.texts("include")),
                exclude: elements.and_then(|elements| elements.texts("exclude")),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (43, 8, 5),
            "{:#?}",
            tally.apart_cases
        );
    }
}
