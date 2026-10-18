//! A Git repository as the tools reach it, the one copy of the workspace (NAM-002, AUD-023): a
//! checkout's folders and branch read from the files Git writes, without starting Git
//! (`REF-GIT-WORKTREE`, `REF-GIT-REV-PARSE`), and a Git command's output, or why it gave none.
//! The standard library only.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

/// Where a folder's checkout keeps its Git data, read from the files Git writes rather than from
/// Git itself: a hook runs before every tool call, and starting Git costs it more than its work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkout {
    /// The checkout's folder, `git rev-parse --show-toplevel`.
    pub top: PathBuf,
    /// Its own Git folder: `.git`, or `.git/worktrees/<name>` of the main checkout for a worktree.
    pub git_dir: PathBuf,
    /// The Git folder every worktree shares, `git rev-parse --git-common-dir`.
    pub common: PathBuf,
}

/// The checkout a folder belongs to: the nearest `.git` above it, a folder for the main checkout, a
/// file `gitdir: <path>` for a linked worktree, whose `commondir` names the shared folder; `None`
/// outside a repository or for a `.git` file that cannot be read.
#[must_use]
pub fn checkout_of(folder: &Path) -> Option<Checkout> {
    let absolute = std::env::current_dir().ok()?.join(folder);
    let dot_git = absolute
        .ancestors()
        .map(|ancestor| ancestor.join(".git"))
        .find(|candidate| candidate.exists())?;
    let top = dot_git.parent()?.to_path_buf();

    if dot_git.is_dir() {
        return Some(Checkout {
            top,
            git_dir: dot_git.clone(),
            common: dot_git,
        });
    }

    let text = fs::read_to_string(&dot_git).ok()?;
    let named = text.strip_prefix("gitdir:")?.trim();
    let git_dir = PathBuf::from(lexical_join(&top.to_string_lossy(), named));
    // A submodule's Git folder has no `commondir`: it is its own common folder.
    let common = fs::read_to_string(git_dir.join("commondir")).map_or_else(
        |_| git_dir.clone(),
        |relative| PathBuf::from(lexical_join(&git_dir.to_string_lossy(), relative.trim())),
    );

    Some(Checkout {
        top,
        git_dir,
        common,
    })
}

/// The branch checked out, as `git branch --show-current` prints it: empty on a detached head.
#[must_use]
pub fn branch_of(checkout: &Checkout) -> String {
    fs::read_to_string(checkout.git_dir.join("HEAD"))
        .ok()
        .and_then(|head| {
            head.trim_end()
                .strip_prefix("ref: refs/heads/")
                .map(str::to_owned)
        })
        .unwrap_or_default()
}

/// The Git common folder of a folder, absolute: the main checkout's `.git`, a worktree's included.
#[must_use]
pub fn common_folder(folder: &Path) -> Option<String> {
    checkout_of(folder).map(|found| found.common.to_string_lossy().into_owned())
}

/// A folder joined to another and normalized without reading the disk: `.` dropped, `..` taking
/// off the folder before it.
#[must_use]
pub fn lexical_join(base: &str, folder: &str) -> String {
    let mut path = PathBuf::new();

    for component in Path::new(base).join(folder).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                path.pop();
            }
            other => path.push(other),
        }
    }
    path.to_string_lossy().into_owned()
}

/// Why a command gave no output.
#[derive(Debug)]
pub enum Failure {
    /// It could not start: its program is missing, or cannot be run.
    NotStarted {
        /// The program, `git`.
        program: String,
        /// What the system said.
        error: io::Error,
    },
    /// It ran and exited with another code than 0.
    Exited {
        /// The program and its arguments, `git rev-parse --git-common-dir`.
        words: String,
        /// Its error output, trimmed: empty when the caller let it through to the terminal.
        error: String,
    },
}

impl fmt::Display for Failure {
    /// `git: <the system's error>`, or `git <arguments>: <its error output>`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted { program, error } => write!(formatter, "{program}: {error}"),
            Self::Exited { words, error } => write!(formatter, "{words}: {error}"),
        }
    }
}

impl std::error::Error for Failure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NotStarted { error, .. } => Some(error),
            Self::Exited { .. } => None,
        }
    }
}

/// What a command prints on its standard output once it exits 0, read as UTF-8 (lossy), untrimmed;
/// otherwise why it gave none. The command is the caller's, set up as it needs (a folder, its
/// standard input and error), so that a test passes one without the inherited `GIT_*` variables
/// (`test_support::command`, EN-103).
///
/// # Errors
///
/// [`Failure::NotStarted`] when it cannot start, [`Failure::Exited`] when it exits with another
/// code than 0.
pub fn stdout_of(command: &mut Command) -> Result<String, Failure> {
    let program = command.get_program().to_string_lossy().into_owned();
    let output = command.output().map_err(|error| Failure::NotStarted {
        program: program.clone(),
        error,
    })?;

    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }

    let words = std::iter::once(program)
        .chain(
            command
                .get_args()
                .map(|argument| argument.to_string_lossy().into_owned()),
        )
        .collect::<Vec<_>>()
        .join(" ");

    Err(Failure::Exited {
        words,
        error: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    })
}
