//! The inputs and outputs of a component or a directive as the binding rules read them, after
//! `@angular-eslint/utils` 22.6.0's `selectors.ts`: a member's name, the alias `@Input()`,
//! `@Output()`, `input()`, `input.required()` or `output()` gives it, and the entries of the
//! `inputs` and `outputs` metadata.

use oxc_ast::ast::{
    Argument, ArrayExpressionElement, CallExpression, Class, ClassElement, Decorator, Expression,
    MethodDefinitionKind, ObjectExpression, PropertyKey,
};

use super::classes::{called_name, key_identifier};
use super::metadata::{class_decorator_of, decorator_properties, names_metadata, properties};

/// A name written in the code: its text and where it starts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Written<'a> {
    /// The text, a string's value or a template's first part.
    pub text: &'a str,
    /// Where it starts, a string's or a template's quote.
    pub at: u32,
}

/// The direction of a binding.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// An input.
    Input,
    /// An output.
    Output,
}

/// A member of a class that may bind: its key, decorators and value.
pub struct Member<'b, 'a> {
    /// Its key.
    pub key: &'b PropertyKey<'a>,
    /// Whether its key is computed.
    pub computed: bool,
    /// Its decorators.
    pub decorators: &'b [Decorator<'a>],
    /// The value a property is set to.
    pub value: Option<&'b Expression<'a>>,
}

/// The members of a class that may carry a binding of a direction: properties, and setters for
/// inputs or getters for outputs.
#[must_use]
pub fn members<'b, 'a>(class: &'b Class<'a>, direction: Direction) -> Vec<Member<'b, 'a>> {
    let accessor = match direction {
        Direction::Input => MethodDefinitionKind::Set,
        Direction::Output => MethodDefinitionKind::Get,
    };
    class
        .body
        .body
        .iter()
        .filter_map(|element| match element {
            ClassElement::PropertyDefinition(property) => Some(Member {
                key: &property.key,
                computed: property.computed,
                decorators: &property.decorators,
                value: property.value.as_ref(),
            }),
            ClassElement::MethodDefinition(method) if method.kind == accessor => Some(Member {
                key: &method.key,
                computed: method.computed,
                decorators: &method.decorators,
                value: None,
            }),
            _ => None,
        })
        .collect()
}

/// The decorator of a member that calls a name.
#[must_use]
pub fn decorator<'b, 'a>(member: &Member<'b, 'a>, name: &str) -> Option<&'b Decorator<'a>> {
    member
        .decorators
        .iter()
        .find(|decorator| called_name(decorator) == Some(name))
}

/// A string or a template as a name written.
#[must_use]
pub fn written<'a>(expression: &Expression<'a>) -> Option<Written<'a>> {
    match expression {
        Expression::StringLiteral(literal) => Some(Written {
            text: literal.value.as_str()?,
            at: literal.span.start,
        }),
        Expression::TemplateLiteral(template) => Some(Written {
            text: template.quasis.first()?.value.raw.as_str(),
            at: template.span.start,
        }),
        _ => None,
    }
}

/// The `alias` of an options object (`ALIAS_PROPERTY_VALUE`).
fn alias_property<'a>(object: &ObjectExpression<'a>) -> Option<Written<'a>> {
    properties(object)
        .into_iter()
        .find(|property| key_identifier(&property.key) == Some("alias"))
        .and_then(|property| written(&property.value))
}

/// The aliases a call's arguments give: each string or template, when `strings` is set, and the
/// `alias` of each options object.
fn call_aliases<'a>(arguments: &[Argument<'a>], strings: bool) -> Vec<Written<'a>> {
    arguments
        .iter()
        .filter_map(|argument| match argument {
            Argument::ObjectExpression(object) => alias_property(object),
            other => other.as_expression().filter(|_| strings).and_then(written),
        })
        .collect()
}

