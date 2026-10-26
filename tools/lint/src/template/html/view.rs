//! A template as html-validate's DOM holds a document (`src/dom/htmlelement.ts` of `html-validate`
//! 11.17.0, REF-HTML-VALIDATE-SOURCE), so that its rules read it as they read one: each element
//! named in lowercase and given its metadata, its attributes keyed in lowercase, its parent and its
//! children. Angular's own constructs are read through: a block's content (`@if`, `@for`,
//! `@switch`…), an `<ng-container>`'s and an ICU case's belong to the element around them, as they
//! render there; an `<ng-template>`'s content renders where it is instantiated, so its elements
//! start a tree of their own. A binding names an attribute whose value is dynamic (html-validate's
//! `DynamicValue`): `[attr.x]` the attribute `x`, `[x]` the attribute of the property `x` as
//! Angular's DOM schema maps them (`_ATTR_TO_PROP` of `@angular/compiler` 22.2.3), any other
//! property its name in lowercase, as HTML reflects it; `[class.x]`, `[style.x]`, events,
//! references and structural directives none. A foreign element's content is dropped, as
//! html-validate's parser drops it, but the `<title>` and `<desc>` of an `<svg>`, kept as its
//! children `svg:title` and `svg:desc`.

use super::elements::{Meta, meta_for};
use super::selector::Selector;
use crate::template::bindings::AttributeKind;
use crate::template::lexer::{Span, TokenType};
use crate::template::rules::r3::{R3, siblings};
use crate::template::rules::{Role, bound3, switch_groups};
use crate::template::tags::split_ns_name;
use crate::template::tree::{Attribute, Block, Element, Node, Text};

/// The DOM properties Angular's schema names apart from their attributes (`_ATTR_TO_PROP` of
/// `packages/compiler/src/schema/dom_element_schema_registry.ts` of `@angular/compiler` 22.2.3):
/// each attribute and its property.
const ATTR_TO_PROP: [(&str, &str); 55] = [
    ("class", "className"),
    ("for", "htmlFor"),
    ("formaction", "formAction"),
    ("innerHtml", "innerHTML"),
    ("readonly", "readOnly"),
    ("tabindex", "tabIndex"),
    ("aria-activedescendant", "ariaActiveDescendantElement"),
    ("aria-atomic", "ariaAtomic"),
    ("aria-autocomplete", "ariaAutoComplete"),
    ("aria-busy", "ariaBusy"),
    ("aria-checked", "ariaChecked"),
    ("aria-colcount", "ariaColCount"),
    ("aria-colindex", "ariaColIndex"),
    ("aria-colindextext", "ariaColIndexText"),
    ("aria-colspan", "ariaColSpan"),
    ("aria-controls", "ariaControlsElements"),
    ("aria-current", "ariaCurrent"),
    ("aria-describedby", "ariaDescribedByElements"),
    ("aria-description", "ariaDescription"),
    ("aria-details", "ariaDetailsElements"),
    ("aria-disabled", "ariaDisabled"),
    ("aria-errormessage", "ariaErrorMessageElements"),
    ("aria-expanded", "ariaExpanded"),
    ("aria-flowto", "ariaFlowToElements"),
    ("aria-haspopup", "ariaHasPopup"),
    ("aria-hidden", "ariaHidden"),
    ("aria-invalid", "ariaInvalid"),
    ("aria-keyshortcuts", "ariaKeyShortcuts"),
    ("aria-label", "ariaLabel"),
    ("aria-labelledby", "ariaLabelledByElements"),
    ("aria-level", "ariaLevel"),
    ("aria-live", "ariaLive"),
    ("aria-modal", "ariaModal"),
    ("aria-multiline", "ariaMultiLine"),
    ("aria-multiselectable", "ariaMultiSelectable"),
    ("aria-orientation", "ariaOrientation"),
    ("aria-owns", "ariaOwnsElements"),
    ("aria-placeholder", "ariaPlaceholder"),
    ("aria-posinset", "ariaPosInSet"),
    ("aria-pressed", "ariaPressed"),
    ("aria-readonly", "ariaReadOnly"),
    ("aria-required", "ariaRequired"),
    ("aria-roledescription", "ariaRoleDescription"),
    ("aria-rowcount", "ariaRowCount"),
    ("aria-rowindex", "ariaRowIndex"),
    ("aria-rowindextext", "ariaRowIndexText"),
    ("aria-rowspan", "ariaRowSpan"),
    ("aria-selected", "ariaSelected"),
    ("aria-setsize", "ariaSetSize"),
    ("aria-sort", "ariaSort"),
    ("aria-valuemax", "ariaValueMax"),
    ("aria-valuemin", "ariaValueMin"),
    ("aria-valuenow", "ariaValueNow"),
    ("aria-valuetext", "ariaValueText"),
    ("aria-relevant", "ariaRelevant"),
];

