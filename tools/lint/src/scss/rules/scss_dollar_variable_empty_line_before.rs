//! `dollar-variable-empty-line-before` of stylelint-scss 7.3.0
//! (`src/rules/dollar-variable-empty-line-before/index.js`, REF-STYLELINT-SOURCE): a `$variable`
//! declaration, not the sheet's first, without the empty line before it its options want, or with
//! one they refuse, reported over it and fixed as the original edits its spaces. The project's
//! options, stylelint-config-standard-scss's: `"always"`, except after another variable and a
//! block's first, comments before and one-line blocks left alone.

use super::empty_lines::Lines;
use super::{Context, Report, SheetRule};
use crate::javascript::JsText;
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct DollarVariableEmptyLineBefore {
    /// The options.
    pub options: Lines,
}

impl DollarVariableEmptyLineBefore {
    /// The project's configuration.
    #[must_use]
    pub fn project() -> Self {
        Self {
            options: Lines::new(
                "always",
                &["after-comment", "inside-single-line-block"],
                &["after-dollar-variable", "first-nested"],
            ),
        }
    }
}

/// Whether spaces hold an empty line (`hasEmptyLine` of stylelint-scss).
fn has_empty_line(text: &str) -> bool {
    text.contains("\n\n") || text.contains("\n\r\n")
}

/// Spaces with their start replaced when it matches (`replace(new RegExp('^' + match), …)`).
fn replace_start(text: &str, prefix: &str, with: &str) -> String {
    text.strip_prefix(prefix)
        .map_or_else(|| text.to_owned(), |rest| format!("{with}{rest}"))
}

impl SheetRule for DollarVariableEmptyLineBefore {
    fn name(&self) -> &'static str {
        "scss/dollar-variable-empty-line-before"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected an empty line before $-variable"),
            ("rejected", "Unexpected empty line before $-variable"),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let options = &self.options;
        let newline = if sheet.text.contains("\r\n")
            && sheet
                .text
                .find('\n')
                .is_some_and(|at| at > 0 && sheet.text.as_bytes()[at - 1] == b'\r')
        {
            "\r\n"
        } else {
            "\n"
        };
        let is_variable = |index: usize| {
            let node = sheet.node(index);
            node.is(Kind::Declaration) && node.prop.starts_with('$')
        };
        for (index, node) in sheet.all(Kind::Declaration) {
            if !node.prop.starts_with('$') || sheet.node(0).nodes().first() == Some(&index) {
                continue;
            }
            let previous = sheet.sheet.previous(index);
            let after_comment = previous.is_some_and(|at| sheet.node(at).is(Kind::Comment));
            let after_variable = previous.is_some_and(is_variable);
            let parent = sheet.node(node.parent.unwrap_or(0));
            let single_line = node.parent != Some(0)
                && parent.children.is_some()
                && !sheet.source(parent)[sheet.prelude_length(parent)..].contains(['\n', '\r']);
            if (options.ignores("after-comment") && after_comment)
                || (options.ignores("inside-single-line-block") && single_line)
                || (options.ignores("after-dollar-variable") && after_variable)
            {
                continue;
            }
            let mut expected = options.primary == "always";
            if options.excepts("first-nested") && parent.nodes().first() == Some(&index) {
                expected = !expected;
            }
            if options.excepts("after-comment") && after_comment {
                expected = !expected;
            }
            if options.excepts("after-dollar-variable") && after_variable {
                expected = !expected;
            }
            let before = &node.before;
            if expected == has_empty_line(before) {
                continue;
            }
            let fixed = if expected {
                let mut text = replace_start(before, newline, &format!("{newline}{newline}"));
                if options.excepts("first-nested") && !before.contains(newline) {
                    let spaces = text.len() - text.js_trim_start().len();
                    if spaces > 0 {
                        text = format!("{newline}{newline}{}", &text[spaces..]);
                    }
                }
                text
            } else {
                let text = replace_start(before, "\n\r\n", "\r\n");
                replace_start(&text, &format!("{newline}{newline}"), newline)
            };
            let start = node.start.saturating_sub(before.len());
            reports.push(
                Report::new(
                    node.start,
                    node.end.unwrap_or(sheet.text.len()),
                    if expected { "expected" } else { "rejected" },
                )
                .fixed(start, node.start, fixed),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DollarVariableEmptyLineBefore;
    use crate::scss::rules::tester::{empty_lines, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/dollar-variable-empty-line-before.txt",
            &|config| {
                Ok(Box::new(DollarVariableEmptyLineBefore {
                    options: empty_lines(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (43, 27, 0),
            "{:#?}",
            tally.apart
        );
    }
}
