//! `markdown/fenced-code-meta`, mode `never`: a fence's info string holds its language only, no
//! title nor line highlights, which GitHub and the site would read differently.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};

/// The rule.
pub struct FencedCodeMeta;

impl MarkdownRule for FencedCodeMeta {
    fn name(&self) -> &'static str {
        "markdown/fenced-code-meta"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            let Node::Code(code) = node else { continue };
            let Some(meta) = code.meta.as_deref().filter(|meta| !meta.is_empty()) else {
                continue;
            };
            let at = start(node);
            let index = page.rest_of_line(at).rfind(meta).unwrap_or_default();
            findings.push(page.finding(
                self.name(),
                at + index,
                "Code block metadata is not allowed.".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FencedCodeMeta;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_metadata_at_its_place() {
        // The documented case: `title="example.js"` starts after "```js ", column 7.
        assert_eq!(
            places(
                &FencedCodeMeta,
                "```js title=\"example.js\"\nconsole.log(1);\n```\n\n> ~~~ts {1,3}\n> a\n> ~~~\n"
            ),
            [
                (1, 7, "Code block metadata is not allowed.".to_owned()),
                (5, 9, "Code block metadata is not allowed.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_a_language_alone() {
        assert_eq!(
            messages(
                &FencedCodeMeta,
                "```js\nconsole.log(1);\n```\n\n```\na\n```\n"
            ),
            NONE
        );
    }
}
