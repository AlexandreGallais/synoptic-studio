//! `unicorn/no-unsafe-json-serialization`: a value `JSON.stringify()` or `Response.json()` would
//! drop, change or fail on — a `Map`, a `Set`, a weak collection, a `BigInt`, a function, a
//! symbol, a `RegExp`, `undefined` or a number that is not finite — given directly, in an object
//! or array literal, or through a `const` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-unsafe-json-serialization.md`). An object with a spread, an unknown or repeated
//! key, `toJSON` or `__proto__` is left whole; `undefined` as a property is allowed; a replacer
//! function or unknown replacer leaves the call, a replacer array keeps only its keys. Reported
//! only: the plugin offers suggestions, never a fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrayExpressionElement, Expression, ObjectPropertyKind, PropertyKey, PropertyKind, TSType,
    UnaryOperator,
};
use oxc_semantic::{AstNode, ScopeId};
use oxc_span::GetSpan;

use super::known_types::{Kind, Known, of_value};
use crate::expressions::is_global;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{Value, const_init, static_value, symbol_of, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoUnsafeJsonSerialization;

/// A node no kind adds.
fn no_node(_: &Source, _: &Expression) -> bool {
    false
}

/// An annotation no kind adds.
fn no_annotation(_: &TSType) -> bool {
    false
}

/// A `BigInt`: a literal, `BigInt.asIntN(…)` or `BigInt.asUintN(…)`.
fn is_big_int_node(_: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::BigIntLiteral(_) => true,
        Expression::CallExpression(call) => {
            !call.optional
                && matches!(&call.callee, Expression::StaticMemberExpression(member)
                    if !member.optional && member.object.is_specific_id("BigInt")
                        && matches!(member.property.name.as_str(), "asIntN" | "asUintN"))
        }
        _ => false,
    }
}

/// `bigint` or a `BigInt` literal type.
fn is_big_int_annotation(annotation: &TSType) -> bool {
    match annotation {
        TSType::TSBigIntKeyword(_) => true,
        TSType::TSLiteralType(literal) => {
            matches!(literal.literal, oxc_ast::ast::TSLiteral::BigIntLiteral(_))
        }
        _ => false,
    }
}

/// A function: a function, an arrow, a class, or the name of a function or class declared once
/// and never reassigned.
fn is_function_node(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::FunctionExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::ClassExpression(_) => true,
        Expression::Identifier(name) => {
            let Some(symbol) = symbol_of(source, name) else {
                return false;
            };
            let scoping = source.semantic.scoping();
            let declaration = scoping.symbol_declaration(symbol);
            scoping.symbol_redeclarations(symbol).len() <= 1
                && matches!(
                    source.semantic.nodes().kind(declaration),
                    AstKind::Function(_) | AstKind::Class(_)
                )
                && !scoping
                    .get_resolved_references(symbol)
                    .any(oxc_semantic::Reference::is_write)
        }
        _ => false,
    }
}

/// A function or constructor type.
fn is_function_annotation(annotation: &TSType) -> bool {
    matches!(
        annotation,
        TSType::TSFunctionType(_) | TSType::TSConstructorType(_)
    )
}

/// `Symbol.for(…)`.
fn is_symbol_node(_: &Source, expression: &Expression) -> bool {
    matches!(expression, Expression::CallExpression(call)
        if matches!(&call.callee, Expression::StaticMemberExpression(member)
            if member.object.is_specific_id("Symbol") && member.property.name == "for"))
}

/// `symbol`.
fn is_symbol_annotation(annotation: &TSType) -> bool {
    matches!(annotation, TSType::TSSymbolKeyword(_))
}

/// A regular expression literal.
fn is_regexp_node(_: &Source, expression: &Expression) -> bool {
    matches!(expression, Expression::RegExpLiteral(_))
}

