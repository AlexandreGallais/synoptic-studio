//! `markdown/no-invalid-label-refs`: no white space alone between the brackets of a reference's
//! label, `[text][ ]`, which no parser reads as a link.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};

/// The rule.
pub struct NoInvalidLabelRefs;

/// The label of the first `[…]` holding something in a text: `!?\[([^\]]+)\]`.
fn first_label(text: &str) -> Option<&str> {
    let mut at = 0;
    while let Some(open) = text[at..].find('[').map(|found| at + found) {
        let inner = &text[open + 1..];
        let length = inner.find(']')?;
        if length > 0 {
            return Some(&inner[..length]);
        }
        at = open + 1;
    }
    None
}

impl MarkdownRule for NoInvalidLabelRefs {
    fn name(&self) -> &'static str {
        "markdown/no-invalid-label-refs"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            if !matches!(node, Node::Text(_)) {
                continue;
            }
            let text = page.source(node);
            let mut at = 0;
            // `\]\[([^\]]+)\]`: a `][`, something, then `]`.
            while let Some(found) = text[at..].find("][").map(|found| at + found) {
                let Some(length) = text[found + 2..].find(']').filter(|length| *length > 0) else {
                    at = found + 1;
                    continue;
                };
                let end = found + 2 + length + 1;
                let blank = text[found + 2..end - 1]
                    .chars()
                    .all(crate::markdown::text::is_js_space);
                if blank {
                    let offset = start(node) + found;
                    let label = page.text[..offset]
                        .rfind('[')
                        .and_then(|open| first_label(&page.text[open..start(node) + end]));
                    if let Some(label) = label {
                        findings.push(page.finding(
                            self.name(),
                            offset + 1,
                            format!(
                                "Label reference '{}' is invalid due to white space between [ and ].",
                                label.trim()
                            ),
                        ));
                    }
                }
                at = end;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidLabelRefs;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_invalid_references() {
        assert_eq!(
            places(
                &NoInvalidLabelRefs,
                "[eslint][ ]\n\n[eslint][\t]\n\n![eslint][ ] and [a] [b][ ]\n\n[ spaced ][ ]\n"
            ),
            [
                (
                    1,
                    9,
                    "Label reference 'eslint' is invalid due to white space between [ and ]."
                        .to_owned()
                ),
                (
                    3,
                    9,
                    "Label reference 'eslint' is invalid due to white space between [ and ]."
                        .to_owned()
                ),
                (
                    5,
                    10,
                    "Label reference 'eslint' is invalid due to white space between [ and ]."
                        .to_owned()
                ),
                (
                    5,
                    25,
                    "Label reference 'b' is invalid due to white space between [ and ].".to_owned()
                ),
                (
                    7,
                    11,
                    "Label reference 'spaced' is invalid due to white space between [ and ]."
                        .to_owned()
                ),
            ]
        );
    }

    #[test]
    fn accepts_full_collapsed_and_shortcut_references() {
        assert_eq!(
            messages(
                &NoInvalidLabelRefs,
                "[eslint][eslint] [eslint][] [eslint]\n\n[eslint]: https://eslint.org/\n"
            ),
            NONE
        );
    }
}
