//! `unicorn/no-invalid-style-set-property`: `style.setProperty(…)` takes a CSS property name,
//! never its camelCase form; `!important` goes in the priority argument, not the value; and the
//! priority is empty or `important` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-invalid-style-set-property.md`). Property names are looked up in the data of
//! `@eslint/css-tree`, as the rule does; the value is read with CSS's tokens, so that strings,
//! URLs, comments and nested blocks are skipped. Reported only: `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::css_properties::CSS_PROPERTIES;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{Value, method_call, static_value, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoInvalidStyleSetProperty;

/// The standard `WebKit` properties `css-tree`'s data lacks (the Compatibility Standard).
const WEBKIT_PROPERTIES: [&str; 53] = [
    "-webkit-align-content",
    "-webkit-align-items",
    "-webkit-align-self",
    "-webkit-animation",
    "-webkit-animation-delay",
    "-webkit-animation-direction",
    "-webkit-animation-duration",
    "-webkit-animation-fill-mode",
    "-webkit-animation-iteration-count",
    "-webkit-animation-name",
    "-webkit-animation-play-state",
    "-webkit-animation-timing-function",
    "-webkit-backface-visibility",
    "-webkit-background-origin",
    "-webkit-background-size",
    "-webkit-border-bottom-left-radius",
    "-webkit-border-bottom-right-radius",
    "-webkit-border-radius",
    "-webkit-border-top-left-radius",
    "-webkit-border-top-right-radius",
    "-webkit-box-align",
    "-webkit-box-flex",
    "-webkit-box-ordinal-group",
    "-webkit-box-orient",
    "-webkit-box-pack",
    "-webkit-box-shadow",
    "-webkit-box-sizing",
    "-webkit-filter",
    "-webkit-flex",
    "-webkit-flex-basis",
    "-webkit-flex-direction",
    "-webkit-flex-flow",
    "-webkit-flex-grow",
    "-webkit-flex-shrink",
    "-webkit-flex-wrap",
    "-webkit-justify-content",
    "-webkit-mask-box-image-outset",
    "-webkit-mask-box-image-repeat",
    "-webkit-mask-box-image-slice",
    "-webkit-mask-box-image-source",
    "-webkit-mask-box-image-width",
    "-webkit-order",
    "-webkit-perspective",
    "-webkit-perspective-origin",
    "-webkit-text-size-adjust",
    "-webkit-transform",
    "-webkit-transform-origin",
    "-webkit-transform-style",
    "-webkit-transition",
    "-webkit-transition-delay",
    "-webkit-transition-duration",
    "-webkit-transition-property",
    "-webkit-transition-timing-function",
];

/// Whether `css-tree` knows a property.
fn is_property(name: &str) -> bool {
    CSS_PROPERTIES.binary_search(&name).is_ok()
}

/// The CSS name of a camelCase property name that is not one itself (`getCssPropertyName`).
fn css_name(property: &str) -> Option<String> {
    let mut characters = property.chars();
    let plain = characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && characters.all(|each| each.is_ascii_alphanumeric());
    if !plain || is_property(&property.to_ascii_lowercase()) {
        return None;
    }
    let mut replacement = if property == "cssFloat" {
        "float".to_owned()
    } else {
        property
            .chars()
            .map(|each| {
                if each.is_ascii_uppercase() {
                    format!("-{}", each.to_ascii_lowercase())
                } else {
                    each.to_string()
                }
            })
            .collect()
    };
    if replacement.starts_with("webkit-") {
        replacement.insert(0, '-');
    }
    (is_property(&replacement) || WEBKIT_PROPERTIES.contains(&replacement.as_str()))
        .then_some(replacement)
}

/// Whether a character may continue a CSS name.
fn is_name_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '-' | '_') || !character.is_ascii()
}

/// Whether a CSS name may start at a position: a letter, `_`, a non-ASCII character, an
/// escape, or `-` followed by one of those or by `-`.
fn starts_name(characters: &[char], at: usize) -> bool {
    let starts = |index: usize| {
        characters
            .get(index)
            .is_some_and(|each| each.is_ascii_alphabetic() || *each == '_' || !each.is_ascii())
            || is_escape(characters, index)
    };
    match characters.get(at) {
        Some('-') => characters.get(at + 1) == Some(&'-') || starts(at + 1),
        Some(_) => starts(at),
        None => false,
    }
}

