//! The backlog in the records (E49, EN-154): its epics, features, stories, tasks and criteria in
//! tables of their own (schema 4), shaped for the agent's questions, and each item's text as a
//! document too, so that `search` finds it. `import-backlog` reads them once from the files the
//! backlog was written in, and may run again: each item's rows are replaced. The criteria of the
//! done features are exported to a data file beside the tests, so that the CI traces each one to
//! a test without the database (ADR-0027; E49, Decisions, Q3); the ids a text cites are checked
//! against the records before a pull request into `develop`.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};

use crate::add::{Origin, add_document};
use crate::document::citations;
use crate::front_matter::front_matter;
use crate::sql::{branch_tables, quoted};
use crate::sync::mark_merged;

/// The kinds of stories the backlog has held, the prefix of their ids.
pub const STORY_KINDS: [&str; 9] = ["US", "EN", "SP", "CHK", "AUD", "VAL", "NAM", "REV", "RET"];

/// What kind of backlog item an id names, by its form alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// `E49`.
    Epic,
    /// `F48`.
    Feature,
    /// `EN-154`, its kind one of [`STORY_KINDS`].
    Story,
}

/// The family of a backlog id, `None` for any other word: `E` or `F` and two digits, or a story's
/// kind, a dash and three digits.
#[must_use]
pub fn family(id: &str) -> Option<Family> {
    let digits = |text: &str, count: usize| {
        text.len() == count && text.bytes().all(|byte| byte.is_ascii_digit())
    };

    if let Some((kind, number)) = id.split_once('-') {
        return (STORY_KINDS.contains(&kind) && digits(number, 3)).then_some(Family::Story);
    }
    match id.split_at_checked(1) {
        Some(("E", number)) if digits(number, 2) => Some(Family::Epic),
        Some(("F", number)) if digits(number, 2) => Some(Family::Feature),
        _ => None,
    }
}

/// An item of the backlog as its file gives it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Item {
    /// `EN-154`, `F48`, `E49`.
    pub id: String,
    /// Its epic, its own id for an epic.
    pub epic: String,
    /// Its feature, empty for an epic, a feature's own id for a feature, empty for a story of an
    /// epic alone.
    pub feature: String,
    pub title: String,
    pub status: String,
    /// A story's points, when its front matter gives them.
    pub points: Option<i64>,
    /// The part of its file's name after its id: `the-backlog-in-the-database`.
    pub topic: String,
    /// The whole text of its file.
    pub text: String,
}

/// The part of a backlog file's stem after its id and a dash, empty when the id is not in it.
#[must_use]
pub fn topic_of(stem: &str, id: &str) -> String {
    let inside = format!("-{id}-");

    stem.find(&inside)
        .map(|at| &stem[at + inside.len()..])
        .or_else(|| stem.strip_prefix(&format!("{id}-")))
        .unwrap_or_default()
        .to_owned()
}

/// An item from its file's name and text, `None` without an id in its front matter.
#[must_use]
pub fn item_of(file: &str, text: &str) -> Option<Item> {
    let fields = front_matter(text);
    let field = |key: &str| fields.get(key).cloned().unwrap_or_default();
    let id = field("id");

    if id.is_empty() {
        return None;
    }

    let stem = file.strip_suffix(".md").unwrap_or(file);
    let (epic, feature) = match family(&id) {
        Some(Family::Epic) => (id.clone(), String::new()),
        Some(Family::Feature) => (field("epic"), id.clone()),
        _ => (field("epic"), field("feature")),
    };

    Some(Item {
        topic: topic_of(stem, &id),
        epic,
        feature,
        title: field("title"),
        status: field("status"),
        points: field("points").parse().ok(),
        text: text.to_owned(),
        id,
    })
}

/// The body of a text's section of level two whose heading starts with a title, up to the next
/// heading of level one or two; empty without one.
#[must_use]
pub fn section_body(text: &str, title: &str) -> String {
    let start = format!("## {title}");
    let mut lines = text.lines();
    let mut body = Vec::new();

    if lines.by_ref().any(|line| line.starts_with(&start)) {
        body.extend(lines.take_while(|line| !line.starts_with("## ") && !line.starts_with("# ")));
    }
    body.join("\n").trim().to_owned()
}

/// A story's task, from a line `- [ ] T1 — title` or `- [x] T1 — title`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub number: i64,
    pub title: String,
    pub done: bool,
}