/// An attribute's value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// Written without a value (`disabled`).
    Omitted,
    /// Written: its text as in the source, entities not decoded, as html-validate reads it.
    Static(String),
    /// Bound or interpolated: known only at run time.
    Dynamic,
}

/// An attribute of an element.
#[derive(Clone, Debug)]
pub struct Attr<'t> {
    /// Its name, in lowercase.
    pub key: String,
    /// Its name as written.
    pub written: String,
    /// Its value.
    pub value: Value,
    /// Whether a property binding (`[x]`, `[attr.x]`, `[(x)]`), an alias of the attribute
    /// rather than the attribute written (html-validate's `originalAttribute`).
    pub bound: bool,
    /// The attribute written.
    pub attribute: &'t Attribute,
}

impl Attr<'_> {
    /// Its static value, `None` when omitted or dynamic.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match &self.value {
            Value::Static(text) => Some(text),
            Value::Omitted | Value::Dynamic => None,
        }
    }

    /// Whether bound or interpolated.
    #[must_use]
    pub fn is_dynamic(&self) -> bool {
        self.value == Value::Dynamic
    }

    /// Its value without the quotes, leading whitespace included (html-validate's
    /// `valueLocation`), or its name when the value is omitted or empty, which html-validate
    /// does not place.
    #[must_use]
    pub fn value_span(&self) -> Span {
        self.attribute
            .value_span
            .filter(|span| span.end > span.full_start)
            .map_or(self.attribute.key_span, |span| {
                Span::new(span.full_start as usize, span.end as usize)
            })
    }

    /// Its name (html-validate's `keyLocation`).
    #[must_use]
    pub const fn key_span(&self) -> Span {
        self.attribute.key_span
    }

    /// The whole attribute, from its name to its closing quote (html-validate's `location`).
    #[must_use]
    pub const fn span(&self) -> Span {
        self.attribute.span
    }

    /// The quote around its value, when one.
    #[must_use]
    pub fn quote(&self, text: &str) -> Option<char> {
        let span = self.attribute.value_span?;
        let before = text.get(..span.full_start as usize)?.chars().next_back()?;
        matches!(before, '"' | '\'').then_some(before)
    }
}

/// A child of an element: an element, or a text.
#[derive(Clone, Copy, Debug)]
pub enum Child<'t> {
    /// The element at this index.
    Element(usize),
    /// A text.
    Text(&'t Text),
}

/// An element.
#[derive(Debug)]
pub struct El<'t> {
    /// Its name in lowercase, `svg:title` for an `<svg>`'s `<title>`.
    pub tag: String,
    /// The element written.
    pub element: &'t Element,
    /// Its parent element.
    pub parent: Option<usize>,
    /// Its children, in order.
    pub children: Vec<Child<'t>>,
    /// Its attributes, in order.
    pub attrs: Vec<Attr<'t>>,
    /// Its metadata, `None` for an element HTML does not know (a component).
    pub meta: Option<&'static Meta>,
    /// The branches it is written in: each chain of exclusive branches (an `@if` and its
    /// `@else`s, a `@switch`'s cases, a `@for` and its `@empty`) by its index, and the branch's.
    pub branches: Vec<(usize, usize)>,
    /// Whether inside a `@for`'s body, written once and rendered many times.
    pub repeated: bool,
    /// Whether inside an `<ng-template>`, rendered where it is instantiated: its ancestors there
    /// unknown.
    pub in_template: bool,
    /// The tree it belongs to: 0 for the template's, another number for each `<ng-template>`'s.
    pub scope: usize,
}

