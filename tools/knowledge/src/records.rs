//! Where a project's records live (ADR-0081, E49): the folder `AGENT_RECORDS_ROOT` holds the
//! records of every project, each one SQLite file named after its repository,
//! `<AGENT_RECORDS_ROOT>/<repo>.sqlite`, so that two projects never write over each other. Shared
//! by `knowledge`, `records-save` and `epic-check`, so that each finds the same file.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The place of the file, shared with the hooks and the library, which find it without Git.
pub use git_repository::records_place::{database_in, database_name, project_name};
use git_repository::{Failure, stdout_of};

/// The variable naming the folder of every project's records.
pub const ROOT_VARIABLE: &str = "AGENT_RECORDS_ROOT";

/// The folder of every project's records, the variable's value.
///
/// # Errors
/// When the variable is not set or empty: the records are never written to a folder nobody chose.
pub fn records_root(root: Option<&str>) -> Result<PathBuf, String> {
    match root {
        Some(path) if !path.is_empty() => Ok(PathBuf::from(path)),
        _ => Err(format!(
            "{ROOT_VARIABLE} is not set: name the folder of the records in the env of ~/.claude/settings.json."
        )),
    }
}

/// A project's file of records, given the variable's value (ADR-0081).
///
/// # Errors
/// When the variable is not set or empty.
pub fn records_database(root: Option<&str>, project: &str) -> Result<PathBuf, String> {
    Ok(database_in(&records_root(root)?, project))
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
/// The file of records of a repository, given its main checkout and its `origin`:
/// `<AGENT_RECORDS_ROOT>/<repo>.sqlite`, the root required to exist, so that a mistyped path never
/// makes a new one (ADR-0081).
///
/// # Errors
/// When the variable is unset or empty, or its folder does not exist.
pub fn project_records(root: Option<&str>, main: &Path, origin: &str) -> Result<PathBuf, String> {
    let file = records_database(root, &project_name(origin, main))?;

    match root {
        Some(root) if !Path::new(root).is_dir() => Err(format!(
            "The records' folder {root} does not exist: create it, or correct {ROOT_VARIABLE}."
        )),
        _ => Ok(file),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts_each_projects_records_in_a_file_of_its_name_in_the_chosen_root_never_elsewhere() {
        assert_eq!(
            records_database(Some("/records"), "synoptic-studio"),
            Ok(PathBuf::from("/records/synoptic-studio.sqlite"))
        );
        for unset in [None, Some("")] {
            assert!(
                records_database(unset, "synoptic-studio")
                    .is_err_and(|error| error.starts_with("AGENT_RECORDS_ROOT is not set"))
            );
        }
    }
}
