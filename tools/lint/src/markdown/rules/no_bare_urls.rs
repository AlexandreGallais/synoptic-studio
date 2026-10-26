//! `markdown/no-bare-urls`: a URL is written as an autolink `<…>` or a link `[…](…)`, never bare,
//! which GFM links but `CommonMark` and the site's parser may not. Autofixed to `<URL>`.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, descendants, is_inline_container, nodes, range};

/// The rule.
pub struct NoBareUrls;

/// The name of an HTML tag the text opens or closes, lower-cased, and whether it closes:
/// `^<(?<tagName>[^!>][^/\s>]*)`.
fn tag(html: &str) -> Option<(String, bool)> {
    let rest = html.strip_prefix('<')?;
    let first = rest
        .chars()
        .next()
        .filter(|first| !matches!(first, '!' | '>'))?;
    let length = first.len_utf8()
        + rest[first.len_utf8()..]
            .chars()
            .take_while(|character| {
                !matches!(character, '/' | '>') && !crate::javascript::is_js_space(*character)
            })
            .map(char::len_utf8)
            .sum::<usize>();
    let name = rest[..length].to_lowercase();

    Some(match name.strip_prefix('/') {
        Some(closed) => (closed.to_owned(), true),
        None => (name, false),
    })
}

/// The links of a heading, a paragraph or a table cell outside an HTML element they sit in.
fn links<'n>(container: &'n Node, found: &mut Vec<&'n Node>) {
    let mut open: Option<String> = None;
    let mut inside = Vec::new();

    for node in descendants(container) {
        match node {
            Node::Html(html) => {
                if let Some((name, closing)) = tag(&html.value) {
                    if !closing && open.is_none() {
                        open = Some(name);
                    } else if closing && open.as_deref() == Some(name.as_str()) {
                        inside.clear();
                        open = None;
                    }
                }
            }
            Node::Link(_) if open.is_some() => inside.push(node),
            Node::Link(_) => found.push(node),
            _ => {}
        }
    }
    found.append(&mut inside);
}

impl MarkdownRule for NoBareUrls {
    fn name(&self) -> &'static str {
        "markdown/no-bare-urls"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let mut found = Vec::new();
        for container in nodes(&page.root)
            .into_iter()
            .filter(|node| is_inline_container(node))
        {
            links(container, &mut found);
        }

        for node in found {
            let Node::Link(link) = node else { continue };
            let text = page.source(node);
            let url = link.url.as_str();
            if url == text || url == format!("http://{text}") || url == format!("mailto:{text}") {
                findings.push(
                    page.fixed(
                        self.name(),
                        range(node).0,
                        "Unexpected bare URL. Use autolink (<URL>) or link ([text](URL)) instead."
                            .into(),
                        range(node),
                        format!("<{text}>"),
                    ),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoBareUrls;
    use crate::markdown::testing::{fixed, messages, places};
    use crate::testing::NONE;

    const MESSAGE: &str =
        "Unexpected bare URL. Use autolink (<URL>) or link ([text](URL)) instead.";

    #[test]
    fn reports_the_documented_bare_urls_and_fixes_them() {
        let page = "For more info, visit https://www.example.com/\n\nContact us at user@example.com\n\n# https://www.example.com/\n\n| a |\n| - |\n| www.example.com |\n";

        assert_eq!(
            places(&NoBareUrls, page),
            [
                (1, 22, MESSAGE.to_owned()),
                (3, 15, MESSAGE.to_owned()),
                (5, 3, MESSAGE.to_owned()),
                (9, 3, MESSAGE.to_owned()),
            ]
        );
        assert_eq!(
            fixed(&NoBareUrls, page),
            "For more info, visit <https://www.example.com/>\n\nContact us at <user@example.com>\n\n# <https://www.example.com/>\n\n| a |\n| - |\n| <www.example.com> |\n"
        );
    }

    #[test]
    fn accepts_autolinks_links_code_and_urls_inside_an_html_element() {
        let page = "For more info, visit <https://www.example.com/>\n\nSee [Example](https://www.example.com/) and [user@example.com](mailto:user@example.com).\n\nNot a link: `https://www.example.com/`\n\n<span>https://www.example.com/</span>\n";

        assert_eq!(messages(&NoBareUrls, page), NONE);
    }

    #[test]
    fn keeps_a_url_after_an_element_left_open() {
        assert_eq!(
            places(&NoBareUrls, "a <b> https://x.org\n"),
            [(1, 7, MESSAGE.to_owned())]
        );
    }
}
