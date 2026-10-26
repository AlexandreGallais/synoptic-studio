//! `@angular-eslint/no-input-prefix`: no input named, nor prefixed, by `is` or `has`, which
//! HTML's attributes never are, `disabled` rather than `isDisabled` (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests), with `prefixes: ["has", "is"]`, the plugin's own examples:
//! its default list is empty. As there, an `@Input()` member's name and alias and the `inputs`
//! metadata are read, never a signal input's.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::bindings::{
    Direction, decorator_alias, entry_parts, is_binding, is_prefixed, member_name, members,
    metadata_entries,
};
use super::selector_names::human_list;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoInputPrefix;

/// The prefixes refused.
const PREFIXES: [&str; 2] = ["has", "is"];

/// Whether a name is refused.
fn refused(name: &str) -> bool {
    PREFIXES.iter().any(|prefix| is_prefixed(name, prefix))
}

impl Rule for NoInputPrefix {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-input-prefix"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let mut found: Vec<u32> = Vec::new();
        for member in members(class, Direction::Input) {
            if !is_binding(&member, Direction::Input) {
                continue;
            }
            if let Some(name) = member_name(&member).filter(|name| refused(name.text)) {
                found.push(name.at);
            }
            if let Some(alias) =
                decorator_alias(&member, "Input").filter(|alias| refused(alias.text))
            {
                found.push(alias.at);
            }
        }
        for entry in metadata_entries(class, Direction::Input) {
            let (name, alias) = entry_parts(entry.text, false);
            if refused(&name) || alias.as_deref().is_some_and(refused) {
                found.push(entry.at);
            }
        }
        found.sort_unstable();
        let message = format!(
            "Input bindings, including aliases, should not be named, nor prefixed by {}",
            human_list(&PREFIXES)
        );
        for at in found {
            findings.push(source.finding(self.name(), at, message.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInputPrefix;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoInputPrefix, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "Input bindings, including aliases, should not be named, nor prefixed by \"has\" or \"is\""
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_names_without_the_prefixes() {
        for code in [
            "@Component()\nclass Test {\n  @Input() disabled: boolean;\n  @Input() issue: string;\n  @Input() hash: string;\n}\n",
            "@Component({ inputs: ['visible', 'island: islandName'] })\nclass Test {}\n",
            // A signal input and a plain property are not read, as in the plugin.
            "@Component()\nclass Test {\n  isOpen = input(false);\n  isShown = true;\n}\n",
            "@Component()\nclass Test {\n  @Input('visible') shown: boolean;\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_a_prefixed_name_or_alias() {
        // `  @Input() ` is 11 columns: the name at 12; `  @Input('` 10: the alias at 10.
        assert_eq!(
            check("@Component()\nclass Test {\n  @Input() isDisabled: boolean;\n}\n"),
            [(3, 12)]
        );
        assert_eq!(
            check("@Component()\nclass Test {\n  @Input() is: boolean;\n}\n"),
            [(3, 12)]
        );
        assert_eq!(
            check("@Component()\nclass Test {\n  @Input('hasError') error: boolean;\n}\n"),
            [(3, 10)]
        );
        // `  @Input({ alias: ` is 18 columns, the alias at 19.
        assert_eq!(
            check("@Component()\nclass Test {\n  @Input({ alias: 'is_open' }) open: boolean;\n}\n"),
            [(3, 19)]
        );
        // `@Component({ inputs: [` is 22 columns, the entry at 23.
        assert_eq!(
            check("@Component({ inputs: ['open: isOpen', 'hasA'] })\nclass Test {}\n"),
            [(1, 23), (1, 39)]
        );
    }
}
