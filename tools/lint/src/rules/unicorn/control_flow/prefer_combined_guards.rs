//! `unicorn/prefer-combined-guards`: two adjacent guards — `if`s without `else` whose body is
//! one `return`, `throw`, `break`, `continue` or `process.exit()` — leaving the same way are
//! joined with `||` (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-combined-guards.md`).
//! Their conditions must be simple: no `&&`, `??`, conditional, assignment, sequence, arrow nor
//! `yield`, an `||` chain of simple ones allowed. As the plugin did under TypeScript's parser
//! without type information, an exit that reads a name, `this` or `super` is never joined: each
//! guard may narrow its type. Autofixed when no comment lies in or before the two.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, IfStatement, LogicalOperator, Statement};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::exits::is_process_exit;
use crate::expressions::{has_comment_in, siblings, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferCombinedGuards;

/// The rule's message.
const MESSAGE: &str = "Combine this guard with the previous one using `||`.";

/// Whether a condition is simple; an `||` chain of simple ones is, unless under a negation.
fn is_simple(expression: &Expression, or_allowed: bool) -> bool {
    match expression {
        Expression::TSAsExpression(inner) => is_simple(&inner.expression, or_allowed),
        Expression::TSSatisfiesExpression(inner) => is_simple(&inner.expression, or_allowed),
        Expression::TSNonNullExpression(inner) => is_simple(&inner.expression, or_allowed),
        Expression::TSTypeAssertion(inner) => is_simple(&inner.expression, or_allowed),
        Expression::LogicalExpression(logical) => {
            or_allowed
                && logical.operator == LogicalOperator::Or
                && is_simple(&logical.left, true)
                && is_simple(&logical.right, true)
        }
        Expression::UnaryExpression(unary) => is_simple(&unary.argument, false),
        Expression::AwaitExpression(inner) => is_simple(&inner.argument, false),
        Expression::ConditionalExpression(_)
        | Expression::AssignmentExpression(_)
        | Expression::SequenceExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::YieldExpression(_) => false,
        _ => true,
    }
}

/// The exit of a guard: its only statement, when it leaves.
fn exit<'s, 'a>(statement: &'s Statement<'a>) -> Option<&'s Statement<'a>> {
    let Statement::IfStatement(guard) = statement else {
        return None;
    };
    if guard.alternate.is_some() {
        return None;
    }
    let only = match &guard.consequent {
        Statement::BlockStatement(block) => match block.body.as_slice() {
            [only] => only,
            _ => return None,
        },
        other => other,
    };
    let leaves = match only {
        Statement::ReturnStatement(_)
        | Statement::ThrowStatement(_)
        | Statement::BreakStatement(_)
        | Statement::ContinueStatement(_) => true,
        Statement::ExpressionStatement(expression) => is_process_exit(&expression.expression),
        _ => false,
    };
    leaves.then_some(only)
}

/// The text of what an exit returns, throws, names or calls.
fn exit_text<'a>(source: &Source<'_, 'a>, exit: &Statement) -> &'a str {
    let span = match exit {
        Statement::ReturnStatement(each) => each.argument.as_ref().map(GetSpan::span),
        Statement::ThrowStatement(each) => Some(each.argument.span()),
        Statement::BreakStatement(each) => each.label.as_ref().map(|label| label.span),
        Statement::ContinueStatement(each) => each.label.as_ref().map(|label| label.span),
        Statement::ExpressionStatement(each) => Some(each.expression.span()),
        _ => None,
    };
    span.map_or("", |span| text(source, span))
}

/// The search of an exit's values for what makes joining unsafe.
#[derive(Default)]
struct Unsafe {
    found: bool,
}

impl<'a> Visit<'a> for Unsafe {
    fn visit_identifier_reference(&mut self, _name: &oxc_ast::ast::IdentifierReference<'a>) {
        self.found = true;
    }

    fn visit_expression(&mut self, expression: &Expression<'a>) {
        if matches!(
            expression,
            Expression::ThisExpression(_)
                | Expression::Super(_)
                | Expression::TaggedTemplateExpression(_)
        ) {
            self.found = true;
        }
        walk::walk_expression(self, expression);
    }

    fn visit_ts_type(&mut self, _written: &oxc_ast::ast::TSType<'a>) {}
}

/// Whether an exit's values read a name, `this`, `super`, or hold a tagged template.
fn is_unsafe(exit: &Statement) -> bool {
    let mut search = Unsafe::default();
    match exit {
        Statement::ExpressionStatement(each) => {
            if let Expression::CallExpression(call) = &each.expression {
                for argument in &call.arguments {
                    search.visit_argument(argument);
                }
            }
        }
        Statement::ReturnStatement(each) => {
            if let Some(argument) = &each.argument {
                search.visit_expression(argument);
            }
        }
        Statement::ThrowStatement(each) => search.visit_expression(&each.argument),
        _ => {}
    }
    search.found
}

/// The text of a condition in the joined test.
fn condition_text(source: &Source, condition: &Expression) -> String {
    let inner = text(source, condition.span());
    let wrapped = matches!(
        condition,
        Expression::TSAsExpression(_)
            | Expression::TSSatisfiesExpression(_)
            | Expression::TSTypeAssertion(_)
    ) || matches!(condition, Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::Coalesce);
    if wrapped {
        format!("({inner})")
    } else {
        inner.to_owned()
    }
}

impl PreferCombinedGuards {
    /// The finding on a guard joined to the one before.
    fn check(
        &self,
        source: &Source,
        previous: &IfStatement,
        current: &IfStatement,
        exits: [&Statement; 2],
    ) -> Option<Finding> {
        let [before, after] = exits;
        let same_kind = std::mem::discriminant(before) == std::mem::discriminant(after);
        if !same_kind
            || exit_text(source, before) != exit_text(source, after)
            || is_unsafe(after)
            || !is_simple(&previous.test, true)
            || !is_simple(&current.test, true)
        {
            return None;
        }
        let range = Span::new(previous.span.start, current.span.end);
        let leading = source
            .program
            .comments
            .iter()
            .any(|comment| comment.attached_to == previous.span.start);
        if leading || has_comment_in(source, range) {
            return Some(source.finding(self.name(), current.span.start, MESSAGE.to_owned()));
        }
        let close = current.test.span().end
            + text(
                source,
                Span::new(current.test.span().end, current.consequent.span().start),
            )
            .find(')')
            .map_or(0, |at| crate::expressions::offset(at) + 1);
        let fix = Fix {
            start: range.start,
            end: range.end,
            text: format!(
                "if ({} || {}){}",
                condition_text(source, &previous.test),
                condition_text(source, &current.test),
                text(source, Span::new(close, range.end))
            ),
        };
        Some(source.fixed(self.name(), current.span.start, MESSAGE.to_owned(), fix))
    }
}

impl Rule for PreferCombinedGuards {
    fn name(&self) -> &'static str {
        "unicorn/prefer-combined-guards"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::IfStatement(current) = node.kind() else {
            return;
        };
        let Some((list, index)) = siblings(source, node) else {
            return;
        };
        if index == 0 {
            return;
        }
        let (Some(after), Some(before)) = (exit(&list[index]), exit(&list[index - 1])) else {
            return;
        };
        let Statement::IfStatement(previous) = &list[index - 1] else {
            return;
        };
        findings.extend(self.check(source, previous, current, [before, after]));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferCombinedGuards;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferCombinedGuards, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn joins_guards_leaving_the_same_way() {
        // The documentation's first and third cases.
        assert_eq!(
            check("function check(context) {\n\tif (context.finished) {\n\t\treturn;\n\t}\n\tif (context.cancelled) {\n\t\treturn;\n\t}\n}"),
            (
                vec![(5, 2)],
                "function check(context) {\n\tif (context.finished || context.cancelled) {\n\t\treturn;\n\t}\n}".to_owned()
            )
        );
        assert_eq!(
            check("for (const item of items) {\n\tif (item.hidden) {\n\t\tcontinue;\n\t}\n\tif (item.disabled) {\n\t\tcontinue;\n\t}\n\tprocessItem(item);\n}").1,
            "for (const item of items) {\n\tif (item.hidden || item.disabled) {\n\t\tcontinue;\n\t}\n\tprocessItem(item);\n}"
        );
        // Literal values, `process.exit()`, an `||` chain, an assertion parenthesized.
        assert_eq!(
            check("function f() { if (a) return 1; if (b || c) return 1; }").1,
            "function f() { if (a || b || c) return 1; }"
        );
        assert_eq!(
            check("if (a) { process.exit(1); }\nif (b as boolean) { process.exit(1); }").1,
            "if (a || (b as boolean)) { process.exit(1); }"
        );
    }

    #[test]
    fn leaves_guards_that_differ_read_names_or_are_compound() {
        for code in [
            "function validate(value) {\n\tif (value < 0) {\n\t\tthrow invalidValue;\n\t}\n\tif (!Number.isFinite(value)) {\n\t\tthrow invalidValue;\n\t}\n}",
            "function check(value: number, finished: boolean, cancelled: boolean) {\n\tif (finished) {\n\t\treturn value;\n\t}\n\tif (cancelled) {\n\t\treturn value;\n\t}\n}",
            "if (firstCondition) {\n\tcount++;\n}\nif (secondCondition) {\n\tcount++;\n}",
            "function f() { if (a) { return; } if (!b && !c) { return; } }",
            "function f() { if (a) { return 1; } if (b) { return 2; } }",
            "function f() { if (a) { return; } if (b) { throw e; } }",
            "function f() { if (a) { g(); return; } if (b) { g(); return; } }",
            "function f() { if (a) { return; } else { b(); } if (c) { return; } }",
            "function f() { if (a) { return t`x`; } if (b) { return t`x`; } }",
            "function f() { if (a) { return; } }",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_without_fix_around_comments() {
        let code =
            "function f() {\n  // first reason\n  if (a) { return; }\n  if (b) { return; }\n}";
        assert_eq!(check(code), (vec![(4, 3)], code.to_owned()));
    }
}