/// The call of a signal function a value makes, `input(…)`, `input.required(…)` or `output(…)`,
/// with the place of its options among the call's arguments: `input(initial, options)`,
/// `input.required(options)`, `output(options)`.
#[must_use]
pub fn signal_call<'b, 'a>(
    value: &'b Expression<'a>,
    direction: Direction,
) -> Option<(&'b CallExpression<'a>, usize)> {
    let Expression::CallExpression(call) = value else {
        return None;
    };
    let function = match direction {
        Direction::Input => "input",
        Direction::Output => "output",
    };
    let options = match &call.callee {
        Expression::Identifier(callee) if callee.name == function => match direction {
            Direction::Input => Some(1),
            Direction::Output => Some(0),
        },
        Expression::StaticMemberExpression(member)
            if direction == Direction::Input
                && member.property.name == "required"
                && matches!(&member.object, Expression::Identifier(object) if object.name == "input") =>
        {
            Some(0)
        }
        _ => None,
    };
    options.map(|index| (&**call, index))
}

/// The aliases a member's binding gives (`INPUT_ALIAS`, `OUTPUT_ALIAS`): the strings and the
/// `alias` option of its decorator, or the `alias` option of its signal function.
#[must_use]
pub fn aliases<'a>(member: &Member<'_, 'a>, direction: Direction) -> Vec<Written<'a>> {
    let name = match direction {
        Direction::Input => "Input",
        Direction::Output => "Output",
    };
    let mut found = Vec::new();
    if let Some(decorator) = decorator(member, name)
        && let Expression::CallExpression(call) = &decorator.expression
    {
        found.extend(call_aliases(&call.arguments, true));
    }
    if let Some((call, _)) = member.value.and_then(|value| signal_call(value, direction)) {
        found.extend(call_aliases(&call.arguments, false));
    }
    found
}

/// Whether a member is a binding of a direction by its decorator, or for an output by its
/// `output()` value too (`INPUT_PROPERTY_OR_SETTER`, `OUTPUT_PROPERTY_OR_GETTER`).
#[must_use]
pub fn is_binding(member: &Member, direction: Direction) -> bool {
    match direction {
        Direction::Input => decorator(member, "Input").is_some(),
        Direction::Output => {
            decorator(member, "Output").is_some()
                || member
                    .value
                    .is_some_and(|value| signal_call(value, Direction::Output).is_some())
        }
    }
}

/// The name of a binding member, an identifier's not computed or a string's
/// (`INPUT_PROPERTY_OR_SETTER`'s `:matches(Identifier, Literal)`).
#[must_use]
pub fn member_name<'a>(member: &Member<'_, 'a>) -> Option<Written<'a>> {
    match member.key {
        PropertyKey::StaticIdentifier(key) if !member.computed => Some(Written {
            text: key.name.as_str(),
            at: key.span.start,
        }),
        PropertyKey::StringLiteral(key) if !member.computed => Some(Written {
            text: key.value.as_str()?,
            at: key.span.start,
        }),
        _ => None,
    }
}

/// The name of a member's key as an identifier, computed or not, for the rename rules.
#[must_use]
pub fn member_identifier<'a>(member: &Member<'_, 'a>) -> Option<&'a str> {
    key_identifier(member.key)
}

