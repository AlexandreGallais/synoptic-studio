//! What the page reads from the backlog: a file's front matter and a feature's table of stories
//! (`| ID | Type | Title | Status |`, `docs/conventions/backlog.md`), as the feature branch holds
//! them.

/// A row of a feature's table of stories.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoryRow {
    /// `EN-087`.
    pub id: String,
    /// `Enabler`.
    pub kind: String,
    /// The story's title.
    pub title: String,
    /// `ready`, `in-progress`, `done`…
    pub status: String,
}

/// A field of a file's front matter (`key: value` between the first two `---` lines), without
/// quotes; `None` when the file has no such field.
#[must_use]
pub fn front_matter(text: &str, key: &str) -> Option<String> {
    let mut lines = text.lines();

    if lines.next()?.trim() != "---" {
        return None;
    }

    lines
        .take_while(|line| line.trim() != "---")
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;

            (name.trim() == key).then(|| value.trim().trim_matches('"').to_owned())
        })
}

/// The cells of a Markdown table row, trimmed; empty for a line that is not one.
fn cells(line: &str) -> Vec<String> {
    let line = line.trim();

    if !line.starts_with('|') || !line.ends_with('|') || line.len() < 2 {
        return Vec::new();
    }

    line[1..line.len() - 1]
        .split('|')
        .map(|cell| cell.trim().to_owned())
        .collect()
}

/// A cell's text, a link `[EN-087](…)` read as its label.
fn label(cell: &str) -> String {
    cell.strip_prefix('[')
        .and_then(|rest| rest.split_once("]("))
        .map_or_else(|| cell.to_owned(), |(text, _)| text.to_owned())
}

/// The rows of the feature's table of stories, in their order: the table whose header is
/// `ID | Type | Title | Status`.
#[must_use]
pub fn story_rows(feature: &str) -> Vec<StoryRow> {
    let mut rows = Vec::new();
    let mut inside = false;

    for line in feature.lines() {
        let row = cells(line);

        if row == ["ID", "Type", "Title", "Status"] {
            inside = true;
        } else if row.is_empty() {
            inside = false;
        } else if inside && row.len() == 4 && !row[0].starts_with("---") {
            rows.push(StoryRow {
                id: label(&row[0]),
                kind: row[1].clone(),
                title: row[2].clone(),
                status: row[3].clone(),
            });
        }
    }

    rows
}

/// The feature number of a branch `feature/f20-a-context-diet` (`origin/` allowed before it), as
/// written in the backlog: `F20`.
#[must_use]
pub fn feature_of_branch(branch: &str) -> Option<String> {
    let name = branch.strip_prefix("origin/").unwrap_or(branch);
    let rest = name.strip_prefix("feature/f")?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();

    (!digits.is_empty() && rest[digits.len()..].starts_with('-')).then(|| format!("F{digits}"))
}

/// The file of `id` among paths of `epics/` and `features/`: an epic `E17` is
/// `epics/E17-<topic>.md`, a feature `F20` is `features/E17-F20-<topic>.md`.
#[must_use]
pub fn file_of<'a>(names: &'a [String], id: &str) -> Option<&'a String> {
    let (folder, part) = if id.starts_with('E') {
        ("epics", 0)
    } else {
        ("features", 1)
    };

    names.iter().find(|name| {
        let mut segments = name.rsplit('/');
        let file = segments.next().unwrap_or(name);

        segments.next() == Some(folder) && file.split('-').nth(part) == Some(id)
    })
}
