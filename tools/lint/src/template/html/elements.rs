//! The metadata of HTML's elements, a port of `src/elements/html5.ts` of `html-validate` 11.17.0
//! (REF-HTML-VALIDATE-SOURCE), as its `MetaTable` loads it (`src/meta/table.ts`, `migrate.ts`):
//! each element's content categories, some by a function of the element; its content model
//! (permitted content, descendants, order and parent, required ancestors and content); its
//! attributes, merged over the global ones (`*`), an element's own taking precedence; its ARIA
//! role and naming. A value enumerated as `"/…/"` is a pattern anchored at both ends unless it
//! anchors itself, named after it; a pattern given as a regular expression is used as written.
//! The fields only html-validate's parser reads (`implicitClosed`, `optionalEnd`,
//! `implicitOpen`) are left out: the template reader closes elements as Angular's parser does;
//! so are the attributes named by a pattern (`aria-*`, `data-*`, `on*`, `xml:*`, `xmlns:*`), which
//! only `no-unknown-attributes` reads, a rule the catalogue skips.

use super::view::{Doc, Value};

/// A category of content, or another property of an element, known or computed.
#[derive(Clone, Copy, Debug)]
pub enum Cat {
    /// Never.
    No,
    /// Always.
    Yes,
    /// By a function of the element.
    When(fn(&Doc, usize) -> bool),
}

impl Cat {
    /// The property for an element.
    #[must_use]
    pub fn of(self, doc: &Doc, index: usize) -> bool {
        match self {
            Self::No => false,
            Self::Yes => true,
            Self::When(function) => function(doc, index),
        }
    }
}

/// An enumerated value of an attribute.
#[derive(Clone, Copy, Debug)]
pub enum Enum {
    /// A word, matched without case.
    Word(&'static str),
    /// A pattern: its name, and the regular expression in Rust's syntax.
    Pattern(&'static str, &'static str),
}

/// When an attribute is allowed (`MetaAttributeAllowedCallback`).
#[derive(Clone, Copy, Debug)]
pub enum Allowed {
    /// `allowedIfAttributeIsPresent`: one of these attributes present.
    IfPresent(&'static [&'static str]),
    /// `allowedIfAttributeIsAbsent`: none of these attributes present.
    IfAbsent(&'static [&'static str]),
    /// `allowedIfAttributeHasValue`: an attribute holding one of the values, a default for one
    /// missing.
    IfValue(&'static str, &'static [&'static str], Option<&'static str>),
    /// `allowedIfParentIsPresent`: one of these elements an ancestor.
    IfParent(&'static [&'static str]),
    /// A function of the element and the attribute's value: why not allowed, `None` when it is.
    When(fn(&Doc, usize, Option<&Value>) -> Option<String>),
}

/// Whether an attribute is required.
#[derive(Clone, Copy, Debug)]
pub enum Required {
    /// No.
    No,
    /// Yes.
    Yes,
    /// By a function of the element: the message's template when required, `None` otherwise.
    When(fn(&Doc, usize) -> Option<&'static str>),
}

/// An attribute's metadata.
#[derive(Clone, Copy, Debug)]
pub struct AttrMeta {
    /// Its name.
    pub name: &'static str,
    /// When allowed.
    pub allowed: Option<Allowed>,
    /// The form of its value.
    pub form: ValueForm,
    /// Whether deprecated.
    pub deprecated: bool,
    /// Its values, none for any.
    pub values: &'static [Enum],
    /// Whether a list of tokens, each checked.
    pub list: bool,
    /// Whether it names an `id`.
    pub reference: bool,
    /// Whether required.
    pub required: Required,
}

/// The form of an attribute's value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueForm {
    /// A value, checked against its values when it has some.
    Text,
    /// Boolean: its value omitted, empty or its own name (`boolean`).
    Boolean,
    /// Its value may be omitted or empty, else checked (`omit`).
    Omissible,
}

/// A deprecated element: its message, its documentation and the standard it comes from.
#[derive(Clone, Copy, Debug)]
pub struct Deprecated {
    /// The message.
    pub message: Option<&'static str>,
    /// The documentation.
    pub documentation: Option<&'static str>,
    /// The source.
    pub source: Option<&'static str>,
}

/// Whether an element's content is transparent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transparent {
    /// No.
    No,
    /// Yes: its content is checked against its parent's.
    Yes,
    /// For these categories or elements only.
    Only(&'static [&'static str]),
}

/// An entry of a permitted content, descendants or parent.
#[derive(Clone, Copy, Debug)]
pub enum Entry {
    /// A category (`@flow`) or a tag, a quantifier (`?`, `*`) after it.
    Item(&'static str),
    /// `{ exclude: [...] }`.
    Exclude(&'static [&'static str]),
}

/// An implicit ARIA role.
#[derive(Clone, Copy, Debug)]
pub enum Role {
    /// None.
    None,
    /// This one.
    Fixed(&'static str),
    /// By a function of the element, always one.
    Of(fn(&Doc, usize) -> &'static str),
    /// By a function of the element, `None` for none.
    When(fn(&Doc, usize) -> Option<&'static str>),
}

/// Whether an element may be named by `aria-label` or `aria-labelledby`.
#[derive(Clone, Copy, Debug)]
pub enum Naming {
    /// Allowed.
    Allowed,
    /// Prohibited.
    Prohibited,
    /// By a function of the element: whether allowed.
    When(fn(&Doc, usize) -> bool),
}

/// What text an element needs (`textContent`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextContent {
    /// None.
    None,
    /// An accessible text.
    Accessible,
}

/// A form-associated element.
#[derive(Clone, Copy, Debug)]
pub struct FormAssociated {
    /// Whether it can be disabled.
    pub disablable: bool,
    /// Whether listed in its form's elements.
    pub listed: bool,
}

/// An element's metadata.
#[derive(Clone, Copy, Debug)]
pub struct Meta {
    /// Its name.
    pub name: &'static str,
    /// Metadata content.
    pub metadata: Cat,
    /// Flow content.
    pub flow: Cat,
    /// Sectioning content.
    pub sectioning: Cat,
    /// Heading content.
    pub heading: Cat,
    /// Phrasing content.
    pub phrasing: Cat,
    /// Embedded content.
    pub embedded: Cat,
    /// Interactive content.
    pub interactive: Cat,
    /// Deprecated, and how.
    pub deprecated: Option<Deprecated>,
    /// Foreign: its content is another language's.
    pub foreign: bool,
    /// Void.
    pub void: bool,
    /// Transparent.
    pub transparent: Transparent,
    /// Script-supporting.
    pub script_supporting: Cat,
    /// Focusable.
    pub focusable: Cat,
    /// A form.
    pub form: Cat,
    /// Form-associated.
    pub form_associated: Option<FormAssociated>,
    /// Labelable.
    pub labelable: Cat,
    /// A submit button.
    pub submit_button: Cat,
    /// A template's root.
    pub template_root: Cat,
    /// Its implicit ARIA role.
    pub implicit_role: Role,
    /// Whether it may be named.
    pub naming: Naming,
    /// Its own attributes, the global ones apart.
    pub attributes: &'static [AttrMeta],
    /// Its permitted content.
    pub permitted_content: Option<&'static [Entry]>,
    /// Its permitted descendants.
    pub permitted_descendants: Option<&'static [Entry]>,
    /// Its children's order.
    pub permitted_order: Option<&'static [&'static str]>,
    /// Its permitted parents.
    pub permitted_parent: Option<&'static [Entry]>,
    /// Its required ancestors, selectors.
    pub required_ancestors: Option<&'static [&'static str]>,
    /// Its required children.
    pub required_content: Option<&'static [&'static str]>,
    /// The text it needs.
    pub text_content: Option<TextContent>,
}

/// An element with nothing set.
const M: Meta = Meta {
    name: "",
    metadata: Cat::No,
    flow: Cat::No,
    sectioning: Cat::No,
    heading: Cat::No,
    phrasing: Cat::No,
    embedded: Cat::No,
    interactive: Cat::No,
    deprecated: None,
    foreign: false,
    void: false,
    transparent: Transparent::No,
    script_supporting: Cat::No,
    focusable: Cat::No,
    form: Cat::No,
    form_associated: None,
    labelable: Cat::No,
    submit_button: Cat::No,
    template_root: Cat::No,
    implicit_role: Role::None,
    naming: Naming::Allowed,
    attributes: &[],
    permitted_content: None,
    permitted_descendants: None,
    permitted_order: None,
    permitted_parent: None,
    required_ancestors: None,
    required_content: None,
    text_content: None,
};

/// An attribute with nothing set.
const fn attr(name: &'static str) -> AttrMeta {
    AttrMeta {
        name,
        allowed: None,
        form: ValueForm::Text,
        deprecated: false,
        values: &[],
        list: false,
        reference: false,
        required: Required::No,
    }
}

/// A deprecated attribute.
const fn dep(name: &'static str) -> AttrMeta {
    AttrMeta {
        deprecated: true,
        ..attr(name)
    }
}

/// A boolean attribute.
const fn boolean(name: &'static str) -> AttrMeta {
    AttrMeta {
        form: ValueForm::Boolean,
        ..attr(name)
    }
}

/// An attribute of enumerated values.
const fn en(name: &'static str, values: &'static [Enum]) -> AttrMeta {
    AttrMeta {
        values,
        ..attr(name)
    }
}

/// An attribute of enumerated values whose value may be omitted.
const fn omit(name: &'static str, values: &'static [Enum]) -> AttrMeta {
    AttrMeta {
        form: ValueForm::Omissible,
        ..en(name, values)
    }
}

/// An attribute naming one `id`.
const fn id_ref(name: &'static str) -> AttrMeta {
    AttrMeta {
        reference: true,
        ..en(name, &[VALID_ID])
    }
}

/// An attribute naming a list of `id`s.
const fn id_refs(name: &'static str) -> AttrMeta {
    AttrMeta {
        list: true,
        ..id_ref(name)
    }
}

/// A word.
use Enum::Word as W;

/// `validId`.
const VALID_ID: Enum = Enum::Pattern("a valid id", r"^\S+$");
/// `validPositiveInteger`.
const VALID_POSITIVE_INTEGER: Enum = Enum::Pattern("a positive integer", r"^\d+$");
/// `validNonEmptyString`.
const VALID_NON_EMPTY_STRING: Enum = Enum::Pattern("a non-empty string", r"^.+$");
/// `validBrowsingContextName`.
const VALID_BROWSING_CONTEXT_NAME: Enum = Enum::Pattern(
    "a browsing context name (non-empty string, must not start with `_`)",
    r"^[^_].*$",
);
/// `validFloatingPoint`, its `i` flag kept.
const VALID_FLOATING_POINT: Enum = Enum::Pattern(
    "a floating-point number",
    r"(?i)^-?(\d+(\.\d+)?|\.\d+)(E[+-]?\d+)?$",
);
/// `ReferrerPolicy`.
const REFERRER_POLICY: &[Enum] = &[
    W(""),
    W("no-referrer"),
    W("no-referrer-when-downgrade"),
    W("same-origin"),
    W("origin"),
    W("strict-origin"),
    W("origin-when-cross-origin"),
    W("strict-origin-when-cross-origin"),
    W("unsafe-url"),
];
/// The targets of a link or a form.
const TARGETS: &[Enum] = &[
    VALID_BROWSING_CONTEXT_NAME,
    W("_blank"),
    W("_self"),
    W("_parent"),
    W("_top"),
];
/// `crossorigin`'s values.
const CROSS_ORIGIN: &[Enum] = &[W("anonymous"), W("use-credentials")];
/// The values of a positive integer.
const POSITIVE_INTEGER: &[Enum] = &[VALID_POSITIVE_INTEGER];
/// The values of a non-empty string.
const NON_EMPTY: &[Enum] = &[VALID_NON_EMPTY_STRING];
/// The values of a floating-point number.
const FLOATING_POINT: &[Enum] = &[VALID_FLOATING_POINT];
/// The flow content.
const FLOW: &[Entry] = &[Entry::Item("@flow")];
/// The phrasing content.
const PHRASING: &[Entry] = &[Entry::Item("@phrasing")];
/// No content.
const NOTHING: &[Entry] = &[];
/// No `<main>` among the descendants.
const NO_MAIN: &[Entry] = &[Entry::Exclude(&["main"])];

