//! `unicorn/prefer-url-can-parse`: a `try` that builds a `URL` only to answer whether it parses
//! is `URL.canParse(…)` (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-url-can-parse.md`):
//! `try { new URL(value); return true; } catch { return false; }`, or the same assigning a
//! boolean. Arguments that may throw on their own — side effects, conversions, objects, classes,
//! templates with expressions, `Symbol` — are left. Autofixed unless a comment is inside.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, AssignmentOperator, AssignmentTarget, BinaryOperator, Expression, NewExpression,
    Statement, UnaryOperator,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::urls::is_url_constructor;
use crate::expressions::has_comment_in;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{symbol_of, text};
use crate::source::Source;

/// The rule.
pub struct PreferUrlCanParse;

/// The search for what may throw while an argument is evaluated or converted.
struct Throwing<'s, 'b, 'a> {
    /// The file, to tell a global `Symbol`.
    source: &'s Source<'b, 'a>,
    /// Whether something was found.
    found: bool,
}

/// Whether an expression is a literal, as `ESTree`'s `Literal`.
fn is_literal(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::BigIntLiteral(_)
    )
}

impl<'a> Visit<'a> for Throwing<'_, '_, 'a> {
    fn visit_expression(&mut self, expression: &Expression<'a>) {
        self.found |= match expression {
            Expression::CallExpression(_)
            | Expression::NewExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::UpdateExpression(_)
            | Expression::AwaitExpression(_)
            | Expression::YieldExpression(_)
            | Expression::ImportExpression(_)
            | Expression::TaggedTemplateExpression(_)
            | Expression::ObjectExpression(_)
            | Expression::ClassExpression(_) => true,
            Expression::TemplateLiteral(template) => !template.expressions.is_empty(),
            Expression::UnaryExpression(unary) => {
                unary.operator == UnaryOperator::Delete
                    || (matches!(
                        unary.operator,
                        UnaryOperator::UnaryNegation
                            | UnaryOperator::UnaryPlus
                            | UnaryOperator::LogicalNot
                            | UnaryOperator::BitwiseNot
                    ) && !is_literal(&unary.argument))
            }
            Expression::BinaryExpression(binary) => {
                let strict = matches!(
                    binary.operator,
                    BinaryOperator::StrictEquality
                        | BinaryOperator::StrictInequality
                        | BinaryOperator::Instanceof
                );
                let literals = is_literal(&binary.left) && is_literal(&binary.right);
                !strict && !literals
            }
            Expression::ComputedMemberExpression(member) => !is_literal(&member.expression),
            Expression::StaticMemberExpression(member) => matches!(&member.object,
                Expression::Identifier(object) if object.name == "Symbol" && symbol_of(self.source, object).is_none()),
            _ => false,
        };
        walk::walk_expression(self, expression);
    }
}

/// Whether an argument may throw on its own.
fn may_throw(source: &Source, argument: &Argument) -> bool {
    let mut search = Throwing {
        source,
        found: false,
    };
    search.visit_argument(argument);
    search.found
}

/// The `new URL(…)` a statement is, of one or two arguments.
fn new_url<'b, 'a>(source: &Source, statement: &'b Statement<'a>) -> Option<&'b NewExpression<'a>> {
    let Statement::ExpressionStatement(statement) = statement else {
        return None;
    };
    let Expression::NewExpression(new) = &statement.expression else {
        return None;
    };
    let fits = (1..=2).contains(&new.arguments.len())
        && !new.arguments.iter().any(Argument::is_spread)
        && is_url_constructor(source, &new.callee)
        && !new
            .arguments
            .iter()
            .any(|argument| may_throw(source, argument));
    fits.then_some(new)
}

/// The boolean a statement returns.
fn returned(statement: &Statement) -> Option<bool> {
    match statement {
        Statement::ReturnStatement(statement) => match &statement.argument {
            Some(Expression::BooleanLiteral(value)) => Some(value.value),
            _ => None,
        },
        _ => None,
    }
}

/// The name and the boolean a statement assigns, `name = true`.
fn assigned<'a>(
    source: &Source,
    statement: &Statement<'a>,
) -> Option<(&'a str, bool, Option<oxc_semantic::SymbolId>)> {
    let Statement::ExpressionStatement(statement) = statement else {
        return None;
    };
    let Expression::AssignmentExpression(assignment) = &statement.expression else {
        return None;
    };
    let (AssignmentTarget::AssignmentTargetIdentifier(name), Expression::BooleanLiteral(value)) =
        (&assignment.left, &assignment.right)
    else {
        return None;
    };
    if assignment.operator != AssignmentOperator::Assign {
        return None;
    }
    Some((name.name.as_str(), value.value, symbol_of(source, name)))
}

