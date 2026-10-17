//! `unicorn/no-invalid-well-known-symbol-methods`: `[Symbol.dispose]` is neither `async`, nor a
//! generator, nor typed to return a `Promise`; `[Symbol.iterator]` returns a sync iterator; an
//! `async` `[Symbol.asyncIterator]` is a generator (`eslint-plugin-unicorn` 78.0.0). An async
//! `[Symbol.dispose]` is fixed to `[Symbol.asyncDispose]` when no sibling has that key; the
//! iterator's renaming is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ClassElement, Expression, MethodDefinitionKind, ObjectPropertyKind, PropertyKey, PropertyKind,
    TSType, TSTypeAnnotation, TSTypeName,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoInvalidWellKnownSymbolMethods;

/// A function a member holds: whether it is `async`, a generator, and its return type.
struct Method<'b, 'a> {
    /// Whether it is `async`.
    is_async: bool,
    /// Whether it is a generator.
    generator: bool,
    /// Its return type.
    return_type: Option<&'b TSTypeAnnotation<'a>>,
}

/// The function an expression is, when it is a function or an arrow.
fn method_of<'b, 'a>(value: &'b Expression<'a>) -> Option<Method<'b, 'a>> {
    match value {
        Expression::FunctionExpression(function) => Some(Method {
            is_async: function.r#async,
            generator: function.generator,
            return_type: function.return_type.as_deref(),
        }),
        Expression::ArrowFunctionExpression(arrow) => Some(Method {
            is_async: arrow.r#async,
            generator: false,
            return_type: arrow.return_type.as_deref(),
        }),
        _ => None,
    }
}

/// The name of the well-known symbol a computed key reads, `Symbol.name`, and where the name is.
fn symbol_name<'a>(key: &PropertyKey<'a>, computed: bool) -> Option<(&'a str, Span)> {
    if !computed {
        return None;
    }
    match key {
        PropertyKey::StaticMemberExpression(member) => match &member.object {
            Expression::Identifier(object) if object.name == "Symbol" => {
                Some((member.property.name.as_str(), member.property.span))
            }
            _ => None,
        },
        _ => None,
    }
}

/// Whether a key is `Symbol.asyncDispose` or `Symbol['asyncDispose']`.
fn is_async_dispose(key: &PropertyKey, computed: bool) -> bool {
    if !computed {
        return false;
    }
    match key {
        PropertyKey::StaticMemberExpression(member) => {
            matches!(&member.object, Expression::Identifier(object) if object.name == "Symbol")
                && member.property.name == "asyncDispose"
        }
        PropertyKey::ComputedMemberExpression(member) => {
            matches!(&member.object, Expression::Identifier(object) if object.name == "Symbol")
                && matches!(&member.expression, Expression::StringLiteral(literal) if literal.value == "asyncDispose")
        }
        _ => false,
    }
}

/// Whether a return type names `Promise` or `PromiseLike`, alone or in a union.
fn returns_promise(annotation: Option<&TSTypeAnnotation>) -> bool {
    let is_promise = |each: &TSType| {
        matches!(each, TSType::TSTypeReference(reference)
            if matches!(&reference.type_name, TSTypeName::IdentifierReference(name)
                if matches!(name.name.as_str(), "Promise" | "PromiseLike")))
    };
    annotation.is_some_and(|annotation| match &annotation.type_annotation {
        TSType::TSUnionType(union) => union.types.iter().any(is_promise),
        other => is_promise(other),
    })
}

