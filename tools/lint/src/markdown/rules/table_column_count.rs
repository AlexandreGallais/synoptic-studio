//! `markdown/table-column-count`, `checkMissingCells`: each row of a table has as many cells as
//! its header; an extra cell is dropped by the renderer, a missing one is often a forgotten
//! value.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, children, nodes, range, start};

/// The rule.
pub struct TableColumnCount;

impl MarkdownRule for TableColumnCount {
    fn name(&self) -> &'static str {
        "markdown/table-column-count"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for table in nodes(&page.root) {
            if !matches!(table, Node::Table(_)) {
                continue;
            }
            let rows = children(table);
            let Some(header) = rows.first() else { continue };
            let expected = children(header).len();

            for row in &rows[1..] {
                let cells = children(row);
                let actual = cells.len();
                if actual > expected {
                    findings.push(page.finding(
                        self.name(),
                        start(&cells[expected]),
                        format!(
                            "Table column count mismatch (Expected: {expected}, Actual: {actual}), extra data starting here will be ignored."
                        ),
                    ));
                } else if actual < expected {
                    let last = cells.last().map_or(range(row).0 + 1, |cell| range(cell).1);
                    findings.push(page.finding(
                        self.name(),
                        last - 1,
                        format!(
                            "Table column count mismatch (Expected: {expected}, Actual: {actual}), row might be missing data."
                        ),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TableColumnCount;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_extra_and_missing_cells() {
        // The documented cases: an extra cell starts at column 23; a missing one is reported from
        // the character before the last cell's end, column 17.
        let page = "| Head1 | Head2 |\n| ----- | ----- |\n| R1C1  | R1C2  | R2C3  |\n\n| A |\n| - |\n| 1 | 2 |\n\n| Col A | Col B | Col C |\n| ----- | ----- | ----- |\n| 4     | 5     |\n";

        assert_eq!(
            places(&TableColumnCount, page),
            [
                (3, 17, "Table column count mismatch (Expected: 2, Actual: 3), extra data starting here will be ignored.".to_owned()),
                (7, 5, "Table column count mismatch (Expected: 1, Actual: 2), extra data starting here will be ignored.".to_owned()),
                (11, 17, "Table column count mismatch (Expected: 3, Actual: 2), row might be missing data.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_full_rows_and_empty_cells() {
        let page = "| Header | Header |\n| ------ | ------ |\n| Cell   | Cell   |\n\n| Col A | Col B | Col C |\n| ----- | ----- | ----- |\n| 1     |       | 3     |\n";

        assert_eq!(messages(&TableColumnCount, page), NONE);
    }
}
