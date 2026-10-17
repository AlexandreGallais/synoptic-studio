//! `unicorn/prefer-string-pad-start-end`: a value is padded with `padStart()` or `padEnd()`,
//! not by adding `'*'.repeat(width - value.length)` before or after it, nor by slicing a
//! padded concatenation (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`, for a name
//! padded to a numeric literal with a literal; the slices, a suggestion there, reported only.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryExpression, BinaryOperator, CallExpression, Expression, UnaryOperator,
    VariableDeclarationKind,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::known_values::is_known_non_string;
use super::syntax::{
    Binding, Value, binding_of, has_comment, method_call, parenthesized_text, same_reference,
    static_value, symbol_of, unwrap,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferStringPadStartEnd;

/// A `'x'.repeat(width)` call: the padding, its one character, and the width.
struct Repeat<'b, 'a> {
    /// The padding written.
    padding: &'b Expression<'a>,
    /// Its value.
    character: String,
    /// The width repeated.
    width: &'b Expression<'a>,
}

/// The string a padding is, through a `const`.
fn static_string(source: &Source, expression: &Expression) -> Option<String> {
    match static_value(source, expression)? {
        Value::Text(text) => Some(text),
        _ => None,
    }
}

/// The repeat call an expression is, of a one-unit string.
fn repeat_call<'b, 'a>(source: &Source, expression: &'b Expression<'a>) -> Option<Repeat<'b, 'a>> {
    let Expression::CallExpression(call) = expression else {
        return None;
    };
    let method = method_call(call)?;
    if method.name != "repeat" || !method.is_plain() || call.arguments.len() != 1 {
        return None;
    }
    let character = static_string(source, method.object)?;
    if character.encode_utf16().count() != 1 {
        return None;
    }
    Some(Repeat {
        padding: method.object,
        character,
        width: call.arguments[0].as_expression()?,
    })
}

/// The repeat call `'x'.repeat(width - target.length)`, with the width and the target.
fn repeat_length_call<'b, 'a>(
    source: &Source,
    expression: &'b Expression<'a>,
) -> Option<(Repeat<'b, 'a>, &'b Expression<'a>, &'b Expression<'a>)> {
    let repeat = repeat_call(source, expression)?;
    let Expression::BinaryExpression(width) = repeat.width else {
        return None;
    };
    if width.operator != BinaryOperator::Subtraction {
        return None;
    }
    let Expression::StaticMemberExpression(length) = &width.right else {
        return None;
    };
    if length.property.name != "length" {
        return None;
    }
    let (target_length, target) = (&width.left, &length.object);
    Some((repeat, target_length, target))
}

/// What a name holds when bound once, directly, by a `const` or as a function or class.
enum Held<'a> {
    /// A `const`'s initializer.
    Value(&'a Expression<'a>),
    /// A function or a class declared.
    Declaration,
    /// Nothing known.
    Unknown,
}

/// What a name is bound to.
fn held<'a>(source: &Source<'_, 'a>, expression: &Expression) -> Held<'a> {
    let Expression::Identifier(identifier) = expression else {
        return Held::Unknown;
    };
    let Some(symbol) = symbol_of(source, identifier) else {
        return Held::Unknown;
    };
    let scoping = source.semantic.scoping();
    let written = scoping
        .get_resolved_reference_ids(symbol)
        .iter()
        .any(|id| scoping.get_reference(*id).is_write());
    if written {
        return Held::Unknown;
    }
    match source
        .semantic
        .nodes()
        .kind(scoping.symbol_declaration(symbol))
    {
        AstKind::Function(_) | AstKind::Class(_) => Held::Declaration,
        _ => match binding_of(source, expression) {
            Some(Binding::Variable(declarator, VariableDeclarationKind::Const)) => {
                declarator.init.as_ref().map_or(Held::Unknown, Held::Value)
            }
            _ => Held::Unknown,
        },
    }
}

