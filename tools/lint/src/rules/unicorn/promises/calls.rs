//! What the rules on promises share: a method's static name, whether a callback is an `async`
//! function, whether a chain's value is discarded with `void`, `new Promise(executor)`, the one
//! expression a function evaluates, and a node's range with its parentheses
//! (`getPropertyName`, `isCallExpressionValueDiscardedWithVoid`, `getFunctionOnlyExpression` and
//! `getParenthesizedRange` of `eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrowFunctionBody, Expression, FunctionBody, NewExpression, Statement, UnaryOperator,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{const_value, is_global};
use crate::rules::unicorn::syntax::{
    Value, is_parenthesized, outer_parent, static_value, symbol_of, unwrap, unwrap_chain,
};
use crate::source::Source;

/// The static name of a called member: a name, or a computed key whose string is known; with
/// where it is written.
#[must_use]
pub fn method_name(source: &Source, callee: &Expression) -> Option<(String, Span)> {
    match callee {
        Expression::StaticMemberExpression(member) => {
            Some((member.property.name.to_string(), member.property.span))
        }
        Expression::ComputedMemberExpression(member) => {
            match static_value(source, &member.expression)? {
                Value::Text(text) => Some((text, member.expression.span())),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Whether an expression is an `async` function that is not a generator.
fn is_async_function(expression: &Expression) -> bool {
    match expression {
        Expression::ArrowFunctionExpression(arrow) => arrow.r#async,
        Expression::FunctionExpression(function) => function.r#async && !function.generator,
        _ => false,
    }
}

/// Whether a callback is an `async` function: written there, a `const` bound to one, or the
/// name of a function declared once, `async` and not a generator; with `body_only`, a
/// declaration counts only when it has a body and the name is never written.
#[must_use]
pub fn is_async_callback(source: &Source, callback: &Expression, body_only: bool) -> bool {
    let callback = if body_only {
        unwrap_chain(callback)
    } else {
        unwrap(callback)
    };
    if is_async_function(callback) {
        return true;
    }
    let Expression::Identifier(name) = callback else {
        return false;
    };
    let Some(symbol) = symbol_of(source, name) else {
        return false;
    };
    if const_value(source, symbol).is_some_and(|init| {
        is_async_function(if body_only {
            unwrap_chain(init)
        } else {
            unwrap(init)
        })
    }) {
        return true;
    }
    let scoping = source.semantic.scoping();
    let written = scoping
        .get_resolved_references(symbol)
        .any(oxc_semantic::Reference::is_write);
    if body_only && written {
        return false;
    }
    let mut declarations = std::iter::once(scoping.symbol_declaration(symbol))
        .chain(
            scoping
                .symbol_redeclarations(symbol)
                .iter()
                .map(|each| each.declaration),
        )
        .collect::<Vec<_>>();
    declarations.dedup();
    let functions: Vec<_> = declarations
        .iter()
        .filter_map(
            |declaration| match source.semantic.nodes().kind(*declaration) {
                AstKind::Function(function) if !body_only || function.body.is_some() => {
                    Some(function)
                }
                _ => None,
            },
        )
        .collect();
    let counted = if body_only {
        functions.len()
    } else {
        declarations.len()
    };
    counted == 1
        && functions
            .first()
            .is_some_and(|function| function.r#async && !function.generator)
}

/// Whether a call's value, through the calls chained on it, `?.` and TypeScript's wrappers, is
/// discarded with `void`.
#[must_use]
pub fn is_discarded_with_void(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let mut current = node.id();
    loop {
        let (parent, child) = outer_parent(source, current);
        let parent_id = nodes.parent_id(child);
        let child_span = nodes.kind(child).span();
        match parent {
            AstKind::StaticMemberExpression(member) if member.object.span() == child_span => {
                if let AstKind::CallExpression(call) = nodes.parent_kind(parent_id)
                    && call.callee.span() == member.span
                {
                    current = nodes.parent_id(parent_id);
                    continue;
                }
                return false;
            }
            AstKind::ComputedMemberExpression(member) if member.object.span() == child_span => {
                if let AstKind::CallExpression(call) = nodes.parent_kind(parent_id)
                    && call.callee.span() == member.span
                {
                    current = nodes.parent_id(parent_id);
                    continue;
                }
                return false;
            }
            AstKind::UnaryExpression(unary) => return unary.operator == UnaryOperator::Void,
            _ => return false,
        }
    }
}

/// The executor of `new Promise(executor)`, the global `Promise` called with one argument.
#[must_use]
pub fn promise_executor<'s, 'a>(
    source: &Source,
    new: &'s NewExpression<'a>,
) -> Option<&'s Expression<'a>> {
    let Expression::Identifier(callee) = &new.callee else {
        return None;
    };
    if callee.name != "Promise" || !is_global(source, callee) || new.arguments.len() != 1 {
        return None;
    }
    new.arguments[0].as_expression()
}

/// A function's body: statements, or an arrow's expression.
#[derive(Clone, Copy)]
pub enum Body<'s, 'a> {
    /// A block of statements.
    Block(&'s FunctionBody<'a>),
    /// An arrow's expression.
    Expression(&'s Expression<'a>),
}

impl<'s, 'a> Body<'s, 'a> {
    /// The body of an arrow.
    #[must_use]
    pub fn of_arrow(body: &'s ArrowFunctionBody<'a>) -> Option<Self> {
        match body {
            ArrowFunctionBody::FunctionBody(block) => Some(Self::Block(block)),
            other => other.as_expression().map(Self::Expression),
        }
    }

    /// The one expression the body evaluates: an arrow's expression, or the expression of its
    /// only statement (`getFunctionOnlyExpression`).
    #[must_use]
    pub fn only_expression(self) -> Option<&'s Expression<'a>> {
        match self {
            Self::Expression(expression) => Some(expression),
            Self::Block(block) => match block.statements.as_slice() {
                [Statement::ExpressionStatement(statement)] => Some(&statement.expression),
                _ => None,
            },
        }
    }
}

/// A range widened over the parentheses written around it.
#[must_use]
pub fn parenthesized_span(source: &Source, span: Span) -> Span {
    let mut span = span;
    while is_parenthesized(source, span) {
        let before = source.text.get(..span.start as usize).unwrap_or_default();
        let after = source.text.get(span.end as usize..).unwrap_or_default();
        let open = before.trim_end().len() - 1;
        let close = span.end as usize + (after.len() - after.trim_start().len()) + 1;
        span = Span::new(
            u32::try_from(open).unwrap_or(span.start),
            u32::try_from(close).unwrap_or(span.end),
        );
    }
    span
}