/// Whether a valid escape starts at a position: `\` not followed by a line break.
fn is_escape(characters: &[char], at: usize) -> bool {
    characters.get(at) == Some(&'\\')
        && characters
            .get(at + 1)
            .is_some_and(|next| !matches!(next, '\n' | '\r' | '\u{c}'))
}

/// Reads a CSS name from a position, its escapes decoded; the position after it.
fn read_name(characters: &[char], mut at: usize) -> (String, usize) {
    let mut name = String::new();
    loop {
        if is_escape(characters, at) {
            at += 1;
            let digits: String = characters[at..]
                .iter()
                .take(6)
                .take_while(|each| each.is_ascii_hexdigit())
                .collect();
            if digits.is_empty() {
                name.push(characters[at]);
                at += 1;
            } else {
                at += digits.len();
                let code = u32::from_str_radix(&digits, 16).unwrap_or(0xFFFD);
                name.push(
                    char::from_u32(code)
                        .filter(|_| code != 0)
                        .unwrap_or('\u{FFFD}'),
                );
                if characters.get(at).is_some_and(|each| each.is_whitespace()) {
                    at += 1;
                }
            }
        } else if characters
            .get(at)
            .is_some_and(|each| is_name_character(*each))
        {
            name.push(characters[at]);
            at += 1;
        } else {
            return (name, at);
        }
    }
}

/// Skips a string from its opening quote; the position after it.
fn skip_string(characters: &[char], mut at: usize) -> usize {
    let quote = characters[at];
    at += 1;
    while let Some(each) = characters.get(at) {
        match each {
            '\\' => at += 2,
            '\n' => return at,
            _ if *each == quote => return at + 1,
            _ => at += 1,
        }
    }
    at
}

/// Skips an unquoted `url(…)` from after its `(`; the position after its `)`.
fn skip_url(characters: &[char], mut at: usize) -> usize {
    while let Some(each) = characters.get(at) {
        match each {
            '\\' => at += 2,
            ')' => return at + 1,
            _ => at += 1,
        }
    }
    at
}

/// Whether a CSS value holds `!important` outside every block, function and string
/// (`getImportanceMarkers`).
fn has_importance(value: &str) -> bool {
    let characters: Vec<char> = value.chars().collect();
    let mut at = 0;
    let mut depth = 0_usize;
    let mut bang = false;
    while let Some(&each) = characters.get(at) {
        if each == '/' && characters.get(at + 1) == Some(&'*') {
            at = (at + 2..characters.len().saturating_sub(1))
                .find(|index| characters[*index] == '*' && characters[*index + 1] == '/')
                .map_or(characters.len(), |end| end + 2);
            continue;
        }
        if each.is_whitespace() {
            at += 1;
            continue;
        }
        let mut is_bang = false;
        if matches!(each, '"' | '\'') {
            at = skip_string(&characters, at);
        } else if starts_name(&characters, at) {
            let (name, after) = read_name(&characters, at);
            at = after;
            if characters.get(at) == Some(&'(') {
                at += 1;
                let next = characters[at..].iter().find(|each| !each.is_whitespace());
                if name.eq_ignore_ascii_case("url") && !matches!(next, Some('"' | '\'')) {
                    at = skip_url(&characters, at);
                } else {
                    depth += 1;
                }
            } else if depth == 0 && bang && name.eq_ignore_ascii_case("important") {
                return true;
            }
        } else if each.is_ascii_digit() {
            at += characters[at..]
                .iter()
                .take_while(|each| each.is_ascii_digit() || **each == '.')
                .count();
            if starts_name(&characters, at) {
                at = read_name(&characters, at).1;
            }
        } else {
            match each {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth = depth.saturating_sub(1),
                '!' => is_bang = depth == 0,
                _ => {}
            }
            at += 1;
        }
        bang = is_bang;
    }
    false
}

/// Whether a priority is valid: absent, empty or `important` in any case.
fn is_valid_priority(priority: &Value) -> bool {
    match priority {
        Value::Null | Value::Undefined => true,
        Value::Text(text) => text.is_empty() || text.eq_ignore_ascii_case("important"),
        _ => false,
    }
}

/// Whether a receiver is a style declaration: `style` or `….style`.
fn is_style(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::Identifier(name) => name.name == "style",
        Expression::StaticMemberExpression(member) => member.property.name == "style",
        _ => false,
    }
}

