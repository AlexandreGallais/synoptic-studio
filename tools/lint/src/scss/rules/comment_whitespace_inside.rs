//! `comment-whitespace-inside` of Stylelint 17.16.0 (`lib/rules/comment-whitespace-inside/index.mjs`,
//! REF-STYLELINT-SOURCE): a `/* … */` comment without a space after its opening stars or before its
//! closing ones (`"always"`, the project's), or with one (`"never"`), its `/*!` and `/*#` openings
//! left out, reported at the place; fixed by rewriting the comment as the original mutates it,
//! one fix a pass.

use super::{Context, Report, SheetRule};
use crate::scss::tree::{Kind, Node};

/// The rule and its option.
pub struct CommentWhitespaceInside {
    /// Whether a space is required (`"always"`), else refused (`"never"`).
    pub always: bool,
}

impl CommentWhitespaceInside {
    /// The project's configuration: `"always"`.
    pub const PROJECT: Self = Self { always: true };
}

/// Whether a character is a space of CSS (`isWhitespace`).
fn is_space(character: Option<char>) -> bool {
    matches!(character, Some(' ' | '\n' | '\t' | '\r' | '\u{c}'))
}

/// A comment printed from its parts, as `PostCSS` prints it.
fn printed(left: &str, text: &str, right: &str) -> String {
    format!("/*{left}{text}{right}*/")
}

/// The comment with a space added after its opening stars (`addWhitespaceBefore`).
fn space_before(node: &Node) -> String {
    match node.text.find(|c: char| c != '*') {
        _ if node.text.starts_with('*') => {
            let stars = node.text.len() - node.text.trim_start_matches('*').len();
            let text = format!("{} {}", &node.text[..stars], &node.text[stars..]);
            printed(&node.left, &text, &node.right)
        }
        _ => printed(" ", &node.text, &node.right),
    }
}

/// The comment with a space added before its closing stars (`addWhitespaceAfter`).
fn space_after(node: &Node) -> String {
    if node.text.ends_with('*') {
        let kept = node.text.trim_end_matches('*');
        let text = format!("{kept} {}", &node.text[kept.len()..]);
        printed(&node.left, &text, &node.right)
    } else {
        printed(&node.left, &node.text, " ")
    }
}

/// The comment without spaces inside (`"never"`'s fix).
fn spaceless(node: &Node) -> String {
    let stars = node.text.len() - node.text.trim_start_matches('*').len();
    let text = format!("{}{}", &node.text[..stars], node.text[stars..].trim_start());
    let kept = text.trim_end_matches('*');
    let trailing = &text[kept.len()..];
    printed("", &format!("{}{trailing}", kept.trim_end()), "")
}

impl SheetRule for CommentWhitespaceInside {
    fn name(&self) -> &'static str {
        "stylelint/comment-whitespace-inside"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expectedOpening", "Expected whitespace after \"/*\""),
            ("rejectedOpening", "Expected no whitespace after \"/*\""),
            ("expectedClosing", "Expected whitespace before \"*/\""),
            ("rejectedClosing", "Expected no whitespace before \"*/\""),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Comment) {
            if node.inline {
                continue;
            }
            let raw = sheet.source(node);
            let (start, end) = (node.start, node.start + raw.len());
            let bytes = raw.as_bytes();
            if bytes.len() >= 4
                && matches!(bytes[2], b'#' | b'!')
                && is_space(raw[3..].chars().next())
            {
                continue;
            }
            let opener = 1 + raw[1..].bytes().take_while(|byte| *byte == b'*').count();
            let left = raw[opener..].len() - raw[opener..].trim_start().len();
            let closer = 1 + raw[..raw.len() - 1]
                .bytes()
                .rev()
                .take_while(|byte| *byte == b'*')
                .count();
            let body = &raw[..raw.len() - closer];
            let right = body.len() - body.trim_end().len();
            let fix = |report: Report, text: String| report.fixed(start, end, text);
            if !self.always && left > 0 {
                let at = start + opener;
                reports.push(fix(
                    Report::new(at, at + left, "rejectedOpening"),
                    spaceless(node),
                ));
            }
            if self.always && !is_space(raw[opener..].chars().next()) {
                let at = start + opener;
                reports.push(fix(
                    Report::new(at, at, "expectedOpening"),
                    space_before(node),
                ));
            }
            if !self.always && right > 0 {
                let at = start + body.len();
                reports.push(fix(
                    Report::new(at - right, at, "rejectedClosing"),
                    spaceless(node),
                ));
            }
            if self.always && !is_space(body.chars().last()) {
                let at = start + body.len() - 1;
                reports.push(fix(
                    Report::new(at, at, "expectedClosing"),
                    space_after(node),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommentWhitespaceInside;
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/comment-whitespace-inside.txt",
            &|config| {
                Ok(Box::new(CommentWhitespaceInside {
                    always: primary(config) == &Js::Text("always".to_owned()),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (22, 23, 2),
            "{:#?}",
            tally.apart
        );
    }
}
