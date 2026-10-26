//! The helpers of the accessibility rules, as `@angular-eslint/eslint-plugin-template` 22.6.0
//! writes them in `src/utils/` (REF-ANGULAR-ESLINT-TEMPLATE-SOURCE): an attribute's value
//! (`get-attribute-value.ts`), whether an element is hidden from a screen reader
//! (`is-hidden-from-screen-reader.ts`), has a presentation role, is content editable or disabled,
//! holds a control (`is-child-node-of.ts`), is interactive by its own semantics
//! (`is-interactive-element/`, `attributes-comparator.ts`), or has the role it is given by its
//! semantics (`is-semantic-role-element.ts`), on the data of `aria-query` 5.3.2 (`aria`) and
//! `axobject-query` 4.1.0 (`ax_objects`).

use std::sync::LazyLock;

use oxc_allocator::Allocator;

use super::aria::{Concept, ConceptAttribute, ROLES, element_roles};
use super::ax_objects::{AX_OBJECTS, ax_object, element_ax_objects};
use super::dom::is_dom_element;
use super::r3::{Literal, Ng, of_bound};
use super::{Bound3, Role, bound3, is_plain_element};
use crate::template::bindings::{AttributeKind, Value, attribute_kind, read_attribute};
use crate::template::tree::{Element, Node};

/// An attribute's value as `getAttributeValue` gives it: a plain attribute's text, a bound
/// literal's value, an array or an object literal, `PROPERTY_READ` for any other binding, `null`
/// when the element has no such attribute.
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    /// `null`: no such attribute, or a bound `null`.
    Null,
    /// A bound `undefined`.
    Undefined,
    /// A plain attribute's text, or a bound string.
    Text(String),
    /// A bound number.
    Number(f64),
    /// A bound boolean.
    Bool(bool),
    /// A bound array literal (its expressions).
    Array,
    /// A bound object literal (a `Map` of its keys to their expressions, never equal to a text).
    Map,
    /// Any other binding (`PROPERTY_READ`).
    PropertyRead,
}

impl AttributeValue {
    /// The text when the value is a string (`typeof value === 'string'`).
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    /// Whether `String(value)` is `true`, case kept or not.
    #[must_use]
    pub fn is_true(&self, ignore_case: bool) -> bool {
        match self {
            Self::Text(text) if ignore_case => text.eq_ignore_ascii_case("true"),
            Self::Text(text) => text == "true",
            Self::Bool(value) => *value,
            // `String` of `null`, `undefined`, a number, an array of expressions, a `Map` or a
            // symbol is never `true`.
            _ => false,
        }
    }
}

/// The value of a bound literal as Angular's parser reads it.
fn literal(value: &Literal) -> AttributeValue {
    match value {
        Literal::Text(text) => AttributeValue::Text(text.clone()),
        Literal::Number(number) => AttributeValue::Number(*number),
        Literal::Bool(value) => AttributeValue::Bool(*value),
        Literal::Null => AttributeValue::Null,
        Literal::Undefined => AttributeValue::Undefined,
    }
}

/// The value of an element's input.
fn input_value(allocator: &Allocator, element: &Element, input: &Bound3) -> AttributeValue {
    let Ok(read) = read_attribute(allocator, &element.name, input.attribute) else {
        return AttributeValue::PropertyRead;
    };
    let Value::Binding(bound) = read.value else {
        // An interpolation.
        return AttributeValue::PropertyRead;
    };
    match of_bound(&bound) {
        Ng::Literal(value) => literal(&value),
        Ng::Array(_) => AttributeValue::Array,
        Ng::Map(..) => AttributeValue::Map,
        _ => AttributeValue::PropertyRead,
    }
}

/// The plain attributes of an element, then its inputs, as `[...attributes, ...inputs]`.
#[must_use]
pub fn attributes_and_inputs(element: &Element) -> Vec<Bound3<'_>> {
    let all = bound3(element);
    let (inputs, texts): (Vec<_>, Vec<_>) = all
        .into_iter()
        .filter(|each| each.role != Role::Output)
        .partition(|each| each.role == Role::Input);
    texts.into_iter().chain(inputs).collect()
}

