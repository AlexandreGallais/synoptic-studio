//! `map-id-name` of `html-validate` 11.17.0 (`src/rules/map-id-name.ts`,
//! REF-HTML-VALIDATE-SOURCE): a `<map>` whose static `id` and `name` differ is reported at the
//! `id`'s value, else the `name`'s.

use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct MapIdName;

impl TemplateRule for MapIdName {
    fn name(&self) -> &'static str {
        "html-validate/map-id-name"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            if element.tag != "map" {
                continue;
            }
            let (Some(id), Some(name)) = (doc.attr(index, "id"), doc.attr(index, "name")) else {
                continue;
            };
            if id.value == Value::Dynamic
                || name.value == Value::Dynamic
                || id.text() == name.text()
            {
                continue;
            }
            // An empty value has no place in html-validate (`valueLocation` is `null`).
            let span = if id.text().is_some_and(|value| !value.is_empty()) {
                id.value_span()
            } else if name.text().is_some_and(|value| !value.is_empty()) {
                name.value_span()
            } else {
                doc.name_span(index)
            };
            reports.push(report(
                span,
                "\"id\" and \"name\" attribute must be the same on <map> elements".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MapIdName;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("map-id-name.txt", "map-id-name", &|_| Box::new(MapIdName));

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 3, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
