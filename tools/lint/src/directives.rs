//! One-off exceptions: `// lint-disable-next-line rule[, rule] -- justification` turns the rules
//! off on the next line (the records' conventions, area lint, each with its row in the register of
//! exceptions). A directive that turns off nothing, or gives no justification, is reported, as
//! `ESLint` reports its unused ones. The style sheets read their `//` and `/* */` comments the
//! same way (`apply_comments`, EN-167).

use crate::finding::Finding;
use crate::source::Source;

/// A comment a directive may be written in: its start, the line it ends on, and its text between
/// its delimiters.
pub struct Written<'a> {
    /// Its first byte.
    pub offset: u32,
    /// The line its last character is on, from 1.
    pub end_line: usize,
    /// Its text.
    pub value: &'a str,
}

/// The rule a directive's own findings carry.
pub const DIRECTIVE_RULE: &str = "lint/disable-directive";

/// The opener of a directive.
const OPENER: &str = "lint-disable-next-line";

/// A directive: the line it covers, its rules, and where it is written.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Directive {
    /// Its comment's start.
    offset: u32,
    /// The line it covers.
    line: usize,
    /// The rules it turns off.
    rules: Vec<String>,
    /// Whether it gives a justification after `--`.
    justified: bool,
}

/// The findings a file's directives leave, theirs added.
#[must_use]
pub fn apply(source: &Source, findings: Vec<Finding>) -> Vec<Finding> {
    let comments: Vec<Written> = source
        .program
        .comments
        .iter()
        .map(|comment| Written {
            offset: comment.span.start,
            end_line: source.line(comment.span.end),
            value: source.comment_value(comment),
        })
        .collect();
    apply_comments(&comments, findings, &|rule, offset, message| {
        source.finding(rule, offset, message)
    })
}

/// The findings a file's directives leave, theirs added, its comments read and its findings
/// placed by the caller.
#[must_use]
pub fn apply_comments(
    comments: &[Written],
    findings: Vec<Finding>,
    finding: &dyn Fn(&'static str, u32, String) -> Finding,
) -> Vec<Finding> {
    let directives = directives(comments);
    let mut used = vec![Vec::<&str>::new(); directives.len()];
    let mut kept: Vec<Finding> = findings
        .into_iter()
        .filter(|finding| {
            let silencing = directives.iter().position(|directive| {
                directive.line == finding.line
                    && directive.rules.iter().any(|rule| rule == finding.rule)
            });
            if let Some(index) = silencing {
                used[index].push(finding.rule);
            }
            silencing.is_none()
        })
        .collect();

    for (directive, used) in directives.iter().zip(&used) {
        if !directive.justified {
            kept.push(finding(
                DIRECTIVE_RULE,
                directive.offset,
                "A lint-disable directive gives its justification after `--`.".to_owned(),
            ));
        }
        for rule in directive
            .rules
            .iter()
            .filter(|rule| !used.contains(&rule.as_str()))
        {
            kept.push(finding(
                DIRECTIVE_RULE,
                directive.offset,
                format!(
                    "Unused lint-disable directive: `{rule}` reports nothing on the next line."
                ),
            ));
        }
    }
    kept
}

/// The directives of a file, in its comments.
fn directives(comments: &[Written]) -> Vec<Directive> {
    comments
        .iter()
        .filter_map(|comment| {
            let value = comment.value.trim();
            let rest = value.strip_prefix(OPENER)?;
            if !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
                return None;
            }
            let (rules, justification) = rest.split_once("--").unwrap_or((rest, ""));

            Some(Directive {
                offset: comment.offset,
                line: comment.end_line + 1,
                rules: rules
                    .split(',')
                    .map(|rule| rule.trim().to_owned())
                    .filter(|rule| !rule.is_empty())
                    .collect(),
                justified: !justification.trim().is_empty(),
            })
        })
        .collect()
}
