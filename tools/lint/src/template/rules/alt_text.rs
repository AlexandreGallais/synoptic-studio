//! `@angular-eslint/template/alt-text`: an image, an area, an object and an image input have a
//! text alternative (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/alt-text.ts`): an
//! `<img>` an `alt`, plain or bound; an `<area>` and an `<input type="image">` an `alt`, an
//! `aria-label` or an `aria-labelledby`; an `<object>` a `title`, an `aria-label` or an
//! `aria-labelledby`, or a text as its first child. Each name any case; the element is reported
//! over itself, a void one over its start tag.

use super::a11y::attribute_value;
use super::r3::{R3, siblings};
use super::{Bound3, Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::template::tree::Element;

/// The rule.
pub struct AltText;

/// Whether a name labels an element (`isAriaLabel`).
fn is_aria_label(name: &str) -> bool {
    name == "aria-label" || name == "aria-labelledby"
}

/// Whether an element has a plain attribute, or else an input, of a name the test accepts.
fn has(attributes: &[Bound3], accepts: &dyn Fn(&str) -> bool) -> bool {
    let of = |role| {
        attributes
            .iter()
            .any(|each| each.role == role && accepts(&each.name))
    };
    of(Role::Text) || of(Role::Input)
}

/// Whether an element has its text alternative.
fn is_valid(template: &Template, element: &Element) -> bool {
    let attributes = bound3(element);
    match element.name.to_lowercase().as_str() {
        "img" => has(&attributes, &|name| name == "alt"),
        "object" => {
            has(&attributes, &|name| name == "title" || is_aria_label(name))
                // A text, or an interpolation, as the first child has a truthy `value`.
                || matches!(siblings(&element.children).first(), Some(R3::Text(_)))
        }
        "area" => has(&attributes, &|name| name == "alt" || is_aria_label(name)),
        _ => {
            attribute_value(template.allocator, element, "type").text() != Some("image")
                || has(&attributes, &|name| name == "alt" || is_aria_label(name))
        }
    }
}

impl TemplateRule for AltText {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/alt-text"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "altText",
            "<{{element}}/> element must have a text alternative.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            let name = element.name.to_lowercase();
            if !matches!(name.as_str(), "img" | "area" | "object" | "input")
                || !is_plain_element(element)
                || is_valid(template, element)
            {
                continue;
            }
            reports.push(Report {
                data: vec![("element", element.name.clone())],
                ..Report::new(element.span, "altText")
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AltText;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("alt-text.txt", &[], &|_| Box::new(AltText));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 25 valid and 6 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (25, 6, 0));
    }
}
