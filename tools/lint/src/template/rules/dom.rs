//! The names of the DOM's elements, as `getDomElements` of `@angular-eslint/eslint-plugin-template`
//! 22.6.0 gives them: the keys of `dom` of `aria-query` 5.3.2 (`src/domMap.js`, Apache-2.0), then
//! the plugin's `nonAriaElements`, `:svg:svg` and `:math:math` written as Angular's compiler names them
//! (REF-ARIA-QUERY-DOM).

/// The DOM's elements, in that order.
pub const DOM_ELEMENTS: [&str; 135] = [
    "a",
    "abbr",
    "acronym",
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "big",
    "blink",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
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
    "menu",
    "menuitem",
    "meta",
    "meter",
    "nav",
    "noembed",
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
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "small",
    "source",
    "spacer",
    "span",
    "strike",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "tt",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "xmp",
    "slot",
    "rb",
    "template",
    "selectedcontent",
    ":svg:svg",
    ":math:math",
];

/// Whether a name is one of the DOM's elements, as written (`toPattern`, case kept).
#[must_use]
pub fn is_dom_element(name: &str) -> bool {
    DOM_ELEMENTS.contains(&name)
}

/// Whether a name is one of the DOM's elements written in lower or upper case, as the rules
/// that match `toPattern([...domElements, ...uppercaseDomElements])` read it.
#[must_use]
pub fn is_dom_name(name: &str) -> bool {
    is_dom_element(name) || (name == name.to_uppercase() && is_dom_element(&name.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::is_dom_name;

    #[test]
    fn reads_a_dom_name_in_lower_or_upper_case_only() {
        // Our own cases, after `toPattern([...domElements, ...uppercaseDomElements])`.
        assert!(is_dom_name("div") && is_dom_name("DIV"));
        assert!(!is_dom_name("Div") && !is_dom_name("app-x"));
        // `:svg:svg` upper-cased is `:SVG:SVG`.
        assert!(is_dom_name(":svg:svg") && is_dom_name(":SVG:SVG"));
    }
}
