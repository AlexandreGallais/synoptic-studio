//! `unicorn/prefer-direct-iteration`: no `.entries()` of a `Map`, `FormData` or `URLSearchParams`,
//! `.values()` of an array, a typed array or a `Set`, nor `.keys()` of a `Set`, where the result
//! is iterated at once: it is the default iterator already (`eslint-plugin-unicorn` 78.0.0).
//! Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Expression, TSType, TSTypeName, TSTypeOperatorOperator, VariableDeclarationKind,
};
use oxc_semantic::AstNode;

use super::syntax::{Binding, binding_of, has_comment, method_call, method_call_tail};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferDirectIteration;

/// The typed arrays' names.
const TYPED_ARRAYS: [&str; 12] = [
    "Int8Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Int16Array",
    "Uint16Array",
    "Int32Array",
    "Uint32Array",
    "Float16Array",
    "Float32Array",
    "Float64Array",
    "BigInt64Array",
    "BigUint64Array",
];

/// The built-in types whose default iterator is a named method, with the method's names.
fn default_methods(name: &str) -> &'static [&'static str] {
    match name {
        "Array" => &["values"],
        "Map" | "FormData" | "URLSearchParams" => &["entries"],
        "Set" => &["values", "keys"],
        other if TYPED_ARRAYS.contains(&other) => &["values"],
        _ => &[],
    }
}

/// The built-in type names an annotation reads.
fn types_of_annotation(annotation: &TSType) -> Option<Vec<String>> {
    match annotation {
        TSType::TSArrayType(_) | TSType::TSTupleType(_) => Some(vec!["Array".to_owned()]),
        TSType::TSParenthesizedType(inner) => types_of_annotation(&inner.type_annotation),
        TSType::TSTypeOperatorType(operator)
            if operator.operator == TSTypeOperatorOperator::Readonly =>
        {
            types_of_annotation(&operator.type_annotation)
        }
        TSType::TSTypeReference(reference) => {
            let TSTypeName::IdentifierReference(name) = &reference.type_name else {
                return None;
            };
            let name = match name.name.as_str() {
                "ReadonlyArray" => "Array",
                "ReadonlyMap" => "Map",
                "ReadonlySet" => "Set",
                other => other,
            };
            (!default_methods(name).is_empty()).then(|| vec![name.to_owned()])
        }
        TSType::TSUnionType(union) => {
            let mut all = Vec::new();
            for each in &union.types {
                all.extend(types_of_annotation(each)?);
            }
            Some(all)
        }
        _ => None,
    }
}

/// The built-in type names a receiver is known to be, from its syntax, binding or annotation.
fn types_of(source: &Source, expression: &Expression, depth: u8) -> Option<Vec<String>> {
    let named = |name: &str| Some(vec![name.to_owned()]);
    match expression {
        Expression::ArrayExpression(_) => named("Array"),
        Expression::NewExpression(new) => match &new.callee {
            Expression::Identifier(callee) if !default_methods(&callee.name).is_empty() => {
                named(&callee.name)
            }
            _ => None,
        },
        Expression::CallExpression(call) => {
            if matches!(&call.callee, Expression::Identifier(callee) if callee.name == "Array") {
                return named("Array");
            }
            let maker = method_call(call)
                .filter(|method| method.is_plain() && matches!(method.name, "from" | "of"))?;
            match maker.object {
                Expression::Identifier(object) if object.name == "Array" => named("Array"),
                Expression::Identifier(object) if TYPED_ARRAYS.contains(&object.name.as_str()) => {
                    named(&object.name)
                }
                _ => None,
            }
        }
        Expression::TSAsExpression(wrapper) => types_of_annotation(&wrapper.type_annotation)
            .or_else(|| types_of(source, &wrapper.expression, depth)),
        Expression::TSSatisfiesExpression(wrapper) => types_of_annotation(&wrapper.type_annotation)
            .or_else(|| types_of(source, &wrapper.expression, depth)),
        Expression::TSTypeAssertion(wrapper) => types_of_annotation(&wrapper.type_annotation)
            .or_else(|| types_of(source, &wrapper.expression, depth)),
        Expression::TSNonNullExpression(wrapper) => types_of(source, &wrapper.expression, depth),
        Expression::Identifier(_) if depth < 4 => match binding_of(source, expression)? {
            Binding::Variable(declarator, kind) => {
                if let Some(annotation) = &declarator.type_annotation {
                    return types_of_annotation(&annotation.type_annotation);
                }
                (kind == VariableDeclarationKind::Const)
                    .then_some(())
                    .and(declarator.init.as_ref())
                    .and_then(|init| types_of(source, init, depth + 1))
            }
            Binding::Parameter(parameter) => parameter
                .type_annotation
                .as_ref()
                .and_then(|annotation| types_of_annotation(&annotation.type_annotation)),
        },
        _ => None,
    }
}

