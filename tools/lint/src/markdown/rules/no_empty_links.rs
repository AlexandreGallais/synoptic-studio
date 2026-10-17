//! `markdown/no-empty-links`: a link has a destination other than `#`.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::rules::no_empty_definitions::is_empty_url;

/// The rule.
pub struct NoEmptyLinks;

impl MarkdownRule for NoEmptyLinks {
    fn name(&self) -> &'static str {
        "markdown/no-empty-links"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            if matches!(node, Node::Link(link) if is_empty_url(&link.url)) {
                findings.push(page.finding(
                    self.name(),
                    start(node),
                    "Unexpected empty link found.".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoEmptyLinks;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_empty_links() {
        assert_eq!(
            places(&NoEmptyLinks, "[]()\n\n[ESLint](#)\n"),
            [
                (1, 1, "Unexpected empty link found.".to_owned()),
                (3, 1, "Unexpected empty link found.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_a_link_with_a_destination() {
        assert_eq!(
            messages(&NoEmptyLinks, "[ESLint](https://eslint.org/) and [a](#b)\n"),
            NONE
        );
    }
}
