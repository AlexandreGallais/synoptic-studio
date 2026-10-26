//! `@angular-eslint/no-queries-metadata-property`: a component's or directive's queries are
//! declared on its members, never in its `queries` metadata (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests); the plugin's message names `@Output`, kept as written there.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::metadata::{class_decorator_of, decorator_property};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoQueriesMetadataProperty;

impl Rule for NoQueriesMetadataProperty {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-queries-metadata-property"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if let Some(property) = class_decorator_of(class, &["Component", "Directive"])
            .and_then(|(decorator, _)| decorator_property(decorator, "queries"))
        {
            findings.push(source.finding(
                self.name(),
                property.span.start,
                "Use @Output rather than the `queries` metadata property".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoQueriesMetadataProperty;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoQueriesMetadataProperty, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "Use @Output rather than the `queries` metadata property"
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_queries_on_members() {
        for code in [
            "class Test {}\n",
            "@Component({\n  selector: 'app-test',\n})\nclass Test {\n  readonly child = viewChild('a');\n}\n",
            "@Injectable({\n  queries: {},\n})\nclass Test {}\n",
            // `getDecoratorProperty` reads identifier keys only.
            "@Component({\n  'queries': {},\n})\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_the_queries_metadata_at_its_property() {
        for code in [
            "@Component({\n  queries: {\n    child: new ViewChild('a'),\n  },\n})\nclass Test {}\n",
            "@Directive({\n  queries: {},\n})\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [(2, 3)], "{code}");
        }
    }
}
