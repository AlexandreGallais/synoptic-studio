//! `@stylistic/multiline-comment-style`: a comment over several lines is written as line comments,
//! never as a block (`@stylistic` 5.10.0, `docs/rules/multiline-comment-style.md`), with the
//! project's options: `separate-lines`, `checkJSDoc` off — a `JSDoc` block, `/**` alone on its
//! first line, and an `/*!` block of the same shape are left. A block that starts after code on
//! its line, ends before code on its line, or holds a directive (`eslint`, `istanbul`…) is left too.
//! Autofixed as the plugin fixes it: one `// ` line per line of the block, its stars and its
//! margin taken off.

use std::cmp::Ordering;

use oxc_ast::ast::Comment;

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

use super::comments::{is_blank, is_space, lines};

/// The rule.
pub struct MultilineCommentStyle;

/// The plugin's message.
const MESSAGE: &str = "Expected multiple line comments instead of a block comment.";

/// The directives the plugin never reads (`COMMENTS_IGNORE_PATTERN`).
const DIRECTIVES: [&str; 7] = [
    "eslint", "jshint", "jslint", "istanbul", "global", "exported", "jscs",
];

/// Whether a comment's text is a directive: `eslint`, `jscs`, or `jshint `, `jslint `,
/// `istanbul `, `global `, `globals `, `exported ` followed by a space.
fn is_directive(value: &str) -> bool {
    let text = value.trim_start_matches(is_space);

    DIRECTIVES.iter().any(|word| {
        let Some(rest) = text.strip_prefix(word) else {
            return false;
        };
        let rest = if *word == "global" {
            rest.strip_prefix('s').unwrap_or(rest)
        } else {
            rest
        };
        matches!(*word, "eslint" | "jscs") || rest.chars().next().is_some_and(is_space)
    })
}

/// Whether a block is a `JSDoc` block, or an `/*!` one when `marker` is `!`, in the plugin's
/// shape: the marker alone on the first line, every inner line starting with a space, the last
/// line blank.
fn is_documentation(value_lines: &[&str], marker: char) -> bool {
    let Some((first, rest)) = value_lines.split_first() else {
        return false;
    };
    let Some((last, middle)) = rest.split_last() else {
        return false;
    };
    first.strip_prefix(marker).is_some_and(is_blank)
        && middle.iter().all(|line| {
            let margin = line.len() - line.trim_start_matches(is_space).len();
            line[..margin].contains(' ')
        })
        && is_blank(last)
}

/// Whether a block has a star at the start of each inner line, its first and last lines blank.
fn is_starred(value_lines: &[&str]) -> bool {
    let count = value_lines.len();

    value_lines.iter().enumerate().all(|(index, line)| {
        if index == 0 || index == count - 1 {
            is_blank(line)
        } else {
            line.trim_start_matches(is_space).starts_with('*')
        }
    })
}

/// A line's leading part `\s*\*?\s*` and the rest.
fn split_margin(line: &str) -> (&str, &str) {
    let after_space = line.trim_start_matches(is_space);
    let after_star = after_space.strip_prefix('*').unwrap_or(after_space);
    let rest = after_star.trim_start_matches(is_space);

    line.split_at(line.len() - rest.len())
}

/// A text's last characters, `count` of them, or all of it (`slice(-count)`).
fn last_characters(text: &str, count: usize) -> &str {
    let characters = text.chars().count();
    let skip = characters.saturating_sub(count);

    text.char_indices()
        .nth(skip)
        .map_or("", |(at, _)| &text[at..])
}

/// A text from its `count`-th character on (`slice(count)`).
fn from_character(text: &str, count: usize) -> &str {
    text.char_indices()
        .nth(count)
        .map_or("", |(at, _)| &text[at..])
}

/// The lines of a starred block: its inner lines, their star and the space after it taken off
/// when every line has that space.
fn starred_lines(value_lines: &[&str]) -> Vec<String> {
    let inner: Vec<&str> = value_lines[1..value_lines.len() - 1]
        .iter()
        .map(|line| if is_blank(line) { "" } else { *line })
        .collect();
    let spaced = inner.iter().all(|line| {
        let after = line
            .trim_start_matches(is_space)
            .strip_prefix('*')
            .unwrap_or("");
        line.is_empty() || is_blank(after) || after.starts_with(' ')
    });

    inner
        .iter()
        .map(|line| {
            let start = line.len() - line.trim_start_matches(is_space).len();
            match line[start..].strip_prefix('*') {
                Some(after) if spaced => after.strip_prefix(' ').unwrap_or(after).to_owned(),
                Some(after) => after.to_owned(),
                None => (*line).to_owned(),
            }
        })
        .collect()
}

