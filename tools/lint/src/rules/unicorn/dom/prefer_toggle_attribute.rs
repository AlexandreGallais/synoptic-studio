//! `unicorn/prefer-toggle-attribute`: an `if` or a conditional expression that sets an
//! attribute on one side and removes it on the other is `toggleAttribute(…)`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-toggle-attribute.md`); `data-*`
//! attributes are left to the dataset rule. Autofixed only for a toggle on `hasAttribute(…)` that
//! sets an empty value, as a statement, no comment lost; the others are reported only, as
//! `ESLint` only suggests them.

use oxc_ast::AstKind;
use oxc_ast::ast::{ChainElement, Expression, Statement, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::nodes::{is_not_dom_node, static_string};
use crate::expressions::has_comment_in;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{
    has_optional_chain, method_call, object_text, same_reference, text,
};
use crate::source::Source;

/// The rule.
pub struct PreferToggleAttribute;

/// A call of `setAttribute`, `removeAttribute` or `hasAttribute`.
struct AttributeCall<'b, 'a> {
    /// The method's name.
    method: &'a str,
    /// The element.
    receiver: &'b Expression<'a>,
    /// The attribute's name.
    attribute: &'b Expression<'a>,
    /// Whether the value set is empty, or the call sets none.
    empty: bool,
    /// Whether the value set is a static string.
    static_value: bool,
    /// Whether the member is read with `?.`.
    optional: bool,
}

/// The attribute call an expression is, `?.` around it climbed (`getAttributeCall`).
fn attribute_call<'b, 'a>(expression: &'b Expression<'a>) -> Option<AttributeCall<'b, 'a>> {
    let call = match expression {
        Expression::CallExpression(call) => call,
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(call) => call,
            _ => return None,
        },
        _ => return None,
    };
    let method = method_call(call)?;
    let count = match method.name {
        "removeAttribute" | "hasAttribute" => 1,
        "setAttribute" => 2,
        _ => return None,
    };
    if call.optional
        || call.arguments.len() != count
        || call.arguments.iter().any(oxc_ast::ast::Argument::is_spread)
    {
        return None;
    }
    let attribute = call.arguments[0].as_expression()?;
    if static_string(attribute).is_some_and(|name| name.to_ascii_lowercase().starts_with("data-")) {
        return None;
    }
    let value = call
        .arguments
        .get(1)
        .and_then(|value| value.as_expression());
    Some(AttributeCall {
        method: method.name,
        receiver: method.object,
        attribute,
        empty: value.is_none_or(
            |value| matches!(value, Expression::StringLiteral(text) if text.value.is_empty()),
        ),
        static_value: value.is_none_or(|value| static_string(value).is_some()),
        optional: method.optional_member,
    })
}

/// The attribute call a branch of an `if` holds as its only statement.
fn statement_call<'b, 'a>(statement: &'b Statement<'a>) -> Option<AttributeCall<'b, 'a>> {
    let statement = match statement {
        Statement::BlockStatement(block) => match block.body.as_slice() {
            [only] => only,
            _ => return None,
        },
        other => other,
    };
    match statement {
        Statement::ExpressionStatement(expression) => attribute_call(&expression.expression),
        _ => None,
    }
}

/// Whether two calls act on the same attribute of the same element, the same way.
fn same_target(a: &AttributeCall, b: &AttributeCall) -> bool {
    a.optional == b.optional
        && same_reference(a.receiver, b.receiver)
        && same_reference(a.attribute, b.attribute)
}

/// Whether a test is `hasAttribute(…)` of the same attribute, negated or not; whether negated.
fn has_attribute_test(test: &Expression, set: &AttributeCall, negative: bool) -> Option<bool> {
    match test {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            has_attribute_test(&unary.argument, set, !negative)
        }
        _ => {
            let call = attribute_call(test)?;
            (call.method == "hasAttribute"
                && same_target(&call, set)
                && !has_optional_chain(call.receiver)
                && !is_not_dom_node(call.receiver))
            .then_some(negative)
        }
    }
}

