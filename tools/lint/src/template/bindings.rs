//! What each attribute, text and block of a template binds, its expressions parsed. The kinds
//! of attribute port `parseAttribute` of `render3/r3_template_transform.ts` (`BIND_NAME_REGEXP`
//! and the delimiters `[(…)]`, `[…]`, `(…)`, the `*` of a template binding) of
//! `@angular/compiler` 22.2.3 (REF-ANGULAR-ML-PARSER); a template binding's microsyntax
//! (`*ngFor="let item of items; index as i"`) follows `parseTemplateBindings` of
//! `expression_parser/parser.ts`, its bindings split at `;` and `,` before each is read; the
//! blocks' parameters follow `render3/r3_control_flow.ts` and `r3_deferred_blocks.ts`, their
//! messages the project's own (REF-ANGULAR-CONTROL-FLOW). `check` reads every expression of a
//! template and gives its errors. The tests are this module's own cases.

use oxc_allocator::Allocator;

use super::expressions::{Bound, expression_end, read, read_action, split_top_level};
use super::lexer::{ParseError, Span, Token, TokenType, offset};
use super::tags::split_ns_name;
use super::tree::{Attribute, Block, Element, Node, Parsed};

/// What an attribute is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeKind {
    /// A plain attribute, `name="text"`.
    Plain,
    /// A plain attribute whose value interpolates, `title="a {{ b }}"`.
    Interpolated,
    /// `[name]="expression"` or `bind-name`.
    Property,
    /// `(name)="statements"` or `on-name`.
    Event,
    /// `[(name)]="expression"` or `bindon-name`.
    TwoWay,
    /// `*name="microsyntax"`.
    Template,
    /// `#name` or `ref-name`.
    Reference,
    /// `let-name` on an `<ng-template>`.
    Variable,
    /// `@name`, an animation trigger.
    Animation,
}

/// An attribute read: its kind, its name without delimiters, what its value binds.
#[derive(Debug)]
pub struct ReadAttribute<'a> {
    /// Its kind.
    pub kind: AttributeKind,
    /// Its name, its prefix and delimiters removed (`click` for `(click)`).
    pub name: String,
    /// What its value binds.
    pub value: Value<'a>,
}

