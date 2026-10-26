//! What the reader knows of each HTML element: whether it is void, which children close it, how
//! its content is read, its namespace; and the names of a namespaced element (`:svg:path`). A port
//! of `getHtmlTagDefinition` and `HtmlTagDefinition` (`ml_parser/html_tags.ts`), of the namespace
//! helpers of `ml_parser/tags.ts`, and of the element names `DomElementSchemaRegistry` knows
//! (`schema/dom_element_schema_registry.ts`, its `SCHEMA`'s names outside a namespace, lowercased),
//! all of `@angular/compiler` 22.2.3 (REF-ANGULAR-ML-PARSER). The compiler's flags `isVoid` and
//! `canSelfClose` are one `Kind` here, its `implicitNamespacePrefix` and
//! `preventNamespaceInheritance` one `Namespace`, and the `<title>` read as markup under `svg`
//! one `ContentType`.

/// How an element's content is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentType {
    /// Text up to the closing tag, entities left as written (`<script>`, `<style>`).
    RawText,
    /// Text up to the closing tag, entities decoded (`<textarea>`).
    EscapableRawText,
    /// Escapable raw text, but markup under the `svg` prefix (`<title>`, `<svg:title>`).
    EscapableRawTextOutsideSvg,
    /// Elements, text, blocks and the rest.
    ParsableData,
}

/// Whether an element is void, known or custom.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// No content nor end tag; never self-closed by the compiler's flag, though the parser
    /// accepts `<br/>` for being void.
    Void,
    /// An element of the DOM schema: self-closing it is an error.
    Known,
    /// Any other element, a component's: it may be self-closed.
    Custom,
}

/// The namespace an element gives its children.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Namespace {
    /// None of its own: its children take its parent's.
    Inherited,
    /// Its children take this one (`svg`, `math`).
    Implicit(&'static str),
    /// It takes this one, its children none (`<foreignObject>`).
    ImplicitNotInherited(&'static str),
}

/// What the reader knows of an element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagDefinition {
    /// Void, known or custom.
    pub kind: Kind,
    /// The children whose start tag closes it (`<li>` closes an open `<li>`).
    pub closed_by_children: &'static [&'static str],
    /// Whether its parent's end tag closes it without an end tag of its own.
    pub closed_by_parent: bool,
    /// Its namespace.
    pub namespace: Namespace,
    /// How its content is read.
    pub content_type: ContentType,
    /// Whether a line feed opening its content is dropped.
    pub ignore_first_lf: bool,
}

impl TagDefinition {
    /// A known element with the compiler's defaults: read as markup, closed by its end tag.
    const PLAIN: Self = Self {
        kind: Kind::Known,
        closed_by_children: &[],
        closed_by_parent: false,
        namespace: Namespace::Inherited,
        content_type: ContentType::ParsableData,
        ignore_first_lf: false,
    };

    /// A void element: no content, closed by its parent.
    const VOID: Self = Self {
        kind: Kind::Void,
        closed_by_parent: true,
        ..Self::PLAIN
    };

    /// Whether it never has content nor an end tag.
    #[must_use]
    pub fn is_void(&self) -> bool {
        self.kind == Kind::Void
    }

    /// Whether it may be written self-closed (`canSelfClose`).
    #[must_use]
    pub fn can_self_close(&self) -> bool {
        self.kind == Kind::Custom
    }

