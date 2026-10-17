//! One Markdown page, read and parsed once (EN-077): its text, its tree (mdast, from
//! `markdown-rs`, the Rust sibling of the parser `@eslint/markdown` reads pages with,
//! `REF-MARKDOWN-RS`) and the places of its offsets. The rules read the tree's nodes and the
//! source text under them, as `@eslint/markdown`'s rules do.

use markdown::mdast::Node;
use markdown::{Constructs, ParseOptions};

use crate::finding::{Finding, Fix};

/// A parsed page.
pub struct Page<'s> {
    /// Its path from the root, `/`-separated.
    pub path: &'s str,
    /// Its text.
    pub text: &'s str,
    /// Its tree.
    pub root: Node,
    /// The offset of each line's start.
    lines: Vec<usize>,
}

/// GitHub-flavored Markdown with YAML front matter, as `@eslint/markdown`'s `markdown/gfm`
/// language with `frontmatter: "yaml"` reads a page.
#[must_use]
pub fn options() -> ParseOptions {
    ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Constructs::gfm()
        },
        ..ParseOptions::gfm()
    }
}

impl<'s> Page<'s> {
    /// A page parsed; the message of the parser when it cannot read it.
    ///
    /// # Errors
    ///
    /// When the parser refuses the text (only MDX can fail, which these options leave off).
    pub fn parse(path: &'s str, text: &'s str) -> Result<Self, String> {
        let root = markdown::to_mdast(text, &options()).map_err(|message| message.to_string())?;
        let lines = std::iter::once(0)
            .chain(text.match_indices('\n').map(|(at, _)| at + 1))
            .collect();

        Ok(Self {
            path,
            text,
            root,
            lines,
        })
    }

    /// The line and the column of a byte offset, from 1, the column in UTF-16 units as `ESLint`
    /// counts them.
    #[must_use]
    pub fn place(&self, offset: usize) -> (usize, usize) {
        let line = self.lines.partition_point(|start| *start <= offset);
        let start = self.lines[line - 1];
        let end = offset.min(self.text.len());
        let column = self
            .text
            .get(start..end)
            .map_or(0, |before| before.encode_utf16().count());

        (line, column + 1)
    }

    /// A finding of a rule at a byte offset.
    #[must_use]
    pub fn finding(&self, rule: &'static str, offset: usize, message: String) -> Finding {
        let (line, column) = self.place(offset);

        Finding {
            path: self.path.to_owned(),
            line,
            column,
            rule,
            message,
            fix: None,
        }
    }

    /// A finding with the text that replaces a byte range to fix it.
    #[must_use]
    pub fn fixed(
        &self,
        rule: &'static str,
        offset: usize,
        message: String,
        range: (usize, usize),
        text: String,
    ) -> Finding {
        Finding {
            fix: Some(Fix {
                start: u32::try_from(range.0).unwrap_or(u32::MAX),
                end: u32::try_from(range.1).unwrap_or(u32::MAX),
                text,
            }),
            ..self.finding(rule, offset, message)
        }
    }

    /// The source text of a node.
    #[must_use]
    pub fn source(&self, node: &Node) -> &'s str {
        let (start, end) = range(node);

        self.text.get(start..end).unwrap_or_default()
    }

    /// The text of the line an offset is on, from that offset to the line's end.
    #[must_use]
    pub fn rest_of_line(&self, offset: usize) -> &'s str {
        let rest = self.text.get(offset..).unwrap_or_default();

        rest.split(['\n', '\r']).next().unwrap_or_default()
    }
}

/// A node's byte range in the page.
#[must_use]
pub fn range(node: &Node) -> (usize, usize) {
    node.position().map_or((0, 0), |position| {
        (position.start.offset, position.end.offset)
    })
}

/// A node's first byte.
#[must_use]
pub fn start(node: &Node) -> usize {
    range(node).0
}

/// A node's children, none for a leaf.
#[must_use]
pub fn children(node: &Node) -> &[Node] {
    node.children().map_or(&[], Vec::as_slice)
}

/// Walks a tree in document order, calling `visit` on entering each node (`true`) and on leaving
/// it (`false`), as `ESLint` walks a tree with its `node` and `node:exit` selectors.
pub fn walk<'n>(node: &'n Node, visit: &mut impl FnMut(&'n Node, bool)) {
    visit(node, true);
    for child in children(node) {
        walk(child, visit);
    }
    visit(node, false);
}

/// Every node of a tree, in document order.
#[must_use]
pub fn nodes(root: &Node) -> Vec<&Node> {
    let mut found = Vec::new();
    walk(root, &mut |node, entering| {
        if entering {
            found.push(node);
        }
    });
    found
}

/// The descendants of a node, in document order, the node left out.
#[must_use]
pub fn descendants(node: &Node) -> Vec<&Node> {
    children(node).iter().flat_map(nodes).collect()
}