/// What a value binds.
#[derive(Debug)]
pub enum Value<'a> {
    /// Nothing parsed: a plain text, a reference's or a variable's name.
    Text,
    /// An expression and its pipes.
    Binding(Bound<'a>),
    /// Statements, an event's.
    Action(Vec<Bound<'a>>),
    /// The interpolations of a text or a value.
    Interpolations(Vec<Bound<'a>>),
    /// A template binding's microsyntax.
    TemplateBindings(Vec<TemplateBinding<'a>>),
}

/// A binding of a template binding's microsyntax.
#[derive(Debug)]
pub enum TemplateBinding<'a> {
    /// `key expression` (`ngForOf items`), the key prefixed by the directive's name after the
    /// first; its expression absent when none is written.
    Expression {
        /// Its key.
        key: String,
        /// Its expression.
        value: Option<Bound<'a>>,
        /// Where its key starts in the template; `None` for the first binding, keyed by the
        /// directive, whose key is the attribute's name.
        key_start: Option<u32>,
        /// Where the next binding starts, past this one's `;` or `,` and the whitespace after:
        /// the end of its source span, as `parseTemplateBindings` spans it.
        end: u32,
    },
    /// `let name = key` or `key as name`: a variable and the context's key it reads.
    Variable {
        /// The variable.
        name: String,
        /// The key read, `$implicit` when none.
        value: String,
    },
}

/// A block's parameters read.
#[derive(Debug, Default)]
pub struct BlockParameters<'a> {
    /// The main expression: `@if`'s and `@else if`'s condition, `@for`'s collection,
    /// `@switch`'s and `@case`'s value.
    pub expression: Option<Bound<'a>>,
    /// `@for`'s item, `@if`'s `as` alias.
    pub alias: Option<String>,
    /// `@for`'s `track` expression.
    pub track: Option<Bound<'a>>,
    /// `@for`'s `let` variables: each name and the context variable it reads (`$index`).
    pub variables: Vec<(String, String)>,
    /// `@defer`'s `when` conditions, `prefetch` and `hydrate` ones among them.
    pub conditions: Vec<Bound<'a>>,
}

/// The names `parseAttribute` reads before the delimiters, and the kind each gives.
const PREFIXES: [(&str, AttributeKind); 7] = [
    ("bindon-", AttributeKind::TwoWay),
    ("bind-", AttributeKind::Property),
    ("let-", AttributeKind::Variable),
    ("ref-", AttributeKind::Reference),
    ("#", AttributeKind::Reference),
    ("on-", AttributeKind::Event),
    ("@", AttributeKind::Animation),
];

/// The delimiters of a binding's name, longest first, and the kind each gives.
const DELIMITERS: [(&str, &str, AttributeKind); 3] = [
    ("[(", ")]", AttributeKind::TwoWay),
    ("[", "]", AttributeKind::Property),
    ("(", ")", AttributeKind::Event),
];

/// An attribute's kind and name, as `parseAttribute` tells them apart.
#[must_use]
pub fn attribute_kind(name: &str) -> (AttributeKind, String) {
    if let Some(key) = name.strip_prefix('*') {
        return (AttributeKind::Template, key.to_owned());
    }
    // `bindon-` is tried before `bind-`, which the compiler's alternation does by its order of
    // groups failing on the `-`.
    for (prefix, kind) in PREFIXES {
        if let Some(identifier) = name.strip_prefix(prefix) {
            return (kind, identifier.to_owned());
        }
    }
    for (start, end, kind) in DELIMITERS {
        if name.len() > start.len() + end.len()
            && let Some(inner) = name
                .strip_prefix(start)
                .and_then(|rest| rest.strip_suffix(end))
        {
            return (kind, inner.to_owned());
        }
    }
    (AttributeKind::Plain, name.to_owned())
}

/// An attribute read, its value parsed by its kind.
///
/// # Errors
///
/// When its value cannot be read, or a `let-` is on another element than `<ng-template>`.
pub fn read_attribute<'a>(
    allocator: &'a Allocator,
    element: &str,
    attribute: &Attribute,
) -> Result<ReadAttribute<'a>, ParseError> {
    let (mut kind, name) = attribute_kind(&attribute.name);
    let start = attribute
        .value_span
        // The value's first byte, its leading whitespace included, as Angular's binding parser
        // offsets an expression from `valueSpan.fullStart`.
        .map_or(attribute.span.end, |span| span.full_start);
    let text = attribute.value.as_str();
    let value = match kind {
        AttributeKind::Property | AttributeKind::TwoWay => {
            Value::Binding(read(allocator, text, start)?)
        }
        AttributeKind::Event => Value::Action(read_action(allocator, text, start)?),
        AttributeKind::Template => {
            Value::TemplateBindings(template_bindings(allocator, &name, text, start)?)
        }
        AttributeKind::Variable if split_ns_name(element).1 != "ng-template" => {
            return Err(ParseError {
                span: attribute.span,
                message: "\"let-\" is only supported on ng-template elements.".to_owned(),
            });
        }
        AttributeKind::Plain => {
            let interpolations = interpolations(allocator, &attribute.value_tokens)?;
            if interpolations.is_empty() {
                Value::Text
            } else {
                kind = AttributeKind::Interpolated;
                Value::Interpolations(interpolations)
            }
        }
        _ => Value::Text,
    };
    Ok(ReadAttribute { kind, name, value })
}

/// The interpolations among a text's or a value's tokens, each read; one left open is text, as
/// `splitInterpolation` leaves it.
///
/// # Errors
///
/// When an interpolation is blank or cannot be read.
pub fn interpolations<'a>(
    allocator: &'a Allocator,
    tokens: &[Token],
) -> Result<Vec<Bound<'a>>, ParseError> {
    let mut found = Vec::new();
    for token in tokens {
        let closed = token.parts.len() == 3;
        if !matches!(
            token.kind,
            TokenType::Interpolation | TokenType::AttrValueInterpolation
        ) || !closed
        {
            continue;
        }
        let text = token.parts.get(1).map_or("", String::as_str);
        let start = token.span.start + 2;
        if text.trim().is_empty() {
            return Err(ParseError {
                span: token.span,
                message: "Blank expressions are not allowed in interpolated strings".to_owned(),
            });
        }
        found.push(read(allocator, text, start)?);
    }
    Ok(found)
}

