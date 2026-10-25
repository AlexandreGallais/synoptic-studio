//! What the agent reads of the records (E49, AUD-057): each record a row of its table, its text
//! cut into the rows of an item table (schema 6). `search` reads the full-text index
//! `records_index`, best first by `bm25()`, each hit with a snippet (`REF-SQLITE-FTS5`); `show`
//! renders a record and its items from whichever table holds its id, as Markdown, so that one of
//! its sections is read by its heading; `list` reads a union of the tables. A record of the
//! backlog keeps the branch and the state of its work: a `dropped` one is read with `--all` alone.

use std::fmt::Write as _;

use library::store::every_word_else_any;
use page::section::sections;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, OptionalExtension, Row, ToSql};

/// The filters of `search`, `list` and `show`, each `None` when not given.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Filters {
    /// `--kind`: the records of that kind, `story`, `decision`, `note`, `rule`, `memory`…, or the
    /// table of a row that is no record, `conventions`.
    pub kind: Option<String>,
    /// `--epic`: the records of that epic.
    pub epic: Option<String>,
    /// `--feature`: the records of that feature.
    pub feature: Option<String>,
    /// `--status`: the records of that status.
    pub status: Option<String>,
    /// `--since`: the records dated that day or later, `2026-10-25`.
    pub since: Option<String>,
    /// `--branch`: the records written on that branch.
    pub branch: Option<String>,
    /// `--state`: the records of that state; `dropped` reads them as `--all` does.
    pub state: Option<String>,
    /// `--all`: the `dropped` records too.
    pub all: bool,
}

/// A row found by `search`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    /// The id of the record it belongs to, which `show` opens.
    pub id: String,
    /// Its section: an item's heading, else the record's kind.
    pub heading: String,
    /// The words found in their context, each between `[` and `]`.
    pub snippet: String,
}

/// A record found by `list`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listed {
    /// Its id.
    pub id: String,
    /// Its kind.
    pub kind: String,
    /// Its status, empty without one.
    pub status: String,
    /// Its date, empty without one.
    pub date: String,
    /// Its state, `draft`, `merged` or `dropped`; `merged` for a table without states.
    pub state: String,
    /// The branch it was written on, empty for a table without branches.
    pub branch: String,
    /// Its title.
    pub title: String,
}

/// How many rows `search` prints at most, as the library's search.
pub const SEARCH_LIMIT: i64 = 8;

/// The records of every table, one row each, as `list` reads them: `(id, kind, status, date,
/// state, branch, title, epic, feature)`. A part of the business rules, the id its rules and its
/// items share, is a record of the kind `rule` too.
const LISTED: &str = "listed (id, kind, status, date, state, branch, title, epic, feature) AS (
    SELECT id, 'story', status, '', state, branch, title, epic, feature FROM stories
    UNION ALL SELECT id, 'feature', status, '', state, branch, title, epic, id FROM features
    UNION ALL SELECT id, 'epic', status, '', state, branch, title, id, '' FROM epics
    UNION ALL SELECT id, 'decision', status, date, 'merged', '', title, '', '' FROM decisions
    UNION ALL SELECT id, kind, status, date, 'merged', '', title, '', '' FROM notes
    UNION ALL SELECT id, 'derivation', '', '', 'merged', '', title, '', '' FROM derivations
    UNION ALL SELECT id, 'source', status, verified, 'merged', '', title, '', '' FROM sources
    UNION ALL SELECT id, 'rule', '', '', 'merged', '', name, '', '' FROM rules
    UNION ALL SELECT part, 'rule', '', '', 'merged', '', 'a part of the business rules', '', ''
      FROM (SELECT page AS part FROM rule_items UNION SELECT section FROM rules)
    UNION ALL SELECT id, 'retrospective', '', '', 'merged', '', title, epic, '' FROM retrospectives
    UNION ALL SELECT 'MEMORY-' || name, 'memory', type, modified, 'merged', '', description, '', ''
      FROM memories
  )";

/// The filters on a record of [`LISTED`], `kind` and `state` given as the expressions to compare,
/// so that a hit of a row that is no record is filtered by its table. Its parameters:
/// [`bindings`].
fn filtered(kind: &str, state: &str) -> String {
    format!(
        "({state} <> 'dropped' OR :all)
        AND (:kind IS NULL OR {kind} = :kind)
        AND (:epic IS NULL OR listed.epic = :epic)
        AND (:feature IS NULL OR listed.feature = :feature)
        AND (:status IS NULL OR listed.status = :status)
        AND (:since IS NULL OR listed.date >= :since)
        AND (:branch IS NULL OR listed.branch = :branch)
        AND (:state IS NULL OR {state} = :state)"
    )
}

