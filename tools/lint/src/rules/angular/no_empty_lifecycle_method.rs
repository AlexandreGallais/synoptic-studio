//! `@angular-eslint/no-empty-lifecycle-method`: no lifecycle method without a statement, dead code
//! Angular still calls (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests); a
//! comment alone leaves it empty. The class's own decorators are read, the plugin's `:has`
//! reading any under the class. The suggested removal is not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::ClassElement;
use oxc_semantic::AstNode;

use super::classes::{CLASS_DECORATORS, is_lifecycle_method};
use super::metadata::class_decorator_of;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::angular::classes::method_name;
use crate::source::Source;

/// The rule.
pub struct NoEmptyLifecycleMethod;

impl Rule for NoEmptyLifecycleMethod {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-empty-lifecycle-method"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if class_decorator_of(class, &CLASS_DECORATORS).is_none() {
            return;
        }
        for element in &class.body.body {
            let ClassElement::MethodDefinition(method) = element else {
                continue;
            };
            let named = method_name(method)
                .or_else(|| match &method.key {
                    oxc_ast::ast::PropertyKey::TemplateLiteral(key) => {
                        key.quasis.first().map(|quasi| quasi.value.raw.as_str())
                    }
                    _ => None,
                })
                .is_some_and(is_lifecycle_method);
            let empty = method
                .value
                .body
                .as_ref()
                .is_some_and(|body| body.statements.is_empty());
            if named && empty {
                findings.push(source.finding(
                    self.name(),
                    method.span.start,
                    "Lifecycle methods should not be empty".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoEmptyLifecycleMethod;
    use crate::rules::angular::classes::LIFECYCLE_METHODS;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoEmptyLifecycleMethod, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.message, "Lifecycle methods should not be empty");
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_hooks_with_a_statement_and_other_methods() {
        for method in LIFECYCLE_METHODS {
            let code =
                format!("@Component()\nclass Test {{\n  {method}() {{\n    a();\n  }}\n}}\n");
            assert_eq!(check(&code), [], "{code}");
            // An empty hook of a class Angular does not know.
            let code = format!("class Test {{\n  {method}() {{}}\n}}\n");
            assert_eq!(check(&code), [], "{code}");
        }
        assert_eq!(
            check("@Component()\nclass Test {\n  ngOnSmth() {}\n}\n"),
            []
        );
    }

    #[test]
    fn f51_ac2_reports_an_empty_hook_at_its_start() {
        for decorator in [
            "Component",
            "Directive",
            "Injectable",
            "NgModule",
            "Pipe",
            "Service",
        ] {
            for method in LIFECYCLE_METHODS {
                let code = format!("@{decorator}()\nclass Test {{\n  {method}() {{}}\n}}\n");

                assert_eq!(check(&code), [(3, 3)], "{code}");
            }
        }
        // A comment is no statement; a string or template key names the hook too.
        assert_eq!(
            check(
                "@Component()\nclass Test {\n  ngOnInit() {\n    // nothing yet\n  }\n  'ngOnDestroy'() {}\n  [`ngDoCheck`]() {}\n}\n"
            ),
            [(3, 3), (6, 3), (7, 3)]
        );
    }
}
