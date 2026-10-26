//! `@angular-eslint/template/valid-aria`: an `aria-*` attribute is an ARIA property, of a value
//! of its type (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/valid-aria.ts`): on an
//! element of the DOM, its name in lower or upper case, a plain attribute or an input named
//! `aria-` and more is reported over itself when `aria-query` 5.3.2 has no such property, or when
//! its value, a text or a bound literal, is not of the property's type; any other binding is not
//! checked. The removal of an unknown attribute is a suggestion, not carried. The checks of a
//! value follow the plugin's JavaScript (`isInteger`, `isNumeric`, `JSON.parse` of a boolean
//! token), its conversions written as JavaScript makes them.

use std::cmp::Ordering;

use super::a11y::AttributeValue;
use super::aria::{AriaProperty, PropertyKind, Token, property};
use super::dom::is_dom_name;
use super::r3::{Literal, Ng, of_bound};
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::javascript::{is_js_space, parse_int, to_number};
use crate::template::bindings::{Value, read_attribute};

/// The rule.
pub struct ValidAria;

/// `parseInt(String(number))`: the integer part, or the first digit of the mantissa when
/// JavaScript writes the number with an exponent (from `1e21` up, and below `1e-6`).
fn parse_int_of_number(number: f64) -> f64 {
    if !number.is_finite() {
        return f64::NAN;
    }
    let size = number.abs();
    if size != 0.0 && !(1e-6..1e21).contains(&size) {
        let written = format!("{number:e}");
        return parse_int(&written);
    }
    number.trunc()
}

/// `Number(value)`.
fn number_of(value: &AttributeValue) -> f64 {
    match value {
        AttributeValue::Text(text) => to_number(text),
        AttributeValue::Number(number) => *number,
        AttributeValue::Bool(value) => f64::from(u8::from(*value)),
        AttributeValue::Null => 0.0,
        _ => f64::NAN,
    }
}

/// `number == value`, JavaScript's loose equality of a number and a value.
fn loosely_equal(number: f64, value: &AttributeValue) -> bool {
    match value {
        AttributeValue::Null | AttributeValue::Undefined => false,
        // JavaScript's `==` of two numbers: `NaN` equal to nothing, `-0` to `0`.
        other => number.partial_cmp(&number_of(other)) == Some(Ordering::Equal),
    }
}

/// `isInteger`.
fn is_integer(value: &AttributeValue) -> bool {
    let leading = match value {
        AttributeValue::Text(text) => parse_int(text),
        // `String` of a finite number reads back as a number.
        AttributeValue::Number(number) if number.is_finite() => 0.0,
        _ => f64::NAN,
    };
    loosely_equal(parse_int_of_number(number_of(value)), value) && !leading.is_nan()
}

/// `isNumeric`.
fn is_numeric(value: &AttributeValue) -> bool {
    match value {
        AttributeValue::Text(text) => crate::javascript::is_numeric(text),
        AttributeValue::Number(number) => number.is_finite(),
        _ => false,
    }
}

/// Whether a value is a boolean, or the text `true` or `false`.
fn boolean_like(value: &AttributeValue) -> Option<bool> {
    match value {
        AttributeValue::Bool(value) => Some(*value),
        AttributeValue::Text(text) if text == "true" => Some(true),
        AttributeValue::Text(text) if text == "false" => Some(false),
        _ => None,
    }
}

/// Whether a token is a property's value.
fn is_value(property: &AriaProperty, token: Token) -> bool {
    property.values.contains(&token)
}

/// Whether a value is of a property's type (`isValidAriaPropertyValue`).
fn is_valid_value(property: &AriaProperty, value: &AttributeValue) -> bool {
    let nil = matches!(value, AttributeValue::Null | AttributeValue::Undefined);
    if property.allow_undefined && nil {
        return true;
    }
    match property.kind {
        PropertyKind::Boolean => boolean_like(value).is_some(),
        PropertyKind::Tristate => {
            value.text() == Some("mixed") || boolean_like(value).is_some() || nil
        }
        PropertyKind::Id | PropertyKind::IdList => true,
        PropertyKind::Integer => is_integer(value),
        PropertyKind::Number => is_numeric(value),
        PropertyKind::String => value.text().is_some(),
        PropertyKind::Token => match (boolean_like(value), value) {
            (Some(parsed), _) => is_value(property, Token::Bool(parsed)),
            (None, AttributeValue::Text(text)) => has_text(property, text),
            _ => false,
        },
        PropertyKind::TokenList => value
            .text()
            .is_some_and(|text| tokens(text).iter().all(|each| has_text(property, each))),
    }
}

/// Whether a text is one of a property's values.
fn has_text(property: &AriaProperty, text: &str) -> bool {
    property
        .values
        .iter()
        .any(|each| matches!(each, Token::Text(value) if *value == text))
}