/// The lines of a bare block, each line's margin taken off against the comment's own indent.
fn bare_lines(value_lines: &[&str], indent: &str) -> Vec<String> {
    let blanked: Vec<&str> = value_lines
        .iter()
        .map(|line| if is_blank(line) { "" } else { *line })
        .collect();
    let leading = format!("{indent}   ");
    let width = leading.chars().count();
    let mut offset = "";

    for line in blanked
        .iter()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
    {
        let margin = split_margin(line).0.chars().count();
        if margin < width {
            let candidate = last_characters(&leading, width - margin);
            if candidate.chars().count() > offset.chars().count() {
                offset = candidate;
            }
        }
    }
    blanked
        .iter()
        .map(|line| {
            let (margin, contents) = split_margin(line);
            let length = margin.chars().count();
            match length.cmp(&width) {
                Ordering::Greater => {
                    let keep = offset.chars().count() + length - width;
                    format!("{}{contents}", last_characters(margin, keep))
                }
                Ordering::Less => format!("{}{contents}", from_character(margin, width)),
                Ordering::Equal => contents.to_owned(),
            }
        })
        .collect()
}

/// The text of a line before an offset.
fn before_on_line<'a>(source: &Source<'_, 'a>, offset: u32) -> &'a str {
    let text = &source.text[..offset as usize];
    let start = text
        .rfind(['\n', '\r', '\u{2028}', '\u{2029}'])
        .map_or(0, |at| {
            at + text[at..].chars().next().map_or(1, char::len_utf8)
        });

    &text[start..]
}

/// The text of a line after an offset.
fn after_on_line<'a>(source: &Source<'_, 'a>, offset: u32) -> &'a str {
    let text = &source.text[offset as usize..];
    let end = text
        .find(['\n', '\r', '\u{2028}', '\u{2029}'])
        .unwrap_or(text.len());

    &text[..end]
}

/// The comment's replacement: one `// ` line per line, at the comment's indent.
fn separate_lines(source: &Source, comment: &Comment, value: &str) -> String {
    let value_lines = lines(value);
    let indent = before_on_line(source, comment.span.start);
    let converted = if is_starred(&value_lines) {
        starred_lines(&value_lines)
    } else {
        bare_lines(&value_lines, indent)
    };

    converted
        .iter()
        .map(|line| format!("// {line}"))
        .collect::<Vec<_>>()
        .join(&format!("\n{indent}"))
}

impl Rule for MultilineCommentStyle {
    fn name(&self) -> &'static str {
        "@stylistic/multiline-comment-style"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for comment in source
            .program
            .comments
            .iter()
            .filter(|each| each.is_block())
        {
            let value = source.comment_value(comment);
            let value_lines = lines(value);
            let left = value_lines.len() < 2
                || is_directive(value)
                || is_documentation(&value_lines, '*')
                || is_documentation(&value_lines, '!')
                || !is_blank(before_on_line(source, comment.span.start))
                || !is_blank(after_on_line(source, comment.span.end));
            if left {
                continue;
            }
            findings.push(source.fixed(
                self.name(),
                comment.span.start,
                MESSAGE.to_owned(),
                Fix {
                    start: comment.span.start,
                    end: comment.span.end,
                    text: separate_lines(source, comment, value),
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MultilineCommentStyle;
    use crate::testing::{fixed, lint};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&MultilineCommentStyle, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    /// The code with the findings' fixes applied.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&MultilineCommentStyle, "src/a/b.ts", code))
    }

    #[test]
    fn reports_a_block_over_several_lines_and_writes_line_comments() {
        // The documentation's cases for `separate-lines`.
        let bare = "/* This line\ncalls foo() */\nfoo();";
        let starred = "/*\n * This line\n * calls foo()\n */\nfoo();";

        assert_eq!(places(bare), [(1, 1)]);
        assert_eq!(fix(bare), "// This line\n// calls foo() \nfoo();");
        assert_eq!(places(starred), [(1, 1)]);
        assert_eq!(fix(starred), "// This line\n// calls foo()\nfoo();");
    }

    #[test]
    fn keeps_the_indent_and_the_margins() {
        assert_eq!(
            fix("function f() {\n  /*\n   * a\n   *   b\n   */\n  return 1;\n}"),
            "function f() {\n  // a\n  //   b\n  return 1;\n}"
        );
        assert_eq!(
            fix("  /* a\n       b\n     c */\nx;"),
            "  // a\n  //   b\n  // c \nx;"
        );
        // A line nearer the margin than the comment's text gives the others an offset.
        assert_eq!(
            fix("/* a\n  b\n      c */\nx;"),
            "// a\n// b\n//     c \nx;"
        );
        // `checkJSDoc` off: a `JSDoc` block not in its shape is converted.
        assert_eq!(fix("/** a\n * b\n */\nx;"), "// a\n// b\n// \nx;");
    }

    #[test]
    fn lets_lines_jsdoc_one_line_blocks_and_directives_through() {
        // The documentation's correct case, its `checkJSDoc` and `checkExclamation` cases
        // (both off here), then the blocks the plugin leaves.
        for code in [
            "// This line\n// calls foo()\nfoo();",
            "/**\n * I am a JSDoc comment\n * and I'm not allowed\n */\nfoo();",
            "/*!\n * I am an exclamation comment\n * and I'm not allowed\n */\nfoo();",
            "/* one line */\nfoo();",
            "foo(); /* after\n code */",
            "/* before\n code */ foo();",
            "/* eslint-disable\n a */",
            "/* istanbul ignore next\n */",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
