//! Uses of the local source library (`docs/tooling/sources.md`): each search and each page read
//! is recorded, so that a retrospective sees which sources the agent consulted most, what it
//! searched for, and how often the library spared a request online. The library serves every
//! project of the machine: each use names the project that asked (EN-101).

use rusqlite::types::ValueRef;
use rusqlite::{Connection, Row, params};

use crate::text;

/// One use of the library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Use<'a> {
    /// Where the page came from — `library`, `online`, `failed` — or, for a search, `found` or
    /// `none`.
    pub origin: &'a str,
    /// The project that asked, named by its folder (`project_of`).
    pub project: &'a str,
    /// The words searched, empty for a page.
    pub query: &'a str,
    /// `search`, `get` or `read`.
    pub tool: &'a str,
    /// The page, empty for a search.
    pub url: &'a str,
}

/// Rows shown in each ranked table.
const TOP: i64 = 15;

/// The folder of a story's worktree inside a project (EN-052).
const WORKTREES: &str = "/.claude/worktrees";

/// The last part of a path as Node.js's `basename` gives it, trailing slashes ignored.
fn base_name(path: &str) -> &str {
    let path = path.trim_end_matches('/');

    path.rsplit('/').next().unwrap_or_default()
}

/// The project a folder belongs to, as the library records it: the name of the project's folder,
/// the same for each of its worktrees; `synoptic-studio` for
/// `/…/synoptic-studio/.claude/worktrees/en-101`, empty for none.
#[must_use]
pub fn project_of(directory: &str) -> &str {
    let project = directory
        .match_indices(WORKTREES)
        .map(|(at, _)| at)
        // The pattern's `.*?` reads no line end, so the project is found before the first one.
        .take_while(|&at| !directory[..at].contains(text::is_line_end))
        .find(|&at| {
            matches!(
                directory[at + WORKTREES.len()..].chars().next(),
                None | Some('/')
            )
        })
        .map_or(directory, |at| &directory[..at]);

    base_name(project)
}

/// Records one use of the library.
///
/// # Errors
///
/// When SQLite fails to write.
pub fn record_use(database: &Connection, use_: &Use<'_>, time: i64) -> rusqlite::Result<()> {
    database
        .prepare_cached(
            "INSERT INTO project_uses (time, tool, url, query, origin, project) VALUES (?, ?, ?, ?, ?, ?)",
        )?
        .execute(params![time, use_.tool, use_.url, use_.query, use_.origin, use_.project])
        .map(|_| ())
}

/// A value of SQLite as JavaScript's `String()` writes the number or text `node:sqlite` read:
/// `67` for the real 67.0, `null` for a missing one.
fn js_text(row: &Row<'_>, index: usize) -> rusqlite::Result<String> {
    Ok(match row.get_ref(index)? {
        ValueRef::Null => "null".to_owned(),
        ValueRef::Integer(value) => value.to_string(),
        ValueRef::Real(value) => real_text(value),
        ValueRef::Text(value) | ValueRef::Blob(value) => {
            String::from_utf8_lossy(value).into_owned()
        }
    })
}

/// A real as JavaScript writes it: without `.0` when whole.
fn real_text(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

/// A Markdown row, each `|` of a cell escaped.
fn table_row(cells: &[String]) -> String {
    let cells: Vec<String> = cells.iter().map(|cell| cell.replace('|', "\\|")).collect();

    format!("| {} |", cells.join(" | "))
}

/// A Markdown table with a header row.
fn markdown_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let headers: Vec<String> = headers.iter().map(|&header| header.to_owned()).collect();
    let rule: Vec<String> = headers.iter().map(|_| "---".to_owned()).collect();

    [headers, rule]
        .iter()
        .chain(rows)
        .map(|cells| table_row(cells))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The rows of a query, each mapped to its cells.
fn rows(
    database: &Connection,
    sql: &str,
    parameters: &[&dyn rusqlite::ToSql],
    cells: impl Fn(&Row<'_>) -> rusqlite::Result<Vec<String>>,
) -> rusqlite::Result<Vec<Vec<String>>> {
    database
        .prepare_cached(sql)?
        .query_map(parameters, |row| cells(row))?
        .collect()
}

/// Uses of the library per project since a time: project, pages consulted, of which online,
/// searches.
fn projects_table(database: &Connection, since: i64) -> rusqlite::Result<String> {
    let rows = rows(
        database,
        "SELECT project, SUM(tool != 'search') AS pages,
          SUM(tool != 'search' AND origin = 'online') AS online, SUM(tool = 'search') AS searches
        FROM project_uses WHERE time >= ? GROUP BY project ORDER BY COUNT(*) DESC",
        &[&since],
        |row| {
            let project = js_text(row, 0)?;

            Ok(vec![
                if project.is_empty() {
                    "(not recorded)".to_owned()
                } else {
                    project
                },
                js_text(row, 1)?,
                js_text(row, 2)?,
                js_text(row, 3)?,
            ])
        },
    )?;

    Ok(markdown_table(
        &["Project", "Pages consulted", "Of which online", "Searches"],
        &rows,
    ))
}

/// Sources consulted most since a time: title, address, consultations, of which online.
fn sources_table(database: &Connection, since: i64) -> rusqlite::Result<String> {
    let rows = rows(
        database,
        "SELECT project_uses.url, COALESCE(pages.title, '') AS title, COUNT(*) AS count,
          SUM(origin = 'online') AS online
        FROM project_uses LEFT JOIN pages ON pages.url = project_uses.url
        WHERE tool != 'search' AND time >= ? GROUP BY project_uses.url ORDER BY count DESC LIMIT ?",
        &[&since, &TOP],
        |row| {
            Ok(vec![
                js_text(row, 1)?,
                format!("<{}>", js_text(row, 0)?),
                js_text(row, 2)?,
                js_text(row, 3)?,
            ])
        },
    )?;

    Ok(markdown_table(
        &["Source", "Address", "Consulted", "Of which online"],
        &rows,
    ))
}

/// Searches run most since a time: words, times, found.
fn searches_table(database: &Connection, since: i64) -> rusqlite::Result<String> {
    let rows = rows(
        database,
        "SELECT query, COUNT(*) AS count, SUM(origin = 'found') AS found FROM project_uses
        WHERE tool = 'search' AND time >= ? GROUP BY query ORDER BY count DESC LIMIT ?",
        &[&since, &TOP],
        |row| {
            Ok(vec![
                format!("`{}`", js_text(row, 0)?),
                js_text(row, 1)?,
                js_text(row, 2)?,
            ])
        },
    )?;

    Ok(markdown_table(&["Words searched", "Times", "Found"], &rows))
}

/// Report of the library's uses since a time, epoch milliseconds: what was consulted most, what
/// was searched, and the share of pages served without going online, as Markdown sections.
///
/// # Errors
///
/// When SQLite fails to read.
pub fn usage_report(database: &Connection, since: i64) -> rusqlite::Result<String> {
    let (reads, share) = database.query_row(
        "SELECT COUNT(*) AS reads, COALESCE(ROUND(100.0 * SUM(origin = 'library') / COUNT(*)), 0) AS share
        FROM project_uses WHERE tool != 'search' AND time >= ?",
        [since],
        |row| Ok((js_text(row, 0)?, js_text(row, 1)?)),
    )?;

    Ok([
        format!("{reads} pages consulted, {share} % served by the library without going online."),
        "### Projects".to_owned(),
        projects_table(database, since)?,
        "### Sources consulted most".to_owned(),
        sources_table(database, since)?,
        "### Searches".to_owned(),
        searches_table(database, since)?,
    ]
    .join("\n\n"))
}
