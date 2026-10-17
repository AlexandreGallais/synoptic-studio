//! `markdown/no-missing-atx-heading-space`: a line of a paragraph that starts with one to six `#`
//! and no space after them was meant as a heading, which needs the space. Autofixed.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};

/// The rule.
pub struct NoMissingAtxHeadingSpace;

impl MarkdownRule for NoMissingAtxHeadingSpace {
    fn name(&self) -> &'static str {
        "markdown/no-missing-atx-heading-space"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            if !matches!(node, Node::Paragraph(_)) {
                continue;
            }
            let text = page.source(node);
            let bytes = text.as_bytes();
            // `(?:^|(?<=[\r\n]))(?<hashes>#{1,6})(?:[^# \t]|$)`: each line's start.
            let line_starts = std::iter::once(0).chain(
                bytes
                    .iter()
                    .enumerate()
                    .filter(|(_, byte)| matches!(byte, b'\n' | b'\r'))
                    .map(|(at, _)| at + 1),
            );
            for line_start in line_starts {
                let hashes = bytes[line_start..]
                    .iter()
                    .take_while(|byte| **byte == b'#')
                    .count();
                let next = bytes.get(line_start + hashes);
                // A space or a tab after the hashes makes a heading, never a paragraph's line:
                // kept as the pattern reads it. The hashes are counted whole, so no `#` follows.
                if (1..=6).contains(&hashes) && !matches!(next, Some(b' ' | b'\t')) {
                    let at = start(node) + line_start;
                    findings.push(page.fixed(
                        self.name(),
                        at,
                        "Missing space after hash(es) on ATX style heading.".into(),
                        (at + hashes, at + hashes),
                        " ".into(),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoMissingAtxHeadingSpace;
    use crate::markdown::testing::{fixed, messages, places};
    use crate::testing::NONE;

    const MESSAGE: &str = "Missing space after hash(es) on ATX style heading.";

    #[test]
    fn reports_and_fixes_a_heading_without_its_space() {
        // The documented case, then a paragraph's second line, and seven hashes, never a heading.
        let page = "#Heading 1\n\nText\n###Heading 3\n\n#######Seven\n";

        assert_eq!(
            places(&NoMissingAtxHeadingSpace, page),
            [(1, 1, MESSAGE.to_owned()), (4, 1, MESSAGE.to_owned())]
        );
        assert_eq!(
            fixed(&NoMissingAtxHeadingSpace, page),
            "# Heading 1\n\nText\n### Heading 3\n\n#######Seven\n"
        );
    }

    #[test]
    fn accepts_headings_hashtags_and_html() {
        let page = "# Heading 1\n\n<h1>#Some Heading</h1>\n\n[#123](link.com)\n\n![#alttext][link.png]\n\nThis is a paragraph with a #hashtag, not a heading.\n";

        assert_eq!(messages(&NoMissingAtxHeadingSpace, page), NONE);
    }
}
