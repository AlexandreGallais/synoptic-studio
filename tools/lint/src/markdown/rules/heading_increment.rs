//! `markdown/heading-increment`: each heading is at most one level below the one before it, with
//! no front matter title read as a first level (`frontmatterTitle: ""`: the pages' front matter
//! holds the backlog's fields, never their title).

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};

/// The rule.
pub struct HeadingIncrement;

impl MarkdownRule for HeadingIncrement {
    fn name(&self) -> &'static str {
        "markdown/heading-increment"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let mut last = 0;

        for node in nodes(&page.root) {
            let Node::Heading(heading) = node else {
                continue;
            };
            if last > 0 && heading.depth > last + 1 {
                findings.push(page.finding(
                    self.name(),
                    start(node),
                    format!("Heading level skipped from {last} to {}.", heading.depth),
                ));
            }
            last = heading.depth;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HeadingIncrement;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_a_skipped_level() {
        // The documented case: 1 then 3, then a setext 2 then 4.
        assert_eq!(
            places(
                &HeadingIncrement,
                "# Hello world!\n\n### Hello world!\n\nGoodbye World!\n--------------\n\n#### Goodbye World!\n"
            ),
            [
                (3, 1, "Heading level skipped from 1 to 3.".to_owned()),
                (8, 1, "Heading level skipped from 2 to 4.".to_owned()),
            ]
        );
    }

    #[test]
    fn reads_no_title_in_front_matter_and_starts_anywhere() {
        assert_eq!(
            messages(
                &HeadingIncrement,
                "---\ntitle: My Title\n---\n\n### Heading 3\n\n#### Four\n\n## Two\n"
            ),
            NONE
        );
    }
}
