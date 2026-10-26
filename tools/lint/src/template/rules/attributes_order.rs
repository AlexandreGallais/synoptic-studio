//! `@angular-eslint/template/attributes-order`: an element's attributes in one order, by kind:
//! its structural directive, its template references, its plain attributes, its inputs, its
//! two-way bindings, its outputs (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/attributes-order.ts`), alphabetical within a kind when asked. The attributes out
//! of place are reported from the first to the last and fixed by moving each into its place. An
//! element with `i18n` marks is read as written, the others as Angular's tree holds them: a
//! property written without brackets (`bind-x`, `title="{{ x }}"`) is a plain attribute there,
//! and a `<ng-template>`'s `let-` variables too. The project runs the plugin's defaults.

use super::{Report, Role, Template, TemplateRule, bound3};
use crate::template::bindings::{AttributeKind, attribute_kind};
use crate::template::lexer::Span;
use crate::template::tags::split_ns_name;
use crate::template::tree::Element;

/// The kinds of attribute the rule orders (`OrderType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderType {
    /// `#name`.
    TemplateReference,
    /// `*directive`.
    StructuralDirective,
    /// A plain attribute.
    AttributeBinding,
    /// `[name]`.
    InputBinding,
    /// `(name)`.
    OutputBinding,
    /// `[(name)]`.
    TwoWayBinding,
}

impl OrderType {
    /// The kind of an option's name, as the plugin writes it.
    #[must_use]
    pub fn from_option(name: &str) -> Option<Self> {
        Some(match name {
            "TEMPLATE_REFERENCE" => Self::TemplateReference,
            "STRUCTURAL_DIRECTIVE" => Self::StructuralDirective,
            "ATTRIBUTE_BINDING" => Self::AttributeBinding,
            "INPUT_BINDING" => Self::InputBinding,
            "OUTPUT_BINDING" => Self::OutputBinding,
            "TWO_WAY_BINDING" => Self::TwoWayBinding,
            _ => return None,
        })
    }
}

/// The rule and its options.
pub struct AttributesOrder {
    /// Whether the attributes of a kind are sorted by name.
    pub alphabetical: bool,
    /// The kinds, in order.
    pub order: &'static [OrderType],
}

/// The plugin's order (`DEFAULT_ORDER`).
const DEFAULT_ORDER: [OrderType; 6] = [
    OrderType::StructuralDirective,
    OrderType::TemplateReference,
    OrderType::AttributeBinding,
    OrderType::InputBinding,
    OrderType::TwoWayBinding,
    OrderType::OutputBinding,
];

impl AttributesOrder {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        alphabetical: false,
        order: &DEFAULT_ORDER,
    };
}

/// An attribute ordered (`SortableAttribute`).
#[derive(Clone, Debug)]
struct Sortable {
    /// Its name as the messages and the alphabetical order read it.
    name: String,
    /// Where: the whole attribute.
    span: Span,
    /// Its kind.
    kind: OrderType,
    /// Whether an `i18n-` mark of an input, named without brackets.
    i18n_for_input: bool,
}

/// The attributes of an element marked for translation, as written
/// (`getAttributesFromHtmlParser`).
fn as_written(element: &Element) -> Vec<Sortable> {
    let inputs: Vec<String> = bound3(element)
        .into_iter()
        .filter(|each| each.role == Role::Input)
        .map(|each| each.name)
        .collect();
    element
        .attrs
        .iter()
        .map(|attribute| {
            let name = attribute.name.as_str();
            let between = |start: &str, end: &str| {
                name.strip_prefix(start)
                    .and_then(|rest| rest.strip_suffix(end))
                    .map(str::to_owned)
            };
            let (written, kind) = if let Some(rest) = name.strip_prefix('#') {
                (rest.to_owned(), OrderType::TemplateReference)
            } else if let Some(rest) = name.strip_prefix('*') {
                (rest.to_owned(), OrderType::StructuralDirective)
            } else if let Some(inner) = between("[(", ")]") {
                (inner, OrderType::TwoWayBinding)
            } else if let Some(inner) = between("[", "]") {
                (inner, OrderType::InputBinding)
            } else if let Some(inner) = between("(", ")") {
                (inner, OrderType::OutputBinding)
            } else {
                let for_input = name
                    .strip_prefix("i18n-")
                    .is_some_and(|rest| inputs.iter().any(|input| input == rest));
                let kind = if for_input {
                    OrderType::InputBinding
                } else {
                    OrderType::AttributeBinding
                };
                return Sortable {
                    name: name.to_owned(),
                    span: attribute.span,
                    kind,
                    i18n_for_input: name.starts_with("i18n-"),
                };
            };
            Sortable {
                name: written,
                span: attribute.span,
                kind,
                i18n_for_input: false,
            }
        })
        .collect()
}

