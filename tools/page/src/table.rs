//! The rows of a page's tables whose first cell is an id.

use crate::fence::fenced;

/// The text of a row's first cell, without the backticks around it nor its link: `` `REF-X` ``
/// and `[EN-092](./file.md)` both give the id. `None` for a line that is not a table row.
#[must_use]
pub fn first_cell(line: &str) -> Option<&str> {
    let cell = line.trim().strip_prefix('|')?.split('|').next()?.trim();
    let unlinked = cell
        .strip_prefix('[')
        .and_then(|rest| rest.split_once("]("))
        .map_or(cell, |(text, _)| text);

    Some(unlinked.trim_matches('`'))
}

/// Each row whose first cell is `id`, preceded by the header line of its table, its first line;
/// a row that is itself a header is given once. Rows inside a fenced code block are skipped.
#[must_use]
pub fn rows(text: &str, id: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let in_fence = fenced(&lines);
    let mut header: Option<&str> = None;
    let mut found = Vec::new();

    for (line, fenced) in lines.iter().zip(in_fence) {
        let is_row = !fenced && line.trim_start().starts_with('|');

        if !is_row {
            header = None;
            continue;
        }

        let top = *header.get_or_insert(line);

        if first_cell(line) == Some(id) {
            found.push(if top == *line {
                format!("{line}\n")
            } else {
                format!("{top}\n{line}\n")
            });
        }
    }

    found
}
