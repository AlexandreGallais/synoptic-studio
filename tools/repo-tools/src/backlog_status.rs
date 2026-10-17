//! A backlog item's status changed in the three places `backlog.test.ts` checks (RET-002, EN-103):
//! the item's front matter, its folder's index and its parent's table — a story's feature (or its
//! epic, for `NAM`, `REV` and `RET`), a feature's epic. The stories index is not edited but
//! generated again from the story files by `npm run backlog:index` (EN-049, ADR-0065).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The text of each line, its `\r` kept apart: a line's end is never part of its value.
fn split_line(line: &str) -> (&str, &str) {
    line.strip_suffix('\r')
        .map_or((line, ""), |text| (text, "\r"))
}

/// The front matter with a new status: the first line `status: …` rewritten, the others kept.
#[must_use]
pub fn with_front_status(text: &str, status: &str) -> String {
    let mut done = false;

    text.split('\n')
        .map(|line| {
            let (body, end) = split_line(line);

            if !done && body.starts_with("status: ") {
                done = true;
                format!("status: {status}{end}")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// A table row's last cell replaced by the status: the cell between the row's last two pipes, the
/// row unchanged when it does not end with a pipe.
fn with_last_cell(line: &str, status: &str) -> String {
    let trimmed = line.trim_end();
    let Some(body) = trimmed.strip_suffix('|') else {
        return line.to_owned();
    };
    let Some(at) = body.rfind('|') else {
        return line.to_owned();
    };

    format!("{}| {status} |", &line[..at])
}

/// Tables with a new status in the row of an item: the last cell of each row linking to it
/// (`[US-030]`).
#[must_use]
pub fn with_row_status(text: &str, id: &str, status: &str) -> String {
    let link = format!("[{id}]");

    text.split('\n')
        .map(|line| {
            if line.starts_with('|') && line.contains(&link) {
                with_last_cell(line, status)
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The folder of an item from the form of its id: `epics` for `E14`, `features` for `F10`,
/// `stories` otherwise.
#[must_use]
pub fn folder_of(id: &str) -> &'static str {
    let two_digits = |rest: &str| rest.len() == 2 && rest.bytes().all(|byte| byte.is_ascii_digit());

    match id.split_at_checked(1) {
        Some(("E", rest)) if two_digits(rest) => "epics",
        Some(("F", rest)) if two_digits(rest) => "features",
        _ => "stories",
    }
}

/// The value of a front-matter field, trimmed: the first line `<key>: …`; empty when missing.
#[must_use]
pub fn front_field(text: &str, key: &str) -> String {
    let start = format!("{key}: ");

    text.lines()
        .find_map(|line| line.strip_prefix(&start))
        .map_or_else(String::new, |value| value.trim().to_owned())
}

/// Of a folder's file names, sorted, the first of an item: holding `-<id>-` or starting with
/// `<id>-`; empty when none.
#[must_use]
pub fn file_of(names: &[String], id: &str) -> String {
    let inside = format!("-{id}-");
    let first = format!("{id}-");

    names
        .iter()
        .find(|name| name.contains(&inside) || name.starts_with(&first))
        .cloned()
        .unwrap_or_default()
}

/// The sorted file names of a folder of the backlog.
///
/// # Errors
/// When the folder cannot be read.
pub fn names_in(folder: &Path) -> io::Result<Vec<String>> {
    let mut names: Vec<String> = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();

    names.sort();
    Ok(names)
}

/// A file rewritten by a change of its text.
///
/// # Errors
/// When it cannot be read or written, naming it.
pub fn rewrite(path: &Path, change: impl Fn(&str) -> String) -> io::Result<()> {
    let text = fs::read_to_string(path)
        .map_err(|error| io::Error::other(format!("{}: {error}", path.display())))?;

    fs::write(path, change(&text))
        .map_err(|error| io::Error::other(format!("{}: {error}", path.display())))
}

/// The file of the parent whose table lists an item: a story's feature, else its epic.
///
/// # Errors
/// When the parent's folder cannot be read.
pub fn parent_of(backlog: &Path, folder: &str, text: &str) -> io::Result<PathBuf> {
    let feature = front_field(text, "feature");

    if folder == "stories" && !feature.is_empty() {
        let features = backlog.join("features");

        return Ok(features.join(file_of(&names_in(&features)?, &feature)));
    }

    let epics = backlog.join("epics");

    Ok(epics.join(file_of(&names_in(&epics)?, &front_field(text, "epic"))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changes_the_front_matter_status_only() {
        assert_eq!(
            with_front_status("---\nid: US-030\nstatus: draft\n---\nstatus: kept", "done"),
            "---\nid: US-030\nstatus: done\n---\nstatus: kept"
        );
        // A line ending in `\r` keeps it; a text without the field is unchanged.
        assert_eq!(with_front_status("status: a\r\nx", "b"), "status: b\r\nx");
        assert_eq!(with_front_status("statuses: a\n", "b"), "statuses: a\n");
    }

    #[test]
    fn changes_the_last_cell_of_the_row_linking_to_the_item_not_another_row() {
        let table = [
            "| ID | Title | Status |",
            "| [US-030](./a.md) | Convert | in-progress |",
            "| [US-031](./b.md) | Other, see [US-030] | draft |",
        ]
        .join("\n");

        assert_eq!(
            with_row_status(&table, "US-031", "done"),
            [
                "| ID | Title | Status |",
                "| [US-030](./a.md) | Convert | in-progress |",
                "| [US-031](./b.md) | Other, see [US-030] | done |",
            ]
            .join("\n")
        );
        // Padding and trailing spaces go with the old cell; a row without a closing pipe stays.
        assert_eq!(
            with_row_status("| [E1](x) |  draft   |  \n", "E1", "done"),
            "| [E1](x) | done |\n"
        );
        assert_eq!(
            with_row_status("| [E1](x) draft", "E1", "done"),
            "| [E1](x) draft"
        );
        assert_eq!(with_row_status("|[E1]|", "E1", "done"), "| done |");
        assert_eq!(with_row_status("[E1] |a|", "E1", "done"), "[E1] |a|");
    }

    #[test]
    fn finds_the_folder_from_the_form_of_the_id() {
        assert_eq!(
            [
                folder_of("E14"),
                folder_of("F10"),
                folder_of("RET-002"),
                folder_of("E1"),
                folder_of("F100"),
                folder_of(""),
            ],
            [
                "epics", "features", "stories", "stories", "stories", "stories"
            ]
        );
    }

    #[test]
    fn reads_a_front_matter_field_and_finds_an_item_file() {
        assert_eq!(front_field("id: X\nfeature: F21 \n", "feature"), "F21");
        assert_eq!(front_field("id: X\n", "epic"), "");

        let names = [
            "E17-F21-EN-103-a.md".to_owned(),
            "E17-F21-EN-1030-b.md".to_owned(),
            "F21-x.md".to_owned(),
        ];

        assert_eq!(file_of(&names, "EN-103"), "E17-F21-EN-103-a.md");
        assert_eq!(file_of(&names, "F21"), "E17-F21-EN-103-a.md");
        assert_eq!(file_of(&names[1..], "F21"), "E17-F21-EN-1030-b.md");
        assert_eq!(file_of(&names, "EN-9"), "");
    }
}
