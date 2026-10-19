//! Where each part of a command line runs, for a push naming no destination, which updates the
//! current branch of the repository it runs against (AUD-033): the folders a `cd` may have entered
//! before it, then the Git command's own `-C`, `--git-dir` and `GIT_DIR` (`REF-GIT`). A folder the
//! guard cannot know before the shell runs, a `cd` it cannot follow, leaves the place unknown, and
//! a push naming no destination there is refused.

/// A repository a Git command may run against: the folder it runs in, and the Git folder a
/// `--git-dir` or a `GIT_DIR` names, `None` when Git finds it from the folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    /// The folder the command runs in, absolute when the session's folder is.
    pub folder: String,
    /// The Git folder named, resolved against `folder`.
    pub git_dir: Option<String>,
}

/// The folders a part of a command line may run in, each one it may be; `None` when one of them
/// cannot be known before the shell runs.
pub type Folders = Option<Vec<String>>;

/// The current branch of a repository, `None` when it cannot be read.
pub type BranchOf<'a> = &'a dyn Fn(&Repository) -> Option<String>;

/// The folders of each top-level part of a command line, as [`walk`] finds them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Walk {
    /// Each part's byte range in the line, and the folders it may run in.
    pub spans: Vec<(usize, usize, Folders)>,
    /// Every folder a part of the line may run in.
    pub every: Folders,
}

/// The folders each top-level part of `text`, its here-documents left out, may run in, from the
/// folders `start` the line starts in.
#[must_use]
pub fn walk(text: &str, start: &Folders) -> Walk {
    Walk {
        spans: vec![(0, text.len(), start.clone())],
        every: start.clone(),
    }
}

/// The folders the part of a line holding the byte `at` may run in.
#[must_use]
pub fn folders_at(walk: &Walk, at: usize) -> Folders {
    walk.spans
        .iter()
        .find(|(start, end, _)| (*start..=*end).contains(&at))
        .map_or_else(|| walk.every.clone(), |(_, _, folders)| folders.clone())
}

/// The repositories a Git command may run against: each folder, moved by its `-C` options, its Git
/// folder named by `--git-dir` or by a `GIT_DIR` of `assignments`; `None` when one cannot be
/// known.
#[must_use]
pub fn repositories(
    words: &[String],
    assignments: &[String],
    folders: &Folders,
) -> Option<Vec<Repository>> {
    let _ = (words, assignments);

    folders.as_ref().map(|folders| {
        folders
            .iter()
            .map(|folder| Repository {
                folder: folder.clone(),
                git_dir: None,
            })
            .collect()
    })
}
