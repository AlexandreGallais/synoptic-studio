//! The release's history from Git: the last tag `vx.y.z` behind the head, the commits since, and
//! the pull request that brought each one, read from GitHub's merge commits on the first-parent
//! line (`Merge pull request #340 from …`) or from a squash merge's subject (`… (#340)`).

use std::collections::HashMap;

use crate::commit::{Commit, merged_branch, merged_pull_request};
use crate::version::Version;

/// A command run for its output: the program, its arguments; its standard output, or the error
/// it printed.
pub type Run<'a> = dyn FnMut(&str, &[&str]) -> Result<String, String> + 'a;

/// The separator of fields in a `git log` format, `%x1f`.
const FIELD: char = '\u{1f}';

/// The separator of records, `%x1e`.
const RECORD: char = '\u{1e}';

/// The last release behind a revision: its tag and its version.
///
/// # Errors
///
/// No tag `vx.y.z` is behind it.
pub fn last_release(run: &mut Run<'_>, head: &str) -> Result<(String, Version), String> {
    let tag = run(
        "git",
        &[
            "describe",
            "--tags",
            "--abbrev=0",
            "--match",
            "v[0-9]*.[0-9]*.[0-9]*",
            head,
        ],
    )
    .map_err(|error| format!("no release tag vx.y.z behind {head}: {error}"))?;
    let tag = tag.trim().to_owned();
    let version = Version::parse(&tag).ok_or_else(|| format!("the tag {tag} is not vx.y.z"))?;

    Ok((tag, version))
}

/// The records of a `git log` output, each split in its fields.
fn records(output: &str) -> Vec<Vec<&str>> {
    output
        .split(RECORD)
        .map(|record| record.trim_start_matches('\n'))
        .filter(|record| !record.is_empty())
        .map(|record| record.split(FIELD).collect())
        .collect()
}

/// A pull request merged on the first-parent line: its number, its merge commit's two parents,
/// the line it joined and its head, and its head branch, empty when the subject names none.
type Merge = (u32, String, String, String);

/// GitHub's merge commits on the first-parent line of a range, the latest first.
fn merges(run: &mut Run<'_>, range: &str) -> Result<Vec<Merge>, String> {
    let log = run(
        "git",
        &[
            "log",
            "--first-parent",
            "--merges",
            "--format=%H%x1f%P%x1f%s%x1e",
            range,
        ],
    )?;

    Ok(records(&log)
        .into_iter()
        .filter_map(|merge| {
            let parents: Vec<&str> = merge.get(1)?.split(' ').collect();
            let subject = merge.get(2)?;
            let number = merged_pull_request(subject)?;
            let [first, second, ..] = parents.as_slice() else {
                return None;
            };

            Some((
                number,
                (*first).to_owned(),
                (*second).to_owned(),
                merged_branch(subject).unwrap_or_default(),
            ))
        })
        .collect())
}

/// The commits of a range, the latest first, merges left out, each with its pull request.
///
/// # Errors
///
/// A Git command failed.
pub fn commits(run: &mut Run<'_>, range: &str) -> Result<Vec<Commit>, String> {
    let log = run(
        "git",
        &["log", "--no-merges", "--format=%H%x1f%s%x1f%b%x1e", range],
    )?;
    let mut pull_requests: HashMap<String, u32> = HashMap::new();

    for (number, first, second, _) in merges(run, range)? {
        let range = format!("{first}..{second}");

        for hash in run("git", &["rev-list", "--no-merges", &range])?.lines() {
            pull_requests.insert(hash.to_owned(), number);
        }
    }

    Ok(records(&log)
        .into_iter()
        .map(|fields| {
            let hash = fields.first().copied().unwrap_or_default().to_owned();
            let mut commit = Commit {
                pull_request: pull_requests.get(&hash).copied(),
                hash,
                subject: fields.get(1).copied().unwrap_or_default().to_owned(),
                body: fields.get(2).copied().unwrap_or_default().to_owned(),
            };

            commit.pull_request = commit.pull_request.or(commit.squashed_pull_request());
            commit
        })
        .collect())
}

/// The pull requests merged on the first-parent line of a range, the latest first, each with its
/// head branch, empty when its subject names none: on `main`, those into `main` since a tag.
///
/// # Errors
///
/// A Git command failed.
pub fn merged_pull_requests(run: &mut Run<'_>, range: &str) -> Result<Vec<(u32, String)>, String> {
    Ok(merges(run, range)?
        .into_iter()
        .map(|(number, _, _, branch)| (number, branch))
        .collect())
}
