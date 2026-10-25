//! Where a repository's records live (ADR-0081, E49): one SQLite file named after the repository in
//! the folder `AGENT_RECORDS_ROOT` names, `<root>/<repo>.sqlite`, the project's name read from the
//! `origin` remote of its main checkout's Git configuration, without starting Git, so that a hook
//! and the library's server find it as cheaply as `knowledge` does.

use std::fs;
use std::path::{Path, PathBuf};

use crate::checkout_of;

/// The name of a project's file of records, `<project>.sqlite`.
#[must_use]
pub fn database_name(project: &str) -> String {
    format!("{project}.sqlite")
}

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

/// A project's file of records in the root: `<root>/<project>.sqlite`. The former place,
/// `<root>/<project>/records.sqlite`, is no longer read (AUD-053): a test's repository named `repo`
/// once left such a file in the real root.
#[must_use]
pub fn database_in(root: &Path, project: &str) -> PathBuf {
    root.join(database_name(project))
}

/// The address of the `origin` remote a Git configuration's text names, empty without one: the
/// `url` of its section `[remote "origin"]`.
#[must_use]
pub fn origin_in_config(config: &str) -> String {
    let mut inside = false;

    for line in config.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == "[remote \"origin\"]";
        } else if inside
            && let Some((key, value)) = line.split_once('=')
            && key.trim() == "url"
        {
            return value.trim().to_owned();
        }
    }
    String::new()
}

/// The records file of the repository a folder belongs to, given the records' root: `None` without
/// a root or outside a repository.
#[must_use]
pub fn records_of_folder(root: Option<&str>, folder: &Path) -> Option<PathBuf> {
    let root = root.filter(|root| !root.is_empty())?;
    let common = checkout_of(folder)?.common;
    let main = common.parent()?;
    let config = fs::read_to_string(common.join("config")).unwrap_or_default();

    Some(database_in(
        Path::new(root),
        &project_name(&origin_in_config(&config), main),
    ))
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
    fn reads_the_origins_address_in_a_git_configuration() {
        let config = "[core]\n\turl = no\n[remote \"upstream\"]\n\turl = up\n[remote \"origin\"]\n\tfetch = +refs/heads/*\n\turl = git@host:owner/studio.git\n";

        assert_eq!(origin_in_config(config), "git@host:owner/studio.git");
        assert_eq!(origin_in_config("[core]\n"), "");
    }

    #[test]
    fn names_the_file_after_the_project_never_the_former_place() {
        let root = std::env::temp_dir().join(format!("records-place-{}", std::process::id()));
        let former = root.join("studio").join("records.sqlite");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(former.parent().expect("folder")).expect("folder");
        fs::write(&former, "").expect("former");
        let found = database_in(&root, "studio");

        fs::remove_dir_all(&root).expect("cleaned");
        assert_eq!(found, root.join("studio.sqlite"));
        assert_eq!(records_of_folder(None, Path::new(".")), None);
        assert_eq!(records_of_folder(Some(""), Path::new(".")), None);
    }
}
