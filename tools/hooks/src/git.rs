//! Git and the hook's input, as the ported hooks read them (EN-104).

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::guard::lexical_join;
use crate::json::{self, Value};

/// Git's output in a folder, trimmed; `None` when it cannot run or fails, its message dropped: a
/// hook that exits 0 shows Claude no error output. The one Git call of the hooks (AUD-023).
#[must_use]
pub fn git(folder: &Path, arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(folder)
        .output()
        .ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

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

/// The hook's input, the JSON on its standard input; `None` when it is not JSON, said on the
/// hook's error output: the hook then exits 1, an error that lets the call through, as the
/// TypeScript hooks did when `JSON.parse` threw.
#[must_use]
pub fn hook_input(hook: &str) -> Option<Value> {
    let mut text = String::new();
    let input = io::stdin()
        .read_to_string(&mut text)
        .ok()
        .and_then(|_| json::parse(&text));

    if input.is_none() {
        eprintln!("{hook}: the hook's input is not JSON");
    }
    input
}
