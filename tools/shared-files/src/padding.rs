//! A change to a shared file that only pads or unpads its Markdown tables (ADR-0068, EN-073): the
//! formatter writing the tables without padding rewrites every table row of the logs and indexes,
//! yet adds, removes and changes nothing, so the pre-push check lets it through. A row compares
//! with its cells trimmed and the dashes of a delimiter cell collapsed, its alignment colons kept.

/// The cells of a table row, split at each `|` not escaped by a backslash, each trimmed, a
/// delimiter cell's dashes collapsed to one; `None` for a line that is not a table row.
fn row_cells(line: &str) -> Option<Vec<String>> {
    let row = line.trim();

    if !row.starts_with('|') {
        return None;
    }

    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut backslashes = 0;

    for character in row.chars().skip(1) {
        if character == '|' && backslashes % 2 == 0 {
            cells.push(cell_key(&cell));
            cell.clear();
        } else {
            cell.push(character);
        }

        backslashes = if character == '\\' {
            backslashes + 1
        } else {
            0
        };
    }

    if !cell.trim().is_empty() {
        cells.push(cell_key(&cell));
    }

    Some(cells)
}

/// A cell as compared: trimmed, and `:---:` written `:-:` whatever its number of dashes.
fn cell_key(cell: &str) -> String {
    let trimmed = cell.trim();
    let inner = trimmed.strip_prefix(':').unwrap_or(trimmed);
    let inner = inner.strip_suffix(':').unwrap_or(inner);

    if !inner.is_empty() && inner.chars().all(|character| character == '-') {
        let left = if trimmed.starts_with(':') { ":" } else { "" };
        let right = if trimmed.len() > 1 && trimmed.ends_with(':') {
            ":"
        } else {
            ""
        };

        format!("{left}-{right}")
    } else {
        trimmed.to_owned()
    }
}

/// Whether two lines are the same table row, padding aside.
fn same_row(old: &str, new: &str) -> bool {
    match (row_cells(old), row_cells(new)) {
        (Some(old_cells), Some(new_cells)) => old_cells == new_cells,
        _ => false,
    }
}

/// Whether a file's new text differs from its old one only in the padding of its table rows: as
/// many lines, each equal to its old line or the same table row, and at least one line changed.
#[must_use]
pub fn padding_only(old: &str, new: &str) -> bool {
    let old_lines: Vec<&str> = old.split('\n').collect();
    let new_lines: Vec<&str> = new.split('\n').collect();

    old != new
        && old_lines.len() == new_lines.len()
        && old_lines
            .iter()
            .zip(&new_lines)
            .all(|(old_line, new_line)| old_line == new_line || same_row(old_line, new_line))
}

#[cfg(test)]
mod tests {
    use super::padding_only;

    /// A log's table, padded as Prettier wrote it.
    const PADDED: &str = "# Log\n\n| Date       | Item          | Kind |\n| ---------- | ------------- | :--: |\n| 2026-10-17 | a row \\| one | tool |\n";

    /// The same table without padding, as `tools/format` writes it.
    const UNPADDED: &str = "# Log\n\n| Date | Item | Kind |\n| --- | --- | :---: |\n| 2026-10-17 | a row \\| one | tool |\n";

    #[test]
    fn accepts_a_table_unpadded_or_padded_again() {
        assert!(padding_only(PADDED, UNPADDED));
        assert!(padding_only(UNPADDED, PADDED));
    }

    #[test]
    fn refuses_a_changed_cell() {
        let changed = UNPADDED.replace("a row", "a new row");

        assert!(!padding_only(PADDED, &changed));
    }

    #[test]
    fn refuses_an_added_or_removed_row() {
        let added = format!("{UNPADDED}| 2026-10-18 | b | tool |\n");
        let removed = UNPADDED.replace("| 2026-10-17 | a row \\| one | tool |\n", "");

        assert!(!padding_only(PADDED, &added));
        assert!(!padding_only(PADDED, &removed));
    }

    #[test]
    fn refuses_a_changed_alignment_or_split_cell() {
        assert!(!padding_only(PADDED, &UNPADDED.replace(":---:", "---:")));
        assert!(!padding_only(
            PADDED,
            &UNPADDED.replace("a row \\| one", "a row | one")
        ));
    }

    #[test]
    fn refuses_a_change_outside_tables_and_no_change() {
        assert!(!padding_only(PADDED, &UNPADDED.replace("# Log", "# Logs")));
        assert!(!padding_only("a  b\n", "a b\n"));
        assert!(!padding_only(PADDED, PADDED));
    }
}
