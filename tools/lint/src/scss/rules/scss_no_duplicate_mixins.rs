//! `no-duplicate-mixins` of stylelint-scss 7.3.0 (`src/rules/no-duplicate-mixins/index.js`,
//! REF-STYLELINT-SOURCE): a `@mixin` defined again under the same name, in the same block or one
//! of them at the root, reported over its name: the second replaces the first.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct NoDuplicateMixins;

/// An at-rule's parameters without their first parentheses (`atRuleBaseName`).
fn base_name(params: &str) -> String {
    let base = match params.find('(') {
        Some(open) => match params[open..].find(')') {
            Some(close) => format!("{}{}", &params[..open], &params[open + close + 1..]),
            None => params.to_owned(),
        },
        None => params.to_owned(),
    };
    base.trim().to_owned()
}

impl SheetRule for NoDuplicateMixins {
    fn name(&self) -> &'static str {
        "scss/no-duplicate-mixins"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected duplicate mixin {}")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut seen: Vec<(String, Option<usize>)> = Vec::new();
        for (_, node) in sheet.all(Kind::AtRule) {
            if node.name != "mixin" {
                continue;
            }
            let name = base_name(&node.params);
            if let Some(position) = seen.iter().position(|(each, _)| *each == name) {
                let parent = seen[position].1;
                if parent != node.parent && parent != Some(0) && node.parent != Some(0) {
                    continue;
                }
                let (start, end) = sheet.word_range(node, &name);
                reports.push(Report::new(start, end, "rejected").with(&[&name]));
                seen.remove(position);
            }
            seen.push((name, node.parent));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NoDuplicateMixins, base_name};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/no-duplicate-mixins.txt",
            &|_| Ok(Box::new(NoDuplicateMixins)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (3, 6, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(base_name("m($a) "), "m");
    }
}
