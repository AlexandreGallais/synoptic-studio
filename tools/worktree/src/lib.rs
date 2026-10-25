//! A worktree named after its story (EN-052, `docs/tooling/agent-tools.md`): `US-061` gets
//! `.claude/worktrees/us-061` on the branch `feat/us-061-<topic>`, made from its feature branch on
//! GitHub. The story's kind, feature and topic come from the records, one database every branch
//! reads, so that a story written on a feature branch is found from the main checkout too (EN-154,
//! after EN-118's search of the feature branches' files); the branch type from its kind, the
//! feature branch from the remote branches. What the tool runs is decided here, without Git.

pub mod seed;

use std::path::{Path, PathBuf};

use git_hooks::branch::is_valid_branch;
use knowledge_call::{is_plain, rows};

/// Folder of the worktrees, from the main checkout.
pub const WORKTREES: &str = ".claude/worktrees";

/// Branch types the pre-push hook accepts (`tools/git-hooks/src/branch.rs`).
const BRANCH_TYPES: [&str; 11] = [
    "feat", "fix", "docs", "refactor", "test", "chore", "build", "ci", "perf", "style", "revert",
];

/// Base of a story that belongs to no feature (a review, a retrospective).
const DEVELOP: &str = "origin/develop";

/// Kinds of the stories made from `develop` outside any feature, which may start before their
/// file is on a branch: an epic's refinement spike and a review (ADR-0086), the prefix with its
/// hyphen.
const OUTSIDE_FEATURES: [&str; 2] = ["SP-", "REV-"];

/// A story as the records hold it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Story {
    /// `US-061`.
    pub id: String,
    /// `US`.
    pub kind: String,
    /// `F18`, `None` for an epic's own story.
    pub feature: Option<String>,
    /// `font-list-held-while-the-default-font-is-read`.
    pub topic: String,
}

/// What `worktree-new` creates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    /// `<main checkout>/.claude/worktrees/us-061`.
    pub dir: PathBuf,
    /// `feat/us-061-<topic>`.
    pub branch: String,
    /// `origin/feature/f18-<topic>`.
    pub base: String,
}

/// A story id in capitals, `US-061`, when `text` is one: letters, a hyphen, three digits.
#[must_use]
pub fn story_id(text: &str) -> Option<String> {
    let id = text.trim().to_ascii_uppercase();
    let (kind, number) = id.split_once('-')?;
    let is_kind = !kind.is_empty() && kind.bytes().all(|byte| byte.is_ascii_uppercase());
    let is_number = number.len() == 3 && number.bytes().all(|byte| byte.is_ascii_digit());

    (is_kind && is_number).then_some(id)
}

/// The story id of a story branch, `EN-100` for `chore/en-100-git-sequences`.
#[must_use]
pub fn story_of_branch(name: &str) -> Option<String> {
    let (_, topic) = name.split_once('/')?;
    let mut words = topic.splitn(3, '-');
    let kind = words.next()?;
    let number = words.next()?;

    story_id(&format!("{kind}-{number}"))
}

/// The story of a row of `stories`, its cells `kind`, `feature` and `topic` in that order, an
/// empty feature for a story of an epic alone; `None` for a row of another shape or without topic.
#[must_use]
pub fn story_of_row(id: &str, row: &[String]) -> Option<Story> {
    let [kind, feature, topic] = row else {
        return None;
    };

    (!kind.is_empty() && !topic.is_empty()).then(|| Story {
        id: id.to_owned(),
        kind: kind.clone(),
        feature: (!feature.is_empty()).then(|| feature.clone()),
        topic: topic.clone(),
    })
}

/// The statement reading a story's row: its kind, feature and topic.
#[must_use]
pub fn story_statement(id: &str) -> Option<String> {
    is_plain(id).then(|| format!("SELECT kind, feature, topic FROM stories WHERE id = '{id}'"))
}

/// The story of an id in the records, read by the `knowledge` binary beside the running one, from
/// a folder of the repository; `None` when the records hold none or cannot be read, the reason
/// then given.
///
/// # Errors
/// When the records cannot be read: the binary missing, `AGENT_RECORDS_ROOT` unset.
pub fn recorded_story(cwd: &Path, id: &str) -> Result<Option<Story>, String> {
    let Some(statement) = story_statement(id) else {
        return Ok(None);
    };

    Ok(rows(cwd, &statement)?
        .first()
        .and_then(|row| story_of_row(id, row)))
}

/// The branch type of a story's kind: `feat` for a user story and a validation, `chore` for an
/// enabler, `docs` for research, checks, audits and reviews; `None` for a kind the pre-push and the
/// auto-merge refuse (`NAM` and `RET` since EN-123).
#[must_use]
pub fn branch_type(kind: &str) -> Option<&'static str> {
    match kind {
        "US" | "VAL" => Some("feat"),
        "EN" => Some("chore"),
        "SP" | "CHK" | "AUD" | "REV" => Some("docs"),
        _ => None,
    }
}

