//! `at-root-no-redundant` of stylelint-scss 7.3.0 (`src/rules/at-root-no-redundant/index.js`,
//! REF-STYLELINT-SOURCE): an `@at-root` at the root, inside `@keyframes`, or whose every selector
//! holds `&` outside interpolations, reported over its `@at-root` and fixed by dropping it.

use super::{Context, Report, SheetRule};
use crate::javascript::JsText;
use crate::scss::tree::Kind;

/// The rule.
pub struct AtRootNoRedundant;

/// A selector without its interpolations, `#{…}` to the last `}` (`replace(/#\{.*\}/g, "")`).
fn without_interpolation(text: &str) -> String {
    match (text.find("#{"), text.rfind('}')) {
        (Some(open), Some(close)) if close > open => {
            format!("{}{}", &text[..open], &text[close + 1..])
        }
        _ => text.to_owned(),
    }
}

impl SheetRule for AtRootNoRedundant {
    fn name(&self) -> &'static str {
        "scss/at-root-no-redundant"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected @at-root rule")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::AtRule) {
            if node.name != "at-root" {
                continue;
            }
            let at_root = node.parent == Some(0);
            let nested = node
                .params
                .split(',')
                .all(|each| without_interpolation(each).contains('&'));
            let keyframes = sheet
                .ancestors(index)
                .iter()
                .any(|each| each.is(Kind::AtRule) && each.name == "keyframes");
            if !(at_root || nested || keyframes) {
                continue;
            }
            let word = "@at-root";
            let (start, end) = sheet.word_range(node, word);
            let text = sheet.source(node);
            let spaces = text[word.len()..].len() - text[word.len()..].js_trim_start().len();
            let report = Report::new(start, end, "rejected");
            reports.push(if spaces > 0 {
                report.fixed(node.start, node.start + word.len() + spaces, "")
            } else {
                report
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtRootNoRedundant;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-root-no-redundant.txt",
            &|_| Ok(Box::new(AtRootNoRedundant)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (8, 11, 0),
            "{:#?}",
            tally.apart
        );
    }
}
