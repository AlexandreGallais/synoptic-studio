//! `no-utf8-bom` of `html-validate` 11.17.0 (`src/rules/no-utf8-bom.ts`,
//! REF-HTML-VALIDATE-SOURCE): a template starting with a byte order mark is reported at it, and
//! fixed by removing it.

use super::super::{MESSAGES, fixed};
use crate::finding::Fix;
use crate::template::lexer::Span;
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule.
pub struct NoUtf8Bom;

impl TemplateRule for NoUtf8Bom {
    fn name(&self) -> &'static str {
        "html-validate/no-utf8-bom"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        if template.text.starts_with('\u{feff}') {
            let length = '\u{feff}'.len_utf8();
            reports.push(fixed(
                Span::new(0, length),
                "File should be saved without UTF-8 BOM".to_owned(),
                Fix {
                    start: 0,
                    end: crate::template::lexer::offset(length),
                    text: String::new(),
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUtf8Bom;
    use crate::template::html::cases::{fixed, run};

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-utf8-bom.txt", "no-utf8-bom", &|_| Box::new(NoUtf8Bom));

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (2, 1, 2),
            "{:#?}",
            tally.apart_cases
        );
    }

    #[test]
    fn removes_the_byte_order_mark() {
        // Our own case: the fix leaves the rest, CRLF line ends included.
        assert_eq!(fixed(&NoUtf8Bom, "\u{feff}<p>\r\n</p>"), "<p>\r\n</p>");
    }
}
