//! Where a project's records live (ADR-0081): the folder `AGENT_RECORDS_ROOT` holds the records of
//! every project, each in a folder named after its repository, so that two projects never write
//! over each other; the project's one SQLite file, `records.sqlite`, sits in it (E49). Shared by
//! `knowledge`, `records-save` and `epic-check`, so that each finds the same folder.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use git_repository::{Failure, stdout_of};

/// The variable naming the folder of every project's records.
pub const ROOT_VARIABLE: &str = "AGENT_RECORDS_ROOT";

/// The project's one file of records, in its records folder.
pub const DATABASE_FILE: &str = "records.sqlite";

/// A project's name: its repository's, the last part of the `origin` remote's address without
/// `.git`, the same in every clone; without a remote, the main checkout's folder.
#[must_use]
pub fn project_name(origin: &str, main: &Path) -> String {
    let address = origin.trim().trim_end_matches('/');
    let last = address.rsplit(['/', ':']).next().unwrap_or_default();
    let name = last.strip_suffix(".git").unwrap_or(last);

    if name.is_empty() {
        main.file_name()
            .map(|folder| folder.to_string_lossy().into_owned())
            .unwrap_or_default()
    } else {
        name.to_owned()
    }
}

/// A project's records folder, `<AGENT_RECORDS_ROOT>/<project>` (ADR-0081).
///
/// # Errors
/// When the variable is not set or empty: the records are never written to a folder nobody chose.
pub fn records_folder(root: Option<&str>, project: &str) -> Result<PathBuf, String> {
    match root {
        Some(path) if !path.is_empty() => Ok(Path::new(path).join(project)),
        _ => Err(format!(
            "{ROOT_VARIABLE} is not set: name the folder of the records in the env of ~/.claude/settings.json."
        )),
    }
}

/// Runs Git in a folder, its standard error shown; its trimmed output.
///
/// # Errors
/// When Git cannot start or fails.
pub fn git(cwd: &Path, words: &[&str]) -> Result<String, String> {
    match stdout_of(
        Command::new("git")
            .args(words)
            .current_dir(cwd)
            .stdin(Stdio::null())
            .stderr(Stdio::inherit()),
    ) {
        Ok(output) => Ok(output.trim().to_owned()),
        Err(failure @ Failure::NotStarted { .. }) => Err(failure.to_string()),
        Err(Failure::Exited { words, .. }) => Err(format!("{words} failed in {}", cwd.display())),
    }
}

/// The main checkout of the repository a folder belongs to, a worktree's included, and its
/// `origin` remote's address, empty without a remote.
///
/// # Errors
/// When the folder is in no repository.
pub fn main_and_origin(cwd: &Path) -> Result<(PathBuf, String), String> {
    let common = git(
        cwd,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    let main = Path::new(&common)
        .parent()
        .map_or_else(PathBuf::new, Path::to_path_buf);
    // No remote: the project is named after its folder.
    let origin = git(&main, &["config", "--get", "remote.origin.url"]).unwrap_or_default();

    Ok((main, origin))
}

/// The records folder of a repository, given its main checkout and its `origin`:
/// `<AGENT_RECORDS_ROOT>/<repo>`, the root required to exist, so that a mistyped path never makes a
/// new one (ADR-0081).
///
/// # Errors
/// When the variable is unset or empty, or its folder does not exist.
pub fn project_records(root: Option<&str>, main: &Path, origin: &str) -> Result<PathBuf, String> {
    let folder = records_folder(root, &project_name(origin, main))?;

    match root {
        Some(root) if !Path::new(root).is_dir() => Err(format!(
            "The records' folder {root} does not exist: create it, or correct {ROOT_VARIABLE}."
        )),
        _ => Ok(folder),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_a_project_after_its_repository_else_its_folder() {
        let main = Path::new("/work/SVG editor");

        for origin in [
            "https://github.com/owner/synoptic-studio.git",
            "https://github.com/owner/synoptic-studio/",
            "git@github.com:owner/synoptic-studio.git",
            "git@host:synoptic-studio",
        ] {
            assert_eq!(project_name(origin, main), "synoptic-studio", "{origin}");
        }
        assert_eq!(project_name("", main), "SVG editor");
        assert_eq!(project_name("\n", main), "SVG editor");
    }

    #[test]
    fn puts_each_projects_records_in_its_folder_of_the_chosen_root_never_elsewhere() {
        assert_eq!(
            records_folder(Some("/records"), "synoptic-studio"),
            Ok(PathBuf::from("/records/synoptic-studio"))
        );
        for unset in [None, Some("")] {
            assert!(
                records_folder(unset, "synoptic-studio")
                    .is_err_and(|error| error.starts_with("AGENT_RECORDS_ROOT is not set"))
            );
        }
    }
}
