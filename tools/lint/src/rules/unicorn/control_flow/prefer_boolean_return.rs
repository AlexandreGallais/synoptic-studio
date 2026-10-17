//! `unicorn/prefer-boolean-return`: an `if` whose branches, or whose branch and the `return`
//! right after it, return opposite boolean literals is replaced by the returned condition
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-boolean-return.md`): itself when it is a
//! boolean at run time, negated with `!` for `return false` first, wrapped in `Boolean(…)`
//! otherwise. The last guard of a run returning the same literal is left. Autofixed unless a
//! comment would be lost or `Boolean` is a local name.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression, IfStatement, Statement, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{has_comment_in, is_global, is_global_call, negated_text, siblings, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferBooleanReturn;

/// The rule's message.
const MESSAGE: &str = "Return the boolean expression directly.";

/// A branch's only statement, through blocks holding one statement besides empty ones.
fn body<'s, 'a>(branch: &'s Statement<'a>) -> &'s Statement<'a> {
    if let Statement::BlockStatement(block) = branch {
        let kept: Vec<&Statement> = block
            .body
            .iter()
            .filter(|each| !matches!(each, Statement::EmptyStatement(_)))
            .collect();
        if let [only] = kept.as_slice() {
            return body(only);
        }
    }
    branch
}

/// The boolean literal a statement returns.
fn returned_boolean(statement: &Statement) -> Option<bool> {
    let Statement::ReturnStatement(statement) = statement else {
        return None;
    };
    match statement.argument.as_ref()?.get_inner_expression() {
        Expression::BooleanLiteral(literal) => Some(literal.value),
        _ => None,
    }
}

/// Whether a value is a boolean at run time, from its syntax alone.
fn is_runtime_boolean(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::BooleanLiteral(_) => true,
        Expression::UnaryExpression(unary) => {
            matches!(
                unary.operator,
                UnaryOperator::LogicalNot | UnaryOperator::Delete
            )
        }
        Expression::BinaryExpression(binary) => {
            binary.operator.is_equality()
                || binary.operator.is_compare()
                || matches!(
                    binary.operator,
                    BinaryOperator::In | BinaryOperator::Instanceof
                )
        }
        Expression::LogicalExpression(logical) => {
            is_runtime_boolean(source, &logical.left) && is_runtime_boolean(source, &logical.right)
        }
        Expression::CallExpression(call) => {
            is_global_call(source, expression, "Boolean")
                || (!call.optional
                    && matches!(&call.callee, Expression::StaticMemberExpression(member)
                        if !member.optional
                            && member.property.name == "isArray"
                            && matches!(&member.object, Expression::Identifier(object)
                                if object.name == "Array" && is_global(source, object))))
        }
        _ => false,
    }
}

/// Whether `Boolean` names the global in the file.
fn boolean_is_global(source: &Source) -> bool {
    let scoping = source.semantic.scoping();
    !scoping.symbol_names().any(|name| name == "Boolean")
}

impl PreferBooleanReturn {
    /// The finding on an `if` and the statement giving its other value, replacing a range.
    fn problem(
        &self,
        source: &Source,
        statement: &IfStatement,
        consequent: &Statement,
        alternate: &Statement,
        range: Span,
    ) -> Option<Finding> {
        if matches!(statement.test, Expression::ConditionalExpression(_)) {
            return None;
        }
        let first = returned_boolean(consequent)?;
        if returned_boolean(alternate)? == first {
            return None;
        }
        let negated = !first;
        let after = &source.text[alternate.span().end as usize..];
        let trailing_comment = source.program.comments.iter().any(|comment| {
            comment.span.start >= alternate.span().end
                && !after[..(comment.span.start - alternate.span().end) as usize].contains('\n')
        });
        let wraps = !negated && !is_runtime_boolean(source, &statement.test);
        if has_comment_in(source, range)
            || trailing_comment
            || (wraps && !boolean_is_global(source))
        {
            return Some(source.finding(self.name(), statement.span.start, MESSAGE.to_owned()));
        }
        let test = &statement.test;
        let value = if negated {
            negated_text(source, test)
        } else if wraps {
            let inner = text(source, test.span());
            if matches!(test, Expression::SequenceExpression(_)) {
                format!("Boolean(({inner}))")
            } else {
                format!("Boolean({inner})")
            }
        } else {
            text(source, test.span()).to_owned()
        };
        let fix = Fix {
            start: range.start,
            end: range.end,
            text: format!("return {value};"),
        };
        Some(source.fixed(self.name(), statement.span.start, MESSAGE.to_owned(), fix))
    }
}

impl Rule for PreferBooleanReturn {
    fn name(&self) -> &'static str {
        "unicorn/prefer-boolean-return"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::IfStatement(statement) = node.kind() else {
            return;
        };
        let consequent = body(&statement.consequent);
        if let Some(alternate) = &statement.alternate {
            findings.extend(self.problem(
                source,
                statement,
                consequent,
                body(alternate),
                statement.span,
            ));
            return;
        }
        let Some((list, index)) = siblings(source, node) else {
            return;
        };
        let Some(next @ Statement::ReturnStatement(_)) = list.get(index + 1) else {
            return;
        };
        if let Some(value) = returned_boolean(consequent) {
            let previous_same = index > 0
                && matches!(&list[index - 1], Statement::IfStatement(previous)
                    if previous.alternate.is_none() && returned_boolean(body(&previous.consequent)) == Some(value));
            if previous_same {
                return;
            }
        }
        let range = Span::new(statement.span.start, next.span().end);
        findings.extend(self.problem(source, statement, consequent, next, range));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferBooleanReturn;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferBooleanReturn, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn returns_the_condition() {
        // The documentation's three cases.
        assert_eq!(
            check(
                "function isValid(value) {\n\tif (value > 0) {\n\t\treturn true;\n\t}\n\treturn false;\n}"
            ),
            (
                vec![(2, 2)],
                "function isValid(value) {\n\treturn value > 0;\n}".to_owned()
            )
        );
        assert_eq!(
            check("function hasItems(items) {\n\tif (items.length) {\n\t\treturn true;\n\t}\n\treturn false;\n}").1,
            "function hasItems(items) {\n\treturn Boolean(items.length);\n}"
        );
        assert_eq!(
            check("function isInvalid(value) {\n\tif (value > 0) {\n\t\treturn false;\n\t}\n\treturn true;\n}").1,
            "function isInvalid(value) {\n\treturn !(value > 0);\n}"
        );
        // With `else`; a negation negated; `Array.isArray` kept as is.
        assert_eq!(
            check("function f(a) { if (!a) return false; else { return true; } }").1,
            "function f(a) { return !!a; }"
        );
        assert_eq!(
            check("function f(a) { if (Array.isArray(a)) { return true; } return false; }").1,
            "function f(a) { return Array.isArray(a); }"
        );
    }

    #[test]
    fn leaves_guard_runs_same_values_and_conditional_tests() {
        for code in [
            "function f(a, b) { if (a) { return true; } if (b) { return true; } return false; }",
            "function f(a) { if (a) { return true; } return true; }",
            "function f(a) { if (a ? b : c) { return true; } return false; }",
            "function f(a) { if (a) { return 1; } return 0; }",
            "function f(a) { if (a) { return true; } g(); }",
            "function f(a) { if (a) { g(); return true; } return false; }",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_without_fix_around_comments_or_a_local_boolean() {
        for code in [
            "function f(a) {\n  if (a) { return true; } // yes\n  return false;\n}",
            "function f(a) {\n  if (a) { return true; }\n  return false; // no\n}",
            "const Boolean = 1;\nfunction f(a) { if (a) { return true; } return false; }",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }
}