/// A template read as html-validate's DOM.
#[derive(Debug)]
pub struct Doc<'t> {
    /// The template's text.
    pub text: &'t str,
    /// Its elements, in the order written.
    pub elements: Vec<El<'t>>,
    /// The children of its root, and of each `<ng-template>`.
    pub roots: Vec<Child<'t>>,
}

/// Where the walk adds what it meets.
#[derive(Clone, Default)]
struct Place {
    /// The element around, `None` at a root.
    parent: Option<usize>,
    /// The branches around.
    branches: Vec<(usize, usize)>,
    /// Whether inside a `@for`'s body.
    repeated: bool,
    /// Whether inside a foreign element, whose content is dropped.
    foreign: Option<usize>,
    /// Whether inside an `<ng-template>`.
    in_template: bool,
    /// The tree it belongs to.
    scope: usize,
}

impl<'t> Doc<'t> {
    /// Reads a template's nodes.
    #[must_use]
    pub fn new(text: &'t str, nodes: &'t [Node]) -> Self {
        let mut doc = Self {
            text,
            elements: Vec::new(),
            roots: Vec::new(),
        };
        let mut chains = 0;
        doc.add_nodes(nodes, &Place::default(), &mut chains);
        doc
    }

    /// Adds a list of siblings.
    fn add_nodes(&mut self, nodes: &'t [Node], place: &Place, chains: &mut usize) {
        let mut branch_of: Vec<(&'t Block, (usize, usize))> = Vec::new();
        for sibling in siblings(nodes) {
            match sibling {
                R3::If(branches) => {
                    for (index, block) in branches.into_iter().enumerate() {
                        branch_of.push((block, (*chains, index)));
                    }
                    *chains += 1;
                }
                R3::For(body, empty) => {
                    branch_of.push((body, (*chains, 0)));
                    if let Some(empty) = empty {
                        branch_of.push((empty, (*chains, 1)));
                    }
                    *chains += 1;
                }
                R3::Element(_) | R3::Text(_) | R3::Other(_) => {}
            }
        }
        for node in nodes {
            match node {
                Node::Element(element) => self.add_element(element, place, chains),
                Node::Text(text) if place.foreign.is_none() => {
                    self.push_child(place, Child::Text(text));
                }
                Node::Block(block) => {
                    let mut inner = place.clone();
                    if let Some((_, branch)) = branch_of
                        .iter()
                        .find(|(each, _)| std::ptr::eq(*each, block))
                    {
                        inner.branches.push(*branch);
                    }
                    if block.name == "for" {
                        inner.repeated = true;
                    }
                    if block.name == "switch" {
                        let chain = *chains;
                        *chains += 1;
                        for (index, group) in switch_groups(block).groups.iter().enumerate() {
                            let mut case = inner.clone();
                            case.branches.push((chain, index));
                            self.add_nodes(&group.body.children, &case, chains);
                        }
                    } else {
                        self.add_nodes(&block.children, &inner, chains);
                    }
                }
                Node::Expansion(expansion) => {
                    for case in &expansion.cases {
                        self.add_nodes(&case.expression, place, chains);
                    }
                }
                Node::Text(_) | Node::Comment(_) | Node::LetDeclaration(_) => {}
            }
        }
    }

    /// Adds a child where the walk is.
    fn push_child(&mut self, place: &Place, child: Child<'t>) {
        match place.parent {
            Some(parent) => self.elements[parent].children.push(child),
            None => self.roots.push(child),
        }
    }

    /// Adds an element, or its content for an `<ng-container>` or an `<ng-template>`.
    fn add_element(&mut self, element: &'t Element, place: &Place, chains: &mut usize) {
        let (namespace, name) = split_ns_name(&element.name);
        if place.foreign.is_some() {
            // html-validate keeps an `<svg>`'s `<title>` and `<desc>`, its other content dropped.
            if namespace == Some("svg") && matches!(name, "title" | "desc") {
                let tag = format!("svg:{}", name.to_lowercase());
                let kept = Place {
                    parent: place.foreign,
                    foreign: None,
                    ..place.clone()
                };
                self.push_element(element, tag, &kept, chains);
            } else {
                self.add_nodes(&element.children, place, chains);
            }
            return;
        }
        match (namespace, name) {
            (None, "ng-container") => self.add_nodes(&element.children, place, chains),
            (None, "ng-template") => {
                *chains += 1;
                let root = Place {
                    parent: None,
                    foreign: None,
                    in_template: true,
                    scope: *chains,
                    ..place.clone()
                };
                self.add_nodes(&element.children, &root, chains);
            }
            _ => self.push_element(element, name.to_lowercase(), place, chains),
        }
    }

    /// Adds an element and its children.
    fn push_element(
        &mut self,
        element: &'t Element,
        tag: String,
        place: &Place,
        chains: &mut usize,
    ) {
        let index = self.elements.len();
        let meta = meta_for(&tag);
        self.elements.push(El {
            meta,
            tag,
            element,
            parent: place.parent,
            children: Vec::new(),
            attrs: attributes(self.text, element),
            branches: place.branches.clone(),
            repeated: place.repeated,
            in_template: place.in_template,
            scope: place.scope,
        });
        self.push_child(place, Child::Element(index));
        let inner = Place {
            parent: Some(index),
            foreign: meta.filter(|meta| meta.foreign).map(|_| index),
            ..place.clone()
        };
        self.add_nodes(&element.children, &inner, chains);
    }

    /// Whether two elements are in exclusive branches of one chain (an `@if` and its `@else`,
    /// two cases of a `@switch`, a `@for` and its `@empty`): never rendered together.
    #[must_use]
    pub fn exclusive(&self, one: usize, other: usize) -> bool {
        self.elements[one].branches.iter().any(|(chain, branch)| {
            self.elements[other]
                .branches
                .iter()
                .any(|(other_chain, other_branch)| chain == other_chain && branch != other_branch)
        })
    }

    /// The element at an index.
    #[must_use]
    pub fn el(&self, index: usize) -> &El<'t> {
        &self.elements[index]
    }

