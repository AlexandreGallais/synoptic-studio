//! `@stylistic/spaced-comment`: a space after `//` or `/*`, and before `*/` (`@stylistic` 5.10.0,
//! `docs/rules/spaced-comment.md`), with the project's options: `always`, the line marker `/`
//! (`///`), the block's `*` marker (`/**`), balanced blocks. An empty comment and a comment that
//! is only its marker are left; a triple-slash `<reference`, which the plugin leaves too, already
//! has its space after the marker. Autofixed: the space written.

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

use super::comments::is_space;

/// The rule.
pub struct SpacedComment;

/// The text a comment opens with, `//` or `/*`.
fn opener(is_block: bool) -> &'static str {
    if is_block { "/*" } else { "//" }
}

/// The marker a comment's text starts with: `/` or `*` on a line, `*` in a block.
fn marker(value: &str, is_block: bool) -> Option<char> {
    let first = value.chars().next()?;
    (first == '*' || (!is_block && first == '/')).then_some(first)
}

/// Whether a comment's text starts with its marker, if any, then white space.
fn spaced_after(value: &str, is_block: bool) -> bool {
    let rest = marker(value, is_block).map_or(value, |found| &value[found.len_utf8()..]);
    rest.chars().next().is_some_and(is_space) || value.chars().next().is_some_and(is_space)
}

impl Rule for SpacedComment {
    fn name(&self) -> &'static str {
        "@stylistic/spaced-comment"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for comment in &source.program.comments {
            let value = source.comment_value(comment);
            let is_block = comment.is_block();
            let is_marker = value == "*" || (!is_block && value == "/");
            if value.is_empty() || is_marker {
                continue;
            }
            if !spaced_after(value, is_block) {
                let found = marker(value, is_block);
                let length = found.map_or(0, char::len_utf8);
                let shown = found.map_or_else(
                    || opener(is_block).to_owned(),
                    |each| format!("{}{each}", opener(is_block)),
                );
                let at = comment.span.start + 2 + u32::try_from(length).unwrap_or(0);
                findings.push(source.fixed(
                    self.name(),
                    comment.span.start,
                    format!("Expected space or tab after '{shown}' in comment."),
                    Fix {
                        start: at,
                        end: at,
                        text: " ".to_owned(),
                    },
                ));
            }
            if is_block && !value.chars().next_back().is_some_and(is_space) {
                let at = comment.span.end - 2;
                findings.push(source.fixed(
                    self.name(),
                    comment.span.start,
                    "Expected space or tab before '*/' in comment.".to_owned(),
                    Fix {
                        start: at,
                        end: at,
                        text: " ".to_owned(),
                    },
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpacedComment;
    use crate::testing::{fixed, lint, messages};

    /// The messages of the findings.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&SpacedComment, "src/a/b.ts", code))
    }

    /// The code with the findings' fixes applied.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&SpacedComment, "src/a/b.ts", code))
    }

    #[test]
    fn reports_a_comment_without_space_after_its_opener() {
        // The documentation's cases for `always`, with `line.markers: ["/"]` and `balanced`.
        assert_eq!(
            found("//This is a comment with no whitespace at the beginning"),
            ["Expected space or tab after '//' in comment."]
        );
        assert_eq!(
            found("/*This is a comment with no whitespace at the beginning */"),
            ["Expected space or tab after '/*' in comment."]
        );
        assert_eq!(
            found("/* This is a comment with whitespace at the beginning but not the end*/"),
            ["Expected space or tab before '*/' in comment."]
        );
        assert_eq!(
            found("///This is a comment with a marker but without whitespace"),
            ["Expected space or tab after '///' in comment."]
        );
        assert_eq!(
            found("//--------------"),
            ["Expected space or tab after '//' in comment."]
        );
        assert_eq!(
            found("/******** COMMENT *******/"),
            [
                "Expected space or tab after '/**' in comment.",
                "Expected space or tab before '*/' in comment."
            ]
        );
        assert_eq!(
            found("/**This is a comment*/"),
            [
                "Expected space or tab after '/**' in comment.",
                "Expected space or tab before '*/' in comment."
            ]
        );
    }

    #[test]
    fn writes_the_missing_spaces() {
        assert_eq!(fix("//x"), "// x");
        assert_eq!(fix("///x"), "/// x");
        assert_eq!(fix("/*x*/"), "/* x */");
        assert_eq!(fix("/**x */"), "/** x */");
    }

    #[test]
    fn lets_spaced_comments_and_markers_through() {
        // The documentation's correct cases for `always` and these options.
        for code in [
            "// This is a comment with a whitespace at the beginning",
            "/* This is a comment with a whitespace at the beginning */",
            "/*\n * This is a comment with a whitespace at the beginning\n */",
            "/** This comment has a newline\n*/",
            "/**\n* I am jsdoc\n*/",
            "//",
            "/**/",
            "///",
            "/// This is a comment with a marker",
            "/// <reference path=\"./a.d.ts\" />",
            "/**\n * JSDoc.\n */",
            "//\tTabbed.",
        ] {
            assert_eq!(found(code), Vec::<String>::new(), "{code}");
        }
    }
}
