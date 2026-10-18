//! `CHANGELOG.md`: its header, a new entry written above the others, its version's reference link
//! at the file's end, and an entry read back for the pull request and the GitHub release.

/// The changelog's header: its title, then the format and the versioning it follows
/// (`REF-COMMON-CHANGELOG`, 2.1; `REF-KEEP-A-CHANGELOG`, Guiding principles).
pub const HEADER: &str = "# Changelog\n\nThe notable changes of each version of the library \
`editor` and its playground, the latest first, written as [Common Changelog](https://common-changelog.org/) \
asks; the versions follow [Semantic Versioning](https://semver.org/).\n";

/// The address of a version's GitHub release.
#[must_use]
pub fn release_address(repository: &str, version: &str) -> String {
    format!("{repository}/releases/tag/v{version}")
}

/// A version's reference link: `[0.3.0]: https://github.com/o/r/releases/tag/v0.3.0`.
#[must_use]
pub fn link_definition(repository: &str, version: &str) -> String {
    format!("[{version}]: {}", release_address(repository, version))
}

/// Whether a line defines a reference link, `[x]: address`.
#[must_use]
pub fn is_link_definition(line: &str) -> bool {
    line.starts_with('[') && line.contains("]: ")
}

/// The changelog with an entry first: the header written again, the entry above the older ones,
/// its reference link with the others at the end.
#[must_use]
pub fn with_entry(changelog: &str, entry: &str, version: &str, repository: &str) -> String {
    let releases = if changelog.starts_with("## ") {
        changelog
    } else {
        changelog
            .find("\n## ")
            .map_or("", |start| &changelog[start + 1..])
    };
    let (older, links): (Vec<&str>, Vec<&str>) =
        releases.lines().partition(|line| !is_link_definition(line));
    let older = older.join("\n");
    let older = older.trim_end();
    let mut text = format!("{HEADER}\n{entry}");

    if !older.is_empty() {
        text.push('\n');
        text.push_str(older);
        text.push('\n');
    }
    text.push('\n');
    text.push_str(&link_definition(repository, version));
    text.push('\n');
    for link in links {
        text.push_str(link);
        text.push('\n');
    }
    text
}

/// The version an entry's heading names: `0.3.0` for `## [0.3.0] - 2026-10-18`, and for
/// release-please's `## [0.2.0](…) (2026-10-14)` and `## 0.1.0 (2026-10-13)`.
#[must_use]
pub fn heading_version(heading: &str) -> Option<&str> {
    let rest = heading.strip_prefix("## ")?;
    let rest = rest.strip_prefix('[').unwrap_or(rest);
    let end = rest
        .find(|character: char| !(character.is_ascii_digit() || character == '.'))
        .unwrap_or(rest.len());

    Some(&rest[..end]).filter(|version| !version.is_empty())
}

/// An entry: its heading's line number, its heading, and each line under it with its number.
pub type Entry<'a> = (usize, &'a str, Vec<(usize, &'a str)>);

/// The entries of a changelog: each heading's line number (from 1), the heading and the lines
/// under it, the reference links left out.
#[must_use]
pub fn entries(changelog: &str) -> Vec<Entry<'_>> {
    let mut found: Vec<Entry<'_>> = Vec::new();

    for (index, line) in changelog.lines().enumerate() {
        if line.starts_with("## ") {
            found.push((index + 1, line, Vec::new()));
        } else if let Some((_, _, body)) = found.last_mut()
            && !is_link_definition(line)
        {
            body.push((index + 1, line));
        }
    }
    found
}

/// The notes of a version: its entry without its heading, the blank lines around trimmed; `None`
/// when the changelog holds no entry of that version.
#[must_use]
pub fn notes(changelog: &str, version: &str) -> Option<String> {
    let (_, _, body) = entries(changelog)
        .into_iter()
        .find(|(_, heading, _)| heading_version(heading) == Some(version))?;
    let text: Vec<&str> = body.iter().map(|(_, line)| *line).collect();

    Some(format!("{}\n", text.join("\n").trim()))
}