/// The tasks of a section's lines.
#[must_use]
pub fn tasks_of(section: &str) -> Vec<Task> {
    section
        .lines()
        .filter_map(|line| {
            let (done, rest) = if let Some(rest) = line.strip_prefix("- [ ] T") {
                (false, rest)
            } else {
                (true, line.strip_prefix("- [x] T")?)
            };
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            let title = rest[digits.len()..]
                .trim_start_matches([' ', '—', '-', ':'])
                .trim()
                .to_owned();

            Some(Task {
                number: digits.parse().ok()?,
                title,
                done,
            })
        })
        .collect()
}

/// A feature's numbered criterion, from a line `3. …` of its section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Criterion {
    pub number: i64,
    pub text: String,
    /// Struck through, `3. ~~…~~ Dropped on …`: dropped by the Product Owner, no test proves it.
    pub struck: bool,
}

/// The criteria of a feature's section: one per numbered line.
#[must_use]
pub fn criteria_of(section: &str) -> Vec<Criterion> {
    section
        .lines()
        .filter_map(|line| {
            let digits: String = line.chars().take_while(char::is_ascii_digit).collect();
            let text = line.get(digits.len()..)?.strip_prefix(". ")?;

            Some(Criterion {
                number: digits.parse().ok()?,
                struck: text.starts_with("~~"),
                text: text.to_owned(),
            })
        })
        .collect()
}

/// Where each id first appears as a link at the start of a table's cell, `| [F18](…`, in a
/// parent's text: the order the parent gives its children, from 1.
#[must_use]
pub fn order_in(text: &str) -> HashMap<String, i64> {
    let mut order = HashMap::new();

    for line in text.lines().filter(|line| line.starts_with('|')) {
        for cell in line.split('|') {
            let Some(rest) = cell.trim().strip_prefix('[') else {
                continue;
            };
            let Some((id, _)) = rest.split_once("](") else {
                continue;
            };

            if family(id).is_some() && !order.contains_key(id) {
                let next = i64::try_from(order.len()).unwrap_or(0) + 1;

                order.insert(id.to_owned(), next);
            }
        }
    }
    order
}

/// The order of the epics, from the epics' index: the first cell of each row giving its number.
#[must_use]
pub fn epic_order(index: &str) -> HashMap<String, i64> {
    index
        .lines()
        .filter(|line| line.starts_with('|'))
        .filter_map(|line| {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            let position = cells.get(1)?.parse::<i64>().ok()?;
            let id = cells.get(2)?.strip_prefix('[')?.split_once("](")?.0;

            (family(id) == Some(Family::Epic)).then(|| (id.to_owned(), position))
        })
        .collect()
}

/// The Markdown files of a folder, by name, with their text; none when it cannot be read.
fn files_in(folder: &Path) -> Vec<(String, String)> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| {
                    Path::new(name)
                        .extension()
                        .is_some_and(|extension| extension == "md")
                        && name != "README.md"
                        && !name.starts_with('_')
                })
                .collect()
        })
        .unwrap_or_default();

    names.sort();
    names
        .into_iter()
        .filter_map(|name| {
            let text = fs::read_to_string(folder.join(&name)).ok()?;

            Some((name, text))
        })
        .collect()
}

/// The items of the backlog's folder: epics, features and stories, each with its file's name.
#[must_use]
pub fn read_backlog(folder: &Path) -> Vec<(String, Item)> {
    ["epics", "features", "stories"]
        .iter()
        .flat_map(|kind| {
            files_in(&folder.join(kind))
                .into_iter()
                .filter_map(move |(name, text)| {
                    item_of(&name, &text).map(|item| (format!("{kind}/{name}"), item))
                })
        })
        .collect()
}

/// What `import-backlog` wrote.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Imported {
    pub epics: usize,
    pub features: usize,
    pub stories: usize,
    pub criteria: usize,
    pub tasks: usize,
}

/// Writes a document of the item as merged, replacing the merged row of its id.
fn merged_document(
    database: &Connection,
    item: &Item,
    kind: &str,
    path: &str,
    branch: &str,
    today: &str,
) -> rusqlite::Result<()> {
    let id = add_document(
        database,
        &item.text,
        Origin {
            kind,
            path,
            branch,
            today,
        },
    )?;

    database.execute(
        "DELETE FROM documents WHERE id = ? AND state = 'merged'",
        [&id],
    )?;
    database.execute(
        "UPDATE documents SET state = 'merged' WHERE id = ? AND branch = ? AND state = 'draft'",
        params![id, branch],
    )?;
    Ok(())
}

