//! One file, read and parsed once, with what every rule asks of it: its path, its AST and
//! semantic model, its tokens when a rule counts lines, the places of its offsets, its `JSDoc`
//! blocks and its top-level functions.

use std::path::{Path, PathBuf};

use oxc_ast::ast::{Comment, Declaration, Function, FunctionType, Program, Statement};
use oxc_parser::Token;
use oxc_semantic::Semantic;

use crate::finding::{Finding, Fix};
use crate::lines::Lines;
use crate::project::Project;

/// A top-level function declaration, exported or not, and where its `JSDoc` block is attached.
#[derive(Clone, Copy)]
pub struct TopLevelFunction<'s, 'a> {
    /// The declaration.
    pub function: &'s Function<'a>,
    /// The start of the node the block precedes: the `export` statement when exported.
    pub documented_at: u32,
}

/// A parsed file.
pub struct Source<'s, 'a> {
    /// Its path from the root, `/`-separated.
    pub path: &'s str,
    /// Its text.
    pub text: &'a str,
    /// Its AST.
    pub program: &'s Program<'a>,
    /// Its scopes, symbols and nodes.
    pub semantic: &'s Semantic<'a>,
    /// Its tokens, when a rule asked for them.
    pub tokens: &'s [Token],
    /// The repository.
    pub project: &'s Project,
    /// Where each line starts.
    lines: Lines,
}

impl<'s, 'a> Source<'s, 'a> {
    /// A file's parts gathered.
    #[must_use]
    pub fn new(
        path: &'s str,
        program: &'s Program<'a>,
        semantic: &'s Semantic<'a>,
        tokens: &'s [Token],
        project: &'s Project,
    ) -> Self {
        let text = program.source_text;
        let lines = Lines::new(text);

        Self {
            path,
            text,
            program,
            semantic,
            tokens,
            project,
            lines,
        }
    }

    /// The file's absolute path.
    #[must_use]
    pub fn absolute(&self) -> PathBuf {
        self.project.root.join(self.path)
    }

    /// The file's folder, absolute.
    #[must_use]
    pub fn folder(&self) -> PathBuf {
        self.absolute()
            .parent()
            .map_or_else(|| self.project.root.clone(), Path::to_path_buf)
    }

