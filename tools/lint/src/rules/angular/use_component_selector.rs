//! `@angular-eslint/use-component-selector`: every component has a selector, a string not empty,
//! a template or a name (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::metadata::{class_decorator, decorator_property};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct UseComponentSelector;

impl Rule for UseComponentSelector {
    fn name(&self) -> &'static str {
        "@angular-eslint/use-component-selector"
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
        let named =
            decorator_property(decorator, "selector").is_some_and(|property| {
                match &property.value {
                    Expression::StringLiteral(literal) => !literal.value.is_empty(),
                    Expression::TemplateLiteral(_) | Expression::Identifier(_) => true,
                    _ => false,
                }
            });
        if !named {
            findings.push(source.finding(
                self.name(),
                decorator.span.start,
                "The selector of the component is mandatory".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UseComponentSelector;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&UseComponentSelector, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.message, "The selector of the component is mandatory");
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_a_component_with_a_selector() {
        // The plugin's valid cases.
        for code in [
            "@Component({\n  selector: 'sg-bar-foo'\n})\nclass Test {}\n",
            "@Component({\n  selector: `sg-bar-foo`\n})\nclass Test {}\n",
            "const selector = 'a';\n@Component({\n  selector\n})\nclass Test {}\n",
            "@Directive({})\nclass Test {}\n",
            "class Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_a_component_without_one_at_its_decorator() {
        // The plugin's invalid cases, and a value the tree cannot read.
        for code in [
            "@Component({\n  template: 'a'\n})\nclass Test {}\n",
            "@Component({\n  selector: ''\n})\nclass Test {}\n",
            "@Component()\nclass Test {}\n",
            "@Component({\n  selector: a.b\n})\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [(1, 1)], "{code}");
        }
    }
}
