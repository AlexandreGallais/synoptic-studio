//! `@angular-eslint/pipe-prefix`: a pipe's name starts with the app's prefix, `appDate`
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests), with `prefixes: ["app"]`,
//! the prefix the playground's `angular.json` gives its selectors.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::bindings::written;
use super::metadata::{class_decorator, decorator_property};
use super::selector_names::{SelectorStyle, after_prefix, has_prefix, human_list};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PipePrefix;

/// The prefixes.
const PREFIXES: [&str; 1] = ["app"];

impl Rule for PipePrefix {
    fn name(&self) -> &'static str {
        "@angular-eslint/pipe-prefix"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some(name) = class_decorator(class, "Pipe")
            .and_then(|decorator| decorator_property(decorator, "name"))
            .and_then(|property| written(&property.value))
            .filter(|name| !name.text.is_empty())
        else {
            return;
        };
        let prefixes = human_list(&PREFIXES);
        let message = if !PREFIXES
            .iter()
            .any(|prefix| has_prefix(name.text, prefix, SelectorStyle::CamelCase))
        {
            format!("@Pipes should be prefixed with {prefixes}")
        } else if PREFIXES
            .iter()
            .all(|prefix| after_prefix(name.text, prefix).is_some_and(str::is_empty))
        {
            format!("@Pipes should have a selector after the {prefixes} prefix")
        } else {
            return;
        };
        findings.push(source.finding(self.name(), name.at, message));
    }
}

#[cfg(test)]
mod tests {
    use super::PipePrefix;
    use crate::testing::lint;

    /// The column and message of each finding on a pipe's name, written on line 1.
    fn check(name: &str) -> Vec<(usize, String)> {
        let code = format!("@Pipe({{ name: {name} }})\nclass Test {{}}\n");
        lint(&PipePrefix, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| (each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_names_prefixed_app() {
        // The plugin's valid cases with this prefix.
        for name in ["'appFoo'", "`appFoo`", "'appFooBar'", "name", "''"] {
            assert_eq!(check(name), [], "{name}");
        }
        assert_eq!(
            lint(
                &PipePrefix,
                "playground/src/app/a.ts",
                "@Pipe({})\nclass Test {}\n"
            ),
            []
        );
    }

    #[test]
    fn f51_ac2_reports_a_name_without_the_prefix_at_the_name() {
        // `@Pipe({ name: ` is 14 columns, the name at 15.
        for (name, message) in [
            ("'foo'", "@Pipes should be prefixed with \"app\""),
            ("`fooApp`", "@Pipes should be prefixed with \"app\""),
            ("'appfoo'", "@Pipes should be prefixed with \"app\""),
            (
                "'app'",
                "@Pipes should have a selector after the \"app\" prefix",
            ),
        ] {
            assert_eq!(check(name), [(15, message.to_owned())], "{name}");
        }
    }
}
