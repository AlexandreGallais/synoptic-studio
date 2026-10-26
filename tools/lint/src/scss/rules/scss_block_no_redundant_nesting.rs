//! `block-no-redundant-nesting` of stylelint-scss 7.3.0
//! (`src/rules/block-no-redundant-nesting/index.js`, REF-STYLELINT-SOURCE): a rule holding nothing
//! but one nested rule, each of one selector, reported over the nested rule and fixed by joining
//! the selectors (`&` replaced by the parent's) around the nested rule's content.

use super::selectors::comma_list;
use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct BlockNoRedundantNesting;

impl SheetRule for BlockNoRedundantNesting {
    fn name(&self) -> &'static str {
        "scss/block-no-redundant-nesting"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected nesting for single child block")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Rule) {
            let [only] = node.nodes() else {
                continue;
            };
            let nested = sheet.node(*only);
            if !nested.is(Kind::Rule)
                || nested.selector.ends_with(':')
                || comma_list(&node.selector).len() != 1
                || comma_list(&nested.selector).len() != 1
            {
                continue;
            }
            let selector = if nested.selector.contains('&') {
                nested.selector.replace('&', &node.selector)
            } else {
                format!("{} {}", node.selector, nested.selector)
            };
            let nested_end = nested.end.unwrap_or(sheet.text.len());
            let content_start = nested.start + sheet.prelude_length(nested) + 1;
            let content_end = nested_end
                .saturating_sub(1 + nested.after.len())
                .max(content_start);
            let fixed = format!(
                "{selector}{}{{{}{}}}",
                node.between,
                &sheet.text[content_start..content_end],
                node.after
            );
            reports.push(Report::new(nested.start, nested_end, "rejected").fixed(
                node.start,
                node.end.unwrap_or(sheet.text.len()),
                fixed,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BlockNoRedundantNesting;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/block-no-redundant-nesting.txt",
            &|_| Ok(Box::new(BlockNoRedundantNesting)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (7, 5, 0),
            "{:#?}",
            tally.apart
        );
    }
}
