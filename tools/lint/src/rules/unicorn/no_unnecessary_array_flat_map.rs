//! `unicorn/no-unnecessary-array-flat-map`: no `flatMap()` whose callback wraps one item in an
//! array, which `map()` says, nor a wrapper around a value that is no array
//! (`eslint-plugin-unicorn` 78.0.0), read as `ESLint` reads a TypeScript file: the conditional
//! callbacks are left to the wrapper check, which only suggests its fix. Autofixed after
//! `filter()`, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrayExpression, ArrayExpressionElement, ArrowFunctionExpression, BindingPattern,
    CallExpression, Expression, TSType,
};
use oxc_semantic::AstNode;

use super::syntax::{
    Binding, MethodCall, binding_of, concise_body_text, has_comment, is_known_non_indexed,
    method_call, method_named, static_value,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUnnecessaryArrayFlatMap;

/// The message on a single-item callback.
const MAP: &str = "Prefer `.map(…)` over `.flatMap(…)` for this single-item array callback.";
/// The message on an array wrapper.
const WRAPPER: &str = "Wrapping this value in an array is unnecessary.";

/// The one element of an array literal, neither a hole nor spread.
fn single_element<'b, 'a>(array: &'b ArrayExpression<'a>) -> Option<&'b Expression<'a>> {
    match array.elements.as_slice() {
        [ArrayExpressionElement::SpreadElement(_) | ArrayExpressionElement::Elision(_)] => None,
        [element] => element.as_expression(),
        _ => None,
    }
}

/// A plain `flatMap(callback)` call on a receiver that may be an array, after a `filter()` on
/// one that may be an array too (`isFlatMapCall`).
fn flat_map_call<'b, 'a>(
    source: &Source,
    call: &'b CallExpression<'a>,
) -> Option<MethodCall<'b, 'a>> {
    let method = method_call(call).filter(|method| {
        method.name == "flatMap"
            && method.is_plain()
            && matches!(method.arguments().as_deref(), Some([_]))
    })?;
    if is_known_non_indexed(source, method.object) {
        return None;
    }
    let filter_on_non_array = filter_call(method.object)
        .is_some_and(|filter| is_known_non_indexed(source, filter.object));
    (!filter_on_non_array).then_some(method)
}

/// A plain `filter(callback)` call.
fn filter_call<'b, 'a>(expression: &'b Expression<'a>) -> Option<MethodCall<'b, 'a>> {
    method_named(expression, &["filter"])
        .filter(|method| method.is_plain() && matches!(method.arguments().as_deref(), Some([_])))
}

/// The arrow of one plain parameter and a concise body.
fn simple_arrow<'b, 'a>(expression: &'b Expression<'a>) -> Option<&'b ArrowFunctionExpression<'a>> {
    let Expression::ArrowFunctionExpression(arrow) = expression else {
        return None;
    };
    let [parameter] = arrow.params.items.as_slice() else {
        return None;
    };
    let plain = !arrow.r#async
        && arrow.return_type.is_none()
        && arrow.type_parameters.is_none()
        && arrow.params.rest.is_none()
        && matches!(parameter.pattern, BindingPattern::BindingIdentifier(_))
        && !parameter.optional
        && parameter.initializer.is_none()
        && arrow.get_expression().is_some();
    plain.then_some(arrow)
}

/// The finding on a callback that returns `[item]`, fixed after `filter()`.
fn single_item_finding(
    source: &Source,
    rule: &'static str,
    flat_map: &MethodCall,
) -> Option<Finding> {
    let arrow = simple_arrow(flat_map.arguments()?.first()?)?;
    let Expression::ArrayExpression(array) = arrow.get_expression()? else {
        return None;
    };
    let element = single_element(array)?;
    let fixable = filter_call(flat_map.object).is_some_and(|filter| {
        filter.call.type_arguments.is_none()
            && flat_map.call.type_arguments.is_none()
            && !is_known_non_indexed(source, filter.object)
            && !has_comment(source, array.span)
    });
    if !fixable {
        return Some(source.finding(rule, flat_map.property.start, MAP.to_owned()));
    }
    let body = concise_body_text(source, element);
    let text = format!(
        "map{}{body}{}",
        &source.text[flat_map.property.end as usize..array.span.start as usize],
        &source.text[array.span.end as usize..flat_map.call.span.end as usize],
    );
    Some(source.fixed(
        rule,
        flat_map.property.start,
        MAP.to_owned(),
        Fix {
            start: flat_map.property.start,
            end: flat_map.call.span.end,
            text,
        },
    ))
}

/// Whether an annotation names a primitive: a keyword, a literal, or a union of them.
fn is_primitive_type(annotation: &TSType) -> bool {
    match annotation {
        TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSNullKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSStringKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSUndefinedKeyword(_)
        | TSType::TSLiteralType(_) => true,
        TSType::TSUnionType(union) => union.types.iter().all(is_primitive_type),
        _ => false,
    }
}

/// Whether a value is known not to be an array: a literal, an operation, a function, a value
/// known statically, or a binding annotated with a primitive type.
fn is_non_array(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::ObjectExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::ClassExpression(_)
        | Expression::TemplateLiteral(_)
        | Expression::UnaryExpression(_)
        | Expression::BinaryExpression(_)
        | Expression::UpdateExpression(_) => true,
        Expression::Identifier(_) => {
            static_value(source, expression).is_some()
                || match binding_of(source, expression) {
                    Some(Binding::Variable(declarator, _)) => declarator
                        .type_annotation
                        .as_ref()
                        .is_some_and(|annotation| is_primitive_type(&annotation.type_annotation)),
                    Some(Binding::Parameter(parameter)) => parameter
                        .type_annotation
                        .as_ref()
                        .is_some_and(|annotation| is_primitive_type(&annotation.type_annotation)),
                    None => false,
                }
        }
        _ => false,
    }
}