/// The kinds the rule reports, in the order it tries them, with their names in its message.
const KINDS: [(&str, Kind); 8] = [
    (
        "Map",
        Kind {
            names: &["Map", "ReadonlyMap"],
            constructors: &["Map"],
            calls: &[],
            node: no_node,
            annotation: no_annotation,
        },
    ),
    (
        "Set",
        Kind {
            names: &["Set", "ReadonlySet"],
            constructors: &["Set"],
            calls: &[],
            node: no_node,
            annotation: no_annotation,
        },
    ),
    (
        "WeakMap",
        Kind {
            names: &["WeakMap"],
            constructors: &["WeakMap"],
            calls: &[],
            node: no_node,
            annotation: no_annotation,
        },
    ),
    (
        "WeakSet",
        Kind {
            names: &["WeakSet"],
            constructors: &["WeakSet"],
            calls: &[],
            node: no_node,
            annotation: no_annotation,
        },
    ),
    (
        "BigInt",
        Kind {
            names: &[],
            constructors: &[],
            calls: &["BigInt"],
            node: is_big_int_node,
            annotation: is_big_int_annotation,
        },
    ),
    (
        "function",
        Kind {
            names: &[],
            constructors: &[],
            calls: &[],
            node: is_function_node,
            annotation: is_function_annotation,
        },
    ),
    (
        "symbol",
        Kind {
            names: &[],
            constructors: &[],
            calls: &["Symbol"],
            node: is_symbol_node,
            annotation: is_symbol_annotation,
        },
    ),
    (
        "RegExp",
        Kind {
            names: &["RegExp"],
            constructors: &["RegExp"],
            calls: &["RegExp"],
            node: is_regexp_node,
            annotation: no_annotation,
        },
    ),
];

/// The kind of value a serialization cannot keep, when an expression is known to be one.
fn unsafe_kind(source: &Source, scope: ScopeId, expression: &Expression) -> Option<&'static str> {
    for (name, kind) in &KINDS {
        if of_value(source, scope, expression, kind, 0) == Known::Target {
            return Some(name);
        }
    }
    if let Expression::TSSatisfiesExpression(inner) = expression
        && is_big_int_annotation(&inner.type_annotation)
    {
        return Some("BigInt");
    }
    let bare = unwrap(expression);
    if matches!(bare, Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::Void) {
        return Some("undefined");
    }
    match static_value(source, bare)? {
        Value::Undefined => Some("undefined"),
        Value::Number(number) if !number.is_finite() => Some("non-finite number"),
        Value::BigInt(_) => Some("BigInt"),
        _ => None,
    }
}

/// The static name of an object literal's key, or nothing when it is not known.
fn key_name(source: &Source, key: &PropertyKey, computed: bool) -> Option<String> {
    match key {
        PropertyKey::StaticIdentifier(name) if !computed => Some(name.name.to_string()),
        PropertyKey::PrivateIdentifier(_) => None,
        other => match static_value(source, other.as_expression()?)? {
            Value::Text(text) => Some(text),
            Value::Number(number) => Some(format!("{number}")),
            _ => None,
        },
    }
}

/// The search for the values a serialization would lose.
struct Search<'s, 'b, 'a> {
    /// The file.
    source: &'s Source<'b, 'a>,
    /// The scope of the call.
    scope: ScopeId,
    /// The keys a replacer array keeps, when it is one.
    keys: Option<Vec<String>>,
}

