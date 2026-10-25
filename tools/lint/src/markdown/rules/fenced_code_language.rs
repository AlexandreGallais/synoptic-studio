//! `markdown/fenced-code-language`: a fenced code block names its language, so that the site and
//! GitHub highlight it (the records' conventions, area writing: "always with a language").

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};

/// The rule.
pub struct FencedCodeLanguage;

impl MarkdownRule for FencedCodeLanguage {
    fn name(&self) -> &'static str {
        "markdown/fenced-code-language"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            let Node::Code(code) = node else { continue };
            let at = start(node);
            let fenced = matches!(page.text.as_bytes().get(at), Some(b'`' | b'~'));
            if code.lang.is_none() && fenced {
                findings.push(page.finding(self.name(), at, "Missing code block language.".into()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FencedCodeLanguage;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_a_fenced_block_without_a_language() {
        // The documented case, then a tilde fence in a list item: its fence opens at column 3.
        assert_eq!(
            places(
                &FencedCodeLanguage,
                "```\nconst a = 1;\n```\n\n- ~~~\n  b\n  ~~~\n"
            ),
            [
                (1, 1, "Missing code block language.".to_owned()),
                (5, 3, "Missing code block language.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_a_language_and_an_indented_block() {
        assert_eq!(
            messages(
                &FencedCodeLanguage,
                "```ts\nconst a = 1;\n```\n\n    indented\n"
            ),
            NONE
        );
    }
}
