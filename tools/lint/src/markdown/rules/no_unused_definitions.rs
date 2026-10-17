//! `markdown/no-unused-definitions`: a link definition or a footnote is referenced, or it is
//! dead text; `[//]: #`, the comment idiom, is allowed.

use std::collections::HashSet;

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::rules::no_duplicate_definitions::ALLOWED_DEFINITIONS;

/// The rule.
pub struct NoUnusedDefinitions;

impl MarkdownRule for NoUnusedDefinitions {
    fn name(&self) -> &'static str {
        "markdown/no-unused-definitions"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let all = nodes(&page.root);
        let mut used = HashSet::new();
        let mut used_footnotes = HashSet::new();
        for node in &all {
            match node {
                Node::LinkReference(each) => used.insert(each.identifier.as_str()),
                Node::ImageReference(each) => used.insert(each.identifier.as_str()),
                Node::FootnoteReference(each) => used_footnotes.insert(each.identifier.as_str()),
                _ => false,
            };
        }

        for node in all {
            let message = match node {
                Node::Definition(definition)
                    if !used.contains(definition.identifier.as_str())
                        && !ALLOWED_DEFINITIONS.contains(&definition.identifier.as_str()) =>
                {
                    format!(
                        "Unexpected unused definition `{}` (label: `{}`) found.",
                        definition.identifier,
                        definition.label.as_deref().unwrap_or_default().trim()
                    )
                }
                Node::FootnoteDefinition(footnote)
                    if !used_footnotes.contains(footnote.identifier.as_str()) =>
                {
                    format!(
                        "Unexpected unused footnote definition `{}` (label: `{}`) found.",
                        footnote.identifier,
                        footnote.label.as_deref().unwrap_or_default()
                    )
                }
                _ => continue,
            };
            findings.push(page.finding(self.name(), start(node), message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnusedDefinitions;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_unused_definitions() {
        let page = "[mercury]: https://example.com/mercury/\n[Venus]: https://example.com/venus.jpg\n\n[^mercury]: Hello, Mercury!\n";

        assert_eq!(
            places(&NoUnusedDefinitions, page),
            [
                (
                    1,
                    1,
                    "Unexpected unused definition `mercury` (label: `mercury`) found.".to_owned()
                ),
                (
                    2,
                    1,
                    "Unexpected unused definition `venus` (label: `Venus`) found.".to_owned()
                ),
                (
                    4,
                    1,
                    "Unexpected unused footnote definition `mercury` (label: `mercury`) found."
                        .to_owned()
                ),
            ]
        );
    }

    #[test]
    fn accepts_used_definitions_and_the_comment_idiom() {
        let page = "[Mercury][mercury]\n\n![Venus Image][venus]\n\nMercury[^mercury]\n\n[mercury]: https://example.com/mercury/\n[venus]: https://example.com/venus.jpg\n\n[^mercury]: Hello, Mercury!\n\n[//]: # (This is a comment 1)\n";

        assert_eq!(messages(&NoUnusedDefinitions, page), NONE);
    }
}
