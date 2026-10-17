//! `jsdoc/convert-to-jsdoc-comments`: a line or block comment right before a function, or right
//! before or after a variable, a type member or a class field, is a `JSDoc` block instead, so
//! that editors and the API pages read it; comments of tools (`@ts-`, `@license`, `istanbul`,
//! `c8`, `v8`, `eslint`, `prettier-`) are left alone (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/convertToJsdocComments.js`, the project's options). Autofixed.

use oxc_ast::AstKind;
use oxc_ast::ast::Comment;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::{Finding, Fix};
use crate::jsdoc::{self, block::trim_end, comment_after, documented_node, plain_comment_before};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ConvertToJsdocComments;

/// The openings of the comments left alone (`allowedPrefixes`).
const ALLOWED: [&str; 8] = [
    "@ts-",
    "@license",
    "@license-end",
    "istanbul ",
    "c8 ",
    "v8 ",
    "eslint",
    "prettier-",
];

/// Whether a comment's text is left alone.
fn is_allowed(source: &Source, comment: &Comment) -> bool {
    let value = jsdoc::block::trim_start(source.comment_value(comment));
    ALLOWED.iter().any(|prefix| value.starts_with(prefix))
}

/// The indent of a generated block: the blanks opening the node's line, then a space
/// (`getIndent` on the line's start and the node).
fn indent(source: &Source, offset: u32) -> String {
    let start = source.text[..offset as usize]
        .rfind('\n')
        .map_or(0, |at| at + 1);
    let prefix = &source.text[start..offset as usize];
    let blanks = prefix.len() - prefix.trim_start_matches([' ', '\t']).len();
    format!("{} ", &prefix[..blanks])
}

/// The line comments stacked right above one, none left alone (`getPrecedingLineCommentGroup`).
fn line_group(source: &Source, comment: Comment) -> Vec<Comment> {
    let mut group = vec![comment];
    loop {
        let current = group[0];
        let jsdoc::Before::Comment(previous) = jsdoc::before(source, current.span.start) else {
            break;
        };
        if previous.is_block()
            || source.line(previous.span.end.saturating_sub(1)) + 1
                != source.line(current.span.start)
            || is_allowed(source, &previous)
        {
            break;
        }
        group.insert(0, previous);
    }
    group
}

/// The opening of a block made from comments, up to its `*/` (`getCommentOpening`, `multi`).
fn opening(source: &Source, indent: &str, comments: &[Comment]) -> String {
    if let [comment] = comments {
        let body = trim_end(source.comment_value(comment));
        let first_line_blank = body
            .find('\n')
            .is_some_and(|at| body[..at].chars().all(|each| each == ' ' || each == '\t'));
        if first_line_blank {
            return format!("/**{}\n{indent}", body.trim_start_matches([' ', '\t']));
        }
        return format!("/**\n{indent}*{body}\n{indent}");
    }
    let inner: Vec<String> = comments
        .iter()
        .map(|comment| format!("{indent}*{}", trim_end(source.comment_value(comment))))
        .collect();
    format!("/**\n{}\n{indent}", inner.join("\n"))
}

/// The message for a comment.
fn message(comment: &Comment) -> &'static str {
    if comment.is_block() {
        "Block comments should be JSDoc-style."
    } else {
        "Line comments should be JSDoc-style."
    }
}

impl ConvertToJsdocComments {
    /// The finding of a comment right before a node, if any (`checkNonJsdoc`).
    fn before(&self, source: &Source, node: &AstNode) -> Option<Finding> {
        let base = documented_node(source, node);
        let comment = plain_comment_before(source, base.kind().span().start)?;
        if is_allowed(source, &comment) {
            return None;
        }
        let group = if comment.is_block() {
            vec![comment]
        } else {
            line_group(source, comment)
        };
        let mut finding = source.finding(
            self.name(),
            group[0].span.start,
            message(&comment).to_owned(),
        );
        finding.column = 1;
        if !group
            .iter()
            .any(|each| source.comment_value(each).contains("*/"))
        {
            let indent = indent(source, base.kind().span().start);
            finding.fix = Some(Fix {
                start: group[0].span.start,
                end: comment.span.end,
                text: format!("{}*/", opening(source, &indent, &group)),
            });
        }
        Some(finding)
    }

    /// The finding of a comment right after a node on its line, if any (`checkNonJsdocAfter`).
    fn after(&self, source: &Source, node: &AstNode) -> Option<Finding> {
        let comment = comment_after(source, node.kind().span().end)?;
        if source.comment_value(&comment).starts_with('*') || is_allowed(source, &comment) {
            return None;
        }
        let mut finding = source.finding(
            self.name(),
            comment.span.start,
            message(&comment).to_owned(),
        );
        finding.column = 1;
        if !source.comment_value(&comment).contains("*/") {
            let base = documented_node(source, node);
            let indent = indent(source, base.kind().span().start);
            let target = match node.kind() {
                AstKind::VariableDeclarator(_) => source.semantic.nodes().parent_node(node.id()),
                _ => node,
            };
            let insert = target.kind().span().start;
            let insertion = format!(
                "{}*/\n{}",
                opening(source, &indent, &[comment]),
                indent.get(1..).unwrap_or_default()
            );
            finding.fix = Some(Fix {
                start: insert,
                end: comment.span.end,
                text: format!(
                    "{insertion}{}",
                    &source.text[insert as usize..comment.span.start as usize]
                ),
            });
        }
        Some(finding)
    }
}

impl Rule for ConvertToJsdocComments {
    fn name(&self) -> &'static str {
        "jsdoc/convert-to-jsdoc-comments"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut reported = false;
        for node in source.semantic.nodes().iter() {
            let kind = node.kind();
            if jsdoc::is_default_context(&kind) {
                if let Some(finding) = self.before(source, node) {
                    reported = true;
                    findings.push(finding);
                }
            } else if matches!(
                kind,
                AstKind::VariableDeclarator(_)
                    | AstKind::TSPropertySignature(_)
                    | AstKind::PropertyDefinition(_)
            ) {
                if let Some(finding) = self.before(source, node) {
                    reported = true;
                    findings.push(finding);
                }
                if !reported && let Some(finding) = self.after(source, node) {
                    findings.push(finding);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConvertToJsdocComments;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(
            &ConvertToJsdocComments,
            "convert-to-jsdoc-comments",
            "src/a.ts",
            &[],
        );
    }
}
