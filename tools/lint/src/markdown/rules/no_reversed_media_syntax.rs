//! `markdown/no-reversed-media-syntax`: a link or an image is `[label](URL)`, never the reversed
//! `(label)[URL]`, which renders as text. Autofixed.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, descendants, is_inline_container, nodes, range};
use crate::markdown::text::{is_line_end, is_unescaped};

/// The rule.
pub struct NoReversedMediaSyntax;

/// A container's text with its HTML, images, references and code turned to spaces, so that
/// brackets inside them are not read.
fn masked(page: &Page, container: &Node) -> Vec<u8> {
    let (from, _) = range(container);
    let mut bytes = page.source(container).as_bytes().to_vec();

    for node in descendants(container) {
        if matches!(
            node,
            Node::Html(_)
                | Node::Image(_)
                | Node::ImageReference(_)
                | Node::InlineCode(_)
                | Node::LinkReference(_)
                | Node::InlineMath(_)
        ) {
            let (start, end) = range(node);
            for byte in &mut bytes[start - from..end - from] {
                *byte = b' ';
            }
        }
    }
    bytes
}

/// Whether a label is one the pattern reads: `(?:\\.|[^()\\]|\([\s\S]*\))*`.
fn is_label(bytes: &[u8]) -> bool {
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' if bytes.get(at + 1).is_some_and(|next| !is_line_end(*next)) => at += 2,
            b'\\' | b')' => return false,
            b'(' => match bytes[at + 1..].iter().rposition(|byte| *byte == b')') {
                Some(close) => at += close + 2,
                None => return false,
            },
            _ => at += 1,
        }
    }
    true
}

/// The end of a reversed link's URL and its `]`, from the `[`: `\[(?:\\.|[^\]\\\r\n])*\]`.
fn url_end(bytes: &[u8], open: usize) -> Option<usize> {
    let mut at = open + 1;
    loop {
        match *bytes.get(at)? {
            b']' => return Some(at),
            b'\\' if bytes.get(at + 1).is_some_and(|next| !is_line_end(*next)) => at += 2,
            b'\\' | b'\n' | b'\r' => return None,
            _ => at += 1,
        }
    }
}

/// The reversed links of a masked text: each start, label end and match end.
fn reversed(bytes: &[u8]) -> Vec<(usize, usize, usize)> {
    let mut found = Vec::new();
    let mut at = 0;

    while let Some(open) = bytes[at..]
        .iter()
        .position(|byte| *byte == b'(')
        .map(|found| at + found)
    {
        at = open + 1;
        if !is_unescaped(bytes, open) {
            continue;
        }
        // The longest label whose `)[URL]` follows, not followed by `(`.
        let matched = (open + 1..bytes.len()).rev().find_map(|close| {
            let candidate = bytes[close] == b')'
                && bytes.get(close + 1) == Some(&b'[')
                && is_label(&bytes[open + 1..close]);
            let end = url_end(bytes, close + 1).filter(|_| candidate)?;
            (bytes.get(end + 1) != Some(&b'(')).then_some((close, end + 1))
        });
        if let Some((close, end)) = matched {
            found.push((open, close, end));
            at = end;
        }
    }
    found
}

impl MarkdownRule for NoReversedMediaSyntax {
    fn name(&self) -> &'static str {
        "markdown/no-reversed-media-syntax"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for container in nodes(&page.root)
            .into_iter()
            .filter(|node| is_inline_container(node))
        {
            let from = range(container).0;
            for (open, close, end) in reversed(&masked(page, container)) {
                let label = &page.text[from + open + 1..from + close];
                let url = &page.text[from + close + 2..from + end - 1];
                findings.push(page.fixed(
                    self.name(),
                    from + open,
                    "Unexpected reversed syntax found. Use [label](URL) syntax instead.".into(),
                    (from + open, from + end),
                    format!("[{label}]({url})"),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoReversedMediaSyntax;
    use crate::markdown::testing::{fixed, messages, places};
    use crate::testing::NONE;

    const MESSAGE: &str = "Unexpected reversed syntax found. Use [label](URL) syntax instead.";

    #[test]
    fn reports_and_fixes_the_documented_reversed_links() {
        let page = "(ESLint)[eslint.html]\n\n!(A beautiful sunset)[sunset.png]\n\n# (ESLint)[eslint.html]\n\n| ESLint | Sunset |\n| - | - |\n| (ESLint)[eslint.html] | !(see (x))[sunset.png] |\n\n(a)[b] and (c)[d]\n";

        assert_eq!(
            places(&NoReversedMediaSyntax, page),
            [
                (1, 1, MESSAGE.to_owned()),
                (3, 2, MESSAGE.to_owned()),
                (5, 3, MESSAGE.to_owned()),
                (9, 3, MESSAGE.to_owned()),
                (9, 28, MESSAGE.to_owned()),
                (11, 1, MESSAGE.to_owned()),
                (11, 12, MESSAGE.to_owned()),
            ]
        );
        assert_eq!(
            fixed(&NoReversedMediaSyntax, page),
            "[ESLint](eslint.html)\n\n![A beautiful sunset](sunset.png)\n\n# [ESLint](eslint.html)\n\n| ESLint | Sunset |\n| - | - |\n| [ESLint](eslint.html) | ![see (x)](sunset.png) |\n\n[a](b) and [c](d)\n"
        );
    }

    #[test]
    fn accepts_links_code_escapes_and_a_link_after_brackets() {
        let page = "[ESLint](eslint.html)\n\n![A beautiful sunset](sunset.png)\n\n`(a)[b]` and \\(a)[b] and (a)[b](c)\n";

        assert_eq!(messages(&NoReversedMediaSyntax, page), NONE);
    }
}
