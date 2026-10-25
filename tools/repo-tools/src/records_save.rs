//! The save of the private records (ADR-0081, EN-114): the agent's traces, which hold the Product
//! Owner's raw messages and local paths, copied into a local folder outside the repository, never
//! sent anywhere. The folder `AGENT_RECORDS_ROOT` holds the records of every project, each in a
//! folder named after its repository, so that two projects never write over each other.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// How a record is copied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A file.
    File,
    /// A folder, files deleted at the source kept.
    Folder,
    /// A SQLite database, copied consistently by `VACUUM INTO`.
    Database,
}

/// One record to save: its source, how it is copied, its place in the records folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordCopy {
    pub from: PathBuf,
    pub kind: Kind,
    pub to: &'static str,
}

/// Where the records live, found as `knowledge` finds them: one copy for every tool (EN-151).
pub use knowledge::records::{ROOT_VARIABLE, project_name, records_folder};

/// The file of a project's records holding the instant of its last save.
pub const LAST_SAVE: &str = "last-save.txt";

/// The records to save; a new kind of private file is one more line here. Claude Code names a
/// project's folder after its path, every character but an ASCII letter or digit a dash.
#[must_use]
pub fn record_copies(main: &Path, home: &Path, library: &Path) -> Vec<RecordCopy> {
    let metrics = main.join(".git").join("metrics");
    let project: String = main
        .to_string_lossy()
        .chars()
        .map(|char| {
            if char.is_ascii_alphanumeric() {
                char
            } else {
                '-'
            }
        })
        .collect();

    vec![
        RecordCopy {
            from: metrics.join("tool-calls.jsonl"),
            kind: Kind::File,
            to: "metrics/tool-calls.jsonl",
        },
        RecordCopy {
            from: metrics.join("turns.jsonl"),
            kind: Kind::File,
            to: "metrics/turns.jsonl",
        },
        RecordCopy {
            from: home.join(".claude").join("projects").join(project),
            kind: Kind::Folder,
            to: "transcripts",
        },
        RecordCopy {
            from: library.to_path_buf(),
            kind: Kind::Database,
            to: "library/library.sqlite",
        },
    ]
}

/// Copies a file over another, its modification time kept; a copy already holding the same size
/// and time is left as it is, so that the transcripts' gigabytes are not written again at each
/// save.
///
/// # Errors
/// When the file cannot be read or written.
pub fn copy_file(from: &Path, to: &Path) -> io::Result<()> {
    let source = fs::metadata(from)?;
    let modified = source.modified()?;

    if let Ok(copy) = fs::metadata(to)
        && copy.is_file()
        && copy.len() == source.len()
        && copy.modified().ok() == Some(modified)
    {
        return Ok(());
    }

    fs::copy(from, to)?;
    fs::File::options()
        .write(true)
        .open(to)?
        .set_modified(modified)
}

/// Copies a folder into another, merging: each file copied over, files the source no longer holds
/// kept.
///
/// # Errors
/// When a file or folder cannot be read or written.
pub fn copy_folder(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_folder(&entry.path(), &target)?;
        } else {
            copy_file(&entry.path(), &target)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_the_metrics_the_transcripts_of_this_project_and_the_library() {
        let copies = record_copies(
            Path::new("/work/SVG editor"),
            Path::new("/home/me"),
            Path::new("/data/library.sqlite"),
        );

        assert_eq!(
            copies,
            [
                RecordCopy {
                    from: PathBuf::from("/work/SVG editor/.git/metrics/tool-calls.jsonl"),
                    kind: Kind::File,
                    to: "metrics/tool-calls.jsonl",
                },
                RecordCopy {
                    from: PathBuf::from("/work/SVG editor/.git/metrics/turns.jsonl"),
                    kind: Kind::File,
                    to: "metrics/turns.jsonl",
                },
                // Claude Code names a project's folder after its path, every other character a
                // dash.
                RecordCopy {
                    from: PathBuf::from("/home/me/.claude/projects/-work-SVG-editor"),
                    kind: Kind::Folder,
                    to: "transcripts",
                },
                RecordCopy {
                    from: PathBuf::from("/data/library.sqlite"),
                    kind: Kind::Database,
                    to: "library/library.sqlite",
                },
            ]
        );
    }

    #[test]
    fn copies_a_folder_keeping_times_and_files_gone_at_the_source() {
        let root = std::env::temp_dir().join(format!("records-copy-{}", std::process::id()));
        let (from, to) = (root.join("from"), root.join("to"));

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(from.join("sub")).expect("folders");
        fs::create_dir_all(&to).expect("to");
        fs::write(from.join("a.jsonl"), "a\n").expect("a");
        fs::write(from.join("sub/b.jsonl"), "b\n").expect("b");
        fs::write(to.join("old.jsonl"), "old\n").expect("old");
        copy_folder(&from, &to).expect("copied");

        let time = |path: &Path| fs::metadata(path).and_then(|data| data.modified()).ok();
        let same_time = time(&from.join("a.jsonl")) == time(&to.join("a.jsonl"));
        let b = fs::read_to_string(to.join("sub/b.jsonl")).expect("b");
        let old = to.join("old.jsonl").exists();

        // A file changed at the source, its size the same: copied again since its time changed.
        fs::write(from.join("a.jsonl"), "c\n").expect("a");
        copy_folder(&from, &to).expect("copied again");

        let a = fs::read_to_string(to.join("a.jsonl")).expect("a");

        fs::remove_dir_all(&root).expect("cleaned");
        assert!(same_time);
        assert_eq!(b, "b\n");
        assert!(old);
        assert_eq!(a, "c\n");
    }
}
