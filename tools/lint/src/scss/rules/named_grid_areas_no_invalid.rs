//! `named-grid-areas-no-invalid` of Stylelint 17.16.0
//! (`lib/rules/named-grid-areas-no-invalid/index.mjs` and its
//! `utils/findNotContiguousOrRectangular.mjs`, REF-STYLELINT-SOURCE): the strings of a `grid`,
//! `grid-template` or `grid-template-areas` declaration, an empty one, rows of different lengths,
//! or a name whose columns differ from row to row, reported over the string.

use super::{Context, Report, SheetRule, is_standard_value};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct NamedGridAreasNoInvalid;

/// The names of the areas whose columns differ between two rows naming them
/// (`findNotContiguousOrRectangular`).
fn misshapen(rows: &[Vec<String>]) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for name in rows.iter().flatten() {
        if name != "." && !names.contains(name) {
            names.push(name.clone());
        }
    }
    names
        .into_iter()
        .filter(|name| {
            let columns: Vec<Vec<usize>> = rows
                .iter()
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .filter(|(_, each)| *each == name)
                        .map(|(at, _)| at)
                        .collect()
                })
                .collect();
            let present: Vec<&Vec<usize>> =
                columns.iter().filter(|each| !each.is_empty()).collect();
            present.windows(2).any(|pair| pair[0] != pair[1])
                || present.iter().any(|each| *each != present[0])
        })
        .collect()
}

impl SheetRule for NamedGridAreasNoInvalid {
    fn name(&self) -> &'static str {
        "stylelint/named-grid-areas-no-invalid"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expectedToken", "Expected cell token within string"),
            (
                "expectedSameNumber",
                "Expected same number of cell tokens in each string",
            ),
            (
                "expectedRectangle",
                "Expected single filled-in rectangle for \"{}\"",
            ),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let property = node.prop.to_ascii_lowercase();
            let value = &node.value;
            if !matches!(
                property.as_str(),
                "grid" | "grid-template" | "grid-template-areas"
            ) || !is_standard_value(value)
                || value.trim().eq_ignore_ascii_case("none")
            {
                continue;
            }
            let start = sheet.value_start(node);
            let mut areas: Vec<(Vec<String>, usize, usize)> = Vec::new();
            let mut sent = false;
            value::parse(value).walk(&mut |_, part| {
                if part.kind != Part::String {
                    return true;
                }
                if part.value.is_empty() {
                    reports.push(Report::new(
                        start + part.start,
                        start + part.end,
                        "expectedToken",
                    ));
                    sent = true;
                    return true;
                }
                areas.push((
                    part.value.split_whitespace().map(str::to_owned).collect(),
                    part.start,
                    part.end,
                ));
                true
            });
            let Some(first) = areas.first() else {
                continue;
            };
            if sent {
                continue;
            }
            if let Some((_, from, to)) = areas.iter().find(|area| area.0.len() != first.0.len()) {
                reports.push(Report::new(start + from, start + to, "expectedSameNumber"));
                continue;
            }
            let rows: Vec<Vec<String>> = areas.iter().map(|area| area.0.clone()).collect();
            let mut names = misshapen(&rows);
            names.sort();
            for name in names {
                if let Some((_, from, to)) = areas.iter().find(|area| area.0.contains(&name)) {
                    reports.push(
                        Report::new(start + from, start + to, "expectedRectangle").with(&[&name]),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NamedGridAreasNoInvalid, misshapen};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/named-grid-areas-no-invalid.txt",
            &|_| Ok(Box::new(NamedGridAreasNoInvalid)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (14, 12, 1),
            "{:#?}",
            tally.apart
        );
        let row = |text: &str| text.split(' ').map(str::to_owned).collect::<Vec<_>>();
        assert_eq!(misshapen(&[row("a a"), row("b a")]), ["a"]);
    }
}
