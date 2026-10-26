//! `declaration-no-important` of Stylelint 17.16.0 (`lib/rules/declaration-no-important/index.mjs`,
//! REF-STYLELINT-SOURCE): each `!important` of a declaration, reported over its first
//! `!\s*important` in the declaration's text.

use super::{Context, Report, SheetRule};
use crate::javascript::JsText;
use crate::scss::tree::Kind;

/// The rule.
pub struct DeclarationNoImportant;

/// Where `!`, spaces and `important` (in any case) first appear in a text (`getImportantPosition`).
fn important_position(text: &str) -> Option<(usize, usize)> {
    let lower = text.to_ascii_lowercase();
    lower.match_indices('!').find_map(|(at, _)| {
        let rest = &lower[at + 1..];
        let spaces = rest.len() - rest.js_trim_start().len();
        let word = &rest[spaces..];
        let bounded = word.strip_prefix("important").is_some_and(|after| {
            !after
                .bytes()
                .next()
                .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        });
        bounded.then_some((at, at + 1 + spaces + "important".len()))
    })
}

impl SheetRule for DeclarationNoImportant {
    fn name(&self) -> &'static str {
        "stylelint/declaration-no-important"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Disallowed !important")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.important {
                continue;
            }
            if let Some((start, end)) = important_position(sheet.source(node)) {
                reports.push(Report::new(
                    node.start + start,
                    node.start + end,
                    "rejected",
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeclarationNoImportant;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/declaration-no-important.txt",
            &|_| Ok(Box::new(DeclarationNoImportant)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (1, 3, 0),
            "{:#?}",
            tally.apart
        );
    }
}
