//! `map-keys-quotes` of stylelint-scss 7.3.0 (`src/rules/map-keys-quotes/index.js`,
//! REF-STYLELINT-SOURCE): a map's key, in a `$variable`'s value, written as a word that is not a
//! number, reported over it: `(red: 1)` reads `red` as a colour, `("red": 1)` as text.

use super::{Context, Report, SheetRule};
use crate::javascript::to_number;
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part, Value};

/// The rule.
pub struct MapKeysQuotes;

/// Whether parentheses' parts read as a map: a key, `:`, a value, `,` (`isMap`).
fn is_map(parsed: &Value, nodes: &[usize]) -> bool {
    let value_of = |at: usize| parsed.part(nodes[at]).value.as_str();
    nodes.len() >= 4
        && matches!(parsed.part(nodes[0]).kind, Part::Word | Part::String)
        && value_of(1) == ":"
        && value_of(3) == ","
}

impl SheetRule for MapKeysQuotes {
    fn name(&self) -> &'static str {
        "scss/map-keys-quotes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected keys in map to be quoted.")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.prop.starts_with('$') {
                continue;
            }
            let parsed = value::parse(&node.value);
            parsed.walk(&mut |_, part| {
                if part.kind != Part::Function
                    || !part.value.is_empty()
                    || !is_map(&parsed, &part.nodes)
                {
                    return true;
                }
                for key in part.nodes.iter().step_by(4).map(|each| parsed.part(*each)) {
                    if matches!(key.value.as_str(), "+" | "/" | "-" | "*" | "%") {
                        continue;
                    }
                    if key.kind == Part::Word && to_number(&key.value).is_nan() {
                        let (start, end) = sheet.word_range(node, &key.value);
                        reports.push(Report::new(start, end, "expected"));
                    }
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MapKeysQuotes;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/map-keys-quotes.txt",
            &|_| Ok(Box::new(MapKeysQuotes)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (11, 1, 0),
            "{:#?}",
            tally.apart
        );
    }
}
