//! The states of the branches' rows from Git (research note 0035, The rows of a branch): the
//! `draft` rows of a branch merged into `develop` become `merged`, each replacing the merged row of
//! its id; of two drafts of one id merged at once, the one written last holds. A branch is merged
//! when a merge commit brought its tip into `develop`, as every merge of the project is one
//! (`the records' conventions, area git`): its tip reachable from `develop` (`git branch --merged`,
//! `REF-GIT-BRANCH`) is not enough, a branch made at `develop`'s tip without a commit of its own
//! being listed too (AUD-050). `develop`'s own rows are merged by each `sync`.

use std::collections::HashSet;

use rusqlite::Connection;

/// The branch every other is merged into, whose own drafts each `sync` marks merged.
pub const TRUNK: &str = "develop";

/// The branches merged into `develop`, given the output of `git branch --merged develop
/// --format=%(refname:short) %(objectname)`, a branch and its tip a line (`REF-GIT-BRANCH`,
/// `REF-GIT-FOR-EACH-REF`), and that of `git log --merges --format=%P develop`, the parents of each
/// of its merge commits a line (`REF-GIT-LOG`): `develop` itself, and each branch whose tip is a
/// merge commit's parent other than the first, the branch it merged in (`REF-GIT-REV-PARSE`,
/// `<rev>^-`). The line of a detached `HEAD`, in parentheses, and the empty ones are left out.
#[must_use]
pub fn merged_branches(listed: &str, parents: &str) -> Vec<String> {
    let merged_in: HashSet<&str> = parents
        .lines()
        .flat_map(|line| line.split_whitespace().skip(1))
        .collect();

    listed
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('('))
        .filter_map(|line| line.rsplit_once(' '))
        .filter(|(branch, tip)| *branch == TRUNK || merged_in.contains(tip.trim()))
        .map(|(branch, _)| branch.trim().to_owned())
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
