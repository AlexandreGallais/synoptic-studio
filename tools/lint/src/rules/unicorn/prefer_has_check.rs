//! `unicorn/prefer-has-check`: `.has(…)` rather than `.get(…)` compared with what a missing key
//! gives, or read for its truth, when the stored values cannot be mistaken for it
//! (`eslint-plugin-unicorn` 78.0.0), from annotations, constructors and `const` bindings without
//! type information. Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrayExpressionElement, BinaryOperator, Expression, TSLiteral, TSType, TSTypeName,
    TSTypeOperatorOperator, TSTypeParameterInstantiation, UnaryOperator,
};
use oxc_semantic::{AstNode, ScopeId};

use super::syntax::{
    Binding, TypeDefinition, Value, binding_of, has_comment, is_boolean_context, is_global,
    method_call, object_text, parenthesized_text, static_value, symbol_of, type_definition, unwrap,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferHasCheck;

/// Its message.
const MESSAGE: &str = "Prefer `.has(…)` when checking existence.";

/// What a stored value must never be for `get()` to tell absence.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Safety {
    /// Never falsy.
    Truthy,
    /// Never `undefined`.
    NotUndefined,
    /// Never `null` nor `undefined`.
    NotNullish,
}

/// What a receiver is: a map, with its value type when written, or a collection whose `get()`
/// gives `null` when the name is missing.
#[derive(Clone, Copy)]
enum Collection<'a> {
    /// A `Map`, `ReadonlyMap` or `WeakMap`.
    Map(Option<&'a TSType<'a>>),
    /// `FormData`, `Headers` or `URLSearchParams`.
    NullSentinel,
}

/// The last type argument written.
fn last_argument<'a>(
    arguments: Option<&'a TSTypeParameterInstantiation<'a>>,
) -> Option<&'a TSType<'a>> {
    arguments.and_then(|each| each.params.last())
}

/// The collection a type annotation names, through `readonly` and the file's type aliases.
fn collection_of_type<'a>(
    source: &Source<'_, 'a>,
    scope: ScopeId,
    annotation: &'a TSType<'a>,
    depth: u8,
) -> Option<Collection<'a>> {
    match annotation {
        TSType::TSTypeOperatorType(operator)
            if operator.operator == TSTypeOperatorOperator::Readonly =>
        {
            collection_of_type(source, scope, &operator.type_annotation, depth)
        }
        TSType::TSTypeReference(reference) => {
            let TSTypeName::IdentifierReference(name) = &reference.type_name else {
                return None;
            };
            let name = name.name.as_str();
            match type_definition(source, scope, name) {
                Some(TypeDefinition::Alias(aliased)) if depth < 4 => {
                    collection_of_type(source, scope, aliased, depth + 1)
                }
                Some(_) => None,
                None => match name {
                    "Map" | "ReadonlyMap" | "WeakMap" => Some(Collection::Map(last_argument(
                        reference.type_arguments.as_deref(),
                    ))),
                    "FormData" | "Headers" | "URLSearchParams" => Some(Collection::NullSentinel),
                    _ => None,
                },
            }
        }
        _ => None,
    }
}

/// The type annotation of an expression's `as`, `satisfies` or `<T>` wrapper.
fn wrapper_annotation<'a>(expression: &'a Expression<'a>) -> Option<&'a TSType<'a>> {
    match expression {
        Expression::TSAsExpression(wrapper) => Some(&wrapper.type_annotation),
        Expression::TSSatisfiesExpression(wrapper) => Some(&wrapper.type_annotation),
        Expression::TSTypeAssertion(wrapper) => Some(&wrapper.type_annotation),
        _ => None,
    }
}

/// The annotation of the binding an identifier reads.
fn binding_annotation<'a>(
    source: &Source<'_, 'a>,
    expression: &Expression,
) -> Option<&'a TSType<'a>> {
    match binding_of(source, expression)? {
        Binding::Variable(declarator, _) => declarator
            .type_annotation
            .as_ref()
            .map(|each| &each.type_annotation),
        Binding::Parameter(parameter) => parameter
            .type_annotation
            .as_ref()
            .map(|each| &each.type_annotation),
    }
}

