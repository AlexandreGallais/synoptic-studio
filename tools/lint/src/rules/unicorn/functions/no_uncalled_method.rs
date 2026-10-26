//! `unicorn/no-uncalled-method`: an `Array` or `String` method read from a receiver known to be
//! an array or a string, and not called — `array.sort`, `string.toLowerCase`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-uncalled-method.md`). The receiver is known
//! from its syntax, its written type or a `const` holding one, or its name, `array` or `string`;
//! an unknown receiver is left. A method that is called, bound, applied, given to `typeof` or
//! `Reflect.apply`, assigned, or read from a `prototype` stays.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, TSType, UnaryOperator};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::GetSpan;

use crate::expressions::{const_value, symbol, unwrapped, written_type};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUncalledMethod;

/// `Array.prototype`'s methods.
const ARRAY_METHODS: [&str; 38] = [
    "at",
    "concat",
    "copyWithin",
    "entries",
    "every",
    "fill",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "flat",
    "flatMap",
    "forEach",
    "includes",
    "indexOf",
    "join",
    "keys",
    "lastIndexOf",
    "map",
    "pop",
    "push",
    "reduce",
    "reduceRight",
    "reverse",
    "shift",
    "slice",
    "some",
    "sort",
    "splice",
    "toLocaleString",
    "toReversed",
    "toSorted",
    "toSpliced",
    "toString",
    "unshift",
    "values",
    "with",
];

/// `String.prototype`'s methods.
const STRING_METHODS: [&str; 49] = [
    "at",
    "anchor",
    "big",
    "blink",
    "charAt",
    "charCodeAt",
    "codePointAt",
    "concat",
    "endsWith",
    "fixed",
    "fontcolor",
    "fontsize",
    "includes",
    "indexOf",
    "isWellFormed",
    "italics",
    "lastIndexOf",
    "link",
    "localeCompare",
    "match",
    "matchAll",
    "normalize",
    "padEnd",
    "padStart",
    "repeat",
    "replace",
    "replaceAll",
    "search",
    "slice",
    "small",
    "split",
    "startsWith",
    "strike",
    "sub",
    "substr",
    "substring",
    "sup",
    "toLocaleLowerCase",
    "toLocaleUpperCase",
    "toLowerCase",
    "toString",
    "toUpperCase",
    "toWellFormed",
    "trim",
    "trimEnd",
    "trimLeft",
    "trimRight",
    "trimStart",
    "valueOf",
];

/// What a receiver is known to be.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Known {
    Array,
    String,
}

/// What a written type says a value is.
fn known_type(written: &TSType) -> Option<Known> {
    match written {
        TSType::TSArrayType(_) | TSType::TSTupleType(_) => Some(Known::Array),
        TSType::TSTypeOperatorType(operator) => known_type(&operator.type_annotation),
        TSType::TSTypeReference(reference) => {
            let name = reference
                .type_name
                .get_identifier_reference()
                .map(|each| each.name.as_str());
            matches!(name, Some("Array" | "ReadonlyArray")).then_some(Known::Array)
        }
        TSType::TSStringKeyword(_) => Some(Known::String),
        TSType::TSLiteralType(literal) => {
            matches!(literal.literal, oxc_ast::ast::TSLiteral::StringLiteral(_))
                .then_some(Known::String)
        }
        TSType::TSParenthesizedType(inner) => known_type(&inner.type_annotation),
        _ => None,
    }
}

/// What an expression is known to be, from its syntax, its written type or its `const`.
fn known(source: &Source, expression: &Expression, depth: usize) -> Option<Known> {
    if depth > 8 {
        return None;
    }
    match expression {
        Expression::ArrayExpression(_) => Some(Known::Array),
        Expression::StringLiteral(_) | Expression::TemplateLiteral(_) => Some(Known::String),
        Expression::CallExpression(call) => {
            let from = matches!(&call.callee, Expression::StaticMemberExpression(member)
                if !member.optional
                    && member.object.is_specific_id("Array")
                    && matches!(member.property.name.as_str(), "from" | "of"));
            (from && !call.optional).then_some(Known::Array)
        }
        Expression::TSAsExpression(inner) => known_type(&inner.type_annotation),
        Expression::TSSatisfiesExpression(inner) => known(source, &inner.expression, depth + 1),
        Expression::TSNonNullExpression(inner) => known(source, &inner.expression, depth + 1),
        Expression::Identifier(name) => {
            let symbol = symbol(source, name)?;
            match written_type(source, symbol) {
                Some(written) => known_type(&written.type_annotation),
                None => {
                    const_value(source, symbol).and_then(|value| known(source, value, depth + 1))
                }
            }
        }
        _ => None,
    }
}

/// Whether an expression is named `name`, or a `const` holding one so named.
fn named(source: &Source, expression: &Expression, name: &str, depth: usize) -> bool {
    match unwrapped(expression) {
        Expression::Identifier(each) if each.name == name => true,
        Expression::Identifier(each) if depth < 8 => symbol(source, each)
            .and_then(|symbol| const_value(source, symbol))
            .is_some_and(|value| named(source, value, name, depth + 1)),
        _ => false,
    }
}

