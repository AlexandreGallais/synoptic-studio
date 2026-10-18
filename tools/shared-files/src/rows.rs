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
    /// At its end, each card under a `## ` heading (`tool-incidents.md`, oldest first): a card is
    /// written `#### <date> — <title>` in a story file, its bullets under it (ADR-0085).
    Cards,
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
pub const LOGS: [Log; 4] = [
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
    Log {
        heading: "### Incidents",
        path: "docs/process/tool-incidents.md",
        place: Place::Cards,
    },
];

/// The rows taken out of a story file, log by log, the lines left in it, and its new text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Taken {
    /// The story file's new text.
    pub story: String,
    /// The rows of each log that has some, in the order of [`LOGS`].
    pub rows: Vec<(Log, Vec<String>)>,
    /// The lines under a log's heading that are not one of its rows, left in the story file under
    /// that heading, in the order of [`LOGS`].
    pub left: Vec<(Log, Vec<String>)>,
}

/// What a line under a log's heading is to the tool.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    /// A row of that log, moved into it.
    Row,
    /// A blank line, a placeholder, or a table's header or separator: dropped.
    Skip,
    /// Anything else: left in the story file and reported.
    Other,
}

/// The texts that say a subsection holds nothing, compared without case nor a final full stop.
const PLACEHOLDERS: [&str; 5] = ["none", "none new", "nothing", "nothing new", "n/a"];

/// Whether a line opens a heading of any level.
fn is_heading(line: &str) -> bool {
    line.starts_with('#')
}

/// Whether a line is a table's separator, such as `| --- | :-: |`.
fn is_separator(line: &str) -> bool {
    let line = line.trim();

    line.len() > 2
        && line.starts_with('|')
        && line.ends_with('|')
        && line[1..line.len() - 1].split('|').all(|cell| {
            let cell = cell.trim();
            let dashes = cell.trim_start_matches(':').trim_end_matches(':');

            !dashes.is_empty() && dashes.chars().all(|letter| letter == '-')
        })
}

/// Whether a line is a table row, `| … |`.
fn is_table_row(line: &str) -> bool {
    line.len() > 1 && line.starts_with('|') && line.ends_with('|')
}

/// The date a text starts with, `2026-10-17`, `None` when it does not start with one.
fn date_at_start(text: &str) -> Option<&str> {
    let date = text.get(..10)?;
    let dated = date.bytes().enumerate().all(|(index, byte)| {
        if index == 4 || index == 7 {
            byte == b'-'
        } else {
            byte.is_ascii_digit()
        }
    });

    dated.then_some(date)
}

/// Whether a table row's first cell starts with a date, `2026-10-17`, as every log row does.
fn is_dated(row: &str) -> bool {
    date_at_start(row[1..].trim_start()).is_some()
}

/// Whether a bullet or a one-cell row only says there is nothing, such as `- None.`.
fn is_placeholder(line: &str) -> bool {
    let text = line
        .strip_prefix("- ")
        .or_else(|| {
            line.strip_prefix('|')
                .and_then(|rest| rest.strip_suffix('|'))
        })
        .unwrap_or(line)
        .trim()
        .trim_end_matches('.')
        .to_ascii_lowercase();

    PLACEHOLDERS.contains(&text.as_str())
}

/// Whether a line, its end trimmed, is a journal bullet opening with its tag and holding a text
/// after it, such as ``- `#run` A finding.``.
fn is_tagged_bullet(line: &str) -> bool {
    line.strip_prefix("- `#")
        .and_then(|rest| rest.split_once("` "))
        .is_some_and(|(tag, _)| {
            !tag.is_empty()
                && tag
                    .chars()
                    .all(|letter| letter.is_ascii_lowercase() || letter == '-')
        })
}

/// What a line under a log's heading is: a table log takes dated table rows, its header (a row
/// followed by a separator) and separator dropped; the journal takes tagged bullets.
fn shape(place: Place, line: &str, next: Option<&str>) -> Shape {
    let line = line.trim_end();

    if line.trim().is_empty() || is_placeholder(line) {
        return Shape::Skip;
    }

    match place {
        Place::Table
            if is_separator(line) || (is_table_row(line) && next.is_some_and(is_separator)) =>
        {
            Shape::Skip
        }
        Place::Table if is_table_row(line) && is_dated(line) => Shape::Row,
        Place::NewestSection if is_tagged_bullet(line) => Shape::Row,
        _ => Shape::Other,
    }
}