/// Why a branch name is refused, `None` when it is a story branch the pre-push hook accepts: the
/// one branch form of `git_hooks::branch` (AUD-025), a type of [`BRANCH_TYPES`] before its `/`.
#[must_use]
pub fn branch_refusal(name: &str) -> Option<String> {
    if name.starts_with("origin/") || name.starts_with("refs/") {
        return Some(format!(
            "`{name}` names a remote or a reference, not a local branch"
        ));
    }

    let is_story_branch = is_valid_branch(name)
        && name
            .split_once('/')
            .is_some_and(|(kind, _)| BRANCH_TYPES.contains(&kind));

    (!is_story_branch).then(|| {
        format!(
            "`{name}` is not `<type>/<story-id>-<topic>` in kebab-case, a story id of a kind the \
             pre-push accepts (docs/tooling/git-workflow.md)"
        )
    })
}

/// The feature branch on GitHub a story starts from.
///
/// # Errors
///
/// When no remote branch, or more than one, is the story's feature branch.
pub fn base_branch(feature: Option<&str>, remote_branches: &[String]) -> Result<String, String> {
    let Some(feature) = feature else {
        return Ok(DEVELOP.to_owned());
    };
    let prefix = format!("origin/feature/{}-", feature.to_ascii_lowercase());
    let found: Vec<&String> = remote_branches
        .iter()
        .filter(|name| name.starts_with(&prefix))
        .collect();

    match found.as_slice() {
        [branch] => Ok((*branch).clone()),
        [] => Err(format!(
            "no branch `{prefix}…` on GitHub: push the feature branch first"
        )),
        _ => Err(format!(
            "several branches `{prefix}…` on GitHub: delete the stale ones"
        )),
    }
}

/// Inputs of a plan, read by the tool from the arguments, the records and Git.
pub struct Request<'a> {
    /// The story id as typed, `us-061` or `US-061`.
    pub id: &'a str,
    /// A branch type typed with `--type`, for a story whose kind's default does not fit.
    pub branch_type: Option<&'a str>,
    /// A branch topic typed with `--topic`, a bubble's own (`tool-cards-and-telemetry`) rather
    /// than the story's; for an epic's spike or a review the records do not hold yet, it stands for
    /// the story's (REV-009).
    pub topic: Option<&'a str>,
    /// The story as the records hold it, `None` when they hold none of the id.
    pub story: Option<&'a Story>,
    /// Remote branches, `origin/feature/f18-<topic>`.
    pub remote_branches: &'a [String],
    /// The main checkout, parent of the Git common directory.
    pub main: &'a Path,
}

/// The worktree, branch and base of a story, or why it cannot be made.
///
/// # Errors
///
/// A malformed id, a story the records do not hold (but an epic's spike or a review given its
/// topic), a branch type unknown, a branch name refused, a base branch not found, or a worktree
/// folder already there.
pub fn plan(request: &Request, exists: impl Fn(&Path) -> bool) -> Result<Plan, String> {
    let id = story_id(request.id)
        .ok_or_else(|| format!("`{}` is not a story id such as US-061", request.id))?;
    let story = match (request.story, request.topic) {
        (Some(story), _) => story.clone(),
        (None, Some(topic)) if OUTSIDE_FEATURES.iter().any(|kind| id.starts_with(kind)) => Story {
            id: id.clone(),
            kind: id.split_once('-').map_or("", |(kind, _)| kind).to_owned(),
            feature: None,
            topic: topic.to_owned(),
        },
        (None, _) => {
            return Err(format!(
                "no story {id} in the records: write its row first, target/release/knowledge sql \"INSERT INTO stories …\""
            ));
        }
    };
    let kind = match request.branch_type {
        Some(kind) if BRANCH_TYPES.contains(&kind) => kind,
        Some(kind) => {
            return Err(format!(
                "`{kind}` is not a branch type: {}",
                BRANCH_TYPES.join(", ")
            ));
        }
        None => branch_type(&story.kind).ok_or_else(|| {
            format!(
                "no branch type for a {} story: give one with --type",
                story.kind
            )
        })?,
    };
    let lower = id.to_ascii_lowercase();
    let branch = format!("{kind}/{lower}-{}", request.topic.unwrap_or(&story.topic));

    if let Some(reason) = branch_refusal(&branch) {
        return Err(reason);
    }

    let base = base_branch(story.feature.as_deref(), request.remote_branches)?;
    let dir = request.main.join(WORKTREES).join(&lower);

    if exists(&dir) {
        return Err(format!(
            "{} already exists: remove it or name another story",
            dir.display()
        ));
    }

    Ok(Plan { dir, branch, base })
}