/// The collection a receiver is, from its syntax.
fn collection_of<'a>(
    source: &Source<'_, 'a>,
    scope: ScopeId,
    expression: &'a Expression<'a>,
    depth: u8,
) -> Option<Collection<'a>> {
    if let Some(found) =
        wrapper_annotation(expression).and_then(|each| collection_of_type(source, scope, each, 0))
    {
        return Some(found);
    }
    let expression = unwrap(expression);
    if let Expression::NewExpression(new) = expression {
        let Expression::Identifier(callee) = &new.callee else {
            return None;
        };
        if symbol_of(source, callee).is_some() {
            return None;
        }
        return match callee.name.as_str() {
            "Map" | "WeakMap" => Some(Collection::Map(None)),
            "FormData" | "Headers" | "URLSearchParams" => Some(Collection::NullSentinel),
            _ => None,
        };
    }
    if !matches!(expression, Expression::Identifier(_)) || depth > 3 {
        return None;
    }
    if let Some(annotation) = binding_annotation(source, expression) {
        return collection_of_type(source, scope, annotation, 0);
    }
    let init = super::syntax::const_init(source, expression)?;
    collection_of(source, scope, init, depth + 1)
}

/// Whether a value never is what the safety rules out.
fn is_safe_value(value: &Value, safety: Safety) -> bool {
    match safety {
        Safety::Truthy => match value {
            Value::Text(text) => !text.is_empty(),
            Value::Number(number) => *number != 0.0 && !number.is_nan(),
            Value::Boolean(boolean) => *boolean,
            Value::BigInt(digits) => digits.trim_start_matches('-') != "0",
            Value::Null | Value::Undefined => false,
        },
        Safety::NotUndefined => *value != Value::Undefined,
        Safety::NotNullish => !matches!(value, Value::Undefined | Value::Null),
    }
}

/// Whether a value type never holds what the safety rules out.
fn is_safe_type(
    source: &Source,
    scope: ScopeId,
    annotation: &TSType,
    safety: Safety,
    depth: u8,
) -> bool {
    match annotation {
        TSType::TSTypeOperatorType(operator) => {
            operator.operator == TSTypeOperatorOperator::Readonly
                && is_safe_type(source, scope, &operator.type_annotation, safety, depth)
        }
        TSType::TSUnionType(union) => union
            .types
            .iter()
            .all(|each| is_safe_type(source, scope, each, safety, depth)),
        TSType::TSIntersectionType(intersection) => intersection
            .types
            .iter()
            .all(|each| is_safe_type(source, scope, each, safety, depth)),
        TSType::TSNullKeyword(_) => safety == Safety::NotUndefined,
        TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSStringKeyword(_)
        | TSType::TSTypeLiteral(_) => safety != Safety::Truthy,
        TSType::TSArrayType(_)
        | TSType::TSConstructorType(_)
        | TSType::TSFunctionType(_)
        | TSType::TSObjectKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSTupleType(_) => true,
        TSType::TSLiteralType(literal) => {
            let value = match &literal.literal {
                TSLiteral::StringLiteral(text) => Value::Text(text.value.to_string()),
                TSLiteral::NumericLiteral(number) => Value::Number(number.value),
                TSLiteral::BooleanLiteral(boolean) => Value::Boolean(boolean.value),
                TSLiteral::BigIntLiteral(digits) => Value::BigInt(digits.value.to_string()),
                TSLiteral::UnaryExpression(unary) => match &unary.argument {
                    Expression::NumericLiteral(number) => Value::Number(-number.value),
                    _ => return safety != Safety::Truthy,
                },
                TSLiteral::TemplateLiteral(template) => Value::Text(
                    template
                        .quasis
                        .first()
                        .and_then(|quasi| quasi.value.cooked)
                        .map(|cooked| cooked.to_string())
                        .unwrap_or_default(),
                ),
            };
            is_safe_value(&value, safety)
        }
        TSType::TSTypeReference(reference) if depth < 4 => {
            let TSTypeName::IdentifierReference(name) = &reference.type_name else {
                return false;
            };
            match type_definition(source, scope, name.name.as_str()) {
                Some(TypeDefinition::Alias(aliased)) => {
                    is_safe_type(source, scope, aliased, safety, depth + 1)
                }
                Some(TypeDefinition::Object) => safety != Safety::Truthy,
                Some(TypeDefinition::Other) | None => false,
            }
        }
        _ => false,
    }
}

/// Whether a value written in a map's entries never is what the safety rules out.
fn is_safe_expression(source: &Source, expression: &Expression, safety: Safety) -> bool {
    if let Some(value) = static_value(source, expression) {
        return is_safe_value(&value, safety);
    }
    matches!(
        expression,
        Expression::ArrayExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::ClassExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::NewExpression(_)
            | Expression::ObjectExpression(_)
    )
}

