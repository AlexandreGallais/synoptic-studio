//! `unicorn/no-invalid-dom-token`: `add()`, `remove()`, `toggle()` and `replace()` of a token
//! list (`classList`, `relList`, `sandbox`, `part`, `sizes`, `blocking`, `htmlFor`,
//! `controlsList`) throw on an empty token or one holding ASCII whitespace
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-invalid-dom-token.md`). Reported only:
//! `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::nodes::{is_not_dom_node, member_object, static_property_name, static_string};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{Value, static_value, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoInvalidDomToken;

/// The token lists' properties.
const TOKEN_LISTS: [&str; 8] = [
    "classList",
    "relList",
    "sandbox",
    "part",
    "sizes",
    "blocking",
    "htmlFor",
    "controlsList",
];

/// Infra's ASCII whitespace, which a token cannot hold.
fn is_ascii_whitespace(character: char) -> bool {
    matches!(character, '\t' | '\n' | '\u{c}' | '\r' | ' ')
}

/// How many arguments of a method are tokens; `None` for another method.
fn token_count(method: &str) -> Option<usize> {
    match method {
        "add" | "remove" => Some(usize::MAX),
        "toggle" => Some(1),
        "replace" => Some(2),
        _ => None,
    }
}

/// The message for a token argument, when it is empty or holds whitespace.
fn problem(source: &Source, argument: &Expression) -> Option<&'static str> {
    let token = unwrap(argument);
    let value = static_string(token).or_else(|| match static_value(source, token) {
        Some(Value::Text(text)) => Some(text),
        _ => None,
    });
    let whitespace = match (&value, token) {
        (Some(text), _) => text.chars().any(is_ascii_whitespace),
        (None, Expression::TemplateLiteral(template)) => template.quasis.iter().any(|quasi| {
            quasi
                .value
                .cooked
                .is_some_and(|cooked| cooked.chars().any(is_ascii_whitespace))
        }),
        _ => false,
    };
    if value.as_deref() == Some("") {
        Some("DOM tokens must not be empty.")
    } else if whitespace {
        Some("DOM tokens must not contain ASCII whitespace.")
    } else {
        None
    }
}

impl Rule for NoInvalidDomToken {
    fn name(&self) -> &'static str {
        "unicorn/no-invalid-dom-token"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let callee = unwrap(&call.callee);
        let Some(count) = static_property_name(callee)
            .as_deref()
            .and_then(token_count)
        else {
            return;
        };
        let Some(receiver) = member_object(callee).map(unwrap) else {
            return;
        };
        let is_token_list = static_property_name(receiver)
            .is_some_and(|name| TOKEN_LISTS.contains(&name.as_str()))
            && member_object(receiver).is_some_and(|owner| !is_not_dom_node(unwrap(owner)));
        if !is_token_list {
            return;
        }
        for argument in call.arguments.iter().take(count) {
            let Argument::SpreadElement(_) = argument else {
                if let Some(expression) = argument.as_expression()
                    && let Some(message) = problem(source, expression)
                {
                    findings.push(source.finding(
                        self.name(),
                        expression.span().start,
                        message.to_owned(),
                    ));
                }
                continue;
            };
            if count != usize::MAX {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidDomToken;
    use crate::testing::lint;

    /// The places and messages of the findings.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&NoInvalidDomToken, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn reports_empty_tokens_and_tokens_with_whitespace() {
        let empty = "DOM tokens must not be empty.".to_owned();
        let blank = "DOM tokens must not contain ASCII whitespace.".to_owned();
        // The documentation's five cases.
        assert_eq!(
            check(
                "element.classList.add('primary active');\nelement.classList.remove('');\nelement.classList.toggle(' primary', isActive);\nelement.classList.replace('primary', 'active selected');\nelement.classList.add(`primary ${suffix}`);"
            ),
            [
                (1, 23, blank.clone()),
                (2, 26, empty.clone()),
                (3, 26, blank.clone()),
                (4, 38, blank.clone()),
                (5, 23, blank.clone()),
            ]
        );
        // A constant, `?.`, a computed name, a wrapper, every argument of `add`.
        assert_eq!(
            check(
                "const both = 'a b';\nel?.classList.add(both);\nel.relList['remove']('x', '' as string);\nel.part.add(...rest, '\\t');"
            ),
            [(2, 19, blank.clone()), (3, 27, empty), (4, 22, blank)]
        );
    }

    #[test]
    fn leaves_valid_tokens_and_other_lists() {
        for code in [
            "element.classList.add('primary', 'active');",
            "element.classList.remove('primary');",
            "element.classList.toggle('primary', isActive);",
            "element.classList.toggle('primary', ' ');",
            "element.classList.replace('primary', 'active');",
            "element.classList.add('primary', suffix);",
            "element.classList.add('a\\u00A0b', 'a\\vb');",
            "element.classList.contains('a b');",
            "element.list.add('a b');",
            "[].classList.add('a b');",
            "element.classList.replace(...pair, 'a b');",
            "element.classList.add(1);",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
