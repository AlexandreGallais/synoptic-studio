//! The page that held an id, when no page is named: `REF-` rows in `docs/references.md`, an ADR,
//! a derivation and a backlog item each in a file of their own. Since they moved into the records
//! (E49), a missing page sends the id to `knowledge show`, by the caller's fallback.

use std::fs;
use std::path::{Path, PathBuf};

/// Where an id lives, and what of it is printed.
#[derive(Debug, PartialEq, Eq)]
pub enum Home {
    /// A row of a table of this page.
    Row(PathBuf),
    /// The first section of this page, the id's own.
    Page(PathBuf),
}

/// The backlog's folder of an item's id, its file named with the id between dashes: a story's
/// such as `EN-105` (`E17-F21-EN-105-….md`), a feature's `F21` (`E17-F21-….md`), an epic's `E17`
/// (`E17-….md`); `None` for any other word.
#[must_use]
pub fn backlog_folder(id: &str) -> Option<&'static str> {
    let is_number = |digits: &str| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit());

    if let Some((letters, digits)) = id.split_once('-') {
        let is_kind = !letters.is_empty() && letters.bytes().all(|b| b.is_ascii_uppercase());

        return (is_kind && is_number(digits)).then_some("docs/backlog/stories");
    }

    match id.split_at_checked(1) {
        Some(("F", digits)) if is_number(digits) => Some("docs/backlog/features"),
        Some(("E", digits)) if is_number(digits) => Some("docs/backlog/epics"),
        _ => None,
    }
}

/// The file of `folder`, relative to `root`, whose name `matches`; `None` when there is none.
fn file_in(root: &Path, folder: &str, matches: impl Fn(&str) -> bool) -> Option<PathBuf> {
    let mut names: Vec<String> = fs::read_dir(root.join(folder))
        .ok()?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| Path::new(name).extension().is_some_and(|ext| ext == "md") && matches(name))
        .collect();

    names.sort();
    names.first().map(|name| Path::new(folder).join(name))
}

/// The page that holds `id`, relative to `root`, the repository's root; an error naming where it
/// was looked for when the id is of no known kind or its file is missing.
///
/// # Errors
///
/// The message to print: the id's kind unknown, or no file for it.
pub fn home(root: &Path, id: &str) -> Result<Home, String> {
    if id.starts_with("REF-") {
        return Ok(Home::Row(PathBuf::from("docs/references.md")));
    }

    if let Some(number) = id.strip_prefix("ADR-") {
        let prefix = format!("{number}-");

        return file_in(root, "docs/adr", |name| name.starts_with(&prefix))
            .map(Home::Page)
            .ok_or_else(|| format!("{id}: no file docs/adr/{number}-*.md"));
    }

    if let Some(name) = id.strip_prefix("DERIV-") {
        let file = format!("{name}.md");

        return file_in(root, "docs/derivations", |found| found == file)
            .map(Home::Page)
            .ok_or_else(|| format!("{id}: no file docs/derivations/{file}"));
    }

    if let Some(folder) = backlog_folder(id) {
        let starts = format!("{id}-");
        let inside = format!("-{id}-");

        return file_in(root, folder, |name| {
            name.starts_with(&starts) || name.contains(&inside)
        })
        .map(Home::Page)
        .ok_or_else(|| format!("{id}: no file in {folder} named with it"));
    }

    Err(format!(
        "{id}: not an id of REF-, ADR-, DERIV- nor of the backlog; name the page: target/release/page <file> {id}"
    ))
}
