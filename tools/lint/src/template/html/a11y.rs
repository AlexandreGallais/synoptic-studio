//! Whether an element is in the accessibility tree, a port of `src/rules/helper/a11y.ts` of
//! `html-validate` 11.17.0 (REF-HTML-VALIDATE-SOURCE), with `parseCssDeclaration`
//! (`src/dom/css/parse-css-declaration.ts`): hidden by `aria-hidden="true"`, by `hidden`, by
//! `inert`, by an inline `display: none` or `visibility: hidden`, itself or through its parent;
//! or presentational by its role. A bound `hidden`, `inert` or `style` hides nothing, its value
//! unknown.

use super::elements::js_number;
use super::view::{Child, Doc, Value};
use crate::template::lexer::TokenType;

/// Whether hidden by itself and by its parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hidden {
    /// By an ancestor.
    pub by_parent: bool,
    /// By the element itself.
    pub by_self: bool,
}

impl Hidden {
    /// Whether hidden either way.
    #[must_use]
    pub const fn any(self) -> bool {
        self.by_parent || self.by_self
    }
}

/// Whether an element or an ancestor holds what hides.
fn hidden_by(doc: &Doc, index: usize, holds: &dyn Fn(&Doc, usize) -> bool) -> Hidden {
    Hidden {
        by_parent: doc
            .ancestors(index)
            .into_iter()
            .any(|ancestor| holds(doc, ancestor)),
        by_self: holds(doc, index),
    }
}

/// `isAriaHidden`: `aria-hidden="true"`, written.
#[must_use]
pub fn aria_hidden(doc: &Doc, index: usize) -> Hidden {
    hidden_by(doc, index, &|doc, each| {
        doc.value(each, "aria-hidden") == Some("true")
    })
}

/// Whether an attribute is written, not bound (`isStatic`).
fn has_static(doc: &Doc, index: usize, key: &str) -> bool {
    doc.attr(index, key)
        .is_some_and(|attr| attr.value != Value::Dynamic)
}

/// `isHTMLHidden`: `hidden`, written.
#[must_use]
pub fn html_hidden(doc: &Doc, index: usize) -> Hidden {
    hidden_by(doc, index, &|doc, each| has_static(doc, each, "hidden"))
}

/// `isInert`: `inert`, written.
#[must_use]
pub fn inert(doc: &Doc, index: usize) -> Hidden {
    hidden_by(doc, index, &|doc, each| has_static(doc, each, "inert"))
}

/// An inline style's declarations, the last of a property winning (`parseCssDeclaration`).
#[must_use]
pub fn css_declarations(value: &str) -> Vec<(String, String)> {
    let mut found: Vec<(String, String)> = Vec::new();
    for declaration in value.trim().split(';').filter(|each| !each.is_empty()) {
        let mut parts = declaration.splitn(3, ':');
        let property = parts.next().unwrap_or_default().trim().to_owned();
        let value = parts.next().unwrap_or_default().trim().to_owned();
        found.retain(|(each, _)| *each != property);
        found.push((property, value));
    }
    found
}

/// `isStyleHidden`: an inline `display: none` or `visibility: hidden`.
#[must_use]
pub fn style_hidden(doc: &Doc, index: usize) -> bool {
    hidden_by(doc, index, &|doc, each| {
        doc.value(each, "style").is_some_and(|style| {
            css_declarations(style).iter().any(|(property, value)| {
                (property == "display" && value == "none")
                    || (property == "visibility" && value == "hidden")
            })
        })
    })
    .any()
}

/// `isPresentation`: `role="presentation"` or `role="none"` on an element neither interactive
/// nor given a `tabindex`.
#[must_use]
pub fn presentation(doc: &Doc, index: usize) -> bool {
    let interactive = doc
        .el(index)
        .meta
        .is_some_and(|meta| meta.interactive.of(doc, index));
    if interactive || doc.has(index, "tabindex") {
        return false;
    }
    matches!(doc.value(index, "role"), Some("presentation" | "none"))
}

