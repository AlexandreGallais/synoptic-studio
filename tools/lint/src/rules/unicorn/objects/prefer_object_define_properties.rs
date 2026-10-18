//! `unicorn/prefer-object-define-properties`: adjacent `Object.defineProperty()` statements on
//! the same target are one `Object.defineProperties()` (`eslint-plugin-unicorn` 78.0.0).
//! Autofixed, as in `ESLint`, unless a key repeats, a comment or a multi-line string sits in
//! the range, or the object literal would define the keys in another order.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, CallExpression, Expression, Statement};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::known_values::{KeyName, expression_key_name};
use crate::rules::unicorn::layout::{indent_before, indent_unit, reindent};
use crate::rules::unicorn::modules::globals::is_identifier_name;
use crate::rules::unicorn::syntax::{has_comment, same_reference, text};
use crate::source::Source;

/// The rule.
pub struct PreferObjectDefineProperties;

/// The `Object.defineProperty(target, key, descriptor)` call a statement is.
fn define_property<'b, 'a>(statement: &'b Statement<'a>) -> Option<&'b CallExpression<'a>> {
    let Statement::ExpressionStatement(statement) = statement else {
        return None;
    };
    let Expression::CallExpression(call) = &statement.expression else {
        return None;
    };
    let Expression::StaticMemberExpression(member) = &call.callee else {
        return None;
    };
    let plain = !call.optional
        && !member.optional
        && member.property.name == "defineProperty"
        && matches!(&member.object, Expression::Identifier(object) if object.name == "Object")
        && call.arguments.len() == 3
        && call
            .arguments
            .iter()
            .all(|argument| !matches!(argument, Argument::SpreadElement(_)));
    plain.then_some(call)
}

/// The expression of a call's argument.
fn argument<'b, 'a>(call: &'b CallExpression<'a>, index: usize) -> Option<&'b Expression<'a>> {
    call.arguments.get(index)?.as_expression()
}

/// The statements of the list a statement sits in.
fn siblings<'a>(source: &Source<'_, 'a>, id: NodeId) -> Option<&'a [Statement<'a>]> {
    Some(match source.semantic.nodes().parent_kind(id) {
        AstKind::Program(program) => &program.body,
        AstKind::BlockStatement(block) => &block.body,
        AstKind::FunctionBody(body) => &body.statements,
        AstKind::SwitchCase(case) => &case.consequent,
        AstKind::StaticBlock(block) => &block.body,
        _ => return None,
    })
}

/// The key a call's second argument writes, as the object literal will.
fn key_text(source: &Source, key: &Expression) -> String {
    let written = text(source, key.span());
    match key {
        Expression::StringLiteral(literal) if literal.value == "__proto__" => {
            format!("[{written}]")
        }
        Expression::StringLiteral(literal) if is_identifier_name(&literal.value) => {
            literal.value.to_string()
        }
        Expression::StringLiteral(_) | Expression::NumericLiteral(_) => written.to_owned(),
        _ => format!("[{written}]"),
    }
}

/// Whether a text is an array index, which an object literal defines first.
fn is_array_index(key: &str) -> bool {
    !key.is_empty()
        && key.chars().all(|each| each.is_ascii_digit())
        && (key == "0" || !key.starts_with('0'))
        && key
            .parse::<u64>()
            .is_ok_and(|index| index < u64::from(u32::MAX))
}

/// Whether a key repeats, statically or as the same reference.
fn has_duplicate_keys(source: &Source, calls: &[&CallExpression]) -> bool {
    let mut seen: Vec<KeyName> = Vec::new();
    let mut dynamic: Vec<&Expression> = Vec::new();
    for call in calls {
        let Some(key) = argument(call, 1) else {
            continue;
        };
        if let Some(name) = expression_key_name(source, key) {
            if seen.contains(&name) {
                return true;
            }
            seen.push(name);
        } else {
            if dynamic.iter().any(|each| same_reference(each, key)) {
                return true;
            }
            dynamic.push(key);
        }
    }
    false
}

/// Whether merging would reorder the definitions: indices first, then strings, then symbols.
fn has_reordered_keys(source: &Source, calls: &[&CallExpression]) -> bool {
    let mut group = 0;
    let mut previous: Option<u64> = None;
    for call in calls {
        let key = argument(call, 1).and_then(|key| expression_key_name(source, key));
        let name = match key {
            Some(KeyName::Symbol(_)) => {
                group = 2;
                continue;
            }
            Some(KeyName::Text(name)) => name,
            None => String::new(),
        };
        if !is_array_index(&name) {
            if group > 1 {
                return true;
            }
            group = 1;
            continue;
        }
        let index = name.parse::<u64>().unwrap_or_default();
        if group > 0 || previous.is_some_and(|before| index < before) {
            return true;
        }
        previous = Some(index);
    }
    false
}

/// Whether a range holds a string or a template running over several lines.
fn has_multiline_string(source: &Source, span: Span) -> bool {
    source.semantic.nodes().iter().any(|node| {
        let at = node.kind().span();
        matches!(
            node.kind(),
            AstKind::StringLiteral(_) | AstKind::TemplateLiteral(_)
        ) && at.start >= span.start
            && at.end <= span.end
            && text(source, at).contains(['\n', '\r', '\u{2028}', '\u{2029}'])
    })
}

