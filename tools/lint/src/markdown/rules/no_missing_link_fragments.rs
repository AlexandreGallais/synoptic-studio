//! `markdown/no-missing-link-fragments`: a link to `#fragment` on the same page reaches a heading
//! (by GitHub's anchor), a custom `{#id}`, an HTML `id` or `name`, an HTML heading, `#top`, or a
//! line of the file (`#L12`); compared without case.

use std::collections::HashSet;

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::javascript::is_js_space;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, start, value, walk};
use crate::markdown::slugger::Slugger;
use crate::markdown::text::{find_without_case, strip_html_comments, without_tags};

/// The rule.
pub struct NoMissingLinkFragments;

/// The custom id that ends a heading's text, `\{#(?<id>[^}\s]+)\}\s*$`.
fn custom_id(text: &str) -> Option<&str> {
    let trimmed = text.trim_end_matches(is_js_space);
    let inner = trimmed.strip_suffix('}')?;
    let mut at = 0;

    while let Some(open) = inner[at..].find("{#").map(|found| at + found) {
        let id = &inner[open + 2..];
        if !id.is_empty() && !id.contains('}') && !id.chars().any(is_js_space) {
            return Some(id);
        }
        at = open + 1;
    }
    None
}

/// The `id` or `name` of each HTML tag of a text, the last of a tag:
/// `(?<!<)<[^>]+\s(?:id|name)\s*=\s*["']?(?<id>[^"'\s>]+)["']?`, without case.
fn html_ids(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;

    while let Some(open) = text[at..].find('<').map(|found| at + found) {
        at = open + 1;
        if open > 0 && bytes[open - 1] == b'<' {
            continue;
        }
        let tag_end = text[open + 1..]
            .find('>')
            .map_or(text.len(), |end| open + 1 + end);
        // The last white space inside the tag, from which `id=` or `name=` reads a value.
        let candidates = (open + 2..tag_end)
            .rev()
            .filter(|space| text[*space..].chars().next().is_some_and(is_js_space));
        for space in candidates {
            if let Some((id, end)) = attribute_value(text, space + 1) {
                found.push(id);
                at = end;
                break;
            }
        }
    }
    found
}

/// The value of an `id` or `name` attribute starting at an index, and the end of the match.
fn attribute_value(text: &str, from: usize) -> Option<(&str, usize)> {
    let bytes = text.as_bytes();
    let name = ["id", "name"]
        .into_iter()
        .find(|name| find_without_case(text, name, from) == Some(from))?;
    let skip_spaces = |mut at: usize| {
        while text[at..].chars().next().is_some_and(is_js_space) {
            at += text[at..].chars().next().map_or(1, char::len_utf8);
        }
        at
    };
    let mut at = skip_spaces(from + name.len());
    if bytes.get(at) != Some(&b'=') {
        return None;
    }
    at = skip_spaces(at + 1);
    if matches!(bytes.get(at), Some(b'"' | b'\'')) {
        at += 1;
    }
    let length: usize = text[at..]
        .chars()
        .take_while(|character| !matches!(character, '"' | '\'' | '>') && !is_js_space(*character))
        .map(char::len_utf8)
        .sum();
    if length == 0 {
        return None;
    }
    let end = at + length;
    let close = usize::from(matches!(bytes.get(end), Some(b'"' | b'\'')));
    Some((&text[at..end], end + close))
}

/// The content of each HTML heading of a text, `<h([1-6])[^>]*>([\s\S]*?)<\/h\1\s*>`, without
/// case.
fn html_headings(text: &str) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;

    while let Some(open) = find_without_case(text, "<h", at) {
        at = open + 1;
        let Some(depth) = bytes
            .get(open + 2)
            .filter(|depth| (b'1'..=b'6').contains(depth))
        else {
            continue;
        };
        let Some(content_start) = text[open + 3..].find('>').map(|end| open + 3 + end + 1) else {
            continue;
        };
        let closing = format!("</h{}", char::from(*depth));
        let mut search = content_start;
        while let Some(close) = find_without_case(text, &closing, search) {
            let after = close + closing.len();
            let spaces: usize = text[after..]
                .chars()
                .take_while(|character| is_js_space(*character))
                .map(char::len_utf8)
                .sum();
            if bytes.get(after + spaces) == Some(&b'>') {
                found.push(&text[content_start..close]);
                at = after + spaces + 1;
                break;
            }
            search = close + 1;
        }
    }
    found
}

/// A fragment decoded as `decodeURIComponent` does; the fragment itself when it cannot be.
pub(crate) fn decode(fragment: &str) -> String {
    let bytes = fragment.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut at = 0;

    while at < bytes.len() {
        if bytes[at] == b'%' {
            let byte = fragment
                .get(at + 1..at + 3)
                .and_then(|hex| u8::from_str_radix(hex, 16).ok());
            match byte {
                Some(byte) => {
                    decoded.push(byte);
                    at += 3;
                }
                None => return fragment.to_owned(),
            }
        } else {
            decoded.push(bytes[at]);
            at += 1;
        }
    }
    String::from_utf8(decoded).unwrap_or_else(|_| fragment.to_owned())
}

