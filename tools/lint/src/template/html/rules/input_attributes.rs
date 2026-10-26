//! `input-attributes` of `html-validate` 11.17.0 (`src/rules/input-attributes.ts`,
//! REF-HTML-VALIDATE-SOURCE): an attribute of an `<input>` its written `type` does not use,
//! reported at the attribute's name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The text field types (`text`, `search`, `url`, `tel`, `email`, `password`).
const TEXT: &[&str] = &["text", "search", "url", "tel", "email", "password"];
/// The date, time and number types.
const RANGE: &[&str] = &[
    "date",
    "month",
    "week",
    "time",
    "datetime-local",
    "number",
    "range",
];

/// Each restricted attribute and the types that use it (`restricted`).
const RESTRICTED: &[(&str, &[&str])] = &[
    ("accept", &["file"]),
    ("alpha", &["color"]),
    ("alt", &["image"]),
    (
        "autocapitalize",
        &[
            "button",
            "checkbox",
            "color",
            "date",
            "datetime-local",
            "file",
            "hidden",
            "image",
            "month",
            "number",
            "radio",
            "range",
            "reset",
            "search",
            "submit",
            "tel",
            "text",
            "time",
            "week",
        ],
    ),
    (
        "autocomplete",
        &[
            "color",
            "date",
            "datetime-local",
            "email",
            "file",
            "hidden",
            "image",
            "month",
            "number",
            "password",
            "range",
            "search",
            "tel",
            "text",
            "time",
            "url",
            "week",
        ],
    ),
    ("capture", &["file"]),
    ("checked", &["checkbox", "radio"]),
    ("colorspace", &["color"]),
    (
        "dirname",
        &["hidden", "text", "search", "url", "tel", "email"],
    ),
    ("height", &["image"]),
    (
        "list",
        &[
            "text",
            "search",
            "url",
            "tel",
            "email",
            "date",
            "month",
            "week",
            "time",
            "datetime-local",
            "number",
            "range",
            "color",
        ],
    ),
    ("max", RANGE),
    ("maxlength", TEXT),
    ("min", RANGE),
    ("minlength", TEXT),
    ("multiple", &["email", "file"]),
    ("pattern", TEXT),
    (
        "placeholder",
        &[
            "text", "search", "url", "tel", "email", "password", "number",
        ],
    ),
    ("popovertarget", &["button"]),
    ("popovertargetaction", &["button"]),
    (
        "readonly",
        &[
            "text",
            "search",
            "url",
            "tel",
            "email",
            "password",
            "date",
            "month",
            "week",
            "time",
            "datetime-local",
            "number",
        ],
    ),
    (
        "required",
        &[
            "text",
            "search",
            "url",
            "tel",
            "email",
            "password",
            "date",
            "month",
            "week",
            "time",
            "datetime-local",
            "number",
            "checkbox",
            "radio",
            "file",
        ],
    ),
    ("size", TEXT),
    ("src", &["image"]),
    ("step", RANGE),
    (
        "value",
        &[
            "button",
            "checkbox",
            "color",
            "date",
            "datetime-local",
            "email",
            "hidden",
            "month",
            "number",
            "password",
            "radio",
            "range",
            "reset",
            "search",
            "submit",
            "tel",
            "text",
            "time",
            "url",
            "week",
        ],
    ),
    ("width", &["image"]),
];

/// The rule.
pub struct InputAttributes;

impl TemplateRule for InputAttributes {
    fn name(&self) -> &'static str {
        "html-validate/input-attributes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if element.tag != "input" {
                continue;
            }
            let Some(kind) = doc.value(index, "type").filter(|kind| !kind.is_empty()) else {
                continue;
            };
            for attr in &element.attrs {
                let Some((_, types)) = RESTRICTED.iter().find(|(name, _)| *name == attr.key) else {
                    continue;
                };
                if !types.contains(&kind) {
                    reports.push(report(
                        attr.key_span(),
                        format!(
                            "Attribute \"{}\" is not allowed on <input type=\"{kind}\">",
                            attr.key
                        ),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InputAttributes;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("input-attributes.txt", "input-attributes", &|_| {
            Box::new(InputAttributes)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (5, 1, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
