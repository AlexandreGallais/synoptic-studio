//! `markdown/no-html`, `span` allowed: a page is Markdown, not HTML, which GitHub and the site
//! render differently; `<span v-pre>` is the site's way to show double curly braces.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::text::{opening_tag, strip_html_comments};

/// The rule.
pub struct NoHtml;

/// The elements allowed, by their name as written.
pub const ALLOWED: [&str; 1] = ["span"];

impl MarkdownRule for NoHtml {
    fn name(&self) -> &'static str {
        "markdown/no-html"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            if !matches!(node, Node::Html(_)) {
                continue;
            }
            let text = strip_html_comments(page.source(node));
            let mut at = 0;
            while let Some(open) = text[at..].find('<').map(|found| at + found) {
                match opening_tag(&text, open) {
                    Some((name, end)) => {
                        if !ALLOWED.contains(&name) {
                            findings.push(page.finding(
                                self.name(),
                                start(node) + open,
                                format!("HTML element \"{name}\" is not allowed."),
                            ));
                        }
                        at = end;
                    }
                    None => at = open + 1,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoHtml;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_each_element_but_span() {
        // The documented case, then a block of HTML with a comment, and an inline tag.
        let page = "<b>Hello world!</b>\n\n<div>\n<!-- <p> -->\n<p class=\"a>b\">x</p>\n</div>\n\nA <Span>b</Span> <span v-pre>{{ c }}</span>\n";

        assert_eq!(
            places(&NoHtml, page),
            [
                (1, 1, "HTML element \"b\" is not allowed.".to_owned()),
                (3, 1, "HTML element \"div\" is not allowed.".to_owned()),
                (5, 1, "HTML element \"p\" is not allowed.".to_owned()),
                (8, 3, "HTML element \"Span\" is not allowed.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_markdown_comments_and_span() {
        assert_eq!(
            messages(
                &NoHtml,
                "# Hello world!\n\n<!-- a -->\n\n<span v-pre>{{ a }}</span>\n"
            ),
            NONE
        );
    }
}
