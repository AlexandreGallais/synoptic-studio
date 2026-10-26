//! The template rules (F52, EN-164): the rules of `@angular-eslint/eslint-plugin-template`
//! 22.6.0 the catalogue keeps (REF-ANGULAR-ESLINT-TEMPLATE-SOURCE), each one module reading the
//! reader's tree and its expressions as the plugin reads Angular's template AST, its name and its
//! messages the plugin's, so that findings compare one for one. A rule runs only on a template
//! that reads cleanly, as the plugin's parser throws on the first error of Angular's: a template
//! that does not read is reported under `lint/parse` instead. Each rule is tested on every case
//! of its file of the plugin's tests (`tester`), counted.

pub mod a11y;
pub mod alt_text;
pub mod aria;
pub mod attributes_order;
pub mod ax_objects;
pub mod banana_in_box;
pub mod button_has_type;
pub mod click_events_have_key_events;
pub mod conditional_complexity;
pub mod cyclomatic_complexity;
pub mod dom;
pub mod elements_content;
pub mod eqeqeq;
pub mod interactive_supports_focus;
pub mod label_has_associated_control;
pub mod mouse_events_have_key_events;
pub mod no_any;
pub mod no_autofocus;
pub mod no_call_expression;
pub mod no_distracting_elements;
pub mod no_duplicate_attributes;
pub mod no_empty_control_flow;
pub mod no_inline_styles;
pub mod no_interpolation_in_attributes;
pub mod no_negated_async;
pub mod no_nested_tags;
pub mod no_non_null_assertion;
pub mod no_outerhtml;
pub mod no_positive_tabindex;
pub mod prefer_at_else;
pub mod prefer_at_empty;
pub mod prefer_built_in_pipes;
pub mod prefer_class_binding;
pub mod prefer_contextual_for_variables;
pub mod prefer_control_flow;
pub mod prefer_ngsrc;
pub mod prefer_self_closing_tags;
pub mod prefer_static_string_properties;
pub mod prefer_style_binding;
pub mod prefer_template_literal;
pub mod r3;
pub mod require_switch_default;
pub mod role_has_required_aria;
pub mod table_scope;
#[cfg(test)]
pub mod tester;
pub mod use_track_by_function;
pub mod valid_aria;

use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_ast_visit::Visit;
use regex::Regex;

use super::bindings::{
    AttributeKind, TemplateBinding, Value, attribute_kind, block_parameters, interpolations,
    is_non_bindable, read_attribute,
};
use super::expressions::{Bound, read};
use super::lexer::{Span, TokenType};
use super::tags::split_ns_name;
use super::tree::{Attribute, Block, Element, Node};
use crate::finding::Fix;

/// A template rule.
pub trait TemplateRule: Sync {
    /// Its name, the plugin's prefixed by `@angular-eslint/template/`.
    fn name(&self) -> &'static str;

    /// Its messages by id, as the plugin writes them, `{{name}}` filled from a report's data.
    fn messages(&self) -> &'static [(&'static str, &'static str)];

    /// Checks a template.
    fn check(&self, template: &Template, reports: &mut Vec<Report>);
}

/// What a rule reports: where, which message with which data, and its fix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    /// From the first byte reported to the byte after the last.
    pub span: Span,
    /// The message's id.
    pub message_id: &'static str,
    /// The values of the message's `{{name}}`s.
    pub data: Vec<(&'static str, String)>,
    /// The fix, for a rule the plugin fixes.
    pub fix: Option<Fix>,
}

impl Report {
    /// A report without data nor fix.
    #[must_use]
    pub const fn new(span: Span, message_id: &'static str) -> Self {
        Self {
            span,
            message_id,
            data: Vec::new(),
            fix: None,
        }
    }
}

/// A template read cleanly: its text, its tree, and the arena its expressions are read into.
pub struct Template<'t> {
    /// The file, from the root.
    pub path: &'t str,
    /// The text.
    pub text: &'t str,
    /// The nodes at its root.
    pub nodes: &'t [Node],
    /// The arena of the expressions read.
    pub allocator: &'t Allocator,
}

/// Where an expression is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Site {
    /// An attribute's value: a binding or an interpolation.
    Attribute,
    /// An event's statement.
    Event,
    /// A template binding's expression (`*ngIf="…"`).
    TemplateBinding,
    /// An interpolation of a text.
    Text,
    /// A block's parameter: a condition, a collection, a `track`, a `when`.
    Block,
    /// A `@let`'s value.
    Let,
    /// An ICU expansion's switched value.
    Expansion,
}

/// An expression of the template and where it is written.
#[derive(Debug)]
pub struct Written<'a> {
    /// The expression and its pipes.
    pub bound: Bound<'a>,
    /// Where.
    pub site: Site,
}

