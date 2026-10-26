//! `@angular-eslint/use-component-view-encapsulation`: no `ViewEncapsulation.None`, which makes a
//! component's styles global (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests);
//! the global styles live in the app's `styles` entry. The plugin's removal is not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::metadata::{class_decorator, decorator_properties, names_metadata};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct UseComponentViewEncapsulation;

impl Rule for UseComponentViewEncapsulation {
    fn name(&self) -> &'static str {
        "@angular-eslint/use-component-view-encapsulation"
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
            if !names_metadata(property, "encapsulation") {
                continue;
            }
            if let Expression::StaticMemberExpression(member) = &property.value
                && member.property.name == "None"
                && matches!(&member.object, Expression::Identifier(object) if object.name == "ViewEncapsulation")
            {
                findings.push(source.finding(
                    self.name(),
                    member.property.span.start,
                    "Using `ViewEncapsulation.None` makes your styles global, which may have an unintended effect".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UseComponentViewEncapsulation;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&UseComponentViewEncapsulation, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "Using `ViewEncapsulation.None` makes your styles global, which may have an unintended effect"
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_emulated_and_shadow_dom_encapsulation() {
        // The plugin's valid cases.
        for code in [
            "@Component({\n  selector: 'app-foo-bar',\n})\nclass Test {}\n",
            "@Component({\n  encapsulation: ViewEncapsulation.Emulated,\n})\nclass Test {}\n",
            "@Component({\n  encapsulation: ViewEncapsulation.ShadowDom,\n})\nclass Test {}\n",
            "@Component({\n  encapsulation: encapsulation,\n})\nclass Test {}\n",
            "@Directive({\n  encapsulation: ViewEncapsulation.None,\n})\nclass Test {}\n",
            "@NgModule({\n  bootstrap: [Foo],\n})\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_none_at_its_name() {
        // `  encapsulation: ViewEncapsulation.` is 35 columns, `None` at 36; the string key adds
        // 2, the computed one 4.
        for (key, column) in [
            ("encapsulation", 36),
            ("'encapsulation'", 38),
            ("[`encapsulation`]", 40),
        ] {
            let code =
                format!("@Component({{\n  {key}: ViewEncapsulation.None,\n}})\nclass Test {{}}\n");

            assert_eq!(check(&code), [(2, column)], "{code}");
        }
    }
}
