//! Sections of a page as the library stores them.

/// One section of a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    /// Heading path of the section, e.g. `Hooks › PostToolUse › Input`.
    pub heading: String,
    /// Its position in the page, from 0.
    pub index: i64,
    /// Its Markdown text, heading line included.
    pub text: String,
}
