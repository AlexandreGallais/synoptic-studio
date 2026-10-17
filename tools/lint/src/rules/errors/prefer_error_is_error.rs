//! `unicorn/prefer-error-is-error`: `value instanceof Error` and the brand check
//! `Object.prototype.toString.call(value) === '[object Error]'` become `Error.isError(value)`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-error-is-error.md`), negated for `!==`.
//! Left when `Error` is a local value. Autofixed unless a comment is inside.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{has_comment_in, symbol, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferErrorIsError;

/// The rule's message.
const MESSAGE: &str = "Prefer `Error.isError(…)`.";

/// Whether a name is a value the file declares, a type left out.
fn declares_value(source: &Source, name: &str) -> bool {
    let scoping = source.semantic.scoping();
    scoping
        .symbol_ids()
        .any(|each| scoping.symbol_name(each) == name && scoping.symbol_flags(each).is_value())
}

/// The value given to `Object.prototype.toString.call(…)` or `{}.toString.call(…)`.
fn brand_checked<'s, 'a>(
    source: &Source,
    expression: &'s Expression<'a>,
) -> Option<&'s Expression<'a>> {
    let Expression::CallExpression(call) = expression else {
        return None;
    };
    let Expression::StaticMemberExpression(method) = &call.callee else {
        return None;
    };
    if call.optional
        || method.optional
        || method.property.name != "call"
        || call.arguments.len() != 1
    {
        return None;
    }
    let Expression::StaticMemberExpression(to_string) = &method.object else {
        return None;
    };
    if to_string.optional || to_string.property.name != "toString" {
        return None;
    }
    let from_prototype = matches!(&to_string.object, Expression::StaticMemberExpression(prototype)
        if !prototype.optional && prototype.property.name == "prototype"
            && matches!(&prototype.object, Expression::Identifier(object) if object.name == "Object" && symbol(source, object).is_none() && !declares_value(source, "Object")));
    let from_empty = matches!(&to_string.object, Expression::ObjectExpression(object) if object.properties.is_empty());
    (from_prototype || from_empty)
        .then(|| call.arguments[0].as_expression())
        .flatten()
}

/// Whether an expression is the string `'[object Error]'`.
fn is_error_tag(expression: &Expression) -> bool {
    matches!(expression, Expression::StringLiteral(literal) if literal.value == "[object Error]")
}

impl Rule for PreferErrorIsError {
    fn name(&self) -> &'static str {
        "unicorn/prefer-error-is-error"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::BinaryExpression(binary) = node.kind() else {
            return;
        };
        let checked = match binary.operator {
            BinaryOperator::Instanceof if binary.right.is_specific_id("Error") => {
                Some(&binary.left)
            }
            BinaryOperator::StrictEquality
            | BinaryOperator::StrictInequality
            | BinaryOperator::Equality
            | BinaryOperator::Inequality => {
                if is_error_tag(&binary.right) {
                    brand_checked(source, &binary.left)
                } else if is_error_tag(&binary.left) {
                    brand_checked(source, &binary.right)
                } else {
                    None
                }
            }
            _ => None,
        };
        let Some(checked) = checked else {
            return;
        };
        if declares_value(source, "Error") {
            return;
        }
        if has_comment_in(source, binary.span) {
            findings.push(source.finding(self.name(), binary.span.start, MESSAGE.to_owned()));
            return;
        }
        let negated = matches!(
            binary.operator,
            BinaryOperator::StrictInequality | BinaryOperator::Inequality
        );
        let inner = text(source, checked.span());
        let inner = if matches!(checked, Expression::SequenceExpression(_)) {
            format!("({inner})")
        } else {
            inner.to_owned()
        };
        let fix = Fix {
            start: binary.span.start,
            end: binary.span.end,
            text: format!("{}Error.isError({inner})", if negated { "!" } else { "" }),
        };
        findings.push(source.fixed(self.name(), binary.span.start, MESSAGE.to_owned(), fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferErrorIsError;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferErrorIsError, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn checks_errors_with_error_is_error() {
        // The documentation's three cases.
        assert_eq!(
            check("error instanceof Error;"),
            (vec![(1, 1)], "Error.isError(error);".to_owned())
        );
        assert_eq!(
            check("Object.prototype.toString.call(error) === '[object Error]';").1,
            "Error.isError(error);"
        );
        assert_eq!(
            check("Object.prototype.toString.call(error) !== '[object Error]';").1,
            "!Error.isError(error);"
        );
        assert_eq!(
            check("const a = '[object Error]' == {}.toString.call(b);").1,
            "const a = Error.isError(b);"
        );
    }

    #[test]
    fn leaves_other_checks_and_a_local_error() {
        for code in [
            "error instanceof TypeError;",
            "Object.prototype.toString.call(error) === '[object Array]';",
            "const Error = f;\nerror instanceof Error;",
            "const Object = f;\nObject.prototype.toString.call(error) === '[object Error]';",
            "x.toString.call(error) === '[object Error]';",
            "error < Error;",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let typed = "type Error = string;\nerror instanceof Error;";
        assert_eq!(check(typed).0, [(2, 1)]);
        let commented = "error /* why */ instanceof Error;";
        assert_eq!(check(commented), (vec![(1, 1)], commented.to_owned()));
    }
}
