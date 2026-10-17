//! `unicorn/no-unsafe-string-replacement`: the replacement of `replace()` and `replaceAll()` is
//! a literal string, a `String.raw` of one, a literal's `repeat()` without `$`, a function, or
//! a plain object, never a computed string whose `$&` or `$1` would expand
//! (`eslint-plugin-unicorn` 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression, ObjectPropertyKind, PropertyKind};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::known_values::is_known_non_string;
use crate::rules::unicorn::syntax::{const_init, is_global, method_call, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoUnsafeStringReplacement;

/// The string a literal or a template without expressions holds.
fn static_string<'a>(expression: &Expression<'a>) -> Option<&'a str> {
    match expression {
        Expression::StringLiteral(literal) => Some(literal.value.as_str()),
        Expression::TemplateLiteral(template) if template.expressions.is_empty() => template
            .quasis
            .first()?
            .value
            .cooked
            .map(|cooked| cooked.as_str()),
        _ => None,
    }
}

/// Whether a replacement inserts its text literally or calls back.
fn is_allowed(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::FunctionExpression(_) | Expression::ArrowFunctionExpression(_) => true,
        Expression::TaggedTemplateExpression(tagged) => {
            tagged.quasi.expressions.is_empty()
                && matches!(&tagged.tag, Expression::StaticMemberExpression(member)
                    if !member.optional && member.property.name == "raw" && is_global(source, &member.object, "String"))
        }
        Expression::CallExpression(call) => method_call(call).is_some_and(|method| {
            method.name == "repeat"
                && method.is_plain()
                && static_string(unwrap(method.object)).is_some_and(|text| !text.contains('$'))
        }),
        other => static_string(other).is_some(),
    }
}

/// Whether a replacement is, or a `const` holds, an object literal of plain properties that
/// does not change how it turns into a string.
fn is_plain_object(source: &Source, expression: &Expression) -> bool {
    let unwrapped = unwrap(expression);
    let value = const_init(source, unwrapped).map_or(unwrapped, unwrap);
    let Expression::ObjectExpression(object) = value else {
        return false;
    };
    object.properties.iter().all(|each| match each {
        ObjectPropertyKind::ObjectProperty(property) => {
            !property.computed
                && !property.method
                && property.kind == PropertyKind::Init
                && !property.key.static_name().is_some_and(|name| {
                    matches!(name.as_ref(), "__proto__" | "toString" | "valueOf")
                })
        }
        ObjectPropertyKind::SpreadProperty(_) => false,
    })
}

impl Rule for NoUnsafeStringReplacement {
    fn name(&self) -> &'static str {
        "unicorn/no-unsafe-string-replacement"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call) else {
            return;
        };
        if !matches!(method.name, "replace" | "replaceAll")
            || call.arguments.len() != 2
            || call
                .arguments
                .iter()
                .any(|argument| matches!(argument, Argument::SpreadElement(_)))
        {
            return;
        }
        let Some(replacement) = call.arguments[1].as_expression() else {
            return;
        };
        if is_allowed(source, replacement)
            || is_plain_object(source, replacement)
            || is_known_non_string(source, method.object)
        {
            return;
        }
        findings.push(source.finding(
            self.name(),
            replacement.span().start,
            format!(
                "Do not use a non-literal replacement value with `String#{}()`.",
                method.name
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnsafeStringReplacement;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnsafeStringReplacement, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_unsafe_replacements() {
        assert_eq!(
            check("template.replace('{url}', htmlEscape(url));"),
            ["Do not use a non-literal replacement value with `String#replace()`."]
        );
        assert_eq!(
            check("template.replaceAll('{url}', htmlEscape(url));"),
            ["Do not use a non-literal replacement value with `String#replaceAll()`."]
        );
        assert_eq!(
            check("text.replaceAll('\\t', '$&'.repeat(2));"),
            ["Do not use a non-literal replacement value with `String#replaceAll()`."]
        );
        assert_eq!(
            check("text.replace(/a/g, `${x}`);"),
            ["Do not use a non-literal replacement value with `String#replace()`."]
        );
        assert_eq!(
            check("text?.replace(/a/g, value);"),
            ["Do not use a non-literal replacement value with `String#replace()`."]
        );
    }

    #[test]
    fn accepts_the_documented_safe_replacements() {
        for code in [
            "template.replace('{url}', () => htmlEscape(url));",
            "template.replaceAll('{url}', () => htmlEscape(url));",
            "template.replace('{url}', 'https://example.com');",
            "template.replace('{url}', `https://example.com`);",
            "text.replaceAll('\\t', ' '.repeat(4));",
            "text.replaceAll('\\t', '\\u00A0'.repeat(4));",
            "text.replaceAll('\\t', ' '.repeat(count));",
            "text.replace('a', String.raw`b`);",
            "text.replace('a', function () { return 'b'; });",
            "text.replace('a', {});",
            "const plain = {a: 1}; text.replace('a', plain);",
            "text.replace('a', b, c);",
            "function f(list: number[]) { return list.replace('a', b); }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
