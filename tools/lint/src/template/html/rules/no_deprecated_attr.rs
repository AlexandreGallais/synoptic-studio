//! `no-deprecated-attr` of `html-validate` 11.17.0 (`src/rules/no-deprecated-attr.ts`,
//! REF-HTML-VALIDATE-SOURCE): an attribute its element's metadata marks deprecated is reported at
//! its name, bound or not.

use super::super::elements::attribute;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct NoDeprecatedAttr;

impl TemplateRule for NoDeprecatedAttr {
    fn name(&self) -> &'static str {
        "html-validate/no-deprecated-attr"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for element in &doc.elements {
            let Some(meta) = element.meta else {
                continue;
            };
            for attr in &element.attrs {
                if attribute(meta, &attr.key).is_some_and(|rule| rule.deprecated) {
                    reports.push(report(
                        attr.key_span(),
                        format!(
                            "Attribute \"{}\" is deprecated on <{}> element",
                            attr.written, element.tag
                        ),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDeprecatedAttr;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-deprecated-attr.txt", "no-deprecated-attr", &|_| {
            Box::new(NoDeprecatedAttr)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (1, 0, 6),
            "{:#?}",
            tally.apart_cases
        );
    }
}