/// Writes a feature's row and its criteria, the `retired` reason of each kept; the number of
/// criteria written.
fn write_feature(
    database: &Connection,
    item: &Item,
    position: i64,
    branch: &str,
) -> rusqlite::Result<usize> {
    let section = section_body(&item.text, "Acceptance criteria");
    let criteria = criteria_of(&section);

    database.execute(
        "INSERT OR REPLACE INTO features (id, epic, title, status, position, topic, criteria, text, branch, state)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'merged')",
        params![
            item.id,
            item.epic,
            item.title,
            item.status,
            position,
            item.topic,
            section,
            item.text,
            branch
        ],
    )?;
    for criterion in &criteria {
        database.execute(
            "INSERT INTO criteria (id, feature, number, text, struck) VALUES (?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET feature = excluded.feature,
              number = excluded.number, text = excluded.text, struck = excluded.struck",
            params![
                format!("{}.AC{}", item.id, criterion.number),
                item.id,
                criterion.number,
                criterion.text,
                criterion.struck
            ],
        )?;
    }
    Ok(criteria.len())
}

/// Writes a story's row and its tasks; the number of tasks written.
fn write_story(
    database: &Connection,
    item: &Item,
    position: i64,
    branch: &str,
) -> rusqlite::Result<usize> {
    let tasks_section = section_body(&item.text, "Tasks");
    let tasks = tasks_of(&tasks_section);
    let question = [
        section_body(&item.text, "Question for the Product Owner"),
        section_body(&item.text, "Questions for the Product Owner"),
    ]
    .join("\n")
    .trim()
    .to_owned();

    database.execute(
        "INSERT OR REPLACE INTO stories (id, epic, feature, kind, title, status, points,
          position, topic, criteria, tasks, test_card, question, text, branch, state)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'merged')",
        params![
            item.id,
            item.epic,
            item.feature,
            item.id.split_once('-').map_or("", |(kind, _)| kind),
            item.title,
            item.status,
            item.points,
            position,
            item.topic,
            section_body(&item.text, "Acceptance criteria"),
            tasks_section,
            section_body(&item.text, "Product Owner test"),
            question,
            item.text,
            branch
        ],
    )?;
    database.execute("DELETE FROM tasks WHERE story = ?", [&item.id])?;
    for task in &tasks {
        database.execute(
            "INSERT OR REPLACE INTO tasks (story, number, title, done) VALUES (?, ?, ?, ?)",
            params![item.id, task.number, task.title, task.done],
        )?;
    }
    Ok(tasks.len())
}

/// Imports the backlog's folder into the tables, in the transaction of the write that calls it:
/// each item's row replaced, its document merged, a feature's criteria and a story's tasks
/// written again, the criteria's `retired` reason kept. `branch` names the rows' branch, their
/// state `merged`: the backlog imported is the one the trunk holds.
///
/// # Errors
/// When SQLite fails.
pub fn import_backlog(
    database: &Connection,
    folder: &Path,
    branch: &str,
    today: &str,
) -> rusqlite::Result<Imported> {
    let items = read_backlog(folder);
    let index = fs::read_to_string(folder.join("epics").join("README.md")).unwrap_or_default();
    let epics_order = epic_order(&index);
    let texts: HashMap<&str, &str> = items
        .iter()
        .map(|(_, item)| (item.id.as_str(), item.text.as_str()))
        .collect();
    let position = |parent: &str, id: &str| {
        texts
            .get(parent)
            .and_then(|text| order_in(text).get(id).copied())
            .unwrap_or(0)
    };
    let mut imported = Imported::default();

    for (path, item) in &items {
        let full = format!("docs/backlog/{path}");

        match family(&item.id) {
            Some(Family::Epic) => {
                database.execute(
                    "INSERT OR REPLACE INTO epics (id, title, status, position, topic, text, branch, state)
                    VALUES (?, ?, ?, ?, ?, ?, ?, 'merged')",
                    params![
                        item.id,
                        item.title,
                        item.status,
                        epics_order.get(&item.id).copied().unwrap_or(0),
                        item.topic,
                        item.text,
                        branch
                    ],
                )?;
                merged_document(database, item, "epic", &full, branch, today)?;
                imported.epics += 1;
            }
            Some(Family::Feature) => {
                let at = position(&item.epic, &item.id);

                imported.criteria += write_feature(database, item, at, branch)?;
                merged_document(database, item, "feature", &full, branch, today)?;
                imported.features += 1;
            }
            Some(Family::Story) => {
                let parent = if item.feature.is_empty() {
                    &item.epic
                } else {
                    &item.feature
                };

                imported.tasks += write_story(database, item, position(parent, &item.id), branch)?;
                merged_document(database, item, "story", &full, branch, today)?;
                imported.stories += 1;
            }
            None => {}
        }
    }
    Ok(imported)
}