impl Rule for PreferObjectDefineProperties {
    fn name(&self) -> &'static str {
        "unicorn/prefer-object-define-properties"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ExpressionStatement(_) = node.kind() else {
            return;
        };
        let Some(list) = siblings(source, node.id()) else {
            return;
        };
        let Some(index) = list
            .iter()
            .position(|each| each.span() == node.kind().span())
        else {
            return;
        };
        let Some(first) = define_property(&list[index]) else {
            return;
        };
        let Some(target) = argument(first, 0) else {
            return;
        };
        let continues = |call: &CallExpression| {
            argument(call, 0).is_some_and(|each| same_reference(target, each))
        };
        if index > 0 && define_property(&list[index - 1]).is_some_and(continues) {
            return;
        }
        let calls: Vec<&CallExpression> = list[index..]
            .iter()
            .map_while(|statement| define_property(statement).filter(|call| continues(call)))
            .collect();
        if calls.len() < 2 {
            return;
        }
        let Expression::StaticMemberExpression(member) = &first.callee else {
            return;
        };
        let range = Span::new(
            list[index].span().start,
            list[index + calls.len() - 1].span().end,
        );
        let message =
            "Prefer `Object.defineProperties()` over multiple `Object.defineProperty()` calls."
                .to_owned();
        let fixable = !has_duplicate_keys(source, &calls)
            && !has_comment(source, range)
            && calls.iter().all(|call| {
                argument(call, 2)
                    .is_none_or(|descriptor| !has_multiline_string(source, descriptor.span()))
            })
            && !has_reordered_keys(source, &calls);
        if !fixable {
            findings.push(source.finding(self.name(), member.property.span.start, message));
            return;
        }
        let indent = indent_before(source, range.start);
        let unit = indent_unit(source.text);
        let properties: Vec<String> = calls
            .iter()
            .filter_map(|call| {
                let key = argument(call, 1)?;
                let descriptor = argument(call, 2)?;
                Some(format!(
                    "{indent}{unit}{}: {},",
                    key_text(source, key),
                    reindent(text(source, descriptor.span()), "", &unit)
                ))
            })
            .collect();
        let replacement = format!(
            "Object.defineProperties({}, {{\n{}\n{indent}}});",
            text(source, target.span()),
            properties.join("\n")
        );
        findings.push(source.fixed(
            self.name(),
            member.property.span.start,
            message,
            Fix {
                start: range.start,
                end: range.end,
                text: replacement,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferObjectDefineProperties;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &PreferObjectDefineProperties,
            "playground/a.ts",
            code,
        ))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferObjectDefineProperties, "playground/a.ts", code),
        )
    }

    const MESSAGE: &str =
        "Prefer `Object.defineProperties()` over multiple `Object.defineProperty()` calls.";

    #[test]
    fn merges_the_documented_adjacent_definitions() {
        let code = "Object.defineProperty(foo, 'bar', {\n\tvalue: 1,\n\twritable: true,\n});\nObject.defineProperty(foo, 'baz', {\n\tvalue: 2,\n\twritable: true,\n});\n";
        assert_eq!(check(code), [MESSAGE]);
        assert_eq!(
            fix(code),
            "Object.defineProperties(foo, {\n\tbar: {\n\t\tvalue: 1,\n\t\twritable: true,\n\t},\n\tbaz: {\n\t\tvalue: 2,\n\t\twritable: true,\n\t},\n});\n"
        );
        assert_eq!(
            fix(
                "function f() {\n  Object.defineProperty(o, 'a-b', d);\n  Object.defineProperty(o, 0, e);\n}"
            ),
            "function f() {\n  Object.defineProperty(o, 'a-b', d);\n  Object.defineProperty(o, 0, e);\n}"
        );
        assert_eq!(
            fix(
                "Object.defineProperty(o, 0, d);\nObject.defineProperty(o, key, e);\nObject.defineProperty(o, '__proto__', f);"
            ),
            "Object.defineProperties(o, {\n\t0: d,\n\t[key]: e,\n\t['__proto__']: f,\n});"
        );
    }

    #[test]
    fn reports_without_a_fix_when_merging_is_unsafe() {
        for code in [
            "Object.defineProperty(foo, 'bar', d);\nObject.defineProperty(foo, 0, e);",
            "Object.defineProperty(foo, 'a', d);\nObject.defineProperty(foo, 'a', e);",
            "Object.defineProperty(foo, k, d);\nObject.defineProperty(foo, k, e);",
            "Object.defineProperty(foo, 'a', d); // note\nObject.defineProperty(foo, 'b', e);",
            "Object.defineProperty(foo, 'a', {value: `x\ny`});\nObject.defineProperty(foo, 'b', e);",
        ] {
            assert_eq!(check(code), [MESSAGE], "{code}");
            assert_eq!(fix(code), code, "{code}");
        }
    }

    #[test]
    fn accepts_single_or_unrelated_definitions() {
        for code in [
            "Object.defineProperty(foo, 'bar', d);",
            "Object.defineProperty(foo, 'bar', d);\nObject.defineProperty(other, 'baz', e);",
            "Object.defineProperty(foo, 'bar', d);\nf();\nObject.defineProperty(foo, 'baz', e);",
            "Reflect.defineProperty(foo, 'bar', d);\nReflect.defineProperty(foo, 'baz', e);",
            "Object.defineProperty?.(foo, 'bar', d);\nObject.defineProperty?.(foo, 'baz', e);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