/// Adds the lines of a log's heading to a list of `(log, lines)`, in the order of [`LOGS`].
fn add(list: &mut Vec<(Log, Vec<String>)>, log: Log, line: &str) {
    if let Some((_, lines)) = list.iter_mut().find(|(known, _)| *known == log) {
        lines.push(line.to_owned());
    } else {
        list.push((log, vec![line.to_owned()]));
        list.sort_by_key(|(known, _)| LOGS.iter().position(|each| each == known));
    }
}

/// Puts back a log's heading with the lines left under it, between blank lines.
fn put_back<'a>(kept: &mut Vec<&'a str>, block: &mut Vec<&'a str>) {
    if block.len() > 1 {
        if kept.last().is_some_and(|line| !line.trim().is_empty()) {
            kept.push("");
        }
        kept.push(block[0]);
        kept.push("");
        kept.extend(&block[1..]);
        kept.push("");
    }
    block.clear();
}

/// The rows under the known headings of a story file's `## For the main thread`, and the story
/// file without them: a heading whose lines all moved goes, and a section left empty too. A line
/// that is not a row of its log stays under its heading, listed in [`Taken::left`]; a blank line,
/// a placeholder (`- None.`) and a table's header and separator are dropped. Other headings stay as
/// they are.
#[must_use]
pub fn take_rows(story: &str) -> Taken {
    let lines: Vec<&str> = story.split('\n').collect();
    let Some(start) = lines.iter().position(|line| line.trim_end() == SECTION) else {
        return Taken {
            story: story.to_owned(),
            rows: Vec::new(),
            left: Vec::new(),
        };
    };
    let end = lines[start + 1..]
        .iter()
        .position(|line| line.starts_with("## ") || line.starts_with("# "))
        .map_or(lines.len(), |offset| start + 1 + offset);
    let body = &lines[start + 1..end];
    let mut kept: Vec<&str> = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    let mut found: Vec<(Log, Vec<String>)> = Vec::new();
    let mut left: Vec<(Log, Vec<String>)> = Vec::new();
    let mut current: Option<Log> = None;

    for (index, line) in body.iter().enumerate() {
        if is_heading(line) {
            put_back(&mut kept, &mut block);
            current = LOGS
                .iter()
                .find(|log| log.heading == line.trim_end())
                .copied();

            if current.is_some() {
                block.push(line);
                continue;
            }
        }

        let Some(log) = current else {
            kept.push(line);
            continue;
        };

        match shape(log.place, line, body.get(index + 1).copied()) {
            Shape::Row => add(&mut found, log, line),
            Shape::Skip => {}
            Shape::Other => {
                block.push(line);
                add(&mut left, log, line);
            }
        }
    }
    put_back(&mut kept, &mut block);

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
        left,
    }
}

/// The title of the journal's section opened for today's rows when the newest one is older.
pub const CONTINUED: &str = "Run continued";

/// The date opening a `## ` heading, `2026-10-17` in `## 2026-10-17 — Run`, `None` when undated.
fn heading_date(heading: &str) -> Option<&str> {
    date_at_start(heading.strip_prefix("## ")?)
}

/// A log with new rows at its place. In the journal, rows dated after its newest section go into
/// a new section, `## <today> — Run continued`, opened above it.
///
/// # Errors
///
/// When the log has no table, or no `## ` section, to take them.
pub fn insert_rows(
    log: &str,
    place: Place,
    rows: &[String],
    today: &str,
) -> Result<String, String> {
    let lines: Vec<&str> = log.split('\n').collect();
    let mut opened: Vec<String> = Vec::new();
    let at = match place {
        Place::Cards => return Err("no card taken yet".to_owned()),
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

            if heading_date(lines[first]).is_some_and(|date| date < today) {
                opened.push(format!("## {today} — {CONTINUED}"));
                opened.push(String::new());
                opened.extend(rows.iter().cloned());
                opened.push(String::new());
                first
            } else {
                let next = lines[first + 1..]
                    .iter()
                    .position(|line| line.starts_with("## "))
                    .map_or(lines.len(), |offset| first + 1 + offset);

                lines[first..next]
                    .iter()
                    .rposition(|line| !line.trim().is_empty())
                    .map_or(next, |offset| first + offset + 1)
            }
        }
    };
    let mut text: Vec<&str> = lines[..at].to_vec();

    if opened.is_empty() {
        text.extend(rows.iter().map(String::as_str));
    } else {
        text.extend(opened.iter().map(String::as_str));
    }
    text.extend(&lines[at..]);

    Ok(text.join("\n"))
}
