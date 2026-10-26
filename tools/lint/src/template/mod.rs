//! The reader of Angular templates (F52, EN-163): a port of the lexer and the parser of
//! `@angular/compiler`'s `ml_parser` (REF-ANGULAR-ML-PARSER, MIT, under ADR-0076), so that the
//! template rules read a template as Angular's compiler reads it (REF-ANGULAR-TEMPLATES,
//! REF-ANGULAR-CONTROL-FLOW): `lexer` gives the tokens, `tree` the nodes, `bindings` what each
//! attribute, text and block binds, `expressions` each expression parsed by Oxc with its pipes.

pub mod bindings;
pub mod entities;
pub mod expressions;
pub mod lexer;
pub mod tags;
pub mod tree;

use std::fs;

use crate::PARSE_RULE;
use crate::finding::{Finding, unreadable};
use crate::lines::Lines;
use crate::project::Project;

/// Whether a path is a component's template, which the reader reads (`*.component.html`).
#[must_use]
pub fn is_template(path: &str) -> bool {
    path.ends_with(".component.html")
}

/// The findings of a template's text: each error of its reading, expressions included, at its
/// line and column under `lint/parse`.
#[must_use]
pub fn lint_text(path: &str, text: &str) -> Vec<Finding> {
    let lines = Lines::new(text);
    bindings::check(&tree::parse(text))
        .into_iter()
        .map(|error| {
            let (line, column) = lines.place(text, error.span.start as usize);
            Finding {
                path: path.to_owned(),
                line,
                column,
                rule: PARSE_RULE,
                message: format!("{}.", error.message.trim_end_matches('.')),
                fix: None,
            }
        })
        .collect()
}

/// The findings of a template read from the disk.
#[must_use]
pub fn lint_path(project: &Project, path: &str) -> Vec<Finding> {
    match fs::read_to_string(project.root.join(path)) {
        Ok(text) => lint_text(path, &text),
        Err(error) => vec![unreadable(path, &error)],
    }
}

#[cfg(test)]
mod tests {
    use super::{is_template, lint_text};

    #[test]
    fn f52_ac1_reports_a_malformed_template_at_its_line_and_column() {
        let findings = lint_text("a.component.html", "<p>\n  <b>{{ a + }}</b>\n</p>\n");

        // Line 2 starts at byte 4; `{{` at 9, the expression ` a + ` at 11 (column 11 - 4 + 1 = 8);
        // Oxc stops at its end, byte 11 + 5 = 16, column 13.
        assert_eq!(
            findings
                .iter()
                .map(|finding| (finding.line, finding.column, finding.rule))
                .collect::<Vec<_>>(),
            [(2, 13, "lint/parse")]
        );
        let tree = lint_text("a.component.html", "<div>\n</span>");
        assert_eq!((tree[0].line, tree[0].column), (2, 1));
        assert!(
            tree[0]
                .message
                .starts_with("Unexpected closing tag \"span\"")
        );
        assert_eq!(
            lint_text("a.component.html", "@if (a) {<b>{{ a | p: 1 }}</b>}"),
            []
        );
    }

    #[test]
    fn reads_a_template_from_the_disk_and_reports_one_it_cannot_read() {
        let root = crate::testing::fixture("templates");
        let written = std::fs::write(root.join("a.component.html"), "<div/>\n");
        let project = crate::project::Project::new(&root);

        assert!(written.is_ok());
        let found = super::lint_path(&project, "a.component.html");
        assert_eq!(
            found
                .iter()
                .map(|finding| finding.message.as_str())
                .collect::<Vec<_>>(),
            ["Only void, custom and foreign elements can be self closed \"div\"."]
        );
        let missing = super::lint_path(&project, "missing.component.html");
        assert!(missing[0].message.starts_with("Cannot read the file"));
    }

    #[test]
    fn reads_only_component_templates() {
        assert!(is_template("playground/src/app/app.component.html"));
        assert!(!is_template("playground/src/index.html"));
    }
}
#[cfg(test)]
mod corpus;