/// `inAccessibilityTree`.
#[must_use]
pub fn in_accessibility_tree(doc: &Doc, index: usize) -> bool {
    !aria_hidden(doc, index).any()
        && !presentation(doc, index)
        && !html_hidden(doc, index).any()
        && !inert(doc, index).any()
        && !style_hidden(doc, index)
}

/// What text an element holds (`TextClassification`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextClass {
    /// None, or whitespace only.
    Empty,
    /// Known at run time only.
    Dynamic,
    /// Written.
    Static,
}

/// Whether an element's content is known only at run time, a reading of Angular's templates of
/// our own: a component (an element HTML does not know, never a foreign one's child) renders its
/// own template, and a bound `innerHTML`, `innerText` or `textContent` sets the content.
fn renders_at_run_time(doc: &Doc, index: usize) -> bool {
    let element = doc.el(index);
    let component = element.meta.is_none() && element.tag.contains('-');
    let bound = ["innerhtml", "innertext", "textcontent"]
        .iter()
        .any(|key| doc.attr(index, key).is_some_and(|attr| attr.bound));
    component || bound
}

/// Adds the texts of an element's children (`findTextNodes`): an element hidden by itself left
/// out, by `aria-hidden` too when the accessible text is asked; whether one is dynamic.
fn find_text(doc: &Doc, index: usize, accessible: bool, text: &mut String) -> bool {
    let mut dynamic = renders_at_run_time(doc, index);
    for child in &doc.el(index).children {
        match child {
            Child::Text(node) => {
                for token in &node.tokens {
                    match token.kind {
                        TokenType::Interpolation => dynamic = true,
                        TokenType::Text
                        | TokenType::EncodedEntity
                        | TokenType::EscapableRawText => {
                            text.push_str(token.parts.first().map_or("", String::as_str));
                        }
                        _ => {}
                    }
                }
            }
            Child::Element(each) => {
                if html_hidden(doc, *each).by_self
                    || (accessible && aria_hidden(doc, *each).by_self)
                {
                    continue;
                }
                dynamic |= find_text(doc, *each, accessible, text);
            }
        }
    }
    dynamic
}

/// `classifyNodeText`: an element hidden, a `<select>` and a `<textarea>` hold no text; else the
/// text of its children, dynamic when one is.
#[must_use]
pub fn classify_text(
    doc: &Doc,
    index: usize,
    accessible: bool,
    ignore_hidden_root: bool,
) -> TextClass {
    if !ignore_hidden_root
        && (html_hidden(doc, index).any() || (accessible && aria_hidden(doc, index).any()))
    {
        return TextClass::Empty;
    }
    if matches!(doc.el(index).tag.as_str(), "select" | "textarea") {
        return TextClass::Empty;
    }
    let mut text = String::new();
    if find_text(doc, index, accessible, &mut text) {
        TextClass::Dynamic
    } else if text.chars().any(|character| !character.is_whitespace()) {
        TextClass::Static
    } else {
        TextClass::Empty
    }
}

/// `hasAltText`: an `alt` with a value, bound or written.
fn has_alt_text(doc: &Doc, index: usize) -> bool {
    doc.attr(index, "alt")
        .is_some_and(|attr| match &attr.value {
            Value::Omitted => false,
            Value::Dynamic => true,
            Value::Static(text) => !text.is_empty(),
        })
}

/// `hasImgAltText`: an image with its `alt`, an `<svg>` with a text, or one inside.
fn has_img_alt_text(doc: &Doc, index: usize, reference: Option<usize>) -> bool {
    match doc.el(index).tag.as_str() {
        "img" => return has_alt_text(doc, index),
        "svg" => return !doc.text_content(index).0.trim().is_empty(),
        _ => {}
    }
    doc.descendants(index)
        .into_iter()
        .filter(|each| matches!(doc.el(*each).tag.as_str(), "img" | "svg"))
        .any(|each| has_accessible_name_from(doc, each, reference))
}