/// The attributes every element has (`*`), each element's own taking precedence.
const GLOBAL: &[AttrMeta] = &[
    AttrMeta {
        list: true,
        ..en("accesskey", &[Enum::Pattern("/./u", ".")])
    },
    id_ref("aria-activedescendant"),
    id_refs("aria-controls"),
    id_refs("aria-describedby"),
    id_ref("aria-details"),
    id_ref("aria-errormessage"),
    id_refs("aria-flowto"),
    id_refs("aria-labelledby"),
    id_refs("aria-owns"),
    en(
        "autocapitalize",
        &[
            W("off"),
            W("none"),
            W("on"),
            W("sentences"),
            W("words"),
            W("characters"),
        ],
    ),
    omit("autocorrect", &[W("on"), W("off")]),
    boolean("autofocus"),
    attr("class"),
    omit("contenteditable", &[W("true"), W("false")]),
    dep("contextmenu"),
    en("dir", &[W("ltr"), W("rtl"), W("auto")]),
    en("draggable", &[W("true"), W("false")]),
    en(
        "enterkeyhint",
        &[
            W("enter"),
            W("done"),
            W("go"),
            W("next"),
            W("previous"),
            W("search"),
            W("send"),
        ],
    ),
    attr("exportparts"),
    en("headingoffset", &[Enum::Pattern("/^[0-8]$/", "^[0-8]$")]),
    boolean("headingreset"),
    omit("hidden", &[W("hidden"), W("until-found")]),
    en("id", &[VALID_ID]),
    boolean("inert"),
    en(
        "inputmode",
        &[
            W("none"),
            W("text"),
            W("decimal"),
            W("numeric"),
            W("tel"),
            W("search"),
            W("email"),
            W("url"),
        ],
    ),
    attr("is"),
    attr("itemid"),
    attr("itemprop"),
    attr("itemref"),
    boolean("itemscope"),
    attr("itemtype"),
    attr("lang"),
    attr("nonce"),
    attr("part"),
    omit("popover", &[W("auto"), W("hint"), W("manual")]),
    attr("role"),
    attr("slot"),
    omit("spellcheck", &[W("true"), W("false")]),
    attr("style"),
    en("tabindex", &[Enum::Pattern(r"/^-?\d+$/", r"^-?\d+$")]),
    attr("title"),
    omit("translate", &[W("yes"), W("no")]),
    omit("writingsuggestions", &[W("true"), W("false")]),
    en("xmlns", NON_EMPTY),
];

/// An element's metadata of an attribute: its own, else the global one.
#[must_use]
pub fn attribute(meta: &Meta, key: &str) -> Option<&'static AttrMeta> {
    let key = key.to_lowercase();
    let own: &'static [AttrMeta] = meta.attributes;
    own.iter()
        .chain(GLOBAL.iter())
        .find(|each| each.name == key)
}

/// An attribute's value, as a metadata callback reads it (`getAttribute` of its adapter): `None`
/// when missing.
fn get<'d>(doc: &'d Doc, index: usize, key: &str) -> Option<&'d Value> {
    doc.attr(index, key).map(|attr| &attr.value)
}

/// Whether an attribute is a static string equal to a text.
fn value_is(doc: &Doc, index: usize, key: &str, text: &str) -> bool {
    doc.value(index, key) == Some(text)
}

/// `isInsideLandmark`.
fn is_inside_landmark(doc: &Doc, index: usize) -> bool {
    doc.closest(
        index,
        "article,aside,main,nav,section,[role=\"article\"],[role=\"complementary\"],[role=\"main\"],[role=\"navigation\"],[role=\"region\"]",
    )
    .is_some()
}

/// Whether an `srcset` uses a width descriptor (`parseImageCandidateString`).
fn has_width_descriptor(srcset: &str) -> bool {
    srcset.trim() != ""
        && srcset.split(',').any(|candidate| {
            let parts: Vec<&str> = candidate.split_whitespace().collect();
            parts.len() >= 2
                && parts.last().is_some_and(|descriptor| {
                    let digits = descriptor.strip_suffix(['w', 'W']).unwrap_or("");
                    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
                })
        })
}

/// `linkBodyOk`.
fn link_body_ok(doc: &Doc, index: usize) -> bool {
    if doc.has(index, "itemprop") {
        return true;
    }
    let Some(rel) = get(doc, index, "rel") else {
        return false;
    };
    let Value::Static(rel) = rel else {
        return false;
    };
    if rel.is_empty() {
        return false;
    }
    rel.to_lowercase().split_whitespace().any(|keyword| {
        matches!(
            keyword,
            "dns-prefetch"
                | "modulepreload"
                | "pingback"
                | "preconnect"
                | "prefetch"
                | "preload"
                | "stylesheet"
        )
    })
}

/// The keywords of a `rel` refused by a disallowed set, or a `dcterms.` one when asked:
/// `<tag> does not allow rel="keyword"` for the first.
fn refused_rel(
    value: Option<&Value>,
    tag: &str,
    disallowed: &[&str],
    dcterms: bool,
) -> Option<String> {
    let Some(Value::Static(rel)) = value else {
        return None;
    };
    rel.to_lowercase()
        .split_whitespace()
        .find(|keyword| {
            disallowed.contains(keyword) || (dcterms && keyword.starts_with("dcterms."))
        })
        .map(|keyword| format!("<{tag}> does not allow rel=\"{keyword}\""))
}

/// The keywords `<a>` and `<area>` refuse in `rel`.
const LINK_REL_REFUSED: &[&str] = &[
    "canonical",
    "dns-prefetch",
    "expect",
    "icon",
    "manifest",
    "modulepreload",
    "pingback",
    "preconnect",
    "prefetch",
    "preload",
    "stylesheet",
    "apple-touch-icon",
    "apple-touch-icon-precomposed",
    "apple-touch-startup-image",
    "authorization_endpoint",
    "component",
    "chrome-webstore-item",
    "edit",
    "gbfs",
    "gtfs-static",
    "gtfs-realtime",
    "import",
    "mask-icon",
    "meta",
    "micropub",
    "openid.delegate",
    "openid.server",
    "openid2.local_id",
    "openid2.provider",
    "p3pv1",
    "pgpkey",
    "schema.dcterms",
    "service",
    "shortlink",
    "sitemap",
    "subresource",
    "sword",
    "timesheet",
    "token_endpoint",
    "wlwmanifest",
    "stylesheet/less",
    "yandex-tableau-widget",
];

/// `rel` of `<a>`.
fn a_rel(doc: &Doc, index: usize, value: Option<&Value>) -> Option<String> {
    if !doc.has(index, "href") {
        return Some("requires \"href\" attribute to be present".to_owned());
    }
    refused_rel(value, "a", LINK_REL_REFUSED, true)
}

/// `rel` of `<area>`.
fn area_rel(doc: &Doc, index: usize, value: Option<&Value>) -> Option<String> {
    if !doc.has(index, "href") {
        return Some("requires \"href\" attribute to be present".to_owned());
    }
    refused_rel(value, "area", LINK_REL_REFUSED, true)
}

/// `rel` of `<form>`.
fn form_rel(_: &Doc, _: usize, value: Option<&Value>) -> Option<String> {
    refused_rel(
        value,
        "form",
        &[
            "alternate",
            "canonical",
            "author",
            "bookmark",
            "dns-prefetch",
            "expect",
            "icon",
            "manifest",
            "modulepreload",
            "pingback",
            "preconnect",
            "prefetch",
            "preload",
            "privacy-policy",
            "stylesheet",
            "tag",
            "terms-of-service",
        ],
        false,
    )
}

/// `rel` of `<link>`.
fn link_rel(_: &Doc, _: usize, value: Option<&Value>) -> Option<String> {
    refused_rel(
        value,
        "link",
        &[
            "bookmark",
            "external",
            "nofollow",
            "noopener",
            "noreferrer",
            "opener",
            "tag",
            "disclosure",
            "entry-content",
            "lightbox",
            "lightvideo",
        ],
        false,
    )
}

/// `coords` of `<area>`.
fn area_coords(doc: &Doc, index: usize, _: Option<&Value>) -> Option<String> {
    value_is(doc, index, "shape", "default")
        .then(|| "cannot be used when \"shape\" attribute is \"default\"".to_owned())
}

/// `shape` of `<area>`.
fn area_shape(doc: &Doc, index: usize, value: Option<&Value>) -> Option<String> {
    let shape = match value {
        Some(Value::Static(shape)) => shape.as_str(),
        Some(Value::Dynamic) => return None,
        Some(Value::Omitted) | None => "rect",
    };
    match shape {
        "circ" | "circle" | "poly" | "polygon" | "rect" | "rectangle" => {
            allowed_if_present(doc, index, &["coords"])
        }
        _ => None,
    }
}

/// `imagesrcset` and `imagesizes` of `<link>`.
fn link_image(doc: &Doc, index: usize, _: Option<&Value>) -> Option<String> {
    match get(doc, index, "rel") {
        None | Some(Value::Omitted) => {
            return Some("\"rel\" attribute must be \"preload\"".to_owned());
        }
        Some(Value::Static(rel))
            if rel.is_empty()
                || !rel
                    .to_lowercase()
                    .split_whitespace()
                    .any(|it| it == "preload") =>
        {
            return Some("\"rel\" attribute must be \"preload\"".to_owned());
        }
        _ => {}
    }
    match get(doc, index, "as") {
        None | Some(Value::Omitted) => Some("\"as\" attribute must be \"image\"".to_owned()),
        Some(Value::Static(value)) if value != "image" => {
            Some("\"as\" attribute must be \"image\"".to_owned())
        }
        _ => None,
    }
}

/// `href` of `<link>`: required unless `imagesrcset` is present.
fn link_href_required(doc: &Doc, index: usize) -> Option<&'static str> {
    (!doc.has(index, "imagesrcset"))
        .then_some("{{ tagName }} is missing required \"href\" or \"imagesrcset\" attribute")
}

/// `imagesizes` of `<link>`: required when `imagesrcset` uses width descriptors.
fn link_imagesizes_required(doc: &Doc, index: usize) -> Option<&'static str> {
    doc.value(index, "imagesrcset")
        .is_some_and(has_width_descriptor)
        .then_some("{{ tagName }} requires \"{{ attr }}\" attribute when \"imagesrcset\" uses width descriptors")
}

/// `naturalJoin` of quoted names, by `or`.
#[must_use]
pub fn natural_join(values: &[String]) -> String {
    match values {
        [] => String::new(),
        [one] => one.clone(),
        [one, two] => format!("{one} or {two}"),
        [rest @ .., last] => format!("{} or {last}", rest.join(", ")),
    }
}

