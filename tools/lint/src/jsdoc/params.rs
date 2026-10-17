//! The names of a function's parameters as `eslint-plugin-jsdoc` 65.3.1 reads them
//! (`src/jsdocUtils.js`: `getFunctionParameterNames`, `getPropertiesFromPropertySignature`,
//! `flattenRoots`), from Oxc's parameters seen as `ESTree` writes them: a parameter with a
//! default is an assignment pattern, its type on its left side.

use oxc_ast::ast::{
    BindingPattern, BindingProperty, BindingRestElement, FormalParameters, PropertyKey,
    TSSignature, TSThisParameter, TSType, TSTypeAnnotation,
};
use oxc_span::GetSpan;

use crate::source::Source;

/// The name of a rest element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RestName {
    /// An identifier.
    Text(String),
    /// None (`undefined`).
    Missing,
    /// The names of an array pattern's elements, `None` where an element is no identifier.
    List(Vec<Option<String>>),
}

impl RestName {
    /// The name as `String(name)` writes it.
    #[must_use]
    pub fn written(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Missing => "undefined".to_owned(),
            Self::List(list) => list
                .iter()
                .map(|each| each.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}

/// The flattened names of a destructured or typed parameter (`FlattendRootInfo`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Flat {
    /// Whether a property holds a rest element.
    pub has_property_rest: bool,
    /// Whether an element is a rest element.
    pub has_rest_element: bool,
    /// The paths of the properties.
    pub names: Vec<String>,
    /// Whether each is a rest property; aligned on `names` as the plugin aligns them.
    pub rests: Vec<bool>,
    /// The parameter's name when its type literal gives the properties.
    pub annotation: Option<String>,
}

/// What a parameter, or a part of one, reads as (`ParamNameInfo`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Info {
    /// Nothing (`undefined`).
    Missing,
    /// A name.
    Name(String),
    /// A rest element, and whether it sits in an object pattern.
    Rest {
        /// Its name.
        name: RestName,
        /// Whether it is a rest property (`isRestProperty`), unset at the top level.
        property: Option<bool>,
    },
    /// An element of an array pattern, by its index (`{ name, restElement }`).
    Index {
        /// Its name.
        name: String,
        /// Whether it is a rest element.
        rest: bool,
    },
    /// A name and its parts.
    Pair(Option<String>, Inner),
}

/// The parts of a pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inner {
    /// Parts not flattened yet.
    List(Vec<Info>),
    /// Parts flattened.
    Flat(Flat),
}

/// Flattens nested names into paths from a root (`flattenRoots`).
#[must_use]
pub fn flatten(params: &[Info], root: &str) -> Flat {
    let mut flat = Flat::default();
    let prefixed = |name: &str| {
        if root.is_empty() {
            name.to_owned()
        } else {
            format!("{root}.{name}")
        }
    };
    for current in params {
        match current {
            Info::Pair(name, inner) => {
                let names: Vec<Info> = match inner {
                    Inner::List(list) => list.clone(),
                    Inner::Flat(inner) => {
                        flat.has_rest_element |= inner.has_rest_element;
                        flat.has_property_rest |= inner.has_property_rest;
                        inner.names.iter().cloned().map(Info::Name).collect()
                    }
                };
                let path = match (root.is_empty(), name) {
                    (true, name) => name.clone(),
                    (false, Some(name)) => Some(format!("{root}.{name}")),
                    (false, None) => Some(format!("{root}.undefined")),
                };
                let nested = flatten(&names, path.as_deref().unwrap_or_default());
                flat.has_rest_element |= nested.has_rest_element;
                flat.has_property_rest |= nested.has_property_rest;
                flat.names.extend(path.filter(|path| !path.is_empty()));
                flat.names.extend(nested.names);
                flat.rests.push(false);
                flat.rests.extend(nested.rests);
            }
            Info::Rest { name, property } => {
                let rest_property = *property == Some(true);
                flat.has_property_rest |= rest_property;
                flat.rests.push(rest_property);
                flat.has_rest_element = true;
                flat.names.push(prefixed(&name.written()));
            }
            Info::Index { name, rest } => {
                flat.rests.push(false);
                flat.has_rest_element |= *rest;
                flat.names.push(prefixed(name));
            }
            Info::Name(name) => {
                flat.rests.push(false);
                flat.names.push(prefixed(name));
            }
            Info::Missing => {}
        }
    }
    flat
}

/// The name of a key that is an identifier, as `key.name` reads it.
fn key_name(key: &PropertyKey) -> Option<String> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.to_string()),
        PropertyKey::Identifier(id) => Some(id.name.to_string()),
        PropertyKey::PrivateIdentifier(id) => Some(id.name.to_string()),
        _ => None,
    }
}

/// The names of a type literal's members (`getPropertiesFromPropertySignature`).
fn member_info(member: &TSSignature) -> Info {
    match member {
        TSSignature::TSPropertySignature(property) => {
            let name = key_name(&property.key);
            match property
                .type_annotation
                .as_deref()
                .map(|annotation| &annotation.type_annotation)
            {
                Some(TSType::TSTypeLiteral(literal)) => Pair(
                    name,
                    Inner::List(literal.members.iter().map(member_info).collect()),
                ),
                _ => name.map_or(Info::Missing, Info::Name),
            }
        }
        TSSignature::TSMethodSignature(method) => {
            key_name(&method.key).map_or(Info::Missing, Info::Name)
        }
        _ => Info::Missing,
    }
}

use Info::Pair;

