//! `@angular-eslint/computed-must-return`: the function given to `computed()` returns a value,
//! or the signal holds `undefined` (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its
//! tests). An arrow with an expression body returns it; a `return` counts for the nearest
//! function expression or arrow around it, as the plugin's stack of them does.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::{AstNode, NodeId};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::descendants;
use crate::source::Source;

/// The rule.
pub struct ComputedMustReturn;

/// The nearest function expression or arrow around a node: a function declaration is not one.
fn owner(source: &Source, id: NodeId) -> Option<NodeId> {
    source
        .semantic
        .nodes()
        .ancestors(id)
        .find(|ancestor| match ancestor.kind() {
            AstKind::Function(function) => function.is_expression(),
            AstKind::ArrowFunctionExpression(_) => true,
            _ => false,
        })
        .map(AstNode::id)
}

impl Rule for ComputedMustReturn {
    fn name(&self) -> &'static str {
        "@angular-eslint/computed-must-return"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        if !matches!(&call.callee, Expression::Identifier(callee) if callee.name == "computed") {
            return;
        }
        let Some(first) = call
            .arguments
            .first()
            .and_then(|first| first.as_expression())
        else {
            return;
        };
        let start = match first {
            Expression::ArrowFunctionExpression(arrow) if arrow.body.as_expression().is_none() => {
                arrow.span.start
            }
            Expression::FunctionExpression(function) => function.span.start,
            _ => return,
        };
        let Some(function) = descendants(source, node.id()).find(|each| {
            matches!(each.kind(), AstKind::ArrowFunctionExpression(arrow) if arrow.span.start == start)
                || matches!(each.kind(), AstKind::Function(function) if function.span.start == start)
        }) else {
            return;
        };
        let returns = descendants(source, function.id()).any(|each| {
            matches!(each.kind(), AstKind::ReturnStatement(statement) if statement.argument.is_some())
                && owner(source, each.id()) == Some(function.id())
        });
        if !returns {
            findings.push(source.finding(
                self.name(),
                call.span.start,
                "computed() is missing a return value".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComputedMustReturn;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&ComputedMustReturn, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.message, "computed() is missing a return value");
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac1_accepts_a_computed_that_returns_a_value() {
        // The plugin's valid cases.
        for code in [
            "const computed = 2;",
            "class computed {}",
            "const x = computed;",
            "computed()",
            "class Test {\n  computed = 2;\n}",
            "class Test {\n  computed() {};\n}",
            "class Test {\n  x = computed(() => y());\n}",
            "function computedFactory() {\n  return computed(() => {\n    return 2;\n  });\n}",
            "computed(() => {\n  return 2;\n});",
            "computed(function() {\n  return 2;\n});",
            "computed(() => {\n  return () => {};\n});",
            "computed(() => {\n  return undefined;\n});",
            "Test.computed(() => {});",
            "instance.computed(() => {});",
            "computed(() => {\n  if (condition()) {\n    return;\n  }\n  return 2;\n})",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_a_computed_without_a_returned_value_at_its_call() {
        // The plugin's invalid cases, the call at the first column of its line.
        for (code, line, column) in [
            ("computed(() => {});", 1, 1),
            ("computed(() => {\n  y();\n});", 1, 1),
            ("computed(function() {\n  y();\n});", 1, 1),
            ("computed(() => {\n  return;\n});", 1, 1),
            ("computed(function() {\n  return;\n});", 1, 1),
            // `  x = ` is 6 columns: the call at 7.
            (
                "class Test {\n  x = computed(() => {\n    y();\n  });\n}",
                2,
                7,
            ),
            ("computed(() => {}, { equal: (a, b) => {} });", 1, 1),
            // A nested arrow's return is its own.
            (
                "computed(() => {\n  const nested = () => {\n    return 2;\n  }\n});",
                1,
                1,
            ),
        ] {
            assert_eq!(check(code), [(line, column)], "{code}");
        }
    }
}
