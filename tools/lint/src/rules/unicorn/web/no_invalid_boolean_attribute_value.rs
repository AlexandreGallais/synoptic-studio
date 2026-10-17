//! `unicorn/no-invalid-boolean-attribute-value`: an HTML boolean attribute is true whenever it
//! is present, so `setAttribute('disabled', 'false')` still disables; its value is empty or its
//! own name (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-invalid-boolean-attribute-value.md`). Only on elements known from
//! `document.createElement(…)`, `createElementNS(…)` in the HTML namespace, a simple
//! `querySelector(…)`, a `const` bound to one, or a native element's annotation. Reported only:
//! `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression, TSType, TSTypeName};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::dom::nodes::{member_object, static_property_name};
use crate::rules::unicorn::syntax::{Value, static_value, symbol_of};
use crate::source::Source;

/// The rule.
pub struct NoInvalidBooleanAttributeValue;

/// The HTML namespace.
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// Each native element type with its tags; `HTMLElement`'s tags make no native type.
const ELEMENT_TAGS: [(&str, &str); 64] = [
    ("HTMLAnchorElement", "a"),
    (
        "HTMLElement",
        "abbr address article aside b bdi bdo cite code dd dfn dt em figcaption figure footer header hgroup i kbd main mark nav noscript rp rt ruby s samp search section small strong sub summary sup u var wbr",
    ),
    ("HTMLAreaElement", "area"),
    ("HTMLAudioElement", "audio"),
    ("HTMLBaseElement", "base"),
    ("HTMLQuoteElement", "blockquote q"),
    ("HTMLBodyElement", "body"),
    ("HTMLBRElement", "br"),
    ("HTMLButtonElement", "button"),
    ("HTMLCanvasElement", "canvas"),
    ("HTMLTableCaptionElement", "caption"),
    ("HTMLTableColElement", "col colgroup"),
    ("HTMLDataElement", "data"),
    ("HTMLDataListElement", "datalist"),
    ("HTMLModElement", "del ins"),
    ("HTMLDetailsElement", "details"),
    ("HTMLDialogElement", "dialog"),
    ("HTMLDivElement", "div"),
    ("HTMLDListElement", "dl"),
    ("HTMLEmbedElement", "embed"),
    ("HTMLFieldSetElement", "fieldset"),
    ("HTMLFormElement", "form"),
    ("HTMLHeadingElement", "h1 h2 h3 h4 h5 h6"),
    ("HTMLHeadElement", "head"),
    ("HTMLHRElement", "hr"),
    ("HTMLHtmlElement", "html"),
    ("HTMLIFrameElement", "iframe"),
    ("HTMLImageElement", "img"),
    ("HTMLInputElement", "input"),
    ("HTMLLabelElement", "label"),
    ("HTMLLegendElement", "legend"),
    ("HTMLLIElement", "li"),
    ("HTMLLinkElement", "link"),
    ("HTMLMapElement", "map"),
    ("HTMLMenuElement", "menu"),
    ("HTMLMetaElement", "meta"),
    ("HTMLMeterElement", "meter"),
    ("HTMLObjectElement", "object"),
    ("HTMLOListElement", "ol"),
    ("HTMLOptGroupElement", "optgroup"),
    ("HTMLOptionElement", "option"),
    ("HTMLOutputElement", "output"),
    ("HTMLParagraphElement", "p"),
    ("HTMLPictureElement", "picture"),
    ("HTMLPreElement", "pre"),
    ("HTMLProgressElement", "progress"),
    ("HTMLScriptElement", "script"),
    ("HTMLSelectElement", "select"),
    ("HTMLSelectedContentElement", "selectedcontent"),
    ("HTMLSlotElement", "slot"),
    ("HTMLSourceElement", "source"),
    ("HTMLSpanElement", "span"),
    ("HTMLStyleElement", "style"),
    ("HTMLTableElement", "table"),
    ("HTMLTableSectionElement", "tbody tfoot thead"),
    ("HTMLTableCellElement", "td th"),
    ("HTMLTemplateElement", "template"),
    ("HTMLTextAreaElement", "textarea"),
    ("HTMLTimeElement", "time"),
    ("HTMLTitleElement", "title"),
    ("HTMLTableRowElement", "tr"),
    ("HTMLTrackElement", "track"),
    ("HTMLUListElement", "ul"),
    ("HTMLVideoElement", "video"),
];

/// The boolean attributes every element has.
const GLOBAL_BOOLEANS: [&str; 4] = ["autofocus", "headingreset", "inert", "itemscope"];