/// Whether the filters read the dropped records.
fn reads_dropped(filters: &Filters) -> bool {
    filters.all || filters.state.as_deref() == Some("dropped")
}

/// The parameters of [`filtered`].
fn bindings<'a>(all: &'a bool, filters: &'a Filters) -> Vec<(&'static str, &'a dyn ToSql)> {
    vec![
        (":all", all as &dyn ToSql),
        (":kind", &filters.kind),
        (":epic", &filters.epic),
        (":feature", &filters.feature),
        (":status", &filters.status),
        (":since", &filters.since),
        (":branch", &filters.branch),
        (":state", &filters.state),
    ]
}

/// The filters of the words of a command, and the words that are none: `--all`, and each other
/// flag followed by its value.
///
/// # Errors
/// On an unknown flag, or a flag without its value.
pub fn parse_filters(words: &[String]) -> Result<(Vec<String>, Filters), String> {
    let mut filters = Filters::default();
    let mut rest = Vec::new();
    let mut words = words.iter();

    while let Some(word) = words.next() {
        if word == "--all" {
            filters.all = true;
            continue;
        }
        if !word.starts_with("--") {
            rest.push(word.clone());
            continue;
        }

        let slot = match word.as_str() {
            "--kind" => &mut filters.kind,
            "--epic" => &mut filters.epic,
            "--feature" => &mut filters.feature,
            "--status" => &mut filters.status,
            "--since" => &mut filters.since,
            "--branch" => &mut filters.branch,
            "--state" => &mut filters.state,
            _ => return Err(format!("{word}: no such filter")),
        };
        let value = words
            .next()
            .filter(|value| !value.starts_with("--"))
            .ok_or_else(|| format!("{word} needs a value"))?;

        *slot = Some(value.clone());
    }
    if let Some(state) = filters
        .state
        .as_deref()
        .filter(|state| !matches!(*state, "draft" | "merged" | "dropped"))
    {
        return Err(format!(
            "--state {state}: a state is draft, merged or dropped"
        ));
    }
    Ok((rest, filters))
}

/// The rows of `records_index` matching an FTS5 query, best first, filtered by the record each
/// belongs to: a task's story, a criterion's feature, an item's record; a row of a table that is
/// no record (a convention, a journal entry) by its table alone.
fn matching(database: &Connection, query: &str, filters: &Filters) -> rusqlite::Result<Vec<Hit>> {
    let all = reads_dropped(filters);
    let mut parameters = bindings(&all, filters);

    parameters.push((":query", &query));
    parameters.push((":limit", &SEARCH_LIMIT));

    let mut statement = database.prepare_cached(&format!(
        "WITH {LISTED}
        SELECT records_index.id, records_index.section,
          snippet(records_index, 4, '[', ']', '…', 24)
        FROM records_index LEFT JOIN listed ON listed.id = CASE records_index.kind
          WHEN 'tasks' THEN substr(records_index.id, 1, instr(records_index.id, '.T') - 1)
          WHEN 'criteria' THEN substr(records_index.id, 1, instr(records_index.id, '.AC') - 1)
          ELSE records_index.id END
        WHERE records_index MATCH :query AND {}
        ORDER BY bm25(records_index) LIMIT :limit",
        filtered(
            "coalesce(listed.kind, records_index.kind)",
            "coalesce(listed.state, 'merged')"
        )
    ))?;

    statement
        .query_map(parameters.as_slice(), |row| {
            Ok(Hit {
                id: row.get(0)?,
                heading: row.get(1)?,
                snippet: row.get(2)?,
            })
        })?
        .collect()
}

/// The rows best matching some words, best first by BM25, at most [`SEARCH_LIMIT`]: every word if
/// a row holds them all, else any; filtered.
///
/// # Errors
/// When SQLite fails to read, or no word is given.
pub fn search(database: &Connection, words: &str, filters: &Filters) -> rusqlite::Result<Vec<Hit>> {
    every_word_else_any(words, |query| matching(database, query, filters))
}