/// `hasLabel`: an `aria-label` with a text, a bound one counted.
fn has_label(doc: &Doc, index: usize) -> bool {
    doc.attr(index, "aria-label")
        .is_some_and(|attr| match &attr.value {
            Value::Omitted => false,
            Value::Dynamic => true,
            Value::Static(text) => !text.trim().is_empty(),
        })
}

/// `isLabelledby`: an `aria-labelledby` naming an element of the template with an accessible
/// name, one level deep; a bound one counted.
fn is_labelledby(doc: &Doc, index: usize, reference: Option<usize>) -> bool {
    if reference.is_some() {
        return false;
    }
    let Some(attr) = doc.attr(index, "aria-labelledby") else {
        return false;
    };
    let ids = match &attr.value {
        Value::Dynamic => return true,
        Value::Omitted => return false,
        Value::Static(ids) => ids.split_whitespace().collect::<Vec<_>>(),
    };
    ids.iter().any(|id| {
        (0..doc.elements.len())
            .filter(|each| doc.value(*each, "id") == Some(*id))
            .any(|each| has_accessible_name_from(doc, each, Some(each)))
    })
}

/// `hasAccessibleNameImpl`.
fn has_accessible_name_from(doc: &Doc, index: usize, reference: Option<usize>) -> bool {
    let referenced = reference == Some(index);
    if !referenced && !in_accessibility_tree(doc, index) {
        return false;
    }
    classify_text(doc, index, true, referenced) != TextClass::Empty
        || has_img_alt_text(doc, index, reference)
        || has_label(doc, index)
        || is_labelledby(doc, index, reference)
}

/// `hasAccessibleName` (`src/rules/helper/has-accessible-name.ts`): a text, an image's
/// alternative, an `aria-label`, or an `aria-labelledby` naming an element with a name.
#[must_use]
pub fn has_accessible_name(doc: &Doc, index: usize) -> bool {
    has_accessible_name_from(doc, index, None)
}

/// `isFocusable` (`src/rules/helper/is-focusable.ts`): not hidden; its `tabindex` deciding when
/// it has one (a bound one counted as 0); else its metadata, a disabled control not.
#[must_use]
pub fn is_focusable(doc: &Doc, index: usize) -> bool {
    if html_hidden(doc, index).any() || inert(doc, index).any() || style_hidden(doc, index) {
        return false;
    }
    if let Some(attr) = doc.attr(index, "tabindex") {
        match &attr.value {
            Value::Dynamic => return true,
            Value::Static(text) if !text.is_empty() => {
                if let Some(number) = js_number(text) {
                    return number.trunc() >= 0.0;
                }
            }
            Value::Static(_) | Value::Omitted => {}
        }
    }
    let Some(meta) = doc.el(index).meta else {
        return false;
    };
    let disablable = meta
        .form_associated
        .is_some_and(|associated| associated.disablable);
    if disablable
        && (doc.has(index, "disabled") || doc.closest(index, "fieldset[disabled]").is_some())
    {
        return false;
    }
    meta.focusable.of(doc, index)
}

/// The roles that cannot be named (WAI-ARIA 5.2.8.6, `prohibitedRoles`).
const PROHIBITED_ROLES: [&str; 11] = [
    "caption",
    "code",
    "deletion",
    "emphasis",
    "generic",
    "insertion",
    "paragraph",
    "presentation",
    "strong",
    "subscript",
    "superscript",
];

