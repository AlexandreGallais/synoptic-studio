//! `markdown/no-duplicate-definitions`: a link label or a footnote is defined once, since only the
//! first definition counts; `[//]: # (…)`, the comment idiom, may repeat.

use std::collections::HashMap;

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};

/// The rule.
pub struct NoDuplicateDefinitions;

/// The identifiers allowed to repeat: the comment idiom `[//]: #`.
pub const ALLOWED_DEFINITIONS: [&str; 1] = ["//"];

impl MarkdownRule for NoDuplicateDefinitions {
    fn name(&self) -> &'static str {
        "markdown/no-duplicate-definitions"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let mut definitions: HashMap<&str, (usize, String)> = HashMap::new();
        let mut footnotes: HashMap<&str, (usize, String)> = HashMap::new();

        for node in nodes(&page.root) {
            let (identifier, label, seen, kind) = match node {
                Node::Definition(definition) => {
                    if ALLOWED_DEFINITIONS.contains(&definition.identifier.as_str()) {
                        continue;
                    }
                    let label = definition.label.as_deref().unwrap_or_default().trim();
                    (
                        &definition.identifier,
                        label.to_owned(),
                        &mut definitions,
                        "definition",
                    )
                }
                Node::FootnoteDefinition(footnote) => {
                    let label = footnote.label.clone().unwrap_or_default();
                    (
                        &footnote.identifier,
                        label,
                        &mut footnotes,
                        "footnote definition",
                    )
                }
                _ => continue,
            };
            let line = page.place(start(node)).0;
            match seen.get(identifier.as_str()) {
                Some((first_line, first_label)) => findings.push(page.finding(
                    self.name(),
                    start(node),
                    format!(
                        "Unexpected duplicate {kind} `{identifier}` (label: `{label}`) found. First defined at line {first_line} (label: `{first_label}`)."
                    ),
                )),
                None => {
                    seen.insert(identifier, (line, label));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateDefinitions;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_duplicates() {
        let page = "[mercury]: https://example.com/mercury/\n[mercury]: https://example.com/venus/\n[earth]: https://example.com/earth/\n[Earth]: https://example.com/mars/\n\n[^mercury]: Hello, Mercury!\n\n[^mercury]: Hello, Venus!\n\n[^earth]: Hello, Earth!\n\n[^Earth]: Hello, Mars!\n";

        assert_eq!(
            places(&NoDuplicateDefinitions, page),
            [
                (2, 1, "Unexpected duplicate definition `mercury` (label: `mercury`) found. First defined at line 1 (label: `mercury`).".to_owned()),
                (4, 1, "Unexpected duplicate definition `earth` (label: `Earth`) found. First defined at line 3 (label: `earth`).".to_owned()),
                (8, 1, "Unexpected duplicate footnote definition `mercury` (label: `mercury`) found. First defined at line 6 (label: `mercury`).".to_owned()),
                (12, 1, "Unexpected duplicate footnote definition `earth` (label: `Earth`) found. First defined at line 10 (label: `earth`).".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_distinct_definitions_and_repeated_comments() {
        let page = "[mercury]: https://example.com/mercury/\n[venus]: https://example.com/venus/\n\n[//]: # (This is a comment 1)\n[//]: <> (This is a comment 2)\n";

        assert_eq!(messages(&NoDuplicateDefinitions, page), NONE);
    }
}