/// A template binding's microsyntax read (`parseTemplateBindings`): the first binding keyed by
/// the directive, `let` variables, `key expression`, `key as alias` and `expression as alias`.
///
/// # Errors
///
/// When an expression cannot be read or a `let` has no name.
pub fn template_bindings<'a>(
    allocator: &'a Allocator,
    directive: &str,
    text: &str,
    start: u32,
) -> Result<Vec<TemplateBinding<'a>>, ParseError> {
    let mut bindings = Vec::new();
    let pieces = split_top_level(text, &[';', ',']);
    if pieces.is_empty() {
        bindings.push(TemplateBinding::Expression {
            key: directive.to_owned(),
            value: None,
            key_start: None,
            end: start + offset(text.len()),
        });
    }
    for (index, (at, piece)) in pieces.into_iter().enumerate() {
        let mut rest = piece;
        let mut rest_at = at;
        let mut first = index == 0;
        while !rest.trim().is_empty() {
            let skipped = rest.len() - rest.trim_start().len();
            rest = rest.trim_start();
            rest_at += skipped;
            if let Some(after_let) = rest
                .strip_prefix("let")
                .filter(|after| after.starts_with(char::is_whitespace))
            {
                let (name, after_name) = take_key(after_let.trim_start());
                if name.is_empty() {
                    return Err(ParseError {
                        span: Span::new(start as usize + rest_at, start as usize + rest_at + 3),
                        message: "Unexpected end of a let, expected identifier or keyword"
                            .to_owned(),
                    });
                }
                let mut value = "$implicit".to_owned();
                let mut after = after_name.trim_start();
                if let Some(assigned) = after.strip_prefix('=') {
                    let (key, after_key) = take_key(assigned.trim_start());
                    key.clone_into(&mut value);
                    after = after_key;
                }
                bindings.push(TemplateBinding::Variable {
                    name: name.to_owned(),
                    value,
                });
                rest_at += rest.len() - after.len();
                rest = after;
                first = false;
                continue;
            }
            let (key, expression) = if first {
                (directive.to_owned(), rest)
            } else {
                let (key, after_key) = take_key(rest);
                let after = after_key.trim_start();
                if let Some((alias, remainder)) = as_alias(after) {
                    bindings.push(TemplateBinding::Variable {
                        name: alias.to_owned(),
                        value: key.to_owned(),
                    });
                    rest_at += rest.len() - remainder.len();
                    rest = remainder;
                    first = false;
                    continue;
                }
                let after = after.strip_prefix(':').unwrap_or(after);
                (format!("{directive}{}", capitalized(key)), after)
            };
            let expression_at = rest_at + rest.len() - expression.len();
            let (expression, mut remainder) = expression.split_at(expression_end(expression));
            let value = if expression.trim().is_empty() {
                None
            } else {
                Some(read(allocator, expression, start + offset(expression_at))?)
            };
            let next = next_binding(
                text,
                at + piece.len(),
                expression_at + expression.len(),
                remainder,
            );
            bindings.push(TemplateBinding::Expression {
                key: key.clone(),
                value,
                key_start: (!first).then(|| start + offset(rest_at)),
                end: start + offset(next),
            });
            if let Some((alias, after_alias)) = as_alias(remainder.trim_start()) {
                bindings.push(TemplateBinding::Variable {
                    name: alias.to_owned(),
                    value: key,
                });
                remainder = after_alias;
            }
            rest_at += rest.len() - remainder.len();
            rest = remainder;
            first = false;
        }
    }
    Ok(bindings)
}

