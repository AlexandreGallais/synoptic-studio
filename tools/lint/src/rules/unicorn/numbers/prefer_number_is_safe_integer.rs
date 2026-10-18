//! `unicorn/prefer-number-is-safe-integer`: `Number.isInteger(…)` and the integer checks
//! `value % 1 === 0`, `Math.floor(value) === value`, `Math.trunc(value) === value` and Lodash's
//! or Underscore's `isInteger` and `isSafeInteger` are written `Number.isSafeInteger(…)`, which
//! also refuses integers beyond 2⁵³ − 1 (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-number-is-safe-integer.md`). The checks are not equivalent, so the plugin
//! only suggests the change: no fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, CallExpression, Expression};
use oxc_semantic::AstNode;

use crate::expressions::{is_global, same_reference, unwrapped};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferNumberIsSafeInteger;

/// The message on a hand-written integer check.
const CHECK: &str = "Prefer `Number.isSafeInteger()` over this integer check.";

/// Whether an expression is the number literal of a value.
fn is_number(expression: &Expression, value: f64) -> bool {
    matches!(unwrapped(expression), Expression::NumericLiteral(each) if each.value.to_bits() == value.to_bits())
}

/// The object, the method and the only argument of a plain method call, `object.method(value)`.
fn method_call<'s, 'a>(
    call: &'s CallExpression<'a>,
) -> Option<(&'s Expression<'a>, &'a str, &'s Expression<'a>)> {
    let Expression::StaticMemberExpression(member) = &call.callee else {
        return None;
    };
    if call.optional || member.optional || call.arguments.len() != 1 {
        return None;
    }
    Some((
        &member.object,
        member.property.name.as_str(),
        call.arguments[0].as_expression()?,
    ))
}

/// The value `Math.floor(value)` or `Math.trunc(value)` truncates.
fn truncated<'s, 'a>(
    source: &Source,
    expression: &'s Expression<'a>,
) -> Option<&'s Expression<'a>> {
    let Expression::CallExpression(call) = unwrapped(expression) else {
        return None;
    };
    let (object, method, argument) = method_call(call)?;
    let math = matches!(object, Expression::Identifier(name) if name.name == "Math" && is_global(source, name));
    (math && matches!(method, "floor" | "trunc")).then_some(argument)
}

/// Whether `Number` names the global.
fn number_is_global(source: &Source) -> bool {
    !source
        .semantic
        .scoping()
        .symbol_names()
        .any(|name| name == "Number")
}

impl Rule for PreferNumberIsSafeInteger {
    fn name(&self) -> &'static str {
        "unicorn/prefer-number-is-safe-integer"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::CallExpression(call) => {
                if let Expression::StaticMemberExpression(member) = &call.callee
                    && !call.optional
                    && !member.optional
                    && member.property.name == "isInteger"
                    && matches!(&member.object, Expression::Identifier(object) if object.name == "Number" && is_global(source, object))
                {
                    let message =
                        "Prefer `Number.isSafeInteger()` over `Number.isInteger()`.".to_owned();
                    findings.push(source.finding(self.name(), member.property.span.start, message));
                    return;
                }
                let lodash = method_call(call).is_some_and(|(object, method, _)| {
                    matches!(object, Expression::Identifier(name) if matches!(name.name.as_str(), "_" | "lodash" | "underscore"))
                        && matches!(method, "isInteger" | "isSafeInteger")
                });
                if lodash && number_is_global(source) {
                    findings.push(source.finding(self.name(), call.span.start, CHECK.to_owned()));
                }
            }
            AstKind::BinaryExpression(binary)
                if binary.operator == BinaryOperator::StrictEquality =>
            {
                let modulo = |side: &Expression| {
                    matches!(unwrapped(side), Expression::BinaryExpression(inner)
                    if inner.operator == BinaryOperator::Remainder && is_number(&inner.right, 1.0))
                };
                let by_modulo = (is_number(&binary.right, 0.0) && modulo(&binary.left))
                    || (is_number(&binary.left, 0.0) && modulo(&binary.right));
                let by_math = truncated(source, &binary.left)
                    .is_some_and(|value| same_reference(value, &binary.right))
                    || truncated(source, &binary.right)
                        .is_some_and(|value| same_reference(value, &binary.left));
                if (by_modulo || by_math) && number_is_global(source) {
                    findings.push(source.finding(self.name(), binary.span.start, CHECK.to_owned()));
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferNumberIsSafeInteger;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferNumberIsSafeInteger, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_integer_checks() {
        // The documentation's cases.
        assert_eq!(
            places(
                "if (!Number.isInteger(index)) {\n\tthrow new Error('Expected an integer.');\n}"
            ),
            [(1, 13)]
        );
        assert_eq!(places("Number.isInteger(largeNumber);"), [(1, 8)]);
        assert_eq!(
            places("if (value % 1 === 0) {\n\tconsole.log('Integer');\n}"),
            [(1, 5)]
        );
        assert_eq!(
            places("if (_.isInteger(value)) {\n\tconsole.log('Integer');\n}"),
            [(1, 5)]
        );
        assert_eq!(
            messages(&lint(
                &PreferNumberIsSafeInteger,
                "scripts/a.ts",
                "Math.floor(a.b) === a.b;\n0 === x % 1;\nx === Math.trunc(x);"
            )),
            [
                "Prefer `Number.isSafeInteger()` over this integer check.",
                "Prefer `Number.isSafeInteger()` over this integer check.",
                "Prefer `Number.isSafeInteger()` over this integer check."
            ]
        );
    }

    #[test]
    fn leaves_other_checks() {
        for code in [
            "Number.isSafeInteger(a);",
            "Number?.isInteger(a);",
            "value % 2 === 0;",
            "Math.round(value) === value;",
            "Math.floor(a) === b;",
            "parseInt(value, 10) === value;",
            "(value | 0) === value;",
            "const Number = f;\nvalue % 1 === 0;",
            "x.isInteger(a);",
            "value % 1 == 0;",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
