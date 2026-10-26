//! `@angular-eslint/no-async-lifecycle-method`: no `async` lifecycle method on a class Angular
//! knows, which Angular never waits for (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its
//! tests). The plugin's `ClassDeclaration:has(Decorator…)` reads every decorator under the class;
//! the class's own are read here, the only ones that make it Angular's.

use oxc_ast::AstKind;
use oxc_ast::ast::{ClassElement, MethodDefinitionType, PropertyKey};
use oxc_semantic::AstNode;

use super::classes::{CLASS_DECORATORS, called_name, is_lifecycle_method};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoAsyncLifecycleMethod;

impl Rule for NoAsyncLifecycleMethod {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-async-lifecycle-method"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let angular = class
            .decorators
            .iter()
            .filter_map(called_name)
            .any(|name| CLASS_DECORATORS.contains(&name));
        if !class.is_declaration() || !angular {
            return;
        }
        for element in &class.body.body {
            let ClassElement::MethodDefinition(method) = element else {
                continue;
            };
            let PropertyKey::StaticIdentifier(key) = &method.key else {
                continue;
            };
            if method.r#type == MethodDefinitionType::MethodDefinition
                && is_lifecycle_method(&key.name)
                && method.value.r#async
            {
                findings.push(source.finding(
                    self.name(),
                    key.span.start,
                    "Angular Lifecycle method should not be async".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAsyncLifecycleMethod;
    use crate::rules::angular::classes::LIFECYCLE_METHODS;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoAsyncLifecycleMethod, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.message, "Angular Lifecycle method should not be async");
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac1_accepts_synchronous_hooks_functions_and_plain_classes() {
        // The plugin's valid cases: each hook synchronous on a component, async functions of the
        // hooks' names, an async hook on a class Angular does not know.
        for method in LIFECYCLE_METHODS {
            let code = format!("@Component()\nclass Test {{\n  {method}() {{}}\n}}\n");
            assert_eq!(check(&code), [], "{code}");
            let code = format!("async function {method}() {{}}\n");
            assert_eq!(check(&code), [], "{code}");
        }
        assert_eq!(check("class Test {\n  async ngOnInit() {}\n}\n"), []);
        // An async method that is no hook.
        assert_eq!(
            check("@Component()\nclass Test {\n  async load() {}\n}\n"),
            []
        );
    }

    #[test]
    fn f51_ac1_reports_each_async_hook_at_its_name() {
        // `  async ` is 8 columns: each name starts at column 9 of line 3; the plugin reports
        // `ngDoBootstrap` on a pipe and `ngOnDestroy` on a service too.
        let mut cases: Vec<(&str, &str)> = LIFECYCLE_METHODS
            .iter()
            .filter(|method| **method != "ngDoBootstrap")
            .map(|method| ("Component", *method))
            .collect();
        cases.extend([("Pipe", "ngDoBootstrap"), ("Service", "ngOnDestroy")]);
        for (decorator, method) in cases {
            let code = format!("@{decorator}()\nclass Test {{\n  async {method}() {{}}\n}}\n");

            assert_eq!(check(&code), [(3, 9)], "{code}");
        }
    }
}
