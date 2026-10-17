//! `unicorn/no-unnecessary-splice`: no `Array#splice()` where a simpler call says it — no change
//! at all, `shift()`, `unshift()`, `pop()`, `push()`, or `.length = 0` to empty the array
//! (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression, Program};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{
    MethodCall, has_comment, has_optional_chain, has_side_effect, is_length_minus_one_of,
    is_length_of, is_value_unused, method_call, object_text, skips_non_array_receiver,
    statement_removal, static_number, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUnnecessarySplice;

/// What replaces the call.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Replacement {
    /// Nothing: the call changes nothing.
    NoOp,
    /// `.length = 0`.
    Empty,
    /// A method, with the arguments kept from the third on.
    Method(&'static str, bool),
}

/// The number an argument is written as, when it is not spread.
fn number(argument: &Argument) -> Option<f64> {
    argument.as_expression().and_then(static_number)
}

/// Whether an argument is written `0` (not `-0`).
fn is_zero(argument: &Argument) -> bool {
    number(argument).is_some_and(|value| value == 0.0 && value.is_sign_positive())
}

/// Whether an argument is written as a number at most 0.
fn is_zero_or_negative(argument: &Argument) -> bool {
    number(argument).is_some_and(|value| value <= 0.0)
}

/// What a call can be replaced with (`getUnnecessarySpliceReplacement`).
fn replacement(arguments: &[Argument], object: &Expression) -> Option<Replacement> {
    let length_of = |argument: &Argument| {
        argument
            .as_expression()
            .is_some_and(|each| is_length_of(each, object))
    };
    match arguments {
        [] => Some(Replacement::NoOp),
        [start] if is_zero(start) => Some(Replacement::Empty),
        [start, count] if is_zero(start) && length_of(count) => Some(Replacement::Empty),
        [_, count] if is_zero_or_negative(count) => Some(Replacement::NoOp),
        [start, count] if is_zero(start) && number(count) == Some(1.0) => {
            Some(Replacement::Method("shift", false))
        }
        [start, count, ..] if is_zero(start) && is_zero_or_negative(count) => {
            Some(Replacement::Method("unshift", true))
        }
        [start, count]
            if start
                .as_expression()
                .is_some_and(|each| is_length_minus_one_of(each, object))
                && number(count) == Some(1.0) =>
        {
            Some(Replacement::Method("pop", false))
        }
        [start, count, ..] if length_of(start) && is_zero_or_negative(count) => {
            Some(Replacement::Method("push", true))
        }
        _ => None,
    }
}

/// The message of a replacement.
fn message(replacement: Replacement) -> String {
    match replacement {
        Replacement::NoOp => "This `splice()` call does not change the array.".to_owned(),
        Replacement::Empty => {
            "Prefer setting `.length = 0` over `.splice()` to empty an array.".to_owned()
        }
        Replacement::Method(name, _) => format!("Prefer `.{name}()` over `.splice()`."),
    }
}

/// Whether an argument may change something when evaluated.
fn argument_has_side_effect(argument: &Argument) -> bool {
    argument.as_expression().is_none_or(has_side_effect)
}

/// The fix of a call, when it is safe.
fn fix(
    node: &AstNode,
    source: &Source,
    method: &MethodCall,
    replacement: Replacement,
) -> Option<Fix> {
    let call = method.call;
    if !is_value_unused(source, node.id()) || has_comment(source, call.span) {
        return None;
    }
    let object = object_text(source, method.object);
    let replaced = |text: String| Fix {
        start: call.span.start,
        end: call.span.end,
        text,
    };
    match replacement {
        Replacement::NoOp => {
            let nodes = source.semantic.nodes();
            let statement = nodes.parent_node(node.id());
            let AstKind::ExpressionStatement(expression) = statement.kind() else {
                return None;
            };
            let removable = matches!(
                nodes.parent_kind(statement.id()),
                AstKind::Program(Program { .. })
                    | AstKind::BlockStatement(_)
                    | AstKind::FunctionBody(_)
            );
            let line_end = source.text[expression.span.end as usize..]
                .find('\n')
                .map_or(source.text.len(), |at| expression.span.end as usize + at);
            let comment_after = has_comment(
                source,
                Span::new(expression.span.end, u32::try_from(line_end).ok()?),
            );
            let effects = has_side_effect(method.object)
                || call.arguments.iter().any(argument_has_side_effect);
            if !removable || has_comment(source, expression.span) || comment_after || effects {
                return None;
            }
            let span = statement_removal(source, expression.span);
            Some(Fix {
                start: span.start,
                end: span.end,
                text: String::new(),
            })
        }
        Replacement::Empty => Some(replaced(format!("{object}.length = 0"))),
        Replacement::Method(name, keeps) => {
            let kept: Vec<&str> = if keeps {
                call.arguments
                    .iter()
                    .skip(2)
                    .map(|argument| text(source, argument.span()))
                    .collect()
            } else {
                Vec::new()
            };
            if name == "push" && call.arguments.iter().skip(2).any(argument_has_side_effect) {
                return None;
            }
            Some(replaced(format!("{object}.{name}({})", kept.join(", "))))
        }
    }
}

