//! `@angular-eslint/prefer-on-push-component-change-detection`: no component opting out of
//! `ChangeDetectionStrategy.OnPush`, Angular's default since its version 22, and none writing it
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests), with
//! `allowExplicitOnPush: false`, so that every component reads the same. The plugin's removals
//! are not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::metadata::{class_decorator, decorator_properties, names_metadata};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferOnPushComponentChangeDetection;

/// The message for a strategy other than `OnPush`.
const OPTED_OUT: &str = "Components should not opt out of the default `ChangeDetectionStrategy.OnPush` change detection strategy";

/// The message for `OnPush` written.
const REDUNDANT: &str = "`changeDetection: ChangeDetectionStrategy.OnPush` is redundant because `ChangeDetectionStrategy.OnPush` is the default change detection strategy";

impl Rule for PreferOnPushComponentChangeDetection {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-on-push-component-change-detection"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some(decorator) = class_decorator(class, "Component") else {
            return;
        };
        for property in decorator_properties(decorator) {
            if !names_metadata(property, "changeDetection") {
                continue;
            }
            let Expression::StaticMemberExpression(member) = &property.value else {
                continue;
            };
            if !matches!(&member.object, Expression::Identifier(object) if object.name == "ChangeDetectionStrategy")
            {
                continue;
            }
            let message = if member.property.name == "OnPush" {
                REDUNDANT
            } else {
                OPTED_OUT
            };
            findings.push(source.finding(self.name(), member.property.span.start, message.into()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OPTED_OUT, PreferOnPushComponentChangeDetection, REDUNDANT};
    use crate::testing::lint;

    /// The place and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, &'static str)> {
        lint(
            &PreferOnPushComponentChangeDetection,
            "playground/src/app/a.ts",
            code,
        )
        .into_iter()
        .map(|each| {
            let message = if each.message == OPTED_OUT {
                OPTED_OUT
            } else {
                assert_eq!(each.message, REDUNDANT);
                REDUNDANT
            };
            (each.line, each.column, message)
        })
        .collect()
    }

    #[test]
    fn f51_ac2_accepts_components_that_leave_the_default() {
        // The plugin's valid cases but the explicit `OnPush`, which the option refuses.
        for code in [
            "class Test {}\n",
            "const options = {};\n@Component(options)\nclass Test {}\n",
            "@Component()\nclass Test {}\n",
            "@Component({})\nclass Test {}\n",
            "@Component({ selector: 'app-test' })\nclass Test {}\n",
            "@Component({ changeDetection: undefined })\nclass Test {}\n",
            "@Component({\n  'changeDetection': changeDetection,\n})\nclass Test {}\n",
            "const changeDetection = ChangeDetectionStrategy.Default;\n@Component({\n  changeDetection,\n})\nclass Test {}\n",
            "@Component({\n  ['changeDetection']: changeDetection(),\n})\nclass Test {}\n",
            "@NgModule({\n  bootstrap: [Foo]\n})\nclass Test {}\n",
            // A directive's metadata, which has no strategy.
            "@Directive({ changeDetection: ChangeDetectionStrategy.Eager })\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_a_strategy_written_at_its_name() {
        // The plugin's invalid cases: `@Component({ changeDetection: ChangeDetectionStrategy.` is
        // 13 + 17 + 24 = 54 columns, the name at 55; `'changeDetection'` adds 2, the computed keys
        // 4 (`['…']`).
        for (code, column, message) in [
            (
                "@Component({ changeDetection: ChangeDetectionStrategy.Eager })",
                55,
                OPTED_OUT,
            ),
            (
                "@Component({ changeDetection: ChangeDetectionStrategy.Default })",
                55,
                OPTED_OUT,
            ),
            (
                "@Component({ 'changeDetection': ChangeDetectionStrategy.Eager })",
                57,
                OPTED_OUT,
            ),
            (
                "@Component({ ['changeDetection']: ChangeDetectionStrategy.Eager })",
                59,
                OPTED_OUT,
            ),
            (
                "@Component({ [`changeDetection`]: ChangeDetectionStrategy.Eager })",
                59,
                OPTED_OUT,
            ),
            (
                "@Component({ changeDetection: ChangeDetectionStrategy.Eager, selector: 'app-test' })",
                55,
                OPTED_OUT,
            ),
            (
                // `selector: 'app-test', ` adds 22.
                "@Component({ selector: 'app-test', changeDetection: ChangeDetectionStrategy.Eager })",
                77,
                OPTED_OUT,
            ),
            (
                "@Component({ changeDetection: ChangeDetectionStrategy.OnPush })",
                55,
                REDUNDANT,
            ),
            (
                "@Component({ 'changeDetection': ChangeDetectionStrategy.OnPush })",
                57,
                REDUNDANT,
            ),
            (
                "@Component({ [`changeDetection`]: ChangeDetectionStrategy.OnPush })",
                59,
                REDUNDANT,
            ),
        ] {
            let code = format!(
                "import {{ ChangeDetectionStrategy }} from '@angular/core';\n{code}\nclass Test {{}}\n"
            );

            assert_eq!(check(&code), [(2, column, message)], "{code}");
        }
    }
}
