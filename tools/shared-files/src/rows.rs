//! The rows a worker writes for the logs in its story file, under `## For the main thread`, moved
//! into the logs when the story is merged: a heading the tool knows names the log, each line under
//! it is a row.

/// The story file's section holding what the main thread writes at merge.
pub const SECTION: &str = "## For the main thread";

/// Where a log takes new rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    /// After the last row of its first table (`improvements.md`, `feedback.md`).
    Table,
    /// At the end of its newest dated section, the first `## ` heading (`journal.md`, newest first).
    NewestSection,
}

/// A log fed from story files: the heading naming it there, its path and where its rows go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Log {
    /// The heading under `## For the main thread`, e.g. `### Improvements`.
    pub heading: &'static str,
    /// The log, relative to the repository's root.
    pub path: &'static str,
    /// Where its rows go.
    pub place: Place,
}

/// The logs whose rows the tool moves.
pub const LOGS: [Log; 3] = [
    Log {
        heading: "### Journal",
        path: "docs/process/journal.md",
        place: Place::NewestSection,
    },
    Log {
        heading: "### Improvements",
        path: "docs/process/improvements.md",
        place: Place::Table,
    },
    Log {
        heading: "### Feedback",
        path: "docs/process/feedback.md",
        place: Place::Table,
    },
];

/// The rows taken out of a story file, log by log, and the story file without them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Taken {
    /// The story file's new text.
    pub story: String,
    /// The rows of each log that has some, in the order of [`LOGS`].
    pub rows: Vec<(Log, Vec<String>)>,
}

/// Whether a line opens a heading of any level.
fn is_heading(line: &str) -> bool {
    line.starts_with('#')
}

/// The rows under the known headings of a story file's `## For the main thread`, and the story
/// file without those headings; a section left empty goes too. Other headings stay as they are.
#[must_use]
pub fn take_rows(story: &str) -> Taken {
    let lines: Vec<&str> = story.split('\n').collect();
    let Some(start) = lines.iter().position(|line| line.trim_end() == SECTION) else {
        return Taken {
            story: story.to_owned(),
            rows: Vec::new(),
        };
    };
    let end = lines[start + 1..]
        .iter()
        .position(|line| line.starts_with("## ") || line.starts_with("# "))
        .map_or(lines.len(), |offset| start + 1 + offset);
    let mut kept: Vec<&str> = Vec::new();
    let mut found: Vec<(Log, Vec<String>)> = Vec::new();
    let mut current: Option<usize> = None;

    for line in &lines[start + 1..end] {
        if is_heading(line) {
            current = LOGS
                .iter()
                .position(|log| log.heading == line.trim_end())
                .map(|index| {
                    found
                        .iter()
                        .position(|(log, _)| *log == LOGS[index])
                        .unwrap_or_else(|| {
                            found.push((LOGS[index], Vec::new()));
                            found.len() - 1
                        })
                });

            if current.is_some() {
                continue;
            }
        }

        match current {
            Some(index) if !line.trim().is_empty() => found[index].1.push((*line).to_owned()),
            Some(_) => {}
            None => kept.push(line),
        }
    }

    found.retain(|(_, rows)| !rows.is_empty());
    found.sort_by_key(|(log, _)| LOGS.iter().position(|known| known == log));

    let empty = kept.iter().all(|line| line.trim().is_empty());
    let mut text: Vec<&str> = lines[..start].to_vec();

    if empty {
        while text.last().is_some_and(|line| line.trim().is_empty()) {
            text.pop();
        }
        if end < lines.len() {
            text.push("");
        }
    } else {
        text.push(lines[start]);
        text.extend(kept);
    }
    text.extend(&lines[end..]);

    let mut joined = text.join("\n");

    if story.ends_with('\n') && !joined.ends_with('\n') {
        joined.push('\n');
    }

    Taken {
        story: joined,
        rows: found,
    }
}

/// A log with new rows at its place.
///
/// # Errors
///
/// When the log has no table, or no `## ` section, to take them.
pub fn insert_rows(log: &str, place: Place, rows: &[String]) -> Result<String, String> {
    let lines: Vec<&str> = log.split('\n').collect();
    let at = match place {
        Place::Table => {
            let first = lines
                .iter()
                .position(|line| line.starts_with('|'))
                .ok_or("no table to take the rows")?;

            lines[first..]
                .iter()
                .position(|line| !line.starts_with('|'))
                .map_or(lines.len(), |offset| first + offset)
        }
        Place::NewestSection => {
            let first = lines
                .iter()
                .position(|line| line.starts_with("## "))
                .ok_or("no dated section to take the rows")?;
            let next = lines[first + 1..]
                .iter()
                .position(|line| line.starts_with("## "))
                .map_or(lines.len(), |offset| first + 1 + offset);

            lines[first..next]
                .iter()
                .rposition(|line| !line.trim().is_empty())
                .map_or(next, |offset| first + offset + 1)
        }
    };
    let mut text: Vec<&str> = lines[..at].to_vec();

    text.extend(rows.iter().map(String::as_str));
    text.extend(&lines[at..]);

    Ok(text.join("\n"))
}