/// `allowedIfAttributeIsPresent`.
fn allowed_if_present(doc: &Doc, index: usize, names: &[&str]) -> Option<String> {
    if names.iter().any(|name| doc.has(index, name)) {
        return None;
    }
    let quoted: Vec<String> = names.iter().map(|name| format!("\"{name}\"")).collect();
    Some(format!(
        "requires {} attribute to be present",
        natural_join(&quoted)
    ))
}

impl Allowed {
    /// Why the attribute is not allowed on an element, `None` when it is.
    #[must_use]
    pub fn check(self, doc: &Doc, index: usize, value: Option<&Value>) -> Option<String> {
        match self {
            Self::IfPresent(names) => allowed_if_present(doc, index, names),
            Self::IfAbsent(names) => {
                let present: Vec<String> = names
                    .iter()
                    .filter(|name| doc.has(index, name))
                    .map(|name| format!("\"{name}\""))
                    .collect();
                (!present.is_empty()).then(|| {
                    format!(
                        "cannot be used at the same time as {}",
                        natural_join(&present)
                    )
                })
            }
            Self::IfValue(key, expected, default) => {
                let actual = match get(doc, index, key) {
                    Some(Value::Dynamic) => return None,
                    Some(Value::Static(text)) if !text.is_empty() => Some(text.as_str()),
                    // `attr ?? defaultValue`: an empty value stays empty, an omitted one is
                    // `null` and takes the default.
                    Some(Value::Static(_)) => None,
                    Some(Value::Omitted) | None => default,
                };
                if actual.is_some_and(|actual| expected.contains(&actual.to_lowercase().as_str())) {
                    return None;
                }
                let quoted: Vec<String> =
                    expected.iter().map(|each| format!("\"{each}\"")).collect();
                Some(format!(
                    "\"{key}\" attribute must be {}",
                    natural_join(&quoted)
                ))
            }
            Self::IfParent(tags) => {
                if tags.iter().any(|tag| doc.closest(index, tag).is_some()) {
                    return None;
                }
                let names: Vec<String> = tags.iter().map(|tag| format!("<{tag}>")).collect();
                Some(format!("requires {} as parent", natural_join(&names)))
            }
            Self::When(function) => function(doc, index, value),
        }
    }
}

/// `a`'s and `area`'s implicit role: a link with `href`, else generic.
fn link_role(doc: &Doc, index: usize) -> &'static str {
    if doc.has(index, "href") {
        "link"
    } else {
        "generic"
    }
}

/// Named only with `href`.
fn named_with_href(doc: &Doc, index: usize) -> bool {
    doc.has(index, "href")
}

/// Inside a `<map>`.
fn inside_map(doc: &Doc, index: usize) -> bool {
    doc.closest(index, "map").is_some()
}

/// With `controls`.
fn has_controls(doc: &Doc, index: usize) -> bool {
    doc.has(index, "controls")
}

/// With `usemap`.
fn has_usemap(doc: &Doc, index: usize) -> bool {
    doc.has(index, "usemap")
}

/// A `<button>` submits: no `type`, or `submit`.
fn button_submits(doc: &Doc, index: usize) -> bool {
    match get(doc, index, "type") {
        None | Some(Value::Omitted) => true,
        Some(Value::Static(text)) => text.is_empty() || text == "submit",
        Some(Value::Dynamic) => false,
    }
}

/// `<header>` and `<footer>` inside a landmark are generic, else the landmark.
fn header_role(doc: &Doc, index: usize) -> &'static str {
    if is_inside_landmark(doc, index) {
        "generic"
    } else {
        "banner"
    }
}

/// See `header_role`.
fn footer_role(doc: &Doc, index: usize) -> &'static str {
    if is_inside_landmark(doc, index) {
        "generic"
    } else {
        "contentinfo"
    }
}

/// Named unless inside a landmark.
fn named_outside_landmark(doc: &Doc, index: usize) -> bool {
    !is_inside_landmark(doc, index)
}

/// Whether an attribute is truthy as JavaScript reads `getAttribute`: present, and not an
/// omitted or empty value.
fn truthy(doc: &Doc, index: usize, key: &str) -> bool {
    match get(doc, index, key) {
        None | Some(Value::Omitted) => false,
        Some(Value::Static(text)) => !text.is_empty(),
        Some(Value::Dynamic) => true,
    }
}

/// `<img>`'s implicit role.
fn img_role(doc: &Doc, index: usize) -> &'static str {
    let decorative = value_is(doc, index, "alt", "")
        && !truthy(doc, index, "aria-label")
        && !truthy(doc, index, "aria-labelledby")
        && !truthy(doc, index, "title");
    if decorative { "none" } else { "img" }
}

/// `<img>`'s naming.
fn img_naming(doc: &Doc, index: usize) -> bool {
    ["alt", "aria-label", "aria-labelledby", "title"]
        .iter()
        .any(|key| truthy(doc, index, key))
}

/// Not `type="hidden"`.
fn not_hidden(doc: &Doc, index: usize) -> bool {
    !value_is(doc, index, "type", "hidden")
}

/// An `<input>` submits: `submit` or `image`.
fn input_submits(doc: &Doc, index: usize) -> bool {
    value_is(doc, index, "type", "submit") || value_is(doc, index, "type", "image")
}

/// `<input>`'s implicit role.
fn input_role(doc: &Doc, index: usize) -> Option<&'static str> {
    if doc.has(index, "list") {
        return Some("combobox");
    }
    match get(doc, index, "type") {
        Some(Value::Static(text)) => match text.as_str() {
            "button" | "image" | "reset" | "submit" => Some("button"),
            "checkbox" => Some("checkbox"),
            "color" | "date" | "datetime-local" | "file" | "hidden" | "month" | "password"
            | "time" | "week" => None,
            "number" => Some("spinbutton"),
            "radio" => Some("radio"),
            "range" => Some("slider"),
            "search" => Some("searchbox"),
            _ => Some("textbox"),
        },
        _ => Some("textbox"),
    }
}

/// `<li>`'s implicit role.
fn li_role(doc: &Doc, index: usize) -> &'static str {
    if doc.closest(index, "ul, ol, menu").is_some() {
        "listitem"
    } else {
        "generic"
    }
}

/// `<meta>` with `itemprop`.
fn has_itemprop(doc: &Doc, index: usize) -> bool {
    doc.has(index, "itemprop")
}

/// `<section>`'s implicit role.
fn section_role(doc: &Doc, index: usize) -> &'static str {
    let named = doc.has(index, "aria-label") || doc.has(index, "aria-labelledby");
    if named { "region" } else { "generic" }
}

/// `<select>`'s implicit role.
fn select_role(doc: &Doc, index: usize) -> &'static str {
    let sized = doc
        .value(index, "size")
        .and_then(js_number)
        .is_some_and(|number| number.trunc() > 1.0);
    if doc.has(index, "multiple") || sized {
        "listbox"
    } else {
        "combobox"
    }
}

/// JavaScript's `Number` of a string, `None` for `NaN`.
#[must_use]
pub fn js_number(text: &str) -> Option<f64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Some(0.0);
    }
    trimmed
        .parse::<f64>()
        .ok()
        .filter(|number| number.is_finite() || trimmed.contains("Infinity"))
}

/// Inside a `<details>`.
fn inside_details(doc: &Doc, index: usize) -> bool {
    doc.closest(index, "details").is_some()
}

/// The role of the `<table>` around a cell: its `role` as written, `table` without one or
/// with its value omitted; `None` when bound.
fn table_role(doc: &Doc, index: usize) -> Option<String> {
    let table = doc.closest(index, "table");
    match table.and_then(|table| get(doc, table, "role")) {
        None | Some(Value::Omitted) => Some("table".to_owned()),
        Some(Value::Static(role)) => Some(role.clone()),
        Some(Value::Dynamic) => None,
    }
}

/// `<td>`'s implicit role.
fn td_role(doc: &Doc, index: usize) -> Option<&'static str> {
    match table_role(doc, index)?.as_str() {
        "table" => Some("cell"),
        "grid" | "treegrid" => Some("gridcell"),
        _ => None,
    }
}

/// `<th>`'s implicit role.
fn th_role(doc: &Doc, index: usize) -> Option<&'static str> {
    let role = table_role(doc, index)?;
    if !matches!(role.as_str(), "table" | "grid" | "treegrid") {
        return None;
    }
    match doc.value(index, "scope") {
        Some("col") => Some("columnheader"),
        Some("row") => Some("rowheader"),
        _ => Some(if role == "table" { "cell" } else { "gridcell" }),
    }
}

/// `<link>` is flow and phrasing content when allowed in the body.
fn link_flow(doc: &Doc, index: usize) -> bool {
    link_body_ok(doc, index)
}

/// The `type` values an `<input>` takes.
const INPUT_TYPES: &[Enum] = &[
    W("button"),
    W("checkbox"),
    W("color"),
    W("date"),
    W("datetime-local"),
    W("email"),
    W("file"),
    W("hidden"),
    W("image"),
    W("month"),
    W("number"),
    W("password"),
    W("radio"),
    W("range"),
    W("reset"),
    W("search"),
    W("submit"),
    W("tel"),
    W("text"),
    W("time"),
    W("url"),
    W("week"),
];

/// An `<input>`'s attributes allowed on a submit or image button only.
const INPUT_SUBMIT: Allowed = Allowed::IfValue("type", &["submit", "image"], Some("submit"));
/// A `<button>`'s attributes allowed on a submit button only.
const BUTTON_SUBMIT: Allowed = Allowed::IfValue("type", &["submit"], Some("submit"));

/// A deprecated `align`.
const ALIGN: &[AttrMeta] = &[dep("align")];
/// The deprecated attributes of a table section.
const TABLE_SECTION: &[AttrMeta] = &[
    dep("align"),
    dep("background"),
    dep("char"),
    dep("charoff"),
    dep("valign"),
];

/// A heading.
const fn heading(name: &'static str) -> Meta {
    Meta {
        name,
        flow: Cat::Yes,
        heading: Cat::Yes,
        permitted_content: Some(PHRASING),
        attributes: ALIGN,
        implicit_role: Role::Fixed("heading"),
        ..M
    }
}

/// A phrasing element of phrasing content, with a role and a naming.
const fn phrasing(name: &'static str, role: Role, naming: Naming) -> Meta {
    Meta {
        name,
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        permitted_content: Some(PHRASING),
        implicit_role: role,
        naming,
        ..M
    }
}

/// A deprecated element.
const fn deprecated(
    name: &'static str,
    message: Option<&'static str>,
    documentation: Option<&'static str>,
    source: Option<&'static str>,
) -> Meta {
    Meta {
        name,
        deprecated: Some(Deprecated {
            message,
            documentation,
            source,
        }),
        ..M
    }
}

/// A table section: `<tbody>`, `<tfoot>`, `<thead>`.
const fn table_section(name: &'static str) -> Meta {
    Meta {
        name,
        permitted_content: Some(&[Entry::Item("@script"), Entry::Item("tr")]),
        attributes: TABLE_SECTION,
        implicit_role: Role::Fixed("rowgroup"),
        ..M
    }
}

