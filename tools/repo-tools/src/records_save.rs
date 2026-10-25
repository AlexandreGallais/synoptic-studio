//! The save of the private records (ADR-0081, EN-114, E49): what the agent's traces hold that
//! serves — the metrics' logs, each session's times, tokens and commands, the Product Owner's own
//! messages, the agent's memories — written into the project's records file outside the
//! repository, never sent anywhere; the transcripts themselves are no longer copied.

use std::path::{Path, PathBuf};

/// Where the records live, found as `knowledge` finds them: one copy for every tool (EN-151).
pub use knowledge::records::{ROOT_VARIABLE, project_name, records_database};

/// The project's folder of Claude Code, which holds its transcripts and its memory folder: Claude
/// Code names it after the main checkout's path, every character but an ASCII letter or digit a
/// dash.
#[must_use]
pub fn claude_project_folder(main: &Path, home: &Path) -> PathBuf {
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

    home.join(".claude").join("projects").join(project)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_project_folder_claude_code_names_after_the_checkouts_path() {
        assert_eq!(
            claude_project_folder(Path::new("/work/SVG editor"), Path::new("/home/me")),
            PathBuf::from("/home/me/.claude/projects/-work-SVG-editor")
        );
    }
}
