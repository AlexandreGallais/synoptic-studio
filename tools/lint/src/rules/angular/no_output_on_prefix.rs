//! `@angular-eslint/no-output-on-prefix`: no output named, nor aliased, `on` or starting with
//! it, `(onClick)` reading as `on-onClick` (`@angular-eslint/eslint-plugin` 22.6.0, the cases of
//! its tests).

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::bindings::{entry_parts, is_prefixed, output_names};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoOutputOnPrefix;

impl Rule for NoOutputOnPrefix {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-output-on-prefix"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        for name in output_names(class) {
            let (property, alias) = entry_parts(name.text, false);
            if is_prefixed(&property, "on") || alias.is_some_and(|alias| is_prefixed(&alias, "on"))
            {
                findings.push(source.finding(
                    self.name(),
                    name.at,
                    "Output bindings, including aliases, should not be named \"on\", nor prefixed with it (https://angular.dev/guide/components/outputs#choosing-event-names)".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoOutputOnPrefix;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoOutputOnPrefix, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert!(
                    each.message.starts_with(
                        "Output bindings, including aliases, should not be named \"on\""
                    )
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_outputs_not_starting_with_on() {
        for code in [
            "@Component({ outputs: ['online', 'a: onion'] })\nclass Test {}\n",
            "@Component()\nclass Test {\n  readonly selected = output();\n  @Output() one = new EventEmitter();\n  onClick() {}\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_an_output_named_on() {
        assert_eq!(
            check("@Component()\nclass Test {\n  @Output() onChange = new EventEmitter();\n}\n"),
            [(3, 13)]
        );
        assert_eq!(
            check("@Component()\nclass Test {\n  readonly on = output();\n}\n"),
            [(3, 12)]
        );
        assert_eq!(
            check(
                "@Component()\nclass Test {\n  @Output('on_change') change = new EventEmitter();\n}\n"
            ),
            [(3, 11)]
        );
        assert_eq!(
            check("@Component({ outputs: ['a: onB'] })\nclass Test {}\n"),
            [(1, 24)]
        );
    }
}