/// The elements, by name.
static ELEMENTS: &[Meta] = &[
    Meta {
        name: "a",
        flow: Cat::Yes,
        focusable: Cat::When(named_with_href),
        phrasing: Cat::Yes,
        interactive: Cat::When(named_with_href),
        transparent: Transparent::Yes,
        attributes: &[
            dep("charset"),
            dep("coords"),
            dep("datafld"),
            dep("datasrc"),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..omit("download", NON_EMPTY)
            },
            en("href", &[Enum::Pattern("/^.*$/", "^.*$")]),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..attr("hreflang")
            },
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..attr("itemprop")
            },
            dep("methods"),
            dep("name"),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..attr("ping")
            },
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..en("referrerpolicy", REFERRER_POLICY)
            },
            AttrMeta {
                allowed: Some(Allowed::When(a_rel)),
                list: true,
                ..en("rel", NON_EMPTY)
            },
            dep("shape"),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..en("target", TARGETS)
            },
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..attr("type")
            },
            dep("urn"),
        ],
        permitted_descendants: Some(&[Entry::Exclude(&["@interactive", "a"])]),
        implicit_role: Role::Of(link_role),
        naming: Naming::When(named_with_href),
        ..M
    },
    Meta {
        naming: Naming::Prohibited,
        ..phrasing("abbr", Role::None, Naming::Prohibited)
    },
    deprecated(
        "acronym",
        Some("use <abbr> instead"),
        Some("`<abbr>` can be used as a replacement."),
        Some("html5"),
    ),
    Meta {
        name: "address",
        flow: Cat::Yes,
        implicit_role: Role::Fixed("group"),
        permitted_content: Some(FLOW),
        permitted_descendants: Some(&[Entry::Exclude(&[
            "address",
            "header",
            "footer",
            "@heading",
            "@sectioning",
        ])]),
        ..M
    },
    Meta {
        attributes: &[dep("datafld"), dep("datasrc")],
        ..deprecated("applet", None, None, Some("html5"))
    },
    Meta {
        name: "area",
        flow: Cat::When(inside_map),
        focusable: Cat::When(named_with_href),
        phrasing: Cat::When(inside_map),
        void: true,
        attributes: &[
            attr("alt"),
            AttrMeta {
                allowed: Some(Allowed::When(area_coords)),
                ..attr("coords")
            },
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..attr("download")
            },
            dep("nohref"),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..attr("itemprop")
            },
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..attr("ping")
            },
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..en("referrerpolicy", REFERRER_POLICY)
            },
            AttrMeta {
                allowed: Some(Allowed::When(area_rel)),
                ..attr("rel")
            },
            AttrMeta {
                allowed: Some(Allowed::When(area_shape)),
                ..en("shape", &[W("rect"), W("circle"), W("poly"), W("default")])
            },
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["href"])),
                ..en("target", TARGETS)
            },
        ],
        implicit_role: Role::Of(link_role),
        naming: Naming::When(named_with_href),
        required_ancestors: Some(&["map", "template"]),
        ..M
    },
    Meta {
        name: "article",
        flow: Cat::Yes,
        sectioning: Cat::Yes,
        permitted_content: Some(FLOW),
        permitted_descendants: Some(NO_MAIN),
        implicit_role: Role::Fixed("article"),
        ..M
    },
    Meta {
        name: "aside",
        flow: Cat::Yes,
        sectioning: Cat::Yes,
        permitted_content: Some(FLOW),
        permitted_descendants: Some(NO_MAIN),
        implicit_role: Role::Fixed("complementary"),
        ..M
    },
    Meta {
        name: "audio",
        flow: Cat::Yes,
        focusable: Cat::When(has_controls),
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        interactive: Cat::When(has_controls),
        transparent: Transparent::Only(&["@flow"]),
        attributes: &[
            boolean("autoplay"),
            boolean("controls"),
            omit("crossorigin", CROSS_ORIGIN),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["src"])),
                ..attr("itemprop")
            },
            boolean("loop"),
            boolean("muted"),
            omit("preload", &[W("none"), W("metadata"), W("auto")]),
            en("src", NON_EMPTY),
        ],
        permitted_content: Some(&[
            Entry::Item("@flow"),
            Entry::Item("track"),
            Entry::Item("source"),
        ]),
        permitted_descendants: Some(&[Entry::Exclude(&["audio", "video"])]),
        permitted_order: Some(&["source", "track", "@flow"]),
        ..M
    },
    phrasing("b", Role::Fixed("generic"), Naming::Prohibited),
    Meta {
        name: "base",
        metadata: Cat::Yes,
        void: true,
        attributes: &[attr("href"), attr("target")],
        permitted_parent: Some(&[Entry::Item("head")]),
        naming: Naming::Prohibited,
        ..M
    },
    deprecated(
        "basefont",
        Some("use CSS instead"),
        Some("Use CSS `font-size` property instead."),
        Some("html4"),
    ),
    phrasing("bdi", Role::Fixed("generic"), Naming::Prohibited),
    phrasing("bdo", Role::Fixed("generic"), Naming::Prohibited),
    deprecated(
        "bgsound",
        Some("use <audio> instead"),
        Some(
            "Use the `<audio>` element instead but consider accessibility concerns with autoplaying sounds.",
        ),
        Some("non-standard"),
    ),
    deprecated(
        "big",
        Some("use CSS instead"),
        Some("Use CSS `font-size` property instead."),
        Some("html5"),
    ),
    deprecated(
        "blink",
        None,
        Some(
            "`<blink>` has no direct replacement and blinking text is frowned upon by accessibility standards.",
        ),
        Some("non-standard"),
    ),
    Meta {
        name: "blockquote",
        flow: Cat::Yes,
        sectioning: Cat::Yes,
        attributes: &[attr("cite")],
        implicit_role: Role::Fixed("blockquote"),
        permitted_content: Some(FLOW),
        ..M
    },
    Meta {
        name: "body",
        permitted_content: Some(FLOW),
        permitted_parent: Some(&[Entry::Item("html")]),
        attributes: &[
            dep("alink"),
            dep("background"),
            dep("bgcolor"),
            dep("link"),
            dep("marginbottom"),
            dep("marginheight"),
            dep("marginleft"),
            dep("marginright"),
            dep("margintop"),
            dep("marginwidth"),
            dep("text"),
            dep("vlink"),
        ],
        implicit_role: Role::Fixed("generic"),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "br",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        void: true,
        attributes: &[dep("clear")],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "button",
        flow: Cat::Yes,
        focusable: Cat::Yes,
        phrasing: Cat::Yes,
        interactive: Cat::Yes,
        form_associated: Some(FormAssociated {
            disablable: true,
            listed: true,
        }),
        labelable: Cat::Yes,
        submit_button: Cat::When(button_submits),
        attributes: &[
            boolean("autofocus"),
            en(
                "command",
                &[
                    W("toggle-popover"),
                    W("show-popover"),
                    W("hide-popover"),
                    W("close"),
                    W("request-close"),
                    W("show-modal"),
                    Enum::Pattern("/^--/", "^--"),
                ],
            ),
            id_ref("commandfor"),
            dep("datafld"),
            dep("dataformatas"),
            dep("datasrc"),
            boolean("disabled"),
            id_ref("form"),
            AttrMeta {
                allowed: Some(BUTTON_SUBMIT),
                ..attr("formaction")
            },
            AttrMeta {
                allowed: Some(BUTTON_SUBMIT),
                ..attr("formenctype")
            },
            AttrMeta {
                allowed: Some(BUTTON_SUBMIT),
                ..en("formmethod", &[W("get"), W("post"), W("dialog")])
            },
            AttrMeta {
                allowed: Some(BUTTON_SUBMIT),
                ..boolean("formnovalidate")
            },
            AttrMeta {
                allowed: Some(BUTTON_SUBMIT),
                ..en("formtarget", TARGETS)
            },
            en("name", NON_EMPTY),
            id_ref("popovertarget"),
            en("popovertargetaction", &[W("toggle"), W("show"), W("hide")]),
            en("type", &[W("submit"), W("reset"), W("button")]),
            attr("value"),
        ],
        implicit_role: Role::Fixed("button"),
        permitted_content: Some(PHRASING),
        permitted_descendants: Some(&[Entry::Exclude(&["@interactive"])]),
        text_content: Some(TextContent::Accessible),
        ..M
    },
    Meta {
        name: "canvas",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        transparent: Transparent::Yes,
        ..M
    },
    Meta {
        name: "caption",
        permitted_content: Some(FLOW),
        permitted_descendants: Some(&[Entry::Exclude(&["table"])]),
        attributes: &[dep("align")],
        implicit_role: Role::Fixed("caption"),
        naming: Naming::Prohibited,
        ..M
    },
    deprecated(
        "center",
        Some("use CSS instead"),
        Some("Use the CSS `text-align` or `margin: auto` properties instead."),
        Some("html4"),
    ),
    phrasing("cite", Role::None, Naming::Prohibited),
    phrasing("code", Role::Fixed("code"), Naming::Prohibited),
    Meta {
        name: "col",
        attributes: &[
            dep("align"),
            dep("char"),
            dep("charoff"),
            en("span", POSITIVE_INTEGER),
            dep("valign"),
            dep("width"),
        ],
        void: true,
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "colgroup",
        attributes: &[en("span", POSITIVE_INTEGER)],
        permitted_content: Some(&[Entry::Item("col"), Entry::Item("template")]),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "data",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        attributes: &[attr("value")],
        permitted_content: Some(PHRASING),
        implicit_role: Role::Fixed("generic"),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "datalist",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        implicit_role: Role::Fixed("listbox"),
        naming: Naming::Prohibited,
        permitted_content: Some(&[Entry::Item("@phrasing"), Entry::Item("option")]),
        ..M
    },
    Meta {
        name: "dd",
        permitted_content: Some(FLOW),
        required_ancestors: Some(&[
            "dl > dd",
            "dl > div > dd",
            "template > dd",
            "template > div > dd",
        ]),
        ..M
    },
    Meta {
        name: "del",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        transparent: Transparent::Yes,
        attributes: &[attr("cite"), attr("datetime")],
        implicit_role: Role::Fixed("deletion"),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "details",
        flow: Cat::Yes,
        sectioning: Cat::Yes,
        interactive: Cat::Yes,
        attributes: &[attr("name"), boolean("open")],
        implicit_role: Role::Fixed("group"),
        permitted_content: Some(&[Entry::Item("summary"), Entry::Item("@flow")]),
        permitted_order: Some(&["summary", "@flow"]),
        required_content: Some(&["summary"]),
        ..M
    },
    Meta {
        name: "dfn",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        implicit_role: Role::Fixed("term"),
        permitted_content: Some(PHRASING),
        permitted_descendants: Some(&[Entry::Exclude(&["dfn"])]),
        ..M
    },
    Meta {
        name: "dialog",
        flow: Cat::Yes,
        permitted_content: Some(FLOW),
        attributes: &[
            omit("closedby", &[W("any"), W("closerequest"), W("none")]),
            boolean("open"),
        ],
        implicit_role: Role::Fixed("dialog"),
        ..M
    },
    deprecated(
        "dir",
        None,
        Some(
            "The non-standard `<dir>` element has no direct replacement but MDN recommends replacing with `<ul>` and CSS.",
        ),
        Some("html4"),
    ),
    Meta {
        name: "div",
        flow: Cat::Yes,
        permitted_content: Some(&[Entry::Item("@flow"), Entry::Item("dt"), Entry::Item("dd")]),
        attributes: &[
            dep("align"),
            dep("datafld"),
            dep("dataformatas"),
            dep("datasrc"),
        ],
        implicit_role: Role::Fixed("generic"),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "dl",
        flow: Cat::Yes,
        permitted_content: Some(&[
            Entry::Item("@script"),
            Entry::Item("dt"),
            Entry::Item("dd"),
            Entry::Item("div"),
        ]),
        attributes: &[dep("compact")],
        ..M
    },
    Meta {
        name: "dt",
        permitted_content: Some(FLOW),
        permitted_descendants: Some(&[Entry::Exclude(&[
            "header",
            "footer",
            "@sectioning",
            "@heading",
        ])]),
        required_ancestors: Some(&[
            "dl > dt",
            "dl > div > dt",
            "template > dt",
            "template > div > dt",
        ]),
        ..M
    },
    phrasing("em", Role::Fixed("emphasis"), Naming::Prohibited),
    Meta {
        name: "embed",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        interactive: Cat::Yes,
        void: true,
        attributes: &[
            en("height", POSITIVE_INTEGER),
            AttrMeta {
                required: Required::Yes,
                ..en("src", NON_EMPTY)
            },
            AttrMeta {
                required: Required::Yes,
                ..attr("title")
            },
            en("width", POSITIVE_INTEGER),
        ],
        ..M
    },
    Meta {
        name: "fieldset",
        flow: Cat::Yes,
        form_associated: Some(FormAssociated {
            disablable: true,
            listed: true,
        }),
        attributes: &[
            dep("datafld"),
            boolean("disabled"),
            id_ref("form"),
            en("name", NON_EMPTY),
        ],
        implicit_role: Role::Fixed("group"),
        permitted_content: Some(&[Entry::Item("@flow"), Entry::Item("legend?")]),
        permitted_order: Some(&["legend", "@flow"]),
        ..M
    },
    Meta {
        name: "figcaption",
        permitted_content: Some(FLOW),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "figure",
        flow: Cat::Yes,
        implicit_role: Role::Fixed("figure"),
        permitted_content: Some(&[Entry::Item("@flow"), Entry::Item("figcaption?")]),
        permitted_order: Some(&["figcaption", "@flow", "figcaption"]),
        ..M
    },
    deprecated(
        "font",
        Some("use CSS instead"),
        Some("Use CSS font properties instead."),
        Some("html4"),
    ),
    Meta {
        name: "footer",
        flow: Cat::Yes,
        implicit_role: Role::Of(footer_role),
        naming: Naming::When(named_outside_landmark),
        permitted_content: Some(FLOW),
        permitted_descendants: Some(&[Entry::Exclude(&["header", "footer", "main"])]),
        ..M
    },
    Meta {
        name: "form",
        flow: Cat::Yes,
        form: Cat::Yes,
        attributes: &[
            dep("accept"),
            attr("accept-charset"),
            en("action", &[Enum::Pattern(r"/^\s*\S+\s*$/", r"^\s*\S+\s*$")]),
            en("autocomplete", &[W("on"), W("off")]),
            en(
                "enctype",
                &[
                    W("application/x-www-form-urlencoded"),
                    W("multipart/form-data"),
                    W("text/plain"),
                ],
            ),
            en("method", &[W("get"), W("post"), W("dialog")]),
            attr("name"),
            boolean("novalidate"),
            AttrMeta {
                allowed: Some(Allowed::When(form_rel)),
                list: true,
                ..en("rel", NON_EMPTY)
            },
            en("target", TARGETS),
        ],
        implicit_role: Role::Fixed("form"),
        permitted_content: Some(FLOW),
        permitted_descendants: Some(&[Entry::Exclude(&["@form"])]),
        ..M
    },
    Meta {
        attributes: &[
            dep("datafld"),
            dep("datasrc"),
            AttrMeta {
                required: Required::Yes,
                ..attr("title")
            },
        ],
        ..deprecated(
            "frame",
            None,
            Some(
                "The `<frame>` element can be replaced with the `<iframe>` element but a better solution is to remove usage of frames entirely.",
            ),
            Some("html5"),
        )
    },
    deprecated(
        "frameset",
        None,
        Some(
            "The `<frameset>` element can be replaced with the `<iframe>` element but a better solution is to remove usage of frames entirely.",
        ),
        Some("html5"),
    ),
    heading("h1"),
    heading("h2"),
    heading("h3"),
    heading("h4"),
    heading("h5"),
    heading("h6"),
    Meta {
        name: "head",
        permitted_content: Some(&[
            Entry::Item("base?"),
            Entry::Item("title?"),
            Entry::Item("@meta"),
        ]),
        permitted_parent: Some(&[Entry::Item("html")]),
        required_content: Some(&["title"]),
        attributes: &[dep("profile")],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "header",
        flow: Cat::Yes,
        implicit_role: Role::Of(header_role),
        naming: Naming::When(named_outside_landmark),
        permitted_content: Some(FLOW),
        permitted_descendants: Some(&[Entry::Exclude(&["header", "footer", "main"])]),
        ..M
    },
    Meta {
        name: "hgroup",
        flow: Cat::Yes,
        heading: Cat::Yes,
        permitted_content: Some(&[Entry::Item("p"), Entry::Item("@heading?")]),
        permitted_descendants: Some(&[Entry::Exclude(&["hgroup"])]),
        required_content: Some(&["@heading"]),
        implicit_role: Role::Fixed("group"),
        ..M
    },
    Meta {
        name: "hr",
        flow: Cat::Yes,
        void: true,
        attributes: &[
            dep("align"),
            dep("color"),
            dep("noshade"),
            dep("size"),
            dep("width"),
        ],
        implicit_role: Role::Fixed("separator"),
        ..M
    },
    Meta {
        name: "html",
        permitted_content: Some(&[Entry::Item("head?"), Entry::Item("body?")]),
        permitted_order: Some(&["head", "body"]),
        required_content: Some(&["head", "body"]),
        attributes: &[
            AttrMeta {
                required: Required::Yes,
                ..attr("lang")
            },
            dep("version"),
        ],
        implicit_role: Role::Fixed("generic"),
        naming: Naming::Prohibited,
        ..M
    },
    phrasing("i", Role::Fixed("generic"), Naming::Prohibited),
    Meta {
        name: "iframe",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        interactive: Cat::Yes,
        attributes: &[
            dep("align"),
            dep("allowtransparency"),
            dep("datafld"),
            dep("datasrc"),
            dep("frameborder"),
            en("height", POSITIVE_INTEGER),
            dep("hspace"),
            dep("marginheight"),
            dep("marginwidth"),
            en("referrerpolicy", REFERRER_POLICY),
            dep("scrolling"),
            en("src", NON_EMPTY),
            AttrMeta {
                required: Required::Yes,
                ..attr("title")
            },
            dep("vspace"),
            en("width", POSITIVE_INTEGER),
        ],
        permitted_content: Some(NOTHING),
        ..M
    },
    Meta {
        name: "img",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        interactive: Cat::When(has_usemap),
        void: true,
        attributes: &[
            dep("align"),
            attr("alt"),
            dep("border"),
            omit("crossorigin", CROSS_ORIGIN),
            dep("datafld"),
            dep("datasrc"),
            en("decoding", &[W("sync"), W("async"), W("auto")]),
            omit("fetchpriority", &[W("high"), W("low"), W("auto")]),
            en("height", POSITIVE_INTEGER),
            dep("hspace"),
            boolean("ismap"),
            omit("loading", &[W("lazy"), W("eager")]),
            dep("lowsrc"),
            dep("name"),
            en("referrerpolicy", REFERRER_POLICY),
            attr("sizes"),
            AttrMeta {
                required: Required::Yes,
                ..en("src", NON_EMPTY)
            },
            // `/[^]+/`: any character, line breaks included, once or more.
            en("srcset", &[Enum::Pattern("/^[^]+$/", r"^(?s:.)+$")]),
            attr("usemap"),
            dep("vspace"),
            en("width", POSITIVE_INTEGER),
        ],
        implicit_role: Role::Of(img_role),
        naming: Naming::When(img_naming),
        ..M
    },
    Meta {
        name: "input",
        flow: Cat::Yes,
        focusable: Cat::When(not_hidden),
        phrasing: Cat::Yes,
        interactive: Cat::When(not_hidden),
        void: true,
        form_associated: Some(FormAssociated {
            disablable: true,
            listed: true,
        }),
        labelable: Cat::When(not_hidden),
        submit_button: Cat::When(input_submits),
        attributes: &[
            attr("accept"),
            dep("align"),
            boolean("alpha"),
            attr("alt"),
            en(
                "autocapitalize",
                &[
                    W("off"),
                    W("none"),
                    W("on"),
                    W("sentences"),
                    W("words"),
                    W("characters"),
                ],
            ),
            attr("autocomplete"),
            boolean("autofocus"),
            omit("capture", &[W("environment"), W("user")]),
            boolean("checked"),
            en("colorspace", &[W("limited-srgb"), W("display-p3")]),
            dep("datafld"),
            dep("dataformatas"),
            dep("datasrc"),
            attr("dirname"),
            boolean("disabled"),
            id_ref("form"),
            AttrMeta {
                allowed: Some(INPUT_SUBMIT),
                ..attr("formaction")
            },
            AttrMeta {
                allowed: Some(INPUT_SUBMIT),
                ..attr("formenctype")
            },
            AttrMeta {
                allowed: Some(INPUT_SUBMIT),
                ..en("formmethod", &[W("get"), W("post"), W("dialog")])
            },
            AttrMeta {
                allowed: Some(INPUT_SUBMIT),
                ..boolean("formnovalidate")
            },
            AttrMeta {
                allowed: Some(INPUT_SUBMIT),
                ..en("formtarget", TARGETS)
            },
            en("height", POSITIVE_INTEGER),
            dep("hspace"),
            en(
                "inputmode",
                &[
                    W("none"),
                    W("text"),
                    W("decimal"),
                    W("numeric"),
                    W("tel"),
                    W("search"),
                    W("email"),
                    W("url"),
                ],
            ),
            dep("ismap"),
            id_ref("list"),
            en("max", NON_EMPTY),
            en("maxlength", POSITIVE_INTEGER),
            en("min", NON_EMPTY),
            en("minlength", POSITIVE_INTEGER),
            boolean("multiple"),
            en("name", NON_EMPTY),
            attr("pattern"),
            attr("placeholder"),
            id_ref("popovertarget"),
            en("popovertargetaction", &[W("toggle"), W("show"), W("hide")]),
            boolean("readonly"),
            boolean("required"),
            en("size", POSITIVE_INTEGER),
            en("src", NON_EMPTY),
            attr("step"),
            en("type", INPUT_TYPES),
            dep("usemap"),
            attr("value"),
            dep("vspace"),
            en("width", POSITIVE_INTEGER),
        ],
        implicit_role: Role::When(input_role),
        naming: Naming::When(not_hidden),
        ..M
    },
    Meta {
        name: "ins",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        transparent: Transparent::Yes,
        attributes: &[attr("cite"), attr("datetime")],
        implicit_role: Role::Fixed("insertion"),
        naming: Naming::Prohibited,
        ..M
    },
    deprecated("isindex", None, None, Some("html4")),
    phrasing("kbd", Role::None, Naming::Prohibited),
    Meta {
        name: "keygen",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        interactive: Cat::Yes,
        void: true,
        labelable: Cat::Yes,
        deprecated: Some(Deprecated {
            message: None,
            documentation: None,
            source: None,
        }),
        ..M
    },
    Meta {
        name: "label",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        interactive: Cat::Yes,
        permitted_content: Some(PHRASING),
        permitted_descendants: Some(&[Entry::Exclude(&["label"])]),
        attributes: &[
            dep("datafld"),
            dep("dataformatas"),
            dep("datasrc"),
            id_ref("for"),
        ],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "legend",
        permitted_content: Some(&[Entry::Item("@phrasing"), Entry::Item("@heading")]),
        attributes: &[
            dep("align"),
            dep("datafld"),
            dep("dataformatas"),
            dep("datasrc"),
        ],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "li",
        permitted_content: Some(FLOW),
        permitted_parent: Some(&[
            Entry::Item("ul"),
            Entry::Item("ol"),
            Entry::Item("menu"),
            Entry::Item("template"),
        ]),
        attributes: &[
            dep("type"),
            en("value", &[Enum::Pattern(r"/^-?\d+$/", r"^-?\d+$")]),
        ],
        implicit_role: Role::Of(li_role),
        ..M
    },
    Meta {
        name: "link",
        metadata: Cat::Yes,
        flow: Cat::When(link_flow),
        phrasing: Cat::When(link_flow),
        void: true,
        attributes: &[
            AttrMeta {
                allowed: Some(Allowed::IfValue(
                    "rel",
                    &["prefetch", "preload", "modulepreload"],
                    None,
                )),
                ..en(
                    "as",
                    &[
                        W("audio"),
                        W("audioworklet"),
                        W("document"),
                        W("embed"),
                        W("fetch"),
                        W("font"),
                        W("frame"),
                        W("iframe"),
                        W("image"),
                        W("manifest"),
                        W("object"),
                        W("paintworklet"),
                        W("report"),
                        W("script"),
                        W("serviceworker"),
                        W("sharedworker"),
                        W("style"),
                        W("track"),
                        W("video"),
                        W("webidentity"),
                        W("worker"),
                        W("xslt"),
                    ],
                )
            },
            AttrMeta {
                allowed: Some(Allowed::IfValue("rel", &["expect", "stylesheet"], None)),
                list: true,
                ..en("blocking", &[W("render")])
            },
            dep("charset"),
            attr("color"),
            omit("crossorigin", CROSS_ORIGIN),
            AttrMeta {
                allowed: Some(Allowed::IfValue("rel", &["stylesheet"], None)),
                ..boolean("disabled")
            },
            en("fetchpriority", &[W("high"), W("low"), W("auto")]),
            AttrMeta {
                required: Required::When(link_href_required),
                ..en("href", NON_EMPTY)
            },
            attr("hreflang"),
            AttrMeta {
                allowed: Some(Allowed::When(link_image)),
                ..attr("imagesrcset")
            },
            AttrMeta {
                allowed: Some(Allowed::When(link_image)),
                required: Required::When(link_imagesizes_required),
                ..attr("imagesizes")
            },
            AttrMeta {
                allowed: Some(Allowed::IfValue(
                    "rel",
                    &["stylesheet", "preload", "modulepreload"],
                    None,
                )),
                ..en("integrity", NON_EMPTY)
            },
            attr("media"),
            dep("methods"),
            en("referrerpolicy", REFERRER_POLICY),
            AttrMeta {
                allowed: Some(Allowed::When(link_rel)),
                list: true,
                ..en("rel", NON_EMPTY)
            },
            attr("sizes"),
            dep("target"),
            attr("type"),
            dep("urn"),
        ],
        naming: Naming::Prohibited,
        ..M
    },
    deprecated("listing", None, None, Some("html32")),
    Meta {
        name: "main",
        flow: Cat::Yes,
        implicit_role: Role::Fixed("main"),
        ..M
    },
    Meta {
        name: "map",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        transparent: Transparent::Yes,
        attributes: &[AttrMeta {
            required: Required::Yes,
            ..en("name", &[Enum::Pattern(r"/^\S+$/", r"^\S+$")])
        }],
        naming: Naming::Prohibited,
        ..M
    },
    phrasing("mark", Role::None, Naming::Prohibited),
    Meta {
        attributes: &[dep("datafld"), dep("dataformatas"), dep("datasrc")],
        ..deprecated(
            "marquee",
            None,
            Some(
                "Marked as obsolete by both W3C and WHATWG standards but still implemented in most browsers. Animated text should be avoided for accessibility reasons as well.",
            ),
            Some("html5"),
        )
    },
    Meta {
        name: "math",
        flow: Cat::Yes,
        foreign: true,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        attributes: &[
            dep("align"),
            en("dir", &[W("ltr"), W("rtl")]),
            en("display", &[W("block"), W("inline")]),
            dep("hspace"),
            dep("name"),
            en(
                "overflow",
                &[
                    W("linebreak"),
                    W("scroll"),
                    W("elide"),
                    W("truncate"),
                    W("scale"),
                ],
            ),
            dep("vspace"),
        ],
        implicit_role: Role::Fixed("math"),
        ..M
    },
    Meta {
        name: "menu",
        flow: Cat::Yes,
        implicit_role: Role::Fixed("list"),
        permitted_content: Some(&[Entry::Item("@script"), Entry::Item("li")]),
        ..M
    },
    Meta {
        name: "meta",
        flow: Cat::When(has_itemprop),
        phrasing: Cat::When(has_itemprop),
        metadata: Cat::Yes,
        void: true,
        attributes: &[
            en("charset", &[W("utf-8")]),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&[
                    "name",
                    "http-equiv",
                    "itemprop",
                    "property",
                ])),
                ..attr("content")
            },
            AttrMeta {
                allowed: Some(Allowed::IfAbsent(&["http-equiv", "name"])),
                ..attr("itemprop")
            },
            AttrMeta {
                allowed: Some(Allowed::IfAbsent(&["http-equiv", "itemprop"])),
                ..attr("name")
            },
            AttrMeta {
                allowed: Some(Allowed::IfAbsent(&["name", "itemprop"])),
                ..attr("http-equiv")
            },
            dep("scheme"),
        ],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "meter",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        labelable: Cat::Yes,
        attributes: &[
            en("high", FLOATING_POINT),
            en("low", FLOATING_POINT),
            en("max", FLOATING_POINT),
            en("min", FLOATING_POINT),
            en("optimum", FLOATING_POINT),
            en("value", FLOATING_POINT),
        ],
        implicit_role: Role::Fixed("meter"),
        permitted_content: Some(PHRASING),
        permitted_descendants: Some(&[Entry::Exclude(&["meter"])]),
        ..M
    },
    deprecated(
        "multicol",
        Some("use CSS instead"),
        Some("Use CSS columns instead."),
        Some("html5"),
    ),
    Meta {
        name: "nav",
        flow: Cat::Yes,
        sectioning: Cat::Yes,
        implicit_role: Role::Fixed("navigation"),
        permitted_content: Some(FLOW),
        permitted_descendants: Some(NO_MAIN),
        ..M
    },
    deprecated("nextid", None, None, Some("html32")),
    deprecated(
        "nobr",
        Some("use CSS instead"),
        Some("Use CSS `white-space` property instead."),
        Some("non-standard"),
    ),
    deprecated("noembed", None, None, Some("non-standard")),
    deprecated("noframes", None, None, Some("html5")),
    Meta {
        name: "noscript",
        metadata: Cat::Yes,
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        transparent: Transparent::Yes,
        permitted_descendants: Some(&[Entry::Exclude(&["noscript"])]),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "object",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        interactive: Cat::When(has_usemap),
        transparent: Transparent::Yes,
        form_associated: Some(FormAssociated {
            disablable: false,
            listed: true,
        }),
        attributes: &[
            dep("align"),
            dep("archive"),
            AttrMeta {
                list: true,
                ..en("blocking", &[W("render")])
            },
            dep("border"),
            dep("classid"),
            dep("code"),
            dep("codebase"),
            dep("codetype"),
            AttrMeta {
                required: Required::Yes,
                ..en("data", NON_EMPTY)
            },
            dep("datafld"),
            dep("dataformatas"),
            dep("datasrc"),
            dep("declare"),
            en("height", POSITIVE_INTEGER),
            dep("hspace"),
            en("name", &[VALID_BROWSING_CONTEXT_NAME]),
            dep("standby"),
            dep("vspace"),
            en("width", POSITIVE_INTEGER),
        ],
        permitted_content: Some(&[Entry::Item("param"), Entry::Item("@flow")]),
        permitted_order: Some(&["param", "@flow"]),
        ..M
    },
    Meta {
        name: "ol",
        flow: Cat::Yes,
        attributes: &[
            dep("compact"),
            boolean("reversed"),
            en("start", POSITIVE_INTEGER),
            en("type", &[W("a"), W("A"), W("i"), W("I"), W("1")]),
        ],
        implicit_role: Role::Fixed("list"),
        permitted_content: Some(&[Entry::Item("@script"), Entry::Item("li")]),
        ..M
    },
    Meta {
        name: "optgroup",
        attributes: &[boolean("disabled"), attr("label")],
        implicit_role: Role::Fixed("group"),
        permitted_content: Some(&[Entry::Item("@script"), Entry::Item("option")]),
        ..M
    },
    Meta {
        name: "option",
        attributes: &[
            dep("dataformatas"),
            dep("datasrc"),
            boolean("disabled"),
            attr("label"),
            dep("name"),
            boolean("selected"),
            attr("value"),
        ],
        implicit_role: Role::Fixed("option"),
        permitted_content: Some(&[Entry::Item("@phrasing"), Entry::Item("div")]),
        permitted_descendants: Some(&[Entry::Exclude(&["@interactive", "datalist", "object"])]),
        ..M
    },
    Meta {
        name: "output",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        form_associated: Some(FormAssociated {
            disablable: false,
            listed: true,
        }),
        labelable: Cat::Yes,
        attributes: &[id_refs("for"), id_ref("form"), en("name", NON_EMPTY)],
        implicit_role: Role::Fixed("status"),
        permitted_content: Some(PHRASING),
        ..M
    },
    Meta {
        name: "p",
        flow: Cat::Yes,
        permitted_content: Some(PHRASING),
        attributes: &[dep("align")],
        implicit_role: Role::Fixed("paragraph"),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "param",
        void: true,
        attributes: &[dep("datafld"), dep("type"), dep("valuetype")],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "picture",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        permitted_content: Some(&[
            Entry::Item("@script"),
            Entry::Item("source"),
            Entry::Item("img"),
        ]),
        permitted_order: Some(&["source", "img"]),
        naming: Naming::Prohibited,
        ..M
    },
    deprecated(
        "plaintext",
        Some("use <pre> or CSS instead"),
        Some("Use the `<pre>` element or use CSS to set a monospace font."),
        Some("html2"),
    ),
    Meta {
        name: "pre",
        flow: Cat::Yes,
        permitted_content: Some(PHRASING),
        attributes: &[dep("width")],
        implicit_role: Role::Fixed("generic"),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "progress",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        labelable: Cat::Yes,
        attributes: &[en("max", FLOATING_POINT), en("value", FLOATING_POINT)],
        implicit_role: Role::Fixed("progressbar"),
        permitted_content: Some(PHRASING),
        permitted_descendants: Some(&[Entry::Exclude(&["progress"])]),
        ..M
    },
    Meta {
        attributes: &[attr("cite")],
        ..phrasing("q", Role::Fixed("generic"), Naming::Prohibited)
    },
    Meta {
        name: "rb",
        permitted_content: Some(PHRASING),
        ..M
    },
    Meta {
        name: "rp",
        permitted_content: Some(PHRASING),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "rt",
        permitted_content: Some(PHRASING),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "rtc",
        permitted_content: Some(&[Entry::Item("@phrasing"), Entry::Item("rt")]),
        ..M
    },
    Meta {
        name: "ruby",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        permitted_content: Some(&[
            Entry::Item("@phrasing"),
            Entry::Item("rb"),
            Entry::Item("rp"),
            Entry::Item("rt"),
            Entry::Item("rtc"),
        ]),
        ..M
    },
    phrasing("s", Role::Fixed("deletion"), Naming::Prohibited),
    phrasing("samp", Role::Fixed("generic"), Naming::Prohibited),
    Meta {
        name: "script",
        metadata: Cat::Yes,
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        script_supporting: Cat::Yes,
        attributes: &[
            boolean("async"),
            AttrMeta {
                list: true,
                ..en("blocking", &[W("render")])
            },
            omit("crossorigin", CROSS_ORIGIN),
            boolean("defer"),
            dep("event"),
            en("fetchpriority", &[W("high"), W("low"), W("auto")]),
            dep("for"),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["src"])),
                ..en("integrity", NON_EMPTY)
            },
            dep("language"),
            boolean("nomodule"),
            en("referrerpolicy", REFERRER_POLICY),
            en("src", NON_EMPTY),
            attr("type"),
        ],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "search",
        flow: Cat::Yes,
        implicit_role: Role::Fixed("search"),
        ..M
    },
    Meta {
        name: "section",
        flow: Cat::Yes,
        sectioning: Cat::Yes,
        implicit_role: Role::Of(section_role),
        permitted_content: Some(FLOW),
        ..M
    },
    Meta {
        name: "select",
        flow: Cat::Yes,
        focusable: Cat::Yes,
        phrasing: Cat::Yes,
        interactive: Cat::Yes,
        form_associated: Some(FormAssociated {
            disablable: true,
            listed: true,
        }),
        labelable: Cat::Yes,
        attributes: &[
            attr("autocomplete"),
            boolean("autofocus"),
            boolean("disabled"),
            id_ref("form"),
            boolean("multiple"),
            en("name", NON_EMPTY),
            boolean("required"),
            en("size", POSITIVE_INTEGER),
        ],
        implicit_role: Role::Of(select_role),
        permitted_content: Some(&[
            Entry::Item("@script"),
            Entry::Item("button?"),
            Entry::Item("datasrc"),
            Entry::Item("datafld"),
            Entry::Item("dataformatas"),
            Entry::Item("option"),
            Entry::Item("optgroup"),
            Entry::Item("hr"),
        ]),
        permitted_order: Some(&["button", "option, optgroup, hr"]),
        ..M
    },
    Meta {
        name: "selectedcontent",
        phrasing: Cat::Yes,
        permitted_content: Some(NOTHING),
        text_content: Some(TextContent::None),
        required_ancestors: Some(&["select > button"]),
        implicit_role: Role::Fixed("generic"),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "slot",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        transparent: Transparent::Yes,
        naming: Naming::Prohibited,
        attributes: &[attr("name")],
        ..M
    },
    phrasing("small", Role::Fixed("generic"), Naming::Prohibited),
    Meta {
        name: "source",
        void: true,
        attributes: &[
            attr("type"),
            attr("media"),
            AttrMeta {
                allowed: Some(Allowed::IfParent(&["audio", "video"])),
                ..attr("src")
            },
            AttrMeta {
                allowed: Some(Allowed::IfParent(&["picture"])),
                ..attr("srcset")
            },
            AttrMeta {
                allowed: Some(Allowed::IfParent(&["picture"])),
                ..attr("sizes")
            },
            AttrMeta {
                allowed: Some(Allowed::IfParent(&["picture"])),
                ..en("width", POSITIVE_INTEGER)
            },
            AttrMeta {
                allowed: Some(Allowed::IfParent(&["picture"])),
                ..en("height", POSITIVE_INTEGER)
            },
        ],
        naming: Naming::Prohibited,
        ..M
    },
    deprecated(
        "spacer",
        Some("use CSS instead"),
        Some("Use CSS margin or padding instead."),
        Some("non-standard"),
    ),
    Meta {
        attributes: &[dep("datafld"), dep("dataformatas"), dep("datasrc")],
        ..phrasing("span", Role::Fixed("generic"), Naming::Prohibited)
    },
    deprecated(
        "strike",
        Some("use <del> or <s> instead"),
        Some("Use the `<del>` or `<s>` element instead."),
        Some("html5"),
    ),
    phrasing("strong", Role::Fixed("strong"), Naming::Prohibited),
    Meta {
        name: "style",
        metadata: Cat::Yes,
        naming: Naming::Prohibited,
        ..M
    },
    phrasing("sub", Role::Fixed("subscript"), Naming::Prohibited),
    Meta {
        name: "summary",
        permitted_content: Some(&[Entry::Item("@phrasing"), Entry::Item("@heading")]),
        focusable: Cat::When(inside_details),
        implicit_role: Role::Fixed("button"),
        ..M
    },
    phrasing("sup", Role::Fixed("superscript"), Naming::Prohibited),
    Meta {
        name: "svg",
        flow: Cat::Yes,
        foreign: true,
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        implicit_role: Role::Fixed("graphics-document"),
        attributes: &[en("focusable", &[W("true"), W("false")])],
        ..M
    },
    Meta {
        name: "svg:desc",
        ..M
    },
    Meta {
        name: "svg:title",
        ..M
    },
    Meta {
        name: "table",
        flow: Cat::Yes,
        permitted_content: Some(&[
            Entry::Item("@script"),
            Entry::Item("caption?"),
            Entry::Item("colgroup"),
            Entry::Item("tbody"),
            Entry::Item("tfoot?"),
            Entry::Item("thead?"),
            Entry::Item("tr"),
        ]),
        permitted_order: Some(&["caption", "colgroup", "thead", "tbody", "tr", "tfoot"]),
        attributes: &[
            dep("align"),
            dep("background"),
            dep("bgcolor"),
            dep("bordercolor"),
            dep("cellpadding"),
            dep("cellspacing"),
            dep("dataformatas"),
            dep("datapagesize"),
            dep("datasrc"),
            dep("frame"),
            dep("rules"),
            dep("summary"),
            dep("width"),
        ],
        implicit_role: Role::Fixed("table"),
        ..M
    },
    table_section("tbody"),
    Meta {
        name: "td",
        flow: Cat::Yes,
        attributes: &[
            dep("align"),
            dep("axis"),
            dep("background"),
            dep("bgcolor"),
            dep("char"),
            dep("charoff"),
            en("colspan", POSITIVE_INTEGER),
            id_refs("headers"),
            dep("height"),
            dep("nowrap"),
            en("rowspan", POSITIVE_INTEGER),
            dep("scope"),
            dep("valign"),
            dep("width"),
        ],
        implicit_role: Role::When(td_role),
        permitted_content: Some(FLOW),
        ..M
    },
    Meta {
        name: "template",
        metadata: Cat::Yes,
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        script_supporting: Cat::Yes,
        template_root: Cat::Yes,
        attributes: &[
            boolean("shadowrootclonable"),
            boolean("shadowrootcustomelementregistry"),
            boolean("shadowrootdelegatesfocus"),
            en("shadowrootmode", &[W("open"), W("closed")]),
            boolean("shadowrootserializable"),
            en("shadowrootslotassignment", &[W("named"), W("manual")]),
        ],
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "textarea",
        flow: Cat::Yes,
        focusable: Cat::Yes,
        phrasing: Cat::Yes,
        interactive: Cat::Yes,
        form_associated: Some(FormAssociated {
            disablable: true,
            listed: true,
        }),
        labelable: Cat::Yes,
        attributes: &[
            attr("autocomplete"),
            boolean("autofocus"),
            en("cols", POSITIVE_INTEGER),
            dep("datafld"),
            dep("datasrc"),
            en("dirname", NON_EMPTY),
            boolean("disabled"),
            id_ref("form"),
            en("maxlength", POSITIVE_INTEGER),
            en("minlength", POSITIVE_INTEGER),
            en("name", NON_EMPTY),
            attr("placeholder"),
            boolean("readonly"),
            boolean("required"),
            en("rows", POSITIVE_INTEGER),
            en("wrap", &[W("hard"), W("soft")]),
        ],
        implicit_role: Role::Fixed("textbox"),
        permitted_content: Some(NOTHING),
        ..M
    },
    table_section("tfoot"),
    Meta {
        name: "th",
        flow: Cat::Yes,
        attributes: &[
            attr("abbr"),
            dep("align"),
            dep("axis"),
            dep("background"),
            dep("bgcolor"),
            dep("char"),
            id_refs("headers"),
            dep("charoff"),
            en("colspan", POSITIVE_INTEGER),
            dep("height"),
            dep("nowrap"),
            en("rowspan", POSITIVE_INTEGER),
            en("scope", &[W("row"), W("col"), W("rowgroup"), W("colgroup")]),
            dep("valign"),
            dep("width"),
        ],
        implicit_role: Role::When(th_role),
        permitted_content: Some(FLOW),
        permitted_descendants: Some(&[Entry::Exclude(&[
            "header",
            "footer",
            "@sectioning",
            "@heading",
        ])]),
        ..M
    },
    table_section("thead"),
    Meta {
        name: "time",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        attributes: &[attr("datetime")],
        implicit_role: Role::Fixed("time"),
        naming: Naming::Prohibited,
        permitted_content: Some(PHRASING),
        ..M
    },
    Meta {
        name: "title",
        metadata: Cat::Yes,
        permitted_content: Some(NOTHING),
        permitted_parent: Some(&[Entry::Item("head")]),
        naming: Naming::Prohibited,
        ..M
    },
    Meta {
        name: "tr",
        permitted_content: Some(&[Entry::Item("@script"), Entry::Item("td"), Entry::Item("th")]),
        attributes: &[
            dep("align"),
            dep("background"),
            dep("bgcolor"),
            dep("char"),
            dep("charoff"),
            dep("valign"),
        ],
        implicit_role: Role::Fixed("row"),
        ..M
    },
    Meta {
        name: "track",
        void: true,
        attributes: &[
            boolean("default"),
            omit(
                "kind",
                &[
                    W("subtitles"),
                    W("captions"),
                    W("descriptions"),
                    W("chapters"),
                    W("metadata"),
                ],
            ),
            attr("label"),
            AttrMeta {
                required: Required::Yes,
                ..en("src", NON_EMPTY)
            },
            attr("srclang"),
        ],
        naming: Naming::Prohibited,
        ..M
    },
    deprecated(
        "tt",
        None,
        Some("Use a more semantically correct element such as `<code>`, `<var>` or `<pre>`."),
        Some("html4"),
    ),
    phrasing("u", Role::Fixed("generic"), Naming::Prohibited),
    Meta {
        name: "ul",
        flow: Cat::Yes,
        permitted_content: Some(&[Entry::Item("@script"), Entry::Item("li")]),
        attributes: &[dep("compact"), dep("type")],
        implicit_role: Role::Fixed("list"),
        ..M
    },
    phrasing("var", Role::None, Naming::Prohibited),
    Meta {
        name: "video",
        flow: Cat::Yes,
        focusable: Cat::When(has_controls),
        phrasing: Cat::Yes,
        embedded: Cat::Yes,
        interactive: Cat::When(has_controls),
        transparent: Transparent::Only(&["@flow"]),
        attributes: &[
            boolean("autoplay"),
            boolean("controls"),
            omit("crossorigin", CROSS_ORIGIN),
            en("height", POSITIVE_INTEGER),
            AttrMeta {
                allowed: Some(Allowed::IfPresent(&["src"])),
                ..attr("itemprop")
            },
            boolean("loop"),
            boolean("muted"),
            boolean("playsinline"),
            en("poster", NON_EMPTY),
            omit("preload", &[W("none"), W("metadata"), W("auto")]),
            en("src", NON_EMPTY),
            en("width", POSITIVE_INTEGER),
        ],
        permitted_content: Some(&[
            Entry::Item("@flow"),
            Entry::Item("track"),
            Entry::Item("source"),
        ]),
        permitted_descendants: Some(&[Entry::Exclude(&["audio", "video"])]),
        permitted_order: Some(&["source", "track", "@flow"]),
        ..M
    },
    Meta {
        name: "wbr",
        flow: Cat::Yes,
        phrasing: Cat::Yes,
        void: true,
        naming: Naming::Prohibited,
        ..M
    },
    deprecated(
        "xmp",
        None,
        Some("Use `<pre>` or `<code>` and escape content using HTML entities instead."),
        Some("html32"),
    ),
];

