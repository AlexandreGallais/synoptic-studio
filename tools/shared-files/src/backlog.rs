//! The backlog's files found by an item's id, the one finder of `move-rows`, `story-finish`,
//! `backlog-status` and `worktree-new` (AUD-025): `EN-137` names
//! `E17-F23-EN-137-the-log-rows-in-the-done-commit.md`, `F23` its feature's file.

use std::fs;
use std::io;
use std::path::Path;

/// The folder of the story files, from a checkout's root.
pub const STORIES: &str = "docs/backlog/stories";

/// Of a folder's file names, the first holding `-<id>-` or starting with `<id>-`; `None` when
/// none does.
#[must_use]
pub fn file_of(names: &[String], id: &str) -> Option<String> {
    let inside = format!("-{id}-");
    let first = format!("{id}-");

    names
        .iter()
        .find(|name| name.contains(&inside) || name.starts_with(&first))
        .cloned()
}

/// The file names of a folder, sorted.
///
/// # Errors
///
/// When the folder cannot be read.
pub fn names_in(folder: &Path) -> io::Result<Vec<String>> {
    let mut names: Vec<String> = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();

    names.sort();
    Ok(names)
}

/// The story file of an id in a checkout, its name in [`STORIES`]; `None` when there is none.
///
/// # Errors
///
/// When the folder of the stories cannot be read.
pub fn story_file(root: &Path, id: &str) -> io::Result<Option<String>> {
    Ok(file_of(&names_in(&root.join(STORIES))?, id))
}
