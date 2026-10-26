//! `@angular-eslint/no-output-rename`: an output is bound by its own name, never an alias, but
//! the directive's own selector (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).
//! The plugin's removal of an alias is not carried.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::bindings::{
    Direction, alias_allowed, aliases, class_selectors, entry_parts, member_identifier, members,
    metadata_entries,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoOutputRename;

impl Rule for NoOutputRename {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-output-rename"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let selectors = class_selectors(class);
        let refused = |property: &str, alias: &str| {
            alias == property || !alias_allowed(&selectors, property, alias, Direction::Output)
        };
        let mut found: Vec<u32> = Vec::new();
        for entry in metadata_entries(class, Direction::Output) {
            let (property, alias) = entry_parts(entry.text, true);
            if alias.is_some_and(|alias| !alias.is_empty() && refused(&property, &alias)) {
                found.push(entry.at);
            }
        }
        for member in members(class, Direction::Output) {
            let Some(property) = member_identifier(&member) else {
                continue;
            };
            for alias in aliases(&member, Direction::Output) {
                if refused(property, alias.text) {
                    found.push(alias.at);
                }
            }
        }
        for at in found {
            findings.push(source.finding(
                self.name(),
                at,
                "Output bindings should not be aliased (https://angular.dev/guide/components/outputs#choosing-event-names)".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoOutputRename;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoOutputRename, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert!(
                    each.message
                        .starts_with("Output bindings should not be aliased")
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_outputs_by_their_own_name_and_the_selector_s() {
        for code in [
            "class Test {}\n",
            "@Component()\nclass Test {\n  @Output() change = new EventEmitter();\n  readonly select = output();\n}\n",
            "@Component({\n  outputs: ['change', `select`],\n})\nclass Test {}\n",
            "@Directive({\n  selector: '[foo], test',\n})\nclass Test {\n  @Output('foo') label = new EventEmitter();\n  readonly other = output({ alias: 'test' });\n}\n",
            "@Directive({\n  selector: 'foo'\n})\nclass Test {\n  @Output('fooMyColor') myColor = new EventEmitter();\n}\n",
            "@Component({\n  selector: 'qx-menuitem',\n  hostDirectives: [{\n    directive: CdkMenuItem,\n    outputs: ['cdkMenuItemTriggered: triggered'],\n  }]\n})\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_alias_at_its_text() {
        for (code, place) in [
            (
                "@Component({\n  outputs: ['a: b']\n})\nclass Test {}\n",
                (2, 13),
            ),
            (
                "@Component({\n  outputs: ['change: change']\n})\nclass Test {}\n",
                (2, 13),
            ),
            (
                "@Component()\nclass Test {\n  @Output('changed') change = new EventEmitter();\n}\n",
                (3, 11),
            ),
            (
                "@Component()\nclass Test {\n  readonly change = output({ alias: 'changed' });\n}\n",
                (3, 37),
            ),
            (
                "@Directive({\n  selector: 'foo'\n})\nclass Test {\n  @Output(`bar`) change = new EventEmitter();\n}\n",
                (5, 11),
            ),
        ] {
            assert_eq!(check(code), [place], "{code}");
        }
    }
}
