//! `@angular-eslint/no-pipe-impure`: no `pure: false` in a pipe's metadata, a pipe Angular would
//! run at each change detection (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).
//! The plugin's removal is a suggestion, never applied by `--fix`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, PropertyKey, UnaryOperator};
use oxc_semantic::AstNode;

use super::classes::enclosing_decorators;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoPipeImpure;

/// Whether a key names `pure` as the plugin's `metadataProperty('pure')` does: an identifier not
/// computed, a string, or a template whose first part is `pure`.
fn names_pure(key: &PropertyKey, computed: bool) -> bool {
    match key {
        PropertyKey::StaticIdentifier(key) => !computed && key.name == "pure",
        PropertyKey::StringLiteral(key) => key.value == "pure",
        PropertyKey::TemplateLiteral(key) => key
            .quasis
            .first()
            .is_some_and(|quasi| quasi.value.raw == "pure"),
        _ => false,
    }
}

/// Whether a value is `false` or `!true`.
fn is_false(value: &Expression) -> bool {
    match value {
        Expression::BooleanLiteral(literal) => !literal.value,
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::LogicalNot
                && matches!(&unary.argument, Expression::BooleanLiteral(literal) if literal.value)
        }
        _ => false,
    }
}

impl Rule for NoPipeImpure {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-pipe-impure"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ObjectProperty(property) = node.kind() else {
            return;
        };
        if names_pure(&property.key, property.computed)
            && is_false(&property.value)
            && enclosing_decorators(source, node.id()).contains(&"Pipe")
        {
            findings.push(source.finding(
                self.name(),
                property.span.start,
                "Impure pipes should be avoided because they are invoked on each change-detection cycle"
                    .into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoPipeImpure;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoPipeImpure, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "Impure pipes should be avoided because they are invoked on each change-detection cycle"
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac1_accepts_pure_pipes_and_values_not_written_false() {
        // The plugin's valid cases.
        for code in [
            "class Test {}\n",
            "@Pipe()\nclass Test {}\n",
            "@Pipe({})\nclass Test {}\n",
            "const options = {};\n@Pipe(options)\nclass Test {}\n",
            "@Pipe({\n  name: 'test',\n})\nclass Test {}\n",
            "@Pipe({\n  pure: true\n})\nclass Test {}\n",
            "@Pipe({\n  'pure': !0,\n})\nclass Test {}\n",
            "@Pipe({\n  ['pure']: !!isPure(),\n})\nclass Test {}\n",
            "const pure = 'pure';\n@Pipe({\n  [pure]: false\n})\nclass Test {}\n",
            "const pure = false;\n@Pipe({\n  pure,\n})\nclass Test {}\n",
            "function isPure() {\n  return false;\n}\n@Pipe({\n  [`pure`]: isPure(),\n})\nclass Test {}\n",
            "@NgModule({\n  bootstrap: [Foo]\n})\nclass Test {}\n",
            // `pure: false` outside a pipe's decorator.
            "@Component({\n  pure: false\n})\nclass Test {}\n",
            "const options = { pure: false };\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_pure_written_false_at_its_property() {
        // The plugin's invalid cases: the property on line 2 or 3, at column 3.
        for (code, line) in [
            ("@Pipe({\n  pure: false\n})\nclass Test {}\n", 2),
            (
                "@Pipe({\n  name: 'test',\n  'pure': false,\n})\nclass Test {}\n",
                3,
            ),
            (
                "@Pipe({\n  name: 'test',\n  ['pure']: !true\n})\nclass Test {}\n",
                3,
            ),
            (
                "@Pipe({\n  name: 'test',\n  [`pure`]: false\n})\nclass Test {}\n",
                3,
            ),
        ] {
            assert_eq!(check(code), [(line, 3)], "{code}");
        }
    }
}