/// The strings and templates of a component's or directive's `inputs` or `outputs` metadata
/// (`INPUTS_METADATA_PROPERTY_LITERAL`, `OUTPUTS_METADATA_PROPERTY_LITERAL`).
#[must_use]
pub fn metadata_entries<'a>(class: &Class<'a>, direction: Direction) -> Vec<Written<'a>> {
    let key = match direction {
        Direction::Input => "inputs",
        Direction::Output => "outputs",
    };
    let Some((decorator, _)) = class_decorator_of(class, &["Component", "Directive"]) else {
        return Vec::new();
    };
    decorator_properties(decorator)
        .into_iter()
        .filter(|property| names_metadata(property, key))
        .flat_map(|property| match &property.value {
            Expression::ArrayExpression(array) => array
                .elements
                .iter()
                .filter_map(|element| match element {
                    ArrayExpressionElement::StringLiteral(_)
                    | ArrayExpressionElement::TemplateLiteral(_) => {
                        element.as_expression().and_then(written)
                    }
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        })
        .collect()
}

/// A text without its spaces and, when `brackets` is set, its brackets
/// (`withoutBracketsAndWhitespaces`).
#[must_use]
pub fn without_spaces(text: &str, brackets: bool) -> String {
    text.chars()
        .filter(|character| {
            !(character.is_whitespace() || (brackets && matches!(character, '[' | ']')))
        })
        .collect()
}

/// A metadata entry's name and alias, `name: alias` with its spaces and, when `brackets` is set,
/// its brackets removed.
#[must_use]
pub fn entry_parts(text: &str, brackets: bool) -> (String, Option<String>) {
    let kept = without_spaces(text, brackets);
    let mut parts = kept.split(':');
    let name = parts.next().unwrap_or_default().to_owned();
    (name, parts.next().map(str::to_owned))
}

/// The selectors of a component's or directive's `selector` (the rename rules' `selectors`):
/// its text without brackets and spaces, split at its commas, and the first name in brackets.
#[must_use]
pub fn class_selectors(class: &Class) -> (Vec<String>, Option<String>) {
    let Some((decorator, _)) = class_decorator_of(class, &["Component", "Directive"]) else {
        return (Vec::new(), None);
    };
    let Some(selector) = decorator_properties(decorator)
        .into_iter()
        .find(|property| names_metadata(property, "selector"))
        .and_then(|property| written(&property.value))
    else {
        return (Vec::new(), None);
    };
    let bracketed = selector
        .text
        .split_once('[')
        .and_then(|(_, rest)| rest.split_once(']').map(|(name, _)| name.to_owned()));
    let parts = without_spaces(selector.text, true)
        .split(',')
        .map(str::to_owned)
        .collect();
    (parts, bracketed)
}

/// Whether an alias is allowed by the class's selectors (`isAliasNameAllowed`): a selector
/// itself, the name in brackets (inputs only), or a selector followed by the capitalised
/// property name.
#[must_use]
pub fn alias_allowed(
    selectors: &(Vec<String>, Option<String>),
    property: &str,
    alias: &str,
    direction: Direction,
) -> bool {
    let mut capitalised = property.chars();
    let capitalised = capitalised
        .next()
        .map(|first| first.to_uppercase().chain(capitalised).collect::<String>())
        .unwrap_or_default();
    selectors.0.iter().any(|selector| {
        selector == alias
            || (direction == Direction::Input && selectors.1.as_deref() == Some(alias))
            || format!("{selector}{capitalised}") == alias
    })
}

/// Every name an output of a class is bound by, in the order of the code: its `outputs`
/// metadata entries, then each output member's name and aliases (`OUTPUTS_METADATA_PROPERTY_LITERAL`,
/// `OUTPUT_ALIAS` and `OUTPUT_PROPERTY_OR_GETTER` together).
#[must_use]
pub fn output_names<'a>(class: &Class<'a>) -> Vec<Written<'a>> {
    let mut names = metadata_entries(class, Direction::Output);
    for member in members(class, Direction::Output) {
        if is_binding(&member, Direction::Output)
            && let Some(name) = member_name(&member)
        {
            names.push(name);
        }
        names.extend(aliases(&member, Direction::Output));
    }
    names.sort_by_key(|name| name.at);
    names
}

/// Whether a name is a prefix or starts with it followed by anything but a lower-case letter
/// (`^prefix(([^a-z])|(?=$))`).
#[must_use]
pub fn is_prefixed(name: &str, prefix: &str) -> bool {
    name.strip_prefix(prefix)
        .is_some_and(|rest| !rest.starts_with(|character: char| character.is_ascii_lowercase()))
}

/// The alias a member's decorator gives (`no-input-prefix`'s reading): the `alias` of its first
/// argument, an object, else that argument when it is a string or a template.
#[must_use]
pub fn decorator_alias<'a>(member: &Member<'_, 'a>, name: &str) -> Option<Written<'a>> {
    let Expression::CallExpression(call) = &decorator(member, name)?.expression else {
        return None;
    };
    match call.arguments.first()? {
        Argument::ObjectExpression(object) => alias_property(object),
        other => other.as_expression().and_then(written),
    }
}