/// The records of every table, filtered, sorted by kind and id.
///
/// # Errors
/// When SQLite fails to read.
pub fn list(database: &Connection, filters: &Filters) -> rusqlite::Result<Vec<Listed>> {
    let all = reads_dropped(filters);
    let parameters = bindings(&all, filters);
    let mut statement = database.prepare_cached(&format!(
        "WITH {LISTED}
        SELECT id, kind, status, date, state, branch, title FROM listed
        WHERE {} ORDER BY kind, id",
        filtered("listed.kind", "listed.state")
    ))?;

    statement
        .query_map(parameters.as_slice(), |row| {
            Ok(Listed {
                id: row.get(0)?,
                kind: row.get(1)?,
                status: cell(row, 2),
                date: cell(row, 3),
                state: row.get(4)?,
                branch: row.get(5)?,
                title: cell(row, 6),
            })
        })?
        .collect()
}

/// A cell as text: a number written out, a null empty.
fn cell(row: &Row<'_>, at: usize) -> String {
    match row.get_ref(at) {
        Ok(ValueRef::Text(bytes)) => String::from_utf8_lossy(bytes).into_owned(),
        Ok(ValueRef::Integer(number)) => number.to_string(),
        Ok(ValueRef::Real(number)) => number.to_string(),
        _ => String::new(),
    }
}

/// The cells of the one row a statement reads for an id, each as text; `None` without one.
fn record(database: &Connection, query: &str, id: &str) -> rusqlite::Result<Option<Vec<String>>> {
    let mut statement = database.prepare_cached(query)?;
    let count = statement.column_count();

    statement
        .query_row([id], |row| Ok((0..count).map(|at| cell(row, at)).collect()))
        .optional()
}

/// The rows a statement reads for an id, each a list of cells as text.
fn rows(database: &Connection, query: &str, id: &str) -> rusqlite::Result<Vec<Vec<String>>> {
    let mut statement = database.prepare_cached(query)?;
    let count = statement.column_count();

    statement
        .query_map([id], |row| Ok((0..count).map(|at| cell(row, at)).collect()))?
        .collect()
}

/// A record rendered as Markdown: its title, a line of its facts and the items of no section, the
/// text above its first heading; then its parts, each a heading of level two.
struct Rendered {
    /// The title, the facts and the items of no section.
    head: String,
    /// The parts.
    body: String,
}

impl Rendered {
    /// A record's first lines: `# <id> — <title>`, then its facts joined by ` · `, the empty
    /// ones left out.
    fn new(id: &str, title: &str, facts: &[String]) -> Self {
        let facts: Vec<&str> = facts
            .iter()
            .map(String::as_str)
            .filter(|fact| !fact.is_empty())
            .collect();
        let mut head = if title.is_empty() {
            format!("# {id}\n")
        } else {
            format!("# {id} — {title}\n")
        };

        if !facts.is_empty() {
            head.push('\n');
            head.push_str(&facts.join(" · "));
            head.push('\n');
        }
        Self {
            head,
            body: String::new(),
        }
    }

    /// A part under its heading, left out when its text is empty.
    fn part(&mut self, heading: &str, text: &str) {
        let text = text.trim();

        if !text.is_empty() {
            let _ = write!(self.body, "\n## {heading}\n\n{text}\n");
        }
    }

    /// A part of one line per row, left out without a row.
    fn lines(&mut self, heading: &str, lines: &[String]) {
        self.part(heading, &lines.join("\n"));
    }

    /// The items of a record, `(section, kind, item, source)` in their order: those of no section
    /// under its facts, the others in the parts, each run of one section under its heading.
    fn items(&mut self, items: &[Vec<String>]) {
        let (intro, sectioned): (Vec<&Vec<String>>, Vec<&Vec<String>>) = items
            .iter()
            .partition(|item| item.first().is_some_and(String::is_empty));

        push_items(&mut self.head, &intro);
        push_items(&mut self.body, &sectioned);
    }

    /// The whole text.
    fn text(self) -> String {
        self.head + &self.body
    }
}