impl<'t> Template<'t> {
    /// Visits every node, each with its ancestors from the root, in the order written.
    pub fn walk(&self, visit: &mut dyn FnMut(&'t Node, &[&'t Node])) {
        walk_nodes(self.nodes, &mut Vec::new(), visit);
    }

    /// Every list of siblings: the root's, then each element's, block's and expansion case's
    /// children, in the order written.
    #[must_use]
    pub fn lists(&self) -> Vec<&'t [Node]> {
        let mut found = vec![self.nodes];
        self.walk(&mut |node, _| match node {
            Node::Element(element) => found.push(&element.children),
            Node::Block(block) => found.push(&block.children),
            Node::Expansion(expansion) => {
                for case in &expansion.cases {
                    found.push(&case.expression);
                }
            }
            Node::Text(_) | Node::Comment(_) | Node::LetDeclaration(_) => {}
        });
        found
    }

    /// Every element, with its ancestors.
    #[must_use]
    pub fn elements(&self) -> Vec<(&'t Element, Vec<&'t Node>)> {
        let mut found = Vec::new();
        self.walk(&mut |node, ancestors| {
            if let Node::Element(element) = node {
                found.push((element, ancestors.to_vec()));
            }
        });
        found
    }

    /// Every expression of the template, read, none under an `ngNonBindable` element, which binds
    /// nothing; what cannot be read was reported by the reader.
    #[must_use]
    pub fn expressions(&self) -> Vec<Written<'t>> {
        let mut found = Vec::new();
        self.walk(&mut |node, ancestors| {
            let bound = ancestors.iter().all(
                |ancestor| !matches!(ancestor, Node::Element(element) if is_non_bindable(element)),
            );
            if bound {
                self.read_node(node, &mut found);
            }
        });
        found
    }

    /// The expressions a node writes itself, its children aside.
    fn read_node(&self, node: &Node, found: &mut Vec<Written<'t>>) {
        let allocator = self.allocator;
        let mut push = |bound, site| found.push(Written { bound, site });
        match node {
            Node::Element(element) => {
                for attribute in &element.attrs {
                    let Ok(read) = read_attribute(allocator, &element.name, attribute) else {
                        continue;
                    };
                    match read.value {
                        Value::Binding(bound) => push(bound, Site::Attribute),
                        Value::Action(bounds) => {
                            for bound in bounds {
                                push(bound, Site::Event);
                            }
                        }
                        Value::Interpolations(bounds) => {
                            for bound in bounds {
                                push(bound, Site::Attribute);
                            }
                        }
                        Value::TemplateBindings(bindings) => {
                            for binding in bindings {
                                if let TemplateBinding::Expression {
                                    value: Some(bound), ..
                                } = binding
                                {
                                    push(bound, Site::TemplateBinding);
                                }
                            }
                        }
                        Value::Text => {}
                    }
                }
            }
            Node::Text(text) => {
                for bound in interpolations(allocator, &text.tokens).unwrap_or_default() {
                    push(bound, Site::Text);
                }
            }
            Node::Block(block) => {
                if let Ok(parameters) = block_parameters(allocator, block) {
                    let all = parameters
                        .expression
                        .into_iter()
                        .chain(parameters.track)
                        .chain(parameters.conditions);
                    for bound in all {
                        push(bound, Site::Block);
                    }
                }
            }
            Node::LetDeclaration(declaration) => {
                if let Ok(bound) = read(allocator, &declaration.value, declaration.value_span.start)
                {
                    push(bound, Site::Let);
                }
            }
            Node::Expansion(expansion) => {
                if let Ok(bound) = read(
                    allocator,
                    &expansion.switch_value,
                    expansion.switch_value_span.start,
                ) {
                    push(bound, Site::Expansion);
                }
            }
            Node::Comment(_) => {}
        }
    }
}

/// What an attribute is on Angular's element (`t.Element` of `render3/r3_ast.ts`): one of its
/// inputs, its outputs or its plain attributes; a template binding, a reference and a variable
/// are none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// A `BoundAttribute`: `[x]`, `bind-x`, `[(x)]`, `@x`, or a value that interpolates.
    Input,
    /// A `BoundEvent`: `(x)`, `on-x`, or the `xChange` of `[(x)]`.
    Output,
    /// A `TextAttribute`: a plain attribute.
    Text,
}

