//! `no-implicit-close` of `html-validate` 11.17.0 (`src/rules/no-implicit-close.ts`,
//! REF-HTML-VALIDATE-SOURCE): an element Angular's parser closes without its end tag (`<li>`,
//! `<p>`, `<option>`…), by the end of the template, its parent's or an ancestor's end tag, a
//! sibling of its name or another element, reported at its name.

use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};
use crate::template::tags::{definition, split_ns_name};
use crate::template::tree::{Element, Node};

/// The rule.
pub struct NoImplicitClose;

/// The last byte an element's content reaches: its children's ends, deepest last.
fn content_end(element: &Element) -> u32 {
    let mut end = element.start_span.end;
    for child in &element.children {
        end = end.max(match child {
            Node::Element(each) => each
                .end_span
                .map_or_else(|| content_end(each), |span| span.end),
            Node::Text(text) => text.span.end,
            Node::Comment(comment) => comment.span.end,
            Node::Block(block) => block.span.end,
            Node::Expansion(expansion) => expansion.span.end,
            Node::LetDeclaration(declaration) => declaration.span.end,
        });
    }
    end
}

impl TemplateRule for NoImplicitClose {
    fn name(&self) -> &'static str {
        "html-validate/no-implicit-close"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (index, each) in doc.elements.iter().enumerate() {
            let element = each.element;
            // Only an element whose end tag is optional is closed implicitly; another left open
            // at the template's end is closed by the tree, as html-validate's parser closes it.
            let definition = definition(split_ns_name(&element.name).1);
            let optional = definition.closed_by_parent || !definition.closed_by_children.is_empty();
            if element.end_span.is_some() || element.is_void || element.is_self_closing || !optional
            {
                continue;
            }
            let rest = template
                .text
                .get(content_end(element) as usize..)
                .unwrap_or_default();
            let next = rest.trim_start();
            let name_of = |text: &str| {
                text.chars()
                    .take_while(|character| {
                        !character.is_whitespace() && !matches!(character, '>' | '/')
                    })
                    .collect::<String>()
                    .to_lowercase()
            };
            let tag = &each.tag;
            let message = if next.is_empty() {
                format!("Element <{tag}> is implicitly closed by document ending")
            } else if let Some(closing) = next.strip_prefix("</") {
                let by = name_of(closing);
                if each.parent.is_some_and(|parent| doc.el(parent).tag == by) {
                    format!("Element <{tag}> is implicitly closed by parent </{by}>")
                } else {
                    format!("Element <{tag}> is implicitly closed by ancestor </{by}>")
                }
            } else if let Some(opening) = next.strip_prefix('<') {
                let by = name_of(opening);
                if by == *tag {
                    format!("Element <{tag}> is implicitly closed by sibling")
                } else {
                    format!("Element <{tag}> is implicitly closed by adjacent <{by}>")
                }
            } else {
                format!("Element <{tag}> is implicitly closed by document ending")
            };
            reports.push(report(doc.name_span(index), message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoImplicitClose;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("no-implicit-close.txt", "no-implicit-close", &|_| {
            Box::new(NoImplicitClose)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (1, 6, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
