//! `block-no-empty` of Stylelint 17.16.0 (`lib/rules/block-no-empty/index.mjs`,
//! REF-STYLELINT-SOURCE): a rule's or an at-rule's block holding nothing but spaces, reported from
//! its `{` to its end; a comment counts as content unless the option `ignore: ["comments"]` is set.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct BlockNoEmpty {
    /// Whether a block holding only comments is empty too (`ignore: ["comments"]`).
    pub ignore_comments: bool,
}

impl BlockNoEmpty {
    /// The project's configuration: a comment is content.
    pub const PROJECT: Self = Self {
        ignore_comments: false,
    };
}

impl SheetRule for BlockNoEmpty {
    fn name(&self) -> &'static str {
        "stylelint/block-no-empty"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Empty block")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            if !(node.is(Kind::Rule) || node.is(Kind::AtRule)) || node.children.is_none() {
                continue;
            }
            let notable = node
                .nodes()
                .iter()
                .any(|&child| !(self.ignore_comments && sheet.node(child).is(Kind::Comment)));
            if notable
                || node
                    .after
                    .chars()
                    .any(|character| !character.is_whitespace())
            {
                continue;
            }
            let start = node.start + sheet.prelude_length(node);
            let end = node.end.unwrap_or(sheet.text.len());
            reports.push(Report::new(start, end, "rejected"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BlockNoEmpty;
    use crate::scss::rules::tester::{Js, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/block-no-empty.txt",
            &|config| {
                let ignore = secondary(config).and_then(|options| options.texts("ignore"));
                Ok(Box::new(BlockNoEmpty {
                    ignore_comments: ignore
                        .is_some_and(|ignore| ignore.iter().any(|each| each == "comments")),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (20, 21, 8),
            "{:#?}",
            tally.apart
        );
        let _ = Js::Null;
    }
}