/// The attributes of an element as Angular's tree holds them (`getAllAttributes`): its
/// structural directive, its plain attributes and unbracketed inputs, its references, its
/// two-way bindings, its inputs, its outputs.
fn as_tree(element: &Element) -> Vec<Sortable> {
    let is_template = split_ns_name(&element.name).1 == "ng-template";
    let mut structural = Vec::new();
    let (mut unbracketed, mut plain, mut references) = (Vec::new(), Vec::new(), Vec::new());
    let (mut two_way, mut inputs, mut outputs) = (Vec::new(), Vec::new(), Vec::new());
    let sortable = |name: &str, span: Span, kind| Sortable {
        name: name.to_owned(),
        span,
        kind,
        i18n_for_input: false,
    };
    for attribute in &element.attrs {
        let (kind, inner) = attribute_kind(&attribute.name);
        let span = attribute.span;
        let bracketed = attribute.name.starts_with('[');
        match kind {
            AttributeKind::Template if !is_template && structural.is_empty() => {
                structural.push(sortable(&inner, span, OrderType::StructuralDirective));
            }
            AttributeKind::Variable if is_template => {
                structural.push(sortable(&attribute.name, span, OrderType::AttributeBinding));
            }
            AttributeKind::Reference => {
                references.push(sortable(&inner, span, OrderType::TemplateReference));
            }
            AttributeKind::TwoWay if bracketed => {
                two_way.push(sortable(&inner, span, OrderType::TwoWayBinding));
            }
            AttributeKind::TwoWay => {
                unbracketed.push(sortable(&inner, span, OrderType::AttributeBinding));
                let change = format!("{inner}Change");
                outputs.push(sortable(&change, span, OrderType::OutputBinding));
            }
            AttributeKind::Property if bracketed => {
                inputs.push(sortable(&inner, span, OrderType::InputBinding));
            }
            AttributeKind::Property | AttributeKind::Animation | AttributeKind::Interpolated => {
                unbracketed.push(sortable(&inner, span, OrderType::AttributeBinding));
            }
            AttributeKind::Event => {
                outputs.push(sortable(&inner, span, OrderType::OutputBinding));
            }
            AttributeKind::Plain
                if attribute.value_tokens.iter().any(|token| {
                    token.kind == crate::template::lexer::TokenType::AttrValueInterpolation
                        && token.parts.len() == 3
                }) =>
            {
                unbracketed.push(sortable(&inner, span, OrderType::AttributeBinding));
            }
            // Angular's tree keeps no `i18n` mark among the attributes.
            AttributeKind::Plain if inner == "i18n" || inner.starts_with("i18n-") => {}
            AttributeKind::Plain => plain.push(sortable(&inner, span, OrderType::AttributeBinding)),
            AttributeKind::Template | AttributeKind::Variable => {}
        }
    }
    [
        structural,
        unbracketed,
        plain,
        references,
        two_way,
        inputs,
        outputs,
    ]
    .concat()
}

/// Whether an element holds an `i18n` mark, of itself or of one of its attributes.
fn has_i18n(element: &Element) -> bool {
    let names: Vec<String> = element
        .attrs
        .iter()
        .map(|attribute| attribute_kind(&attribute.name).1)
        .collect();
    element.attrs.iter().any(|attribute| {
        attribute.name == "i18n"
            || attribute
                .name
                .strip_prefix("i18n-")
                .is_some_and(|rest| names.iter().any(|name| name == rest))
    })
}

