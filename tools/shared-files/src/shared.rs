//! Which files are shared, which branches are a worker's, and which pushed files are refused.

use std::path::Path;

/// The kinds of shared file (ADR-0065, decision 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The stories, features and epics indexes of the backlog.
    Index,
    /// An append-only log of `docs/process/`.
    Log,
    /// A feature's file, holding its stories' table.
    Feature,
    /// An epic's checklist in the playground.
    Checklist,
}

/// The backlog's indexes.
const INDEXES: [&str; 3] = [
    "docs/backlog/stories/README.md",
    "docs/backlog/features/README.md",
    "docs/backlog/epics/README.md",
];

/// The logs.
pub const LOGS: [&str; 3] = [
    "docs/process/journal.md",
    "docs/process/improvements.md",
    "docs/process/feedback.md",
];

/// The story types a worker implements in a run (ADR-0063); the others are the main thread's.
const WORKER_TYPES: [&str; 2] = ["us", "en"];

/// The kind of a shared file, from its path relative to the repository's root.
#[must_use]
pub fn shared_kind(path: &str) -> Option<Kind> {
    if INDEXES.contains(&path) {
        return Some(Kind::Index);
    }

    if LOGS.contains(&path) {
        return Some(Kind::Log);
    }

    if let Some(name) = path.strip_prefix("docs/backlog/features/") {
        let markdown = Path::new(name)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));

        return (name.starts_with('E') && name.contains("-F") && markdown).then_some(Kind::Feature);
    }

    path.strip_prefix("playground/checklists/epic-")
        .is_some()
        .then_some(Kind::Checklist)
}

/// The story id of a story branch (`<type>/<id>-<topic>`, ADR-0038), `chore/en-050-topic` giving
/// `EN-050`; `None` for `develop`, `main` or a feature branch.
#[must_use]
pub fn story_of(branch: &str) -> Option<String> {
    let name = branch.strip_prefix("refs/heads/").unwrap_or(branch);
    let (_, story) = name.split_once('/')?;
    let (kind, rest) = story.split_once('-')?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    let after = &rest[digits.len()..];
    let letters = !kind.is_empty() && kind.chars().all(|letter| letter.is_ascii_lowercase());

    (letters && !digits.is_empty() && (after.is_empty() || after.starts_with('-')))
        .then(|| format!("{}-{digits}", kind.to_ascii_uppercase()))
}

/// The story id of a worker's branch: a story branch of a type a worker implements (`US`, `EN`);
/// `None` for any other branch, those of the stories the main thread writes included.
#[must_use]
pub fn worker_story(branch: &str) -> Option<String> {
    story_of(branch).filter(|story| {
        story
            .split_once('-')
            .is_some_and(|(kind, _)| WORKER_TYPES.contains(&kind.to_ascii_lowercase().as_str()))
    })
}

/// A pushed commit: its message and the files it changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    /// Its abbreviated hash, to name it.
    pub hash: String,
    /// Its whole message.
    pub message: String,
    /// The files it changes, relative to the repository's root.
    pub files: Vec<String>,
}

/// Whether a commit is the story's `done` commit (`Closes: <ID>`), written by the main thread.
#[must_use]
pub fn closes(commit: &Commit, story: &str) -> bool {
    commit
        .message
        .lines()
        .filter_map(|line| line.strip_prefix("Closes:"))
        .flat_map(|ids| ids.split(','))
        .any(|id| id.trim() == story)
}

/// The shared files a commit of a worker's branch may not change: all of them, except, in the
/// story's `done` commit, its status in the indexes and its feature's table.
#[must_use]
pub fn refused_files(commit: &Commit, story: &str) -> Vec<String> {
    let done = closes(commit, story);

    commit
        .files
        .iter()
        .filter(|path| match shared_kind(path) {
            None => false,
            Some(Kind::Index | Kind::Feature) => !done,
            Some(Kind::Log | Kind::Checklist) => true,
        })
        .cloned()
        .collect()
}

/// The refusal printed for a branch's commits, empty when none changes a shared file.
#[must_use]
pub fn refusal(branch: &str, story: &str, commits: &[Commit]) -> String {
    let lines: Vec<String> = commits
        .iter()
        .flat_map(|commit| {
            refused_files(commit, story)
                .into_iter()
                .map(move |path| format!("  {path} (commit {})", commit.hash))
        })
        .collect();

    if lines.is_empty() {
        return String::new();
    }

    format!(
        "pre-push: {branch} is {story}'s branch and edits shared files (ADR-0065):\n{}\n\
         Write their rows under `## For the main thread` in the story file instead; the main \
         thread writes them at merge. Undo the change, e.g. `git restore --source=origin/<feature \
         branch> -- <file>`, and commit.\n",
        lines.join("\n")
    )
}
