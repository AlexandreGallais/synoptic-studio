//! `unicorn/no-unreadable-new-expression`: no member read straight off a `new` expression, and
//! no constructor other than a name or a path of names (`eslint-plugin-unicorn` 78.0.0): the
//! instance and the constructor are named first.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUnreadableNewExpression;

/// Whether a constructor is a name or a path of static members from a name.
fn is_simple_constructor(expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(_) => true,
        Expression::StaticMemberExpression(member) => is_static_path(&member.object),
        _ => false,
    }
}

/// Whether an object is a name or a path of static members from one.
fn is_static_path(expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(_) => true,
        Expression::StaticMemberExpression(member) => is_static_path(&member.object),
        _ => false,
    }
}

impl Rule for NoUnreadableNewExpression {
    fn name(&self) -> &'static str {
        "unicorn/no-unreadable-new-expression"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (offset, message) = match node.kind() {
            AstKind::StaticMemberExpression(member)
                if matches!(member.object, Expression::NewExpression(_)) =>
            {
                (
                    member.property.span.start,
                    "Do not access members directly from a `new` expression.",
                )
            }
            AstKind::ComputedMemberExpression(member)
                if matches!(member.object, Expression::NewExpression(_)) =>
            {
                (
                    member.expression.span().start,
                    "Do not access members directly from a `new` expression.",
                )
            }
            AstKind::PrivateFieldExpression(member)
                if matches!(member.object, Expression::NewExpression(_)) =>
            {
                (
                    member.field.span.start,
                    "Do not access members directly from a `new` expression.",
                )
            }
            AstKind::NewExpression(new) if !is_simple_constructor(&new.callee) => (
                new.callee.span().start,
                "Do not use a complex expression as a constructor.",
            ),
            _ => return,
        };
        findings.push(source.finding(self.name(), offset, message.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnreadableNewExpression;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnreadableNewExpression, "playground/a.ts", code))
    }

    const MEMBER: &str = "Do not access members directly from a `new` expression.";
    const COMPLEX: &str = "Do not use a complex expression as a constructor.";

    #[test]
    fn reports_the_documented_unreadable_expressions() {
        assert_eq!(check("const bar = new Foo().getBar();"), [MEMBER]);
        assert_eq!(check("const Bar = new Foo().Bar;"), [MEMBER]);
        assert_eq!(check("const bar = new (foo().Bar)();"), [COMPLEX]);
        assert_eq!(check("const bar = new foo[Bar]();"), [COMPLEX]);
        assert_eq!(check("const time = new Date(text).getTime();"), [MEMBER]);
        assert_eq!(check("const first = new Foo()[0];"), [MEMBER]);
        assert_eq!(check("const a = new this.Foo();"), [COMPLEX]);
    }

    #[test]
    fn accepts_the_documented_readable_expressions() {
        for code in [
            "const foo = new Foo();\nconst bar = foo.getBar();",
            "const foo = new Foo();\nconst Bar = foo.Bar;",
            "const {Bar} = foo();\nconst bar = new Bar();",
            "const Bar = foo[Bar];\nconst bar = new Bar();",
            "const bar = new Foo.Bar();",
            "const bar = new a.b.C<T>();",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