/// An attribute as Angular's element holds it.
#[derive(Clone, Debug)]
pub struct Bound3<'t> {
    /// Its role.
    pub role: Role,
    /// Its kind, as written.
    pub kind: AttributeKind,
    /// Its name without delimiters (`name` of `[name]`, `ngModelChange` of the output of
    /// `[(ngModel)]`, `click` of `(window:click)`).
    pub name: String,
    /// Its name as `getOriginalAttributeName` gives it: the name written inside the delimiters,
    /// `attr.` removed (`style.width.px`, `role` of `[attr.role]`, `window:click`), the output of
    /// a two-way binding its `xChange`.
    pub original: String,
    /// The attribute.
    pub attribute: &'t Attribute,
}

/// The inputs, outputs and plain attributes of an element, in Angular's order: the inputs, then
/// the outputs, then the plain attributes, each in the order written; a two-way binding gives
/// an input, then an output of the same source span.
#[must_use]
pub fn bound3(element: &Element) -> Vec<Bound3<'_>> {
    let (mut inputs, mut outputs, mut texts) = (Vec::new(), Vec::new(), Vec::new());
    for attribute in &element.attrs {
        let (kind, inner) = attribute_kind(&attribute.name);
        let each = |role, name: String, original: String| Bound3 {
            role,
            kind,
            name,
            original,
            attribute,
        };
        match kind {
            AttributeKind::Property | AttributeKind::Animation => {
                inputs.push(each(
                    Role::Input,
                    property_name(&inner),
                    inner.replacen("attr.", "", 1),
                ));
            }
            AttributeKind::TwoWay => {
                inputs.push(each(
                    Role::Input,
                    property_name(&inner),
                    inner.replacen("attr.", "", 1),
                ));
                let change = format!("{inner}Change");
                outputs.push(each(Role::Output, change.clone(), change));
            }
            AttributeKind::Event => {
                let name = inner.rsplit(':').next().unwrap_or(&inner).to_owned();
                outputs.push(each(Role::Output, name, inner.clone()));
            }
            AttributeKind::Plain if interpolates(attribute) => {
                inputs.push(each(Role::Input, inner.clone(), inner.clone()));
            }
            AttributeKind::Plain => texts.push(each(Role::Text, inner.clone(), inner.clone())),
            AttributeKind::Interpolated
            | AttributeKind::Template
            | AttributeKind::Reference
            | AttributeKind::Variable => {}
        }
    }
    inputs.extend(outputs);
    inputs.extend(texts);
    inputs
}

/// The name of a bound property as Angular's binding parser gives it: an animation's without its
/// `@`, `x` of `attr.x`, of `class.x` and of `style.x.unit`, any other as written.
#[must_use]
pub fn property_name(inner: &str) -> String {
    let inner = inner.strip_prefix('@').unwrap_or(inner);
    if let Some(attribute) = inner.strip_prefix("attr.") {
        return attribute.to_owned();
    }
    match inner.split('.').collect::<Vec<_>>().as_slice() {
        ["class" | "style", name, ..] => (*name).to_owned(),
        _ => inner.to_owned(),
    }
}

/// The span of the name an attribute binds, its prefix and delimiters left out (the `keySpan` of
/// Angular's bound attributes and events): `x` of `[x]`, `bind-x`, `[(x)]`, `(x)` and `on-x`.
#[must_use]
pub fn key_span(attribute: &Attribute) -> Span {
    let (kind, inner) = attribute_kind(&attribute.name);
    let suffix = match (kind, attribute.name.as_bytes().first()) {
        (AttributeKind::TwoWay, Some(b'[')) => 2,
        (AttributeKind::Property, Some(b'[')) | (AttributeKind::Event, Some(b'(')) => 1,
        _ => 0,
    };
    let start = attribute.key_span.start as usize + attribute.name.len() - suffix - inner.len();
    Span::new(start, start + inner.len())
}

/// A bound attribute of Angular's tree (`BoundAttribute`): an input of an element or a
/// template, or a binding of a structural directive's microsyntax (`*ngIf="…"`).
#[derive(Debug)]
pub struct BoundAttribute<'t> {
    /// Its name: the input's (`property_name`), or the binding's key (`ngIf`, `ngForOf`).
    pub name: String,
    /// Its source span: the attribute, or the binding from its key to the next binding.
    pub span: Span,
    /// The attribute written.
    pub attribute: &'t Attribute,
    /// Whether a binding of a microsyntax.
    pub microsyntax: bool,
}

/// The bound attributes of an element, its inputs first, then its microsyntax's bindings.
#[must_use]
pub fn bound_attributes<'t>(
    allocator: &Allocator,
    element: &'t Element,
) -> Vec<BoundAttribute<'t>> {
    let mut found: Vec<BoundAttribute> = bound3(element)
        .into_iter()
        .filter(|each| each.role == Role::Input)
        .map(|each| BoundAttribute {
            name: each.name,
            span: each.attribute.span,
            attribute: each.attribute,
            microsyntax: false,
        })
        .collect();
    found.extend(
        template_attrs(allocator, element)
            .into_iter()
            .filter(|each| each.value.is_some())
            .map(|each| BoundAttribute {
                name: each.name,
                span: each.span,
                attribute: each.attribute,
                microsyntax: true,
            }),
    );
    found
}

