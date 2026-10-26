//! The reader of the style sheets (F53, EN-167): every `.scss` file is read as Stylelint reads it
//! with `postcss-scss`, by a port of `PostCSS`'s parser and `postcss-scss`'s tokenizer and parser
//! (REF-POSTCSS-SCSS-SOURCE, MIT, under ADR-0076; the choice against a parser crate, ADR-0105):
//! `tokenizer` cuts the text, `parser` builds the `tree` of rules, at-rules, declarations and
//! comments, each with its span and its raw text, and `value` reads a value into words,
//! functions, strings and separators as `postcss-value-parser` does. A sheet that cannot be read
//! is reported under `lint/parse` at its first error, as Stylelint reports `PostCSS`'s.

pub mod parser;
pub mod tokenizer;
pub mod tree;
pub mod value;

use std::fs;

use crate::PARSE_RULE;
use crate::finding::{Finding, unreadable};
use crate::lines::Lines;
use crate::project::Project;

/// Whether a path is a style sheet the reader reads (`*.scss`).
#[must_use]
pub fn is_sheet(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("scss"))
}

/// The findings of a sheet's text: its first reading error, under `lint/parse`.
#[must_use]
pub fn lint_text(path: &str, text: &str) -> Vec<Finding> {
    match parser::parse(text) {
        Ok(_) => Vec::new(),
        Err(error) => {
            let (line, column) = Lines::new(text).place(text, error.offset);
            vec![Finding {
                path: path.to_owned(),
                line,
                column,
                rule: PARSE_RULE,
                message: format!("{}.", error.message),
                fix: None,
            }]
        }
    }
}

/// The findings of a sheet read from the disk.
#[must_use]
pub fn lint_path(project: &Project, path: &str) -> Vec<Finding> {
    match fs::read_to_string(project.root.join(path)) {
        Ok(text) => lint_text(path, &text),
        Err(error) => vec![unreadable(path, &error)],
    }
}

#[cfg(test)]
mod tests {
    use super::{is_sheet, lint_path, lint_text};

    #[test]
    fn f53_ac2_reports_a_sheet_it_cannot_read_at_its_first_error() {
        let findings = lint_text("a.scss", "a {\n  color: red;\n  b\n}\n");

        // `b` then `}`: neither a declaration nor a rule, line 3, column 3.
        assert_eq!(
            findings
                .iter()
                .map(|each| (each.line, each.column, each.rule, each.message.as_str()))
                .collect::<Vec<_>>(),
            [(3, 3, "lint/parse", "Unknown word b.")]
        );
        assert_eq!(
            lint_text("a.scss", "@use 'x';\na { &:hover { b: c; } }\n"),
            []
        );
    }

    #[test]
    fn reads_only_the_scss_files_from_the_disk() {
        let root = crate::testing::fixture("sheets");
        let written = std::fs::write(root.join("a.scss"), "a {\n");
        let project = crate::project::Project::new(&root);

        assert!(written.is_ok());
        assert!(is_sheet("a.component.scss") && !is_sheet("a.css"));
        let found = lint_path(&project, "a.scss");
        assert_eq!(
            found
                .iter()
                .map(|each| (each.line, each.column, each.message.as_str()))
                .collect::<Vec<_>>(),
            [(1, 1, "Unclosed block.")]
        );
        let missing = lint_path(&project, "missing.scss");
        assert!(missing[0].message.starts_with("Cannot read the file"));
    }
}