impl Rule for PreferUrlCanParse {
    fn name(&self) -> &'static str {
        "unicorn/prefer-url-can-parse"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::TryStatement(statement) = node.kind() else {
            return;
        };
        let Some(handler) = &statement.handler else {
            return;
        };
        let plain_parameter = handler.param.as_ref().is_none_or(|parameter| {
            matches!(
                parameter.pattern,
                oxc_ast::ast::BindingPattern::BindingIdentifier(_)
            )
        });
        let ([first, second], [caught]) = (
            statement.block.body.as_slice(),
            handler.body.body.as_slice(),
        ) else {
            return;
        };
        if !plain_parameter || statement.finalizer.is_some() {
            return;
        }
        let Some(new) = new_url(source, first) else {
            return;
        };
        let arguments: Vec<&str> = new
            .arguments
            .iter()
            .map(|argument| text(source, argument.span()))
            .collect();
        let call = |negate: bool| {
            format!(
                "{}{}.canParse({})",
                if negate { "!" } else { "" },
                text(source, new.callee.span()),
                arguments.join(", ")
            )
        };
        let replacement = match (returned(second), returned(caught)) {
            (Some(tried), Some(failed)) if tried != failed => format!("return {};", call(!tried)),
            _ => match (assigned(source, second), assigned(source, caught)) {
                (Some((name, tried, symbol)), Some((other, failed, same)))
                    if name == other && symbol == same && tried != failed =>
                {
                    format!("{name} = {};", call(!tried))
                }
                _ => return,
            },
        };
        let message =
            "Prefer `URL.canParse()` over constructing a `URL` in a try/catch for validation."
                .to_owned();
        if has_comment_in(source, statement.span) {
            findings.push(source.finding(self.name(), statement.span.start, message));
            return;
        }
        let fix = Fix {
            start: statement.span.start,
            end: statement.span.end,
            text: replacement,
        };
        findings.push(source.fixed(self.name(), statement.span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferUrlCanParse;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferUrlCanParse, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn asks_url_can_parse() {
        // The documentation's two cases.
        assert_eq!(
            check("function f(value) {\n\ttry {\n\t\tnew URL(value);\n\t\treturn true;\n\t} catch {\n\t\treturn false;\n\t}\n}\nlet valid;\ntry {\n\tnew URL(value, base);\n\tvalid = true;\n} catch {\n\tvalid = false;\n}"),
            (
                vec![(2, 2), (10, 1)],
                "function f(value) {\n\treturn URL.canParse(value);\n}\nlet valid;\nvalid = URL.canParse(value, base);"
                    .to_owned()
            )
        );
        // Negated; a member argument; a comment inside: reported only.
        assert_eq!(
            check("function g(o) {\n  try { new URL(o.href); return false; } catch (error) { return true; }\n}\nfunction h(v) {\n  try { new URL(v); /* ok */ return true; } catch { return false; }\n}"),
            (
                vec![(2, 3), (5, 3)],
                "function g(o) {\n  return !URL.canParse(o.href);\n}\nfunction h(v) {\n  try { new URL(v); /* ok */ return true; } catch { return false; }\n}"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_other_try_statements() {
        for code in [
            "function f(v) { return URL.canParse(v); }",
            "function f(v) { try { new URL(v); return true; } catch { return true; } }",
            "function f(v) { try { new URL(v); return true; } catch { return false; } finally { x(); } }",
            "function f(v) { try { new URL(`${v}`); return true; } catch { return false; } }",
            "function f(v) { try { new URL(g(v)); return true; } catch { return false; } }",
            "function f(v) { try { new URL(v + 1); return true; } catch { return false; } }",
            "function f(v) { try { new URL(v[k]); return true; } catch { return false; } }",
            "function f(v) { try { new URL({}); return true; } catch { return false; } }",
            "function f(v) { try { new URL(Symbol.iterator); return true; } catch { return false; } }",
            "function f(v) { try { new Other(v); return true; } catch { return false; } }",
            "function f(v) { try { new URL(v); x(); } catch { return false; } }",
            "function f(v) { try { new URL(v); return true; } catch ({ message }) { return false; } }",
            "let a, b; try { new URL(v); a = true; } catch { b = false; }",
            "let a; try { new URL(v); a = true; } catch (a) { a = false; }",
            "const URL = class {}; function f(v) { try { new URL(v); return true; } catch { return false; } }",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