/// The flattened names a type literal annotation gives, if the annotation is one.
fn literal_names(annotation: Option<&TSTypeAnnotation>, name: Option<String>) -> Option<Flat> {
    let TSType::TSTypeLiteral(literal) = &annotation?.type_annotation else {
        return None;
    };
    let members: Vec<Info> = literal.members.iter().map(member_info).collect();
    Some(Flat {
        annotation: name,
        ..flatten(&members, "")
    })
}

/// The names of an array pattern's elements, by index, a rest element last.
fn array_elements(
    elements: &[Option<BindingPattern>],
    rest: Option<&BindingRestElement>,
    quoted: bool,
) -> Vec<Info> {
    (0..elements.len() + usize::from(rest.is_some()))
        .map(|index| Info::Index {
            name: if quoted {
                format!("\"{index}\"")
            } else {
                index.to_string()
            },
            rest: index == elements.len(),
        })
        .collect()
}

/// The source text a literal key is written with (`raw`).
fn literal_key(source: &Source, key: &PropertyKey) -> Option<String> {
    match key {
        PropertyKey::StringLiteral(_) | PropertyKey::NumericLiteral(_) => source
            .text
            .get(key.span().start as usize..key.span().end as usize)
            .map(ToOwned::to_owned),
        _ => None,
    }
}

/// A property of an object pattern (`getParamName` on a `Property`).
fn property_info(source: &Source, property: &BindingProperty) -> Info {
    let key = key_name(&property.key);
    let value = match &property.value {
        BindingPattern::AssignmentPattern(assignment) => &assignment.left,
        other => other,
    };
    match value {
        BindingPattern::ArrayPattern(array) => Pair(
            key,
            Inner::List(array_elements(
                &array.elements,
                array.rest.as_deref(),
                false,
            )),
        ),
        BindingPattern::ObjectPattern(object) => Pair(
            key,
            Inner::List(object_infos(
                source,
                &object.properties,
                object.rest.as_deref(),
            )),
        ),
        _ => key
            .map(Info::Name)
            .or_else(|| literal_key(source, &property.key).map(Info::Name))
            .unwrap_or(Info::Missing),
    }
}

/// The properties of an object pattern, its rest element last.
fn object_infos(
    source: &Source,
    properties: &[BindingProperty],
    rest: Option<&BindingRestElement>,
) -> Vec<Info> {
    let mut infos: Vec<Info> = properties
        .iter()
        .map(|property| property_info(source, property))
        .collect();
    if let Some(rest) = rest {
        infos.push(rest_info(rest, Some(true)));
    }
    infos
}

/// A rest element.
fn rest_info(rest: &BindingRestElement, property: Option<bool>) -> Info {
    let name = match &rest.argument {
        BindingPattern::BindingIdentifier(id) => RestName::Text(id.name.to_string()),
        BindingPattern::ArrayPattern(array) => RestName::List(
            array
                .elements
                .iter()
                .map(|element| match element {
                    Some(BindingPattern::BindingIdentifier(id)) => Some(id.name.to_string()),
                    _ => None,
                })
                .collect(),
        ),
        _ => RestName::Missing,
    };
    Info::Rest { name, property }
}

/// A parameter, seen as `ESTree` writes it: with a default, an assignment pattern whose left
/// side holds the type (`getParamName` at the top level).
fn parameter_info(
    source: &Source,
    pattern: &BindingPattern,
    annotation: Option<&TSTypeAnnotation>,
    assigned: bool,
) -> Info {
    let identifier = match pattern {
        BindingPattern::BindingIdentifier(id) => Some(id.name.to_string()),
        _ => None,
    };
    let annotated = if assigned { None } else { identifier.clone() };
    if let Some(flat) = literal_names(annotation, annotated) {
        return Pair(identifier, Inner::Flat(flat));
    }
    match pattern {
        BindingPattern::BindingIdentifier(id) => Info::Name(id.name.to_string()),
        BindingPattern::ObjectPattern(object) => Pair(
            None,
            Inner::Flat(flatten(
                &object_infos(source, &object.properties, object.rest.as_deref()),
                "",
            )),
        ),
        BindingPattern::ArrayPattern(array) => Pair(
            None,
            Inner::Flat(flatten(
                &array_elements(&array.elements, array.rest.as_deref(), true),
                "",
            )),
        ),
        BindingPattern::AssignmentPattern(assignment) => {
            parameter_info(source, &assignment.left, annotation, true)
        }
    }
}

/// The names of a function's parameters, `this` first when typed (`getFunctionParameterNames`).
#[must_use]
pub fn parameter_names(
    source: &Source,
    this: Option<&TSThisParameter>,
    params: &FormalParameters,
) -> Vec<Info> {
    let mut names = Vec::new();
    if let Some(this) = this {
        let flat = literal_names(this.type_annotation.as_deref(), Some("this".to_owned()));
        names.push(flat.map_or_else(
            || Info::Name("this".to_owned()),
            |flat| Pair(Some("this".to_owned()), Inner::Flat(flat)),
        ));
    }
    for item in &params.items {
        names.push(parameter_info(
            source,
            &item.pattern,
            item.type_annotation.as_deref(),
            item.initializer.is_some(),
        ));
    }
    if let Some(rest) = &params.rest {
        let annotation = rest.type_annotation.as_deref();
        names.push(literal_names(annotation, None).map_or_else(
            || rest_info(&rest.rest, None),
            |flat| Pair(None, Inner::Flat(flat)),
        ));
    }
    names
}