/// Whether an expression is written as a number: a numeric literal, a negation, or arithmetic
/// other than `+`.
fn is_number_syntax(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::NumericLiteral(_) => true,
        Expression::UnaryExpression(unary) => matches!(
            unary.operator,
            UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus | UnaryOperator::BitwiseNot
        ),
        Expression::BinaryExpression(binary) => matches!(
            binary.operator,
            BinaryOperator::Subtraction
                | BinaryOperator::Multiplication
                | BinaryOperator::Division
                | BinaryOperator::Remainder
                | BinaryOperator::Exponential
                | BinaryOperator::BitwiseAnd
                | BinaryOperator::BitwiseOR
                | BinaryOperator::BitwiseXOR
                | BinaryOperator::ShiftLeft
                | BinaryOperator::ShiftRight
                | BinaryOperator::ShiftRightZeroFill
        ),
        _ => false,
    }
}

/// Whether an expression is written as a boolean: a literal, `!`, or a comparison.
fn is_boolean_syntax(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::BooleanLiteral(_) => true,
        Expression::UnaryExpression(unary) => unary.operator == UnaryOperator::LogicalNot,
        Expression::BinaryExpression(binary) => {
            binary.operator.is_equality()
                || binary.operator.is_compare()
                || matches!(
                    binary.operator,
                    BinaryOperator::In | BinaryOperator::Instanceof
                )
        }
        _ => false,
    }
}

/// Whether a padded value is plainly no string, which has no `padStart()`.
fn is_clearly_non_string(source: &Source, expression: &Expression) -> bool {
    let is_object_like = |each: &Expression| {
        matches!(
            unwrap(each),
            Expression::ArrayExpression(_)
                | Expression::ArrowFunctionExpression(_)
                | Expression::ClassExpression(_)
                | Expression::FunctionExpression(_)
                | Expression::ObjectExpression(_)
        )
    };
    let value = match held(source, expression) {
        Held::Declaration => return true,
        Held::Value(value) => value,
        Held::Unknown => expression,
    };
    is_object_like(expression)
        || is_object_like(value)
        || static_value(source, value).is_some_and(|each| !matches!(each, Value::Text(_)))
        || is_number_syntax(value)
        || is_boolean_syntax(value)
        || is_known_non_string(source, expression)
}

/// Whether a padding is written as a literal string.
fn is_literal_padding(expression: &Expression) -> bool {
    match expression {
        Expression::StringLiteral(_) => true,
        Expression::TemplateLiteral(template) => template.expressions.is_empty(),
        _ => false,
    }
}

/// The call that replaces a padding: `target.padStart(width)` or with its padding.
fn padded(
    source: &Source,
    method: &str,
    target: &Expression,
    width: &Expression,
    repeat: &Repeat,
) -> String {
    let padding = if repeat.character == " " {
        String::new()
    } else {
        format!(", {}", parenthesized_text(source, repeat.padding))
    };
    format!(
        "{}.{method}({}{padding})",
        parenthesized_text(source, target),
        parenthesized_text(source, width)
    )
}

impl PreferStringPadStartEnd {
    /// Reports a padding by concatenation, fixed when the target is a name and the width a
    /// number written out.
    fn check_concatenation(
        &self,
        source: &Source,
        binary: &BinaryExpression,
        findings: &mut Vec<Finding>,
    ) {
        if binary.operator != BinaryOperator::Addition {
            return;
        }
        let start = repeat_length_call(source, &binary.left)
            .filter(|(_, _, target)| same_reference(&binary.right, target))
            .map(|found| ("padStart", found, &binary.right));
        let found = start.or_else(|| {
            repeat_length_call(source, &binary.right)
                .filter(|(_, _, target)| same_reference(&binary.left, target))
                .map(|found| ("padEnd", found, &binary.left))
        });
        let Some((method, (repeat, width, _), target)) = found else {
            return;
        };
        if is_clearly_non_string(source, target) {
            return;
        }
        let message = format!("Prefer `String#{method}()` over manual string padding.");
        let fixable = matches!(target, Expression::Identifier(_))
            && matches!(width, Expression::NumericLiteral(_))
            && is_literal_padding(repeat.padding)
            && !has_comment(source, binary.span);
        if !fixable {
            findings.push(source.finding(self.name(), binary.span.start, message));
            return;
        }
        findings.push(source.fixed(
            self.name(),
            binary.span.start,
            message,
            Fix {
                start: binary.span.start,
                end: binary.span.end,
                text: padded(source, method, target, width, &repeat),
            },
        ));
    }

