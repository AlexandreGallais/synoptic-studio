//! The states of the branches' rows from Git (research note 0035, The rows of a branch): the
//! `draft` rows of a branch merged into `develop`, as `git branch --merged` lists them
//! (`REF-GIT-BRANCH`), become `merged`, each replacing the merged row of its id; of two drafts of
//! one id merged at once, the one written last holds.

use rusqlite::Connection;

/// The branches of `git branch --merged develop --format=%(refname:short)`'s output, one a line,
/// the line of a detached `HEAD`, in parentheses, and the empty ones left out.
#[must_use]
pub fn merged_branches(output: &str) -> Vec<String> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('('))
        .map(str::to_owned)
        .collect()
}

/// Marks `merged` the `draft` rows of the branches given, in the transaction of the write that
/// calls it; the number of rows marked.
///
/// # Errors
/// When SQLite fails.
pub fn mark_merged(database: &Connection, branches: &[String]) -> rusqlite::Result<usize> {
    database.execute_batch(
        "CREATE TEMP TABLE IF NOT EXISTS merging (branch TEXT PRIMARY KEY); DELETE FROM merging;",
    )?;
    for branch in branches {
        database.execute(
            "INSERT OR IGNORE INTO merging (branch) VALUES (?)",
            [branch],
        )?;
    }
    // The merged rows of the ids merging go, then the drafts merge; of two drafts of an id merged
    // at once, the one of the highest rowid, written last, stays.
    database.execute(
        "DELETE FROM documents WHERE state = 'merged' AND id IN (
          SELECT id FROM documents WHERE state = 'draft' AND branch IN (SELECT branch FROM merging)
        )",
        [],
    )?;

    let marked = database.execute(
        "UPDATE documents SET state = 'merged'
        WHERE state = 'draft' AND branch IN (SELECT branch FROM merging)",
        [],
    )?;

    database.execute(
        "DELETE FROM documents WHERE state = 'merged' AND rowid NOT IN (
          SELECT max(rowid) FROM documents WHERE state = 'merged' GROUP BY id
        )",
        [],
    )?;
    database.execute("DELETE FROM merging", [])?;
    Ok(marked)
}