/// Where the binding after an expression starts: past the whitespace after it when more of
/// its piece follows (`as alias`, `else block`), else past its piece's `;` or `,` and the
/// whitespace after, or the value's end; a binding's source span ends there.
fn next_binding(text: &str, piece_end: usize, expression_end: usize, remainder: &str) -> usize {
    if !remainder.trim().is_empty() {
        return expression_end + remainder.len() - remainder.trim_start().len();
    }
    match text.get(piece_end + 1..) {
        Some(after) => piece_end + 1 + after.len() - after.trim_start().len(),
        None => text.len(),
    }
}

/// A key of the microsyntax: letters, digits, `_`, `$` and `-`; and the text after it.
fn take_key(text: &str) -> (&str, &str) {
    let end = text
        .find(|each: char| !(each.is_ascii_alphanumeric() || matches!(each, '_' | '$' | '-')))
        .unwrap_or(text.len());
    text.split_at(end)
}

/// The alias of `as alias`, when the text is only that.
fn strip_as(text: &str) -> Option<&str> {
    as_alias(text)
        .filter(|(_, rest)| rest.trim().is_empty())
        .map(|(alias, _)| alias)
}

/// The alias of a text starting with `as alias`, and the text after it.
fn as_alias(text: &str) -> Option<(&str, &str)> {
    let after = text
        .strip_prefix("as")
        .filter(|after| after.starts_with(char::is_whitespace))?
        .trim_start();
    let (alias, rest) = take_key(after);
    (!alias.is_empty()).then_some((alias, rest))
}

/// A key with its first letter capitalised (`of` gives `Of`).
fn capitalized(key: &str) -> String {
    let mut characters = key.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

/// A block's parameters read by the block's name.
///
/// # Errors
///
/// When a parameter cannot be read or is not one the block takes.
pub fn block_parameters<'a>(
    allocator: &'a Allocator,
    block: &Block,
) -> Result<BlockParameters<'a>, ParseError> {
    let mut read_parameters = BlockParameters::default();
    let parameters = &block.parameters;
    let unrecognized = |parameter: &super::tree::BlockParameter| ParseError {
        span: parameter.span,
        message: format!(
            "Unrecognized parameter of @{}: \"{}\"",
            block.name, parameter.expression
        ),
    };
    match block.name.as_str() {
        "if" | "else if" | "switch" | "case" => {
            if let Some(main) = parameters.first() {
                read_parameters.expression =
                    Some(read(allocator, &main.expression, main.span.start)?);
            }
            for parameter in parameters.iter().skip(1) {
                match strip_as(&parameter.expression) {
                    Some(alias) if block.name.contains("if") => {
                        read_parameters.alias = Some(alias.to_owned());
                    }
                    _ => return Err(unrecognized(parameter)),
                }
            }
        }
        "for" => {
            let Some(main) = parameters.first() else {
                return Err(ParseError {
                    span: block.span,
                    message: "@for loop must have a \"track\" expression".to_owned(),
                });
            };
            let (item, after_item) = take_key(main.expression.trim_start());
            let of = after_item
                .trim_start()
                .strip_prefix("of")
                .filter(|after| after.starts_with(char::is_whitespace))
                .map(str::trim_start);
            let Some(collection) = of.filter(|_| !item.is_empty()) else {
                return Err(ParseError {
                    span: main.span,
                    message: "Cannot parse expression. @for loop expression must match the pattern \"<identifier> of <expression>\"".to_owned(),
                });
            };
            read_parameters.alias = Some(item.to_owned());
            let at = main.span.start + offset(main.expression.len() - collection.len());
            read_parameters.expression = Some(read(allocator, collection, at)?);
            for parameter in parameters.iter().skip(1) {
                let text = parameter.expression.as_str();
                if let Some(track) = keyword(text, "track") {
                    let at = parameter.span.start + offset(text.len() - track.len());
                    read_parameters.track = Some(read(allocator, track, at)?);
                } else if let Some(variables) = keyword(text, "let") {
                    for (_, variable) in split_top_level(variables, &[',']) {
                        let (name, value) = variable.split_once('=').unwrap_or((variable, ""));
                        read_parameters
                            .variables
                            .push((name.trim().to_owned(), value.trim().to_owned()));
                    }
                } else {
                    return Err(unrecognized(parameter));
                }
            }
            if read_parameters.track.is_none() {
                return Err(ParseError {
                    span: block.span,
                    message: "@for loop must have a \"track\" expression".to_owned(),
                });
            }
        }
        _ => {
            for parameter in parameters {
                let text = parameter.expression.as_str();
                let condition = ["prefetch", "hydrate"]
                    .iter()
                    .find_map(|prefix| keyword(text, prefix))
                    .unwrap_or(text);
                if let Some(when) = keyword(condition, "when") {
                    let at = parameter.span.start + offset(text.len() - when.len());
                    read_parameters.conditions.push(read(allocator, when, at)?);
                }
            }
        }
    }
    Ok(read_parameters)
}