impl Rule for PreferToggleAttribute {
    fn name(&self) -> &'static str {
        "unicorn/prefer-toggle-attribute"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (span, test, consequent, alternate, statement) = match node.kind() {
            AstKind::IfStatement(statement) => {
                let Some(alternate) = &statement.alternate else {
                    return;
                };
                (
                    statement.span,
                    &statement.test,
                    statement_call(&statement.consequent),
                    statement_call(alternate),
                    true,
                )
            }
            AstKind::ConditionalExpression(conditional) => (
                conditional.span,
                &conditional.test,
                attribute_call(&conditional.consequent),
                attribute_call(&conditional.alternate),
                matches!(
                    source.semantic.nodes().parent_kind(node.id()),
                    AstKind::ExpressionStatement(_)
                ),
            ),
            _ => return,
        };
        let (Some(consequent), Some(alternate)) = (consequent, alternate) else {
            return;
        };
        let pair = matches!(
            (consequent.method, alternate.method),
            ("setAttribute", "removeAttribute") | ("removeAttribute", "setAttribute")
        );
        if !pair
            || !same_target(&consequent, &alternate)
            || has_optional_chain(consequent.receiver)
            || has_optional_chain(alternate.receiver)
            || is_not_dom_node(consequent.receiver)
            || is_not_dom_node(alternate.receiver)
        {
            return;
        }
        let set_when_true = consequent.method == "setAttribute";
        let set = if set_when_true {
            &consequent
        } else {
            &alternate
        };
        let direct =
            has_attribute_test(test, set, false).is_some_and(|negative| negative == set_when_true);
        let message = "Prefer using `Element#toggleAttribute()` to toggle attributes.".to_owned();
        let receiver = set.receiver.span();
        let attribute = set.attribute.span();
        let commented = has_comment_in(source, Span::new(span.start, receiver.start))
            || has_comment_in(source, Span::new(receiver.end, attribute.start))
            || has_comment_in(source, Span::new(attribute.end, span.end));
        if !direct || !set.empty || !set.static_value || !statement || commented {
            findings.push(source.finding(self.name(), span.start, message));
            return;
        }
        let optional = if set.optional { "?" } else { "" };
        let semicolon = if matches!(node.kind(), AstKind::IfStatement(_)) {
            ";"
        } else {
            ""
        };
        let fix = Fix {
            start: span.start,
            end: span.end,
            text: format!(
                "{}{optional}.toggleAttribute({}){semicolon}",
                object_text(source, set.receiver),
                text(source, attribute)
            ),
        };
        findings.push(source.fixed(self.name(), span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferToggleAttribute;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferToggleAttribute, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn toggles_attributes() {
        // The documentation's four cases: the toggles on `hasAttribute` are fixed, those on a
        // condition reported only, as ESLint suggests them.
        assert_eq!(
            check("if (element.hasAttribute('hidden')) {\n\telement.removeAttribute('hidden');\n} else {\n\telement.setAttribute('hidden', '');\n}\nelement.hasAttribute('hidden')\n\t? element.removeAttribute('hidden')\n\t: element.setAttribute('hidden', '');\nif (condition) {\n\telement.setAttribute('hidden', '');\n} else {\n\telement.removeAttribute('hidden');\n}\ncondition\n\t? element.setAttribute('hidden', '')\n\t: element.removeAttribute('hidden');"),
            (
                vec![(1, 1), (6, 1), (9, 1), (14, 1)],
                "element.toggleAttribute('hidden');\nelement.toggleAttribute('hidden');\nif (condition) {\n\telement.setAttribute('hidden', '');\n} else {\n\telement.removeAttribute('hidden');\n}\ncondition\n\t? element.setAttribute('hidden', '')\n\t: element.removeAttribute('hidden');"
                    .to_owned()
            )
        );
        // `?.` on the calls, a negated test, a non-empty value, a dynamic one.
        assert_eq!(
            check("if (!e?.hasAttribute('a')) e?.setAttribute('a', ''); else e?.removeAttribute('a');\nif (e.hasAttribute('b')) e.removeAttribute('b'); else e.setAttribute('b', 'b');\nif (c) e.setAttribute('d', v); else e.removeAttribute('d');"),
            (
                vec![(1, 1), (2, 1), (3, 1)],
                "e?.toggleAttribute('a');\nif (e.hasAttribute('b')) e.removeAttribute('b'); else e.setAttribute('b', 'b');\nif (c) e.setAttribute('d', v); else e.removeAttribute('d');"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_other_branches() {
        for code in [
            "element.toggleAttribute('hidden');",
            "if (c) e.setAttribute('data-x', ''); else e.removeAttribute('data-x');",
            "if (c) e.setAttribute('a', ''); else f.removeAttribute('a');",
            "if (c) e.setAttribute('a', ''); else e.removeAttribute('b');",
            "if (c) e.setAttribute('a', '');",
            "if (c) { e.setAttribute('a', ''); x(); } else e.removeAttribute('a');",
            "if (c) a?.b.setAttribute('a', ''); else a?.b.removeAttribute('a');",
            "if (c) e.removeAttribute('a'); else e.removeAttribute('a');",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
        assert_eq!(
            check(
                "const x = e.hasAttribute('a') ? e.removeAttribute('a') : e.setAttribute('a', '');"
            )
            .0,
            [(1, 11)]
        );
    }
}
