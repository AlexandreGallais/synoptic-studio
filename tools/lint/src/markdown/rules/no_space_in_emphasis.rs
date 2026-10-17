//! `markdown/no-space-in-emphasis`: no space inside emphasis markers, `* text *` or `** text **`,
//! which renders the stars instead of the emphasis. Autofixed.

use std::collections::HashMap;

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, children, is_inline_container, nodes, range};
use crate::markdown::text::is_unescaped;

/// The rule.
pub struct NoSpaceInEmphasis;

/// The markers, longest first, as the pattern tries them: `***`, `**`, `*`, `___`, `__`, `_`.
const MARKERS: [&[u8]; 6] = [b"***", b"**", b"*", b"___", b"__", b"_"];

/// A container's text where only its own text children are kept, the rest turned to spaces.
fn masked(page: &Page, container: &Node) -> Vec<u8> {
    let (from, to) = range(container);
    let mut bytes = vec![b' '; to - from];

    for child in children(container) {
        if matches!(child, Node::Text(_)) {
            let (start, end) = range(child);
            bytes[start - from..end - from].copy_from_slice(&page.text.as_bytes()[start..end]);
        }
    }
    bytes
}

impl MarkdownRule for NoSpaceInEmphasis {
    fn name(&self) -> &'static str {
        "markdown/no-space-in-emphasis"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let text = page.text.as_bytes();
        let is_space = |at: usize| matches!(text.get(at), Some(b' ' | b'\t'));

        for container in nodes(&page.root)
            .into_iter()
            .filter(|node| is_inline_container(node))
        {
            let from = range(container).0;
            let bytes = masked(page, container);
            let mut groups: HashMap<&[u8], Vec<(usize, usize)>> = HashMap::new();
            let mut order: Vec<&[u8]> = Vec::new();
            let mut at = 0;
            while at < bytes.len() {
                let marker = MARKERS
                    .into_iter()
                    .find(|marker| bytes[at..].starts_with(marker))
                    .filter(|_| is_unescaped(&bytes, at));
                let Some(marker) = marker else {
                    at += 1;
                    continue;
                };
                if !groups.contains_key(marker) {
                    order.push(marker);
                }
                groups
                    .entry(marker)
                    .or_default()
                    .push((from + at, from + at + marker.len()));
                at += marker.len();
            }

            for marker in order {
                for [open, close] in groups[marker].as_chunks::<2>().0 {
                    for (checked, place) in [(open.1, open.0), (close.0 - 1, close.0 - 2)] {
                        if is_space(checked) {
                            findings.push(page.fixed(
                                self.name(),
                                place,
                                "Unexpected space around emphasis marker.".into(),
                                (checked, checked + 1),
                                String::new(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoSpaceInEmphasis;
    use crate::markdown::testing::{fixed, messages, places};
    use crate::testing::NONE;

    const MESSAGE: &str = "Unexpected space around emphasis marker.";

    #[test]
    fn reports_and_fixes_the_documented_spaces() {
        let page = "Here is some ** bold ** text.\n\nHere is some * italic * text.\n\nHere is some ___ bold italic ___ text.\n\nA product 2 * 3 and \\* a *.\n";

        // `** bold **`: the opening marker at column 14; the closing marker's space at column 20,
        // reported from the character before it, column 20 as `@eslint/markdown` reports it.
        assert_eq!(
            places(&NoSpaceInEmphasis, page),
            [
                (1, 14, MESSAGE.to_owned()),
                (1, 20, MESSAGE.to_owned()),
                (3, 14, MESSAGE.to_owned()),
                (3, 21, MESSAGE.to_owned()),
                (5, 14, MESSAGE.to_owned()),
                (5, 28, MESSAGE.to_owned()),
                // Two stars on a line read as a pair, as in `@eslint/markdown`; the escaped one
                // is left out.
                (7, 13, MESSAGE.to_owned()),
                (7, 24, MESSAGE.to_owned()),
            ]
        );
        assert_eq!(
            fixed(&NoSpaceInEmphasis, page),
            "Here is some **bold** text.\n\nHere is some *italic* text.\n\nHere is some ___bold italic___ text.\n\nA product 2 *3 and \\* a*.\n"
        );
    }

    #[test]
    fn accepts_the_documented_emphasis() {
        let page = "Here is some **bold** text.\n\nHere is some *italic* text.\n\nHere is some ***bold italic*** text.\n\nHere is some **_ bold _** text.\n\nA snake_case_name.\n";

        assert_eq!(messages(&NoSpaceInEmphasis, page), NONE);
    }
}