/// A criterion of a done feature for the trace of the CI: its id, and the reason it no longer
/// needs a test, empty when it does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Traced {
    pub id: String,
    pub retired: String,
}

/// The criteria of the done features but those struck through, by feature and number.
///
/// # Errors
/// When SQLite fails.
pub fn done_criteria(database: &Connection) -> rusqlite::Result<Vec<Traced>> {
    let mut statement = database.prepare(
        "SELECT criteria.id, criteria.retired FROM criteria
        JOIN features ON features.id = criteria.feature
        WHERE features.status = 'done' AND criteria.struck = 0
        ORDER BY criteria.feature, criteria.number",
    )?;

    statement
        .query_map([], |row| {
            Ok(Traced {
                id: row.get(0)?,
                retired: row.get(1)?,
            })
        })?
        .collect()
}

/// A text as a JSON string.
#[must_use]
pub fn json_string(text: &str) -> String {
    let mut quoted = String::from('"');

    for character in text.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\t' => quoted.push_str("\\t"),
            '\r' => quoted.push_str("\\r"),
            control if u32::from(control) < 0x20 => {
                let _ = write!(quoted, "\\u{:04x}", u32::from(control));
            }
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

/// The data file of the criteria to trace: a JSON array, one object per criterion and per line,
/// `{ "id": "F48.AC1" }`, with `"retired"` and its reason for one no test needs to prove any more.
#[must_use]
pub fn criteria_json(criteria: &[Traced]) -> String {
    let lines: Vec<String> = criteria
        .iter()
        .map(|criterion| {
            if criterion.retired.is_empty() {
                format!("  {{ \"id\": {} }}", json_string(&criterion.id))
            } else {
                format!(
                    "  {{ \"id\": {}, \"retired\": {} }}",
                    json_string(&criterion.id),
                    json_string(&criterion.retired)
                )
            }
        })
        .collect();

    format!("[\n{}\n]\n", lines.join(",\n"))
}

/// Whether the records hold an item of a backlog id.
fn holds_item(database: &Connection, id: &str, family: Family) -> rusqlite::Result<bool> {
    let table = match family {
        Family::Epic => "epics",
        Family::Feature => "features",
        Family::Story => "stories",
    };

    database
        .query_row(
            &format!("SELECT 1 FROM {} WHERE id = ?", quoted(table)),
            [id],
            |_| Ok(()),
        )
        .optional()
        .map(|found| found.is_some())
}

/// Whether the records hold documents of an id's kind, its part before its first dash, and so
/// whether they can tell it is missing: `JOURNAL-` once a journal row is in, never `REF-` before
/// the bibliography moves.
fn holds_kind(database: &Connection, id: &str) -> rusqlite::Result<bool> {
    let Some((kind, _)) = id.split_once('-') else {
        return Ok(false);
    };
    let prefix = format!("{kind}-");

    database
        .query_row(
            "SELECT 1 FROM documents WHERE substr(id, 1, length(?1)) = ?1 LIMIT 1",
            [&prefix],
            |_| Ok(()),
        )
        .optional()
        .map(|found| found.is_some())
}

/// Whether the records hold a document of an id, in any state.
fn holds_document(database: &Connection, id: &str) -> rusqlite::Result<bool> {
    database
        .query_row("SELECT 1 FROM documents WHERE id = ? LIMIT 1", [id], |_| {
            Ok(())
        })
        .optional()
        .map(|found| found.is_some())
}

/// Whether the records hold a business rule of an id, a bullet of a domain page (E49).
fn holds_rule(database: &Connection, id: &str) -> rusqlite::Result<bool> {
    if !id.starts_with("RULE-") {
        return Ok(false);
    }
    database
        .query_row("SELECT 1 FROM rules WHERE id = ?", [id], |_| Ok(()))
        .optional()
        .map(|found| found.is_some())
}

/// The ids a text cites that the records should hold and do not, each with its line, from 1: a
/// backlog id missing from its table, or an id of a kind whose documents the records hold,
/// missing from them, a business rule's `RULE-` id read in `rules` too; an id of a kind the
/// records hold nothing of is not checked.
///
/// # Errors
/// When SQLite fails.
pub fn missing_ids(database: &Connection, text: &str) -> rusqlite::Result<Vec<(usize, String)>> {
    let mut missing = Vec::new();

    for (number, line) in (1..).zip(text.lines()) {
        // A number of zeros is a placeholder, `US-000`, `F00`, never an item.
        for id in citations(line).into_iter().filter(|id| {
            !id.chars()
                .filter(char::is_ascii_digit)
                .all(|digit| digit == '0')
        }) {
            let is_missing = match family(&id) {
                Some(family) => !holds_item(database, &id, family)?,
                None => {
                    holds_kind(database, &id)?
                        && !holds_document(database, &id)?
                        && !holds_rule(database, &id)?
                }
            };

            if is_missing {
                missing.push((number, id));
            }
        }
    }
    Ok(missing)
}

/// Marks a branch's rows merged, in the transaction of the write that calls it: its drafts of
/// documents as `sync` marks them, and the drafts of every table holding a `branch` and a `state`.
/// The number of rows marked.
///
/// # Errors
/// When SQLite fails.
pub fn merge_branch(database: &Connection, branch: &str) -> rusqlite::Result<usize> {
    let mut marked = mark_merged(database, &[branch.to_owned()])?;

    for table in branch_tables(database)? {
        marked += database.execute(
            &format!(
                "UPDATE {} SET state = 'merged' WHERE branch = ? AND state = 'draft'",
                quoted(&table)
            ),
            [branch],
        )?;
    }
    Ok(marked)
}

/// Marks a branch's drafts dropped, in the transaction of the write that calls it: a branch
/// deleted without its merge, whose rows no other branch reads any more but with `--all`
/// (AUD-050). A dropped row of the same document is replaced. The number of rows marked.
///
/// # Errors
/// When SQLite fails.
pub fn drop_branch(database: &Connection, branch: &str) -> rusqlite::Result<usize> {
    database.execute(
        "DELETE FROM documents WHERE branch = ?1 AND state = 'dropped' AND id IN (
          SELECT id FROM documents WHERE branch = ?1 AND state = 'draft'
        )",
        [branch],
    )?;

    let mut marked = database.execute(
        "UPDATE documents SET state = 'dropped' WHERE branch = ? AND state = 'draft'",
        [branch],
    )?;

    for table in branch_tables(database)? {
        marked += database.execute(
            &format!(
                "UPDATE {} SET state = 'dropped' WHERE branch = ? AND state = 'draft'",
                quoted(&table)
            ),
            [branch],
        )?;
    }
    Ok(marked)
}

/// Sets a backlog item's status, in its table by the form of its id; whether a row had that id.
///
/// # Errors
/// When the id is no backlog id, or SQLite fails.
pub fn set_status(database: &Connection, id: &str, status: &str) -> Result<bool, String> {
    let table = match family(id) {
        Some(Family::Epic) => "epics",
        Some(Family::Feature) => "features",
        Some(Family::Story) => "stories",
        None => {
            return Err(format!(
                "{id}: not an id of the backlog, E49, F48 or EN-154"
            ));
        }
    };

    database
        .execute(
            &format!("UPDATE {} SET status = ? WHERE id = ?", quoted(table)),
            [status, id],
        )
        .map(|changed| changed > 0)
        .map_err(|error| error.to_string())
}

/// The features and stories of an epic, each with its status, by id.
///
/// # Errors
/// When SQLite fails.
pub fn epic_items(database: &Connection, epic: &str) -> rusqlite::Result<Vec<(String, String)>> {
    let mut statement = database.prepare(
        "SELECT id, status FROM features WHERE epic = ?1
        UNION ALL SELECT id, status FROM stories WHERE epic = ?1 ORDER BY 1",
    )?;

    statement
        .query_map([epic], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect()
}