/// The outermost node of a member read through chains and TypeScript's wrappers.
fn outermost(source: &Source, id: NodeId) -> NodeId {
    let nodes = source.semantic.nodes();
    let mut current = id;
    while matches!(
        nodes.parent_kind(current),
        AstKind::ChainExpression(_)
            | AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
    ) {
        current = nodes.parent_id(current);
    }
    current
}

/// Whether a member read stays: called, bound or applied, given to `typeof` or
/// `Reflect.apply`, written to, or read from `Array.prototype` or `String.prototype`.
fn stays(source: &Source, node: &AstNode, object: &Expression) -> bool {
    let nodes = source.semantic.nodes();
    let top = outermost(source, node.id());
    let span = nodes.kind(top).span();
    let callee = |id: NodeId, span: oxc_span::Span| match nodes.parent_kind(id) {
        AstKind::CallExpression(call) => call.callee.span() == span,
        AstKind::NewExpression(call) => call.callee.span() == span,
        AstKind::TaggedTemplateExpression(tagged) => tagged.tag.span() == span,
        _ => false,
    };
    let bound = match nodes.parent_kind(top) {
        AstKind::StaticMemberExpression(member) => {
            member.object.span() == span
                && matches!(member.property.name.as_str(), "apply" | "bind" | "call")
                && callee(nodes.parent_id(top), member.span)
        }
        _ => false,
    };
    let applied = matches!(nodes.parent_kind(top), AstKind::CallExpression(call)
        if call.arguments.first().is_some_and(|first| first.span() == span)
            && matches!(&call.callee, Expression::StaticMemberExpression(member)
                if member.object.is_specific_id("Reflect") && member.property.name == "apply"));
    let typed = matches!(nodes.parent_kind(top), AstKind::UnaryExpression(unary) if unary.operator == UnaryOperator::Typeof);
    let written = match nodes.parent_kind(top) {
        AstKind::AssignmentExpression(assignment) => assignment.left.span() == span,
        AstKind::ForOfStatement(each) => each.left.span() == span,
        AstKind::ForInStatement(each) => each.left.span() == span,
        AstKind::UpdateExpression(_)
        | AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetWithDefault(_)
        | AstKind::AssignmentTargetPropertyProperty(_) => true,
        _ => false,
    };
    let prototype = matches!(object, Expression::StaticMemberExpression(member)
        if member.property.name == "prototype"
            && (member.object.is_specific_id("Array") || member.object.is_specific_id("String")));
    callee(top, span) || bound || applied || typed || written || prototype
}

impl Rule for NoUncalledMethod {
    fn name(&self) -> &'static str {
        "unicorn/no-uncalled-method"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (object, method, at) = match node.kind() {
            AstKind::StaticMemberExpression(member) => (
                &member.object,
                member.property.name.as_str(),
                member.property.span.start,
            ),
            AstKind::ComputedMemberExpression(member) => match &member.expression {
                Expression::StringLiteral(key) => match key.value.as_str() {
                    Some(name) => (&member.object, name, key.span.start),
                    None => return,
                },
                _ => return,
            },
            _ => return,
        };
        if stays(source, node, object) {
            return;
        }
        let array = ARRAY_METHODS.contains(&method)
            && (named(source, object, "array", 0)
                || known(source, object, 0) == Some(Known::Array));
        let string = STRING_METHODS.contains(&method)
            && (named(source, object, "string", 0)
                || known(source, object, 0) == Some(Known::String));
        if array || string {
            findings.push(source.finding(
                self.name(),
                at,
                format!("Call `{method}()` instead of referencing the method."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUncalledMethod;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoUncalledMethod, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_method_read_without_a_call() {
        // The documentation's cases.
        assert_eq!(places("const sorted = array.sort;"), [(1, 22)]);
        assert_eq!(
            places("function normalize(string) {\n\treturn string.toLowerCase;\n}"),
            [(2, 16)]
        );
        assert_eq!(
            messages(&lint(
                &NoUncalledMethod,
                "scripts/a.ts",
                "const sorted = array.sort;"
            )),
            ["Call `sort()` instead of referencing the method."]
        );
        // Receivers known from a literal, a written type, a `const`, an alias or an assertion.
        let code = "[1].map;\n'a'.trim;\nfunction f(a: string[], b: ReadonlyArray<number>, c: string) { return [a.at, b.join, c.trim]; }\nconst d = [1];\nconst e = d;\nconst g = e.reverse;\nconst list = array;\nconst h = list.flat;\nconst k = (x as string).trim;\nconst m = Array.from(x).keys;\nconst n = `t`['split'];";
        assert_eq!(
            places(code),
            [
                (1, 5),
                (2, 5),
                (3, 74),
                (3, 80),
                (3, 88),
                (6, 13),
                (8, 16),
                (9, 25),
                (10, 25),
                (11, 15)
            ]
        );
    }

    #[test]
    fn leaves_calls_bindings_and_unknown_receivers() {
        for code in [
            "const sorted = array.sort();",
            "array.sort.call(array);",
            "typeof string.toLowerCase === 'function';",
            "const s = list.sort;",
            "const f = Array.prototype.map;",
            "Reflect.apply(array.push, array, []);",
            "array.sort = f;",
            "const n = array.length;",
            "const m = new array.map();",
            "array?.sort();",
            "(array.sort as F)();",
            "function f(a: number) { return a.toString; }",
            "array.join`x`;",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
