//! The stories and features of `docs/backlog/`, read from their front matter: a commit's `Refs:`
//! footer names its story, the story its feature, so that the draft gives one line per story and
//! keeps a feature's lines together.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A story or a feature as its front matter gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    /// `US-041`, `F16`.
    pub id: String,
    /// Its title, unquoted.
    pub title: String,
    /// The feature of a story, `None` for a feature or a story of no feature.
    pub feature: Option<String>,
}

/// The value of a front matter's key: `title: "A: b"` gives `A: b`.
fn value(front_matter: &str, key: &str) -> Option<String> {
    front_matter.lines().find_map(|line| {
        let raw = line.strip_prefix(key)?.strip_prefix(':')?.trim();
        let unquoted = raw
            .strip_prefix('"')
            .and_then(|inner| inner.strip_suffix('"'))
            .map_or_else(|| raw.to_owned(), |inner| inner.replace("\\\"", "\""));

        Some(unquoted)
    })
}

/// The item a page's front matter describes, `None` without one or without its `id` and `title`.
#[must_use]
pub fn item(text: &str) -> Option<Item> {
    let front_matter = text.strip_prefix("---\n")?.split("\n---").next()?;

    Some(Item {
        id: value(front_matter, "id")?,
        title: value(front_matter, "title")?,
        feature: value(front_matter, "feature"),
    })
}

/// The items of a folder's Markdown pages by id; an unreadable folder gives none.
#[must_use]
pub fn items(folder: &Path) -> HashMap<String, Item> {
    let Ok(entries) = fs::read_dir(folder) else {
        return HashMap::new();
    };

    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
        .filter_map(|entry| fs::read_to_string(entry.path()).ok())
        .filter_map(|text| item(&text))
        .map(|item| (item.id.clone(), item))
        .collect()
}

/// The backlog a repository holds: its stories and its features.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Backlog {
    /// By id, `US-041`.
    pub stories: HashMap<String, Item>,
    /// By id, `F16`.
    pub features: HashMap<String, Item>,
}

/// The backlog under a repository's root.
#[must_use]
pub fn backlog(root: &Path) -> Backlog {
    Backlog {
        stories: items(&root.join("docs/backlog/stories")),
        features: items(&root.join("docs/backlog/features")),
    }
}