/// Writes items, `(section, kind, item, source)`: a heading at each new section; a bullet or a
/// table's row as a line of a list, a paragraph or a block of code apart; a source cited after
/// its item.
fn push_items(text: &mut String, items: &[&Vec<String>]) {
    let mut section: Option<&str> = None;
    let mut was_listed = false;

    for item in items {
        let [heading, kind, item, source] = item.as_slice() else {
            continue;
        };

        if section != Some(heading.as_str()) {
            section = Some(heading.as_str());
            if !heading.is_empty() {
                let _ = write!(text, "\n## {heading}\n");
            }
            was_listed = false;
        }

        let is_listed = matches!(kind.as_str(), "bullet" | "row");

        if !(is_listed && was_listed) {
            text.push('\n');
        }
        if is_listed {
            text.push_str("- ");
        }
        text.push_str(item.trim_end());
        if !source.is_empty() {
            let _ = write!(text, " ({source})");
        }
        text.push('\n');
        was_listed = is_listed;
    }
}

/// `<label> <value>`, empty when the value is.
fn fact(label: &str, value: &str) -> String {
    if value.is_empty() {
        String::new()
    } else {
        format!("{label} {value}")
    }
}

/// The items of a record, from its item table: `(section, kind, item, source)` by `n`.
fn items_of(
    database: &Connection,
    table: &str,
    owner: &str,
    text: &str,
    source: &str,
    id: &str,
) -> rusqlite::Result<Vec<Vec<String>>> {
    rows(
        database,
        &format!(
            "SELECT section, kind, {text}, {source} FROM {table} WHERE {owner} = ? ORDER BY n"
        ),
        id,
    )
}

/// The record items of a table whose items are `item` with no source.
fn plain_items(
    database: &Connection,
    table: &str,
    owner: &str,
    id: &str,
) -> rusqlite::Result<Vec<Vec<String>>> {
    items_of(database, table, owner, "item", "''", id)
}

/// A story, its tasks and its items.
fn story(database: &Connection, id: &str, all: bool) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, kind, epic, feature, status, points, state, branch, summary, criteria,
          tasks, test_card, question FROM stories WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    if row[6] == "dropped" && !all {
        return Ok(None);
    }

    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            format!("story {}", row[1]),
            fact("epic", &row[2]),
            fact("feature", &row[3]),
            fact("status", &row[4]),
            fact("points", &row[5]),
            format!("{} on {}", row[6], row[7]),
        ],
    );
    let tasks: Vec<String> = rows(
        database,
        "SELECT number, title, done FROM tasks WHERE story = ? ORDER BY number",
        id,
    )?
    .iter()
    .map(|task| {
        let mark = if task[2] == "1" { "x" } else { " " };

        format!("- [{mark}] T{} — {}", task[0], task[1])
    })
    .collect();

    rendered.part("Summary", &row[8]);
    rendered.part("Acceptance criteria", &row[9]);
    if tasks.is_empty() {
        rendered.part("Tasks", &row[10]);
    } else {
        rendered.lines("Tasks", &tasks);
    }
    rendered.part("Product Owner test", &row[11]);
    rendered.part("Question", &row[12]);
    rendered.items(&plain_items(database, "story_items", "story", id)?);
    Ok(Some(rendered.text()))
}

/// The lines `- <id> [<status>] <title>` of the stories or features a query reads for an id.
fn children(database: &Connection, query: &str, id: &str) -> rusqlite::Result<Vec<String>> {
    Ok(rows(database, query, id)?
        .iter()
        .map(|child| format!("- {} [{}] {}", child[0], child[1], child[2]))
        .collect())
}

/// A feature, its criteria, its stories and its items.
fn feature(database: &Connection, id: &str, all: bool) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, epic, status, state, branch, hypothesis, criteria FROM features
        WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    if row[3] == "dropped" && !all {
        return Ok(None);
    }

    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            "feature".to_owned(),
            fact("epic", &row[1]),
            fact("status", &row[2]),
            format!("{} on {}", row[3], row[4]),
        ],
    );
    let criteria: Vec<String> = rows(
        database,
        "SELECT number, text, retired FROM criteria WHERE feature = ? ORDER BY number",
        id,
    )?
    .iter()
    .map(|criterion| {
        if criterion[2].is_empty() {
            format!("{}. {}", criterion[0], criterion[1])
        } else {
            format!(
                "{}. {} (retired: {})",
                criterion[0], criterion[1], criterion[2]
            )
        }
    })
    .collect();

    rendered.part("Hypothesis", &row[5]);
    if criteria.is_empty() {
        rendered.part("Acceptance criteria", &row[6]);
    } else {
        rendered.lines("Acceptance criteria", &criteria);
    }
    rendered.lines(
        "Stories",
        &children(
            database,
            "SELECT id, status, title FROM stories WHERE feature = ? AND state <> 'dropped'
            ORDER BY position, id",
            id,
        )?,
    );
    rendered.items(&plain_items(database, "feature_items", "feature", id)?);
    Ok(Some(rendered.text()))
}

