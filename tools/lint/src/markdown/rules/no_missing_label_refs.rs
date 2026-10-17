//! `markdown/no-missing-label-refs`: a reference `[text][label]`, `[label][]` or `[label]` has
//! its definition; otherwise it renders as bracketed text, not a link. GitHub's alerts
//! (`[!NOTE]`…) and the bibliography's status markers (`[verified]`, `[unverified]`) are allowed.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::text::{is_js_space, is_line_end, is_unescaped};

/// The rule.
pub struct NoMissingLabelRefs;

/// The labels that need no definition.
pub const ALLOWED_LABELS: [&str; 7] = [
    "!CAUTION",
    "!IMPORTANT",
    "!NOTE",
    "!TIP",
    "!WARNING",
    "unverified",
    "verified",
];

/// The end of a bracketed label's content from an index, at its `]`: escapes `\x` (no line end
/// after the backslash), and any other character but `]`, and `[` too unless `open_allowed` is
/// false; `None` when the text ends first.
fn label_end(text: &str, from: usize, open_allowed: bool) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut at = from;

    loop {
        match *bytes.get(at)? {
            b']' => return Some(at),
            b'\\' => {
                let next = text[at + 1..].chars().next()?;
                if is_line_end(bytes[at + 1]) || matches!(next, '\u{2028}' | '\u{2029}') {
                    return None;
                }
                at += 1 + next.len_utf8();
            }
            b'[' if !open_allowed => return None,
            _ => at += text[at..].chars().next().map_or(1, char::len_utf8),
        }
    }
}

/// The labels of a text's bracketed references, with their offsets in the text: the pattern
/// `\[(?<left>…)\](?:\[(?<right>…)\])?` of `@eslint/markdown`, a `[` no backslash escapes.
fn labels(text: &str) -> Vec<(usize, &str)> {
    let mut found = Vec::new();
    let bytes = text.as_bytes();
    let mut at = 0;

    while let Some(open) = text[at..].find('[').map(|found| at + found) {
        let Some(left_end) = is_unescaped(bytes, open)
            .then(|| label_end(text, open + 1, false))
            .flatten()
        else {
            at = open + 1;
            continue;
        };
        let right = (bytes.get(left_end + 1) == Some(&b'['))
            .then(|| label_end(text, left_end + 2, true))
            .flatten()
            .map(|right_end| (left_end + 2, right_end));
        let end = right.map_or(left_end + 1, |(_, right_end)| right_end + 1);
        at = end;

        let left = &text[open + 1..left_end];
        let right_text = right.map_or("", |(from, to)| &text[from..to]);
        if !right_text.is_empty() && right_text.chars().all(is_js_space) {
            continue;
        }
        if !right_text.is_empty() {
            found.push((left_end + 2, right_text));
        } else if !left.is_empty() {
            found.push((open + 1, left));
        }
    }
    found
}

impl MarkdownRule for NoMissingLabelRefs {
    fn name(&self) -> &'static str {
        "markdown/no-missing-label-refs"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let mut missing: Vec<(usize, String)> = Vec::new();

        for node in nodes(&page.root) {
            match node {
                Node::Text(_) => {
                    for (offset, label) in labels(page.source(node)) {
                        let label = label.trim_matches(is_js_space);
                        if !ALLOWED_LABELS.contains(&label) {
                            missing.push((start(node) + offset, label.to_owned()));
                        }
                    }
                }
                Node::Definition(definition) => {
                    missing.retain(|(_, label)| *label != definition.identifier);
                }
                _ => {}
            }
        }
        for (offset, label) in missing {
            findings.push(page.finding(
                self.name(),
                offset,
                format!("Label reference '{label}' not found."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoMissingLabelRefs;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_missing_references() {
        let page = "[ESLint][eslint]\n\n[eslint][]\n\n[eslint]\n\n[ spaced ] and \\[escaped] and \\\\[two]\n";

        assert_eq!(
            places(&NoMissingLabelRefs, page),
            [
                (1, 10, "Label reference 'eslint' not found.".to_owned()),
                (3, 2, "Label reference 'eslint' not found.".to_owned()),
                (5, 2, "Label reference 'eslint' not found.".to_owned()),
                (7, 2, "Label reference 'spaced' not found.".to_owned()),
                (7, 34, "Label reference 'two' not found.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_defined_references_alerts_and_status_markers() {
        let page = "[ESLint][eslint] [eslint][] [eslint]\n\n> [!NOTE]\n> A note.\n\n| `REF-A` | x | [verified] 2026-10-17 |\n| - | - | - |\n\n[eslint]: https://eslint.org/\n\n[] and [x][ ]\n";

        assert_eq!(messages(&NoMissingLabelRefs, page), NONE);
    }
}
