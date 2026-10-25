//! What a branch reads of the knowledge (research note 0035, The rows of a branch; The tool): the
//! `merged` rows and its own `draft` rows, its draft of an id over the merged row of it, the
//! `dropped` rows with `--all`; searched by FTS5, best first by `bm25()` (`REF-SQLITE-FTS5`).

use library::store::fts_query;
use page::section::sections;
use rusqlite::{Connection, ToSql};

/// The filters of `search`, `list` and `show`, each `None` when not given.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Filters {
    /// `--kind`: the documents of that kind.
    pub kind: Option<String>,
    /// `--epic`: the documents of that epic.
    pub epic: Option<String>,
    /// `--feature`: the documents of that feature.
    pub feature: Option<String>,
    /// `--status`: the documents of that status.
    pub status: Option<String>,
    /// `--since`: the documents dated that day or later, `2026-10-25`.
    pub since: Option<String>,
    /// `--branch`: the rows written on that branch, read as that branch reads them.
    pub branch: Option<String>,
    /// `--state`: the rows of that state; `dropped` reads them as `--all` does.
    pub state: Option<String>,
    /// `--all`: the `dropped` rows too.
    pub all: bool,
}

/// A section found by `search`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    /// Its document's id.
    pub id: String,
    /// Its heading, empty for the text above the first heading.
    pub heading: String,
    /// The words found in their context, each between `[` and `]`.
    pub snippet: String,
}

/// A document found by `list`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listed {
    /// Its id.
    pub id: String,
    /// Its kind.
    pub kind: String,
    /// Its status, empty without one.
    pub status: String,
    /// Its date.
    pub date: String,
    /// Its state, `draft`, `merged` or `dropped`.
    pub state: String,
    /// The branch it was written on.
    pub branch: String,
    /// Its title.
    pub title: String,
}

/// How many sections `search` prints at most, as the library's search.
pub const SEARCH_LIMIT: i64 = 8;

/// The rows a branch reads, filtered: its drafts, the merged rows of the ids it has no draft of,
/// the dropped rows when asked for, then each filter given. Its parameters: [`bindings`].
const READ: &str = "
  (documents.state = 'draft' AND documents.branch = :reader
    OR documents.state = 'merged' AND NOT EXISTS (
      SELECT 1 FROM documents AS own
      WHERE own.id = documents.id AND own.state = 'draft' AND own.branch = :reader
    )
    OR documents.state = 'dropped' AND :all)
  AND (:kind IS NULL OR documents.kind = :kind)
  AND (:epic IS NULL OR documents.epic = :epic)
  AND (:feature IS NULL OR documents.feature = :feature)
  AND (:status IS NULL OR documents.status = :status)
  AND (:since IS NULL OR documents.date >= :since)
  AND (:branch IS NULL OR documents.branch = :branch)
  AND (:state IS NULL OR documents.state = :state)";

/// The branch that reads, `--branch`'s when given, and whether it reads the dropped rows.
fn reading<'a>(reader: &'a str, filters: &'a Filters) -> (&'a str, bool) {
    (
        filters.branch.as_deref().unwrap_or(reader),
        filters.all || filters.state.as_deref() == Some("dropped"),
    )
}

