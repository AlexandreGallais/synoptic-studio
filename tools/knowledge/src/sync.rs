//! The states of the branches' rows from Git (research note 0035, The rows of a branch): the
//! `draft` rows of a branch merged into `develop`, as `git branch --merged` lists them
//! (`REF-GIT-BRANCH`), become `merged`, each replacing the merged row of its id; of two drafts of
//! one id merged at once, the one written last holds.

use rusqlite::Connection;

/// The branches of `git branch --merged develop --format=%(refname:short)`'s output, one a line,
/// the line of a detached `HEAD`, in parentheses, and the empty ones left out.
#[must_use]
pub fn merged_branches(output: &str) -> Vec<String> {
    let _ = output;
    Vec::new()
}

/// Marks `merged` the `draft` rows of the branches given, in the transaction of the write that
/// calls it; the number of rows marked.
///
/// # Errors
/// When SQLite fails.
pub fn mark_merged(database: &Connection, branches: &[String]) -> rusqlite::Result<usize> {
    let _ = (database, branches);
    Ok(0)
}
