//! The draft of a release text from its commits (`the records' conventions, area release`, What a release
//! text holds): the commits a user of the library or the playground sees, a revert and what it
//! reverts left out, one line per story in the group of its impact, a feature's lines together,
//! each line marked `[draft]` until the agent rewrites it.

use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt::Write as _;

use crate::backlog::Backlog;
use crate::body::written;
use crate::commit::Commit;
use crate::version::Bump;

/// A change group of Common Changelog, in its order (`REF-COMMON-CHANGELOG`, 2.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    /// A change in existing functionality: a refactoring, a performance, a revert.
    Changed,
    /// New functionality: a `feat`.
    Added,
    /// Functionality removed: a `feat` whose description starts with `remove` or `drop`.
    Removed,
    /// A bug fix: a `fix`.
    Fixed,
}

/// The groups in their order, with their headings.
pub const GROUPS: [(Group, &str); 4] = [
    (Group::Changed, "Changed"),
    (Group::Added, "Added"),
    (Group::Removed, "Removed"),
    (Group::Fixed, "Fixed"),
];

/// The mark of a line still to rewrite; `release check` refuses it.
pub const DRAFT_MARK: &str = "[draft] ";

/// The prefix of a breaking change (`REF-COMMON-CHANGELOG`, 2.4.4).
pub const BREAKING_MARK: &str = "**Breaking:** ";

/// The notice of a release without a change of the library nor the playground
/// (`REF-COMMON-CHANGELOG`, 2.3).
pub const NO_CHANGE_NOTICE: &str =
    "_No change to the library nor the playground: documentation and tooling only._";

/// One line of the draft and the commits behind it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// Its group.
    pub group: Group,
    /// Whether one of its commits breaks.
    pub breaking: bool,
    /// The text to rewrite: its story's title, else its commit's description, capitalised.
    pub text: String,
    /// Its story, `US-041`.
    pub story: Option<String>,
    /// Its story's feature, `F16`.
    pub feature: Option<String>,
    /// Its commits, the latest first.
    pub commits: Vec<Commit>,
}

/// The group of one commit.
#[must_use]
pub fn group_of(commit: &Commit) -> Group {
    match commit.header() {
        Some(header) if header.kind == "feat" => {
            let description = header.description.to_lowercase();

            if description.starts_with("remove ") || description.starts_with("drop ") {
                Group::Removed
            } else {
                Group::Added
            }
        }
        Some(header) if header.kind == "fix" => Group::Fixed,
        _ => Group::Changed,
    }
}

/// The commits left once each revert and the commit it reverts, both in the release, are left out
/// (`REF-COMMON-CHANGELOG`, 3.5).
#[must_use]
pub fn without_no_ops(commits: &[Commit]) -> Vec<Commit> {
    let hashes: HashSet<&str> = commits.iter().map(|commit| commit.hash.as_str()).collect();
    let undone: HashSet<String> = commits
        .iter()
        .filter_map(|commit| {
            commit
                .reverted()
                .filter(|reverted| hashes.contains(reverted.as_str()))
                .map(|reverted| [commit.hash.clone(), reverted])
        })
        .flatten()
        .collect();

    commits
        .iter()
        .filter(|commit| !undone.contains(&commit.hash))
        .cloned()
        .collect()
}

