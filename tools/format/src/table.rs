//! Markdown tables without padding (ADR-0068): each cell trimmed, one space on each side of every
//! `|`, the delimiter row written `---` with its alignment colons. A table is a row starting with
//! `|` followed by a delimiter row of as many cells, then every row starting with `|` at the same
//! indent; lines inside a fenced code block are left as written.

/// Cells of a row written from its leading `|`: split at every `|` not escaped by a backslash,
/// each trimmed, the empty piece before the leading `|` and after a trailing `|` left out.
fn cells(row: &str) -> Vec<&str> {
    let mut pieces = Vec::new();
    let mut start = 0;
    let mut backslashes = 0;

    for (index, character) in row.char_indices() {
        if character == '|' && backslashes % 2 == 0 {
            pieces.push(&row[start..index]);
            start = index + 1;
        }

        backslashes = if character == '\\' {
            backslashes + 1
        } else {
            0
        };
    }

    let rest = &row[start..];

    if !rest.trim().is_empty() {
        pieces.push(rest);
    }

    pieces.into_iter().skip(1).map(str::trim).collect()
}

/// Whether a cell belongs to a delimiter row: dashes, a colon at either end for the alignment.
fn is_delimiter_cell(cell: &str) -> bool {
    let dashes = cell.strip_prefix(':').unwrap_or(cell);
    let dashes = dashes.strip_suffix(':').unwrap_or(dashes);

    !dashes.is_empty() && dashes.chars().all(|character| character == '-')
}

/// A delimiter cell written without padding: `---`, `:---`, `---:` or `:---:`.
fn delimiter_cell(cell: &str) -> String {
    let left = if cell.starts_with(':') { ":" } else { "" };
    let right = if cell.len() > 1 && cell.ends_with(':') {
        ":"
    } else {
        ""
    };

    format!("{left}---{right}")
}

/// A row written without padding, after its indent.
fn row_line(indent: &str, cells: &[String]) -> String {
    format!("{indent}| {} |", cells.join(" | "))
}

/// The leading spaces and tabs of a line, and the rest when it starts with `|`.
fn split_row(line: &str) -> Option<(&str, &str)> {
    let rest = line.trim_start_matches([' ', '\t']);

    rest.starts_with('|')
        .then(|| (&line[..line.len() - rest.len()], rest))
}

/// The fence a line opens or closes: its character and length, at least three.
fn fence(line: &str) -> Option<(char, usize)> {
    let rest = line.trim_start_matches([' ', '\t']);
    let character = rest
        .chars()
        .next()
        .filter(|first| *first == '`' || *first == '~')?;
    let length = rest.chars().take_while(|next| *next == character).count();

    (length >= 3).then_some((character, length))
}

/// Whether a line closes an open fence: the same character, as many at least, nothing after.
fn closes(line: &str, open: (char, usize)) -> bool {
    fence(line).is_some_and(|(character, length)| {
        let rest = line.trim_start_matches([' ', '\t']);

        character == open.0 && length >= open.1 && rest[length..].trim().is_empty()
    })
}

/// The table starting at a line: its indent and the index after its last row, when the line is a
/// header followed by a delimiter row of as many cells.
fn table_at(lines: &[&str], index: usize) -> Option<(usize, usize)> {
    let (indent, header) = split_row(lines[index])?;
    let (delimiter_indent, delimiter) = split_row(lines.get(index + 1)?)?;
    let delimiters = cells(delimiter);

    if delimiter_indent != indent
        || delimiters.is_empty()
        || delimiters.len() != cells(header).len()
        || !delimiters.iter().all(|cell| is_delimiter_cell(cell))
    {
        return None;
    }

    let end = (index + 2..lines.len())
        .find(|row| split_row(lines[*row]).is_none_or(|(row_indent, _)| row_indent != indent))
        .unwrap_or(lines.len());

    Some((indent.len(), end))
}

