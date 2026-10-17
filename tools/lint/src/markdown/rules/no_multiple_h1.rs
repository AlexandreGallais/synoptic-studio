//! `markdown/no-multiple-h1`: a page has one title, a first-level heading in Markdown or an HTML
//! `<h1>`; no front matter title counts (`frontmatterTitle: ""`).

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::text::{find_without_case, is_js_space, strip_html_comments};

/// The rule.
pub struct NoMultipleH1;

/// The offsets of each `<h1…>…</h1>` of a text, `<h1[^>]*>[\s\S]*?<\/h1\s*>` without case.
fn h1_tags(text: &str) -> Vec<usize> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;

    while let Some(open) = find_without_case(text, "<h1", at) {
        at = open + 1;
        let Some(content) = text[open + 3..].find('>').map(|end| open + 3 + end + 1) else {
            continue;
        };
        let mut search = content;
        while let Some(close) = find_without_case(text, "</h1", search) {
            let after = close + 4;
            let spaces: usize = text[after..]
                .chars()
                .take_while(|character| is_js_space(*character))
                .map(char::len_utf8)
                .sum();
            if bytes.get(after + spaces) == Some(&b'>') {
                found.push(open);
                at = after + spaces + 1;
                break;
            }
            search = close + 1;
        }
    }
    found
}

impl MarkdownRule for NoMultipleH1 {
    fn name(&self) -> &'static str {
        "markdown/no-multiple-h1"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let mut count = 0;
        let mut report = |offset: usize, findings: &mut Vec<Finding>| {
            count += 1;
            if count > 1 {
                findings.push(page.finding(
                    self.name(),
                    offset,
                    "Unexpected additional H1 heading found.".into(),
                ));
            }
        };

        for node in nodes(&page.root) {
            match node {
                Node::Heading(heading) if heading.depth == 1 => report(start(node), findings),
                Node::Html(html) => {
                    for offset in h1_tags(&strip_html_comments(&html.value)) {
                        report(start(node) + offset, findings);
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoMultipleH1;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    const MESSAGE: &str = "Unexpected additional H1 heading found.";

    #[test]
    fn reports_the_documented_additional_titles() {
        let page = "# Heading 1\n\n# Another H1 heading\n\nAnother H1 heading\n==================\n\n<h1>First</h1>\n<H1 class=\"a\">Second</h1 >\n";

        assert_eq!(
            places(&NoMultipleH1, page),
            [
                (3, 1, MESSAGE.to_owned()),
                (5, 1, MESSAGE.to_owned()),
                (8, 1, MESSAGE.to_owned()),
                (9, 1, MESSAGE.to_owned()),
            ]
        );
    }

    #[test]
    fn reads_no_title_in_front_matter_nor_in_comments() {
        let page = "---\ntitle: My Title\n---\n\n# Heading 1\n\n<!-- <h1>x</h1> -->\n\n## Two\n";

        assert_eq!(messages(&NoMultipleH1, page), NONE);
    }
}
