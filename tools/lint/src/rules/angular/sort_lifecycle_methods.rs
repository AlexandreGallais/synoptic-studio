//! `@angular-eslint/sort-lifecycle-methods`: a component declares its lifecycle methods in the
//! order Angular calls them (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests); a
//! method declared before the one above it in that order is reported.

use oxc_ast::AstKind;
use oxc_ast::ast::ClassElement;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::classes::{is_lifecycle_method, method_name};
use super::metadata::class_decorator;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct SortLifecycleMethods;

/// The lifecycle methods in the order Angular calls them (`angularLifecycleMethodsOrdered`),
/// `ngDoBootstrap` left out as there.
const ORDER: [&str; 8] = [
    "ngOnChanges",
    "ngOnInit",
    "ngDoCheck",
    "ngAfterContentInit",
    "ngAfterContentChecked",
    "ngAfterViewInit",
    "ngAfterViewChecked",
    "ngOnDestroy",
];

/// A method's place in the order, before every other when it has none (`indexOf`'s −1).
fn rank(name: &str) -> Option<usize> {
    ORDER.iter().position(|each| *each == name)
}

impl Rule for SortLifecycleMethods {
    fn name(&self) -> &'static str {
        "@angular-eslint/sort-lifecycle-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if class_decorator(class, "Component").is_none() {
            return;
        }
        let hooks: Vec<(&str, u32)> = class
            .body
            .body
            .iter()
            .filter_map(|element| match element {
                ClassElement::MethodDefinition(method) => method_name(method)
                    .filter(|name| is_lifecycle_method(name))
                    .map(|name| (name, method.key.span().start)),
                _ => None,
            })
            .collect();
        for pair in hooks.windows(2) {
            if rank(pair[1].0) < rank(pair[0].0) {
                findings.push(source.finding(
                    self.name(),
                    pair[1].1,
                    "Lifecycle Methods are not declared in order of execution".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SortLifecycleMethods;
    use crate::testing::lint;

    /// The lines of the findings on a component's methods, one a line from line 3, each at
    /// column 3.
    fn check(methods: &[&str]) -> Vec<usize> {
        let body: String = methods.iter().fold(String::new(), |mut body, method| {
            body.push_str("  ");
            body.push_str(method);
            body.push_str("(): void {}\n");
            body
        });
        let code = format!("@Component()\nclass Test {{\n{body}}}\n");
        lint(&SortLifecycleMethods, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "Lifecycle Methods are not declared in order of execution"
                );
                assert_eq!(each.column, 3);
                each.line
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_hooks_in_the_order_angular_calls_them() {
        // The plugin's valid cases.
        for methods in [
            &[
                "ngOnChanges",
                "ngOnInit",
                "ngDoCheck",
                "ngAfterContentInit",
                "ngAfterContentChecked",
                "ngAfterViewInit",
                "ngAfterViewChecked",
                "ngOnDestroy",
                "doSomething",
            ][..],
            &[
                "ngOnChanges",
                "ngOnInit",
                "ngAfterContentInit",
                "ngAfterContentChecked",
                "ngAfterViewChecked",
                "ngOnDestroy",
            ],
            &[
                "ngDoCheck",
                "ngAfterContentInit",
                "ngAfterContentChecked",
                "ngAfterViewChecked",
                "doSomething",
                "doSomethingElse",
            ],
            &["ngOnInit"],
            &[],
        ] {
            assert_eq!(check(methods), Vec::<usize>::new(), "{methods:?}");
        }
        // A class other than a component.
        let code =
            "@Directive()\nclass Test {\n  ngOnInit(): void {}\n  ngOnChanges(): void {}\n}\n";
        assert_eq!(
            lint(&SortLifecycleMethods, "playground/src/app/a.ts", code),
            []
        );
    }

    #[test]
    fn f51_ac2_reports_a_hook_declared_before_the_one_above_at_its_name() {
        // The plugin's invalid cases: the methods from line 3.
        for (methods, lines) in [
            (
                &[
                    "ngOnInit",
                    "ngOnChanges",
                    "ngDoCheck",
                    "ngAfterContentInit",
                    "ngAfterContentChecked",
                    "ngAfterViewInit",
                    "ngAfterViewChecked",
                    "ngOnDestroy",
                    "doSomething",
                ][..],
                vec![4],
            ),
            (
                &[
                    "ngOnChanges",
                    "ngOnInit",
                    "ngAfterContentInit",
                    "ngAfterContentChecked",
                    "ngOnDestroy",
                    "ngAfterViewChecked",
                    "doSomething",
                ],
                vec![8],
            ),
            (
                &[
                    "ngDoCheck",
                    "ngAfterContentInit",
                    "ngOnDestroy",
                    "ngAfterContentChecked",
                    "ngAfterViewChecked",
                    "doSomething",
                ],
                vec![6],
            ),
            (&["ngOnInit", "ngOnChanges"], vec![4]),
            // `ngDoBootstrap`, outside the order, after any hook.
            (&["ngOnInit", "ngDoBootstrap"], vec![4]),
        ] {
            assert_eq!(check(methods), lines, "{methods:?}");
        }
    }
}
