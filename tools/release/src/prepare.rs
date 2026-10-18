//! `release prepare`: the next version from the commits since the last tag, and the three files it
//! writes, `package.json`, `package-lock.json` and `CHANGELOG.md` with the entry's draft.

use std::fs;
use std::path::Path;

use crate::backlog::backlog;
use crate::changelog::{entries, heading_version, with_entry};
use crate::draft::{bump, entry, hints, lines};
use crate::history::{Run, commits, last_release};
use crate::manifest::{repository, version, with_lock_version, with_package_version};
use crate::version::Version;

/// What `release prepare` writes and says.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prepared {
    /// The version of the release.
    pub version: Version,
    /// The tag it follows, `v0.2.0`.
    pub previous: String,
    /// `package.json`, its version set.
    pub package: String,
    /// `package-lock.json`, its two versions set.
    pub lock: String,
    /// `CHANGELOG.md`, the entry's draft first.
    pub changelog: String,
    /// The commits behind each line, for the agent rewriting them.
    pub hints: String,
    /// The number of lines drafted.
    pub lines: usize,
}

/// A file of the repository, read.
fn read(root: &Path, file: &str) -> Result<String, String> {
    fs::read_to_string(root.join(file)).map_err(|error| format!("{file}: {error}"))
}

/// The release prepared from the repository's head, on a date `YYYY-MM-DD`.
///
/// # Errors
///
/// A file or the history cannot be read, `package.json` disagrees with the last tag, no commit
/// came since it, or the changelog already holds the version.
pub fn prepare(root: &Path, run: &mut Run<'_>, date: &str) -> Result<Prepared, String> {
    let package = read(root, "package.json")?;
    let lock = read(root, "package-lock.json")?;
    let changelog = fs::read_to_string(root.join("CHANGELOG.md")).unwrap_or_default();
    let address = repository(&package).ok_or("package.json names no repository url".to_owned())?;
    let (previous, current) = last_release(run, "HEAD")?;
    let written = version(&package).unwrap_or_default();

    if written != current.to_string() {
        return Err(format!(
            "package.json says {written}, the last tag {previous}: they must agree"
        ));
    }

    let released = commits(run, &format!("{previous}..HEAD"))?;

    if released.is_empty() {
        return Err(format!("no commit since {previous}: nothing to release"));
    }

    let backlog = backlog(root);
    let drafted = lines(&released, &backlog);
    let next = current.bumped(bump(&drafted));
    let next_text = next.to_string();

    if entries(&changelog)
        .iter()
        .any(|(_, heading, _)| heading_version(heading) == Some(next_text.as_str()))
    {
        return Err(format!("CHANGELOG.md already holds {next_text}"));
    }

    Ok(Prepared {
        version: next,
        previous,
        package: with_package_version(&package, &next_text)
            .ok_or("package.json holds no version".to_owned())?,
        lock: with_lock_version(&lock, &next_text)
            .ok_or("package-lock.json holds no root package version".to_owned())?,
        changelog: with_entry(
            &changelog,
            &entry(&next_text, date, &drafted, &address),
            &next_text,
            &address,
        ),
        hints: hints(&drafted, &backlog),
        lines: drafted.len(),
    })
}