    /// An element's first attribute of a name, in any case.
    #[must_use]
    pub fn attr(&self, index: usize, key: &str) -> Option<&Attr<'t>> {
        self.attrs(index, key).next()
    }

    /// An element's attributes of a name, in any case.
    pub fn attrs<'d>(&'d self, index: usize, key: &str) -> impl Iterator<Item = &'d Attr<'t>> {
        let key = key.to_lowercase();
        self.elements[index]
            .attrs
            .iter()
            .filter(move |attr| attr.key == key)
    }

    /// Whether an element has an attribute.
    #[must_use]
    pub fn has(&self, index: usize, key: &str) -> bool {
        self.attr(index, key).is_some()
    }

    /// An element's static attribute value, `None` when missing, omitted or dynamic.
    #[must_use]
    pub fn value(&self, index: usize, key: &str) -> Option<&str> {
        self.attr(index, key).and_then(Attr::text)
    }

    /// An element's child elements.
    #[must_use]
    pub fn child_elements(&self, index: usize) -> Vec<usize> {
        elements_of(&self.elements[index].children)
    }

    /// The elements at the roots.
    #[must_use]
    pub fn root_elements(&self) -> Vec<usize> {
        elements_of(&self.roots)
    }

    /// An element's siblings, itself among them: its parent's child elements, or the roots'.
    #[must_use]
    pub fn siblings(&self, index: usize) -> Vec<usize> {
        match self.elements[index].parent {
            Some(parent) => self.child_elements(parent),
            None => self.root_elements(),
        }
    }

    /// An element's descendants, depth first.
    #[must_use]
    pub fn descendants(&self, index: usize) -> Vec<usize> {
        let mut found = Vec::new();
        for child in self.child_elements(index) {
            found.push(child);
            found.extend(self.descendants(child));
        }
        found
    }

    /// An element's ancestors, its parent first.
    #[must_use]
    pub fn ancestors(&self, index: usize) -> Vec<usize> {
        let mut found = Vec::new();
        let mut current = self.elements[index].parent;
        while let Some(parent) = current {
            found.push(parent);
            current = self.elements[parent].parent;
        }
        found
    }

    /// Whether an element matches a selector list.
    #[must_use]
    pub fn matches(&self, index: usize, selectors: &str) -> bool {
        Selector::parse(selectors).matches(self, index)
    }

    /// The element itself or its closest ancestor matching a selector list.
    #[must_use]
    pub fn closest(&self, index: usize, selectors: &str) -> Option<usize> {
        let selector = Selector::parse(selectors);
        std::iter::once(index)
            .chain(self.ancestors(index))
            .find(|each| selector.matches(self, *each))
    }

    /// Whether an element is named so, in any case, `*` naming any.
    #[must_use]
    pub fn is(&self, index: usize, tag: &str) -> bool {
        tag == "*" || self.elements[index].tag.eq_ignore_ascii_case(tag)
    }

    /// The span html-validate places an element's report at: its name in its start tag.
    #[must_use]
    pub fn name_span(&self, index: usize) -> Span {
        let start = self.elements[index].element.start_span.start as usize + 1;
        let rest = self.text.get(start..).unwrap_or_default();
        let length = rest
            .find(|character: char| character.is_whitespace() || matches!(character, '/' | '>'))
            .unwrap_or(rest.len());
        Span::new(start, start + length)
    }

    /// An element's text, its descendants' included, entities decoded, interpolations left out;
    /// with whether an interpolation, a bound `innerHTML`, `innerText` or `textContent` or a
    /// component makes it known only at run time.
    #[must_use]
    pub fn text_content(&self, index: usize) -> (String, bool) {
        let mut text = String::new();
        let mut dynamic = false;
        self.collect_text(&self.elements[index].children, &mut text, &mut dynamic);
        (text, dynamic)
    }

    /// Adds the text of children.
    fn collect_text(&self, children: &[Child], text: &mut String, dynamic: &mut bool) {
        for child in children {
            match child {
                Child::Text(node) => {
                    for token in &node.tokens {
                        match token.kind {
                            TokenType::Interpolation => *dynamic = true,
                            TokenType::Text | TokenType::EncodedEntity => {
                                text.push_str(token.parts.first().map_or("", String::as_str));
                            }
                            _ => {}
                        }
                    }
                }
                Child::Element(index) => {
                    self.collect_text(&self.elements[*index].children, text, dynamic);
                }
            }
        }
    }
}