    /// The namespace its children take (`implicitNamespacePrefix`).
    #[must_use]
    pub fn implicit_namespace_prefix(&self) -> Option<&'static str> {
        match self.namespace {
            Namespace::Inherited => None,
            Namespace::Implicit(prefix) | Namespace::ImplicitNotInherited(prefix) => Some(prefix),
        }
    }

    /// Whether its children leave its namespace (`preventNamespaceInheritance`).
    #[must_use]
    pub fn prevent_namespace_inheritance(&self) -> bool {
        matches!(self.namespace, Namespace::ImplicitNotInherited(_))
    }

    /// Whether the start tag of a child closes this element.
    #[must_use]
    pub fn is_closed_by_child(&self, name: &str) -> bool {
        let lower = name.to_lowercase();
        self.is_void() || self.closed_by_children.contains(&lower.as_str())
    }

    /// How the content is read under a tag's prefix, empty when it has none.
    #[must_use]
    pub fn content_type(&self, prefix: &str) -> ContentType {
        match self.content_type {
            ContentType::EscapableRawTextOutsideSvg if prefix == "svg" => ContentType::ParsableData,
            ContentType::EscapableRawTextOutsideSvg => ContentType::EscapableRawText,
            other => other,
        }
    }
}

/// The elements `DomElementSchemaRegistry` knows outside a namespace, lowercased: written
/// self-closed, they are an error, where an unknown (custom) element may be.
const KNOWN_ELEMENTS: [&str; 129] = [
    "[element]",
    "[htmlelement]",
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "content",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "font",
    "footer",
    "form",
    "frame",
    "frameset",
    "geolocation",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "keygen",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "marquee",
    "media",
    "menu",
    "menuitem",
    "meta",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "search",
    "section",
    "select",
    "selectedcontent",
    "slot",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "u",
    "ul",
    "unknown",
    "var",
    "video",
    "wbr",
];

/// The definition the compiler gives an element written under a name, as `getHtmlTagDefinition`
/// looks it up: the name, then the name lowercased, then a custom element's.
#[must_use]
pub fn definition(name: &str) -> TagDefinition {
    specific(name)
        .or_else(|| specific(&name.to_lowercase()))
        .unwrap_or(TagDefinition {
            kind: Kind::Custom,
            ..TagDefinition::PLAIN
        })
}

/// The definition of an element under its exact name, `None` for a custom element.
fn specific(name: &str) -> Option<TagDefinition> {
    let plain = TagDefinition::PLAIN;
    let closed = |children: &'static [&'static str], by_parent: bool| TagDefinition {
        closed_by_children: children,
        closed_by_parent: by_parent,
        ..plain
    };
    let definition = match name {
        "base" | "meta" | "area" | "embed" | "link" | "img" | "input" | "param" | "hr" | "br"
        | "source" | "track" | "wbr" | "col" => TagDefinition::VOID,
        "p" => closed(
            &[
                "address",
                "article",
                "aside",
                "blockquote",
                "div",
                "dl",
                "fieldset",
                "footer",
                "form",
                "h1",
                "h2",
                "h3",
                "h4",
                "h5",
                "h6",
                "header",
                "hgroup",
                "hr",
                "main",
                "nav",
                "ol",
                "p",
                "pre",
                "section",
                "table",
                "ul",
            ],
            true,
        ),
        "thead" => closed(&["tbody", "tfoot"], false),
        "tbody" => closed(&["tbody", "tfoot"], true),
        "tfoot" => closed(&["tbody"], true),
        "tr" => closed(&["tr"], true),
        "td" | "th" => closed(&["td", "th"], true),
        "svg" => TagDefinition {
            namespace: Namespace::Implicit("svg"),
            ..plain
        },
        "foreignObject" => TagDefinition {
            namespace: Namespace::ImplicitNotInherited("svg"),
            ..plain
        },
        "math" => TagDefinition {
            namespace: Namespace::Implicit("math"),
            ..plain
        },
        "li" => closed(&["li"], true),
        "dt" => closed(&["dt", "dd"], false),
        "dd" => closed(&["dt", "dd"], true),
        "rb" | "rt" | "rp" => closed(&["rb", "rt", "rtc", "rp"], true),
        "rtc" => closed(&["rb", "rtc", "rp"], true),
        "optgroup" => closed(&["optgroup"], true),
        "option" => closed(&["option", "optgroup"], true),
        "pre" | "listing" => TagDefinition {
            ignore_first_lf: true,
            ..plain
        },
        "style" | "script" => TagDefinition {
            content_type: ContentType::RawText,
            ..plain
        },
        "title" => TagDefinition {
            content_type: ContentType::EscapableRawTextOutsideSvg,
            ..plain
        },
        "textarea" => TagDefinition {
            content_type: ContentType::EscapableRawText,
            ignore_first_lf: true,
            ..plain
        },
        known if KNOWN_ELEMENTS.contains(&known) => plain,
        _ => return None,
    };
    Some(definition)
}