/// An epic, its features, its stories of no feature and its items.
fn epic(database: &Connection, id: &str, all: bool) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, status, state, branch, vision FROM epics WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    if row[2] == "dropped" && !all {
        return Ok(None);
    }

    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            "epic".to_owned(),
            fact("status", &row[1]),
            format!("{} on {}", row[2], row[3]),
        ],
    );

    rendered.part("Vision", &row[4]);
    rendered.lines(
        "Features",
        &children(
            database,
            "SELECT id, status, title FROM features WHERE epic = ? AND state <> 'dropped'
            ORDER BY position, id",
            id,
        )?,
    );
    rendered.lines(
        "Stories of no feature",
        &children(
            database,
            "SELECT id, status, title FROM stories WHERE epic = ? AND feature = ''
              AND state <> 'dropped'
            ORDER BY position, id",
            id,
        )?,
    );
    rendered.items(&plain_items(database, "epic_items", "epic", id)?);
    Ok(Some(rendered.text()))
}

/// A decision, its columns as parts and its items.
fn decision(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, status, date, changed, principle, decision, reason, consequences,
          status_note, replaces, replaced_by, removed_reason, refs FROM decisions WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            "decision".to_owned(),
            fact("status", &row[1]),
            fact("date", &row[2]),
            fact("changed", &row[3]),
        ],
    );

    for (heading, at) in [
        ("Principle", 4),
        ("Decision", 5),
        ("Reason", 6),
        ("Consequences", 7),
        ("Status", 8),
        ("Replaces", 9),
        ("Replaced by", 10),
        ("Removed", 11),
        ("References", 12),
    ] {
        rendered.part(heading, &row[at]);
    }
    rendered.items(&plain_items(database, "decision_items", "decision", id)?);
    Ok(Some(rendered.text()))
}

/// A research note, request or page, and its findings with their sources.
fn note(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, kind, status, date FROM notes WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            row[1].clone(),
            fact("status", &row[2]),
            fact("date", &row[3]),
        ],
    );

    rendered.items(&items_of(
        database,
        "note_findings",
        "note",
        "finding",
        "source",
        id,
    )?);
    Ok(Some(rendered.text()))
}

/// A derivation and its steps with their sources.
fn derivation(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, used_by FROM derivations WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    let mut rendered = Rendered::new(id, &row[0], &["derivation".to_owned()]);

    rendered.part("Used by", &row[1]);
    rendered.items(&items_of(
        database,
        "derivation_steps",
        "derivation",
        "statement",
        "source",
        id,
    )?);
    Ok(Some(rendered.text()))
}

/// A source of the bibliography.
fn source(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, status, verified, topic, url, used_for, notes FROM sources WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            "source".to_owned(),
            fact(&row[1], &row[2]),
            fact("topic", &row[3]),
        ],
    );

    rendered.part("Address", &row[4]);
    rendered.part("Used for", &row[5]);
    rendered.part("Notes", &row[6]);
    Ok(Some(rendered.text()))
}

/// A business rule, its rule and its example.
fn rule(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT name, page, section, rule, example FROM rules WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            "rule".to_owned(),
            fact("page", &row[1]),
            fact("part", &row[2]),
        ],
    );

    rendered.part("Rule", &row[3]);
    rendered.part("Example", &row[4]);
    Ok(Some(rendered.text()))
}

/// A part of the business rules: its items, then its rules, each with its example.
fn rule_part(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let items = plain_items(database, "rule_items", "page", id)?;
    let rules: Vec<String> = rows(
        database,
        "SELECT id, name, rule, example FROM rules WHERE section = ? ORDER BY position, id",
        id,
    )?
    .iter()
    .map(|rule| {
        format!(
            "- **{}** ({}): {} Example: {}",
            rule[1], rule[0], rule[2], rule[3]
        )
    })
    .collect();

    if items.is_empty() && rules.is_empty() {
        return Ok(None);
    }

    let mut rendered = Rendered::new(id, "", &["a part of the business rules".to_owned()]);

    rendered.items(&items);
    rendered.lines("Rules", &rules);
    Ok(Some(rendered.text()))
}

