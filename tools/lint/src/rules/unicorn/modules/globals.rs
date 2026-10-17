//! What the rules on the global object share: its names, the globals `ESLint` declares for the
//! latest ECMAScript (`conf/globals.js` of `eslint` 10.12.0, `es2026`, read 2026-10-17), whether
//! a member is written, and a member's static name (`isLeftHandSide` and `getPropertyName` of
//! `eslint-plugin-unicorn` 78.0.0 and `@eslint-community/eslint-utils`).

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, ForStatementLeft, UnaryOperator};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::GetSpan;

use crate::rules::unicorn::syntax::{Value, outer_parent, static_value};
use crate::source::Source;

/// The names the global object goes by.
pub const GLOBAL_OBJECTS: [&str; 4] = ["global", "globalThis", "self", "window"];

/// The globals `ESLint` declares when no environment is configured: those of ES2026.
pub const ES_GLOBALS: [&str; 73] = [
    "AggregateError",
    "Array",
    "ArrayBuffer",
    "AsyncDisposableStack",
    "Atomics",
    "BigInt",
    "BigInt64Array",
    "BigUint64Array",
    "Boolean",
    "DataView",
    "Date",
    "DisposableStack",
    "Error",
    "EvalError",
    "FinalizationRegistry",
    "Float16Array",
    "Float32Array",
    "Float64Array",
    "Function",
    "Infinity",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Intl",
    "Iterator",
    "JSON",
    "Map",
    "Math",
    "NaN",
    "Number",
    "Object",
    "Promise",
    "Proxy",
    "RangeError",
    "ReferenceError",
    "Reflect",
    "RegExp",
    "Set",
    "SharedArrayBuffer",
    "String",
    "SuppressedError",
    "Symbol",
    "SyntaxError",
    "Temporal",
    "TypeError",
    "URIError",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "WeakMap",
    "WeakRef",
    "WeakSet",
    "constructor",
    "decodeURI",
    "decodeURIComponent",
    "encodeURI",
    "encodeURIComponent",
    "escape",
    "eval",
    "globalThis",
    "hasOwnProperty",
    "isFinite",
    "isNaN",
    "isPrototypeOf",
    "parseFloat",
    "parseInt",
    "propertyIsEnumerable",
    "toLocaleString",
    "toString",
    "undefined",
    "unescape",
    "valueOf",
];

/// How a member is written, when it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    /// Assigned, updated, destructured into or a loop's target.
    Assigned,
    /// Deleted.
    Deleted,
}

/// How a node is written, through TypeScript's wrappers, when it is.
#[must_use]
pub fn written(source: &Source, id: NodeId) -> Option<Write> {
    let (parent, child) = outer_parent(source, id);
    let span = source.semantic.nodes().kind(child).span();
    let assigned = match parent {
        AstKind::AssignmentExpression(assignment) => assignment.left.span() == span,
        AstKind::AssignmentTargetWithDefault(target) => target.binding.span() == span,
        AstKind::AssignmentTargetPropertyProperty(property) => property.binding.span() == span,
        AstKind::ForOfStatement(statement) => is_left(&statement.left, span),
        AstKind::ForInStatement(statement) => is_left(&statement.left, span),
        AstKind::UpdateExpression(_)
        | AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetRest(_) => true,
        AstKind::UnaryExpression(unary) if unary.operator == UnaryOperator::Delete => {
            return Some(Write::Deleted);
        }
        _ => false,
    };
    assigned.then_some(Write::Assigned)
}

/// Whether a loop's left side is a target at a range.
fn is_left(left: &ForStatementLeft, span: oxc_span::Span) -> bool {
    !matches!(left, ForStatementLeft::VariableDeclaration(_)) && left.span() == span
}

/// The object and the static name of a member: a name, or a computed key whose value is known.
#[must_use]
pub fn member_name<'s, 'a>(
    source: &Source,
    node: &'s AstNode<'a>,
) -> Option<(&'s Expression<'a>, String, bool)> {
    match node.kind() {
        AstKind::StaticMemberExpression(member) => Some((
            &member.object,
            member.property.name.to_string(),
            member.optional,
        )),
        AstKind::ComputedMemberExpression(member) => {
            let name = match static_value(source, &member.expression)? {
                Value::Text(text) => text,
                Value::Number(number) if number.fract() == 0.0 && number.abs() < 1e15 => {
                    format!("{number}")
                }
                _ => return None,
            };
            Some((&member.object, name, member.optional))
        }
        _ => None,
    }
}

/// Whether a text is an `IdentifierName`, as `isIdentifierName` of `@babel/helper-validator-identifier`
/// tells it for the ASCII and Latin letters this code writes.
#[must_use]
pub fn is_identifier_name(text: &str) -> bool {
    let mut characters = text.chars();
    characters
        .next()
        .is_some_and(|first| first == '$' || first == '_' || first.is_alphabetic())
        && characters.all(|each| {
            each == '$'
                || each == '_'
                || each == '\u{200c}'
                || each == '\u{200d}'
                || each.is_alphanumeric()
        })
}
