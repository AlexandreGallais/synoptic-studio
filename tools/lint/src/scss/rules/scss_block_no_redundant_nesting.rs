//! `block-no-redundant-nesting` of stylelint-scss 7.3.0
//! (`src/rules/block-no-redundant-nesting/index.js`, REF-STYLELINT-SOURCE): a rule holding nothing
//! but one nested rule, each of one selector, reported over the nested rule and fixed by joining
//! the selectors (`&` replaced by the parent's) around the nested rule's content.

use super::selectors::comma_list;
use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct BlockNoRedundantNesting {
    /// Whether the fix moves the nested content up one level (`dedented`), the project's choice;
    /// else it keeps its lines as the original does.
    pub reindent: bool,
}

impl BlockNoRedundantNesting {
    /// The project's configuration: the content moved up one level.
    pub const PROJECT: Self = Self { reindent: true };
}

/// The indentation a node's preceding text gives it: what follows its last line end, none when
/// it starts on the line of the text before it.
fn indentation(before: &str) -> Option<&str> {
    before.rfind(['\n', '\r']).map(|at| &before[at + 1..])
}

/// A nested rule's content moved up one level: each of its lines loses the indentation the
/// nested rule had beyond its parent's, as a formatter writes it (our own departure: the
/// original keeps the lines as they were, which `format:check` then refuses, AUD-060). Left as
/// it is when either rule shares its line with the text before it.
fn dedented(content: &str, parent: Option<&str>, nested: Option<&str>) -> String {
    let extra = match (parent, nested) {
        (Some(parent), Some(nested)) => nested.strip_prefix(parent).unwrap_or_default(),
        _ => "",
    };
    if extra.is_empty() {
        return content.to_owned();
    }
    let mut lines = content.split('\n');
    let mut text = lines.next().unwrap_or_default().to_owned();
    for line in lines {
        text.push('\n');
        text.push_str(line.strip_prefix(extra).unwrap_or(line));
    }
    text
}

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
                if self.reindent {
                    dedented(
                        &sheet.text[content_start..content_end],
                        indentation(&node.before),
                        indentation(&nested.before),
                    )
                } else {
                    sheet.text[content_start..content_end].to_owned()
                },
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
    fn moves_the_nested_content_up_one_level_as_the_formatter_writes_it() {
        let code =
            "a {\n  display: block;\n\n  .d {\n    .e {\n      display: none;\n    }\n  }\n}\n";
        let findings = crate::scss::lint_text("a.scss", code);

        // Own case: `.e` sat two spaces deeper than `.d`, so its content loses two spaces, its
        // closing brace taking `.d`'s place; on one line, nothing to move.
        assert_eq!(
            crate::testing::fixed(code, &findings),
            "a {\n  display: block;\n\n  .d .e {\n    display: none;\n  }\n}\n"
        );
        assert_eq!(super::dedented("x\n  y", Some(""), Some("  ")), "x\ny");
        assert_eq!(super::dedented(" y ", None, Some("  ")), " y ");
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/block-no-redundant-nesting.txt",
            &|_| Ok(Box::new(BlockNoRedundantNesting { reindent: false })),
            &[],
        );
        // Our own departure: with the project's reindenting, the 3 fixed cases whose nested
        // content spans lines are written one level up.
        let reindented = run(
            "stylelint-scss/block-no-redundant-nesting.txt",
            &|_| Ok(Box::new(BlockNoRedundantNesting::PROJECT)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(reindented.failures.len(), 3);
        assert!(
            reindented
                .failures
                .iter()
                .all(|each| each.starts_with("fix "))
        );
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (7, 5, 0),
            "{:#?}",
            tally.apart
        );
    }
}