/// The tokens of a list, as `text.trim().split(/\s+/)` gives them: one empty token for a list
/// of whitespace only.
fn tokens(text: &str) -> Vec<&str> {
    let trimmed = text.trim_matches(is_js_space);
    if trimmed.is_empty() {
        return vec![""];
    }
    trimmed
        .split(is_js_space)
        .filter(|each| !each.is_empty())
        .collect()
}

impl TemplateRule for ValidAria {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/valid-aria"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "validAria",
                "The `{{attribute}}` is an invalid ARIA attribute",
            ),
            (
                "validAriaValue",
                "The `{{attribute}}` has an invalid value. Check the valid values at https://raw.githack.com/w3c/aria/stable/#roles",
            ),
            (
                "suggestRemoveInvalidAria",
                "Remove attribute `{{attribute}}`",
            ),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !is_plain_element(element) || !is_dom_name(&element.name) {
                continue;
            }
            for each in bound3(element) {
                let aria = each
                    .name
                    .strip_prefix("aria-")
                    .is_some_and(|rest| !rest.is_empty());
                if each.role == Role::Output || !aria {
                    continue;
                }
                let data = vec![("attribute", each.name.clone())];
                let span = each.attribute.span;
                let Some(found) = property(&each.name) else {
                    reports.push(Report {
                        data,
                        ..Report::new(span, "validAria")
                    });
                    continue;
                };
                let value = if each.role == Role::Text {
                    AttributeValue::Text(each.attribute.value.clone())
                } else {
                    let read = read_attribute(template.allocator, &element.name, each.attribute);
                    let Ok(Value::Binding(bound)) = read.map(|read| read.value) else {
                        continue;
                    };
                    match of_bound(&bound) {
                        Ng::Literal(Literal::Text(text)) => AttributeValue::Text(text),
                        Ng::Literal(Literal::Number(number)) => AttributeValue::Number(number),
                        Ng::Literal(Literal::Bool(value)) => AttributeValue::Bool(value),
                        Ng::Literal(Literal::Null) => AttributeValue::Null,
                        // `undefined`, and the `value` an array or an object literal lacks.
                        Ng::Literal(Literal::Undefined) | Ng::Array(_) | Ng::Map(..) => {
                            AttributeValue::Undefined
                        }
                        _ => continue,
                    }
                };
                if !is_valid_value(found, &value) {
                    reports.push(Report {
                        data,
                        ..Report::new(span, "validAriaValue")
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AttributeValue, ValidAria, is_integer, is_numeric, tokens};
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("valid-aria.txt", &[], &|_| Box::new(ValidAria));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 23 valid and 4 invalid cases, all run; 4 suggestions, not carried.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (23, 4, 4));
    }

    #[test]
    fn reads_numbers_as_javascript_does() {
        // Our own cases, after the ECMAScript conversions.
        let text = |text: &str| AttributeValue::Text(text.to_owned());
        // "-1": parseInt(Number("-1")) is -1, loosely equal to "-1".
        assert!(is_integer(&text("-1")));
        // "1.5": parseInt(1.5) is 1, not 1.5.
        assert!(!is_integer(&text("1.5")));
        // "1e3": Number gives 1000, parseInt(1000) is 1000; parseInt("1e3") is 1, a number.
        assert!(is_integer(&text("1e3")));
        // 1e21 is written "1e+21", which parseInt reads as 1.
        assert!(!is_integer(&AttributeValue::Number(1e21)));
        assert!(is_integer(&AttributeValue::Number(2.0)));
        // true: parseInt(Number(true)) is 1 == true, but parseInt("true") is NaN.
        assert!(!is_integer(&AttributeValue::Bool(true)));
        assert!(!is_integer(&AttributeValue::Undefined));
        assert!(is_numeric(&text(" 2.5 ")) && !is_numeric(&text("2px")));
        assert!(!is_numeric(&text("Infinity")) && !is_numeric(&AttributeValue::Null));
        assert_eq!(tokens("  a   b "), ["a", "b"]);
        assert_eq!(tokens("   "), [""]);
    }

    #[test]
    fn reads_an_empty_number_as_no_number() {
        let places = |text: &str| {
            crate::template::lint_with("a.component.html", text, &[&ValidAria])
                .iter()
                .map(|finding| (finding.line, finding.column))
                .collect::<Vec<_>>()
        };

        // Our own cases: `parseFloat("")` is `NaN`, though `Number("")` is 0, so an empty
        // `aria-valuenow` is no number, reported at it after `<p `; `"1"` is one.
        assert_eq!(places("<p aria-valuenow=\"\">a</p>"), [(1, 4)]);
        assert_eq!(places("<p aria-valuenow=\" \">a</p>"), [(1, 4)]);
        assert_eq!(places("<p aria-valuenow=\"1\">a</p>"), []);
    }
}
