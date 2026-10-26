//! `wcag/h63` of `html-validate` 11.17.0 (`src/rules/wcag/h63.ts`, REF-HTML-VALIDATE-SOURCE): a
//! `<th>` of a table not simple (its headers all in the first row, or all in the first column
//! without a `<thead>`) without a valid `scope`, reported at the `scope` or the element; strict,
//! every table.

use super::super::view::{Doc, Value};
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The valid scopes, `<th>`'s `scope` values.
const SCOPES: [&str; 4] = ["row", "col", "rowgroup", "colgroup"];

/// The rule and its option.
pub struct WcagH63 {
    /// Whether every table is checked.
    pub strict: bool,
}

impl WcagH63 {
    /// The project's configuration: the rule's default, simple tables left alone.
    pub const PROJECT: Self = Self { strict: false };
}

/// The children of an element of a tag.
fn children_of(doc: &Doc, index: usize, tag: &str) -> Vec<usize> {
    doc.child_elements(index)
        .into_iter()
        .filter(|each| doc.el(*each).tag == tag)
        .collect()
}

/// `isSimpleTable`.
fn is_simple(doc: &Doc, table: usize) -> bool {
    let mut rows: Vec<usize> = children_of(doc, table, "tr");
    for section in ["thead", "tbody"] {
        for each in children_of(doc, table, section) {
            rows.extend(children_of(doc, each, "tr"));
        }
    }
    rows.sort_unstable();
    let direct_rows = children_of(doc, table, "tr").into_iter().chain(
        children_of(doc, table, "tbody")
            .into_iter()
            .flat_map(|body| children_of(doc, body, "tr")),
    );
    let has_headers = direct_rows
        .flat_map(|row| doc.child_elements(row))
        .any(|cell| doc.has(cell, "headers"));
    if has_headers || rows.is_empty() {
        return false;
    }
    let cells: Vec<Vec<usize>> = rows
        .iter()
        .map(|row| {
            doc.child_elements(*row)
                .into_iter()
                .map(|cell| usize::from(doc.el(cell).tag == "th"))
                .collect()
        })
        .collect();
    let columns = cells[0].len();
    if columns == 0 || cells.iter().any(|row| row.len() != columns) {
        return false;
    }
    let per_row: Vec<usize> = cells.iter().map(|row| row.iter().sum()).collect();
    if per_row[0] == columns && per_row[1..].iter().all(|each| *each == 0) {
        return true;
    }
    let per_column: Vec<usize> = (0..columns)
        .map(|column| cells.iter().map(|row| row[column]).sum())
        .collect();
    per_column[0] == cells.len()
        && per_column[1..].iter().all(|each| *each == 0)
        && children_of(doc, table, "thead").is_empty()
}

impl TemplateRule for WcagH63 {
    fn name(&self) -> &'static str {
        "html-validate/wcag/h63"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        for (table, element) in doc.elements.iter().enumerate() {
            if element.tag != "table" || (!self.strict && is_simple(&doc, table)) {
                continue;
            }
            for th in doc
                .descendants(table)
                .into_iter()
                .filter(|each| doc.el(*each).tag == "th")
            {
                let scope = doc.attr(th, "scope");
                let span = match scope.map(|attr| &attr.value) {
                    Some(Value::Dynamic) => continue,
                    Some(Value::Static(value)) if SCOPES.contains(&value.as_str()) => continue,
                    Some(_) => scope
                        .map_or_else(|| doc.name_span(th), super::super::view::Attr::value_span),
                    None => doc.name_span(th),
                };
                reports.push(report(
                    span,
                    "<th> element must have a valid scope attribute: row, col, rowgroup or colgroup".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WcagH63;
    use super::is_simple;
    use crate::template::html::cases::run_with;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run_with(
            "wcag-h63.txt",
            "wcag/h63",
            &[],
            &[("isSimpleTable", &is_simple)],
            &|options| {
                Box::new(WcagH63 {
                    strict: options
                        .and_then(|options| options.flag("strict"))
                        .unwrap_or(false),
                })
            },
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (5, 28, 2),
            "{:#?}",
            tally.apart_cases
        );
    }
}