impl<'a> Search<'_, '_, 'a> {
    /// The unsafe values under a value, each with its start and kind, the bindings already
    /// followed left.
    fn problems(
        &self,
        expression: &Expression<'a>,
        allow_undefined: bool,
        seen: &[u32],
    ) -> Vec<(u32, &'static str)> {
        let start = expression.span().start;
        let bare = unwrap(expression);
        if seen.contains(&bare.span().start) {
            return Vec::new();
        }
        let path: Vec<u32> = seen.iter().copied().chain([bare.span().start]).collect();
        match bare {
            Expression::ArrayExpression(array) => array
                .elements
                .iter()
                .filter_map(|element| match element {
                    ArrayExpressionElement::SpreadElement(_)
                    | ArrayExpressionElement::Elision(_) => None,
                    other => other.as_expression(),
                })
                .flat_map(|element| self.problems(element, false, &path))
                .collect(),
            Expression::ObjectExpression(object) => {
                let mut names: Vec<String> = Vec::new();
                let mut values = Vec::new();
                for property in &object.properties {
                    let ObjectPropertyKind::ObjectProperty(property) = property else {
                        return Vec::new();
                    };
                    let Some(name) = key_name(self.source, &property.key, property.computed) else {
                        return Vec::new();
                    };
                    if name == "toJSON" || name == "__proto__" || names.contains(&name) {
                        return Vec::new();
                    }
                    names.push(name.clone());
                    let kept = self.keys.as_ref().is_none_or(|keys| keys.contains(&name));
                    if property.kind == PropertyKind::Init && kept {
                        values.push(&property.value);
                    }
                }
                values
                    .into_iter()
                    .flat_map(|value| self.problems(value, true, &path))
                    .collect()
            }
            _ => match unsafe_kind(self.source, self.scope, expression) {
                Some("undefined") if allow_undefined => Vec::new(),
                Some(kind) => vec![(start, kind)],
                None => match const_init(self.source, bare) {
                    Some(init) => self
                        .problems(init, allow_undefined, &path)
                        .first()
                        .map(|(_, kind)| vec![(start, *kind)])
                        .unwrap_or_default(),
                    None => Vec::new(),
                },
            },
        }
    }
}

/// What a replacer keeps.
enum Replacer {
    /// Every key: `null`, `undefined` or another plain value.
    All,
    /// The keys an array of strings and numbers names.
    Keys(Vec<String>),
    /// Not known: a function, or a value not known statically.
    Unknown,
}

/// What a replacer keeps.
fn replacer(source: &Source, replacer: &Expression) -> Replacer {
    match unwrap(replacer) {
        Expression::ArrayExpression(array) => {
            let mut keys = Vec::new();
            for element in &array.elements {
                let value = element
                    .as_expression()
                    .and_then(|element| static_value(source, element));
                match value {
                    Some(Value::Text(text)) => keys.push(text),
                    Some(Value::Number(number)) => keys.push(format!("{number}")),
                    Some(_) => {}
                    None => return Replacer::Unknown,
                }
            }
            Replacer::Keys(keys)
        }
        other if static_value(source, other).is_some() => Replacer::All,
        _ => Replacer::Unknown,
    }
}