/// Whether a fragment names lines of a file as GitHub does: `^L\d+(?:C\d+)?(?:-L\d+(?:C\d+)?)?$`.
pub(crate) fn is_line_reference(fragment: &str) -> bool {
    let line = |text: &str| -> bool {
        let Some(rest) = text.strip_prefix('L') else {
            return false;
        };
        let (row, column) = rest.split_once('C').unwrap_or((rest, "1"));
        let digits =
            |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
        digits(row) && digits(column)
    };
    let (first, last) = fragment.split_once('-').unwrap_or((fragment, "L1"));
    line(first) && line(last)
}

impl MarkdownRule for NoMissingLinkFragments {
    fn name(&self) -> &'static str {
        "markdown/no-missing-link-fragments"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let mut ids: HashSet<String> = page_ids(&page.root).into_iter().map(|id| id.id).collect();
        let mut links: Vec<(&Node, &str)> = Vec::new();

        ids.insert("top".to_owned());
        walk(&page.root, &mut |node, entering| match (node, entering) {
            (Node::Link(link), true) => links.push((node, &link.url)),
            (Node::Definition(definition), true) => links.push((node, &definition.url)),
            _ => {}
        });

        for (node, url) in links {
            let Some(fragment) = url
                .strip_prefix('#')
                .filter(|fragment| !fragment.is_empty())
            else {
                continue;
            };
            let decoded = decode(fragment);
            if !is_line_reference(&decoded) && !ids.contains(&decoded.to_lowercase()) {
                findings.push(page.finding(
                    self.name(),
                    start(node),
                    format!(
                        "Link fragment '#{fragment}' does not reference a heading or anchor in this document."
                    ),
                ));
            }
        }
    }
}

/// One id of a page a link's fragment may name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageId {
    /// The id, as GitHub's slugger gives it.
    pub id: String,
    /// The text of the Markdown heading it is the anchor of, `None` for a heading's custom
    /// `{#id}`, an HTML `id` or `name`, or an HTML heading.
    pub heading: Option<String>,
}

/// The ids of a page in its order, one slugger numbering them all: each heading's anchor, from its
/// custom `{#id}` when it ends with one, given when the heading is left, after the HTML inside it;
/// each HTML `id` or `name` and each HTML heading. Read by this rule and by `local/anchor-slugs`
/// on another page (AUD-035, F5).
#[must_use]
pub fn page_ids(root: &Node) -> Vec<PageId> {
    let mut ids = Vec::new();
    let mut slugger = Slugger::default();
    let mut add = |text: &str, heading: Option<String>| {
        ids.push(PageId {
            id: slugger.slug(text),
            heading,
        });
    };

    walk(root, &mut |node, entering| match (node, entering) {
        (Node::Heading(_), false) => {
            let text = heading_text(node);

            match custom_id(&text) {
                Some(id) => add(id, None),
                None => add(&text, Some(text.clone())),
            }
        }
        (Node::Html(html), true) => {
            let text = strip_html_comments(&html.value);

            for id in html_ids(&text) {
                add(id, None);
            }
            for content in html_headings(&text) {
                add(&without_tags(content), None);
            }
        }
        _ => {}
    });
    ids
}

/// The text of a heading for its anchor: the values of its descendants, HTML left out.
#[must_use]
pub fn heading_text(heading: &Node) -> String {
    crate::markdown::page::descendants(heading)
        .into_iter()
        .filter(|node| !matches!(node, Node::Html(_)))
        .filter_map(value)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{NoMissingLinkFragments, decode, is_line_reference};
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_missing_fragments() {
        let page = "[Invalid Link](#non-existent-heading)\n\n# Some Heading\n\n[Case Mismatch](#other-heading)\n\n[a]: #nowhere\n";

        assert_eq!(
            places(&NoMissingLinkFragments, page),
            [
                (1, 1, "Link fragment '#non-existent-heading' does not reference a heading or anchor in this document.".to_owned()),
                (5, 1, "Link fragment '#other-heading' does not reference a heading or anchor in this document.".to_owned()),
                (7, 1, "Link fragment '#nowhere' does not reference a heading or anchor in this document.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_the_documented_fragments() {
        let page = "# Introduction\n\n[Valid Link](#introduction) [Case](#INTRODUCTION)\n\n# Another Section {#custom-id}\n\n[Link to custom ID](#custom-id)\n\n<h1 id=\"html-anchor\">HTML Anchor</h1>\n\n[Link to HTML anchor](#html-anchor)\n\n<a name=\"named-anchor\">Named Anchor</a>\n\n[Link to named anchor](#named-anchor)\n\n[Link to top of page](#top)\n\n[Link](#L2) [Lines](#L2C3-L4)\n\n# café\n\n[Link to café](#caf%C3%A9)\n\n# <span class=\"icon-star\"></span> Starred Projects\n\n[Link to starred projects](#-starred-projects)\n\n## Introduction\n\n[Second](#introduction-1) [HTML heading](#html-anchor-1) [External](./a.md#x)\n";

        assert_eq!(messages(&NoMissingLinkFragments, page), NONE);
    }

    #[test]
    fn decodes_percent_escapes_and_reads_line_references() {
        assert_eq!(decode("caf%C3%A9"), "café");
        assert_eq!(decode("100%"), "100%");
        assert_eq!(decode("%FF"), "%FF");
        assert!(is_line_reference("L12"));
        assert!(is_line_reference("L1C2-L3C4"));
        assert!(!is_line_reference("L"));
        assert!(!is_line_reference("intro"));
    }
}