/// A binding of a structural directive's microsyntax (`templateAttrs` of Angular's template):
/// a bound attribute when it has an expression, a plain one otherwise.
#[derive(Debug)]
pub struct TemplateAttr<'t, 'a> {
    /// Its key: the directive's name first, then `ngForOf`, `ngForTrackBy`…
    pub name: String,
    /// From its key to the next binding.
    pub span: Span,
    /// Its expression.
    pub value: Option<Bound<'a>>,
    /// The attribute written.
    pub attribute: &'t Attribute,
}

/// The bindings of an element's structural directive, in the order written; a reading error
/// was reported by the reader.
#[must_use]
pub fn template_attrs<'t, 'a>(
    allocator: &'a Allocator,
    element: &'t Element,
) -> Vec<TemplateAttr<'t, 'a>> {
    let mut found = Vec::new();
    for attribute in &element.attrs {
        let Ok(read) = read_attribute(allocator, &element.name, attribute) else {
            continue;
        };
        let Value::TemplateBindings(bindings) = read.value else {
            continue;
        };
        for binding in bindings {
            if let TemplateBinding::Expression {
                key,
                value,
                key_start,
                end,
            } = binding
            {
                let start = key_start.unwrap_or(attribute.key_span.start + 1);
                found.push(TemplateAttr {
                    name: key,
                    span: Span::new(start as usize, end as usize),
                    value,
                    attribute,
                });
            }
        }
    }
    found
}

/// Whether an attribute's value holds a closed interpolation, which makes it an input.
fn interpolates(attribute: &Attribute) -> bool {
    attribute
        .value_tokens
        .iter()
        .any(|token| token.kind == TokenType::AttrValueInterpolation && token.parts.len() == 3)
}

/// Whether a name matches an entry of `ignoreWithDirectives`: a pattern `/…/flags`, its `i`,
/// `m` and `s` flags kept, or the name itself; a pattern Rust's `regex` cannot read matches
/// nothing.
fn matches_directive(entry: &str, name: &str) -> bool {
    let pattern = entry
        .strip_prefix('/')
        .and_then(|rest| rest.rsplit_once('/'))
        .filter(|(pattern, flags)| {
            !pattern.is_empty() && flags.chars().all(|flag| flag.is_ascii_lowercase())
        });
    match pattern {
        Some((pattern, flags)) => {
            let inline: String = flags.chars().filter(|flag| "ims".contains(*flag)).collect();
            let written = if inline.is_empty() {
                pattern.to_owned()
            } else {
                format!("(?{inline}){pattern}")
            };
            Regex::new(&written).is_ok_and(|regex| regex.is_match(name))
        }
        None => entry == name,
    }
}

/// Whether an element carries one of `ignoreWithDirectives` (a name, or a pattern written
/// `/…/flags`) among its inputs and plain attributes (`createIgnoredDirectiveMatcher` of
/// `src/utils/has-ignored-directive.ts` of `@angular-eslint/eslint-plugin-template` 22.6.0).
#[must_use]
pub fn has_ignored_directive(element: &Element, entries: &[&str]) -> bool {
    bound3(element)
        .iter()
        .filter(|each| each.role != Role::Output)
        .any(|each| {
            entries
                .iter()
                .any(|entry| matches_directive(entry, &each.name))
        })
}

/// Whether an element is Angular's element rather than a template or a content projection:
/// neither `<ng-template>` nor `<ng-content>`, its namespace aside.
#[must_use]
pub fn is_plain_element(element: &Element) -> bool {
    let name = split_ns_name(&element.name).1;
    name != "ng-template" && name != "ng-content"
}

/// The cases of a `@switch` sharing one body: the cases written without one (`@case (1)`), then
/// the case or `@default` that has it.
#[derive(Debug)]
pub struct CaseGroup<'t> {
    /// The cases, the one with the body last.
    pub cases: Vec<&'t Block>,
    /// The case with the body.
    pub body: &'t Block,
}

/// A `@switch` as Angular's compiler groups it (`createSwitchBlock` of `render3/r3_control_flow.ts`
/// of `@angular/compiler` 22.2.3): its groups of cases, and whether it ends with the exhaustive
/// `@default never;`. A child that is neither a `@case` with a value nor a `@default` is left out,
/// as are cases without a body that no case with one follows.
#[derive(Debug)]
pub struct Switch<'t> {
    /// The groups, in order.
    pub groups: Vec<CaseGroup<'t>>,
    /// Whether a `@default never` checks the switch exhaustive.
    pub exhaustive: bool,
}