/// The value of the first plain attribute or input of an element whose original name is the
/// one asked (`getAttributeValue`).
#[must_use]
pub fn attribute_value(allocator: &Allocator, element: &Element, name: &str) -> AttributeValue {
    match attributes_and_inputs(element)
        .iter()
        .find(|each| each.original == name)
    {
        None => AttributeValue::Null,
        Some(each) if each.role == Role::Text => AttributeValue::Text(each.attribute.value.clone()),
        Some(each) => input_value(allocator, element, each),
    }
}

/// Whether an attribute's value is the empty string or `true` (`isHtmlTruthy`).
fn is_html_truthy(allocator: &Allocator, element: &Element, name: &str) -> bool {
    let value = attribute_value(allocator, element, name);
    value.text() == Some("") || value.is_true(false)
}

/// Whether a character is whitespace for JavaScript's `\s`.
#[must_use]
pub const fn is_js_space(character: char) -> bool {
    matches!(
        character,
        '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// A text as JavaScript's `Number(text)` reads it: its whitespace trimmed, empty as 0, a decimal
/// literal, `Infinity` with its sign, or an unsigned `0x`, `0o` or `0b` integer; `NaN` otherwise.
#[must_use]
pub fn to_number(text: &str) -> f64 {
    let trimmed = text.trim_matches(is_js_space);
    if trimmed.is_empty() {
        return 0.0;
    }
    let radix = match trimmed.get(..2) {
        Some("0x" | "0X") => 16,
        Some("0o" | "0O") => 8,
        Some("0b" | "0B") => 2,
        _ => 10,
    };
    if radix != 10 {
        let digits = &trimmed[2..];
        if digits.is_empty() || !digits.chars().all(|each| each.is_digit(radix)) {
            return f64::NAN;
        }
        return digits.chars().fold(0.0, |value, each| {
            value * f64::from(radix) + f64::from(each.to_digit(radix).unwrap_or(0))
        });
    }
    let (sign, body) = match trimmed.as_bytes().first() {
        Some(b'-') => (-1.0, &trimmed[1..]),
        Some(b'+') => (1.0, &trimmed[1..]),
        _ => (1.0, trimmed),
    };
    if body == "Infinity" {
        return sign * f64::INFINITY;
    }
    // Rust reads `inf` and `nan` too, which JavaScript does not: only digits, a point and an
    // exponent are let through, with a digit before the exponent.
    let mantissa = body.split(['e', 'E']).next().unwrap_or_default();
    let decimal = body
        .chars()
        .all(|each| each.is_ascii_digit() || matches!(each, '.' | 'e' | 'E' | '+' | '-'))
        && mantissa.chars().any(|each| each.is_ascii_digit())
        && !body.starts_with(['+', '-']);
    if !decimal {
        return f64::NAN;
    }
    body.parse::<f64>().map_or(f64::NAN, |value| sign * value)
}

/// Whether an element itself, its ancestors aside, is hidden from a screen reader: hidden by its
/// static styles (`display: none`, `visibility: hidden`) or by bound literal styles, by a truthy
/// `aria-hidden` or `hidden`, or an `<input type="hidden">`. The plugin's `ngStyle` check
/// compares the object's values, which are expressions, to strings: never equal, left out.
fn is_self_hidden(allocator: &Allocator, element: &Element) -> bool {
    let value = |name: &str| attribute_value(allocator, element, name);
    let static_style = value("style").text().is_some_and(|style| {
        let raw: String = style
            .chars()
            .filter(|each| !is_js_space(*each) && *each != '\'' && *each != '"')
            .collect();
        raw.contains("display:none") || raw.contains("visibility:hidden")
    });
    let dynamic_style = value("style.display").text() == Some("none")
        || value("style.display.none").is_true(false)
        || value("style.visibility").text() == Some("hidden")
        || value("style.visibility.hidden").is_true(false);
    if static_style
        || dynamic_style
        || is_html_truthy(allocator, element, "aria-hidden")
        || is_html_truthy(allocator, element, "hidden")
    {
        return true;
    }
    element.name.eq_ignore_ascii_case("input")
        && value("type")
            .text()
            .is_some_and(|kind| kind.eq_ignore_ascii_case("hidden"))
}

/// Whether an element's content cannot be read by a screen reader: it, or an element above it,
/// is hidden (`isHiddenFromScreenReader`).
#[must_use]
pub fn is_hidden_from_screen_reader(
    allocator: &Allocator,
    element: &Element,
    ancestors: &[&Node],
) -> bool {
    let hidden_above = ancestors.iter().any(|node| match node {
        Node::Element(above) if is_plain_element(above) => is_self_hidden(allocator, above),
        _ => false,
    });
    hidden_above || is_self_hidden(allocator, element)
}

/// Whether an element's role is `none` or `presentation` (`isPresentationRole`).
#[must_use]
pub fn is_presentation_role(allocator: &Allocator, element: &Element) -> bool {
    attribute_value(allocator, element, "role")
        .text()
        .is_some_and(|role| role == "none" || role == "presentation")
}

/// Whether an element is content editable: a `contenteditable` empty or `true`, any case
/// (`isContentEditable`).
#[must_use]
pub fn is_content_editable(allocator: &Allocator, element: &Element) -> bool {
    let present = attributes_and_inputs(element)
        .iter()
        .any(|each| each.original == "contenteditable");
    let value = attribute_value(allocator, element, "contenteditable");
    present && (value.text() == Some("") || value.is_true(true))
}

/// Whether an element is disabled: a `disabled` of any value but a bound `undefined`, or an
/// `aria-disabled` of `true`, any case (`isDisabledElement`).
#[must_use]
pub fn is_disabled_element(allocator: &Allocator, element: &Element) -> bool {
    let present = attributes_and_inputs(element)
        .iter()
        .any(|each| each.original == "disabled");
    if present && attribute_value(allocator, element, "disabled") != AttributeValue::Undefined {
        return true;
    }
    attribute_value(allocator, element, "aria-disabled").is_true(true)
}

/// Whether an element is Angular's element in its parent's children: neither a `<ng-template>`,
/// nor a `<ng-content>`, nor an element under a structural directive, which Angular wraps in a
/// template.
#[must_use]
pub fn is_child_element(element: &Element) -> bool {
    is_plain_element(element)
        && !element
            .attrs
            .iter()
            .any(|each| attribute_kind(&each.name).0 == AttributeKind::Template)
}

/// Whether an element holds an element of a name, through its child elements only
/// (`isChildNodeOf`): a template, a block or a structural directive stops the search.
#[must_use]
pub fn is_child_node_of(element: &Element, name: &str) -> bool {
    element.children.iter().any(|child| match child {
        Node::Element(child) if is_child_element(child) => {
            child.name == name || is_child_node_of(child, name)
        }
        _ => false,
    })
}

/// Whether an element has every attribute of a concept, by name, and by value when the concept
/// gives one; an `<a>` with a `routerLink` has them all (`attributesComparator`).
#[must_use]
pub fn attributes_comparator(
    allocator: &Allocator,
    attributes: &[ConceptAttribute],
    element: &Element,
) -> bool {
    let all = attributes_and_inputs(element);
    attributes.iter().all(|wanted| {
        all.iter().any(|each| {
            if element.name == "a" && each.name == "routerLink" {
                return true;
            }
            if wanted.name != each.name {
                return false;
            }
            wanted.value.is_none_or(|value| {
                value.is_empty()
                    || attribute_value(allocator, element, wanted.name).text() == Some(value)
            })
        })
    })
}

/// Whether a role descends from `widget` by one of its chains of superclasses.
fn is_widget(super_class: &[&[&str]]) -> bool {
    super_class.iter().any(|chain| chain.contains(&"widget"))
}

/// The interactive roles: the concrete roles descending from `widget` but `progressbar`, and
/// `toolbar`.
fn interactive_roles() -> Vec<&'static str> {
    ROLES
        .iter()
        .filter(|role| {
            !role.is_abstract && role.name != "progressbar" && is_widget(role.super_class)
        })
        .map(|role| role.name)
        .chain(["toolbar"])
        .collect()
}

/// The non-interactive roles: the concrete roles not descending from `widget` but `toolbar`, and
/// `progressbar` (`getNonInteractiveRoles`).
static NON_INTERACTIVE_ROLES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ROLES
        .iter()
        .filter(|role| !role.is_abstract && role.name != "toolbar" && !is_widget(role.super_class))
        .map(|role| role.name)
        .chain(["progressbar"])
        .collect()
});

