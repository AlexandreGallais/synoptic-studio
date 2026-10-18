//! `markdown/no-empty-definitions`: a link definition has a URL other than `#`, and a footnote
//! has content other than HTML comments; `[//]: #`, the comment idiom, is allowed.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, children, nodes, start};
use crate::markdown::rules::no_duplicate_definitions::ALLOWED_DEFINITIONS;
use crate::markdown::text::strip_html_comments;

/// The rule.
pub struct NoEmptyDefinitions;

/// Whether a URL leads nowhere: empty, or `#` alone.
#[must_use]
pub fn is_empty_url(url: &str) -> bool {
    url.is_empty() || url == "#"
}

impl MarkdownRule for NoEmptyDefinitions {
    fn name(&self) -> &'static str {
        "markdown/no-empty-definitions"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            match node {
                Node::Definition(definition)
                    if is_empty_url(&definition.url)
                        && !ALLOWED_DEFINITIONS.contains(&definition.identifier.as_str()) =>
                {
                    findings.push(page.finding(
                        self.name(),
                        start(node),
                        format!(
                            "Unexpected empty definition `{}` (label: `{}`) found.",
                            definition.identifier,
                            definition.label.as_deref().unwrap_or_default().trim()
                        ),
                    ));
                }
                Node::FootnoteDefinition(footnote)
                    if children(node).iter().all(|child| {
                        matches!(child, Node::Html(html) if strip_html_comments(&html.value).trim().is_empty())
                    }) =>
                {
                    findings.push(page.finding(
                        self.name(),
                        start(node),
                        format!(
                            "Unexpected empty footnote definition `{}` (label: `{}`) found.",
                            footnote.identifier,
                            footnote.label.as_deref().unwrap_or_default()
                        ),
                    ));
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoEmptyDefinitions;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_empty_definitions() {
        let page = "[earth]: <>\n[mars]: #\n\n[^note]:\n\n[^comment]: <!-- only a comment -->\n";

        assert_eq!(
            places(&NoEmptyDefinitions, page),
            [
                (
                    1,
                    1,
                    "Unexpected empty definition `earth` (label: `earth`) found.".to_owned()
                ),
                (
                    2,
                    1,
                    "Unexpected empty definition `mars` (label: `mars`) found.".to_owned()
                ),
                (
                    4,
                    1,
                    "Unexpected empty footnote definition `note` (label: `note`) found.".to_owned()
                ),
                (
                    6,
                    1,
                    "Unexpected empty footnote definition `comment` (label: `comment`) found."
                        .to_owned()
                ),
            ]
        );
    }

    #[test]
    fn accepts_urls_content_and_the_comment_idiom() {
        let page = "[earth]: https://example.com/earth/\n[//]: # (a comment)\n\n[^note]: A note.\n";

        assert_eq!(messages(&NoEmptyDefinitions, page), NONE);
    }
}
