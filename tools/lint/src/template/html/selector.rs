//! The CSS selectors html-validate's metadata and rules match elements against (`matches`,
//! `closest`, `querySelectorAll` of `src/dom/selector/` of `html-validate` 11.17.0,
//! REF-HTML-VALIDATE-SOURCE), the part they write: a list of complex selectors joined by `,`, each
//! of compounds joined by a descendant (` `) or child (`>`) combinator, a compound a tag name or
//! `*` and its attribute conditions, `[name]` or `[name="value"]`.

use super::view::Doc;
use crate::javascript::JsText;

/// A condition on an attribute.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Condition {
    /// The attribute's name.
    name: String,
    /// The value it must have, `None` for any.
    value: Option<String>,
}

/// A compound: a tag and conditions.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Compound {
    /// The tag, `*` for any.
    tag: String,
    /// The conditions.
    conditions: Vec<Condition>,
    /// Whether the compound before it must be its parent (`>`) rather than an ancestor.
    child: bool,
}

/// A selector list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector {
    /// Each complex selector's compounds, the leftmost first.
    complexes: Vec<Vec<Compound>>,
}

/// A compound read from its text.
fn compound(text: &str, child: bool) -> Compound {
    let (tag, rest) = text.find('[').map_or((text, ""), |at| text.split_at(at));
    let conditions = rest
        .split('[')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let inner = part.trim_end_matches(']');
            match inner.split_once('=') {
                Some((name, value)) => Condition {
                    name: name.to_lowercase(),
                    value: Some(value.trim_matches(['"', '\'']).to_owned()),
                },
                None => Condition {
                    name: inner.to_lowercase(),
                    value: None,
                },
            }
        })
        .collect();
    Compound {
        tag: if tag.is_empty() {
            "*".to_owned()
        } else {
            tag.to_lowercase()
        },
        conditions,
        child,
    }
}

impl Selector {
    /// Reads a selector list.
    #[must_use]
    pub fn parse(text: &str) -> Self {
        let complexes = text
            .split(',')
            .map(|complex| {
                let spaced = complex.replace('>', " > ");
                let mut compounds = Vec::new();
                let mut child = false;
                for part in spaced.js_split_whitespace() {
                    if part == ">" {
                        child = true;
                    } else {
                        compounds.push(compound(part, child));
                        child = false;
                    }
                }
                compounds
            })
            .filter(|compounds| !compounds.is_empty())
            .collect();
        Self { complexes }
    }

    /// Whether an element matches one of the list's selectors.
    #[must_use]
    pub fn matches(&self, doc: &Doc, index: usize) -> bool {
        self.complexes
            .iter()
            .any(|compounds| matches_complex(doc, index, compounds))
    }
}

/// Whether an element matches a compound.
fn matches_compound(doc: &Doc, index: usize, compound: &Compound) -> bool {
    doc.is(index, &compound.tag)
        && compound.conditions.iter().all(|condition| {
            doc.attrs(index, &condition.name)
                .any(|attr| match &condition.value {
                    None => true,
                    Some(value) => attr.text() == Some(value.as_str()),
                })
        })
}

/// Whether an element matches a complex selector, read from its rightmost compound.
fn matches_complex(doc: &Doc, index: usize, compounds: &[Compound]) -> bool {
    let Some((last, before)) = compounds.split_last() else {
        return false;
    };
    if !matches_compound(doc, index, last) {
        return false;
    }
    if before.is_empty() {
        return true;
    }
    let mut parent = doc.el(index).parent;
    while let Some(ancestor) = parent {
        if matches_complex(doc, ancestor, before) {
            return true;
        }
        if last.child {
            return false;
        }
        parent = doc.el(ancestor).parent;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::super::view::Doc;
    use crate::template::tree::parse;

    #[test]
    fn matches_the_selectors_html_validate_s_metadata_writes() {
        // Our own cases: a tag, `*`, a list, a child and a descendant combinator, an attribute
        // present or of a value.
        let text = "<dl><div><dd role=\"x\"></dd></div></dl>";
        let parsed = parse(text);
        let doc = Doc::new(text, &parsed.nodes);

        assert!(doc.matches(2, "dd"));
        assert!(doc.matches(2, "*"));
        assert!(doc.matches(2, "dt, dd"));
        assert!(doc.matches(2, "dl > div > dd"));
        assert!(!doc.matches(2, "dl > dd"));
        assert!(doc.matches(2, "dl dd"));
        assert!(doc.matches(2, "[role]"));
        assert!(doc.matches(2, "dd[role=\"x\"]"));
        assert!(!doc.matches(2, "[role=\"y\"]"));
        assert_eq!(doc.closest(2, "dl"), Some(0));
        assert_eq!(doc.closest(2, "table"), None);
    }
}
