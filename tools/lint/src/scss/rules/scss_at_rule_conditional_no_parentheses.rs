//! `at-rule-conditional-no-parentheses` of stylelint-scss 7.3.0
//! (`src/rules/at-rule-conditional-no-parentheses/index.js`, REF-STYLELINT-SOURCE): an `@if`,
//! `@else if` or `@while` whose condition is wrapped whole in parentheses, reported over its
//! condition and fixed by unwrapping it (a following `and`, `or` or `not` operand unwrapped too).

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct AtRuleConditionalNoParentheses;

/// Whether a text is `(…)` (`/^\(.*\)$/`, `.` no line break).
fn is_wrapped(text: &str) -> bool {
    text.starts_with('(') && text.ends_with(')') && text.len() >= 2 && !text.contains(['\n', '\r'])
}

/// A logical expression with its operand's parentheses removed (`processLogicalExpression`).
fn logical(text: &str) -> String {
    let lower = text.to_lowercase();
    let keyword = ["and", "or", "not"].iter().find_map(|word| {
        lower
            .strip_prefix(word)
            .filter(|rest| rest.starts_with(char::is_whitespace))
            .map(|rest| text.len() - rest.trim_start().len())
    });
    let Some(length) = keyword else {
        return text.to_owned();
    };
    let (word, after) = (&text[..length], text[length..].trim());
    if is_wrapped(after) {
        format!("{word}{}", after[1..after.len() - 1].trim())
    } else {
        format!("{word}{}", logical(after))
    }
}

/// A condition without its outer parentheses (`removeOuterParens`).
fn unwrap(text: &str) -> String {
    let trimmed = text.trim();
    if !is_wrapped(trimmed) {
        return text.to_owned();
    }
    let mut depth = 0;
    let mut end = 0;
    for (at, character) in trimmed.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            end = at;
            break;
        }
    }
    let inside = trimmed[1..end].trim();
    let after = trimmed[end + 1..].trim();
    if after.is_empty() {
        inside.to_owned()
    } else {
        format!("{inside} {}", logical(after))
    }
}

impl SheetRule for AtRuleConditionalNoParentheses {
    fn name(&self) -> &'static str {
        "scss/at-rule-conditional-no-parentheses"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "rejected",
            "Unexpected () used to surround statements for @-rules",
        )]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            if !matches!(node.name.as_str(), "if" | "while" | "else") {
                continue;
            }
            let params = &node.params;
            let else_if = (node.name == "else").then(|| {
                params
                    .strip_prefix("if")
                    .map(|rest| 2 + usize::from(rest.starts_with(' ')))
            });
            let start = match else_if {
                Some(Some(start)) => start,
                Some(None) => continue,
                None => 0,
            };
            if !is_wrapped(params[start..].trim()) {
                continue;
            }
            let prefix = if node.name == "else" { "if " } else { "" };
            let after_name = if node.name == "else" {
                node.after_name.clone()
            } else {
                " ".to_owned()
            };
            let fixed = format!("{after_name}{prefix}{}", unwrap(&params[start..]));
            let word = params.replacen("if", "", usize::from(node.name == "else"));
            let (from, to) = sheet.word_range(node, word.trim_start_matches(' '));
            let name_end = node.start + 1 + node.name.len();
            let params_end = sheet.params_start(node) + sheet.params(node).len();
            reports.push(Report::new(from, to, "rejected").fixed(name_end, params_end, fixed));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AtRuleConditionalNoParentheses, unwrap};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-rule-conditional-no-parentheses.txt",
            &|_| Ok(Box::new(AtRuleConditionalNoParentheses)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (9, 11, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(unwrap("(a) and (b)"), "a and b");
    }
}
