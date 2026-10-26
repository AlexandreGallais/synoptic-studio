//! Whether an element is in the accessibility tree, a port of `src/rules/helper/a11y.ts` of
//! `html-validate` 11.17.0 (REF-HTML-VALIDATE-SOURCE), with `parseCssDeclaration`
//! (`src/dom/css/parse-css-declaration.ts`): hidden by `aria-hidden="true"`, by `hidden`, by
//! `inert`, by an inline `display: none` or `visibility: hidden`, itself or through its parent;
//! or presentational by its role. A bound `hidden`, `inert` or `style` hides nothing, its value
//! unknown.

use super::view::{Doc, Value};

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

#[cfg(test)]
mod tests {
    use super::css_declarations;

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