/// The finding on an array literal that wraps a non-array value returned to `flatMap()`.
fn wrapper_finding(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    array: &ArrayExpression,
) -> Option<Finding> {
    let element = single_element(array)?;
    let nodes = source.semantic.nodes();
    let mut returned = node.id();
    loop {
        match nodes.parent_kind(returned) {
            AstKind::ConditionalExpression(conditional)
                if conditional.test.node_id() != returned =>
            {
                returned = nodes.parent_id(returned);
            }
            _ => break,
        }
    }
    let mut callback = nodes.parent_node(returned);
    if matches!(callback.kind(), AstKind::ReturnStatement(_)) {
        callback = nodes.ancestors(callback.id()).find(|each| {
            matches!(
                each.kind(),
                AstKind::Function(_) | AstKind::ArrowFunctionExpression(_)
            )
        })?;
    }
    let plain = match callback.kind() {
        AstKind::ArrowFunctionExpression(arrow) => !arrow.r#async && arrow.return_type.is_none(),
        AstKind::Function(function) => {
            !function.r#async && !function.generator && function.return_type.is_none()
        }
        _ => false,
    };
    let AstKind::CallExpression(call) = nodes.parent_kind(callback.id()) else {
        return None;
    };
    let first = call
        .arguments
        .first()
        .and_then(|argument| argument.as_expression())
        .is_some_and(|argument| argument.node_id() == callback.id());
    let flat_map = flat_map_call(source, call)?;
    if !plain
        || !first
        || single_item_finding(source, rule, &flat_map).is_some()
        || !is_non_array(source, element)
    {
        return None;
    }
    Some(source.finding(rule, array.span.start, WRAPPER.to_owned()))
}

impl Rule for NoUnnecessaryArrayFlatMap {
    fn name(&self) -> &'static str {
        "unicorn/no-unnecessary-array-flat-map"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::CallExpression(call) => {
                if let Some(flat_map) = flat_map_call(source, call) {
                    findings.extend(single_item_finding(source, self.name(), &flat_map));
                }
            }
            AstKind::ArrayExpression(array) => {
                findings.extend(wrapper_finding(source, self.name(), node, array));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnnecessaryArrayFlatMap;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnnecessaryArrayFlatMap, "src/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&NoUnnecessaryArrayFlatMap, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_single_item_callbacks() {
        let map = ["Prefer `.map(…)` over `.flatMap(…)` for this single-item array callback."];

        assert_eq!(
            check("const ids = array.flatMap(value => [value.id]);"),
            map
        );
        assert_eq!(
            fix("const ids = array.flatMap(value => [value.id]);"),
            "const ids = array.flatMap(value => [value.id]);"
        );
        assert_eq!(
            check("const ids = array.filter(value => value.active).flatMap(value => [value.id]);"),
            map
        );
        assert_eq!(
            fix("const ids = array.filter(value => value.active).flatMap(value => [value.id]);"),
            "const ids = array.filter(value => value.active).map(value => value.id);"
        );
        assert_eq!(
            fix("const o = array.filter(Boolean).flatMap((value) => [{ value }]);"),
            "const o = array.filter(Boolean).map((value) => ({ value }));"
        );
    }

    #[test]
    fn reports_the_documented_wrappers_of_non_arrays() {
        let wrapper = ["Wrapping this value in an array is unnecessary."];

        assert_eq!(
            check("const values = array.flatMap(() => Math.random() ? [1] : [2, 3]);"),
            wrapper
        );
        assert_eq!(
            check("declare const value: string;\narray.flatMap(() => condition ? [value] : []);"),
            wrapper
        );
        assert_eq!(
            check("array.flatMap(function (x) { return x ? [x + 1] : []; });"),
            wrapper
        );
        assert_eq!(
            check("const n = 2; array.flatMap(x => x ? [n] : [x]);"),
            wrapper
        );
        // The conditional form is TypeScript's: no `filter()` rewrite, the wrapper alone.
        assert_eq!(
            check("const active = array.flatMap(value => value.active ? [value] : []);"),
            NONE
        );
        assert_eq!(
            check("const ids = array.flatMap(value => value.active ? [value.id] : []);"),
            NONE
        );
    }

    #[test]
    fn accepts_real_flattening_and_other_callbacks() {
        for code in [
            "const ids = array.map(value => value.id);",
            "const descendants = array.flatMap(value => value.children);",
            "const values = array.flatMap(value => [value, value * 2]);",
            "const values = array.flatMap(() => Math.random() ? 1 : [2, 3]);",
            "const values = array.flatMap(value => [...value]);",
            "const values = array.flatMap(async value => [value]);",
            "const values = array.flatMap((value): number[] => [value]);",
            "const values = array.flatMap((a, b) => [a]);",
            "const values = array.flatMap(value => { return [value]; });",
            "const values = array?.flatMap(value => [value]);",
            "const values = array.flatMap(value => [value], thisArg);",
            "const values = new Set().flatMap(value => [value]);",
            "const values = map.values().flatMap(value => [value]);",
            "const values = array.flatMap(() => condition ? [unknownValue] : []);",
            "const values = list.forEach(() => [1]);",
            "const values = array.flatMap(f, [1]);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