/// The name an attribute is written with in a message (`getMessageName`).
fn message_name(attribute: &Sortable) -> String {
    let name = &attribute.name;
    match attribute.kind {
        OrderType::StructuralDirective => format!("*{name}"),
        OrderType::TemplateReference => format!("#{name}"),
        OrderType::InputBinding if attribute.i18n_for_input => name.clone(),
        OrderType::InputBinding => format!("[{name}]"),
        OrderType::OutputBinding => format!("({name})"),
        OrderType::TwoWayBinding => format!("[({name})]"),
        OrderType::AttributeBinding => name.clone(),
    }
}

impl AttributesOrder {
    /// The rank of a kind (`getOrderIndex`), `-1` when absent from the order.
    fn rank(&self, kind: OrderType) -> i64 {
        self.order
            .iter()
            .position(|each| *each == kind)
            .map_or(-1, |index| i64::try_from(index).unwrap_or(i64::MAX))
    }

    /// The comparison of two attributes (`byOrder`).
    fn compare(&self, one: &Sortable, other: &Sortable) -> std::cmp::Ordering {
        let by_kind = self.rank(one.kind).cmp(&self.rank(other.kind));
        if !self.alphabetical || by_kind.is_ne() {
            return by_kind;
        }
        let one_name = one.name.strip_prefix("i18n-").unwrap_or(&one.name);
        let other_name = other.name.strip_prefix("i18n-").unwrap_or(&other.name);
        if one_name == other_name {
            return if one.name.starts_with("i18n-") {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            };
        }
        if one_name > other_name {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

impl TemplateRule for AttributesOrder {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/attributes-order"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "attributesOrder",
            "The element's attributes/bindings did not match the expected order: expected {{expected}} instead of {{actual}}",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            let all = if has_i18n(element) {
                as_written(element)
            } else {
                as_tree(element)
            };
            if all.len() < 2 {
                continue;
            }
            let mut by_place = all.clone();
            by_place.sort_by_key(|each| each.span.start);
            let mut expected = all;
            expected.sort_by(|one, other| self.compare(one, other));
            let differs: Vec<usize> = (0..by_place.len())
                .filter(|index| by_place[*index].span != expected[*index].span)
                .collect();
            let (Some(&first), Some(&last)) = (differs.first(), differs.last()) else {
                continue;
            };
            let text = template.text;
            let mut replacement = String::new();
            let mut position = by_place[first].span.start;
            for index in first..=last {
                let old = by_place[index].span;
                let new = expected[index].span;
                replacement.push_str(super::r3::slice(text, position, old.start));
                replacement.push_str(super::r3::slice(text, new.start, new.end));
                position = old.end;
            }
            let names = |list: &[Sortable]| {
                list[first..=last]
                    .iter()
                    .map(|each| format!("`{}`", message_name(each)))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let span = Span::new(
                by_place[first].span.start as usize,
                by_place[last].span.end as usize,
            );
            reports.push(Report {
                data: vec![("expected", names(&expected)), ("actual", names(&by_place))],
                fix: Some(super::r3::replace(span.start, span.end, &replacement)),
                ..Report::new(span, "attributesOrder")
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AttributesOrder, DEFAULT_ORDER, OrderType};
    use crate::template::corpus::Value;
    use crate::template::rules::tester::run;

    /// The rule with a case's options.
    fn build(options: Option<&Value>) -> AttributesOrder {
        let order: Option<Vec<OrderType>> =
            options.and_then(|each| each.texts("order")).map(|names| {
                names
                    .iter()
                    .filter_map(|name| OrderType::from_option(name))
                    .collect()
            });
        AttributesOrder {
            alphabetical: options
                .and_then(|each| each.flag("alphabetical"))
                .unwrap_or(false),
            order: order.map_or(&DEFAULT_ORDER[..], |order| &*Vec::leak(order)),
        }
    }

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let constants = [
            ("OrderType.TemplateReferenceVariable", "TEMPLATE_REFERENCE"),
            ("OrderType.StructuralDirective", "STRUCTURAL_DIRECTIVE"),
            ("OrderType.AttributeBinding", "ATTRIBUTE_BINDING"),
            ("OrderType.InputBinding", "INPUT_BINDING"),
            ("OrderType.OutputBinding", "OUTPUT_BINDING"),
            ("OrderType.TwoWayBinding", "TWO_WAY_BINDING"),
        ];
        let tally = run("attributes-order.txt", &constants, &|options| {
            Box::new(build(options))
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 23 valid and 29 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (23, 29, 0));
    }
}
