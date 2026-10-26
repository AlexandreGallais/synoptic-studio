//! `no-dup-class` of `html-validate` 11.17.0 (`src/rules/no-dup-class.ts`,
//! REF-HTML-VALIDATE-SOURCE): a class written twice in one `class` is reported at its second
//! occurrence and fixed by removing it with the whitespace before it.

use super::super::view::Doc;
use super::super::{MESSAGES, fixed, remove, tokens};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct NoDupClass;

impl TemplateRule for NoDupClass {
    fn name(&self) -> &'static str {
        "html-validate/no-dup-class"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for element in &doc.elements {
            for attr in element.attrs.iter().filter(|attr| attr.key == "class") {
                let Some(value) = attr.text() else {
                    continue;
                };
                let mut seen: Vec<&str> = Vec::new();
                for (token, span) in tokens(value, attr.value_span()) {
                    if seen.contains(&token) {
                        reports.push(fixed(
                            span,
                            format!("Class \"{token}\" duplicated"),
                            remove(template.text, span, true, false),
                        ));
                    }
                    seen.push(token);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDupClass;
    use crate::template::html::cases::{fixed, run};

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-dup-class.txt", "no-dup-class", &|_| {
            Box::new(NoDupClass)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (3, 1, 3),
            "{:#?}",
            tally.apart_cases
        );
    }

    #[test]
    fn fixes_a_duplicate_class() {
        // The case "should autofix by removing duplicated class", which reads the report's
        // messages, run here.
        assert_eq!(
            fixed(&NoDupClass, " <div class=\"foo bar foo\"></div> "),
            " <div class=\"foo bar\"></div> "
        );
        // Our own case: CRLF line ends and an entity before the duplicate keep their offsets.
        assert_eq!(
            fixed(&NoDupClass, "<p\r\n  class=\"a&amp;\r\n a\r\n a\"></p>"),
            "<p\r\n  class=\"a&amp;\r\n a\"></p>"
        );
    }
}