/// Whether the values a map receiver holds never are what the safety rules out.
fn has_safe_values(
    source: &Source,
    scope: ScopeId,
    receiver: &Expression,
    safety: Safety,
    depth: u8,
) -> bool {
    if depth == 0
        && let Expression::NewExpression(new) = unwrap(receiver)
        && let Expression::Identifier(callee) = &new.callee
        && matches!(callee.name.as_str(), "Map" | "WeakMap")
        && symbol_of(source, callee).is_none()
    {
        match new.arguments.as_slice() {
            [] => return true,
            [entries] => {
                if let Some(Expression::ArrayExpression(entries)) = entries.as_expression() {
                    return entries.elements.iter().all(|element| match element {
                        ArrayExpressionElement::ArrayExpression(pair) => match pair.elements.get(1)
                        {
                            Some(value) if pair.elements.len() >= 2 => value
                                .as_expression()
                                .is_some_and(|value| is_safe_expression(source, value, safety)),
                            _ => false,
                        },
                        _ => false,
                    });
                }
            }
            _ => {}
        }
    }
    if let Some(annotation) = wrapper_annotation(receiver)
        && let Some(Collection::Map(value)) = collection_of_type(source, scope, annotation, 0)
    {
        return value.is_some_and(|value| is_safe_type(source, scope, value, safety, 0));
    }
    let receiver = unwrap(receiver);
    if let Expression::NewExpression(new) = receiver {
        let map = matches!(&new.callee, Expression::Identifier(callee)
            if matches!(callee.name.as_str(), "Map" | "WeakMap") && symbol_of(source, callee).is_none());
        if map {
            return last_argument(new.type_arguments.as_deref())
                .is_some_and(|value| is_safe_type(source, scope, value, safety, 0));
        }
    }
    if !matches!(receiver, Expression::Identifier(_)) || depth > 3 {
        return false;
    }
    if let Some(annotation) = binding_annotation(source, receiver)
        && let Some(Collection::Map(value)) = collection_of_type(source, scope, annotation, 0)
    {
        return value.is_some_and(|value| is_safe_type(source, scope, value, safety, 0));
    }
    super::syntax::const_init(source, receiver)
        .is_some_and(|init| has_safe_values(source, scope, init, safety, depth + 1))
}

/// Whether an expression is `undefined` (the global) or `void 0`.
fn is_undefined(source: &Source, expression: &Expression) -> bool {
    is_global(source, expression, "undefined")
        || matches!(expression, Expression::UnaryExpression(unary)
            if unary.operator == UnaryOperator::Void
                && matches!(&unary.argument, Expression::NumericLiteral(zero) if zero.value == 0.0))
}

/// Whether a comparison of `get()` with a value checks absence: `null` for the null-sentinel
/// collections, `undefined` (or loosely `null`) for a map whose values are never that.
fn compares_absence(
    source: &Source,
    scope: ScopeId,
    collection: Option<Collection>,
    receiver: &Expression,
    value: &Expression,
    loose: bool,
) -> bool {
    match collection {
        Some(Collection::NullSentinel) => {
            matches!(value, Expression::NullLiteral(_)) || (loose && is_undefined(source, value))
        }
        Some(Collection::Map(_)) => {
            let safety = if loose {
                Safety::NotNullish
            } else {
                Safety::NotUndefined
            };
            (is_undefined(source, value) && has_safe_values(source, scope, receiver, safety, 0))
                || (matches!(value, Expression::NullLiteral(_))
                    && loose
                    && has_safe_values(source, scope, receiver, Safety::NotNullish, 0))
        }
        None => false,
    }
}