/// A node's type as mdast names it (`inlineCode`, `tableCell`…), which `@eslint/markdown`'s
/// selectors and messages use.
#[must_use]
pub const fn type_name(node: &Node) -> &'static str {
    match node {
        Node::Root(_) => "root",
        Node::Blockquote(_) => "blockquote",
        Node::FootnoteDefinition(_) => "footnoteDefinition",
        Node::MdxJsxFlowElement(_) => "mdxJsxFlowElement",
        Node::List(_) => "list",
        Node::MdxjsEsm(_) => "mdxjsEsm",
        Node::Toml(_) => "toml",
        Node::Yaml(_) => "yaml",
        Node::Break(_) => "break",
        Node::InlineCode(_) => "inlineCode",
        Node::InlineMath(_) => "inlineMath",
        Node::Delete(_) => "delete",
        Node::Emphasis(_) => "emphasis",
        Node::MdxTextExpression(_) => "mdxTextExpression",
        Node::FootnoteReference(_) => "footnoteReference",
        Node::Html(_) => "html",
        Node::Image(_) => "image",
        Node::ImageReference(_) => "imageReference",
        Node::MdxJsxTextElement(_) => "mdxJsxTextElement",
        Node::Link(_) => "link",
        Node::LinkReference(_) => "linkReference",
        Node::Strong(_) => "strong",
        Node::Text(_) => "text",
        Node::Code(_) => "code",
        Node::Math(_) => "math",
        Node::MdxFlowExpression(_) => "mdxFlowExpression",
        Node::Heading(_) => "heading",
        Node::Table(_) => "table",
        Node::ThematicBreak(_) => "thematicBreak",
        Node::TableRow(_) => "tableRow",
        Node::TableCell(_) => "tableCell",
        Node::ListItem(_) => "listItem",
        Node::Definition(_) => "definition",
        Node::Paragraph(_) => "paragraph",
    }
}

/// Whether a node holds a run of inline content the text rules read: a heading, a paragraph or a
/// table cell.
#[must_use]
pub const fn is_inline_container(node: &Node) -> bool {
    matches!(
        node,
        Node::Heading(_) | Node::Paragraph(_) | Node::TableCell(_)
    )
}

#[cfg(test)]
mod tests {
    use markdown::mdast::Node;

    use super::{Page, descendants, nodes, range, type_name};

    fn kinds<'p>(page: &'p Page) -> Vec<&'p str> {
        nodes(&page.root).into_iter().map(type_name).collect()
    }

    #[test]
    fn reads_front_matter_headings_links_code_and_tables() {
        let text = "---\ntitle: A\n---\n\n# Title\n\nSee [a](#b) and https://x.org.\n\n```ts\nconst a = 1;\n```\n\n| a | b |\n| - | - |\n| 1 | 2 | 3 |\n\n[c]: https://c.org\n";
        let page = Page::parse("a.md", text).unwrap_or_else(|error| panic!("{error}"));

        assert_eq!(
            kinds(&page),
            [
                "root",
                "yaml",
                "heading",
                "text",
                "paragraph",
                "text",
                "link",
                "text",
                "text",
                "link",
                "text",
                "text",
                "code",
                "table",
                "tableRow",
                "tableCell",
                "text",
                "tableCell",
                "text",
                "tableRow",
                "tableCell",
                "text",
                "tableCell",
                "text",
                "tableCell",
                "text",
                "definition",
            ]
        );
        // The third cell of the second row is kept, as mdast keeps it: `| 1 | 2 | 3 |` holds
        // three cells.
        let table = nodes(&page.root)
            .into_iter()
            .find(|node| matches!(node, Node::Table(_)));
        let rows: Vec<usize> = table
            .map(|table| {
                super::children(table)
                    .iter()
                    .map(|row| super::children(row).len())
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(rows, [2, 3]);
    }

    #[test]
    fn places_offsets_in_bytes_and_columns_in_utf16_units() {
        let text = "é𝄞 [a](b)\n";
        let page = Page::parse("a.md", text).unwrap_or_else(|error| panic!("{error}"));
        let link = descendants(&page.root)
            .into_iter()
            .find(|node| matches!(node, Node::Link(_)));

        // `é` is 2 bytes, `𝄞` 4, the space 1: the link starts at byte 7; in UTF-16 units `é`
        // is 1 and `𝄞` 2, so its column is 1 + 1 + 2 + 1 = 5.
        let start = link.map(|node| range(node).0);
        assert_eq!(start, Some(7));
        assert_eq!(page.place(7), (1, 5));
        assert_eq!(page.place(text.len()), (2, 1));
    }

    #[test]
    fn reads_the_source_of_a_node_and_the_rest_of_its_line() {
        let page = Page::parse("a.md", "a *b*\r\nc\n").unwrap_or_else(|error| panic!("{error}"));
        let emphasis = descendants(&page.root)
            .into_iter()
            .find(|node| matches!(node, Node::Emphasis(_)));

        assert_eq!(emphasis.map(|node| page.source(node)), Some("*b*"));
        assert_eq!(page.rest_of_line(2), "*b*");
    }
}
