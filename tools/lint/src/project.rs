//! The repository the linter runs in: its root, and what the rules read from the disk beside the
//! file, read once per run and shared by the threads (ADR-0069: work done once) — folder
//! listings and the packages of `package.json`.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use crate::package::{self, Dependencies};

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
    folders: Mutex<HashMap<PathBuf, Arc<Vec<Entry>>>>,
    dependencies: OnceLock<Dependencies>,
}

impl Project {
    /// A repository at a root.
    #[must_use]
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            folders: Mutex::new(HashMap::new()),
            dependencies: OnceLock::new(),
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

    /// The packages `package.json` names, none when it cannot be read.
    pub fn dependencies(&self) -> &Dependencies {
        self.dependencies.get_or_init(|| {
            package::parse(&fs::read_to_string(self.root.join("package.json")).unwrap_or_default())
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
