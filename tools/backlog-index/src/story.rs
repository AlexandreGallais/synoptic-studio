//! A story of the backlog, as its front matter describes it.

use std::collections::HashMap;

/// A story: its front-matter fields and its file's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Story {
    pub id: String,
    pub epic: String,
    /// Empty for a story attached to its epic: `NAM`, `REV`, `RET`.
    pub feature: String,
    pub title: String,
    pub status: String,
    pub file: String,
}

impl Story {
    /// A story from its file's name and its front-matter fields; missing fields stay empty.
    pub fn from_fields(file: &str, fields: &HashMap<String, String>) -> Self {
        let field = |key: &str| fields.get(key).cloned().unwrap_or_default();

        Self {
            id: field("id"),
            epic: field("epic"),
            feature: field("feature"),
            title: field("title"),
            status: field("status"),
            file: file.to_owned(),
        }
    }

    /// The item the story belongs to: its feature, or its epic when it has none.
    pub fn parent(&self) -> &str {
        if self.feature.is_empty() {
            &self.epic
        } else {
            &self.feature
        }
    }
}