/// The HTML concepts of `elementRoles` whose every role is among some.
fn schemas_of(roles: &[&str]) -> Vec<&'static Concept> {
    element_roles()
        .into_iter()
        .filter(|(_, each)| each.iter().all(|role| roles.contains(role)))
        .map(|(concept, _)| concept)
        .collect()
}

/// The concepts of interactive elements (`getInteractiveElementRoleSchemas`).
static INTERACTIVE_SCHEMAS: LazyLock<Vec<&'static Concept>> =
    LazyLock::new(|| schemas_of(&interactive_roles()));

/// The concepts of non-interactive elements (`getNonInteractiveElementRoleSchemas`).
static NON_INTERACTIVE_SCHEMAS: LazyLock<Vec<&'static Concept>> =
    LazyLock::new(|| schemas_of(&NON_INTERACTIVE_ROLES));

/// A concept of the accessibility tree, the attributes it is matched on.
type AxSchema = (Option<&'static str>, &'static [ConceptAttribute]);

/// `elementAXObjects`, built once.
static ELEMENT_AX_OBJECTS: LazyLock<Vec<(&'static Concept, Vec<&'static str>)>> =
    LazyLock::new(element_ax_objects);

/// The concepts of `elementAXObjects` whose every object is a widget, a `<summary>`'s without
/// its attributes (`getInteractiveElementAXObjectSchemas`).
static INTERACTIVE_AX_SCHEMAS: LazyLock<Vec<AxSchema>> = LazyLock::new(|| {
    ELEMENT_AX_OBJECTS
        .iter()
        .filter(|(_, objects)| {
            objects
                .iter()
                .all(|name| ax_object(name).is_some_and(|object| object.kind == "widget"))
        })
        .map(|(concept, _)| {
            let attributes: &'static [ConceptAttribute] = if concept.name == Some("summary") {
                &[]
            } else {
                concept.attributes
            };
            (concept.name, attributes)
        })
        .collect()
});

