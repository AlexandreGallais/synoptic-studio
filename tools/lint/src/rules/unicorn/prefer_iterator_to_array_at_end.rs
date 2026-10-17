//! `unicorn/prefer-iterator-to-array-at-end`: no `.toArray()` before `.map()`, `.filter()` or
//! `.flatMap()` with an inline callback that cannot read the array: the helpers run lazily on
//! the iterator, `.toArray()` at the end (`eslint-plugin-unicorn` 78.0.0). Its fix is a
//! suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::syntax::{function_like, method_call, method_named, unwrap_chain};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferIteratorToArrayAtEnd;

/// Whether a value is no iterator: a literal, a function, a class, an instance of a class of the
/// file, or a `const` holding one or a `toArray()`, `slice()`, `concat()` or `split()` result.
fn is_known_non_iterator(source: &Source, expression: &Expression, follow: bool) -> bool {
    let expression = unwrap_chain(expression);
    let literal = matches!(
        expression,
        Expression::ObjectExpression(_)
            | Expression::ArrayExpression(_)
            | Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::TemplateLiteral(_)
            | Expression::FunctionExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::ClassExpression(_)
    );
    let local_class = matches!(expression, Expression::NewExpression(new) if matches!(&new.callee,
    Expression::Identifier(callee) if super::syntax::symbol_of(source, callee).is_some_and(|symbol| {
        matches!(
            source.semantic.nodes().kind(source.semantic.scoping().symbol_declaration(symbol)),
            AstKind::Class(_)
        )
    })));
    if literal || local_class {
        return true;
    }
    if !follow || !matches!(expression, Expression::Identifier(_)) {
        return false;
    }
    super::syntax::const_init(source, expression).is_some_and(|init| {
        is_known_non_iterator(source, init, false)
            || method_named(init, &["toArray", "slice", "concat", "split"])
                .is_some_and(|method| method.is_plain())
    })
}

impl Rule for PreferIteratorToArrayAtEnd {
    fn name(&self) -> &'static str {
        "unicorn/prefer-iterator-to-array-at-end"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(helper) = method_call(call).filter(|method| {
            matches!(method.name, "filter" | "flatMap" | "map") && method.is_plain()
        }) else {
            return;
        };
        let Some(&[callback]) = helper.arguments().as_deref() else {
            return;
        };
        let Some(to_array) = method_named(helper.object, &["toArray"])
            .filter(|method| method.is_plain() && method.call.arguments.is_empty())
        else {
            return;
        };
        let reads_array =
            function_like(callback).is_none_or(|function| function.may_read_argument(source, 2));
        if reads_array || is_known_non_iterator(source, to_array.object, true) {
            return;
        }
        findings.push(source.finding(
            self.name(),
            to_array.property.start,
            format!("Move `.toArray()` after `.{}(…)`.", helper.name),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferIteratorToArrayAtEnd;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferIteratorToArrayAtEnd, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_early_to_array() {
        assert_eq!(
            check("const result = iterator.toArray().map(element => element * 2);"),
            ["Move `.toArray()` after `.map(…)`."]
        );
        assert_eq!(
            check("const result = iterator.toArray().filter(element => element > 1);"),
            ["Move `.toArray()` after `.filter(…)`."]
        );
        assert_eq!(
            check("const result = iterator.toArray().flatMap(element => [element, element]);"),
            ["Move `.toArray()` after `.flatMap(…)`."]
        );
        assert_eq!(
            check("const result = makeVector().toArray().map(element => element * 2);").len(),
            1
        );
        assert_eq!(
            check("const r = it.toArray().map(function (e, i) { return e + i; });").len(),
            1
        );
    }

    #[test]
    fn accepts_named_callbacks_and_known_arrays() {
        for code in [
            "const result = iterator.map(element => element * 2).toArray();",
            "const result = iterator.toArray().map(callback);",
            "class Vector {\n\ttoArray() {\n\t\treturn [1, 2, 3];\n\t}\n}\nconst vector = new Vector();\nconst result = vector.toArray().map(element => element * 2);",
            "const result = [1].toArray().map(e => e);",
            "const parts = text.split(',');\nconst result = parts.toArray().map(e => e);",
            "const result = it.toArray().map((e, i, all) => all);",
            "const result = it.toArray().map((...all) => all);",
            "const result = it.toArray().map(function (e) { return arguments[2]; });",
            "const result = it.toArray().reduce((a, e) => a + e, 0);",
            "const result = it.toArray(1).map(e => e);",
            "const result = it?.toArray().map(e => e);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
