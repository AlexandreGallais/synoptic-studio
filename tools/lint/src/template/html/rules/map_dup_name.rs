//! `map-dup-name` of `html-validate` 11.17.0 (`src/rules/map-dup-name.ts`,
//! REF-HTML-VALIDATE-SOURCE): a `<map>` whose static `name` an earlier `<map>` has is reported at
//! its `name`.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct MapDupName;

impl TemplateRule for MapDupName {
    fn name(&self) -> &'static str {
        "html-validate/map-dup-name"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let mut names: Vec<&str> = Vec::new();
        for (index, element) in doc.elements.iter().enumerate() {
            if element.tag != "map" {
                continue;
            }
            let Some(attr) = doc.attr(index, "name") else {
                continue;
            };
            let Some(name) = attr.text().filter(|name| !name.is_empty()) else {
                continue;
            };
            if names.contains(&name) {
                reports.push(report(
                    attr.key_span(),
                    "<map> name must be unique".to_owned(),
                ));
            }
            names.push(name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MapDupName;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("map-dup-name.txt", "map-dup-name", &|_| {
            Box::new(MapDupName)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 1, 1),
            "{:#?}",
            tally.apart_cases
        );
    }
}
