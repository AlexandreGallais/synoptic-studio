//! `attribute-misuse` of `html-validate` 11.17.0 (`src/rules/attribute-misuse.ts`,
//! REF-HTML-VALIDATE-SOURCE): an attribute its element's metadata allows only in another context
//! (`download` without `href`), reported at its name with the reason.

use super::super::elements::attribute;
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct AttributeMisuse;

impl TemplateRule for AttributeMisuse {
    fn name(&self) -> &'static str {
        "html-validate/attribute-misuse"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, element) in doc.elements.iter().enumerate() {
            let Some(meta) = element.meta else {
                continue;
            };
            for attr in &element.attrs {
                let Some(allowed) = attribute(meta, &attr.key).and_then(|rule| rule.allowed) else {
                    continue;
                };
                if let Some(details) = allowed.check(&doc, index, Some(&attr.value)) {
                    reports.push(report(
                        attr.key_span(),
                        format!(
                            "\"{}\" attribute cannot be used on <{}> in this context: {details}",
                            attr.key, element.tag
                        ),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AttributeMisuse;
    use crate::template::html::cases::{messages, run};

    #[test]
    fn reports_an_attribute_used_out_of_its_context() {
        // Our own cases: `download` needs `href`; a bound `[download]` too; with `href` it is
        // allowed. `<a download></a>` is 16 characters, `<a ` 3 more: `[download]` at column 20.
        assert_eq!(
            messages(
                &AttributeMisuse,
                "<a download></a><a [download]=\"x\"></a><a href=\"x\" download></a>"
            ),
            [
                "1:4 \"download\" attribute cannot be used on <a> in this context: requires \"href\" attribute to be present",
                "1:20 \"download\" attribute cannot be used on <a> in this context: requires \"href\" attribute to be present",
            ]
        );
    }

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("attribute-misuse.txt", "attribute-misuse", &|_| {
            Box::new(AttributeMisuse)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (0, 0, 3),
            "{:#?}",
            tally.apart_cases
        );
    }
}
