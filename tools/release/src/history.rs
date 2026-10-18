//! The release's history from Git: the last tag `vx.y.z` behind the head, the commits since, and
//! the pull request that brought each one, read from GitHub's merge commits on the first-parent
//! line (`Merge pull request #340 from …`) or from a squash merge's subject (`… (#340)`).

use std::collections::HashMap;

use crate::commit::{Commit, merged_pull_request};
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
    let merges = run(
        "git",
        &[
            "log",
            "--first-parent",
            "--merges",
            "--format=%H%x1f%P%x1f%s%x1e",
            range,
        ],
    )?;
    let mut pull_requests: HashMap<String, u32> = HashMap::new();

    for merge in records(&merges) {
        let (Some(parents), Some(subject)) = (merge.get(1), merge.get(2)) else {
            continue;
        };
        let parents: Vec<&str> = parents.split(' ').collect();
        let (Some(number), [first, second, ..]) =
            (merged_pull_request(subject), parents.as_slice())
        else {
            continue;
        };
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