/// A `@switch` block's groups.
#[must_use]
pub fn switch_groups(block: &Block) -> Switch<'_> {
    let mut groups = Vec::new();
    let mut collected = Vec::new();
    let mut exhaustive = false;
    for node in &block.children {
        let Node::Block(child) = node else {
            continue;
        };
        match child.name.as_str() {
            "default never" => {
                exhaustive = true;
                continue;
            }
            "case" if !child.parameters.is_empty() => collected.push(child),
            "default" => collected.push(child),
            _ => continue,
        }
        // A case written without braces, `@case (1)` before another, waits for a body.
        let without_body =
            child.children.is_empty() && child.end_span.is_some_and(|end| end.start == end.end);
        if !without_body {
            groups.push(CaseGroup {
                cases: std::mem::take(&mut collected),
                body: child,
            });
        }
    }
    Switch { groups, exhaustive }
}

/// Visits nodes and their descendants, depth first.
fn walk_nodes<'t>(
    nodes: &'t [Node],
    ancestors: &mut Vec<&'t Node>,
    visit: &mut dyn FnMut(&'t Node, &[&'t Node]),
) {
    for node in nodes {
        visit(node, ancestors);
        ancestors.push(node);
        match node {
            Node::Element(element) => walk_nodes(&element.children, ancestors, visit),
            Node::Block(block) => walk_nodes(&block.children, ancestors, visit),
            Node::Expansion(expansion) => {
                for case in &expansion.cases {
                    walk_nodes(&case.expression, ancestors, visit);
                }
            }
            Node::Text(_) | Node::Comment(_) | Node::LetDeclaration(_) => {}
        }
        ancestors.pop();
    }
}

/// A bound and every bound under it: its pipes' arguments and its nested pipe expressions,
/// each with theirs, depth first.
#[must_use]
pub fn every_bound<'b, 'a>(bound: &'b Bound<'a>) -> Vec<&'b Bound<'a>> {
    let mut found = vec![bound];
    for pipe in &bound.pipes {
        for argument in &pipe.arguments {
            found.extend(every_bound(argument));
        }
    }
    for nested in &bound.nested {
        found.extend(every_bound(nested));
    }
    found
}

/// Visits every node of every bound under a written expression, outermost first, the elements
/// of an optional chain among them, each with the bound it belongs to, whose `start` places its
/// span in the template.
pub fn each_node<'a>(written: &Written<'a>, visit: &mut dyn FnMut(AstKind<'a>, &Bound<'a>)) {
    /// Calls the visit on each node.
    struct Visitor<'v, 'a> {
        /// The bound visited.
        bound: &'v Bound<'a>,
        /// The visit.
        visit: &'v mut dyn FnMut(AstKind<'a>, &Bound<'a>),
    }
    impl<'a> Visit<'a> for Visitor<'_, 'a> {
        fn enter_node(&mut self, kind: AstKind<'a>) {
            (self.visit)(kind, self.bound);
        }
    }
    for bound in every_bound(&written.bound) {
        if let Some(expression) = &bound.expression {
            Visitor {
                bound,
                visit: &mut *visit,
            }
            .visit_expression(expression);
        }
    }
}

/// The nested pipe expression a bound's Oxc text masks at a span (`(a | async)` read apart and
/// written `(___________)` for Oxc), when one is there.
#[must_use]
pub fn nested_at<'b, 'a>(bound: &'b Bound<'a>, span: oxc_span::Span) -> Option<&'b Bound<'a>> {
    let at = placed(bound, span);
    bound
        .nested
        .iter()
        .find(|nested| nested.span.start == at.start && nested.span.end == at.end)
}

/// The whole of a bound with its pipes, as Angular's outermost `BindingPipe` spans it: from its
/// expression's first character to its last pipe's last, the whitespace around left out; up to
/// its first pipe only when asked.
#[must_use]
pub fn pipe_span(text: &str, bound: &Bound, through_first_only: bool) -> Span {
    let end = if through_first_only {
        bound
            .pipes
            .first()
            .map_or(bound.span.end, |pipe| pipe.span.end)
    } else {
        bound.span.end
    };
    trimmed(text, Span::new(bound.span.start as usize, end as usize))
}

/// A span of the template without the whitespace at its ends.
#[must_use]
pub fn trimmed(text: &str, span: Span) -> Span {
    let (start, end) = (span.start as usize, span.end as usize);
    let inner = text.get(start..end).unwrap_or_default();
    let leading = inner.len() - inner.trim_start().len();
    let trailing = inner.len() - inner.trim_end().len();
    if leading == inner.len() {
        return Span::new(start, start);
    }
    Span::new(start + leading, end - trailing)
}