/// The boolean attributes of some elements, with the types that have them.
const BOOLEANS: [(&str, &[&str]); 26] = [
    ("allowfullscreen", &["HTMLIFrameElement"]),
    ("alpha", &["HTMLInputElement"]),
    ("async", &["HTMLScriptElement"]),
    ("autoplay", &["HTMLAudioElement", "HTMLVideoElement"]),
    ("checked", &["HTMLInputElement"]),
    (
        "controls",
        &["HTMLAudioElement", "HTMLVideoElement", "HTMLImageElement"],
    ),
    ("default", &["HTMLTrackElement"]),
    ("defer", &["HTMLScriptElement"]),
    (
        "disabled",
        &[
            "HTMLButtonElement",
            "HTMLInputElement",
            "HTMLOptGroupElement",
            "HTMLOptionElement",
            "HTMLSelectElement",
            "HTMLTextAreaElement",
            "HTMLFieldSetElement",
            "HTMLLinkElement",
        ],
    ),
    ("formnovalidate", &["HTMLButtonElement", "HTMLInputElement"]),
    ("ismap", &["HTMLImageElement"]),
    ("loop", &["HTMLAudioElement", "HTMLVideoElement"]),
    ("multiple", &["HTMLInputElement", "HTMLSelectElement"]),
    ("muted", &["HTMLAudioElement", "HTMLVideoElement"]),
    ("nomodule", &["HTMLScriptElement"]),
    ("novalidate", &["HTMLFormElement"]),
    ("open", &["HTMLDetailsElement", "HTMLDialogElement"]),
    ("playsinline", &["HTMLVideoElement"]),
    ("readonly", &["HTMLInputElement", "HTMLTextAreaElement"]),
    (
        "required",
        &[
            "HTMLInputElement",
            "HTMLSelectElement",
            "HTMLTextAreaElement",
        ],
    ),
    ("reversed", &["HTMLOListElement"]),
    ("selected", &["HTMLOptionElement"]),
    ("shadowrootclonable", &["HTMLTemplateElement"]),
    ("shadowrootcustomelementregistry", &["HTMLTemplateElement"]),
    ("shadowrootdelegatesfocus", &["HTMLTemplateElement"]),
    ("shadowrootserializable", &["HTMLTemplateElement"]),
];

/// The tags a selector of one element may name with its type, the ambiguous ones left out.
const AMBIGUOUS_TAGS: [&str; 4] = ["a", "script", "style", "title"];

/// Whether a type is a native element's, `HTMLElement` aside.
fn is_native(name: &str) -> bool {
    name != "HTMLElement" && ELEMENT_TAGS.iter().any(|(each, _)| *each == name)
}

/// The type of an element by its tag.
fn type_of_tag(tag: &str) -> Option<&'static str> {
    ELEMENT_TAGS
        .iter()
        .find(|(_, tags)| tags.split(' ').any(|each| each == tag))
        .map(|(name, _)| *name)
}

/// The string a value always is.
fn string_value(source: &Source, expression: &Expression) -> Option<String> {
    match static_value(source, expression) {
        Some(Value::Text(text)) => Some(text),
        _ => None,
    }
}

/// The text a primitive converts to, as `String(…)`.
fn as_string(value: &Value) -> String {
    match value {
        Value::Text(text) => text.clone(),
        Value::Number(number) if number.fract() == 0.0 && number.abs() < 1e21 => {
            format!("{number:.0}")
        }
        Value::Number(number) => number.to_string(),
        Value::Boolean(boolean) => boolean.to_string(),
        Value::Null => "null".to_owned(),
        Value::Undefined => "undefined".to_owned(),
        Value::BigInt(digits) => digits.trim_end_matches('n').to_owned(),
    }
}

/// Whether an expression is the global `document`, `window.document`, or a `const` bound to it.
fn is_document(source: &Source, expression: &Expression, depth: u8) -> bool {
    match expression {
        Expression::Identifier(name) => {
            if name.name == "document" && symbol_of(source, name).is_none() {
                return true;
            }
            depth < 8
                && crate::rules::unicorn::syntax::const_init(source, expression)
                    .is_some_and(|init| is_document(source, init, depth + 1))
        }
        Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
            static_property_name(expression).as_deref() == Some("document")
                && !matches!(expression, Expression::StaticMemberExpression(member) if member.optional)
                && member_object(expression).is_some_and(|object| matches!(object,
                    Expression::Identifier(name) if matches!(name.name.as_str(), "globalThis" | "self" | "window")
                        && symbol_of(source, name).is_none()))
        }
        _ => false,
    }
}

