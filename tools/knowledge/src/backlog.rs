//! The backlog in the records (E49, EN-154): its epics, features, stories, tasks and criteria in
//! tables of their own (schema 4), shaped for the agent's questions, each item's text cut into an
//! item table (schema 6). The criteria of the done features are exported to a data file beside the
//! tests, so that the CI traces each one to a test without the database (ADR-0027; E49,
//! Decisions, Q3); the ids a text cites are checked against the records before a pull request
//! into `develop`.

use std::fmt::Write as _;

use rusqlite::{Connection, OptionalExtension};

use crate::document::citations;
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

/// The ids of the records that are no backlog item, one per row: the decisions, the notes, the
/// derivations, the sources, the business rules and their parts, the retrospectives and the
/// memories.
const RECORD_IDS: &str = "SELECT id FROM decisions UNION ALL SELECT id FROM notes
  UNION ALL SELECT id FROM derivations UNION ALL SELECT id FROM sources
  UNION ALL SELECT id FROM rules UNION ALL SELECT section FROM rules
  UNION ALL SELECT page FROM rule_items UNION ALL SELECT id FROM retrospectives
  UNION ALL SELECT 'MEMORY-' || name FROM memories";

/// Whether the records hold a record of an id's kind, its part before its first dash, and so
/// whether they can tell it is missing: `ADR-`, `REF-`, `DERIV-`, `NOTE-`, `RULE-` once their
/// tables hold a row, never `JOURNAL-`, whose rows have no such id.
fn holds_kind(database: &Connection, id: &str) -> rusqlite::Result<bool> {
    let Some((kind, _)) = id.split_once('-') else {
        return Ok(false);
    };
    let prefix = format!("{kind}-");

    database
        .query_row(
            &format!("SELECT 1 FROM ({RECORD_IDS}) WHERE substr(id, 1, length(?1)) = ?1 LIMIT 1"),
            [&prefix],
            |_| Ok(()),
        )
        .optional()
        .map(|found| found.is_some())
}

/// Whether the records hold a record of an id.
fn holds_record(database: &Connection, id: &str) -> rusqlite::Result<bool> {
    database
        .query_row(
            &format!("SELECT 1 FROM ({RECORD_IDS}) WHERE id = ? LIMIT 1"),
            [id],
            |_| Ok(()),
        )
        .optional()
        .map(|found| found.is_some())
}

/// Whether an id is a placeholder: digits, all zeros, `US-000`, `F00`, never an item.
fn is_placeholder(id: &str) -> bool {
    let mut digits = id.chars().filter(char::is_ascii_digit).peekable();

    digits.peek().is_some() && digits.all(|digit| digit == '0')
}

/// The ids a text cites that the records should hold and do not, each with its line, from 1: a
/// backlog id missing from its table, or an id of a kind the other records hold (a decision, a
/// note, a derivation, a source, a rule or a part of the rules), missing from them; an id of a
/// kind the records hold nothing of is not checked, nor a placeholder.
///
/// # Errors
/// When SQLite fails.
pub fn missing_ids(database: &Connection, text: &str) -> rusqlite::Result<Vec<(usize, String)>> {
    let mut missing = Vec::new();

    for (number, line) in (1..).zip(text.lines()) {
        for id in citations(line).into_iter().filter(|id| !is_placeholder(id)) {
            let is_missing = match family(&id) {
                Some(family) => !holds_item(database, &id, family)?,
                None => holds_kind(database, &id)? && !holds_record(database, &id)?,
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
