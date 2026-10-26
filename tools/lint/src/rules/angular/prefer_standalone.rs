//! `@angular-eslint/prefer-standalone`: no component, directive or pipe opting out of standalone,
//! and none writing the default `standalone: true` (`@angular-eslint/eslint-plugin` 22.6.0, the
//! cases of its tests), with `allowExplicitStandalone: false`, so that every class reads the same.
//! The plugin's removals are not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::metadata::{class_decorator_of, decorator_property};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferStandalone;

/// The message for a value other than `true`.
const OPTED_OUT: &str = "Components, Directives and Pipes should not opt out of standalone. Following this guide is highly recommended: https://angular.dev/reference/migrations/standalone";

/// The message for `standalone: true`.
const REDUNDANT: &str = "`standalone: true` is redundant because `standalone: true` is the default";

impl Rule for PreferStandalone {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-standalone"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some((decorator, _)) = class_decorator_of(class, &["Component", "Directive", "Pipe"])
        else {
            return;
        };
        let Some(property) = decorator_property(decorator, "standalone") else {
            return;
        };
        let message = if matches!(&property.value, Expression::BooleanLiteral(value) if value.value)
        {
            REDUNDANT
        } else {
            OPTED_OUT
        };
        findings.push(source.finding(self.name(), property.span.start, message.into()));
    }
}

#[cfg(test)]
mod tests {
    use super::{OPTED_OUT, PreferStandalone, REDUNDANT};
    use crate::testing::lint;

    /// The place and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&PreferStandalone, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_classes_that_leave_standalone_to_its_default() {
        // The plugin's valid cases without `standalone: true`, which the option refuses.
        for code in [
            "@Component({})\nclass Test {}\n",
            "@Component({\n  selector: 'test-selector',\n})\nclass Test {}\n",
            "@Component({\n  selector: 'test-selector',\n  template: '<div></div>',\n  styleUrls: ['./test.css']\n})\nclass Test {}\n",
            "@Directive({})\nclass Test {}\n",
            "@Directive({\n  selector: 'test-selector',\n  providers: []\n})\nclass Test {}\n",
            "@Directive()\nabstract class Test {}\n",
            "@Pipe({})\nclass Test {}\n",
            "@Pipe({\n  name: 'my-pipe',\n  pure: true\n})\nclass Test {}\n",
            // A class Angular does not know, and an injectable.
            "@Injectable({ standalone: false })\nclass Test {}\n",
            "class Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_standalone_written_at_its_property() {
        // `@Component({ ` is 13 columns: the property at 14; `@Pipe({ ` 8, at 9.
        for (code, at, message) in [
            (
                "@Component({ standalone: false })\nclass Test {}\n",
                (1, 14),
                OPTED_OUT,
            ),
            (
                "@Directive({ standalone: false })\nclass Test {}\n",
                (1, 14),
                OPTED_OUT,
            ),
            (
                "@Pipe({ standalone: false })\nclass Test {}\n",
                (1, 9),
                OPTED_OUT,
            ),
            (
                "@Component({\n  standalone: false,\n  template: '<div></div>'\n})\nclass Test {}\n",
                (2, 3),
                OPTED_OUT,
            ),
            (
                "@Component({ standalone: true })\nclass Test {}\n",
                (1, 14),
                REDUNDANT,
            ),
            (
                "@Component({ selector: 'app-test', standalone: true })\nclass Test {}\n",
                // `@Component({ selector: 'app-test', ` is 35 columns.
                (1, 36),
                REDUNDANT,
            ),
            (
                "@Pipe({ standalone: true, name: 'testPipe' })\nclass Test {}\n",
                (1, 9),
                REDUNDANT,
            ),
        ] {
            assert_eq!(check(code), [(at.0, at.1, message.to_owned())], "{code}");
        }
    }
}