impl Rule for PreferHasCheck {
    fn name(&self) -> &'static str {
        "unicorn/prefer-has-check"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(get) =
            method_call(call).filter(|method| method.name == "get" && method.is_plain())
        else {
            return;
        };
        let Some(&[key]) = get.arguments().as_deref() else {
            return;
        };
        let scope = node.scope_id();
        let collection = collection_of(source, scope, get.object, 0);
        let nodes = source.semantic.nodes();
        let mut target = node.id();
        while matches!(
            nodes.parent_kind(target),
            AstKind::TSAsExpression(_)
                | AstKind::TSSatisfiesExpression(_)
                | AstKind::TSNonNullExpression(_)
                | AstKind::TSTypeAssertion(_)
                | AstKind::TSInstantiationExpression(_)
        ) {
            target = nodes.parent_id(target);
        }
        if let AstKind::BinaryExpression(comparison) = nodes.parent_kind(target) {
            let loose = matches!(
                comparison.operator,
                BinaryOperator::Equality | BinaryOperator::Inequality
            );
            let strict = matches!(
                comparison.operator,
                BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
            );
            if !loose && !strict {
                return;
            }
            let value = if comparison.left.node_id() == target {
                &comparison.right
            } else {
                &comparison.left
            };
            let reported = compares_absence(source, scope, collection, get.object, value, loose);
            if !reported {
                return;
            }
            if has_comment(source, comparison.span) {
                findings.push(source.finding(self.name(), get.property.start, MESSAGE.to_owned()));
                return;
            }
            let positive = matches!(
                comparison.operator,
                BinaryOperator::StrictInequality | BinaryOperator::Inequality
            );
            let replacement = format!(
                "{}{}.has({})",
                if positive { "" } else { "!" },
                object_text(source, get.object),
                parenthesized_text(source, key)
            );
            findings.push(source.fixed(
                self.name(),
                get.property.start,
                MESSAGE.to_owned(),
                Fix {
                    start: comparison.span.start,
                    end: comparison.span.end,
                    text: replacement,
                },
            ));
            return;
        }
        let truthy_map = matches!(collection, Some(Collection::Map(_)))
            && has_safe_values(source, scope, get.object, Safety::Truthy, 0);
        if truthy_map && is_boolean_context(source, node.id()) {
            findings.push(source.fixed(
                self.name(),
                get.property.start,
                MESSAGE.to_owned(),
                Fix {
                    start: get.property.start,
                    end: get.property.end,
                    text: "has".to_owned(),
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferHasCheck;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferHasCheck, "src/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferHasCheck, "src/a.ts", code))
    }

    #[test]
    fn reports_and_fixes_the_documented_checks() {
        assert_eq!(
            check("declare const map: Map<string, object>;\nif (map.get(key)) {}"),
            ["Prefer `.has(…)` when checking existence."]
        );
        let cases = [
            (
                "declare const map: Map<string, object>;\nif (map.get(key)) {}",
                "declare const map: Map<string, object>;\nif (map.has(key)) {}",
            ),
            (
                "declare const map: Map<string, object>;\nmap.get(key) !== undefined;",
                "declare const map: Map<string, object>;\nmap.has(key);",
            ),
            (
                "new Map([[key, {}]]).get(key) !== undefined;",
                "new Map([[key, {}]]).has(key);",
            ),
            (
                "new URLSearchParams().get(name) !== null;",
                "new URLSearchParams().has(name);",
            ),
            (
                "const p = new URLSearchParams(); const a = p.get(name) == undefined;",
                "const p = new URLSearchParams(); const a = !p.has(name);",
            ),
            (
                "const m = new Map<string, number>(); const a = undefined === m.get(k);",
                "const m = new Map<string, number>(); const a = !m.has(k);",
            ),
            (
                "const m = new Map<string, number[]>(); const a = !m.get(k);",
                "const m = new Map<string, number[]>(); const a = !m.has(k);",
            ),
            (
                "type Item = { a: number };\nfunction f(m: ReadonlyMap<string, Item>) { return m.get(k) != null; }",
                "type Item = { a: number };\nfunction f(m: ReadonlyMap<string, Item>) { return m.has(k); }",
            ),
            (
                "const a = new Map([['a', 1]]).get(k) !== void 0;",
                "const a = new Map([['a', 1]]).has(k);",
            ),
            (
                "const m = new Map<string, string[]>(); while (m.get(k) && ready) {}",
                "const m = new Map<string, string[]>(); while (m.has(k) && ready) {}",
            ),
        ];
        for (code, after) in cases {
            assert_eq!(fix(code), after, "{code}");
        }
    }

    #[test]
    fn accepts_values_that_could_be_mistaken_for_absence() {
        for code in [
            "declare const map: Map<string, boolean>;\nif (map.get(key)) {}",
            "declare const map: Map<string, number>;\nif (map.get(key)) {}",
            "declare const map: Map<string, number | undefined>;\nmap.get(key) !== undefined;",
            "declare const map: Map<string, unknown>;\nmap.get(key) !== undefined;",
            "declare const map: Map<string, number | null>;\nmap.get(key) != null;",
            "const m = new Map(); const a = m.get(k) !== undefined;",
            "const m = new Map([['a', undefined]]); const a = m.get(k) !== undefined;",
            "const m = new Map([['a', 0]]); if (m.get(k)) {}",
            "const a = cache.get(k) !== undefined;",
            "new URLSearchParams().get(name) !== undefined;",
            "declare const map: Map<string, object>;\nmap.get(key) !== null;",
            "declare const map: Map<string, object>;\nconst v = map.get(key);",
            "declare const map: Map<string, object>;\nmap.get(key) > undefined;",
            "declare const map: Map<string, object>;\nmap?.get(key) !== undefined;",
            "declare const map: Map<string, object>;\nif (map.get(key) as object) {}",
            "type Map<K, V> = { get(k: K): V };\ndeclare const map: Map<string, object>;\nif (map.get(key)) {}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
