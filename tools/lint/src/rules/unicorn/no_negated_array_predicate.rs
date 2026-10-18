//! `unicorn/no-negated-array-predicate`: no `!array.some(…)` nor `!array.every(…)` when the
//! swapped method with the predicate negated says it directly, De Morgan's laws in the small
//! (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{
    callback, has_comment, is_known_non_indexed, method_call, negated_text, text, unwrap,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoNegatedArrayPredicate;

/// The text that replaces a predicate: its operand when it is negated, its negation otherwise.
fn replacement_predicate(source: &Source, predicate: &Expression, concise: bool) -> String {
    match predicate {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            let inner = text(source, unary.argument.span());
            let needs_parentheses = concise
                && (matches!(unary.argument, Expression::SequenceExpression(_))
                    || inner.trim_start().starts_with('{'));
            if needs_parentheses {
                format!("({inner})")
            } else {
                inner.to_owned()
            }
        }
        _ => negated_text(source, predicate),
    }
}

impl Rule for NoNegatedArrayPredicate {
    fn name(&self) -> &'static str {
        "unicorn/no-negated-array-predicate"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::UnaryExpression(unary) = node.kind() else {
            return;
        };
        if unary.operator != UnaryOperator::LogicalNot {
            return;
        }
        let Expression::CallExpression(call) = unwrap(&unary.argument) else {
            return;
        };
        let Some(method) = method_call(call)
            .filter(|method| matches!(method.name, "some" | "every") && method.is_plain())
        else {
            return;
        };
        let Some(arguments) = method
            .arguments()
            .filter(|each| (1..=2).contains(&each.len()))
        else {
            return;
        };
        let Some(function) = callback(arguments[0]).filter(|function| !function.special) else {
            return;
        };
        let Some(predicate) = function.returned() else {
            return;
        };
        if call.type_arguments.is_some() || is_known_non_indexed(source, method.object) {
            return;
        }
        let replacement = if method.name == "some" {
            "every"
        } else {
            "some"
        };
        let message = format!(
            "Prefer `Array#{replacement}()` with a negated predicate over negating `Array#{}()`.",
            method.name
        );
        let after_bang = Span::new(unary.span.start + 1, unary.argument.span().start);
        let fixable = !has_comment(source, unary.span) && !text(source, after_bang).contains('\n');
        if !fixable {
            findings.push(source.finding(self.name(), method.property.start, message));
            return;
        }
        let predicate_span = predicate.span();
        let new_predicate = replacement_predicate(source, predicate, function.expression.is_some());
        let receiver_end = method.property.start;
        let space = source
            .text
            .get(..unary.span.start as usize)
            .and_then(|before| before.chars().last())
            .is_some_and(|last| last.is_alphanumeric() || last == '_' || last == '$');
        let fixed = format!(
            "{}{}{replacement}{}{new_predicate}{}",
            if space { " " } else { "" },
            text(source, Span::new(unary.argument.span().start, receiver_end)),
            text(source, Span::new(method.property.end, predicate_span.start)),
            text(source, Span::new(predicate_span.end, unary.span.end)),
        );
        findings.push(source.fixed(
            self.name(),
            method.property.start,
            message,
            Fix {
                start: unary.span.start,
                end: unary.span.end,
                text: fixed,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoNegatedArrayPredicate;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The fixed code, after one finding.
    fn fix(code: &str) -> String {
        let findings = lint(&NoNegatedArrayPredicate, "src/a.ts", code);
        assert_eq!(findings.len(), 1, "{code}");
        fixed(code, &findings)
    }

    #[test]
    fn reports_and_fixes_the_documented_negations() {
        let findings = lint(
            &NoNegatedArrayPredicate,
            "src/a.ts",
            "const isMissing = !array.some(element => isUnicorn(element));",
        );
        assert_eq!(
            messages(&findings),
            ["Prefer `Array#every()` with a negated predicate over negating `Array#some()`."]
        );
        // On `some`: `const isMissing = !array.` is 25 characters.
        assert_eq!(findings[0].column, 26);
        assert_eq!(
            fix("const isMissing = !array.some(element => isUnicorn(element));"),
            "const isMissing = array.every(element => !isUnicorn(element));"
        );
        assert_eq!(
            fix("const isNotEveryUnicorn = !array.every(element => isUnicorn(element));"),
            "const isNotEveryUnicorn = array.some(element => !isUnicorn(element));"
        );
        assert_eq!(
            fix("if (!array.some(element => !isUnicorn(element))) {}"),
            "if (array.every(element => isUnicorn(element))) {}"
        );
    }

    #[test]
    fn negates_with_parentheses_and_keeps_the_rest() {
        assert_eq!(
            fix("const a = !items.some((item) => item.a === 1 || item.b);"),
            "const a = items.every((item) => !(item.a === 1 || item.b));"
        );
        assert_eq!(
            fix("const a = !items.every(function (item) { return item.ok; }, context);"),
            "const a = items.some(function (item) { return !item.ok; }, context);"
        );
        assert_eq!(
            fix("const a = !items.some(item => !({ a } = item));"),
            "const a = items.every(item => ({ a } = item));"
        );
        assert_eq!(
            fix("function f() { return!items.some(x => x); }"),
            "function f() { return items.every(x => !x); }"
        );
        assert_eq!(
            fix("const a = !items.some(x => x)!;"),
            "const a = items.every(x => !x)!;"
        );
    }

    #[test]
    fn reports_without_a_fix_around_a_comment() {
        let code = "const a = !items.some(x => /* why */ x);";
        let findings = lint(&NoNegatedArrayPredicate, "src/a.ts", code);

        assert_eq!(findings.len(), 1);
        assert_eq!(fixed(code, &findings), code);
    }

    #[test]
    fn accepts_callbacks_it_cannot_negate_and_other_receivers() {
        for code in [
            "const isMissing = array.every(element => !isUnicorn(element));",
            "if (!array.every(Boolean)) {}",
            "if (!array.some(async x => x)) {}",
            "if (!array.some((x): boolean => x)) {}",
            "if (!array.some(x => { use(x); return x; })) {}",
            "if (!array.some(x => { use(x); })) {}",
            "if (!array?.some(x => x)) {}",
            "if (!array.some?.(x => x)) {}",
            "if (!array.some()) {}",
            "if (!array.some(x => x, a, b)) {}",
            "if (!array.some<number>(x => x)) {}",
            "if (!new Set().some(x => x)) {}",
            "if (!array.find(x => x)) {}",
            "if (-array.some(x => x)) {}",
        ] {
            assert_eq!(
                messages(&lint(&NoNegatedArrayPredicate, "src/a.ts", code)),
                NONE,
                "{code}"
            );
        }
    }
}
