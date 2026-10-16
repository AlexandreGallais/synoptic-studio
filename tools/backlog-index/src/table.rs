//! The stories index as a Markdown table, and the same table read back cell by cell.

use crate::story::Story;

/// The index's heading and its table's header.
const HEADER: &str = "# Stories\n\n| ID | Feature | Story | Status |\n| --- | --- | --- | --- |\n";

/// The index of the stories, one row each, in the order given.
///
/// Cells are not padded: Prettier aligns the columns when it formats the file.
pub fn render(stories: &[&Story]) -> String {
    let rows: String = stories.iter().map(|story| row(story)).collect();

    format!("{HEADER}{rows}")
}

/// One story's row: its id linked to its file, its feature (its epic for `NAM`, `REV`, `RET`),
/// its title and its status.
fn row(story: &Story) -> String {
    format!(
        "| [{}](./{}) | {} | {} | {} |\n",
        story.id,
        story.file,
        story.parent(),
        story.title.replace('|', "\\|"),
        story.status
    )
}

/// The lines of a Markdown file as their content, whatever their padding: blank lines and a
/// table's alignment row left out, each table row as its trimmed cells.
pub fn normalized(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !is_alignment_row(line))
        .map(|line| {
            if line.starts_with('|') {
                cells(line).join(" | ")
            } else {
                line.to_owned()
            }
        })
        .collect()
}

/// The trimmed cells of a table row, an escaped `\|` kept inside its cell.
fn cells(line: &str) -> Vec<String> {
    let inner = line.trim_matches('|');
    let mut cells = vec![String::new()];
    let mut previous = ' ';

    for character in inner.chars() {
        if character == '|' && previous != '\\' {
            cells.push(String::new());
        } else if let Some(cell) = cells.last_mut() {
            cell.push(character);
        }
        previous = character;
    }

    cells.iter().map(|cell| cell.trim().to_owned()).collect()
}

/// Whether a line is a table's alignment row: `| --- | :-: |`.
fn is_alignment_row(line: &str) -> bool {
    line.starts_with('|')
        && line.contains('-')
        && line
            .chars()
            .all(|character| matches!(character, '|' | '-' | ':' | ' '))
}

#[cfg(test)]
mod tests {
    use super::{normalized, render};
    use crate::story::Story;

    fn story(id: &str, feature: &str, title: &str) -> Story {
        Story {
            id: id.to_owned(),
            epic: "E17".to_owned(),
            feature: feature.to_owned(),
            title: title.to_owned(),
            status: "ready".to_owned(),
            file: format!("E17-{id}-x.md"),
        }
    }

    #[test]
    fn writes_one_row_per_story_its_epic_for_an_epic_story() {
        let en = story("EN-049", "F18", "A | B");
        let rev = story("REV-004", "", "Review");

        assert_eq!(
            render(&[&en, &rev]),
            "# Stories\n\n| ID | Feature | Story | Status |\n| --- | --- | --- | --- |\n\
             | [EN-049](./E17-EN-049-x.md) | F18 | A \\| B | ready |\n\
             | [REV-004](./E17-REV-004-x.md) | E17 | Review | ready |\n"
        );
    }

    #[test]
    fn reads_a_padded_table_as_the_same_cells() {
        let compact = "# Stories\n\n| ID | Story |\n| --- | --- |\n| [A](./a.md) | x \\| y |\n";
        let padded = "# Stories\n\n| ID          | Story  |\n| ----------- | ------ |\n\
                      | [A](./a.md) | x \\| y |\n";

        assert_eq!(normalized(compact), normalized(padded));
        assert_eq!(
            normalized(compact),
            vec!["# Stories", "ID | Story", "[A](./a.md) | x \\| y"]
        );
    }

    #[test]
    fn tells_a_changed_cell() {
        assert_ne!(
            normalized("| [A](./a.md) | ready |\n"),
            normalized("| [A](./a.md) | done |\n")
        );
    }
}
