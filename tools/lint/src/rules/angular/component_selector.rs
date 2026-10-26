//! `@angular-eslint/component-selector`: a component's selector is an element, kebab-case,
//! prefixed `app-` as the playground's `angular.json` sets (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests, with the Angular CLI's options `{ type: "element", prefix:
//! "app", style: "kebab-case" }`). The plugin's override for a `ShadowDom` component asks
//! kebab-case of a camelCase configuration only, never met with these options.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::metadata::{class_decorator, decorator_property};
use super::selector_names::{
    Failure, SelectorConfig, SelectorStyle, SelectorType, failure, human_list, parse_value,
};
use crate::expressions::text;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ComponentSelector;

/// The options.
const CONFIG: SelectorConfig = SelectorConfig {
    selector_type: SelectorType::Element,
    prefixes: &["app"],
    style: SelectorStyle::KebabCase,
};

/// The style guide's section the messages cite.
const LINK: &str = "https://angular.dev/style-guide#choosing-component-selectors";

/// The message of a failure.
fn message(failure: &Failure) -> String {
    let prefix = human_list(CONFIG.prefixes);
    match failure {
        Failure::Type => format!("The selector should be used as an element ({LINK})"),
        Failure::SelectorAfterPrefix => {
            format!("There should be a selector after the {prefix} prefix")
        }
        Failure::StyleAndPrefix => format!(
            "The selector should be kebab-case and start with one of these prefixes: {prefix} ({LINK} and {LINK})"
        ),
        Failure::Style => format!("The selector should be kebab-case ({LINK})"),
        Failure::Prefix => {
            format!("The selector should start with one of these prefixes: {prefix} ({LINK})")
        }
    }
}

impl Rule for ComponentSelector {
    fn name(&self) -> &'static str {
        "@angular-eslint/component-selector"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some(property) = class_decorator(class, "Component")
            .and_then(|each| decorator_property(each, "selector"))
        else {
            return;
        };
        let value = &property.value;
        let Some(selectors) = parse_value(text(source, value.span()), value)
            .filter(|selectors| !selectors.is_empty())
        else {
            return;
        };
        if let Some(failure) = failure(&selectors, &CONFIG) {
            findings.push(source.finding(self.name(), value.span().start, message(&failure)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentSelector;
    use crate::testing::lint;

    /// The column and message of each finding on a component's selector, written on line 2.
    fn check(selector: &str) -> Vec<(usize, String)> {
        let code = format!("@Component({{\n  selector: {selector}\n}})\nclass Test {{}}\n");
        lint(&ComponentSelector, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.line, 2);
                (each.column, each.message)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_kebab_case_elements_prefixed_app() {
        // The plugin's valid cases under these options.
        for selector in [
            "'app-foo-bar'",
            "'app-bar-foo[baz].bar'",
            "selectorName",
            "`app-foo-bar`",
        ] {
            assert_eq!(check(selector), [], "{selector}");
        }
        // A directive's selector, and a component without one.
        let code =
            "@Directive({ selector: '[appFoo]' })\nclass Test {}\n@Component({})\nclass Other {}\n";
        assert_eq!(
            lint(&ComponentSelector, "playground/src/app/a.ts", code),
            []
        );
    }

    #[test]
    fn f51_ac2_reports_each_failure_at_the_selector() {
        // `  selector: ` is 12 columns, the value at 13.
        let link = "https://angular.dev/style-guide#choosing-component-selectors";
        for (selector, message) in [
            (
                "'foo-bar'",
                format!("The selector should start with one of these prefixes: \"app\" ({link})"),
            ),
            (
                "'appFooBar'",
                format!(
                    "The selector should be kebab-case and start with one of these prefixes: \"app\" ({link} and {link})"
                ),
            ),
            (
                "'app'",
                "There should be a selector after the \"app\" prefix".to_owned(),
            ),
            (
                "'[appFooBar]'",
                format!("The selector should be used as an element ({link})"),
            ),
            // An empty selector is no element.
            (
                "''",
                format!("The selector should be used as an element ({link})"),
            ),
            (
                "'root'",
                format!("The selector should start with one of these prefixes: \"app\" ({link})"),
            ),
            (
                "'app-Foo'",
                format!("The selector should be kebab-case ({link})"),
            ),
        ] {
            assert_eq!(check(selector), [(13, message)], "{selector}");
        }
    }
}
