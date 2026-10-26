//! The tree of a style sheet, as `postcss` 8.5.30 builds it with `postcss-scss` 4.0.9: a root, its
//! rules, at-rules, declarations and comments, each with where it starts and ends in the text and
//! the raw text around it (`raws`), which the rules read and the fixes rewrite. The nodes live in
//! one list, each knowing its parent and its children by index, so that a rule walks the tree
//! without recursion.

/// What a node is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The sheet.
    Root,
    /// A style rule: a selector and its block.
    Rule,
    /// An at-rule, `@name params` with or without a block.
    AtRule,
    /// A declaration, `prop: value`, a `$variable` among them; with a block when nested
    /// (`font: 12px { family: x }`).
    Declaration,
    /// A comment, `/* … */` or `// …`.
    Comment,
}

/// A node of the tree.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Node {
    /// What it is.
    pub kind: Option<Kind>,
    /// Its parent, none for the root.
    pub parent: Option<usize>,
    /// Its children, for a container; none for a node without a block.
    pub children: Option<Vec<usize>>,
    /// Its first byte.
    pub start: usize,
    /// The byte after its last, a declaration's `;` included; none while it is unclosed.
    pub end: Option<usize>,
    /// The spaces and comments before it.
    pub before: String,
    /// The text between its name and its value or block: `: ` of a declaration, the spaces
    /// before a rule's or an at-rule's `{`.
    pub between: String,
    /// The spaces before a container's `}` or the root's end.
    pub after: String,
    /// Whether a container's last declaration ends with `;`.
    pub semicolon: bool,
    /// The text of a rule's own `;` after its `}`, spaces before it included.
    pub own_semicolon: Option<String>,
    /// A rule's selector, without its comments.
    pub selector: String,
    /// An at-rule's name, without `@`.
    pub name: String,
    /// An at-rule's text after its name, without its comments.
    pub params: String,
    /// The spaces after an at-rule's name.
    pub after_name: String,
    /// A declaration's property.
    pub prop: String,
    /// A declaration's value, without its comments nor `!important`.
    pub value: String,
    /// Whether a declaration is `!important`.
    pub important: bool,
    /// The raw text of a declaration's `!important`, when not ` !important`.
    pub important_raw: Option<String>,
    /// The raw text of a selector, params or value holding comments, when it differs.
    pub raw: Option<String>,

    /// A comment's text, trimmed.
    pub text: String,
    /// The spaces before a comment's text.
    pub left: String,
    /// The spaces after a comment's text.
    pub right: String,
    /// Whether a comment is `//`.
    pub inline: bool,
}

impl Node {
    /// A node of a kind starting at a byte.
    #[must_use]
    pub fn new(kind: Kind, start: usize) -> Self {
        Self {
            kind: Some(kind),
            start,
            ..Self::default()
        }
    }

    /// Whether it is of a kind.
    #[must_use]
    pub fn is(&self, kind: Kind) -> bool {
        self.kind == Some(kind)
    }

    /// Whether it is a declaration holding a block of nested properties.
    #[must_use]
    pub fn is_nested(&self) -> bool {
        self.is(Kind::Declaration) && self.children.is_some()
    }

    /// Its children, none for a node without a block.
    #[must_use]
    pub fn nodes(&self) -> &[usize] {
        self.children.as_deref().unwrap_or_default()
    }
}

/// A sheet read: its nodes, the root first.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Sheet {
    /// Every node, the root at 0, each after its parent.
    pub nodes: Vec<Node>,
}

impl Sheet {
    /// A node by its index.
    #[must_use]
    pub fn node(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    /// The indexes of a node's ancestors, its parent first.
    #[must_use]
    pub fn ancestors(&self, index: usize) -> Vec<usize> {
        let mut found = Vec::new();
        let mut current = self.nodes[index].parent;
        while let Some(parent) = current {
            found.push(parent);
            current = self.nodes[parent].parent;
        }
        found
    }

    /// Every node under the root, in the order written.
    #[must_use]
    pub fn walk(&self) -> Vec<usize> {
        let mut found = Vec::with_capacity(self.nodes.len());
        let mut stack: Vec<usize> = self.nodes[0].nodes().iter().rev().copied().collect();
        while let Some(index) = stack.pop() {
            found.push(index);
            stack.extend(self.nodes[index].nodes().iter().rev());
        }
        found
    }

    /// The node before one among its siblings.
    #[must_use]
    pub fn previous(&self, index: usize) -> Option<usize> {
        let siblings = self.nodes[self.nodes[index].parent?].nodes();
        let at = siblings.iter().position(|each| *each == index)?;
        at.checked_sub(1).map(|before| siblings[before])
    }

    /// The node after one among its siblings.
    #[must_use]
    pub fn next(&self, index: usize) -> Option<usize> {
        let siblings = self.nodes[self.nodes[index].parent?].nodes();
        let at = siblings.iter().position(|each| *each == index)?;
        siblings.get(at + 1).copied()
    }
}