/// The parameters of [`READ`].
fn bindings<'a>(
    reader: &'a &'a str,
    all: &'a bool,
    filters: &'a Filters,
) -> Vec<(&'static str, &'a dyn ToSql)> {
    vec![
        (":reader", reader as &dyn ToSql),
        (":all", all),
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

/// The sections matching an FTS5 query, best first, of the rows a branch reads, filtered.
fn matching(
    database: &Connection,
    reader: &str,
    query: &str,
    filters: &Filters,
) -> rusqlite::Result<Vec<Hit>> {
    let (reader, all) = reading(reader, filters);
    let mut parameters = bindings(&reader, &all, filters);

    parameters.push((":query", &query));
    parameters.push((":limit", &SEARCH_LIMIT));

    let mut statement = database.prepare_cached(&format!(
        "SELECT documents.id, sections.heading, snippet(sections_index, 1, '[', ']', '…', 24)
        FROM sections_index JOIN sections ON sections.id = sections_index.rowid
        JOIN documents ON documents.id = sections.document AND documents.branch = sections.branch
          AND documents.state = sections.state
        WHERE sections_index MATCH :query AND {READ}
        ORDER BY bm25(sections_index) LIMIT :limit"
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

/// The sections best matching some words, best first by BM25, at most [`SEARCH_LIMIT`]: every word
/// if a section holds them all, else any; of the rows `reader`, a branch, reads, filtered.
///
/// # Errors
/// When SQLite fails to read, or no word is given.
pub fn search(
    database: &Connection,
    reader: &str,
    words: &str,
    filters: &Filters,
) -> rusqlite::Result<Vec<Hit>> {
    let every = matching(database, reader, &fts_query(words, "AND"), filters)?;

    if every.is_empty() {
        matching(database, reader, &fts_query(words, "OR"), filters)
    } else {
        Ok(every)
    }
}

/// The documents `reader`, a branch, reads, filtered, sorted by id.
///
/// # Errors
/// When SQLite fails to read.
pub fn list(
    database: &Connection,
    reader: &str,
    filters: &Filters,
) -> rusqlite::Result<Vec<Listed>> {
    let (reader, all) = reading(reader, filters);
    let parameters = bindings(&reader, &all, filters);
    let mut statement = database.prepare_cached(&format!(
        "SELECT id, kind, status, date, state, branch, title FROM documents
        WHERE {READ} ORDER BY id, branch"
    ))?;

    statement
        .query_map(parameters.as_slice(), |row| {
            Ok(Listed {
                id: row.get(0)?,
                kind: row.get(1)?,
                status: row.get(2)?,
                date: row.get(3)?,
                state: row.get(4)?,
                branch: row.get(5)?,
                title: row.get(6)?,
            })
        })?
        .collect()
}

/// The text of the document `id` as `reader`, a branch, reads it, filtered: its own draft, else
/// the merged row, else, with `--all`, the dropped row written last; `None` when it reads none.
///
/// # Errors
/// When SQLite fails to read.
pub fn show(
    database: &Connection,
    reader: &str,
    id: &str,
    filters: &Filters,
) -> rusqlite::Result<Option<String>> {
    let (reader, all) = reading(reader, filters);
    let mut parameters = bindings(&reader, &all, filters);

    parameters.push((":id", &id));

    let mut statement = database.prepare_cached(&format!(
        "SELECT documents.branch, documents.state FROM documents
        WHERE documents.id = :id AND {READ}
        ORDER BY CASE documents.state WHEN 'draft' THEN 0 WHEN 'merged' THEN 1 ELSE 2 END,
          documents.rowid DESC
        LIMIT 1"
    ))?;
    let Some((branch, state)) = statement
        .query_map(parameters.as_slice(), |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .next()
        .transpose()?
    else {
        return Ok(None);
    };
    let mut texts = database.prepare_cached(
        "SELECT text FROM sections WHERE document = ? AND branch = ? AND state = ?
        ORDER BY position",
    )?;
    let text = texts
        .query_map([id, branch.as_str(), state.as_str()], |row| {
            row.get::<_, String>(0)
        })?
        .collect::<rusqlite::Result<String>>()?;

    Ok(Some(text))
}

/// The sections of a document's text under a heading, each preceded by `<id>:<line>` as `page`
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

/// A document's line: `<id> [<kind>, <status>, <date>] <title> — <state> on <branch>`, an empty
/// status left out.
#[must_use]
pub fn listed_line(listed: &Listed) -> String {
    let status = if listed.status.is_empty() {
        String::new()
    } else {
        format!("{}, ", listed.status)
    };

    format!(
        "{} [{}, {status}{}] {} — {} on {}",
        listed.id, listed.kind, listed.date, listed.title, listed.state, listed.branch
    )
}