/// An Oxc span of a bound's expression placed in the template.
#[must_use]
pub fn placed(bound: &Bound, span: oxc_span::Span) -> Span {
    Span::new(
        (bound.start + span.start) as usize,
        (bound.start + span.end) as usize,
    )
}

/// A message written with its data, as `ESLint` interpolates it: each `{{ name }}` the data
/// gives is replaced, any other kept.
#[must_use]
pub fn interpolate(message: &str, data: &[(&str, String)]) -> String {
    let mut result = String::with_capacity(message.len());
    let mut rest = message;
    while let Some(open) = rest.find("{{") {
        let after = &rest[open + 2..];
        let Some(close) = after.find("}}") else {
            break;
        };
        let term = &after[..close];
        result.push_str(&rest[..open]);
        match data.iter().find(|(name, _)| *name == term.trim()) {
            Some((_, value)) if !term.contains(['{', '}']) && !term.is_empty() => {
                result.push_str(value);
            }
            _ => result.push_str(&rest[open..open + 2 + close + 2]),
        }
        rest = &after[close + 2..];
    }
    result.push_str(rest);
    result
}

/// A rule's message for a report.
#[must_use]
pub fn message(rule: &dyn TemplateRule, report: &Report) -> String {
    let template = rule
        .messages()
        .iter()
        .find(|(id, _)| *id == report.message_id)
        .map_or(report.message_id, |(_, text)| text);
    interpolate(template, &report.data)
}

/// The reports of a rule on a template read cleanly, sorted by place as `ESLint` sorts them.
#[must_use]
pub fn reports(rule: &dyn TemplateRule, path: &str, text: &str, nodes: &[Node]) -> Vec<Report> {
    let allocator = Allocator::default();
    let template = Template {
        path,
        text,
        nodes,
        allocator: &allocator,
    };
    let mut found = Vec::new();
    rule.check(&template, &mut found);
    let decoded = decoded_values(&template);
    for report in &mut found {
        if report
            .fix
            .as_ref()
            .is_some_and(|fix| decoded.iter().any(|value| cuts_into(fix, *value)))
        {
            report.fix = None;
        }
    }
    found.sort_by_key(|report| report.span.start);
    found
}

/// The values read as expressions whose source is not the text read: an entity written in
/// `[title]="a &amp;&amp; b"`, which the reader decodes, as Angular's does, before parsing, so
/// that the offsets of the expression are the decoded text's, not the source's.
fn decoded_values(template: &Template) -> Vec<Span> {
    let mut found = Vec::new();
    for (element, _) in template.elements() {
        for attribute in &element.attrs {
            let read_as_expression = matches!(
                attribute_kind(&attribute.name).0,
                AttributeKind::Property
                    | AttributeKind::TwoWay
                    | AttributeKind::Event
                    | AttributeKind::Template
            );
            if let Some(span) = attribute.value_span
                && read_as_expression
                && template
                    .text
                    .get(span.full_start as usize..span.end as usize)
                    != Some(attribute.value.as_str())
            {
                found.push(span);
            }
        }
    }
    found
}

/// Whether a fix replaces or inserts text inside a value without replacing it whole: a range
/// placed from the offsets of a decoded value does not hold the text the rule read there, so
/// the fix is dropped, its report kept, rather than writing over the wrong characters.
fn cuts_into(fix: &Fix, value: Span) -> bool {
    let overlaps = fix.start <= value.end && fix.end >= value.full_start;
    let whole = fix.start <= value.full_start && fix.end >= value.end;
    overlaps && !whole
}

