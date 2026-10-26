//! `at-mixin-argumentless-call-parentheses` of stylelint-scss 7.3.0
//! (`src/rules/at-mixin-argumentless-call-parentheses/index.js`, REF-STYLELINT-SOURCE): an
//! `@include` of no arguments written with empty parentheses where the option refuses them
//! (`"never"`, the project's, stylelint-config-standard-scss's) or without them where it wants
//! them, its `using (…)` clause kept, reported over the mixin's name and fixed.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct AtMixinArgumentlessCallParentheses {
    /// Whether parentheses are wanted (`"always"`), else refused.
    pub always: bool,
}

impl AtMixinArgumentlessCallParentheses {
    /// The project's configuration: `"never"`.
    pub const PROJECT: Self = Self { always: false };
}

/// The call without its `using (…)` clause, and the clause (`/([) ]+)(using\s*\([^)]+\)+\s*)$/`).
fn split_using(params: &str) -> (String, String) {
    let found = params.match_indices("using").find(|(at, _)| {
        let before = &params[..*at];
        let rest = params[at + 5..].trim_start();
        before.ends_with([')', ' '])
            && rest.starts_with('(')
            && rest.trim_end().ends_with(')')
            && !rest[1..].trim_end().trim_end_matches(')').contains(')')
    });
    match found {
        Some((at, _)) => (params[..at].trim().to_owned(), params[at..].to_owned()),
        None => (params.trim().to_owned(), String::new()),
    }
}

/// Whether a call ends with empty parentheses (`/\(\s*\)\s*$/`).
fn ends_empty(call: &str) -> bool {
    let trimmed = call.trim_end();
    trimmed
        .strip_suffix(')')
        .is_some_and(|inner| inner.trim_end().ends_with('('))
}

impl SheetRule for AtMixinArgumentlessCallParentheses {
    fn name(&self) -> &'static str {
        "scss/at-mixin-argumentless-call-parentheses"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected parentheses in mixin \"{}\" call"),
            (
                "rejected",
                "Unexpected parentheses in argumentless mixin \"{}\" call",
            ),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if node.name != "include" {
                continue;
            }
            let (call, using) = split_using(&node.params);
            if (!self.always && !ends_empty(&call)) || (self.always && call.contains('(')) {
                continue;
            }
            let fixed_call = if self.always {
                format!("{call}()")
            } else {
                let open = call.rfind('(').unwrap_or(call.len());
                call[..open].trim_end().to_owned()
            };
            let fixed = if using.is_empty() {
                fixed_call
            } else {
                format!("{fixed_call} {using}")
            };
            let name_end = call
                .find(|c: char| c.is_whitespace() || c == '(')
                .unwrap_or(call.len());
            let name = &call[..name_end];
            let (start, end) = sheet.word_range(node, name);
            let params_start = sheet.params_start(node);
            let key = if self.always { "expected" } else { "rejected" };
            reports.push(Report::new(start, end, key).with(&[name]).fixed(
                params_start,
                params_start + sheet.params(node).len(),
                fixed,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtMixinArgumentlessCallParentheses;
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-mixin-argumentless-call-parentheses.txt",
            &|config| {
                Ok(Box::new(AtMixinArgumentlessCallParentheses {
                    always: primary(config) == &Js::Text("always".to_owned()),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (28, 48, 0),
            "{:#?}",
            tally.apart
        );
    }
}