/// The elements among children.
fn elements_of(children: &[Child]) -> Vec<usize> {
    children
        .iter()
        .filter_map(|child| match child {
            Child::Element(index) => Some(*index),
            Child::Text(_) => None,
        })
        .collect()
}

/// The attribute a property binds: Angular's for those its schema maps, its name in lowercase
/// for any other.
fn property_attribute(property: &str) -> String {
    ATTR_TO_PROP
        .iter()
        .find(|(attribute, each)| *each == property || *attribute == property)
        .map_or_else(
            || property.to_lowercase(),
            |(attribute, _)| (*attribute).to_lowercase(),
        )
}

/// An element's attributes as html-validate reads them, in the order written.
fn attributes<'t>(text: &str, element: &'t Element) -> Vec<Attr<'t>> {
    let mut found: Vec<Attr> = bound3(element)
        .into_iter()
        .filter_map(|each| {
            let key = match (each.role, each.kind) {
                (Role::Text, _) => match split_ns_name(&each.name) {
                    (Some(namespace), name) => format!("{namespace}:{name}"),
                    (None, name) => name.to_owned(),
                },
                (Role::Input, AttributeKind::Plain) => each.name.clone(),
                (Role::Input, AttributeKind::Property | AttributeKind::TwoWay)
                    if !each.original.starts_with("class.")
                        && !each.original.starts_with("style.") =>
                {
                    let inner = crate::template::bindings::attribute_kind(&each.attribute.name).1;
                    match inner.strip_prefix("attr.") {
                        Some(attribute) => attribute.to_owned(),
                        None => property_attribute(&inner),
                    }
                }
                _ => return None,
            };
            let value = match (each.role, each.attribute.value_span) {
                // `x=""`: quotes and no text.
                (Role::Text, None) if each.attribute.span.end > each.attribute.key_span.end => {
                    Value::Static(String::new())
                }
                (Role::Text, None) => Value::Omitted,
                (Role::Text, Some(span)) => Value::Static(
                    text.get(span.full_start as usize..span.end as usize)
                        .unwrap_or_default()
                        .to_owned(),
                ),
                _ => Value::Dynamic,
            };
            Some(Attr {
                key: key.to_lowercase(),
                written: key,
                value,
                bound: matches!(each.kind, AttributeKind::Property | AttributeKind::TwoWay),
                attribute: each.attribute,
            })
        })
        .collect();
    found.sort_by_key(|attr| attr.attribute.span.start);
    found
}

