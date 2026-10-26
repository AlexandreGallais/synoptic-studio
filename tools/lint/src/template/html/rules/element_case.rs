//! `element-case` of `html-validate` 11.17.0 (`src/rules/element-case.ts`,
//! `src/rules/helper/case-style.ts`, REF-HTML-VALIDATE-SOURCE): an element's name, its letters, in
//! the asked case, lowercase for the project, the rule's default, else reported at the name and
//! fixed, its end tag renamed with it. An end tag written in another case than its start tag is
//! refused by Angular's parser (`lint/parse`), which matches them in the same case: the rule's
//! check of it is left out. A foreign element's children are not read, nor checked.

use regex::Regex;

use super::super::view::Doc;
use super::super::{MESSAGES, fixed, report};
use crate::finding::Fix;
use crate::template::lexer::{Span, offset};
use crate::template::rules::{Report, Template, TemplateRule};
use crate::template::tags::split_ns_name;
use crate::template::tree::Element;

/// A case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Case {
    /// `lowercase`.
    Lower,
    /// `uppercase`.
    Upper,
    /// `PascalCase`.
    Pascal,
    /// `camelCase`.
    Camel,
}

impl Case {
    /// Its pattern and its name.
    const fn style(self) -> (&'static str, &'static str) {
        match self {
            Self::Lower => ("^[a-z]*$", "lowercase"),
            Self::Upper => ("^[A-Z]*$", "uppercase"),
            Self::Pascal => ("^[A-Z][A-Za-z]*$", "PascalCase"),
            Self::Camel => ("^[a-z][A-Za-z]*$", "camelCase"),
        }
    }
}

/// The rule and its option.
pub struct ElementCase {
    /// The cases accepted, lowercase when empty.
    pub styles: Vec<Case>,
}

impl ElementCase {
    /// The project's configuration: lowercase.
    pub const PROJECT: Self = Self { styles: Vec::new() };

    /// The cases accepted.
    fn cases(&self) -> Vec<Case> {
        if self.styles.is_empty() {
            vec![Case::Lower]
        } else {
            self.styles.clone()
        }
    }
}

impl TemplateRule for ElementCase {
    fn name(&self) -> &'static str {
        "html-validate/element-case"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let cases = self.cases();
        let styles: Vec<&str> = cases.iter().map(|case| case.style().1).collect();
        let named = match styles.as_slice() {
            [one] => (*one).to_owned(),
            [rest @ .., last] => format!("{} or {last}", rest.join(", ")),
            [] => String::new(),
        };
        let letters = Regex::new("[^a-zA-Z]+").ok();
        for index in 0..doc.elements.len() {
            let element = doc.el(index).element;
            let span = doc.name_span(index);
            let written = template
                .text
                .get(span.start as usize..span.end as usize)
                .unwrap_or_default();
            let tag = split_ns_name(&element.name).1;
            let only = letters.as_ref().map_or_else(
                || tag.to_owned(),
                |regex| regex.replace_all(tag, "").into_owned(),
            );
            let fits = cases
                .iter()
                .any(|case| Regex::new(case.style().0).is_ok_and(|regex| regex.is_match(&only)));
            if !fits {
                let message = format!("Element \"{written}\" should be {named}");
                let fix = match cases.as_slice() {
                    [Case::Lower] => Some(written.to_lowercase()),
                    [Case::Upper] => Some(written.to_uppercase()),
                    _ => None,
                };
                reports.push(match fix {
                    Some(text) => {
                        fixed(span, message, renamed(template.text, element, span, &text))
                    }
                    None => report(span, message),
                });
            }
        }
    }
}

/// The fix renaming an element: its start tag's name, and its end tag's with it, as Angular's
/// parser matches an end tag to its start tag in the same case; the text between kept as written.
fn renamed(text: &str, element: &Element, name: Span, new: &str) -> Fix {
    let Some(end) = element.end_span.filter(|end| end.end > end.start) else {
        return Fix {
            start: name.start,
            end: name.end,
            text: new.to_owned(),
        };
    };
    let closing = end.start as usize + 2;
    let closing_end = closing + (name.end - name.start) as usize;
    let between = text.get(name.end as usize..closing).unwrap_or_default();
    Fix {
        start: name.start,
        end: offset(closing_end),
        text: format!("{new}{between}{new}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Case, ElementCase};
    use crate::template::corpus::Value;
    use crate::template::html::cases::{fixed, run};

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("element-case.txt", "element-case", &|options| {
            let case = |name: &str| match name {
                "uppercase" => Case::Upper,
                "pascalcase" => Case::Pascal,
                "camelcase" => Case::Camel,
                _ => Case::Lower,
            };
            let styles = match options.and_then(|options| options.get("style")) {
                Some(Value::Text(name)) => vec![case(name)],
                Some(Value::List(list)) => list.iter().map(|each| case(&each.display())).collect(),
                _ => Vec::new(),
            };
            Box::new(ElementCase { styles })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (6, 6, 12),
            "{:#?}",
            tally.apart_cases
        );
    }

    #[test]
    fn writes_an_element_in_lowercase() {
        // Our own cases: CRLF line ends and an entity before keep the offsets; the end tag is
        // renamed with the start tag, a value read decoded moved whole between them.
        assert_eq!(
            fixed(&ElementCase::PROJECT, "<p>&amp;\r\n</p>\r\n<DIV>\r\n</DIV>"),
            "<p>&amp;\r\n</p>\r\n<div>\r\n</div>"
        );
        assert_eq!(
            fixed(
                &ElementCase::PROJECT,
                "<Span [title]=\"a &amp;&amp; b\">x</Span><BR>"
            ),
            "<span [title]=\"a &amp;&amp; b\">x</span><br>"
        );
    }
}