/// The rules, in the order of their names, as the project runs them: every module's but
/// `no_call_expression`, off by the Product Owner's choice (a signal read in a template is a
/// call), its module and tests kept.
pub const RULES: &[&dyn TemplateRule] = &[
    &alt_text::AltText,
    &attributes_order::AttributesOrder::PROJECT,
    &banana_in_box::BananaInBox,
    &button_has_type::ButtonHasType::PROJECT,
    &click_events_have_key_events::ClickEventsHaveKeyEvents::PROJECT,
    &conditional_complexity::ConditionalComplexity::PROJECT,
    &cyclomatic_complexity::CyclomaticComplexity::PROJECT,
    &elements_content::ElementsContent::PROJECT,
    &eqeqeq::Eqeqeq::PROJECT,
    &interactive_supports_focus::InteractiveSupportsFocus::PROJECT,
    &label_has_associated_control::LabelHasAssociatedControl::PROJECT,
    &mouse_events_have_key_events::MouseEventsHaveKeyEvents,
    &no_any::NoAny,
    &no_autofocus::NoAutofocus,
    &no_distracting_elements::NoDistractingElements,
    &no_duplicate_attributes::NoDuplicateAttributes::PROJECT,
    &no_empty_control_flow::NoEmptyControlFlow,
    &no_inline_styles::NoInlineStyles::PROJECT,
    &no_interpolation_in_attributes::NoInterpolationInAttributes::PROJECT,
    &no_negated_async::NoNegatedAsync,
    &no_nested_tags::NoNestedTags,
    &no_non_null_assertion::NoNonNullAssertion,
    &no_outerhtml::NoOuterhtml,
    &no_positive_tabindex::NoPositiveTabindex,
    &prefer_at_else::PreferAtElse,
    &prefer_at_empty::PreferAtEmpty,
    &prefer_built_in_pipes::PreferBuiltInPipes::PROJECT,
    &prefer_class_binding::PreferClassBinding,
    &prefer_contextual_for_variables::PreferContextualForVariables::PROJECT,
    &prefer_control_flow::PreferControlFlow,
    &prefer_ngsrc::PreferNgsrc,
    &prefer_self_closing_tags::PreferSelfClosingTags,
    &prefer_static_string_properties::PreferStaticStringProperties::PROJECT,
    &prefer_style_binding::PreferStyleBinding,
    &prefer_template_literal::PreferTemplateLiteral,
    &require_switch_default::RequireSwitchDefault,
    &role_has_required_aria::RoleHasRequiredAria,
    &table_scope::TableScope,
    &use_track_by_function::UseTrackByFunction::PROJECT,
    &valid_aria::ValidAria,
];

#[cfg(test)]
mod tests {
    use super::{
        RULES, attributes_order::AttributesOrder, banana_in_box::BananaInBox, eqeqeq::Eqeqeq,
        interpolate, matches_directive,
        no_interpolation_in_attributes::NoInterpolationInAttributes, prefer_at_else::PreferAtElse,
        prefer_at_empty::PreferAtEmpty,
        prefer_contextual_for_variables::PreferContextualForVariables,
        prefer_self_closing_tags::PreferSelfClosingTags,
        prefer_static_string_properties::PreferStaticStringProperties,
        prefer_template_literal::PreferTemplateLiteral,
    };

    #[test]
    fn matches_a_name_or_a_pattern_with_its_flags() {
        // Our own cases, after `createIgnoredDirectiveMatcher`.
        assert!(matches_directive("/^tui/", "tuiButton"));
        assert!(matches_directive("/^TUI/i", "tuiButton"));
        assert!(!matches_directive("/^TUI/", "tuiButton"));
        assert!(!matches_directive("tui", "tuiButton"));
        assert!(matches_directive("tuiButton", "tuiButton"));
        // `//` is no pattern, so compared as a name.
        assert!(!matches_directive("//", "a"));
    }

    #[test]
    fn interpolates_a_message_as_eslint_does() {
        let data = [("name", "a".to_owned())];

        assert_eq!(interpolate("x {{name}} {{ name }} y", &data), "x a a y");
        // A name the data lacks is kept; an unclosed `{{` too.
        assert_eq!(interpolate("{{other}} {{name", &data), "{{other}} {{name");
    }

    /// A template with the fixes of one pass of a rule applied.
    fn fixed_once(rule: &dyn super::TemplateRule, text: &str) -> String {
        let parsed = crate::template::tree::parse(text);
        assert_eq!(crate::template::bindings::check(&parsed), [], "{text:?}");
        let found = super::reports(rule, "a.component.html", text, &parsed.nodes);
        let fixes: Vec<&crate::finding::Fix> = found
            .iter()
            .filter_map(|report| report.fix.as_ref())
            .collect();
        crate::finding::apply(text, &fixes)
    }

