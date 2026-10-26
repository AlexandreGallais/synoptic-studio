//! `@angular-eslint/no-developer-preview`: no use of what is tagged `@developerPreview`, an API
//! not yet under Angular's breaking change policy (`@angular-eslint/eslint-plugin` 22.6.0, the
//! cases of its tests); its syntactic part, a name read (`tagged_use`).

use oxc_semantic::AstNode;

use super::tagged_use::tagged_use;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDeveloperPreview;

impl Rule for NoDeveloperPreview {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-developer-preview"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let Some((name, at)) = tagged_use(source, node, "developerPreview") {
            findings.push(source.finding(
                self.name(),
                at,
                format!("`{name}` is in developer preview"),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDeveloperPreview;
    use crate::rules::angular::tagged_use::{angular_project, invalid_cases, valid_cases};
    use crate::testing::{lint, lint_at};

    /// The place and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&NoDeveloperPreview, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac1_reports_a_name_declared_in_developer_preview_in_the_file() {
        for (code, line, column, name) in invalid_cases("developerPreview") {
            assert_eq!(
                check(&code),
                [(line, column, format!("`{name}` is in developer preview"))],
                "{code}"
            );
        }
        // Another tag is another rule's.
        for (code, _, _, _) in invalid_cases("experimental") {
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_accepts_what_is_not_tagged_or_not_read() {
        for code in valid_cases("developerPreview") {
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_what_angular_s_declarations_tag_in_developer_preview() {
        let root = angular_project();
        let at = |code: &str| -> Vec<(usize, usize, String)> {
            lint_at(&NoDeveloperPreview, &root, "playground/src/app/a.ts", code)
                .into_iter()
                .map(|each| (each.line, each.column, each.message))
                .collect()
        };

        // A function called on line 2, a class constructed after `new ` on line 3 (column 5).
        assert_eq!(
            at(
                "import { previewFunction, PreviewClass, regularFunction, experimentalFunction } from '@angular/core';\npreviewFunction();\nnew PreviewClass();\nregularFunction();\nexperimentalFunction();\n"
            ),
            [
                (2, 1, "`previewFunction` is in developer preview".to_owned()),
                (3, 5, "`PreviewClass` is in developer preview".to_owned()),
            ]
        );
        // An alias is reported by the name read; an export passes it on unread.
        assert_eq!(
            at("import { previewFunction as alias } from '@angular/core';\nalias();\n"),
            [(2, 1, "`alias` is in developer preview".to_owned())]
        );
        assert_eq!(
            at(
                "import { previewFunction } from '@angular/core';\nexport { previewFunction as alias };\n"
            ),
            []
        );
        // A package not installed, or not Angular's, is not read.
        assert_eq!(
            at("import { previewFunction } from '@angular/common';\npreviewFunction();\n"),
            []
        );
        assert_eq!(
            at("import { previewFunction } from './dev-preview';\npreviewFunction();\n"),
            []
        );
    }
}
