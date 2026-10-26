//! `@angular-eslint/directive-selector`: a directive's selector is an attribute, camelCase,
//! prefixed `app` (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests, with the
//! Angular CLI's options `{ type: "attribute", prefix: "app", style: "camelCase" }`).

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
pub struct DirectiveSelector;

/// The options.
const CONFIG: SelectorConfig = SelectorConfig {
    selector_type: SelectorType::Attribute,
    prefixes: &["app"],
    style: SelectorStyle::CamelCase,
};

/// The message of a failure; a directive's style failure says nothing of the prefix.
fn message(failure: &Failure) -> String {
    let prefix = human_list(CONFIG.prefixes);
    match failure {
        Failure::Type => "The selector should be used as an attribute".to_owned(),
        Failure::SelectorAfterPrefix => {
            format!("There should be a selector after the {prefix} prefix")
        }
        Failure::StyleAndPrefix | Failure::Style => "The selector should be camelCase".to_owned(),
        Failure::Prefix => {
            format!("The selector should start with one of these prefixes: {prefix}")
        }
    }
}

impl Rule for DirectiveSelector {
    fn name(&self) -> &'static str {
        "@angular-eslint/directive-selector"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some(property) = class_decorator(class, "Directive")
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
    use super::DirectiveSelector;
    use crate::testing::lint;

    /// The column and message of each finding on a directive's selector, written on line 2.
    fn check(selector: &str) -> Vec<(usize, String)> {
        let code = format!("@Directive({{\n  selector: {selector}\n}})\nclass Test {{}}\n");
        lint(&DirectiveSelector, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.line, 2);
                (each.column, each.message)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_camel_case_attributes_prefixed_app() {
        for selector in [
            "'[appFooBar]'",
            "`[appFooBar]`",
            "'button[appFooBar]'",
            "'[appFooBar], [appBarFoo]'",
            "'[appFoo]:not(.a)'",
            "selectorName",
        ] {
            assert_eq!(check(selector), [], "{selector}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_failure_at_the_selector() {
        // `  selector: ` is 12 columns, the value at 13.
        for (selector, message) in [
            (
                "'app-foo-bar'",
                "The selector should be used as an attribute",
            ),
            (
                "'[app]'",
                "There should be a selector after the \"app\" prefix",
            ),
            ("'[app-foo-bar]'", "The selector should be camelCase"),
            ("'[foo-bar]'", "The selector should be camelCase"),
            (
                "'[fooBar]'",
                "The selector should start with one of these prefixes: \"app\"",
            ),
            (
                "'[appfoo]'",
                "The selector should start with one of these prefixes: \"app\"",
            ),
        ] {
            assert_eq!(check(selector), [(13, message.to_owned())], "{selector}");
        }
    }
}