/// Whether an element matches a concept: its name in lower case, then its attributes.
fn matches_schema(
    allocator: &Allocator,
    element: &Element,
    name: Option<&str>,
    attributes: &[ConceptAttribute],
) -> bool {
    name == Some(element.name.to_lowercase().as_str())
        && attributes_comparator(allocator, attributes, element)
}

/// Whether an element of the DOM is interactive by its semantics: one of an interactive role's
/// elements, else not one of a non-interactive role's, else one of a widget's of the
/// accessibility tree (`isInherentlyInteractiveElement`).
#[must_use]
pub fn is_inherently_interactive_element(allocator: &Allocator, element: &Element) -> bool {
    if !is_dom_element(&element.name.to_lowercase()) {
        return false;
    }
    let schema =
        |concept: &&Concept| matches_schema(allocator, element, concept.name, concept.attributes);
    if INTERACTIVE_SCHEMAS.iter().any(schema) {
        return true;
    }
    if NON_INTERACTIVE_SCHEMAS.iter().any(schema) {
        return false;
    }
    INTERACTIVE_AX_SCHEMAS
        .iter()
        .any(|(name, attributes)| matches_schema(allocator, element, *name, attributes))
}

/// Whether an element's role is a non-interactive one (`isNonInteractiveRole`).
#[must_use]
pub fn is_non_interactive_role(allocator: &Allocator, element: &Element) -> bool {
    attribute_value(allocator, element, "role")
        .text()
        .is_some_and(|role| NON_INTERACTIVE_ROLES.contains(&role))
}