impl Rule for NoInvalidStyleSetProperty {
    fn name(&self) -> &'static str {
        "unicorn/no-invalid-style-set-property"
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
        if method.name != "setProperty"
            || call.arguments.len() < 2
            || call.arguments.iter().take(3).any(Argument::is_spread)
            || !is_style(method.object)
        {
            return;
        }
        let arguments: Vec<&Expression> = call
            .arguments
            .iter()
            .filter_map(Argument::as_expression)
            .collect();
        let (property, value) = (arguments[0], arguments[1]);
        let priority = arguments.get(2).copied();
        if let Some(Value::Text(name)) = static_value(source, property)
            && let Some(replacement) = css_name(&name)
        {
            findings.push(source.finding(
                self.name(),
                property.span().start,
                format!("Use the CSS property name `{replacement}` instead of `{name}` in `setProperty()`."),
            ));
        }
        let value_result = static_value(source, value);
        if let Some(Value::Text(text)) = &value_result
            && has_importance(text)
        {
            findings.push(source.finding(
                self.name(),
                value.span().start,
                "Pass `important` as the priority argument instead of including `!important` in the value.".to_owned(),
            ));
        }
        let nonempty = matches!(&value_result, Some(value) if !matches!(value, Value::Null)
            && *value != Value::Text(String::new()));
        let Some(priority) = priority else {
            return;
        };
        if nonempty
            && let Some(result) = static_value(source, priority)
            && !is_valid_priority(&result)
        {
            findings.push(source.finding(
                self.name(),
                priority.span().start,
                "The `setProperty()` priority must be an empty string or `important` (case-insensitive).".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidStyleSetProperty;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoInvalidStyleSetProperty, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_camel_case_names_importance_in_values_and_bad_priorities() {
        // The documentation's four cases.
        let findings = lint(
            &NoInvalidStyleSetProperty,
            "playground/a/b.ts",
            "element.style.setProperty('backgroundColor', 'red');\nelement.style.setProperty('color', 'red !important');\nelement.style.setProperty('color', 'red', '!important');\nelement.style.setProperty('color', 'red', ' important ');",
        );
        assert_eq!(
            findings
                .iter()
                .map(|each| (each.line, each.column, each.message.as_str()))
                .collect::<Vec<_>>(),
            [
                (
                    1,
                    27,
                    "Use the CSS property name `background-color` instead of `backgroundColor` in `setProperty()`."
                ),
                (
                    2,
                    36,
                    "Pass `important` as the priority argument instead of including `!important` in the value."
                ),
                (
                    3,
                    43,
                    "The `setProperty()` priority must be an empty string or `important` (case-insensitive)."
                ),
                (
                    4,
                    43,
                    "The `setProperty()` priority must be an empty string or `important` (case-insensitive)."
                ),
            ]
        );
        // `cssFloat`, a WebKit name, a vendor's, comments, escapes and casing.
        assert_eq!(
            check(
                "style.setProperty('cssFloat', 'left');\nstyle.setProperty('webkitTransform', 'none');\nstyle.setProperty('MozAppearance', 'none');\nstyle.setProperty('color', 'red !/* x */IMPORTANT');\nstyle.setProperty('color', 'red !imp\\\\6f rtant');\nstyle.setProperty('color', 'red', false);\nstyle.setProperty('color', undefined, 'x');"
            ),
            [
                (1, 19),
                (2, 19),
                (3, 19),
                (4, 28),
                (5, 28),
                (6, 35),
                (7, 39)
            ]
        );
    }

    #[test]
    fn leaves_valid_calls_and_nested_importance() {
        for code in [
            "element.style.setProperty('background-color', 'red');",
            "element.style.setProperty('color', 'red', 'important');",
            "element.style.setProperty('color', 'red', 'IMPORTANT');",
            "style.setProperty('content', '\"!important\"');",
            "style.setProperty('background-image', 'url(\"!important\")');",
            "style.setProperty('background-image', 'url(!important)');",
            "style.setProperty('--tokens', 'var(--other, !important)');",
            "style.setProperty('color', '', '!important');",
            "style.setProperty('color', null, false);",
            "style.setProperty('color', value, priority);",
            "style.setProperty('--brandColor', 'red');",
            "style.setProperty('color', 'red', null);",
            "style.setProperty('fooBar', 'red');",
            "style.setProperty('Color', 'red');",
            "other.setProperty('backgroundColor', 'red');",
            "style['setProperty']('backgroundColor', 'red');",
            "style.setProperty(...args, 'red !important');",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