/// Whether the call's value is iterated at once where it stands.
fn is_iterated(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    let is_first = |arguments: &[oxc_ast::ast::Argument]| {
        arguments
            .first()
            .and_then(|argument| argument.as_expression())
            .is_some_and(|argument| argument.node_id() == node.id())
    };
    match parent.kind() {
        AstKind::ForOfStatement(statement) => statement.right.node_id() == node.id(),
        AstKind::YieldExpression(expression) => expression.delegate,
        AstKind::SpreadElement(_) => matches!(
            nodes.parent_kind(parent.id()),
            AstKind::ArrayExpression(_) | AstKind::CallExpression(_) | AstKind::NewExpression(_)
        ),
        AstKind::NewExpression(new) => {
            new.arguments.len() == 1
                && is_first(&new.arguments)
                && matches!(&new.callee, Expression::Identifier(callee)
                    if matches!(callee.name.as_str(), "Map" | "WeakMap" | "Set" | "WeakSet")
                        || TYPED_ARRAYS.contains(&callee.name.as_str()))
        }
        AstKind::CallExpression(call) => {
            let Some(method) = method_call(call).filter(super::syntax::MethodCall::is_plain) else {
                return false;
            };
            let object = |name: &str| matches!(method.object, Expression::Identifier(object) if object.name == name);
            let consumes = (method.name == "from"
                && (object("Array")
                    || matches!(method.object, Expression::Identifier(object) if TYPED_ARRAYS.contains(&object.name.as_str()))))
                || (method.name == "fromEntries" && object("Object"))
                || (matches!(method.name, "all" | "allSettled" | "any" | "race")
                    && object("Promise")
                    && call.arguments.len() == 1);
            consumes && is_first(&call.arguments)
        }
        _ => false,
    }
}

impl Rule for PreferDirectIteration {
    fn name(&self) -> &'static str {
        "unicorn/prefer-direct-iteration"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call).filter(|method| {
            matches!(method.name, "entries" | "keys" | "values")
                && method.is_plain()
                && call.arguments.is_empty()
        }) else {
            return;
        };
        if !is_iterated(source, node) {
            return;
        }
        let Some(types) = types_of(source, method.object, 0).filter(|types| !types.is_empty())
        else {
            return;
        };
        if !types
            .iter()
            .all(|each| default_methods(each).contains(&method.name))
        {
            return;
        }
        let message = format!("Prefer direct iteration instead of `.{}()`.", method.name);
        if has_comment(source, call.span) {
            findings.push(source.finding(self.name(), method.property.start, message));
            return;
        }
        let tail = method_call_tail(source, &method);
        findings.push(source.fixed(
            self.name(),
            method.property.start,
            message,
            Fix {
                start: tail.start,
                end: tail.end,
                text: String::new(),
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferDirectIteration;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferDirectIteration, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferDirectIteration, "playground/a.ts", code))
    }

    #[test]
    fn reports_and_fixes_the_documented_calls() {
        assert_eq!(
            check("const map = new Map();\nfor (const entry of map.entries()) {}"),
            ["Prefer direct iteration instead of `.entries()`."]
        );
        let cases = [
            (
                "const map = new Map();\nfor (const entry of map.entries()) {}",
                "const map = new Map();\nfor (const entry of map) {}",
            ),
            (
                "const array = [1, 2, 3];\nconst values = [...array.values()];",
                "const array = [1, 2, 3];\nconst values = [...array];",
            ),
            (
                "const map = new Map();\nconst copy = new Map(map.entries());",
                "const map = new Map();\nconst copy = new Map(map);",
            ),
            (
                "function f(s: ReadonlySet<number>) { return Array.from(s.keys()); }",
                "function f(s: ReadonlySet<number>) { return Array.from(s); }",
            ),
            (
                "const p = new URLSearchParams();\nconst o = Object.fromEntries(p.entries());",
                "const p = new URLSearchParams();\nconst o = Object.fromEntries(p);",
            ),
            (
                "function* g(a: number[] | Uint8Array) { yield* a.values(); }",
                "function* g(a: number[] | Uint8Array) { yield* a; }",
            ),
            (
                "const a = Array.of(1);\nPromise.all(a.values());",
                "const a = Array.of(1);\nPromise.all(a);",
            ),
        ];
        for (code, after) in cases {
            assert_eq!(fix(code), after, "{code}");
        }
    }

    #[test]
    fn accepts_other_methods_receivers_and_places() {
        for code in [
            "const map = new Map();\nfor (const entry of map) {}",
            "const array = [1];\nfor (const entry of array.entries()) {}",
            "for (const item of collection.values()) {}",
            "const map = new Map();\nfor (const value of map.values()) {}",
            "const set = new Set();\nfor (const entry of set.entries()) {}",
            "const map = new Map();\nconst it = map.entries();",
            "const map = new Map();\nuse(map.entries());",
            "const map = new Map();\nfor (const e of map?.entries()) {}",
            "const map = new Map();\nfor (const e of map.entries(1)) {}",
            "function f(a: number[] | Map<string, number>) { for (const e of a.values()) {} }",
            "let map = new Map();\nfor (const e of map.entries()) {}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
