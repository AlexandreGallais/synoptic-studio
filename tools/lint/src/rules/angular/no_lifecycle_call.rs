//! `@angular-eslint/no-lifecycle-call`: no explicit call of a lifecycle method inside a class
//! Angular knows, but `super.ngOnInit()` from the class's own `ngOnInit`
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests). As the plugin's
//! `CallExpression > MemberExpression`, a member read as one of the call's arguments counts too.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, PropertyKey};
use oxc_semantic::AstNode;

use super::classes::{angular_decorator, enclosing_class, is_lifecycle_method};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoLifecycleCall;

/// The lifecycle method a member reads and whether it reads it on `super`: `a.ngOnInit` or
/// `a[ngOnInit]`, as the plugin's `[property.name=…]`.
fn lifecycle_member<'a>(expression: &Expression<'a>) -> Option<(&'a str, bool)> {
    let (object, name) = match expression {
        Expression::StaticMemberExpression(member) => {
            (&member.object, member.property.name.as_str())
        }
        Expression::ComputedMemberExpression(member) => match &member.expression {
            Expression::Identifier(name) => (&member.object, name.name.as_str()),
            _ => return None,
        },
        _ => return None,
    };
    is_lifecycle_method(name).then_some((name, matches!(object, Expression::Super(_))))
}

impl Rule for NoLifecycleCall {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-lifecycle-call"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let members = std::iter::once(&call.callee)
            .chain(
                call.arguments
                    .iter()
                    .filter_map(|argument| argument.as_expression()),
            )
            .filter_map(lifecycle_member);
        for (name, on_super) in members {
            let Some(class) = enclosing_class(source, node.id()) else {
                return;
            };
            if angular_decorator(class).is_none() {
                return;
            }
            // `super.ngOnInit()` is allowed in the method of the same name (`isSuperCallAllowed`).
            let allowed = on_super
                && source
                    .semantic
                    .nodes()
                    .ancestors(node.id())
                    .find_map(|ancestor| match ancestor.kind() {
                        AstKind::MethodDefinition(method) => Some(method),
                        _ => None,
                    })
                    .is_some_and(|method| {
                        matches!(&method.key, PropertyKey::StaticIdentifier(key) if key.name == name)
                    });
            if !allowed {
                findings.push(source.finding(
                    self.name(),
                    call.span.start,
                    "Avoid explicit calls to lifecycle methods".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoLifecycleCall;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoLifecycleCall, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.message, "Avoid explicit calls to lifecycle methods");
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac1_accepts_super_from_the_same_hook_other_names_and_plain_classes() {
        // The plugin's valid case, cut in its parts.
        for code in [
            "@Component()\nclass Test {\n  ngAfterContentChecked(): void {\n    super.ngAfterContentChecked();\n  }\n}\n",
            "@Directive()\nclass Test {\n  test(): void {\n    this.ngAfterContentChecked1();\n    this.angAfterContentInit();\n    this.ngAfterViewChecked2();\n    this.ngAfterViewInit3();\n    this.ngOnChange$();\n    this.ngOnDestroyx();\n    this.ngOnInitialize();\n    this.ngDoChecking();\n    ngOnInit();\n  }\n}\n",
            "@MyDecorator()\nclass Test {\n  test(): void {\n    this.ngDoCheck();\n  }\n}\n",
            "ngOnDestroy();\nthis.ngOnInit();\n",
            // A string member is no `property.name`.
            "@Component()\nclass Test {\n  test(): void {\n    this['ngOnInit']();\n  }\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_each_call_of_a_hook_at_the_call() {
        // The plugin's invalid cases: `    this.<hook>();` on line 4, the call at column 5.
        for (decorator, method) in [
            ("Component", "ngAfterContentChecked"),
            ("Directive", "ngAfterContentInit"),
            ("Injectable", "ngAfterViewChecked"),
            ("NgModule", "ngAfterViewInit"),
            ("Pipe", "ngDoBootstrap"),
            ("Component", "ngDoCheck"),
            ("Directive", "ngOnChanges"),
            ("Injectable", "ngOnDestroy"),
            ("NgModule", "ngOnInit"),
            ("Service", "ngOnDestroy"),
        ] {
            let code = format!(
                "@{decorator}()\nclass Test {{\n  test(): void {{\n    this.{method}();\n  }}\n}}\n"
            );

            assert_eq!(check(&code), [(4, 5)], "{code}");
        }
        // `super.ngOnChanges()` from another method.
        assert_eq!(
            check(
                "@Component({ template: '' })\nclass Test extends ParentComponent {\n  test(): void {\n    super.ngOnChanges();\n  }\n}\n"
            ),
            [(4, 5)]
        );
        // A hook handed to a call is a member under it: `    queue(this.ngOnInit);` at column 5.
        assert_eq!(
            check(
                "@Component()\nclass Test {\n  test(): void {\n    queue(this.ngOnInit);\n  }\n}\n"
            ),
            [(4, 5)]
        );
    }
}
