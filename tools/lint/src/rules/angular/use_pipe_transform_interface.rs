//! `@angular-eslint/use-pipe-transform-interface`: a pipe implements `PipeTransform`, so that the
//! compiler checks its `transform` (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its
//! tests). The added interface is not carried.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::metadata::{class_decorator, implemented_names};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct UsePipeTransformInterface;

impl Rule for UsePipeTransformInterface {
    fn name(&self) -> &'static str {
        "@angular-eslint/use-pipe-transform-interface"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if class_decorator(class, "Pipe").is_none()
            || implemented_names(class).contains(&"PipeTransform")
        {
            return;
        }
        let at = class
            .id
            .as_ref()
            .map_or(class.span.start, |id| id.span.start);
        findings.push(source.finding(
            self.name(),
            at,
            "Pipes should implement `PipeTransform` interface".into(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::UsePipeTransformInterface;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&UsePipeTransformInterface, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "Pipes should implement `PipeTransform` interface"
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_pipes_implementing_pipe_transform() {
        // The plugin's valid cases.
        for code in [
            "@Component({ template: 'test' })\nclass Test {}\n",
            "@Pipe({ name: 'test' })\nclass Test implements PipeTransform {\n  transform(value: string) {}\n}\n",
            "@OtherDecorator() @Pipe({ name: 'test' })\nclass Test implements PipeTransform {\n  transform(value: string) {}\n}\n",
            "@Pipe({ name: 'test' })\nclass Test implements ng.PipeTransform {\n  transform(value: string) {}\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_a_pipe_without_it_at_the_class_name() {
        // The plugin's invalid cases: `class ` is 6 columns, the name at 7.
        for (code, line) in [
            (
                "@Pipe({ name: 'test' })\nclass Test {\n  transform(value: string) {}\n}\n",
                2,
            ),
            (
                "import type { PipeTransform } from '@angular/core';\n@Pipe({ name: 'test' })\nclass Test implements AnInterface {\n  transform(value: string) {}\n}\n",
                3,
            ),
            (
                "@OtherDecorator() @Pipe({ name: 'test' })\nclass Test implements AnInterface, AnotherInterface {\n  transform(value: string) {}\n}\n",
                2,
            ),
            (
                "@Pipe({ name: 'test' })\nclass Test<T> {\n  transform(value: T): T {}\n}\n",
                2,
            ),
            (
                "@Pipe({ name: 'test' })\nclass Test<T, U extends SomeType> {\n  transform(value: T): U {}\n}\n",
                2,
            ),
        ] {
            assert_eq!(check(code), [(line, 7)], "{code}");
        }
    }
}