    /// Each fixing rule on a template written with CRLF line ends, its fix at the source's
    /// offsets; then with an entity: a value read decoded keeps the fixes that move it whole
    /// or touch it not, and loses those that cut into it (eqeqeq, prefer-contextual-for-
    /// variables, prefer-template-literal), their offsets the decoded text's.
    const FIX_CASES: [(&dyn super::TemplateRule, &str, &str); 20] = [
        (
            &AttributesOrder::PROJECT,
            "<p\r\n  title=\"a\"\r\n  #r></p>",
            "<p\r\n  #r\r\n  title=\"a\"></p>",
        ),
        (
            &AttributesOrder::PROJECT,
            "<p [title]=\"a &amp;&amp; b\" #r></p>",
            "<p #r [title]=\"a &amp;&amp; b\"></p>",
        ),
        (
            &BananaInBox,
            "<input\r\n  ([ngModel])=\"\r\n x\">",
            "<input\r\n  [(ngModel)]=\"\r\n x\">",
        ),
        (
            &BananaInBox,
            "<input ([ngModel])=\"x &amp;&amp; y\">",
            "<input [(ngModel)]=\"x &amp;&amp; y\">",
        ),
        (
            &Eqeqeq::PROJECT,
            "<p [title]=\"\r\n  b &&\r\n  a == 'c'\"></p>",
            "<p [title]=\"\r\n  b &&\r\n  a === 'c'\"></p>",
        ),
        (
            &Eqeqeq::PROJECT,
            "<p [title]=\"x &lt; y &lt; z || a == 'b' == 'c'\"></p>",
            "<p [title]=\"x &lt; y &lt; z || a == 'b' == 'c'\"></p>",
        ),
        (
            &NoInterpolationInAttributes::PROJECT,
            "<p\r\n  title=\"{{\r\n a }}\"></p>",
            "<p\r\n  [title]=\"a\"></p>",
        ),
        (
            &NoInterpolationInAttributes::PROJECT,
            "<p [x]=\"a &amp;&amp; b\" title=\"{{ a }}\"></p>",
            "<p [x]=\"a &amp;&amp; b\" [title]=\"a\"></p>",
        ),
        (
            &PreferAtElse,
            "@if (a) {\r\nx\r\n}\r\n@if (!a) {\r\ny\r\n}",
            "@if (a) {\r\nx\r\n}\r\n@else {\r\ny\r\n}",
        ),
        (
            &PreferAtElse,
            "@if (a) {&amp;}\n@if (!a) {&lt;}",
            "@if (a) {&amp;}\n@else {&lt;}",
        ),
        (
            &PreferAtEmpty,
            "@for (i of l; track i) {\r\n{{ i }}\r\n}\r\n@if (l.length === 0) {\r\nnone\r\n}",
            "@for (i of l; track i) {\r\n{{ i }}\r\n}\r\n@empty {\r\nnone\r\n}",
        ),
        (
            &PreferAtEmpty,
            "@for (i of l; track i) {&amp;}\n@if (l.length === 0) {&lt;}",
            "@for (i of l; track i) {&amp;}\n@empty {&lt;}",
        ),
        (
            &PreferContextualForVariables::PROJECT,
            "@for (item of items; track item; let i = $index) {\r\n<p (click)=\"\r\n go(i)\"></p>}",
            "@for (item of items; track item) {\r\n<p (click)=\"\r\n go($index)\"></p>}",
        ),
        (
            &PreferContextualForVariables::PROJECT,
            "@for (item of items; track item; let i = $index) {<p (click)=\"a &amp;&amp; go(i)\"></p>}",
            "@for (item of items; track item; let i = $index) {<p (click)=\"a &amp;&amp; go(i)\"></p>}",
        ),
        (
            &PreferSelfClosingTags,
            "<app-a\r\n  title=\"&amp;\"\r\n></app-a>",
            "<app-a\r\n  title=\"&amp;\"\r\n/>",
        ),
        (
            &PreferSelfClosingTags,
            "<app-a title=\"&amp;\">\r\n</app-a>",
            "<app-a title=\"&amp;\" />",
        ),
        (
            &PreferStaticStringProperties::PROJECT,
            "<p\r\n  [title]=\"\r\n'a'\"></p>",
            "<p\r\n  title=\"a\"></p>",
        ),
        (
            &PreferStaticStringProperties::PROJECT,
            "<p [title]=\"'a&amp;b'\"></p>",
            "<p title=\"a&amp;b\"></p>",
        ),
        (
            &PreferTemplateLiteral,
            "<p>{{\r\n\r\n x + 'a' + y }}</p>",
            "<p>{{\r\n\r\n `${x}a${y}` }}</p>",
        ),
        (
            &PreferTemplateLiteral,
            "<p [title]=\"b &amp;&amp; x + 'a'\"></p>",
            "<p [title]=\"b &amp;&amp; x + 'a'\"></p>",
        ),
    ];

    #[test]
    fn f52_ac2_fixes_crlf_templates_and_no_value_whose_entities_moved_the_offsets() {
        let failures: Vec<String> = FIX_CASES
            .iter()
            .map(|(rule, text, expected)| (rule, text, expected, fixed_once(*rule, text)))
            .filter(|(_, _, expected, fixed)| fixed != *expected)
            .map(|(rule, text, _, fixed)| format!("{} {text:?} -> {fixed:?}", rule.name()))
            .collect();
        assert_eq!(failures, Vec::<String>::new());
    }

    #[test]
    fn names_each_rule_once_in_order() {
        let names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(names, sorted);
        assert!(
            names
                .iter()
                .all(|name| name.starts_with("@angular-eslint/template/"))
        );
    }
}