/// `ariaNaming` (`src/rules/helper/aria-naming.ts`): by an explicit role when written, a bound one
/// allowing it; else by the element's metadata, an unknown element allowing it.
#[must_use]
pub fn naming_allowed(doc: &Doc, index: usize) -> bool {
    match doc.attr(index, "role").map(|attr| &attr.value) {
        Some(Value::Dynamic) => return true,
        Some(Value::Static(role)) if !role.is_empty() => {
            return !PROHIBITED_ROLES.contains(&role.as_str());
        }
        _ => {}
    }
    doc.el(index)
        .meta
        .is_none_or(|meta| meta.naming_allowed(doc, index))
}

#[cfg(test)]
mod tests {
    use super::{
        TextClass, classify_text, css_declarations, has_accessible_name, is_focusable,
        naming_allowed,
    };
    use crate::template::html::view::Doc;
    use crate::template::tree::parse;

    /// A check on the first element of a template.
    fn first(text: &str, check: &dyn Fn(&Doc, usize) -> bool) -> bool {
        let parsed = parse(text);
        let doc = Doc::new(text, &parsed.nodes);
        check(&doc, 0)
    }

    #[test]
    fn names_an_element_as_has_accessible_name_does() {
        // Our own cases: a text, an `aria-label` (bound counted), an `aria-labelledby` of the
        // template, an image's `alt`, a component rendering its own content; none for a text
        // hidden by `aria-hidden` or an empty label.
        let named = |text: &str| first(text, &has_accessible_name);
        assert!(named("<button>a</button>"));
        assert!(named("<button [attr.aria-label]=\"x\"></button>"));
        assert!(named(
            "<button aria-labelledby=\"l\"></button><span id=\"l\">a</span>"
        ));
        assert!(named("<button><img alt=\"a\"></button>"));
        assert!(named("<button><app-icon></app-icon></button>"));
        assert!(named("<button>{{ a }}</button>"));
        assert!(!named(
            "<button><span aria-hidden=\"true\">a</span></button>"
        ));
        assert!(!named("<button aria-label=\" \"></button>"));
        assert!(!named("<button aria-labelledby=\"none\"></button>"));
        assert!(first("<textarea>a</textarea>", &|doc, index| {
            classify_text(doc, index, false, false) == TextClass::Empty
        }));
    }

    #[test]
    fn focuses_an_element_as_is_focusable_does() {
        // Our own cases, after `isFocusable`: a `tabindex` decides first, then a disabled
        // control, then the metadata.
        let focusable = |text: &str| first(text, &is_focusable);
        assert!(focusable("<button></button>"));
        assert!(!focusable("<button disabled></button>"));

        assert!(!focusable("<button tabindex=\"-1\"></button>"));
        assert!(focusable("<div tabindex=\"0\"></div>"));
        assert!(focusable("<div [tabindex]=\"t\"></div>"));
        assert!(!focusable("<div></div>"));
        assert!(!focusable("<button hidden></button>"));
        let in_fieldset = "<fieldset disabled><input></fieldset>";
        let parsed = parse(in_fieldset);
        let doc = Doc::new(in_fieldset, &parsed.nodes);
        assert!(!is_focusable(&doc, 1));
    }

    #[test]
    fn allows_naming_as_aria_naming_does() {
        // Our own cases: a written role decides, a bound one allows, else the metadata.
        let allowed = |text: &str| first(text, &naming_allowed);
        assert!(!allowed("<div role=\"generic\"></div>"));
        assert!(allowed("<span role=\"button\"></span>"));
        assert!(allowed("<span [attr.role]=\"r\"></span>"));
        assert!(!allowed("<span></span>"));
        assert!(allowed("<app-x></app-x>"));
    }

    #[test]
    fn reads_an_inline_style_as_parse_css_declaration_does() {
        // Our own cases: the last declaration of a property wins; a value after a second `:`
        // is dropped, as `split(":", 2)` drops it.
        assert_eq!(
            css_declarations(" display: block; display:none ;a:b:c;"),
            [
                ("display".to_owned(), "none".to_owned()),
                ("a".to_owned(), "b".to_owned())
            ]
        );
    }
}
