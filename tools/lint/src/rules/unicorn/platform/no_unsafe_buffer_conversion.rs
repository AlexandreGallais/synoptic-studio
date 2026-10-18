//! `unicorn/no-unsafe-buffer-conversion`: a view's whole `.buffer` read where only the view's
//! bytes are meant — a typed array built on it, `Buffer.from(…)` of it, or `.slice()` of it —
//! without the view's `byteOffset` and `byteLength` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-unsafe-buffer-conversion.md`). Without type information, every `.buffer` is a
//! view's; when an offset and a length are both given, only those known wrong are reported: `0`,
//! `undefined`, the view's length for the offset, the buffer's or a mismatched length. Reported
//! only: the plugin offers a suggestion, never a fix. `new DataView(…)` is not read.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, BinaryOperator, CallExpression, Expression, ImportDeclarationSpecifier,
    ModuleExportName,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::is_global;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::modules::globals::GLOBAL_OBJECTS;
use crate::rules::unicorn::syntax::{same_reference, symbol_of, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoUnsafeBufferConversion;

/// The rule's message.
const MESSAGE: &str =
    "Preserve `byteOffset` and `byteLength` when converting an ArrayBuffer view through `.buffer`.";

/// The typed arrays and the bytes each element takes.
const TYPED_ARRAYS: [(&str, u8); 12] = [
    ("Int8Array", 1),
    ("Uint8Array", 1),
    ("Uint8ClampedArray", 1),
    ("Int16Array", 2),
    ("Uint16Array", 2),
    ("Float16Array", 2),
    ("Int32Array", 4),
    ("Uint32Array", 4),
    ("Float32Array", 4),
    ("Float64Array", 8),
    ("BigInt64Array", 8),
    ("BigUint64Array", 8),
];

/// The bytes an element of a typed array takes, by its name.
fn bytes_per_element(name: &str) -> Option<u8> {
    TYPED_ARRAYS
        .iter()
        .find(|(each, _)| *each == name)
        .map(|(_, bytes)| *bytes)
}

/// The view whose `.buffer` an expression reads, `view.buffer` or `view?.buffer`.
fn buffer_view<'s, 'a>(expression: &'s Expression<'a>) -> Option<&'s Expression<'a>> {
    let expression = match expression {
        Expression::ChainExpression(chain) => chain.expression.as_member_expression()?,
        other => other.as_member_expression()?,
    };
    match expression {
        oxc_ast::ast::MemberExpression::StaticMemberExpression(member)
            if member.property.name == "buffer" =>
        {
            Some(&member.object)
        }
        _ => None,
    }
}

/// Whether an expression reads a static member of a view, `view.name`.
fn is_member_of(expression: &Expression, names: &[&str], view: &Expression) -> bool {
    matches!(expression, Expression::StaticMemberExpression(member)
        if names.contains(&member.property.name.as_str()) && same_reference(&member.object, view))
}

/// Whether an expression is the number `0` or the global `undefined`.
fn is_zero_or_undefined(source: &Source, expression: &Expression) -> (bool, bool) {
    match expression {
        Expression::NumericLiteral(number) => (number.value == 0.0, false),
        Expression::Identifier(name) => {
            (false, name.name == "undefined" && is_global(source, name))
        }
        _ => (false, false),
    }
}

/// Whether an offset is known wrong: `0`, `undefined`, or the view's length.
fn is_unsafe_offset(source: &Source, offset: &Expression, view: &Expression) -> bool {
    let (zero, undefined) = is_zero_or_undefined(source, offset);
    zero || undefined || is_member_of(offset, &["byteLength", "length"], view)
}

/// Whether a length is known wrong: `undefined`, or the whole buffer's length.
fn is_unsafe_length(source: &Source, length: &Expression, view: &Expression) -> bool {
    is_zero_or_undefined(source, length).1
        || matches!(length, Expression::StaticMemberExpression(member)
            if member.property.name == "byteLength"
                && matches!(&member.object, Expression::StaticMemberExpression(buffer)
                    if buffer.property.name == "buffer" && same_reference(&buffer.object, view)))
}

/// The bytes per element a length divides the view's `byteLength` by, `view.byteLength /
/// Name.BYTES_PER_ELEMENT`.
fn divided_length(length: &Expression, view: &Expression) -> Option<u8> {
    let Expression::BinaryExpression(division) = length else {
        return None;
    };
    if division.operator != BinaryOperator::Division
        || !is_member_of(&division.left, &["byteLength"], view)
    {
        return None;
    }
    match &division.right {
        Expression::StaticMemberExpression(member)
            if member.property.name == "BYTES_PER_ELEMENT" =>
        {
            match &member.object {
                Expression::Identifier(name) => bytes_per_element(&name.name),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The arguments, unless one of the first three is spread.
fn plain_arguments<'s, 'a>(arguments: &'s [Argument<'a>]) -> Option<Vec<&'s Expression<'a>>> {
    arguments
        .iter()
        .take(3)
        .map(Argument::as_expression)
        .collect()
}

/// Whether a typed array is built on a view's whole buffer.
fn is_unsafe_typed_array(source: &Source, callee: &Expression, arguments: &[Argument]) -> bool {
    let Expression::Identifier(name) = callee else {
        return false;
    };
    let Some(bytes) = bytes_per_element(&name.name) else {
        return false;
    };
    if !is_global(source, name) || arguments.is_empty() || arguments.len() > 3 {
        return false;
    }
    let Some(arguments) = plain_arguments(arguments) else {
        return false;
    };
    let Some(view) = buffer_view(arguments[0]) else {
        return false;
    };
    let (Some(offset), Some(length)) = (arguments.get(1), arguments.get(2)) else {
        return true;
    };
    let safe_offset = is_member_of(offset, &["byteOffset"], view);
    let safe_length = if bytes == 1 {
        is_member_of(length, &["byteLength"], view)
    } else {
        divided_length(length, view) == Some(bytes)
    };
    if safe_offset && safe_length {
        return false;
    }
    let mismatched = is_member_of(length, &["byteLength"], view)
        || divided_length(length, view).is_some_and(|each| each != bytes);
    is_unsafe_offset(source, offset, view)
        || (!safe_length && (is_unsafe_length(source, length, view) || mismatched))
}

/// Whether an expression is `Buffer`: the global, read from the global object, or imported from
/// `buffer`.
fn is_buffer(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::StaticMemberExpression(member) if member.property.name == "Buffer" => {
            match unwrap(&member.object) {
                Expression::Identifier(object) => {
                    (GLOBAL_OBJECTS.contains(&object.name.as_str()) && is_global(source, object))
                        || matches!(
                            buffer_import(source, object),
                            Some(
                                ImportDeclarationSpecifier::ImportNamespaceSpecifier(_)
                                    | ImportDeclarationSpecifier::ImportDefaultSpecifier(_)
                            )
                        )
                }
                _ => false,
            }
        }
        Expression::Identifier(name) => {
            (name.name == "Buffer" && is_global(source, name))
                || matches!(buffer_import(source, name),
                    Some(ImportDeclarationSpecifier::ImportSpecifier(specifier))
                        if matches!(&specifier.imported, ModuleExportName::IdentifierName(imported) if imported.name == "Buffer"))
        }
        _ => false,
    }
}

/// The specifier importing a name from `buffer` or `node:buffer`.
fn buffer_import<'a>(
    source: &Source<'_, 'a>,
    name: &oxc_ast::ast::IdentifierReference,
) -> Option<&'a ImportDeclarationSpecifier<'a>> {
    let symbol = symbol_of(source, name)?;
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let AstKind::ImportDeclaration(import) = source.semantic.nodes().parent_kind(declaration)
    else {
        return None;
    };
    if !matches!(import.source.value.as_str(), "buffer" | "node:buffer") {
        return None;
    }
    import.specifiers.as_ref()?.iter().find(|each| {
        let local = match each {
            ImportDeclarationSpecifier::ImportSpecifier(specifier) => &specifier.local,
            ImportDeclarationSpecifier::ImportDefaultSpecifier(specifier) => &specifier.local,
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(specifier) => &specifier.local,
        };
        local.symbol_id.get() == Some(symbol)
    })
}

/// The receiver and the name of a call of a static, non-computed member.
fn method<'s, 'a>(call: &'s CallExpression<'a>) -> Option<(&'s Expression<'a>, &'a str)> {
    match &call.callee {
        Expression::StaticMemberExpression(member) => {
            Some((&member.object, member.property.name.as_str()))
        }
        _ => None,
    }
}

/// Whether `Buffer.from(…)` reads a view's whole buffer.
fn is_unsafe_buffer_from(source: &Source, call: &CallExpression) -> bool {
    let Some((object, "from")) = method(call) else {
        return false;
    };
    if call.arguments.is_empty() || call.arguments.len() > 3 || !is_buffer(source, object) {
        return false;
    }
    let Some(arguments) = plain_arguments(&call.arguments) else {
        return false;
    };
    let Some(view) = buffer_view(arguments[0]) else {
        return false;
    };
    let (Some(offset), Some(length)) = (arguments.get(1), arguments.get(2)) else {
        return true;
    };
    let safe_offset = is_member_of(offset, &["byteOffset"], view);
    let safe_length = is_member_of(length, &["byteLength"], view);
    if safe_offset && safe_length {
        return false;
    }
    is_unsafe_offset(source, offset, view)
        || is_unsafe_length(source, length, view)
        || divided_length(length, view).is_some_and(|bytes| bytes > 1)
}

/// Whether `view.buffer.slice(…)` copies more than the view's bytes.
fn is_unsafe_slice(source: &Source, call: &CallExpression) -> bool {
    let Some((object, "slice")) = method(call) else {
        return false;
    };
    if call.arguments.len() > 2 {
        return false;
    }
    let Some(arguments) = plain_arguments(&call.arguments) else {
        return false;
    };
    let Some(view) = buffer_view(object) else {
        return false;
    };
    let (Some(start), Some(end)) = (arguments.first(), arguments.get(1)) else {
        return true;
    };
    let safe_start = is_member_of(start, &["byteOffset"], view);
    let safe_end = matches!(end, Expression::BinaryExpression(sum)
        if sum.operator == BinaryOperator::Addition
            && is_member_of(&sum.left, &["byteOffset"], view)
            && is_member_of(&sum.right, &["byteLength"], view));
    if safe_start && safe_end {
        return false;
    }
    is_unsafe_offset(source, start, view)
        || (safe_start && is_member_of(end, &["byteLength"], view))
        || is_unsafe_length(source, end, view)
}

impl Rule for NoUnsafeBufferConversion {
    fn name(&self) -> &'static str {
        "unicorn/no-unsafe-buffer-conversion"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let unsafe_conversion = match node.kind() {
            AstKind::NewExpression(new) => {
                is_unsafe_typed_array(source, &new.callee, &new.arguments)
            }
            AstKind::CallExpression(call) => {
                is_unsafe_buffer_from(source, call) || is_unsafe_slice(source, call)
            }
            _ => false,
        };
        if unsafe_conversion {
            findings.push(source.finding(
                self.name(),
                node.kind().span().start,
                MESSAGE.to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnsafeBufferConversion;
    use crate::testing::lint;

    /// The lines of the findings.
    fn lines(code: &str) -> Vec<usize> {
        lint(&NoUnsafeBufferConversion, "src/a/b.ts", code)
            .iter()
            .map(|each| each.line)
            .collect()
    }

    #[test]
    fn reports_a_view_s_whole_buffer() {
        // The documentation's four cases, each wrong form then its right one.
        let code = "new Uint8Array(buffer.buffer);\nnew Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);\nnew Uint32Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);\nnew Uint32Array(buffer.buffer, buffer.byteOffset, buffer.byteLength / Uint32Array.BYTES_PER_ELEMENT);\nBuffer.from(bytes.buffer);\nBuffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength);\nbytes.buffer.slice();\nbytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);";

        assert_eq!(lines(code), [1, 3, 5, 7]);
    }

    #[test]
    fn reports_offsets_and_lengths_known_wrong() {
        let code = "new Uint8Array(a.buffer, 0, a.byteLength);\nnew Uint8Array(a.buffer, a.byteOffset, undefined);\nnew Uint8Array(a.buffer, a.byteOffset, a.buffer.byteLength);\nnew Int16Array(a.buffer, a.byteOffset, a.byteLength / Uint32Array.BYTES_PER_ELEMENT);\nnew Uint8Array(a.buffer, a.length, n);\nnew Uint8Array(a.buffer, start, n);\nnew Uint8Array(a?.buffer);\nnew Uint8Array(a.buffer, a.byteOffset);\nBuffer.from(a.buffer, 0, n);\nBuffer.from(a.buffer, a.byteOffset, a.byteLength / Float64Array.BYTES_PER_ELEMENT);\nglobalThis.Buffer.from(a.buffer);\na.buffer.slice(a.byteOffset, a.byteLength);\na.buffer.slice(0, end);\na.buffer.slice(start, end);";

        assert_eq!(lines(code), [1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13]);
    }

    #[test]
    fn follows_buffer_imports_and_leaves_other_constructors() {
        let code = "import { Buffer as B } from 'node:buffer';\nimport * as buffer from 'buffer';\nB.from(a.buffer);\nbuffer.Buffer.from(a.buffer);\nnew DataView(a.buffer);\nnew Other(a.buffer);\nconst Uint8Array = f;\nnew Uint8Array(a.buffer);\nOther.from(a.buffer);\nBuffer.from(a.data);\nnew Float64Array(...a);\na.buffer.slice(1, 2, 3);";

        assert_eq!(lines(code), [3, 4]);
    }
}