    /// The file's name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(self.path)
    }

    /// The line of an offset, from 1.
    #[must_use]
    pub fn line(&self, offset: u32) -> usize {
        self.lines.line(offset as usize)
    }

    /// The line and the column of an offset, from 1, the column in UTF-16 units.
    #[must_use]
    pub fn place(&self, offset: u32) -> (usize, usize) {
        self.lines.place(self.text, offset as usize)
    }

    /// A finding of a rule at an offset.
    #[must_use]
    pub fn finding(&self, rule: &'static str, offset: u32, message: String) -> Finding {
        let (line, column) = self.place(offset);

        Finding {
            path: self.path.to_owned(),
            line,
            column,
            rule,
            message,
            fix: None,
        }
    }

    /// A finding with the text that fixes it.
    #[must_use]
    pub fn fixed(&self, rule: &'static str, offset: u32, message: String, fix: Fix) -> Finding {
        Finding {
            fix: Some(fix),
            ..self.finding(rule, offset, message)
        }
    }

    /// The text of a comment between its delimiters, as `ESLint`'s `comment.value`.
    #[must_use]
    pub fn comment_value(&self, comment: &Comment) -> &'a str {
        let span = comment.content_span();

        self.text
            .get(span.start as usize..span.end as usize)
            .unwrap_or_default()
    }

    /// Whether a comment is a `JSDoc` block, `/** … */`.
    #[must_use]
    pub fn is_jsdoc(&self, comment: &Comment) -> bool {
        comment.is_block() && self.comment_value(comment).starts_with('*')
    }

    /// The `JSDoc` block right before the node starting at an offset: the last comment attached
    /// to it, when that comment is a `JSDoc` block.
    #[must_use]
    pub fn jsdoc_before(&self, offset: u32) -> Option<&'a str> {
        self.program
            .comments
            .iter()
            .rfind(|comment| comment.attached_to == offset)
            .filter(|comment| self.is_jsdoc(comment))
            .map(|comment| self.comment_value(comment))
    }

    /// The function declarations of the module's body, exported or not, overload signatures and
    /// `export default` left out, in source order.
    #[must_use]
    pub fn top_level_functions(&self) -> Vec<TopLevelFunction<'s, 'a>> {
        self.program
            .body
            .iter()
            .filter_map(|statement| match statement {
                Statement::FunctionDeclaration(function) => Some(TopLevelFunction {
                    function,
                    documented_at: function.span.start,
                }),
                Statement::ExportDeclaration(export) => match &export.declaration {
                    Declaration::FunctionDeclaration(function) => Some(TopLevelFunction {
                        function,
                        documented_at: export.span.start,
                    }),
                    _ => None,
                },
                _ => None,
            })
            .filter(|each| each.function.r#type == FunctionType::FunctionDeclaration)
            .collect()
    }

    /// The values of a block tag in a top-level function's `JSDoc` block.
    #[must_use]
    pub fn tags(&self, function: &TopLevelFunction, tag: &str) -> Vec<&'a str> {
        tag_values(self.jsdoc_before(function.documented_at), tag)
    }

    /// The kind of a top-level function, when it carries exactly one `@kind` tag.
    #[must_use]
    pub fn kind(&self, function: &TopLevelFunction) -> Option<&'a str> {
        match self.tags(function, "kind").as_slice() {
            [kind] => Some(kind),
            _ => None,
        }
    }

    /// The offset a finding on a top-level function points to: its name.
    #[must_use]
    pub fn name_offset(function: &TopLevelFunction) -> u32 {
        function
            .function
            .id
            .as_ref()
            .map_or(function.function.span.start, |id| id.span.start)
    }
}

/// The first word after each `@tag` of a `JSDoc` block, empty when none follows on its line.
#[must_use]
pub fn tag_values<'t>(block: Option<&'t str>, tag: &str) -> Vec<&'t str> {
    let Some(block) = block else {
        return Vec::new();
    };
    let marker = format!("@{tag}");
    let mut values = Vec::new();
    let mut from = 0;

    while let Some(found) = block[from..].find(&marker) {
        let after = from + found + marker.len();
        let rest = &block[after..];
        let blank = rest.len() - rest.trim_start_matches([' ', '\t']).len();
        let word_length = rest[blank..]
            .find(char::is_whitespace)
            .unwrap_or(rest.len() - blank);

        if blank > 0 && word_length > 0 {
            values.push(&rest[blank..blank + word_length]);
            from = after + blank + word_length;
        } else {
            values.push("");
            from = after;
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::tag_values;

    #[test]
    fn reads_the_first_word_after_each_tag() {
        let block = "*\n * Doc.\n *\n * @kind math\n * @see REF-A rest\n * @see\tDERIV-b\n ";

        assert_eq!(tag_values(Some(block), "kind"), ["math"]);
        assert_eq!(tag_values(Some(block), "see"), ["REF-A", "DERIV-b"]);
        assert_eq!(tag_values(Some(block), "param"), [] as [&str; 0]);
        assert_eq!(tag_values(None, "kind"), [] as [&str; 0]);
    }

    #[test]
    fn reads_an_empty_value_when_no_word_follows_on_the_line() {
        assert_eq!(tag_values(Some("* @kind "), "kind"), [""]);
        assert_eq!(tag_values(Some("* @kind\n * math"), "kind"), [""]);
        assert_eq!(tag_values(Some("* @kind"), "kind"), [""]);
        // A longer tag holds the name of a shorter one: `@kindness` reads as `@kind` without value.
        assert_eq!(tag_values(Some("* @kindness x"), "kind"), [""]);
    }
}
