//! `at-use-no-unnamespaced` of stylelint-scss 7.3.0 (`src/rules/at-use-no-unnamespaced/index.js`,
//! REF-STYLELINT-SOURCE): an `@use … as *`, which puts a module's members in the global scope,
//! reported over `as *`.

use super::{Context, Report, SheetRule};
use crate::javascript::JsText;
use crate::scss::tree::Kind;

/// The rule.
pub struct AtUseNoUnnamespaced;

/// Where `as *` stands before the end or `with (` (`/(as\s*\*)\s*(?:$|with\s*\()/`): its text.
fn unnamespaced(params: &str) -> Option<&str> {
    params.match_indices("as").find_map(|(at, _)| {
        let rest = &params[at + 2..];
        let star = rest.len() - rest.js_trim_start().len();
        if !rest[star..].starts_with('*') {
            return None;
        }
        let after = rest[star + 1..].js_trim_start();
        let ends = after.is_empty()
            || after
                .strip_prefix("with")
                .is_some_and(|tail| tail.js_trim_start().starts_with('('));
        ends.then(|| &params[at..=(at + 2 + star)])
    })
}

impl SheetRule for AtUseNoUnnamespaced {
    fn name(&self) -> &'static str {
        "scss/at-use-no-unnamespaced"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected @use without namespace")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if node.name != "use" {
                continue;
            }
            if let Some(word) = unnamespaced(&node.params) {
                let (start, end) = sheet.word_range(node, word);
                reports.push(Report::new(start, end, "rejected"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtUseNoUnnamespaced;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-use-no-unnamespaced.txt",
            &|_| Ok(Box::new(AtUseNoUnnamespaced)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (2, 5, 0),
            "{:#?}",
            tally.apart
        );
    }
}