impl Rule for NoInvalidWellKnownSymbolMethods {
    fn name(&self) -> &'static str {
        "unicorn/no-invalid-well-known-symbol-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let parent = source.semantic.nodes().parent_kind(node.id());
        let (key, computed, value, other_async_dispose) = match node.kind() {
            AstKind::ObjectProperty(property) if property.kind == PropertyKind::Init => {
                let AstKind::ObjectExpression(object) = parent else {
                    return;
                };
                let other = object.properties.iter().any(|each| {
                    matches!(each, ObjectPropertyKind::ObjectProperty(sibling)
                        if sibling.span != property.span && is_async_dispose(&sibling.key, sibling.computed))
                });
                (&property.key, property.computed, &property.value, other)
            }
            AstKind::MethodDefinition(method)
                if !matches!(
                    method.kind,
                    MethodDefinitionKind::Get | MethodDefinitionKind::Set
                ) =>
            {
                let other = class_has_async_dispose(parent, method.span, method.r#static);
                let function = &method.value;
                let method_info = Method {
                    is_async: function.r#async,
                    generator: function.generator,
                    return_type: function.return_type.as_deref(),
                };
                self.report(
                    source,
                    &method.key,
                    method.computed,
                    &method_info,
                    other,
                    findings,
                );
                return;
            }
            AstKind::PropertyDefinition(property) => {
                let Some(value) = &property.value else {
                    return;
                };
                let other = class_has_async_dispose(parent, property.span, property.r#static);
                (&property.key, property.computed, value, other)
            }
            _ => return,
        };
        if let Some(method) = method_of(value) {
            self.report(
                source,
                key,
                computed,
                &method,
                other_async_dispose,
                findings,
            );
        }
    }
}

/// Whether a class body has another member of key `Symbol.asyncDispose`, as static as this one.
fn class_has_async_dispose(parent: AstKind, span: Span, is_static: bool) -> bool {
    let AstKind::ClassBody(body) = parent else {
        return false;
    };
    body.body.iter().any(|element| match element {
        ClassElement::MethodDefinition(method) => {
            method.span != span
                && method.r#static == is_static
                && is_async_dispose(&method.key, method.computed)
        }
        ClassElement::PropertyDefinition(property) => {
            property.span != span
                && property.r#static == is_static
                && is_async_dispose(&property.key, property.computed)
        }
        _ => false,
    })
}

impl NoInvalidWellKnownSymbolMethods {
    /// Reports a member whose function does not fit its well-known symbol.
    fn report(
        &self,
        source: &Source,
        key: &PropertyKey,
        computed: bool,
        method: &Method,
        other_async_dispose: bool,
        findings: &mut Vec<Finding>,
    ) {
        let Some((name, name_span)) = symbol_name(key, computed) else {
            return;
        };
        let at = key.span().start;
        let promise = !method.is_async && returns_promise(method.return_type);
        match name {
            "dispose" if method.generator => findings.push(source.finding(
                self.name(),
                at,
                "`Symbol.dispose` must not be a generator. Use a normal method for synchronous disposal or `Symbol.asyncDispose` for asynchronous disposal.".to_owned(),
            )),
            "dispose" if method.is_async || promise => {
                let message =
                    "`Symbol.dispose` must not return a Promise. Use `Symbol.asyncDispose` instead."
                        .to_owned();
                findings.push(if other_async_dispose {
                    source.finding(self.name(), at, message)
                } else {
                    source.fixed(
                        self.name(),
                        at,
                        message,
                        Fix {
                            start: name_span.start,
                            end: name_span.end,
                            text: "asyncDispose".to_owned(),
                        },
                    )
                });
            }
            "iterator" if method.is_async || promise => findings.push(source.finding(
                self.name(),
                at,
                "`Symbol.iterator` must return a sync iterator. Use `Symbol.asyncIterator` for async iterators.".to_owned(),
            )),
            "asyncIterator" if !method.generator && (method.is_async || promise) => {
                findings.push(source.finding(
                    self.name(),
                    at,
                    "`Symbol.asyncIterator` must return an async iterator directly. Use an async generator or return the iterator synchronously.".to_owned(),
                ));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidWellKnownSymbolMethods;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &NoInvalidWellKnownSymbolMethods,
            "playground/a.ts",
            code,
        ))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&NoInvalidWellKnownSymbolMethods, "playground/a.ts", code),
        )
    }

    #[test]
    fn reports_the_documented_invalid_methods() {
        assert_eq!(
            check("class Resource {\n\tasync [Symbol.dispose]() {}\n}"),
            ["`Symbol.dispose` must not return a Promise. Use `Symbol.asyncDispose` instead."]
        );
        assert_eq!(
            check("const iterable = {\n\tasync *[Symbol.iterator]() {\n\t\tyield value;\n\t},\n};"),
            [
                "`Symbol.iterator` must return a sync iterator. Use `Symbol.asyncIterator` for async iterators."
            ]
        );
        assert_eq!(
            check(
                "const asyncIterable = {\n\tasync [Symbol.asyncIterator]() {\n\t\treturn iterator;\n\t},\n};"
            ),
            [
                "`Symbol.asyncIterator` must return an async iterator directly. Use an async generator or return the iterator synchronously."
            ]
        );
        assert_eq!(
            check("const o = { *[Symbol.dispose]() {} };"),
            [
                "`Symbol.dispose` must not be a generator. Use a normal method for synchronous disposal or `Symbol.asyncDispose` for asynchronous disposal."
            ]
        );
        assert_eq!(
            check("const o = { [Symbol.iterator]: (): Promise<T> => p };"),
            [
                "`Symbol.iterator` must return a sync iterator. Use `Symbol.asyncIterator` for async iterators."
            ]
        );
    }

    #[test]
    fn fixes_an_async_dispose_without_a_sibling() {
        assert_eq!(
            fix("const o = { async [Symbol.dispose]() {} };"),
            "const o = { async [Symbol.asyncDispose]() {} };"
        );
        let both =
            "const o = { async [Symbol.dispose]() {}, async [Symbol['asyncDispose']]() {} };";
        assert_eq!(fix(both), both);
    }

    #[test]
    fn accepts_the_documented_valid_methods() {
        for code in [
            "class Resource {\n\tasync [Symbol.asyncDispose]() {}\n}",
            "const iterable = {\n\tasync *[Symbol.asyncIterator]() {\n\t\tyield value;\n\t},\n};",
            "const o = { *[Symbol.iterator]() {} };",
            "const o = { [Symbol.dispose]() {} };",
            "const o = { get [Symbol.iterator]() { return f; } };",
            "const o = { async [iterator]() {} };",
            "const o = { [Symbol.iterator]: value };",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
