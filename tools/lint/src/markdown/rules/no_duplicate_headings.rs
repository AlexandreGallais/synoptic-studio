//! `markdown/no-duplicate-headings`, `checkSiblingsOnly`: two headings under the same parent
//! heading differ, so that each has its own anchor and the outline reads; the same title under
//! two parents is allowed.

use std::collections::HashSet;
use std::fmt::Write as _;

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, descendants, nodes, start, type_name, value};

/// The rule.
pub struct NoDuplicateHeadings;

impl MarkdownRule for NoDuplicateHeadings {
    fn name(&self) -> &'static str {
        "markdown/no-duplicate-headings"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let mut levels: [HashSet<String>; 6] = Default::default();
        let mut last = 1;

        for node in nodes(&page.root) {
            let Node::Heading(heading) = node else {
                continue;
            };
            let level = usize::from(heading.depth);
            for deeper in level + 1..=last {
                levels[deeper - 1].clear();
            }
            last = level;

            let mut sequence = String::new();
            let mut text = String::new();
            for child in descendants(node) {
                match value(child).filter(|value| !value.is_empty()) {
                    Some(value) => {
                        let _ = write!(sequence, "[{},{value}]", type_name(child));
                        if !matches!(child, Node::Html(_)) {
                            text.push_str(value);
                        }
                    }
                    None => {
                        let _ = write!(sequence, "[{}]", type_name(child));
                    }
                }
            }
            if !levels[level - 1].insert(sequence) {
                findings.push(page.finding(
                    self.name(),
                    start(node),
                    format!("Duplicate heading \"{text}\" found."),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateHeadings;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_a_heading_repeated_under_the_same_parent() {
        let page =
            "# A\n\n## Hello *world*\n\n## Hello *world*\n\n## Hello world\n\n### `x`\n\n### `x`\n";

        assert_eq!(
            places(&NoDuplicateHeadings, page),
            [
                (5, 1, "Duplicate heading \"Hello world\" found.".to_owned()),
                (11, 1, "Duplicate heading \"x\" found.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_the_same_titles_under_two_parents() {
        // The documented case of `checkSiblingsOnly`.
        let page = "# Change log\n\n## 1.0.0\n\n### Features\n\n### Bug Fixes\n\n## 2.0.0\n\n### Features\n\n### Bug Fixes\n";

        assert_eq!(messages(&NoDuplicateHeadings, page), NONE);
    }
}
