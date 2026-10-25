//! What the importers of the private records share (E49, the move into the records): the count
//! of what an import wrote, and the Markdown files of a folder. The one-shot importers of the
//! Markdown that moved into the records — the backlog, the decisions, the research, the
//! derivations, the bibliography, the rules, the logs and the guide — are gone with their files
//! (AUD-057): the records hold each as rows of its tables.

use std::fs;
use std::path::Path;

/// What an import wrote: the rows of its tables, and the files it read whole, a transcript.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Moved {
    pub rows: usize,
    pub documents: usize,
}

/// The Markdown files of a folder, by name, with their text; none when it cannot be read.
#[must_use]
pub fn markdown_files(folder: &Path) -> Vec<(String, String)> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| {
                    Path::new(name)
                        .extension()
                        .is_some_and(|extension| extension == "md")
                })
                .collect()
        })
        .unwrap_or_default();

    names.sort();
    names
        .into_iter()
        .filter_map(|name| {
            let text = fs::read_to_string(folder.join(&name)).ok()?;

            Some((name, text))
        })
        .collect()
}