/// A namespaced name split into its namespace and its local name: `:svg:path` gives
/// `(Some("svg"), "path")`, `div` gives `(None, "div")` (`splitNsName`, not fatal).
#[must_use]
pub fn split_ns_name(name: &str) -> (Option<&str>, &str) {
    let Some(rest) = name.strip_prefix(':') else {
        return (None, name);
    };
    match rest.find(':') {
        Some(colon) => (Some(&rest[..colon]), &rest[colon + 1..]),
        None => (None, name),
    }
}

/// The namespace of a full name, `None` outside one (`getNsPrefix`).
#[must_use]
pub fn ns_prefix(name: &str) -> Option<&str> {
    split_ns_name(name).0
}

/// A local name under a namespace, `:svg:path`, or the name alone without one (`mergeNsAndName`).
#[must_use]
pub fn merge_ns_and_name(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_owned()
    } else {
        format!(":{prefix}:{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ContentType, KNOWN_ELEMENTS, definition, merge_ns_and_name, ns_prefix, split_ns_name,
    };

    #[test]
    fn knows_the_schemas_129_elements_sorted() {
        assert!(KNOWN_ELEMENTS.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn gives_void_known_custom_and_special_elements_their_definition() {
        let input = definition("input");
        assert!(input.is_void() && input.closed_by_parent && !input.can_self_close());
        assert!(input.is_closed_by_child("span"));
        assert!(!definition("div").can_self_close());
        assert!(!definition("DIV").can_self_close());
        assert!(definition("app-item").can_self_close());
        assert!(definition("li").is_closed_by_child("LI"));
        assert!(!definition("li").is_closed_by_child("div"));
        assert_eq!(definition("svg").implicit_namespace_prefix(), Some("svg"));
        assert!(definition("foreignObject").prevent_namespace_inheritance());
        assert_eq!(definition("math").implicit_namespace_prefix(), Some("math"));
        assert!(definition("pre").ignore_first_lf && definition("textarea").ignore_first_lf);
        assert_eq!(definition("script").content_type(""), ContentType::RawText);
        assert_eq!(
            definition("textarea").content_type(""),
            ContentType::EscapableRawText
        );
        assert_eq!(
            definition("title").content_type(""),
            ContentType::EscapableRawText
        );
        assert_eq!(
            definition("title").content_type("svg"),
            ContentType::ParsableData
        );
        assert!(definition("tbody").closed_by_parent && !definition("thead").closed_by_parent);
        assert!(definition("dd").closed_by_parent && !definition("dt").closed_by_parent);
        assert!(definition("rtc").is_closed_by_child("rb"));
        assert!(definition("option").is_closed_by_child("optgroup"));
        assert!(definition("p").is_closed_by_child("ul"));
        assert!(definition("td").is_closed_by_child("th"));
        assert!(definition("tr").is_closed_by_child("tr"));
        assert!(definition("tfoot").is_closed_by_child("tbody"));
    }

    #[test]
    fn splits_and_merges_namespaced_names() {
        assert_eq!(split_ns_name(":svg:path"), (Some("svg"), "path"));
        assert_eq!(split_ns_name("div"), (None, "div"));
        assert_eq!(split_ns_name(":broken"), (None, ":broken"));
        assert_eq!(ns_prefix(":math:mi"), Some("math"));
        assert_eq!(merge_ns_and_name("svg", "g"), ":svg:g");
        assert_eq!(merge_ns_and_name("", "g"), "g");
    }
}