/// Whether an element has a role by its semantics (`isSemanticRoleElement`): a concept of the
/// accessibility tree of its name, case kept, whose every attribute it has as a plain attribute
/// of the same value, and one of whose objects has the role. A bound attribute's value is an
/// expression, never equal to a concept's text.
#[must_use]
pub fn is_semantic_role_element(element: &str, role: &str, attributes: &[Bound3]) -> bool {
    ELEMENT_AX_OBJECTS.iter().any(|(concept, objects)| {
        concept.name == Some(element)
            && concept.attributes.iter().all(|wanted| {
                attributes.iter().any(|each| {
                    each.role == Role::Text
                        && each.name == wanted.name
                        && wanted.value == Some(each.attribute.value.as_str())
                })
            })
            && objects.iter().any(|name| {
                AX_OBJECTS
                    .iter()
                    .any(|object| object.name == *name && object.roles.contains(&role))
            })
    })
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;

    use super::{
        AttributeValue, NON_INTERACTIVE_ROLES, attribute_value, interactive_roles,
        is_hidden_from_screen_reader, is_inherently_interactive_element, is_js_space, to_number,
    };
    use crate::template::tree::{Element, Node, parse};

    /// The first element of a template.
    fn first(text: &str) -> Element {
        match parse(text).nodes.into_iter().next() {
            Some(Node::Element(element)) => element,
            _ => panic!("no element in {text}"),
        }
    }

    #[test]
    fn reads_an_attribute_s_value_as_the_plugin_does() {
        let allocator = Allocator::default();
        let value = |text: &str, name: &str| attribute_value(&allocator, &first(text), name);

        assert_eq!(
            value("<a role=\"none\"></a>", "role"),
            AttributeValue::Text("none".into())
        );
        assert_eq!(
            value("<a [attr.aria-checked]=\"true\"></a>", "aria-checked"),
            AttributeValue::Bool(true)
        );
        assert_eq!(value("<a [x]=\"2\"></a>", "x"), AttributeValue::Number(2.0));
        assert_eq!(
            value("<a [x]=\"undefined\"></a>", "x"),
            AttributeValue::Undefined
        );
        assert_eq!(value("<a [x]=\"null\"></a>", "x"), AttributeValue::Null);
        assert_eq!(value("<a [x]=\"[1]\"></a>", "x"), AttributeValue::Array);
        assert_eq!(value("<a [x]=\"{ a: 1 }\"></a>", "x"), AttributeValue::Map);
        assert_eq!(
            value("<a [x]=\"y\"></a>", "x"),
            AttributeValue::PropertyRead
        );
        assert_eq!(
            value("<a [x]=\"'y' | p\"></a>", "x"),
            AttributeValue::PropertyRead
        );
        assert_eq!(
            value("<a x=\"{{ y }}\"></a>", "x"),
            AttributeValue::PropertyRead
        );
        assert_eq!(
            value("<a [style.display]=\"'none'\"></a>", "style.display"),
            AttributeValue::Text("none".into())
        );
        // The plain attribute comes first, whatever the order written.
        assert_eq!(
            value("<a [x]=\"1\" x=\"a\"></a>", "x"),
            AttributeValue::Text("a".into())
        );
        assert_eq!(value("<a></a>", "x"), AttributeValue::Null);
        assert!(AttributeValue::Text("TRUE".into()).is_true(true));
        assert!(!AttributeValue::Text("TRUE".into()).is_true(false));
        assert!(!AttributeValue::Number(1.0).is_true(true));
    }

    #[test]
    fn tells_interactive_roles_and_elements() {
        let allocator = Allocator::default();
        let interactive = |text: &str| is_inherently_interactive_element(&allocator, &first(text));
        let roles = interactive_roles();

        assert!(roles.contains(&"button") && roles.contains(&"toolbar"));
        assert!(!roles.contains(&"progressbar") && !roles.contains(&"widget"));
        assert!(NON_INTERACTIVE_ROLES.contains(&"progressbar"));
        assert!(
            !NON_INTERACTIVE_ROLES.contains(&"toolbar")
                && !NON_INTERACTIVE_ROLES.contains(&"button")
        );
        assert!(interactive("<button></button>"));
        assert!(interactive("<INPUT type=\"checkbox\">"));
        assert!(interactive("<a routerLink=\"/\"></a>"));
        assert!(!interactive("<div></div>"));
        // `<li>` is a `listitem`, a non-interactive role's element.
        assert!(!interactive("<li></li>"));
        // Not an element of the DOM.
        assert!(!interactive("<app-button></app-button>"));
    }

    #[test]
    fn tells_an_element_hidden_from_a_screen_reader() {
        let allocator = Allocator::default();
        let hidden = |text: &str| {
            let element = first(text);
            is_hidden_from_screen_reader(&allocator, &element, &[])
        };

        // Our own cases, after the examples of `isHiddenFromScreenReader`.
        assert!(hidden("<div style=\"display : none\"></div>"));
        assert!(hidden(
            "<div [style]=\"'visibility: hidden; color: red'\"></div>"
        ));
        assert!(hidden("<div [style.display]=\"'none'\"></div>"));
        assert!(hidden("<div [style.visibility.hidden]=\"true\"></div>"));
        assert!(hidden("<div hidden></div>"));
        assert!(hidden("<div aria-hidden=\"true\"></div>"));
        assert!(hidden("<INPUT type=\"Hidden\">"));
        // A bound value is not known, an object literal never compared.
        assert!(!hidden("<div [hidden]=\"h\"></div>"));
        assert!(!hidden("<div [ngStyle]=\"{ display: 'none' }\"></div>"));
        assert!(!hidden("<div aria-hidden=\"false\"></div>"));
        // An element above hides it.
        let parsed = parse("<div hidden><a></a></div>");
        let Some(Node::Element(above)) = parsed.nodes.first() else {
            panic!("no element");
        };
        let Some(Node::Element(inner)) = above.children.first() else {
            panic!("no child");
        };
        let ancestors = [&parsed.nodes[0]];
        assert!(is_hidden_from_screen_reader(&allocator, inner, &ancestors));
        assert!(!is_hidden_from_screen_reader(&allocator, inner, &[]));
    }

    #[test]
    fn reads_a_number_as_javascript_does() {
        // Our own cases, after ECMAScript's StringToNumber.
        assert_eq!(to_number(" 12 "), 12.0);
        assert_eq!(to_number(""), 0.0);
        assert_eq!(to_number("-1.5e1"), -15.0);
        assert_eq!(to_number(".5"), 0.5);
        assert_eq!(to_number("0x1F"), 31.0);
        assert_eq!(to_number("-Infinity"), f64::NEG_INFINITY);
        for text in ["inf", "NaN", "1px", "0x", "-0x1", "+-1", "e5", "1e"] {
            assert!(to_number(text).is_nan(), "{text}");
        }
    }

    #[test]
    fn reads_whitespace_as_javascript_does() {
        assert!(is_js_space('\u{feff}') && is_js_space('\u{2009}'));
        // NEL is whitespace for Rust, not for `\s`.
        assert!(!is_js_space('\u{85}') && '\u{85}'.is_whitespace());
    }
}