/// The text after a leading keyword and whitespace.
fn keyword<'t>(text: &'t str, word: &str) -> Option<&'t str> {
    text.trim_start()
        .strip_prefix(word)
        .filter(|after| after.starts_with(char::is_whitespace))
        .map(str::trim_start)
}

/// Whether an element carries `ngNonBindable`, its name compared as written: Angular reads its
/// children as plain text and attributes, binding nothing (`preparseElement` and
/// `NonBindableVisitor` of `render3/r3_template_transform.ts`), so none of their expressions is
/// read; its own attributes are.
#[must_use]
pub fn is_non_bindable(element: &Element) -> bool {
    element
        .attrs
        .iter()
        .any(|attribute| attribute.name == "ngNonBindable")
}

/// Every expression of a template read; the errors met, the tree's first.
#[must_use]
pub fn check(parsed: &Parsed) -> Vec<ParseError> {
    let allocator = Allocator::default();
    let mut errors = parsed.errors.clone();
    if errors.is_empty() {
        check_nodes(&allocator, &parsed.nodes, &mut errors);
    }
    errors
}

/// The expressions of nodes read, their errors kept.
fn check_nodes(allocator: &Allocator, nodes: &[Node], errors: &mut Vec<ParseError>) {
    for node in nodes {
        let found = match node {
            Node::Element(element) => {
                let mut templates = 0;
                for attribute in &element.attrs {
                    if attribute.name.starts_with('*') {
                        templates += 1;
                        if templates > 1 {
                            errors.push(ParseError {
                                span: attribute.span,
                                message: "Can't have multiple template bindings on one element. Use only one attribute prefixed with *".to_owned(),
                            });
                        }
                    }
                    if let Err(error) = read_attribute(allocator, &element.name, attribute) {
                        errors.push(error);
                    }
                }
                if !is_non_bindable(element) {
                    check_nodes(allocator, &element.children, errors);
                }
                Ok(())
            }
            Node::Text(text) => interpolations(allocator, &text.tokens).map(|_| ()),
            Node::Block(block) => {
                check_nodes(allocator, &block.children, errors);
                block_parameters(allocator, block).map(|_| ())
            }
            Node::LetDeclaration(declaration) => {
                read(allocator, &declaration.value, declaration.value_span.start).map(|_| ())
            }
            Node::Expansion(expansion) => {
                for case in &expansion.cases {
                    check_nodes(allocator, &case.expression, errors);
                }
                read(
                    allocator,
                    &expansion.switch_value,
                    expansion.switch_value_span.start,
                )
                .map(|_| ())
            }
            Node::Comment(_) => Ok(()),
        };
        if let Err(error) = found {
            errors.push(error);
        }
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;

    use super::{
        AttributeKind, TemplateBinding, Value, attribute_kind, block_parameters, check,
        read_attribute,
    };
    use crate::template::tree::{Node, parse};

    /// The messages of a template's errors.
    fn messages(source: &str) -> Vec<String> {
        check(&parse(source))
            .into_iter()
            .map(|error| error.message)
            .collect()
    }

    #[test]
    fn tells_the_kinds_of_attributes_apart() {
        let kinds: Vec<(AttributeKind, String)> = [
            "[x]",
            "(click)",
            "[(m)]",
            "*ngIf",
            "#ref",
            "ref-r",
            "let-v",
            "bind-b",
            "on-o",
            "bindon-t",
            "@fade",
            "[attr.aria-label]",
            "class",
            "[]",
            "()",
        ]
        .iter()
        .map(|name| attribute_kind(name))
        .collect();
        let expected = [
            (AttributeKind::Property, "x"),
            (AttributeKind::Event, "click"),
            (AttributeKind::TwoWay, "m"),
            (AttributeKind::Template, "ngIf"),
            (AttributeKind::Reference, "ref"),
            (AttributeKind::Reference, "r"),
            (AttributeKind::Variable, "v"),
            (AttributeKind::Property, "b"),
            (AttributeKind::Event, "o"),
            (AttributeKind::TwoWay, "t"),
            (AttributeKind::Animation, "fade"),
            (AttributeKind::Property, "attr.aria-label"),
            (AttributeKind::Plain, "class"),
            (AttributeKind::Plain, "[]"),
            (AttributeKind::Plain, "()"),
        ]
        .map(|(kind, name)| (kind, name.to_owned()));
        assert_eq!(kinds, expected);
    }

    #[test]
    fn f52_ac2_places_a_value_from_its_first_byte_its_leading_whitespace_included() {
        let parsed = parse("<a [b]=\"\n  c\" *ngIf=\"d; else e\"></a>");
        let Some(Node::Element(element)) = parsed.nodes.first() else {
            panic!("an element");
        };
        let allocator = Allocator::default();
        let read: Vec<_> = element
            .attrs
            .iter()
            .map(|attribute| read_attribute(&allocator, &element.name, attribute))
            .collect();

        // `"` at byte 7, the value from byte 8, its line end and two spaces before `c` at 11.
        assert!(
            matches!(&read[0], Ok(attribute) if matches!(&attribute.value,
            Value::Binding(bound) if bound.start == 8))
        );
        // `*ngIf` at byte 14, its value from 21: `d`, its binding ending past `; ` at 24, where
        // `else` starts; `e` at 29, the last binding ending with the value, at 30.
        assert!(
            matches!(&read[1], Ok(attribute) if matches!(&attribute.value,
            Value::TemplateBindings(bindings) if matches!(&bindings[..], [
                TemplateBinding::Expression { key_start: None, end: 24, .. },
                TemplateBinding::Expression { key_start: Some(24), end: 30, .. },
            ])))
        );
    }

    #[test]
    fn reads_each_attributes_value_by_its_kind() {
        let parsed = parse(
            "<a [x]=\"a | p\" (click)=\"f(); g()\" title=\"t {{ u }}\" *ngFor=\"let item of items; index as i; trackBy: id\" #r></a>",
        );
        let Some(Node::Element(element)) = parsed.nodes.first() else {
            panic!("an element");
        };
        let allocator = Allocator::default();
        let read: Vec<_> = element
            .attrs
            .iter()
            .map(|attribute| read_attribute(&allocator, &element.name, attribute))
            .collect();

        assert!(
            matches!(&read[0], Ok(attribute) if matches!(&attribute.value, Value::Binding(bound) if bound.pipes.len() == 1))
        );
        assert!(
            matches!(&read[1], Ok(attribute) if matches!(&attribute.value, Value::Action(statements) if statements.len() == 2))
        );
        assert!(
            matches!(&read[2], Ok(attribute) if attribute.kind == AttributeKind::Interpolated && matches!(&attribute.value, Value::Interpolations(found) if found.len() == 1))
        );
        let Ok(template) = &read[3] else {
            panic!("a template binding");
        };
        let Value::TemplateBindings(bindings) = &template.value else {
            panic!("template bindings");
        };
        let shown: Vec<String> = bindings
            .iter()
            .map(|binding| match binding {
                TemplateBinding::Expression { key, value, .. } => {
                    format!("{key} {}", value.is_some())
                }
                TemplateBinding::Variable { name, value } => format!("let {name} = {value}"),
            })
            .collect();
        assert_eq!(
            shown,
            [
                "let item = $implicit",
                "ngForOf true",
                "let i = index",
                "ngForTrackBy true"
            ]
        );
        assert!(matches!(&read[4], Ok(attribute) if attribute.kind == AttributeKind::Reference));
    }

    #[test]
    fn reads_an_expression_with_its_alias_and_a_let_with_its_key() {
        let parsed = parse(
            "<ng-template *ngIf=\"user$ | async as user; else other\" let-x=\"y\"></ng-template>",
        );
        let Some(Node::Element(element)) = parsed.nodes.first() else {
            panic!("an element");
        };
        let allocator = Allocator::default();
        let Ok(template) = read_attribute(&allocator, &element.name, &element.attrs[0]) else {
            panic!("a template binding");
        };
        let Value::TemplateBindings(bindings) = template.value else {
            panic!("template bindings");
        };
        assert!(
            matches!(&bindings[0], TemplateBinding::Expression { key, value: Some(bound), .. } if key == "ngIf" && bound.pipes[0].name == "async")
        );
        assert!(
            matches!(&bindings[1], TemplateBinding::Variable { name, value } if name == "user" && value == "ngIf")
        );
        assert!(
            matches!(&bindings[2], TemplateBinding::Expression { key, value: Some(_), .. } if key == "ngIfElse")
        );
        assert!(read_attribute(&allocator, &element.name, &element.attrs[1]).is_ok());
    }

    #[test]
    fn reads_the_parameters_of_each_block() {
        let parsed = parse(
            "@if (a | p; as b) {} @for (item of items | q; track item.id; let i = $index, odd = $odd) {} @defer (on viewport; prefetch when c; when d) {}",
        );
        let allocator = Allocator::default();
        let blocks: Vec<_> = parsed
            .nodes
            .iter()
            .filter_map(|node| match node {
                Node::Block(block) => Some(block_parameters(&allocator, block)),
                _ => None,
            })
            .collect();

        let Ok(condition) = &blocks[0] else {
            panic!("@if");
        };
        assert_eq!(condition.alias.as_deref(), Some("b"));
        assert_eq!(
            condition.expression.as_ref().map(|bound| bound.pipes.len()),
            Some(1)
        );
        let Ok(repeat) = &blocks[1] else {
            panic!("@for");
        };
        assert_eq!(repeat.alias.as_deref(), Some("item"));
        // `@for (` is 6 bytes after the `@if` block and its space (21 bytes): `items` at 21 + 6 + 8.
        assert_eq!(
            repeat.expression.as_ref().map(|bound| bound.start),
            Some(35)
        );
        assert!(repeat.track.is_some());
        assert_eq!(
            repeat.variables,
            [
                ("i".to_owned(), "$index".to_owned()),
                ("odd".to_owned(), "$odd".to_owned())
            ]
        );
        let Ok(deferred) = &blocks[2] else {
            panic!("@defer");
        };
        assert_eq!(deferred.conditions.len(), 2);
    }

    #[test]
    fn checks_every_expression_and_reports_the_first_error_of_each() {
        assert_eq!(
            messages("<a [x]=\"a | p\">{{ b }}</a>@let c = d;"),
            Vec::<String>::new()
        );
        assert_eq!(
            messages("<a (click)=\"a | p\" *x=\"1\" *y=\"2\">{{ }}</a>"),
            [
                "Cannot have a pipe in an action expression",
                "Can't have multiple template bindings on one element. Use only one attribute prefixed with *",
                "Blank expressions are not allowed in interpolated strings",
            ]
        );
        assert_eq!(
            messages("<div let-x=\"y\"></div>"),
            ["\"let-\" is only supported on ng-template elements."]
        );
        assert_eq!(
            messages("@for (items) {}"),
            [
                "Cannot parse expression. @for loop expression must match the pattern \"<identifier> of <expression>\""
            ]
        );
        assert_eq!(
            messages("@for (i of l) {}"),
            ["@for loop must have a \"track\" expression"]
        );
        assert_eq!(
            messages("@if (a; b) {}"),
            ["Unrecognized parameter of @if: \"b\""]
        );
        assert_eq!(messages("{n | p, plural, =0 {a}}"), Vec::<String>::new());
        assert_eq!(
            messages("<div/>").len(),
            1,
            "the tree's errors stop the reading of expressions"
        );
    }
}