/// An element's metadata by its name in lowercase, `None` for an element HTML does not know.
#[must_use]
pub fn meta_for(tag: &str) -> Option<&'static Meta> {
    ELEMENTS.iter().find(|meta| meta.name == tag)
}

impl Meta {
    /// Its implicit ARIA role for an element.
    #[must_use]
    pub fn role(&self, doc: &Doc, index: usize) -> Option<&'static str> {
        match self.implicit_role {
            Role::None => None,
            Role::Fixed(role) => Some(role),
            Role::Of(function) => Some(function(doc, index)),
            Role::When(function) => function(doc, index),
        }
    }

    /// Whether an element may be named.
    #[must_use]
    pub fn naming_allowed(&self, doc: &Doc, index: usize) -> bool {
        match self.naming {
            Naming::Allowed => true,
            Naming::Prohibited => false,
            Naming::When(function) => function(doc, index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ELEMENTS, Required, attribute, has_width_descriptor, js_number, meta_for, natural_join,
    };
    use crate::template::html::view::Doc;
    use crate::template::tree::parse;

    /// Runs a check on the first element of a template read as html-validate's DOM.
    fn first<T>(text: &str, check: &dyn Fn(&Doc, usize) -> T) -> T {
        let parsed = parse(text);
        let doc = Doc::new(text, &parsed.nodes);
        check(&doc, 0)
    }

    /// The implicit role of a template's first element.
    fn role(text: &str) -> Option<&'static str> {
        first(text, &|doc, index| {
            doc.el(index).meta.and_then(|meta| meta.role(doc, index))
        })
    }

    /// Why an attribute of a template's first element is not allowed there.
    fn misuse(text: &str, key: &str) -> Option<String> {
        first(text, &|doc, index| {
            let meta = doc.el(index).meta?;
            let value = doc.attr(index, key).map(|attr| attr.value.clone());
            attribute(meta, key)?
                .allowed?
                .check(doc, index, value.as_ref())
        })
    }

    #[test]
    fn gives_the_roles_html5_ts_gives() {
        // Our own cases, each after its `implicitRole` in `src/elements/html5.ts`.
        let cases = [
            ("<a href=\"x\"></a>", Some("link")),
            ("<a></a>", Some("generic")),
            ("<header></header>", Some("banner")),
            ("<main><header></header></main>", Some("main")),
            ("<footer></footer>", Some("contentinfo")),
            ("<img alt=\"\">", Some("none")),
            ("<img alt=\"\" title=\"t\">", Some("img")),
            ("<input>", Some("textbox")),
            ("<input list=\"l\">", Some("combobox")),
            ("<input type=\"checkbox\">", Some("checkbox")),
            ("<input type=\"date\">", None),
            ("<input type=\"number\">", Some("spinbutton")),
            ("<input type=\"radio\">", Some("radio")),
            ("<input type=\"range\">", Some("slider")),
            ("<input type=\"search\">", Some("searchbox")),
            ("<input type=\"reset\">", Some("button")),
            ("<li></li>", Some("generic")),
            ("<section aria-label=\"s\"></section>", Some("region")),
            ("<section></section>", Some("generic")),
            ("<select></select>", Some("combobox")),
            ("<select size=\"3\"></select>", Some("listbox")),
            ("<select multiple></select>", Some("listbox")),
            ("<td></td>", Some("cell")),
            ("<th scope=\"col\"></th>", Some("columnheader")),
            ("<th scope=\"row\"></th>", Some("rowheader")),
            ("<th></th>", Some("cell")),
            ("<div></div>", Some("generic")),
            ("<app-x></app-x>", None),
        ];
        let failures: Vec<_> = cases
            .iter()
            .filter(|(text, wanted)| role(text) != *wanted)
            .collect();
        assert_eq!(failures, Vec::<&(&str, Option<&str>)>::new());
        // A cell's role follows its table's.
        let grid = "<table role=\"grid\"><tr><td></td><th></th></tr></table>";
        let parsed = parse(grid);
        let doc = Doc::new(grid, &parsed.nodes);
        let role_of = |index: usize| doc.el(index).meta.and_then(|meta| meta.role(&doc, index));
        assert_eq!(
            (role_of(2), role_of(3)),
            (Some("gridcell"), Some("gridcell"))
        );
    }

    #[test]
    fn tells_the_categories_and_states_html5_ts_tells() {
        // Our own cases, each after its callback in `src/elements/html5.ts`.
        let category = |text: &str, which: &dyn Fn(&super::Meta) -> super::Cat| {
            first(text, &|doc, index| {
                doc.el(index)
                    .meta
                    .is_some_and(|meta| which(meta).of(doc, index))
            })
        };
        assert!(category("<a href=\"x\"></a>", &|meta| meta.interactive));
        assert!(!category("<a></a>", &|meta| meta.interactive));
        assert!(category("<audio controls></audio>", &|meta| meta.focusable));
        assert!(category("<img usemap=\"#m\">", &|meta| meta.interactive));
        assert!(!category("<input type=\"hidden\">", &|meta| meta.labelable));
        assert!(category("<input type=\"image\">", &|meta| meta.submit_button));
        assert!(category("<button></button>", &|meta| meta.submit_button));
        assert!(!category("<button type=\"button\"></button>", &|meta| meta
            .submit_button));
        assert!(category("<meta itemprop=\"x\">", &|meta| meta.flow));
        assert!(category("<link rel=\"stylesheet\">", &|meta| meta.phrasing));
        assert!(category("<link itemprop=\"x\">", &|meta| meta.flow));
        assert!(!category("<link rel=\"icon\">", &|meta| meta.flow));
        assert!(category("<details><summary></summary></details>", &|_| {
            super::Cat::Yes
        }));
        let naming = |text: &str| {
            first(text, &|doc, index| {
                doc.el(index)
                    .meta
                    .is_some_and(|meta| meta.naming_allowed(doc, index))
            })
        };
        assert!(naming("<img alt=\"a\">"));
        assert!(!naming("<img>"));
        assert!(naming("<footer></footer>"));
        let landmark = "<main><footer></footer></main>";
        let parsed = parse(landmark);
        let doc = Doc::new(landmark, &parsed.nodes);
        assert!(
            !doc.el(1)
                .meta
                .is_some_and(|meta| meta.naming_allowed(&doc, 1))
        );
        assert!(!naming("<span></span>"));
    }

    #[test]
    fn allows_an_attribute_as_html5_ts_does() {
        // Our own cases, each after its `allowed` callback and the helpers of
        // `src/meta/helper.ts`.
        let cases = [
            (
                "<a download></a>",
                "download",
                Some("requires \"href\" attribute to be present"),
            ),
            (
                "<a href=\"x\" rel=\"icon nofollow\"></a>",
                "rel",
                Some("<a> does not allow rel=\"icon\""),
            ),
            (
                "<a href=\"x\" rel=\"dcterms.x\"></a>",
                "rel",
                Some("<a> does not allow rel=\"dcterms.x\""),
            ),
            ("<a href=\"x\" rel=\"nofollow\"></a>", "rel", None),
            (
                "<area rel=\"x\">",
                "rel",
                Some("requires \"href\" attribute to be present"),
            ),
            (
                "<area href=\"x\" rel=\"icon\">",
                "rel",
                Some("<area> does not allow rel=\"icon\""),
            ),
            (
                "<area shape=\"default\" coords=\"1\">",
                "coords",
                Some("cannot be used when \"shape\" attribute is \"default\""),
            ),
            (
                "<area shape=\"circle\">",
                "shape",
                Some("requires \"coords\" attribute to be present"),
            ),
            ("<area shape=\"default\">", "shape", None),
            (
                "<form rel=\"author\"></form>",
                "rel",
                Some("<form> does not allow rel=\"author\""),
            ),
            (
                "<link rel=\"nofollow\">",
                "rel",
                Some("<link> does not allow rel=\"nofollow\""),
            ),
            (
                "<link imagesrcset=\"a.png\">",
                "imagesrcset",
                Some("\"rel\" attribute must be \"preload\""),
            ),
            (
                "<link rel=\"preload\" imagesrcset=\"a.png\">",
                "imagesrcset",
                Some("\"as\" attribute must be \"image\""),
            ),
            (
                "<link rel=\"preload\" as=\"image\" imagesrcset=\"a.png\">",
                "imagesrcset",
                None,
            ),
            (
                "<link as=\"font\">",
                "as",
                Some("\"rel\" attribute must be \"prefetch\", \"preload\" or \"modulepreload\""),
            ),
            (
                "<meta name=\"a\" itemprop=\"b\">",
                "name",
                Some("cannot be used at the same time as \"itemprop\""),
            ),
            (
                "<source srcset=\"a\">",
                "srcset",
                Some("requires <picture> as parent"),
            ),
            (
                "<button type=\"button\" formaction=\"x\"></button>",
                "formaction",
                Some("\"type\" attribute must be \"submit\""),
            ),
            ("<button formaction=\"x\"></button>", "formaction", None),
            (
                "<input type=\"text\" formaction=\"x\">",
                "formaction",
                Some("\"type\" attribute must be \"submit\" or \"image\""),
            ),
        ];
        let failures: Vec<String> = cases
            .iter()
            .filter(|(text, key, wanted)| misuse(text, key).as_deref() != *wanted)
            .map(|(text, key, _)| format!("{text} {key}: {:?}", misuse(text, key)))
            .collect();
        assert_eq!(failures, Vec::<String>::new());
    }

    #[test]
    fn requires_a_link_s_href_or_imagesizes() {
        // Our own cases, after `required` of `<link>`'s `href` and `imagesizes`.
        let required = |text: &str, key: &str| {
            first(text, &|doc, index| match meta_for("link")
                .and_then(|meta| attribute(meta, key))
                .map(|each| each.required)
            {
                Some(Required::When(function)) => function(doc, index),
                _ => None,
            })
        };
        assert!(required("<link>", "href").is_some());
        assert_eq!(required("<link imagesrcset=\"a.png\">", "href"), None);
        assert!(required("<link imagesrcset=\"a.png 100w\">", "imagesizes").is_some());
        assert_eq!(
            required("<link imagesrcset=\"a.png 2x\">", "imagesizes"),
            None
        );
        assert_eq!(
            (js_number(" 3 "), js_number(""), js_number("x")),
            (Some(3.0), Some(0.0), None)
        );
    }

    #[test]
    fn names_each_element_once_in_order() {
        // Our own case: the table is sorted and each name is unique, as the TypeScript object's
        // keys are.
        let names: Vec<&str> = ELEMENTS.iter().map(|meta| meta.name).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(names, sorted);
        assert_eq!(names.len(), 145);
    }

    #[test]
    fn joins_names_and_finds_width_descriptors() {
        // Our own cases, after `naturalJoin` and `parseImageCandidateString`.
        let names = |items: &[&str]| {
            items
                .iter()
                .map(|item| (*item).to_owned())
                .collect::<Vec<_>>()
        };
        assert_eq!(natural_join(&names(&["a"])), "a");
        assert_eq!(natural_join(&names(&["a", "b"])), "a or b");
        assert_eq!(natural_join(&names(&["a", "b", "c"])), "a, b or c");
        assert!(has_width_descriptor("a.png 100w, b.png 2x"));
        assert!(!has_width_descriptor("a.png 2x, b.png"));
        assert!(!has_width_descriptor(" "));
    }
}
