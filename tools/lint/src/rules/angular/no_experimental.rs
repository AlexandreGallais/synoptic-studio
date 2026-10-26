//! `@angular-eslint/no-experimental`: no use of what is tagged `@experimental`, an API Angular may
//! change or remove in any release (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its
//! tests); its syntactic part, a name read (`tagged_use`).

use oxc_semantic::AstNode;

use super::tagged_use::{report_unread_declarations, tagged_use};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoExperimental;

impl Rule for NoExperimental {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-experimental"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        report_unread_declarations(source, node, self.name(), findings);
        if let Some((name, at)) = tagged_use(source, node, "experimental") {
            findings.push(source.finding(self.name(), at, format!("`{name}` is experimental")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoExperimental;
    use crate::rules::angular::tagged_use::{angular_project, invalid_cases, valid_cases};
    use crate::testing::{lint, lint_at};

    /// The place and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&NoExperimental, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac1_reports_a_name_declared_experimental_in_the_file() {
        for (code, line, column, name) in invalid_cases("experimental") {
            assert_eq!(
                check(&code),
                [(line, column, format!("`{name}` is experimental"))],
                "{code}"
            );
        }
        // Another tag is another rule's.
        for (code, _, _, _) in invalid_cases("developerPreview") {
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_accepts_what_is_not_tagged_or_not_read() {
        for code in valid_cases("experimental") {
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_what_angular_s_declarations_tag_experimental() {
        let root = angular_project();
        let at = |code: &str| -> Vec<(usize, usize, String)> {
            lint_at(&NoExperimental, &root, "playground/src/app/a.ts", code)
                .into_iter()
                .map(|each| (each.line, each.column, each.message))
                .collect()
        };

        // A function called on line 2, a class constructed after `new ` on line 3 (column 5).
        assert_eq!(
            at(
                "import { experimentalFunction, ExperimentalClass, regularFunction, previewFunction } from '@angular/core';\nexperimentalFunction();\nnew ExperimentalClass();\nregularFunction();\npreviewFunction();\n"
            ),
            [
                (2, 1, "`experimentalFunction` is experimental".to_owned()),
                (3, 5, "`ExperimentalClass` is experimental".to_owned()),
            ]
        );
        // An alias is reported by the name read; an export passes it on unread.
        assert_eq!(
            at("import { experimentalFunction as alias } from '@angular/core';\nalias();\n"),
            [(2, 1, "`alias` is experimental".to_owned())]
        );
        assert_eq!(
            at(
                "import { experimentalFunction } from '@angular/core';\nexport { experimentalFunction as alias };\n"
            ),
            []
        );
        // An Angular package whose declarations cannot be read is reported at its specifier:
        // `import { experimentalFunction } from ` is 37 columns, the quote at 38; a package not
        // Angular's is not read.
        assert_eq!(
            at(
                "import { experimentalFunction } from '@angular/common';\nexperimentalFunction();\n"
            ),
            [(
                1,
                38,
                "The declarations of `@angular/common` cannot be read: install the packages (`npm ci --prefix playground`)".to_owned()
            )]
        );
        assert_eq!(
            at("import { experimentalFunction } from './dev-preview';\nexperimentalFunction();\n"),
            []
        );
    }
}