/// The tag a selector of one element names: a type, then classes and IDs only.
fn selector_tag(selector: &str) -> Option<String> {
    let selector = selector.trim();
    let end = selector.find(['.', '#']).unwrap_or(selector.len());
    let (tag, rest) = selector.split_at(end);
    let is_name = |text: &str| {
        !text.is_empty()
            && text.chars().all(|each| {
                each.is_ascii_alphanumeric() || matches!(each, '-' | '_') || !each.is_ascii()
            })
    };
    let starts = tag
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_' || !first.is_ascii());
    if !starts || !is_name(tag) {
        return None;
    }
    let mut remaining = rest;
    while !remaining.is_empty() {
        let after = &remaining[1..];
        let next = after.find(['.', '#']).unwrap_or(after.len());
        if !is_name(&after[..next]) {
            return None;
        }
        remaining = &after[next..];
    }
    let tag = tag.to_ascii_lowercase();
    (!AMBIGUOUS_TAGS.contains(&tag.as_str())).then_some(tag)
}

/// The native element types a written type names; `None` when it names something else.
fn annotation_types(source: &Source, written: &TSType, depth: u8) -> Option<Vec<&'static str>> {
    if depth > 8 {
        return None;
    }
    match written {
        TSType::TSParenthesizedType(inner) => {
            annotation_types(source, &inner.type_annotation, depth + 1)
        }
        TSType::TSNullKeyword(_) | TSType::TSUndefinedKeyword(_) => Some(Vec::new()),
        TSType::TSUnionType(union) => {
            let mut types = Vec::new();
            for member in &union.types {
                for each in annotation_types(source, member, depth + 1)? {
                    if !types.contains(&each) {
                        types.push(each);
                    }
                }
            }
            Some(types)
        }
        TSType::TSTypeReference(reference) => {
            let TSTypeName::IdentifierReference(name) = &reference.type_name else {
                return None;
            };
            let Some(symbol) = symbol_of(source, name) else {
                return ELEMENT_TAGS
                    .iter()
                    .find(|(each, _)| *each == name.name && is_native(each))
                    .map(|(each, _)| vec![*each]);
            };
            match source
                .semantic
                .nodes()
                .kind(source.semantic.scoping().symbol_declaration(symbol))
            {
                AstKind::TSTypeAliasDeclaration(alias) => {
                    annotation_types(source, &alias.type_annotation, depth + 1)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// The native element types a receiver may be; `None` when unknown.
fn receiver_types(
    source: &Source,
    expression: &Expression,
    depth: u8,
) -> Option<Vec<&'static str>> {
    if depth > 8 {
        return None;
    }
    match expression {
        Expression::ChainExpression(chain) => match &chain.expression {
            oxc_ast::ast::ChainElement::CallExpression(call) => call_types(source, call),
            _ => None,
        },
        Expression::TSNonNullExpression(inner) => {
            receiver_types(source, &inner.expression, depth + 1)
        }
        Expression::TSSatisfiesExpression(inner) => {
            receiver_types(source, &inner.expression, depth + 1)
        }
        Expression::TSAsExpression(assertion) => {
            annotation_types(source, &assertion.type_annotation, depth)
        }
        Expression::TSTypeAssertion(assertion) => {
            annotation_types(source, &assertion.type_annotation, depth)
        }
        Expression::Identifier(name) => {
            let symbol = symbol_of(source, name)?;
            let nodes = source.semantic.nodes();
            let declaration = source.semantic.scoping().symbol_declaration(symbol);
            match nodes.kind(declaration) {
                AstKind::VariableDeclarator(declarator) => {
                    let is_const = matches!(nodes.parent_kind(declaration),
                        AstKind::VariableDeclaration(statement) if statement.kind.is_const());
                    if let Some(init) = declarator
                        .init
                        .as_ref()
                        .filter(|_| declarator.id.get_binding_identifier().is_some())
                        && let Some(types) = receiver_types(source, init, depth + 1)
                        && (is_const || types.is_empty())
                    {
                        return Some(types);
                    }
                    annotation_types(
                        source,
                        &declarator.type_annotation.as_ref()?.type_annotation,
                        depth,
                    )
                }
                AstKind::FormalParameter(parameter) => annotation_types(
                    source,
                    &parameter.type_annotation.as_ref()?.type_annotation,
                    depth,
                ),
                _ => None,
            }
        }
        Expression::CallExpression(call) => call_types(source, call),
        _ => None,
    }
}

/// The native element types a call of the document gives: `createElement(…)`,
/// `createElementNS(…)` in the HTML namespace or a simple `querySelector(…)`.
fn call_types(source: &Source, call: &oxc_ast::ast::CallExpression) -> Option<Vec<&'static str>> {
    let method = static_property_name(&call.callee)?;
    if !matches!(
        method.as_str(),
        "createElement" | "createElementNS" | "querySelector"
    ) {
        return None;
    }
    if !member_object(&call.callee).is_some_and(|object| is_document(source, object, 0)) {
        return Some(Vec::new());
    }
    let argument = |index: usize| {
        call.arguments[index]
            .as_expression()
            .and_then(|each| string_value(source, each))
    };
    let tag = match (method.as_str(), call.arguments.len()) {
        ("createElement", 1) => argument(0).map(|name| name.to_ascii_lowercase()),
        ("createElementNS", 2) if argument(0).as_deref() == Some(HTML_NAMESPACE) => argument(1),
        ("querySelector", 1) => argument(0).and_then(|selector| selector_tag(&selector)),
        _ => None,
    };
    Some(tag.and_then(|tag| type_of_tag(&tag)).into_iter().collect())
}

impl Rule for NoInvalidBooleanAttributeValue {
    fn name(&self) -> &'static str {
        "unicorn/no-invalid-boolean-attribute-value"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        if call.arguments.len() != 2
            || call.arguments.iter().any(Argument::is_spread)
            || static_property_name(&call.callee).as_deref() != Some("setAttribute")
        {
            return;
        }
        let (Some(name), Some(value)) = (
            call.arguments[0].as_expression(),
            call.arguments[1].as_expression(),
        ) else {
            return;
        };
        let Some(attribute) = string_value(source, name).map(|name| name.to_ascii_lowercase())
        else {
            return;
        };
        let global = GLOBAL_BOOLEANS.contains(&attribute.as_str());
        let owners = BOOLEANS
            .iter()
            .find(|(each, _)| *each == attribute)
            .map(|(_, owners)| *owners);
        if !global && owners.is_none() {
            return;
        }
        let Some(written) = static_value(source, value) else {
            return;
        };
        let text = as_string(&written).to_ascii_lowercase();
        if text.is_empty() || text == attribute {
            return;
        }
        let Some(types) =
            member_object(&call.callee).and_then(|object| receiver_types(source, object, 0))
        else {
            return;
        };
        if types.is_empty()
            || (!global
                && types
                    .iter()
                    .any(|each| !owners.unwrap_or_default().contains(each)))
        {
            return;
        }
        findings.push(source.finding(
            self.name(),
            value.span().start,
            format!("Invalid value for boolean attribute `{attribute}`. Use an empty string or the attribute name; remove the attribute to represent false."),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidBooleanAttributeValue;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoInvalidBooleanAttributeValue, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_values_of_boolean_attributes() {
        // The documentation's cases.
        assert_eq!(
            check(
                "const button = document.createElement('button');\nbutton.setAttribute('disabled', 'false');\nbutton.setAttribute('disabled', false);\nbutton.setAttribute('disabled', 'true');\nbutton.setAttribute('disabled', ' disabled ');\nconst input = document.querySelector('input.required');\ninput?.setAttribute('required', 'false');"
            ),
            [(2, 33), (3, 33), (4, 33), (5, 33), (7, 33)]
        );
        // A global attribute, an annotation, an alias of `document`, a number.
        assert_eq!(
            check(
                "function f(d: HTMLDivElement, v: HTMLVideoElement | null) {\n  d.setAttribute('inert', 'no');\n  v.setAttribute('MUTED', 1);\n}\nconst doc = window.document;\ndoc.createElementNS('http://www.w3.org/1999/xhtml', 'details').setAttribute('open', 'yes');"
            ),
            [(2, 27), (3, 27), (6, 85)]
        );
    }

    #[test]
    fn leaves_valid_values_and_unknown_elements() {
        for code in [
            "const button = document.createElement('button');\nbutton.removeAttribute('disabled');\nbutton.setAttribute('disabled', '');\nbutton.setAttribute('disabled', 'disabled');\nbutton.setAttribute('disabled', 'DISABLED');",
            "const input = document.querySelector('input.required');\ninput?.removeAttribute('required');\ninput?.setAttribute('required', '');",
            "element.setAttribute('disabled', 'false');",
            "function f(e: HTMLElement) { e.setAttribute('disabled', 'false'); }",
            "const a = document.createElement('div'); a.setAttribute('disabled', 'false');",
            "const a = document.querySelector('a.x'); a.setAttribute('autofocus', 'false');",
            "const a = document.querySelector('div span'); a.setAttribute('autofocus', 'false');",
            "const a = other.createElement('input'); a.setAttribute('checked', 'false');",
            "const a = document.createElement('input'); a.setAttribute('hidden', 'false');",
            "const a = document.createElement('input'); a.setAttribute('checked', value);",
            "const a = document.createElement('input'); a.setAttribute('aria-checked', 'false');",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
