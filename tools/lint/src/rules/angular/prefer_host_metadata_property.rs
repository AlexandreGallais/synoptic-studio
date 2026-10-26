//! `@angular-eslint/prefer-host-metadata-property`: host bindings and listeners in the `host`
//! metadata rather than `@HostBinding()` and `@HostListener()` (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::classes::called_name;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferHostMetadataProperty;

impl Rule for PreferHostMetadataProperty {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-host-metadata-property"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Decorator(decorator) = node.kind() else {
            return;
        };
        let kind = match called_name(decorator) {
            Some("HostBinding") => "bindings instead of the `@HostBinding`",
            Some("HostListener") => "listeners instead of the `@HostListener`",
            _ => return,
        };
        findings.push(source.finding(
            self.name(),
            decorator.span.start,
            format!(
                "Prefer using the `host` metadata property in the `@Component` decorator for host {kind} decorator."
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferHostMetadataProperty;
    use crate::testing::lint;

    /// The place and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&PreferHostMetadataProperty, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_the_host_metadata() {
        // The plugin's valid cases.
        for code in [
            "@Component({\n  host: {\n    class: 'my-class',\n  },\n})\nclass Test {}\n",
            "@Component({\n  host: {\n    '(click)': 'bar()'\n  },\n})\nclass Test {\n  bar() {}\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_host_decorators_at_their_start() {
        // The plugin's invalid cases: the decorator at column 3 of line 2.
        assert_eq!(
            check("class Test {\n  @HostBinding('class')\n  readonly cssClass = 'my-class';\n}\n"),
            [(
                2,
                3,
                "Prefer using the `host` metadata property in the `@Component` decorator for host bindings instead of the `@HostBinding` decorator.".to_owned()
            )]
        );
        assert_eq!(
            check("class Test {\n  @HostListener('click')\n  bar() {}\n}\n"),
            [(
                2,
                3,
                "Prefer using the `host` metadata property in the `@Component` decorator for host listeners instead of the `@HostListener` decorator.".to_owned()
            )]
        );
    }
}
