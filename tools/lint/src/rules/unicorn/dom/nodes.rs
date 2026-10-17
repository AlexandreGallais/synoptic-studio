//! What Unicorn's DOM rules share, read from the syntax alone: values that cannot be a DOM node,
//! static strings, and calls of methods named statically (`eslint-plugin-unicorn` 78.0.0,
//! `utils/is-node-value-not-dom-node.js`, `ast/literal.js`, `ast/is-method-call.js`).

use oxc_ast::ast::{Argument, CallExpression, Expression};

use crate::rules::unicorn::syntax::{MethodCall, const_init, method_call, symbol_of};
use crate::source::Source;

/// Whether an expression cannot be a DOM node: an array, a function, a class, a literal, an
/// object, a template, or `undefined` (`isNodeValueNotDomNode`).
#[must_use]
pub fn is_not_dom_node(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::ArrayExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::ClassExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::ObjectExpression(_)
            | Expression::TemplateLiteral(_)
    ) || matches!(expression, Expression::Identifier(name) if name.name == "undefined")
}

/// The string an expression is written as: a string literal, or a template without
/// expressions (`getStaticStringValue`).
#[must_use]
pub fn static_string(expression: &Expression) -> Option<String> {
    match expression {
        Expression::StringLiteral(literal) => Some(literal.value.to_string()),
        Expression::TemplateLiteral(template) if template.expressions.is_empty() => template
            .quasis
            .first()
            .and_then(|quasi| quasi.value.cooked)
            .map(|cooked| cooked.to_string()),
        _ => None,
    }
}

/// A plain call of a method among some names, with a number of arguments none spread: neither
/// its member nor its call optional (`isMethodCall` with `optionalCall` and `optionalMember`
/// false).
#[must_use]
pub fn plain_method_call<'b, 'a>(
    call: &'b CallExpression<'a>,
    names: &[&str],
    count: usize,
) -> Option<MethodCall<'b, 'a>> {
    method_call(call).filter(|method| {
        method.is_plain()
            && names.contains(&method.name)
            && call.arguments.len() == count
            && !call.arguments.iter().any(Argument::is_spread)
    })
}

/// Whether an expression is the global name `document`, or `window.document` or
/// `globalThis.document`.
#[must_use]
pub fn is_document(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(name) => name.name == "document",
        Expression::StaticMemberExpression(member) => {
            member.property.name == "document"
                && matches!(&member.object, Expression::Identifier(object)
                    if matches!(object.name.as_str(), "window" | "globalThis")
                        && symbol_of(source, object).is_none())
        }
        _ => false,
    }
}

/// The name of a member written statically: `a.name`, `a['name']` or ``a[`name`]``, or a
/// number (`getStaticPropertyName`).
#[must_use]
pub fn static_property_name(expression: &Expression) -> Option<String> {
    match expression {
        Expression::StaticMemberExpression(member) => Some(member.property.name.to_string()),
        Expression::ComputedMemberExpression(member) => match &member.expression {
            Expression::NumericLiteral(number) => Some(number.value.to_string()),
            key => static_string(key),
        },
        _ => None,
    }
}

/// The object of a member, `a` in `a.b` or `a[b]`.
#[must_use]
pub fn member_object<'b, 'a>(expression: &'b Expression<'a>) -> Option<&'b Expression<'a>> {
    match expression {
        Expression::StaticMemberExpression(member) => Some(&member.object),
        Expression::ComputedMemberExpression(member) => Some(&member.object),
        _ => None,
    }
}

/// Where the name of a member is written: after the dot, or the key between brackets.
#[must_use]
pub fn property_start(expression: &Expression) -> Option<u32> {
    match expression {
        Expression::StaticMemberExpression(member) => Some(member.property.span.start),
        Expression::ComputedMemberExpression(member) => {
            Some(oxc_span::GetSpan::span(&member.expression).start)
        }
        _ => None,
    }
}

/// Whether an expression reads a global by a path, `document.write`, from the global name or
/// through `window`, `globalThis`, `self` or `global`, or a `const` bound to it, the global not
/// shadowed (`ReferenceTracker.iterateGlobalReferences`).
#[must_use]
pub fn is_global_path(source: &Source, expression: &Expression, path: &[&str]) -> bool {
    let Some((last, rest)) = path.split_last() else {
        return false;
    };
    if let Expression::Identifier(name) = expression
        && symbol_of(source, name).is_some()
    {
        return const_init(source, expression)
            .is_some_and(|init| is_global_path(source, init, path));
    }
    if rest.is_empty() {
        return match expression {
            Expression::Identifier(name) => name.name == *last && symbol_of(source, name).is_none(),
            _ => {
                static_property_name(expression).as_deref() == Some(last)
                    && member_object(expression).is_some_and(|object| {
                        ["window", "globalThis", "self", "global"]
                            .iter()
                            .any(|global| is_global_path(source, object, &[global]))
                    })
            }
        };
    }
    static_property_name(expression).as_deref() == Some(last)
        && member_object(expression).is_some_and(|object| is_global_path(source, object, rest))
}

/// A plain call of a method among some names, with at least a number of arguments, spread or
/// not (`isMethodCall` with `minimumArguments`).
#[must_use]
pub fn plain_method_call_at_least<'b, 'a>(
    call: &'b CallExpression<'a>,
    names: &[&str],
    least: usize,
) -> Option<MethodCall<'b, 'a>> {
    method_call(call).filter(|method| {
        method.is_plain() && names.contains(&method.name) && call.arguments.len() >= least
    })
}