/// A text with its first letter in upper case.
#[must_use]
pub fn capitalised(text: &str) -> String {
    let mut characters = text.chars();

    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

/// The kinds of story whose title names a change for a user: a user story and an enabler. An
/// audit, a checkpoint, a validation or a naming pass is titled by its step (`Audit F12`), so its
/// line takes its oldest commit's description.
pub const CHANGE_STORIES: [&str; 2] = ["EN-", "US-"];

/// Whether a story's title names a change: `US-041`, `EN-039`.
#[must_use]
pub fn is_change_story(id: &str) -> bool {
    CHANGE_STORIES.iter().any(|prefix| id.starts_with(prefix))
}

/// The description of a commit's header, its subject without one.
fn description(commit: &Commit) -> String {
    commit
        .header()
        .map_or_else(|| commit.subject.clone(), |header| header.description)
}

/// The feature's number for the order, the latest first and a line of no feature last.
fn feature_rank(feature: Option<&str>) -> (bool, Reverse<u32>) {
    let number = feature
        .and_then(|id| id.strip_prefix('F'))
        .and_then(|digits| digits.parse().ok());

    (number.is_none(), Reverse(number.unwrap_or(0)))
}

/// The draft's lines, from the release's commits, the latest first: one line per story, its
/// removals on a line of their own, so that a removal is always listed (`REF-KEEP-A-CHANGELOG`,
/// Ignoring deprecations); a commit of no story is a line alone. Sorted by group, then breaking
/// first, then by feature, the latest first, then the latest change first (`REF-COMMON-CHANGELOG`,
/// 2.4).
#[must_use]
pub fn lines(commits: &[Commit], backlog: &Backlog) -> Vec<Line> {
    let mut keyed: Vec<((String, bool), Vec<Commit>)> = Vec::new();

    for commit in without_no_ops(commits)
        .into_iter()
        .filter(Commit::is_user_visible)
    {
        let key = (
            commit
                .story()
                .unwrap_or_else(|| format!("commit {}", commit.hash)),
            group_of(&commit) == Group::Removed,
        );

        match keyed.iter_mut().find(|(known, _)| *known == key) {
            Some((_, grouped)) => grouped.push(commit),
            None => keyed.push((key, vec![commit])),
        }
    }

    let stories_with_two: HashSet<String> = keyed
        .iter()
        .filter(|((_, removal), _)| *removal)
        .filter(|((story, _), _)| keyed.iter().any(|((other, r), _)| other == story && !r))
        .map(|((story, _), _)| story.clone())
        .collect();

    let mut drafted: Vec<Line> = keyed
        .into_iter()
        .map(|((key, removal), grouped)| {
            let story = grouped.first().and_then(Commit::story);
            let item = story.as_ref().and_then(|id| backlog.stories.get(id));
            let groups: Vec<Group> = grouped.iter().map(group_of).collect();
            let group = if removal {
                Group::Removed
            } else if groups.contains(&Group::Added) {
                Group::Added
            } else if groups.contains(&Group::Fixed) {
                Group::Fixed
            } else {
                Group::Changed
            };
            let oldest = grouped.last().map(description).unwrap_or_default();
            let text = match item {
                Some(item)
                    if is_change_story(&item.id)
                        && !(removal && stories_with_two.contains(&key)) =>
                {
                    item.title.clone()
                }
                _ => oldest,
            };

            Line {
                group,
                breaking: grouped.iter().any(|commit| commit.breaking().is_some()),
                text: capitalised(&text),
                feature: item.and_then(|item| item.feature.clone()),
                story,
                commits: grouped,
            }
        })
        .collect();

    drafted.sort_by_key(|line| {
        (
            line.group,
            !line.breaking,
            feature_rank(line.feature.as_deref()),
        )
    });
    drafted
}

/// How much the lines raise the version: a breaking change, then a `feat`, else a patch.
#[must_use]
pub fn bump(lines: &[Line]) -> Bump {
    let commits = || lines.iter().flat_map(|line| &line.commits);

    if lines.iter().any(|line| line.breaking) {
        Bump::Breaking
    } else if commits().any(|commit| commit.header().is_some_and(|header| header.kind == "feat")) {
        Bump::Minor
    } else {
        Bump::Patch
    }
}

/// A line's references (`REF-COMMON-CHANGELOG`, 2.4.2): its pull requests, the latest first, else
/// its commits, the oldest first; the best starting point alone beyond two, the latest pull
/// request or the oldest commit.
#[must_use]
pub fn references(line: &Line, repository: &str) -> String {
    let mut pull_requests: Vec<u32> = Vec::new();

    for number in line.commits.iter().filter_map(|commit| commit.pull_request) {
        if !pull_requests.contains(&number) {
            pull_requests.push(number);
        }
    }

    let links: Vec<String> = if pull_requests.is_empty() {
        let mut commits: Vec<&Commit> = line.commits.iter().rev().collect();

        if commits.len() > 2 {
            commits.truncate(1);
        }
        commits
            .iter()
            .map(|commit| {
                format!(
                    "[`{}`]({repository}/commit/{})",
                    commit.short(),
                    commit.hash
                )
            })
            .collect()
    } else {
        if pull_requests.len() > 2 {
            pull_requests.truncate(1);
        }
        pull_requests
            .iter()
            .map(|number| format!("[#{number}]({repository}/pull/{number})"))
            .collect()
    };

    format!("({})", links.join(", "))
}

/// The draft of a release text: each group and its lines, each marked `[draft]`, or the notice of
/// a release without a change of its own.
#[must_use]
pub fn text(lines: &[Line], repository: &str) -> String {
    let changes: Vec<(Group, String)> = lines
        .iter()
        .map(|line| {
            let breaking = if line.breaking { BREAKING_MARK } else { "" };

            (
                line.group,
                format!(
                    "{breaking}{DRAFT_MARK}{} {}",
                    line.text,
                    references(line, repository)
                ),
            )
        })
        .collect();
    let notice = lines.is_empty().then_some(NO_CHANGE_NOTICE);

    written(notice, &changes)
}

/// What the agent reads to rewrite the lines: each line's group, feature and story, then its
/// commits and the text of a breaking change.
#[must_use]
pub fn hints(lines: &[Line], backlog: &Backlog) -> String {
    let mut text = String::new();

    for line in lines {
        let heading = GROUPS
            .iter()
            .find(|(group, _)| *group == line.group)
            .map_or("", |(_, heading)| heading);
        let feature = line.feature.as_ref().map_or_else(
            || "no feature".to_owned(),
            |id| {
                backlog
                    .features
                    .get(id)
                    .map_or_else(|| id.clone(), |item| format!("{id} {}", item.title))
            },
        );
        let story = line.story.as_deref().unwrap_or("no story");

        let _ = writeln!(text, "{heading} · {feature} · {story}: {}", line.text);
        for commit in &line.commits {
            let _ = writeln!(text, "    {} {}", commit.short(), commit.subject);
            if let Some(breaking) = commit.breaking() {
                let _ = writeln!(text, "      breaking: {breaking}");
            }
        }
    }
    text
}