/// The lines of one table written without padding.
fn unpadded_table(rows: &[&str], indent: usize) -> Vec<String> {
    rows.iter()
        .enumerate()
        .map(|(position, row)| {
            let row_cells = cells(&row[indent..]);
            let written: Vec<String> = if position == 1 {
                row_cells.iter().map(|cell| delimiter_cell(cell)).collect()
            } else {
                row_cells.iter().map(|cell| (*cell).to_owned()).collect()
            };

            row_line(&row[..indent], &written)
        })
        .collect()
}

/// A Markdown text with every table written without padding; the other lines as they are.
#[must_use]
pub fn unpad(text: &str) -> String {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut written: Vec<String> = Vec::with_capacity(lines.len());
    let mut open: Option<(char, usize)> = None;
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];

        if let Some(opened) = open {
            open = (!closes(line, opened)).then_some(opened);
        } else if let Some((indent, end)) = table_at(&lines, index) {
            written.extend(unpadded_table(&lines[index..end], indent));
            index = end;

            continue;
        } else {
            open = fence(line);
        }

        written.push(line.to_owned());
        index += 1;
    }

    written.join("\n")
}

#[cfg(test)]
mod tests {
    use super::unpad;

    #[test]
    fn writes_a_padded_table_without_padding() {
        let padded = "| a                 |   b |\n| ----------------- | --: |\n| x                 |   1 |\n| a much wider cell |  22 |\n";

        assert_eq!(
            unpad(padded),
            "| a | b |\n| --- | ---: |\n| x | 1 |\n| a much wider cell | 22 |\n"
        );
    }

    #[test]
    fn keeps_every_alignment() {
        assert_eq!(
            unpad("| a | b | c | d |\n| :-- | :-: | --: | - |\n"),
            "| a | b | c | d |\n| :--- | :---: | ---: | --- |\n"
        );
    }

    #[test]
    fn keeps_escaped_pipes_and_code_inside_cells() {
        assert_eq!(
            unpad("| a      | b   |\n| ------ | --- |\n| `x \\| y` | \\\\ |\n"),
            "| a | b |\n| --- | --- |\n| `x \\| y` | \\\\ |\n"
        );
    }

    #[test]
    fn keeps_empty_cells() {
        assert_eq!(
            unpad("| a   | b   |\n| --- | --- |\n|     | 1   |\n"),
            "| a | b |\n| --- | --- |\n|  | 1 |\n"
        );
    }

    #[test]
    fn keeps_the_indent_of_a_table_in_a_list() {
        assert_eq!(
            unpad("1. Item:\n\n   | a   | b   |\n   | --- | --- |\n   | x   | y   |\n\n2. Next\n"),
            "1. Item:\n\n   | a | b |\n   | --- | --- |\n   | x | y |\n\n2. Next\n"
        );
    }

    #[test]
    fn leaves_paragraphs_lists_and_code_as_written() {
        let text = "A paragraph\nwritten on two lines.\n\n- item one\n  continued\n\n```md\n| a   | b   |\n| --- | --- |\n```\n\n~~~~\n| a   |\n| --- |\n```\n~~~~\n";

        assert_eq!(unpad(text), text);
    }

    #[test]
    fn leaves_a_row_without_a_delimiter_row_alone() {
        let text = "| not   | a table |\nplain text\n\n| a   | b   |\n| --- |\n";

        assert_eq!(unpad(text), text);
    }

    #[test]
    fn ends_a_table_at_its_first_line_without_a_row() {
        assert_eq!(
            unpad("| a   |\n| --- |\n| x   |\n\n| other |\n"),
            "| a |\n| --- |\n| x |\n\n| other |\n"
        );
    }

    #[test]
    fn writes_the_same_text_twice() {
        let texts = [
            "| a                 |   b |\n| ----------------- | --: |\n| x                 |   1 |\n",
            "  | a | b |\n  | :-: | - |\n  | \\| | `|` |\n",
            "",
            "|",
            "| a |\n| - |",
        ];

        for text in texts {
            let once = unpad(text);

            assert_eq!(unpad(&once), once, "{text:?}");
        }
    }
}
