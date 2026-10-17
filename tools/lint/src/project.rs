//! The repository the linter runs in: its root, and what the rules read from the disk beside the
//! file, read once per run and shared by the threads (ADR-0069: work done once) — folder
//! listings, the bibliography and the ADRs.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use crate::settings::ReferenceSources;

/// An entry of a folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Its name.
    pub name: String,
    /// Whether it is a folder (a link is neither a folder nor a file).
    pub is_folder: bool,
    /// Whether it is a regular file.
    pub is_file: bool,
}

/// The repository, with what was read of it.
#[derive(Debug)]
pub struct Project {
    /// Its root, absolute.
    pub root: PathBuf,
    /// Where `@see` targets are resolved.
    pub sources: ReferenceSources,
    folders: Mutex<HashMap<PathBuf, Arc<Vec<Entry>>>>,
    bibliography: OnceLock<HashMap<String, bool>>,
    decisions: OnceLock<Vec<String>>,
}

impl Project {
    /// A repository at a root, its reference sources given.
    #[must_use]
    pub fn new(root: &Path, sources: ReferenceSources) -> Self {
        Self {
            root: root.to_path_buf(),
            sources,
            folders: Mutex::new(HashMap::new()),
            bibliography: OnceLock::new(),
            decisions: OnceLock::new(),
        }
    }

    /// The entries of a folder, empty when it cannot be read.
    pub fn entries(&self, folder: &Path) -> Arc<Vec<Entry>> {
        let mut folders = self.folders.lock().unwrap_or_else(PoisonError::into_inner);

        Arc::clone(
            folders
                .entry(folder.to_path_buf())
                .or_insert_with(|| Arc::new(read_entries(folder))),
        )
    }

    /// Whether a path is a regular file.
    pub fn is_file(&self, path: &Path) -> bool {
        self.entry(path).is_some_and(|entry| entry.is_file)
    }

    /// Whether a path is a folder.
    pub fn is_folder(&self, path: &Path) -> bool {
        self.entry(path).is_some_and(|entry| entry.is_folder)
    }

    /// Whether a folder holds a barrel, `index.ts`.
    pub fn has_index(&self, folder: &Path) -> bool {
        self.is_file(&folder.join("index.ts"))
    }

    /// The entry of a path in its folder.
    fn entry(&self, path: &Path) -> Option<Entry> {
        let (Some(parent), Some(name)) = (path.parent(), path.file_name()) else {
            return None;
        };
        let name = name.to_string_lossy();

        self.entries(parent)
            .iter()
            .find(|entry| entry.name == name)
            .cloned()
    }

    /// The bibliography's identifiers, each with whether it is verified.
    pub fn bibliography(&self) -> &HashMap<String, bool> {
        self.bibliography.get_or_init(|| {
            let text = fs::read_to_string(self.root.join(self.sources.references_file))
                .unwrap_or_default();

            text.lines()
                .filter_map(|line| {
                    reference_row(line).map(|id| {
                        (
                            id.to_owned(),
                            !line.contains(self.sources.unverified_marker),
                        )
                    })
                })
                .collect()
        })
    }

    /// The file names of the ADRs.
    pub fn decisions(&self) -> &[String] {
        self.decisions.get_or_init(|| {
            self.entries(&self.root.join(self.sources.adr_directory))
                .iter()
                .map(|entry| entry.name.clone())
                .collect()
        })
    }
}

/// The entries of a folder as the disk gives them.
fn read_entries(folder: &Path) -> Vec<Entry> {
    let Ok(read) = fs::read_dir(folder) else {
        return Vec::new();
    };

    read.filter_map(Result::ok)
        .map(|entry| {
            let kind = entry.file_type().ok();

            Entry {
                name: entry.file_name().to_string_lossy().into_owned(),
                is_folder: kind.is_some_and(|kind| kind.is_dir()),
                is_file: kind.is_some_and(|kind| kind.is_file()),
            }
        })
        .collect()
}

/// The identifier of a bibliography row, ``| `REF-…` |``.
fn reference_row(line: &str) -> Option<&str> {
    let rest = line.strip_prefix('|')?.trim_start().strip_prefix('`')?;
    let id = &rest[..rest.find('`')?];
    let valid = id.strip_prefix("REF-").is_some_and(|tail| {
        !tail.is_empty()
            && tail
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-')
    });

    valid.then_some(id)
}

#[cfg(test)]
mod tests {
    use super::reference_row;

    #[test]
    fn reads_the_identifier_of_a_bibliography_row() {
        assert_eq!(reference_row("| `REF-OK-2` | x |"), Some("REF-OK-2"));
        assert_eq!(reference_row("|`REF-A`|"), Some("REF-A"));
        assert_eq!(reference_row("| `REF-a` | x |"), None);
        assert_eq!(reference_row("| `DERIV-a` | x |"), None);
        assert_eq!(reference_row("| `REF-A | x |"), None);
        assert_eq!(reference_row("text"), None);
        assert_eq!(reference_row("| `REF-` |"), None);
    }
}
