//! `@angular-eslint/use-lifecycle-interface`: a class Angular knows implements the interface of
//! each lifecycle method it declares, `OnInit` for `ngOnInit`, so that the compiler checks the
//! method (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests). A method written
//! `override` is left to its base class, as there. The added interface is not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::{ClassElement, MethodDefinitionType};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::classes::{angular_decorator, is_lifecycle_method, key_identifier};
use super::metadata::implemented_names;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct UseLifecycleInterface;

impl Rule for UseLifecycleInterface {
    fn name(&self) -> &'static str {
        "@angular-eslint/use-lifecycle-interface"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if angular_decorator(class).is_none() {
            return;
        }
        let interfaces = implemented_names(class);
        for element in &class.body.body {
            let ClassElement::MethodDefinition(method) = element else {
                continue;
            };
            if method.r#type != MethodDefinitionType::MethodDefinition || method.r#override {
                continue;
            }
            let Some(name) = key_identifier(&method.key).filter(|name| is_lifecycle_method(name))
            else {
                continue;
            };
            // `getLifecycleInterfaceByMethodName`: the name without its `ng`.
            let interface = &name[2..];
            if !interfaces.contains(&interface) {
                findings.push(source.finding(
                    self.name(),
                    method.key.span().start,
                    format!(
                        "Lifecycle interface '{interface}' should be implemented for method '{name}'. (https://angular.dev/style-guide#use-lifecycle-hook-interfaces)"
                    ),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UseLifecycleInterface;
    use crate::testing::lint;

    /// The place and method of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&UseLifecycleInterface, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                let method = each
                    .message
                    .split('\'')
                    .nth(3)
                    .unwrap_or_default()
                    .to_owned();
                assert_eq!(
                    each.message,
                    format!(
                        "Lifecycle interface '{}' should be implemented for method '{method}'. (https://angular.dev/style-guide#use-lifecycle-hook-interfaces)",
                        &method[2..]
                    )
                );
                (each.line, each.column, method)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_implemented_interfaces_overrides_and_plain_classes() {
        // The plugin's valid cases.
        for code in [
            "class Test implements OnInit {\n  ngOnInit() {}\n}\n",
            "class Test implements DoBootstrap {\n  ngDoBootstrap() {}\n}\n",
            "class Test extends Component implements OnInit, OnDestroy {\n  ngOnInit() {}\n  private ngOnChanges = '';\n  ngOnDestroy() {}\n  ngOnSmth() {}\n}\n",
            "@Component()\nclass Test extends Component implements ng.OnInit, ng.OnDestroy {\n  ngOnInit() {}\n  private ngOnChanges = '';\n  ngOnDestroy() {}\n  ngOnSmth() {}\n}\n",
            "class Test {}\n",
            "@Directive()\nclass FoobarBase implements OnDestroy {\n  ngOnDestroy(): void {}\n}\n@Component()\nclass FoobarComponent extends FoobarBase {\n  override ngOnDestroy(): void {\n    super.ngOnDestroy();\n  }\n}\n",
            "class BaseClass {\n  ngOnInit(): void {}\n}\n@Component()\nclass DerivedComponent extends BaseClass {\n  override ngOnInit(): void {\n    super.ngOnInit();\n  }\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_hook_without_its_interface_at_its_name() {
        // The plugin's invalid cases: each method's name at column 3.
        let one = |line: usize, method: &str| vec![(line, 3, method.to_owned())];
        for (code, places) in [
            (
                "@Component()\nclass Test {\n  ngOnInit() {\n  }\n}\n",
                one(3, "ngOnInit"),
            ),
            (
                "import { OnInit } from '@angular/core';\n@Directive()\nclass Test extends Component implements OnInit {\n  ngOnInit() {}\n  ngOnDestroy() {\n  }\n}\n",
                one(5, "ngOnDestroy"),
            ),
            (
                "@Injectable()\nclass Test {\n  ngDoBootstrap() {}\n  ngOnInit() {}\n  ngOnDestroy() {}\n}\n",
                vec![
                    (3, 3, "ngDoBootstrap".to_owned()),
                    (4, 3, "ngOnInit".to_owned()),
                    (5, 3, "ngOnDestroy".to_owned()),
                ],
            ),
            (
                "@NgModule()\nclass Test extends Component implements ng.OnInit {\n  ngOnInit() {}\n  ngOnDestroy() {\n  }\n}\n",
                one(4, "ngOnDestroy"),
            ),
            (
                "@NgModule()\nclass Test extends Component {\n  ngOnInit() {\n  }\n}\n",
                one(3, "ngOnInit"),
            ),
            (
                "@Directive()\nclass FoobarBase implements OnDestroy {\n  ngOnDestroy(): void {}\n}\n@Component()\nclass FoobarComponent extends FoobarBase {\n  ngOnDestroy(): void {\n    super.ngOnDestroy();\n  }\n}\n",
                one(7, "ngOnDestroy"),
            ),
            (
                "@Service()\nclass Test {\n  ngOnDestroy() {\n  }\n}\n",
                one(3, "ngOnDestroy"),
            ),
            (
                "@Component()\nclass Test<T> {\n  ngOnInit() {\n  }\n}\n",
                one(3, "ngOnInit"),
            ),
            (
                "@Injectable()\nclass Test<A, B extends C = D> {\n  ngOnInit() {\n  }\n}\n",
                one(3, "ngOnInit"),
            ),
        ] {
            assert_eq!(check(code), places, "{code}");
        }
    }
}
