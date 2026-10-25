//! The text files under the paths given to `check-ids` (EN-154): a file as it is, a
//! folder walked, its build, dependency and Git folders left out.

use std::fs;
use std::path::{Path, PathBuf};

/// The folders never walked: Git's, the dependencies' and the builds'.
const SKIPPED: [&str; 4] = [".git", "node_modules", "target", "dist"];

/// The extensions of the text files read.
const TEXTS: [&str; 9] = ["md", "ts", "rs", "yml", "yaml", "toml", "json", "txt", "sh"];

/// Whether a path is a test's or a fixture's, whose ids are made up on purpose (`EN-999`, a color
/// `F80`): `check-ids` reads none of them.
#[must_use]
pub fn is_test_path(path: &str) -> bool {
    path.contains(".test.")
        || path.contains("/tests/")
        || path.starts_with("tests/")
        || path.contains("/fixtures/")
}

/// Whether a file is one of the text files read, by its extension.
#[must_use]
pub fn is_text(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| TEXTS.contains(&extension))
}

/// The text files under some paths, each folder walked in the order of its sorted names; a path
/// given as a file is kept whatever its extension.
///
/// # Errors
/// When a path does not exist or a folder cannot be read, naming it.
pub fn text_files(paths: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();

    for path in paths {
        let path = PathBuf::from(path);

        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            walk(&path, &mut files)?;
        } else {
            return Err(format!("{}: no such file or folder", path.display()));
        }
    }
    Ok(files)
}

/// Adds the text files of a folder and its folders.
fn walk(folder: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(folder)
        .map_err(|error| format!("{}: {error}", folder.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    entries.sort();
    for entry in entries {
        let name = entry
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();

        if entry.is_dir() {
            if !SKIPPED.contains(&name.as_str()) {
                walk(&entry, files)?;
            }
        } else if is_text(&entry) && !is_test_path(&entry.to_string_lossy()) {
            files.push(entry);
        }
    }
    Ok(())
}