#[cfg(test)]
mod tests {
    use super::{Doc, Value};
    use crate::template::tree::parse;

    #[test]
    fn reads_angular_s_constructs_through() {
        // Our own case: a block and an `<ng-container>` are read through, an `<ng-template>`'s
        // content starts a tree of its own, an `<svg>` keeps its `<title>` only.
        let text = "<ul>@if (a) {<ng-container><li></li></ng-container>}</ul>\
            <ng-template><li></li></ng-template>\
            <svg><g><title>t</title><path /></g></svg>";
        let parsed = parse(text);
        let doc = Doc::new(text, &parsed.nodes);
        let tags: Vec<(&str, Option<usize>)> = doc
            .elements
            .iter()
            .map(|element| (element.tag.as_str(), element.parent))
            .collect();

        assert_eq!(
            tags,
            [
                ("ul", None),
                ("li", Some(0)),
                ("li", None),
                ("svg", None),
                ("svg:title", Some(3))
            ]
        );
        assert!(doc.el(2).in_template && !doc.el(1).in_template);
        assert_eq!(doc.root_elements(), [0, 2, 3]);
        assert_eq!(doc.siblings(1), [1]);
    }

    #[test]
    fn names_a_binding_s_attribute_as_angular_s_schema_does() {
        // Our own case: `[attr.x]` binds `x`, `[htmlFor]` and `[for]` the attribute `for`,
        // `[readOnly]` `readonly`, `[class.x]` and events none; an interpolated value is dynamic,
        // a written one kept as in the source, entities included.
        let text = "<input [attr.aria-label]=\"a\" [htmlFor]=\"b\" [readOnly]=\"c\" [class.x]=\"d\" \
            (click)=\"e()\" title=\"{{ f }}\" value=\"a&amp;b\" disabled id=\"\">";
        let parsed = parse(text);
        let doc = Doc::new(text, &parsed.nodes);
        let attrs: Vec<(&str, &Value)> = doc
            .el(0)
            .attrs
            .iter()
            .map(|attr| (attr.key.as_str(), &attr.value))
            .collect();

        assert_eq!(
            attrs,
            [
                ("aria-label", &Value::Dynamic),
                ("for", &Value::Dynamic),
                ("readonly", &Value::Dynamic),
                ("title", &Value::Dynamic),
                ("value", &Value::Static("a&amp;b".to_owned())),
                ("disabled", &Value::Omitted),
                ("id", &Value::Static(String::new())),
            ]
        );
        assert_eq!(
            doc.attr(0, "id").map(super::Attr::value_span),
            doc.attr(0, "id").map(super::Attr::key_span)
        );
        assert_eq!(
            doc.attr(0, "value").and_then(|attr| attr.quote(text)),
            Some('"')
        );
    }

    #[test]
    fn collects_an_element_s_text() {
        // Our own case: entities decoded, an interpolation makes the text dynamic.
        let text = "<p>a &amp; <b>b</b>{{ c }}</p>";
        let parsed = parse(text);
        let doc = Doc::new(text, &parsed.nodes);

        assert_eq!(doc.text_content(0), ("a & b".to_owned(), true));
        assert_eq!(doc.descendants(0), [1]);
        assert_eq!(doc.ancestors(1), [0]);
    }
}