    /// Reports a padding sliced to its width, `('*'.repeat(10) + bar).slice(-10)`.
    fn check_slice(&self, source: &Source, call: &CallExpression, findings: &mut Vec<Finding>) {
        let Some(method) = method_call(call) else {
            return;
        };
        if method.name != "slice" || !method.is_plain() || !(1..=2).contains(&call.arguments.len())
        {
            return;
        }
        let Expression::BinaryExpression(sum) = method.object else {
            return;
        };
        if sum.operator != BinaryOperator::Addition {
            return;
        }
        let arguments: Vec<Option<&Expression>> = call
            .arguments
            .iter()
            .map(|each| each.as_expression())
            .collect();
        let start_width = match arguments.first() {
            Some(Some(Expression::UnaryExpression(unary)))
                if unary.operator == UnaryOperator::UnaryNegation =>
            {
                Some(&unary.argument)
            }
            _ => None,
        };
        let pads_start = arguments.len() == 1
            && repeat_call(source, &sum.left).is_some_and(|repeat| {
                matches!(repeat.width, Expression::NumericLiteral(_))
                    && start_width.is_some_and(|width| same_reference(repeat.width, width))
            })
            && matches!(sum.right, Expression::Identifier(_))
            && !is_clearly_non_string(source, &sum.right);
        let pads_end = !pads_start
            && arguments.len() == 2
            && matches!(arguments[0], Some(Expression::NumericLiteral(zero)) if zero.value == 0.0)
            && repeat_call(source, &sum.right).is_some_and(|repeat| {
                matches!(repeat.width, Expression::NumericLiteral(_))
                    && arguments[1].is_some_and(|width| same_reference(repeat.width, width))
            })
            && matches!(sum.left, Expression::Identifier(_))
            && !is_clearly_non_string(source, &sum.left);
        let method = if pads_start {
            "padStart"
        } else if pads_end {
            "padEnd"
        } else {
            return;
        };
        findings.push(source.finding(
            self.name(),
            call.span().start,
            format!("Prefer `String#{method}()` over manual string padding."),
        ));
    }
}

impl Rule for PreferStringPadStartEnd {
    fn name(&self) -> &'static str {
        "unicorn/prefer-string-pad-start-end"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::BinaryExpression(binary) => self.check_concatenation(source, binary, findings),
            AstKind::CallExpression(call) => self.check_slice(source, call, findings),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferStringPadStartEnd;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferStringPadStartEnd, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferStringPadStartEnd, "playground/a.ts", code),
        )
    }

    #[test]
    fn fixes_the_documented_paddings() {
        let start = "const formatted = '*'.repeat(10 - name.length) + name;";
        assert_eq!(
            check(start),
            ["Prefer `String#padStart()` over manual string padding."]
        );
        assert_eq!(fix(start), "const formatted = name.padStart(10, '*');");
        let end = "const padded = value + ' '.repeat(5 - value.length);";
        assert_eq!(fix(end), "const padded = value.padEnd(5);");
        assert_eq!(
            check("const a = '0'.repeat(width - text.length) + text;"),
            ["Prefer `String#padStart()` over manual string padding."]
        );
        assert_eq!(
            fix("const a = '0'.repeat(width - text.length) + text;"),
            "const a = '0'.repeat(width - text.length) + text;"
        );
    }

    #[test]
    fn reports_the_documented_slices_without_a_fix() {
        let start = "const foo = ('*'.repeat(10) + bar).slice(-10);";
        assert_eq!(
            check(start),
            ["Prefer `String#padStart()` over manual string padding."]
        );
        assert_eq!(fix(start), start);
        assert_eq!(
            check("const foo = (bar + '*'.repeat(10)).slice(0, 10);"),
            ["Prefer `String#padEnd()` over manual string padding."]
        );
    }

    #[test]
    fn accepts_the_documented_and_non_string_cases() {
        for code in [
            "const col1 = 'ID'.padEnd(10);",
            "const hours = String(hours).padStart(2, '0');",
            "const a = '**'.repeat(10 - name.length) + name;",
            "const a = '*'.repeat(10 - name.length) + other;",
            "const n = 5; const a = '*'.repeat(10 - n.length) + n;",
            "const a = [1]; const b = ' '.repeat(3 - a.length) + a;",
            "function f(list: number[]) { return ' '.repeat(3 - list.length) + list; }",
            "const foo = ('*'.repeat(10) + bar).slice(-9);",
            "const foo = (bar + '*'.repeat(10)).slice(1, 10);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
