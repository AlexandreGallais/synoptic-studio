//! `@angular-eslint/contextual-lifecycle`: no lifecycle method on a class whose decorator's kind
//! Angular never calls it on, `ngOnInit` on an `@Injectable()` (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_ast::ast::{ClassElement, MethodDefinitionType};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::classes::{
    CLASS_DECORATORS, called_lifecycle_methods, called_name, is_lifecycle_method, method_name,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ContextualLifecycle;

impl Rule for ContextualLifecycle {
    fn name(&self) -> &'static str {
        "@angular-eslint/contextual-lifecycle"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if !class.is_declaration() {
            return;
        }
        // Each decorator is checked by itself, as each of the plugin's selectors is.
        for decorator in class
            .decorators
            .iter()
            .filter_map(called_name)
            .filter(|name| CLASS_DECORATORS.contains(name))
        {
            let called = called_lifecycle_methods(decorator);
            for element in &class.body.body {
                let ClassElement::MethodDefinition(method) = element else {
                    continue;
                };
                if method.r#type != MethodDefinitionType::MethodDefinition {
                    continue;
                }
                let Some(name) = method_name(method) else {
                    continue;
                };
                if is_lifecycle_method(name) && !called.contains(&name) {
                    findings.push(source.finding(
                        self.name(),
                        method.key.span().start,
                        format!(
                            "Angular will not invoke the `{name}` lifecycle method within `@{decorator}()` classes"
                        ),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContextualLifecycle;
    use crate::rules::angular::classes::LIFECYCLE_METHODS;
    use crate::testing::lint;

    /// The place and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&ContextualLifecycle, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac1_accepts_the_methods_angular_calls_on_each_kind_of_class() {
        // The plugin's valid cases: every hook but `ngDoBootstrap` on a component or a directive,
        // `ngDoBootstrap` on a module, `ngOnDestroy` on a service, an injectable or a pipe.
        let mut codes: Vec<String> = ["Component", "Directive"]
            .iter()
            .flat_map(|decorator| {
                LIFECYCLE_METHODS
                    .iter()
                    .filter(|method| **method != "ngDoBootstrap")
                    .map(move |method| {
                        format!("@{decorator}()\nclass Test {{\n  {method}() {{}}\n}}\n")
                    })
            })
            .collect();
        codes.extend(
            ["Injectable", "Pipe", "Service"].map(|decorator| {
                format!("@{decorator}()\nclass Test {{\n  ngOnDestroy() {{}}\n}}\n")
            }),
        );
        codes.push("@NgModule()\nclass Test {\n  ngDoBootstrap() {}\n}\n".into());
        // A class Angular does not know, a decorator without a call, a class expression.
        codes.push("class Test {\n  ngOnInit() {}\n}\n".into());
        codes.push("@Injectable\nclass Test {\n  ngOnInit() {}\n}\n".into());
        codes.push("const Test = @Injectable() class {\n  ngOnInit() {}\n};\n".into());
        for code in codes {
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_each_method_angular_never_calls_on_the_class() {
        // The plugin's invalid cases, each hook at its name: line 3, column 3.
        let cases = [
            ("Component", "ngDoBootstrap"),
            ("Directive", "ngDoBootstrap"),
            ("Injectable", "ngAfterContentChecked"),
            ("Injectable", "ngAfterContentInit"),
            ("Injectable", "ngAfterViewInit"),
            ("Injectable", "ngDoBootstrap"),
            ("Injectable", "ngDoCheck"),
            ("Injectable", "ngOnChanges"),
            ("Injectable", "ngOnInit"),
            ("NgModule", "ngAfterContentChecked"),
            ("NgModule", "ngAfterContentInit"),
            ("NgModule", "ngAfterViewChecked"),
            ("NgModule", "ngAfterViewInit"),
            ("NgModule", "ngDoCheck"),
            ("NgModule", "ngOnChanges"),
            ("NgModule", "ngOnInit"),
            ("NgModule", "ngOnDestroy"),
            ("Pipe", "ngAfterContentChecked"),
            ("Pipe", "ngAfterContentInit"),
            ("Pipe", "ngAfterViewChecked"),
            ("Pipe", "ngAfterViewInit"),
            ("Pipe", "ngDoBootstrap"),
            ("Pipe", "ngDoCheck"),
            ("Pipe", "ngOnChanges"),
            ("Pipe", "ngOnInit"),
            ("Service", "ngOnInit"),
            ("Service", "ngAfterViewInit"),
        ];
        for (decorator, method) in cases {
            let code = format!("@{decorator}()\nclass Test {{\n  {method}() {{}}\n}}\n");

            assert_eq!(
                check(&code),
                [(
                    3,
                    3,
                    format!(
                        "Angular will not invoke the `{method}` lifecycle method within `@{decorator}()` classes"
                    )
                )],
                "{code}"
            );
        }
    }

    #[test]
    fn f51_ac1_reports_the_pipe_beside_a_directive_and_a_string_key() {
        // The plugin's last case: the pipe's `ngDoCheck` (line 4), the directive's `ngOnInit` kept.
        let code = "@Pipe()\nclass Test implements DoCheck {\n  constructor() {}\n  ngDoCheck() {}\n}\n\n@Directive()\nclass TestDirective implements OnInit {\n  ngOnInit() {}\n}\n";

        assert_eq!(
            check(code),
            [(
                4,
                3,
                "Angular will not invoke the `ngDoCheck` lifecycle method within `@Pipe()` classes"
                    .into()
            )]
        );
        // A string key is the method's name (`getMethodName`): `'ngOnInit'` at column 3.
        assert_eq!(
            check("@Injectable()\nclass Test {\n  'ngOnInit'() {}\n}\n").len(),
            1
        );
    }
}