impl Rule for NoUnsafeJsonSerialization {
    fn name(&self) -> &'static str {
        "unicorn/no-unsafe-json-serialization"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let (object, method) = match &call.callee {
            Expression::StaticMemberExpression(member) => {
                (&member.object, member.property.name.to_string())
            }
            Expression::ComputedMemberExpression(member) => {
                match static_value(source, &member.expression) {
                    Some(Value::Text(text)) => (&member.object, text),
                    _ => return,
                }
            }
            _ => return,
        };
        let Expression::Identifier(object) = object else {
            return;
        };
        let count = call.arguments.len();
        let named = match (object.name.as_str(), method.as_str()) {
            ("JSON", "stringify") => "JSON.stringify",
            ("Response", "json") if count <= 2 => "Response.json",
            _ => return,
        };
        if count == 0 || count > 3 || !is_global(source, object) {
            return;
        }
        let Some(arguments) = call
            .arguments
            .iter()
            .map(oxc_ast::ast::Argument::as_expression)
            .collect::<Option<Vec<_>>>()
        else {
            return;
        };
        let keys = match arguments.get(1) {
            Some(given) if named == "JSON.stringify" => match replacer(source, given) {
                Replacer::All => None,
                Replacer::Keys(keys) => Some(keys),
                Replacer::Unknown => return,
            },
            _ => None,
        };
        let search = Search {
            source,
            scope: node.scope_id(),
            keys,
        };
        for (at, kind) in search.problems(arguments[0], false, &[]) {
            findings.push(source.finding(
                self.name(),
                at,
                format!("`{named}()` cannot faithfully serialize {kind} values."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnsafeJsonSerialization;
    use crate::testing::lint;

    /// The findings, each its place and message.
    fn found(code: &str) -> Vec<(usize, usize, String)> {
        lint(&NoUnsafeJsonSerialization, "playground/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        found(code)
            .into_iter()
            .map(|(line, column, _)| (line, column))
            .collect()
    }

    #[test]
    fn reports_the_documentation_s_values() {
        // The documentation's five wrong cases, then its right ones.
        let code = "JSON.stringify({permissions: new Set(['read'])});\nJSON.stringify(new Map([['name', 'unicorn']]));\nJSON.stringify({count: 1n});\nJSON.stringify(/unicorn/gi);\nResponse.json({permissions: new Set(['read'])});\nJSON.stringify({permissions: Array.from(new Set(['read']))});\nJSON.stringify(Object.fromEntries(new Map([['name', 'unicorn']])));\nJSON.stringify({count: String(1n)});\nJSON.stringify({optional: undefined});\nResponse.json({permissions: Array.from(new Set(['read']))});\nconst pattern = /unicorn/gi;\nJSON.stringify({source: pattern.source, flags: pattern.flags});\nJSON.stringify({permissions: new Set(['read'])}, (key, value) => value);\nJSON.stringify({permissions: new Set(['read']), name: 'unicorn'}, ['name']);\nJSON.stringify({permissions: new Set(['read']), toJSON() { return {}; }});";

        assert_eq!(
            found(code),
            [
                (
                    1,
                    30,
                    "`JSON.stringify()` cannot faithfully serialize Set values.".to_owned()
                ),
                (
                    2,
                    16,
                    "`JSON.stringify()` cannot faithfully serialize Map values.".to_owned()
                ),
                (
                    3,
                    24,
                    "`JSON.stringify()` cannot faithfully serialize BigInt values.".to_owned()
                ),
                (
                    4,
                    16,
                    "`JSON.stringify()` cannot faithfully serialize RegExp values.".to_owned()
                ),
                (
                    5,
                    29,
                    "`Response.json()` cannot faithfully serialize Set values.".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn follows_constants_annotations_and_nesting() {
        let code = "const s = new Set();\nJSON.stringify(s);\nconst o = { a: [new WeakMap()] };\nJSON.stringify(o);\nfunction f(m: ReadonlyMap<string, number>, n?: Set<string>) { return JSON.stringify([m, n]); }\nJSON.stringify([undefined, () => 1, Symbol('a'), NaN, -Infinity, void 0]);\nfunction g() {}\nJSON.stringify({ g, h() {} });\nfunction make(): WeakSet<object> { return new WeakSet(); }\nJSON.stringify(make());\nJSON.stringify(x as Map<string, string>);\nJSON.stringify(undefined);\nJSON.stringify(BigInt(1), null, 2);\nJSON.stringify(new Set() as unknown);\nJSON.stringify(new Set() as string);";

        assert_eq!(
            places(code),
            [
                (2, 16),
                (4, 16),
                (5, 86),
                (6, 17),
                (6, 28),
                (6, 37),
                (6, 50),
                (6, 55),
                (6, 66),
                (8, 18),
                (8, 22),
                (10, 16),
                (11, 16),
                (12, 16),
                (13, 16),
                (14, 16),
            ]
        );
    }

    #[test]
    fn leaves_unknown_objects_and_other_calls() {
        for code in [
            "JSON.stringify({...a, b: new Set()});",
            "JSON.stringify({[key]: new Set()});",
            "JSON.stringify({a: 1, a: new Set()});",
            "JSON.stringify({__proto__: new Set()});",
            "JSON.stringify(value);",
            "JSON.stringify({a: new Set()}, replacer);",
            "JSON.parse(new Set());",
            "Response.json(new Set(), {}, 1);",
            "const JSON = f;\nJSON.stringify(new Set());",
            "JSON.stringify(...a);",
            "JSON.stringify({ get a() { return new Set(); } });",
            "let s = new Set();\nJSON.stringify(s);",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
