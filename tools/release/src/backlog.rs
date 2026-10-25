//! The stories and features of the backlog, read from the records (EN-154): a commit's `Refs:`
//! footer names its story, the story its feature, so that the draft gives one line per story and
//! keeps a feature's lines together. Read by `knowledge sql` beside the release binary; records
//! that cannot be read, on the CI, give an empty backlog: the draft then names the stories by
//! their commits alone.

use std::collections::HashMap;
use std::path::Path;

/// A story or a feature as the records hold it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    /// `US-041`, `F16`.
    pub id: String,
    /// Its title.
    pub title: String,
    /// The feature of a story, `None` for a feature or a story of no feature.
    pub feature: Option<String>,
}

/// The backlog a repository holds: its stories and its features.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Backlog {
    /// By id, `US-041`.
    pub stories: HashMap<String, Item>,
    /// By id, `F16`.
    pub features: HashMap<String, Item>,
}

/// The statement `knowledge sql` runs: each story and each feature, its id, title and feature.
pub const ITEMS: &str =
    "SELECT id, title, feature FROM stories UNION ALL SELECT id, title, '' FROM features";

/// Whether an id is a feature's: `F` and two digits.
fn is_feature(id: &str) -> bool {
    id.strip_prefix('F')
        .is_some_and(|digits| digits.len() == 2 && digits.bytes().all(|byte| byte.is_ascii_digit()))
}

/// The backlog of the rows of [`ITEMS`]; a row of another shape left out, an empty feature none.
#[must_use]
pub fn backlog_of_rows(rows: &[Vec<String>]) -> Backlog {
    let mut backlog = Backlog::default();

    for row in rows {
        let [id, title, feature] = row.as_slice() else {
            continue;
        };
        let item = Item {
            id: id.clone(),
            title: title.clone(),
            feature: (!feature.is_empty()).then(|| feature.clone()),
        };
        let items = if is_feature(id) {
            &mut backlog.features
        } else {
            &mut backlog.stories
        };

        items.insert(id.clone(), item);
    }
    backlog
}

/// The backlog of the records of the repository under a root, empty when they cannot be read.
#[must_use]
pub fn backlog(root: &Path) -> Backlog {
    knowledge_call::rows(root, ITEMS)
        .map(|rows| backlog_of_rows(&rows))
        .unwrap_or_default()
}
