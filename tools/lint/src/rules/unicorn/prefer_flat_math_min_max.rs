//! `unicorn/prefer-flat-math-min-max`: no `Math.min()` nested in `Math.min()`, nor `Math.max()` in
//! `Math.max()`, since each takes any number of arguments (`eslint-plugin-unicorn` 78.0.0).
//! Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, CallExpression, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{has_comment, method_call, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferFlatMathMinMax;

/// The method of a plain `Math.min(…)` or `Math.max(…)` call.
fn min_or_max<'a>(call: &CallExpression<'a>) -> Option<&'a str> {
    let method = method_call(call)?;
    let math = matches!(method.object, Expression::Identifier(object) if object.name == "Math");
    (math && method.is_plain() && matches!(method.name, "min" | "max")).then_some(method.name)
}

/// The call an argument is, when it is the same `Math` method.
fn same_call<'b, 'a>(argument: &'b Argument<'a>, method: &str) -> Option<&'b CallExpression<'a>> {
    match argument.as_expression()? {
        Expression::CallExpression(call) if min_or_max(call) == Some(method) => Some(call),
        _ => None,
    }
}

/// The texts of a call's arguments, those of nested calls of the same method flattened.
fn flattened(source: &Source, call: &CallExpression, method: &str, texts: &mut Vec<String>) {
    for argument in &call.arguments {
        if let Some(inner) = same_call(argument, method) {
            flattened(source, inner, method, texts);
        } else {
            let written = text(source, argument.span());
            let sequence = matches!(argument, Argument::SequenceExpression(_));
            texts.push(if sequence {
                format!("({written})")
            } else {
                written.to_owned()
            });
        }
    }
}

impl Rule for PreferFlatMathMinMax {
    fn name(&self) -> &'static str {
        "unicorn/prefer-flat-math-min-max"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = min_or_max(call) else {
            return;
        };
        let nested = call
            .arguments
            .iter()
            .any(|argument| same_call(argument, method).is_some());
        let inside_same = matches!(
            source.semantic.nodes().parent_kind(node.id()),
            AstKind::CallExpression(parent) if min_or_max(parent) == Some(method)
                && parent.callee.node_id() != node.id()
        );
        if !nested || inside_same {
            return;
        }
        let message = format!("Prefer a flat `Math.{method}()` call instead of nested calls.");
        if has_comment(source, call.span) {
            findings.push(source.finding(self.name(), call.span.start, message));
            return;
        }
        let mut texts = Vec::new();
        flattened(source, call, method, &mut texts);
        let fix = Fix {
            start: call.span.start,
            end: call.span.end,
            text: format!("{}({})", text(source, call.callee.span()), texts.join(", ")),
        };
        findings.push(source.fixed(self.name(), call.span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferFlatMathMinMax;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferFlatMathMinMax, "src/a.ts", code))
    }

    #[test]
    fn reports_and_flattens_the_documented_nested_calls() {
        let findings = lint(
            &PreferFlatMathMinMax,
            "src/a.ts",
            "const biggest = Math.max(Math.max(a, b), c);",
        );
        assert_eq!(
            messages(&findings),
            ["Prefer a flat `Math.max()` call instead of nested calls."]
        );
        // On the outer call: `const biggest = ` is 16 characters.
        assert_eq!(findings[0].column, 17);
        assert_eq!(
            fix("const biggest = Math.max(Math.max(a, b), c);"),
            "const biggest = Math.max(a, b, c);"
        );
        assert_eq!(
            fix("const smallest = Math.min(a, Math.min(b, c));"),
            "const smallest = Math.min(a, b, c);"
        );
        assert_eq!(
            fix("const m = Math.min(Math.min(Math.min(a, (b, c)), ...d), e);"),
            "const m = Math.min(a, (b, c), ...d, e);"
        );
    }

    #[test]
    fn reports_without_a_fix_around_a_comment() {
        let code = "const m = Math.min(Math.min(a, b) /* low */, c);";
        let findings = lint(&PreferFlatMathMinMax, "src/a.ts", code);

        assert_eq!(findings.len(), 1);
        assert_eq!(fixed(code, &findings), code);
    }

    #[test]
    fn accepts_clamps_and_flat_calls() {
        for code in [
            "const clamped = Math.max(Math.min(value, upper), lower);",
            "const biggest = Math.max(a, b, c);",
            "const m = Math?.max(Math.max(a, b), c);",
            "const m = Math.max(Math?.max(a, b), c);",
            "const m = Other.max(Other.max(a, b), c);",
            "const m = Math.max(f(Math.max(a, b)), c);",
        ] {
            assert_eq!(
                messages(&lint(&PreferFlatMathMinMax, "src/a.ts", code)),
                NONE,
                "{code}"
            );
        }
    }
}
