//! A worktree named after its story (EN-052, `docs/tooling/agent-tools.md`): `US-061` gets
//! `.claude/worktrees/us-061` on the branch `feat/us-061-<topic>`, made from its feature branch on
//! GitHub. The topic comes from the story's file name, the branch type from the story's kind, the
//! feature branch from the remote branches; a story whose file the checkout lacks, from the one
//! feature branch of `origin` holding it (EN-118). What the tool runs is decided here, without Git.

pub mod seed;

use std::path::{Path, PathBuf};

use git_hooks::branch::is_valid_branch;
pub use shared_files::backlog::STORIES;
use shared_files::backlog::{file_of, names_in};

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

/// A story as its file name gives it.
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

/// The story named by a file of `docs/backlog/stories/`, `E17-F18-US-061-<topic>.md`.
#[must_use]
pub fn story_of_file(id: &str, file: &str) -> Option<Story> {
    let stem = file.strip_suffix(".md")?;
    let (epic, rest) = stem.split_once('-')?;
    let is_epic = epic.len() == 3 && epic.starts_with('E');
    let (feature, rest) = match rest.split_once('-') {
        Some((feature, after)) if feature.len() == 3 && feature.starts_with('F') => {
            (Some(feature.to_owned()), after)
        }
        _ => (None, rest),
    };
    let topic = rest.strip_prefix(id)?.strip_prefix('-')?;
    let (kind, _) = id.split_once('-')?;

    (is_epic && !topic.is_empty()).then(|| Story {
        id: id.to_owned(),
        kind: kind.to_owned(),
        feature,
        topic: topic.to_owned(),
    })
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

/// Inputs of a plan, read by the tool from the arguments, the story files and Git.
pub struct Request<'a> {
    /// The story id as typed, `us-061` or `US-061`.
    pub id: &'a str,
    /// A branch type typed with `--type`, for a story whose kind's default does not fit.
    pub branch_type: Option<&'a str>,
    /// A branch topic typed with `--topic`, a bubble's own (`tool-cards-and-telemetry`) rather
    /// than the story file's; for an epic's spike or a review whose file no branch holds yet, it
    /// stands for the file's (REV-009).
    pub topic: Option<&'a str>,
    /// Names of the files of `docs/backlog/stories/`.
    pub story_files: &'a [String],
    /// Remote branches, `origin/feature/f18-<topic>`.
    pub remote_branches: &'a [String],
    /// Each feature branch of `origin` with the names of its story files (`git ls-tree`), where a
    /// story missing from the checkout is looked for: the main checkout on `develop` lacks the
    /// stories written on a feature branch.
    pub branch_files: &'a [(String, Vec<String>)],
    /// The main checkout, parent of the Git common directory.
    pub main: &'a Path,
}

/// The story of `id` on the one feature branch of `origin` holding its file, and that branch.
///
/// # Errors
///
/// When no branch holds its file, or several do, naming them.
pub fn story_on_branches(
    id: &str,
    branch_files: &[(String, Vec<String>)],
) -> Result<(Story, String), String> {
    let found: Vec<(Story, &String)> = branch_files
        .iter()
        .filter_map(|(branch, files)| {
            file_of(files, id)
                .and_then(|file| story_of_file(id, &file))
                .map(|story| (story, branch))
        })
        .collect();

    match found.as_slice() {
        [(story, branch)] => Ok((story.clone(), (*branch).clone())),
        [] => Err(format!(
            "no file `E…-{id}-<topic>.md` in docs/backlog/stories/ nor on a feature branch of origin"
        )),
        several => Err(format!(
            "`{id}` is on several feature branches of origin: {}: keep its file on one",
            several
                .iter()
                .map(|(_, branch)| branch.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// The worktree, branch and base of a story, or why it cannot be made.
///
/// # Errors
///
/// A malformed id, a story whose file neither the checkout nor one feature branch holds, a branch
/// type unknown, a branch name refused, a base branch not found, or a worktree folder already
/// there.
pub fn plan(request: &Request, exists: impl Fn(&Path) -> bool) -> Result<Plan, String> {
    let id = story_id(request.id)
        .ok_or_else(|| format!("`{}` is not a story id such as US-061", request.id))?;
    let (story, holder) = if let Some(story) =
        file_of(request.story_files, &id).and_then(|file| story_of_file(&id, &file))
    {
        (story, None)
    } else {
        match (story_on_branches(&id, request.branch_files), request.topic) {
            (Ok((story, branch)), _) => (story, Some(branch)),
            (Err(_), Some(topic)) if OUTSIDE_FEATURES.iter().any(|kind| id.starts_with(kind)) => (
                Story {
                    id: id.clone(),
                    kind: id.split_once('-').map_or("", |(kind, _)| kind).to_owned(),
                    feature: None,
                    topic: topic.to_owned(),
                },
                None,
            ),
            (Err(reason), _) => return Err(reason),
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

    let base = match holder {
        Some(branch) => branch,
        None => base_branch(story.feature.as_deref(), request.remote_branches)?,
    };
    let dir = request.main.join(WORKTREES).join(&lower);

    if exists(&dir) {
        return Err(format!(
            "{} already exists: remove it or name another story",
            dir.display()
        ));
    }

    Ok(Plan { dir, branch, base })
}

/// Names of the story files of a checkout, sorted, the only reading of the disk here:
/// `worktree-new`, `git-sequences sync` and `check-handback` each list them (AUD-022), by the
/// backlog's one finder (`shared_files::backlog`, AUD-025).
///
/// # Errors
///
/// When the folder cannot be read: a checkout without a backlog.
pub fn story_files(top: &Path) -> std::io::Result<Vec<String>> {
    names_in(&top.join(STORIES))
}
