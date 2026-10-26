//! `@angular-eslint/no-attribute-decorator`: no `@Attribute()` on a constructor's parameter, which
//! reads an attribute once where an input follows it (`@angular-eslint/eslint-plugin` 22.6.0, the
//! cases of its tests).

use oxc_ast::AstKind;
use oxc_ast::ast::MethodDefinitionKind;
use oxc_semantic::AstNode;

use super::classes::called_name;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoAttributeDecorator;

impl Rule for NoAttributeDecorator {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-attribute-decorator"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Decorator(decorator) = node.kind() else {
            return;
        };
        if called_name(decorator) != Some("Attribute") {
            return;
        }
        // `ClassDeclaration MethodDefinition[key.name="constructor"] Decorator`.
        let nodes = source.semantic.nodes();
        let mut ancestors = nodes.ancestors(node.id()).map(AstNode::kind);
        let in_constructor = ancestors.any(|kind| {
            matches!(kind, AstKind::MethodDefinition(method)
                if method.kind == MethodDefinitionKind::Constructor)
        });
        let in_class =
            ancestors.any(|kind| matches!(kind, AstKind::Class(class) if class.is_declaration()));
        if in_constructor && in_class {
            findings.push(source.finding(
                self.name(),
                decorator.span.start,
                "@Attribute can only obtain a single value and is rarely what is required. Use @Input instead to retrieve a stream of values.".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAttributeDecorator;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoAttributeDecorator, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_constructors_without_attribute() {
        // The plugin's valid cases.
        for code in [
            "@Component()\nclass Test {\n  constructor(@Inject(A) foo: string) {}\n}\n",
            "@Component()\nclass Test {\n  @Input() foo: string;\n}\n",
            "@Component()\nclass Test {\n  method(@Attribute('a') a: string) {}\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_attribute_at_the_decorator() {
        // The plugin's invalid cases: `  constructor(` is 14 columns, the decorator at 15.
        assert_eq!(
            check(
                "@Component()\nclass Test {\n  constructor(@Attribute('foo') foo: string) {}\n}\n"
            ),
            [(3, 15)]
        );
        assert_eq!(
            check(
                "@Directive()\nclass Test {\n  constructor(\n    @Attribute('a') a: string,\n    @Attribute('b') b: string,\n  ) {}\n}\n"
            ),
            [(4, 5), (5, 5)]
        );
    }
}