/// A retrospective and its items.
fn retrospective(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT title, epic FROM retrospectives WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    let mut rendered = Rendered::new(
        id,
        &row[0],
        &["retrospective".to_owned(), fact("epic", &row[1])],
    );

    rendered.items(&plain_items(
        database,
        "retrospective_items",
        "retrospective",
        id,
    )?);
    Ok(Some(rendered.text()))
}

/// A memory of the agent, `MEMORY-<name>`, and its text.
fn memory(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(name) = id.strip_prefix("MEMORY-") else {
        return Ok(None);
    };
    let Some(row) = record(
        database,
        "SELECT description, type, modified, text FROM memories WHERE name = ?",
        name,
    )?
    else {
        return Ok(None);
    };
    let mut rendered = Rendered::new(
        id,
        &row[0],
        &[
            "memory".to_owned(),
            fact("type", &row[1]),
            fact("modified", &row[2]),
        ],
    );

    rendered.part("Text", &row[3]);
    Ok(Some(rendered.text()))
}

/// A feature's criterion, `F48.AC1`.
fn criterion(database: &Connection, id: &str) -> rusqlite::Result<Option<String>> {
    let Some(row) = record(
        database,
        "SELECT feature, struck, retired, text FROM criteria WHERE id = ?",
        id,
    )?
    else {
        return Ok(None);
    };
    let struck = if row[1] == "1" { "struck" } else { "" };
    let mut rendered = Rendered::new(
        id,
        "",
        &[
            "criterion".to_owned(),
            fact("feature", &row[0]),
            struck.to_owned(),
            fact("retired:", &row[2]),
        ],
    );

    rendered.part("Criterion", &row[3]);
    Ok(Some(rendered.text()))
}

/// The record of an id rendered as Markdown, from whichever table holds it: a story, a feature,
/// an epic, a decision, a note, a derivation, a source, a rule or a part of the rules, a
/// retrospective, a memory or a criterion; a `dropped` story, feature or epic only with `--all`;
/// `None` when no table holds it.
///
/// # Errors
/// When SQLite fails to read.
pub fn show(
    database: &Connection,
    id: &str,
    filters: &Filters,
) -> rusqlite::Result<Option<String>> {
    let all = reads_dropped(filters);
    let found = match story(database, id, all)? {
        Some(text) => Some(text),
        None => match feature(database, id, all)? {
            Some(text) => Some(text),
            None => epic(database, id, all)?,
        },
    };

    if found.is_some() {
        return Ok(found);
    }
    for render in [
        decision,
        note,
        derivation,
        source,
        rule,
        rule_part,
        retrospective,
        memory,
        criterion,
    ] {
        if let Some(text) = render(database, id)? {
            return Ok(Some(text));
        }
    }
    Ok(None)
}

/// The sections of a record's text under a heading, each preceded by `<id>:<line>` as `page`
/// prints a section; `None` when no heading of it names it.
#[must_use]
pub fn section_of(id: &str, text: &str, heading: &str) -> Option<String> {
    let parts: Vec<String> = sections(text, heading)
        .into_iter()
        .map(|section| format!("{id}:{}\n{}", section.line, section.text))
        .collect();

    (!parts.is_empty()).then(|| parts.join("\n"))
}

/// A hit's two lines: `<id> › <heading>`, the heading left out when empty, then its snippet on one
/// line, indented.
#[must_use]
pub fn hit_lines(hit: &Hit) -> String {
    let snippet = hit.snippet.split_whitespace().collect::<Vec<_>>().join(" ");

    if hit.heading.is_empty() {
        format!("{}\n    {snippet}", hit.id)
    } else {
        format!("{} › {}\n    {snippet}", hit.id, hit.heading)
    }
}

/// A record's line: `<id> [<kind>, <status>, <date>] <title> — <state> on <branch>`, an empty
/// status or date left out, and ` on <branch>` without a branch.
#[must_use]
pub fn listed_line(listed: &Listed) -> String {
    let facts: Vec<&str> = [
        listed.kind.as_str(),
        listed.status.as_str(),
        listed.date.as_str(),
    ]
    .into_iter()
    .filter(|fact| !fact.is_empty())
    .collect();
    let branch = if listed.branch.is_empty() {
        String::new()
    } else {
        format!(" on {}", listed.branch)
    };

    format!(
        "{} [{}] {} — {}{branch}",
        listed.id,
        facts.join(", "),
        listed.title,
        listed.state
    )
}