impl Rule for NoUnnecessarySplice {
    fn name(&self) -> &'static str {
        "unicorn/no-unnecessary-splice"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) =
            method_call(call).filter(|method| method.name == "splice" && method.is_plain())
        else {
            return;
        };
        if has_optional_chain(method.object) {
            return;
        }
        let Some(replacement) = replacement(&call.arguments, method.object) else {
            return;
        };
        let empty_used = replacement == Replacement::Empty && !is_value_unused(source, node.id());
        if empty_used || skips_non_array_receiver(source, method.object) {
            return;
        }
        let message = message(replacement);
        findings.push(match fix(node, source, &method, replacement) {
            Some(fix) => source.fixed(self.name(), method.property.start, message, fix),
            None => source.finding(self.name(), method.property.start, message),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnnecessarySplice;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnnecessarySplice, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&NoUnnecessarySplice, "playground/a.ts", code))
    }

    #[test]
    fn reports_and_fixes_the_documented_calls() {
        let cases = [
            (
                "array.splice(index, 0);",
                "This `splice()` call does not change the array.",
                "",
            ),
            (
                "array.splice(0, 1);",
                "Prefer `.shift()` over `.splice()`.",
                "array.shift();",
            ),
            (
                "array.splice(0, 0, item);",
                "Prefer `.unshift()` over `.splice()`.",
                "array.unshift(item);",
            ),
            (
                "array.splice(array.length - 1, 1);",
                "Prefer `.pop()` over `.splice()`.",
                "array.pop();",
            ),
            (
                "array.splice(array.length, 0, item);",
                "Prefer `.push()` over `.splice()`.",
                "array.push(item);",
            ),
            (
                "array.splice(0);",
                "Prefer setting `.length = 0` over `.splice()` to empty an array.",
                "array.length = 0;",
            ),
        ];
        for (code, message, after) in cases {
            assert_eq!(check(code), [message], "{code}");
            assert_eq!(fix(code), after, "{code}");
        }
    }

    #[test]
    fn fixes_with_care() {
        // A lone statement goes with its line; the receiver keeps its parentheses.
        assert_eq!(fix("a();\n  array.splice();\nb();\n"), "a();\nb();\n");
        assert_eq!(
            fix("(a || b).splice(0, a.length);"),
            "(a || b).splice(0, a.length);"
        );
        assert_eq!(fix("(x as T[]).splice(0);"), "(x as T[]).length = 0;");
        assert_eq!(
            fix("array.splice(0, -1, ...items);"),
            "array.unshift(...items);"
        );
        // Kept as written: a used value, a comment, a side effect.
        for code in [
            "const removed = array.splice(0, 1);",
            "array.splice(0, /* first */ 1);",
            "array.splice(array.length, 0, next());",
            "array.splice(next(), 0);",
            "array.splice(index, 0); // why",
            "if (a) array.splice(index, 0);",
        ] {
            let findings = lint(&NoUnnecessarySplice, "playground/a.ts", code);
            assert_eq!(findings.len(), 1, "{code}");
            assert_eq!(fixed(code, &findings), code, "{code}");
        }
        assert_eq!(fix("(array.splice(0, 1), b);"), "(array.shift(), b);");
    }

    #[test]
    fn accepts_other_calls() {
        for code in [
            "array.splice(index, 1);",
            "array.splice(1, 1);",
            "const removed = array.splice(0);",
            "array.splice(-0, 1);",
            "array?.splice(0, 1);",
            "a?.b.splice(0, 1);",
            "array.splice?.(0, 1);",
            "array[\"splice\"](0, 1);",
            "array.splice(other.length - 1, 1);",
            "const set = new Set(); set.splice(0, 1);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
