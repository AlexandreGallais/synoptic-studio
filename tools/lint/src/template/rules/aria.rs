//! The ARIA data of `aria-query` 5.3.2 (Apache-2.0, Copyright 2020 A11yance) the accessibility
//! rules read (REF-ARIA-QUERY-ROLES), from the `lib/` of its npm tarball: `aria`, each ARIA
//! property with the type of its value (`lib/ariaPropsMap.js`), and `roles`, each role with what
//! the rules read of it (`lib/rolesMap.js` and `lib/etc/roles/`, in that order: the abstract
//! roles, the literal ones, the DPUB ones, the graphics ones): whether abstract, its
//! superclasses, its required properties, and the HTML concepts of its base and related
//! concepts, from which `elementRoles` is built (`lib/elementRoleMap.js`).

/// The type of an ARIA property's value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyKind {
    /// An element's id.
    Id,
    /// A list of ids.
    IdList,
    /// `true` or `false`.
    Boolean,
    /// `true`, `false` or `mixed`.
    Tristate,
    /// An integer.
    Integer,
    /// A number.
    Number,
    /// Any string.
    String,
    /// One of the property's values.
    Token,
    /// A list of the property's values.
    TokenList,
}

/// A value of a token property: a string, or a boolean as `aria-query` writes some.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    /// A string.
    Text(&'static str),
    /// A boolean.
    Bool(bool),
}

/// An ARIA property.
#[derive(Debug)]
pub struct AriaProperty {
    /// Its name.
    pub name: &'static str,
    /// The type of its value.
    pub kind: PropertyKind,
    /// The values of a token or a token list.
    pub values: &'static [Token],
    /// Whether `undefined` is a value of it.
    pub allow_undefined: bool,
}

/// An attribute of a concept: its name, its value when it has one, its constraints.
#[derive(Debug, PartialEq, Eq)]
pub struct ConceptAttribute {
    /// Its name.
    pub name: &'static str,
    /// Its value.
    pub value: Option<&'static str>,
    /// Its constraints (`set`, `undefined`, `>1`…).
    pub constraints: &'static [&'static str],
}

/// A concept of the HTML module: an element, the attributes it is matched on, its constraints.
#[derive(Debug, PartialEq, Eq)]
pub struct Concept {
    /// The element's name; `None` for a concept of attributes only.
    pub name: Option<&'static str>,
    /// Its attributes.
    pub attributes: &'static [ConceptAttribute],
    /// Its constraints.
    pub constraints: &'static [&'static str],
}

/// An ARIA role.
#[derive(Debug)]
pub struct Role {
    /// Its name.
    pub name: &'static str,
    /// Whether abstract, never written in a template.
    pub is_abstract: bool,
    /// Its chains of superclasses.
    pub super_class: &'static [&'static [&'static str]],
    /// Its required properties, in `aria-query`'s order.
    pub required_props: &'static [&'static str],
    /// The HTML concepts of its base and related concepts, in that order.
    pub html: &'static [Concept],
}

/// `aria`: the ARIA properties.
pub const ARIA_PROPERTIES: [AriaProperty; 51] = [
    AriaProperty {
        name: "aria-activedescendant",
        kind: PropertyKind::Id,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-atomic",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-autocomplete",
        kind: PropertyKind::Token,
        values: &[
            Token::Text("inline"),
            Token::Text("list"),
            Token::Text("both"),
            Token::Text("none"),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-braillelabel",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-brailleroledescription",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-busy",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-checked",
        kind: PropertyKind::Tristate,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-colcount",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-colindex",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-colspan",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-controls",
        kind: PropertyKind::IdList,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-current",
        kind: PropertyKind::Token,
        values: &[
            Token::Text("page"),
            Token::Text("step"),
            Token::Text("location"),
            Token::Text("date"),
            Token::Text("time"),
            Token::Bool(true),
            Token::Bool(false),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-describedby",
        kind: PropertyKind::IdList,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-description",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-details",
        kind: PropertyKind::Id,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-disabled",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-dropeffect",
        kind: PropertyKind::TokenList,
        values: &[
            Token::Text("copy"),
            Token::Text("execute"),
            Token::Text("link"),
            Token::Text("move"),
            Token::Text("none"),
            Token::Text("popup"),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-errormessage",
        kind: PropertyKind::Id,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-expanded",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: true,
    },
    AriaProperty {
        name: "aria-flowto",
        kind: PropertyKind::IdList,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-grabbed",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: true,
    },
    AriaProperty {
        name: "aria-haspopup",
        kind: PropertyKind::Token,
        values: &[
            Token::Bool(false),
            Token::Bool(true),
            Token::Text("menu"),
            Token::Text("listbox"),
            Token::Text("tree"),
            Token::Text("grid"),
            Token::Text("dialog"),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-hidden",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: true,
    },
    AriaProperty {
        name: "aria-invalid",
        kind: PropertyKind::Token,
        values: &[
            Token::Text("grammar"),
            Token::Bool(false),
            Token::Text("spelling"),
            Token::Bool(true),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-keyshortcuts",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-label",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-labelledby",
        kind: PropertyKind::IdList,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-level",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-live",
        kind: PropertyKind::Token,
        values: &[
            Token::Text("assertive"),
            Token::Text("off"),
            Token::Text("polite"),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-modal",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-multiline",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-multiselectable",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-orientation",
        kind: PropertyKind::Token,
        values: &[
            Token::Text("vertical"),
            Token::Text("undefined"),
            Token::Text("horizontal"),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-owns",
        kind: PropertyKind::IdList,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-placeholder",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-posinset",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-pressed",
        kind: PropertyKind::Tristate,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-readonly",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-relevant",
        kind: PropertyKind::TokenList,
        values: &[
            Token::Text("additions"),
            Token::Text("all"),
            Token::Text("removals"),
            Token::Text("text"),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-required",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-roledescription",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-rowcount",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-rowindex",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-rowspan",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-selected",
        kind: PropertyKind::Boolean,
        values: &[],
        allow_undefined: true,
    },
    AriaProperty {
        name: "aria-setsize",
        kind: PropertyKind::Integer,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-sort",
        kind: PropertyKind::Token,
        values: &[
            Token::Text("ascending"),
            Token::Text("descending"),
            Token::Text("none"),
            Token::Text("other"),
        ],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-valuemax",
        kind: PropertyKind::Number,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-valuemin",
        kind: PropertyKind::Number,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-valuenow",
        kind: PropertyKind::Number,
        values: &[],
        allow_undefined: false,
    },
    AriaProperty {
        name: "aria-valuetext",
        kind: PropertyKind::String,
        values: &[],
        allow_undefined: false,
    },
];

/// `roles`: the roles.
pub const ROLES: [Role; 139] = [
    Role {
        name: "command",
        is_abstract: true,
        super_class: &[&["roletype", "widget"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "composite",
        is_abstract: true,
        super_class: &[&["roletype", "widget"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "input",
        is_abstract: true,
        super_class: &[&["roletype", "widget"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "landmark",
        is_abstract: true,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "range",
        is_abstract: true,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "roletype",
        is_abstract: true,
        super_class: &[],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "section",
        is_abstract: true,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "sectionhead",
        is_abstract: true,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "select",
        is_abstract: true,
        super_class: &[
            &["roletype", "widget", "composite"],
            &["roletype", "structure", "section", "group"],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "structure",
        is_abstract: true,
        super_class: &[&["roletype"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "widget",
        is_abstract: true,
        super_class: &[&["roletype"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "window",
        is_abstract: true,
        super_class: &[&["roletype"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "alert",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "alertdialog",
        is_abstract: false,
        super_class: &[
            &["roletype", "structure", "section", "alert"],
            &["roletype", "window", "dialog"],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "application",
        is_abstract: false,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "article",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "document"]],
        required_props: &[],
        html: &[Concept {
            name: Some("article"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "banner",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[Concept {
            name: Some("header"),
            attributes: &[],
            constraints: &["scoped to the body element"],
        }],
    },
    Role {
        name: "blockquote",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("blockquote"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "button",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "command"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("input"),
                attributes: &[ConceptAttribute {
                    name: "type",
                    value: Some("button"),
                    constraints: &[],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[ConceptAttribute {
                    name: "type",
                    value: Some("image"),
                    constraints: &[],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[ConceptAttribute {
                    name: "type",
                    value: Some("reset"),
                    constraints: &[],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[ConceptAttribute {
                    name: "type",
                    value: Some("submit"),
                    constraints: &[],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("button"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "caption",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("caption"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "cell",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("td"),
            attributes: &[],
            constraints: &["ancestor table element has table role"],
        }],
    },
    Role {
        name: "checkbox",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "input"]],
        required_props: &["aria-checked"],
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("checkbox"),
                constraints: &[],
            }],
            constraints: &[],
        }],
    },
    Role {
        name: "code",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("code"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "columnheader",
        is_abstract: false,
        super_class: &[
            &["roletype", "structure", "section", "cell"],
            &["roletype", "structure", "section", "cell", "gridcell"],
            &["roletype", "widget", "gridcell"],
            &["roletype", "structure", "sectionhead"],
        ],
        required_props: &[],
        html: &[
            Concept {
                name: Some("th"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("th"),
                attributes: &[ConceptAttribute {
                    name: "scope",
                    value: Some("col"),
                    constraints: &[],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("th"),
                attributes: &[ConceptAttribute {
                    name: "scope",
                    value: Some("colgroup"),
                    constraints: &[],
                }],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "combobox",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "input"]],
        required_props: &["aria-controls", "aria-expanded"],
        html: &[
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["set"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("email"),
                        constraints: &[],
                    },
                ],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["set"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("search"),
                        constraints: &[],
                    },
                ],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["set"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("tel"),
                        constraints: &[],
                    },
                ],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["set"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("text"),
                        constraints: &[],
                    },
                ],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["set"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("url"),
                        constraints: &[],
                    },
                ],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["set"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("url"),
                        constraints: &[],
                    },
                ],
                constraints: &[],
            },
            Concept {
                name: Some("select"),
                attributes: &[
                    ConceptAttribute {
                        name: "multiple",
                        value: None,
                        constraints: &["undefined"],
                    },
                    ConceptAttribute {
                        name: "size",
                        value: None,
                        constraints: &["undefined"],
                    },
                ],
                constraints: &[
                    "the multiple attribute is not set and the size attribute does not have a value greater than 1",
                ],
            },
        ],
    },
    Role {
        name: "complementary",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("aside"),
                attributes: &[],
                constraints: &["scoped to the body element", "scoped to the main element"],
            },
            Concept {
                name: Some("aside"),
                attributes: &[ConceptAttribute {
                    name: "aria-label",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[
                    "scoped to a sectioning content element",
                    "scoped to a sectioning root element other than body",
                ],
            },
            Concept {
                name: Some("aside"),
                attributes: &[ConceptAttribute {
                    name: "aria-labelledby",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[
                    "scoped to a sectioning content element",
                    "scoped to a sectioning root element other than body",
                ],
            },
        ],
    },
    Role {
        name: "contentinfo",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[Concept {
            name: Some("footer"),
            attributes: &[],
            constraints: &["scoped to the body element"],
        }],
    },
    Role {
        name: "definition",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("dd"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "deletion",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("del"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "dialog",
        is_abstract: false,
        super_class: &[&["roletype", "window"]],
        required_props: &[],
        html: &[Concept {
            name: Some("dialog"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "directory",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "list"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "document",
        is_abstract: false,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[Concept {
            name: Some("html"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "emphasis",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("em"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "feed",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "list"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "figure",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("figure"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "form",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("form"),
                attributes: &[ConceptAttribute {
                    name: "aria-label",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("form"),
                attributes: &[ConceptAttribute {
                    name: "aria-labelledby",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("form"),
                attributes: &[ConceptAttribute {
                    name: "name",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "generic",
        is_abstract: false,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("a"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("area"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("aside"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("b"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("bdo"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("body"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("data"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("div"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("footer"),
                attributes: &[],
                constraints: &[
                    "scoped to the main element",
                    "scoped to a sectioning content element",
                    "scoped to a sectioning root element other than body",
                ],
            },
            Concept {
                name: Some("header"),
                attributes: &[],
                constraints: &[
                    "scoped to the main element",
                    "scoped to a sectioning content element",
                    "scoped to a sectioning root element other than body",
                ],
            },
            Concept {
                name: Some("hgroup"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("i"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("pre"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("q"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("samp"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("section"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("small"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("span"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("u"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "grid",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite"],
            &["roletype", "structure", "section", "table"],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "gridcell",
        is_abstract: false,
        super_class: &[
            &["roletype", "structure", "section", "cell"],
            &["roletype", "widget"],
        ],
        required_props: &[],
        html: &[Concept {
            name: Some("td"),
            attributes: &[],
            constraints: &[
                "ancestor table element has grid role",
                "ancestor table element has treegrid role",
            ],
        }],
    },
    Role {
        name: "group",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("details"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("fieldset"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("optgroup"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("address"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "heading",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "sectionhead"]],
        required_props: &["aria-level"],
        html: &[
            Concept {
                name: Some("h1"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("h2"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("h3"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("h4"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("h5"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("h6"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "img",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("img"),
                attributes: &[ConceptAttribute {
                    name: "alt",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("img"),
                attributes: &[ConceptAttribute {
                    name: "alt",
                    value: None,
                    constraints: &["undefined"],
                }],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "insertion",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("ins"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "link",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "command"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("a"),
                attributes: &[ConceptAttribute {
                    name: "href",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("area"),
                attributes: &[ConceptAttribute {
                    name: "href",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "list",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("menu"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("ol"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("ul"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "listbox",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite", "select"],
            &["roletype", "structure", "section", "group", "select"],
        ],
        required_props: &[],
        html: &[
            Concept {
                name: Some("select"),
                attributes: &[ConceptAttribute {
                    name: "size",
                    value: None,
                    constraints: &[">1"],
                }],
                constraints: &["the size attribute value is greater than 1"],
            },
            Concept {
                name: Some("select"),
                attributes: &[ConceptAttribute {
                    name: "multiple",
                    value: None,
                    constraints: &[],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("datalist"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "listitem",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("li"),
            attributes: &[],
            constraints: &[
                "direct descendant of ol",
                "direct descendant of ul",
                "direct descendant of menu",
            ],
        }],
    },
    Role {
        name: "log",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "main",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[Concept {
            name: Some("main"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "mark",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("mark"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "marquee",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "math",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("math"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "menu",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite", "select"],
            &["roletype", "structure", "section", "group", "select"],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "menubar",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite", "select", "menu"],
            &[
                "roletype",
                "structure",
                "section",
                "group",
                "select",
                "menu",
            ],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "menuitem",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "command"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "menuitemcheckbox",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "input", "checkbox"],
            &["roletype", "widget", "command", "menuitem"],
        ],
        required_props: &["aria-checked"],
        html: &[],
    },
    Role {
        name: "menuitemradio",
        is_abstract: false,
        super_class: &[
            &[
                "roletype",
                "widget",
                "input",
                "checkbox",
                "menuitemcheckbox",
            ],
            &[
                "roletype",
                "widget",
                "command",
                "menuitem",
                "menuitemcheckbox",
            ],
            &["roletype", "widget", "input", "radio"],
        ],
        required_props: &["aria-checked"],
        html: &[],
    },
    Role {
        name: "meter",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "range"]],
        required_props: &["aria-valuenow"],
        html: &[Concept {
            name: Some("meter"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "navigation",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[Concept {
            name: Some("nav"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "none",
        is_abstract: false,
        super_class: &[],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "note",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "option",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "input"]],
        required_props: &["aria-selected"],
        html: &[Concept {
            name: Some("option"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "paragraph",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("p"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "presentation",
        is_abstract: false,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[Concept {
            name: Some("img"),
            attributes: &[ConceptAttribute {
                name: "alt",
                value: Some(""),
                constraints: &[],
            }],
            constraints: &[],
        }],
    },
    Role {
        name: "progressbar",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "range"], &["roletype", "widget"]],
        required_props: &[],
        html: &[Concept {
            name: Some("progress"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "radio",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "input"]],
        required_props: &["aria-checked"],
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("radio"),
                constraints: &[],
            }],
            constraints: &[],
        }],
    },
    Role {
        name: "radiogroup",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite", "select"],
            &["roletype", "structure", "section", "group", "select"],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "region",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("section"),
                attributes: &[ConceptAttribute {
                    name: "aria-label",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("section"),
                attributes: &[ConceptAttribute {
                    name: "aria-labelledby",
                    value: None,
                    constraints: &["set"],
                }],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "row",
        is_abstract: false,
        super_class: &[
            &["roletype", "structure", "section", "group"],
            &["roletype", "widget"],
        ],
        required_props: &[],
        html: &[Concept {
            name: Some("tr"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "rowgroup",
        is_abstract: false,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("tbody"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("tfoot"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("thead"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "rowheader",
        is_abstract: false,
        super_class: &[
            &["roletype", "structure", "section", "cell"],
            &["roletype", "structure", "section", "cell", "gridcell"],
            &["roletype", "widget", "gridcell"],
            &["roletype", "structure", "sectionhead"],
        ],
        required_props: &[],
        html: &[
            Concept {
                name: Some("th"),
                attributes: &[ConceptAttribute {
                    name: "scope",
                    value: Some("row"),
                    constraints: &[],
                }],
                constraints: &[],
            },
            Concept {
                name: Some("th"),
                attributes: &[ConceptAttribute {
                    name: "scope",
                    value: Some("rowgroup"),
                    constraints: &[],
                }],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "scrollbar",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "range"], &["roletype", "widget"]],
        required_props: &["aria-controls", "aria-valuenow"],
        html: &[],
    },
    Role {
        name: "search",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "searchbox",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "input", "textbox"]],
        required_props: &[],
        html: &[Concept {
            name: Some("input"),
            attributes: &[
                ConceptAttribute {
                    name: "list",
                    value: None,
                    constraints: &["undefined"],
                },
                ConceptAttribute {
                    name: "type",
                    value: Some("search"),
                    constraints: &[],
                },
            ],
            constraints: &["the list attribute is not set"],
        }],
    },
    Role {
        name: "separator",
        is_abstract: false,
        super_class: &[&["roletype", "structure"]],
        required_props: &[],
        html: &[Concept {
            name: Some("hr"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "slider",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "input"],
            &["roletype", "structure", "range"],
        ],
        required_props: &["aria-valuenow"],
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("range"),
                constraints: &[],
            }],
            constraints: &[],
        }],
    },
    Role {
        name: "spinbutton",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite"],
            &["roletype", "widget", "input"],
            &["roletype", "structure", "range"],
        ],
        required_props: &[],
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("number"),
                constraints: &[],
            }],
            constraints: &[],
        }],
    },
    Role {
        name: "status",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("output"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "strong",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("strong"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "subscript",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("sub"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "superscript",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("sup"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "switch",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "input", "checkbox"]],
        required_props: &["aria-checked"],
        html: &[],
    },
    Role {
        name: "tab",
        is_abstract: false,
        super_class: &[
            &["roletype", "structure", "sectionhead"],
            &["roletype", "widget"],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "table",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("table"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "tablist",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "composite"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "tabpanel",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "term",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("dfn"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("dt"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "textbox",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "input"]],
        required_props: &[],
        html: &[
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "type",
                        value: None,
                        constraints: &["undefined"],
                    },
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["undefined"],
                    },
                ],
                constraints: &["the list attribute is not set"],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["undefined"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("email"),
                        constraints: &[],
                    },
                ],
                constraints: &["the list attribute is not set"],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["undefined"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("tel"),
                        constraints: &[],
                    },
                ],
                constraints: &["the list attribute is not set"],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["undefined"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("text"),
                        constraints: &[],
                    },
                ],
                constraints: &["the list attribute is not set"],
            },
            Concept {
                name: Some("input"),
                attributes: &[
                    ConceptAttribute {
                        name: "list",
                        value: None,
                        constraints: &["undefined"],
                    },
                    ConceptAttribute {
                        name: "type",
                        value: Some("url"),
                        constraints: &[],
                    },
                ],
                constraints: &["the list attribute is not set"],
            },
            Concept {
                name: Some("textarea"),
                attributes: &[],
                constraints: &[],
            },
        ],
    },
    Role {
        name: "time",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[Concept {
            name: Some("time"),
            attributes: &[],
            constraints: &[],
        }],
    },
    Role {
        name: "timer",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "status"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "toolbar",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "group"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "tooltip",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "tree",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite", "select"],
            &["roletype", "structure", "section", "group", "select"],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "treegrid",
        is_abstract: false,
        super_class: &[
            &["roletype", "widget", "composite", "grid"],
            &["roletype", "structure", "section", "table", "grid"],
            &["roletype", "widget", "composite", "select", "tree"],
            &[
                "roletype",
                "structure",
                "section",
                "group",
                "select",
                "tree",
            ],
        ],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "treeitem",
        is_abstract: false,
        super_class: &[
            &["roletype", "structure", "section", "listitem"],
            &["roletype", "widget", "input", "option"],
        ],
        required_props: &["aria-selected"],
        html: &[],
    },
    Role {
        name: "doc-abstract",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-acknowledgments",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-afterword",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-appendix",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-backlink",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "command", "link"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-biblioentry",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "listitem"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-bibliography",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-biblioref",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "command", "link"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-chapter",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-colophon",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-conclusion",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-cover",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "img"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-credit",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-credits",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-dedication",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-endnote",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "listitem"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-endnotes",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-epigraph",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-epilogue",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-errata",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-example",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-footnote",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-foreword",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-glossary",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-glossref",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "command", "link"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-index",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark", "navigation"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-introduction",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-noteref",
        is_abstract: false,
        super_class: &[&["roletype", "widget", "command", "link"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-notice",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "note"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-pagebreak",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "separator"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-pagefooter",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-pageheader",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-pagelist",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark", "navigation"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-part",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-preface",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-prologue",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-pullquote",
        is_abstract: false,
        super_class: &[&["none"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-qna",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-subtitle",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "sectionhead"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-tip",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "note"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "doc-toc",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "landmark", "navigation"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "graphics-document",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "document"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "graphics-object",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "group"]],
        required_props: &[],
        html: &[],
    },
    Role {
        name: "graphics-symbol",
        is_abstract: false,
        super_class: &[&["roletype", "structure", "section", "img"]],
        required_props: &[],
        html: &[],
    },
];

/// A property by its name (`aria.get`).
#[must_use]
pub fn property(name: &str) -> Option<&'static AriaProperty> {
    ARIA_PROPERTIES.iter().find(|each| each.name == name)
}

/// A role by its name (`roles.get`).
#[must_use]
pub fn role(name: &str) -> Option<&'static Role> {
    ROLES.iter().find(|each| each.name == name)
}

/// `elementRoles`: each HTML concept and the roles it has, in the order `aria-query` builds
/// them: the roles walked in order, each concept of each joined to an equal one already met
/// (same name, constraints and attributes, each attribute's name, value and constraints), a
/// role listed once per concept.
#[must_use]
pub fn element_roles() -> Vec<(&'static Concept, Vec<&'static str>)> {
    let mut found: Vec<(&'static Concept, Vec<&'static str>)> = Vec::new();
    for role in &ROLES {
        for concept in role.html {
            match found.iter_mut().find(|(each, _)| *each == concept) {
                Some((_, roles)) if !roles.contains(&role.name) => roles.push(role.name),
                Some(_) => {}
                None => found.push((concept, vec![role.name])),
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::{ARIA_PROPERTIES, PropertyKind, ROLES, Token, element_roles, property, role};

    #[test]
    fn holds_aria_query_s_properties_and_roles() {
        // `src/ariaPropsMap.js` lists 51 properties; 12 abstract, 83 literal, 41 DPUB and 3
        // graphics roles make 139.
        assert_eq!((ARIA_PROPERTIES.len(), ROLES.len()), (51, 139));
        let current = property("aria-current").map(|each| (each.kind, each.values.len()));
        assert_eq!(current, Some((PropertyKind::Token, 7)));
        assert_eq!(
            property("aria-haspopup").map(|each| each.values[0]),
            Some(Token::Bool(false))
        );
        assert!(property("aria-expanded").is_some_and(|each| each.allow_undefined));
        assert_eq!(
            role("combobox").map(|each| each.required_props),
            Some(&["aria-controls", "aria-expanded"][..])
        );
        assert!(role("widget").is_some_and(|each| each.is_abstract));
        assert!(role("aria-x").is_none());
    }

    #[test]
    fn joins_equal_concepts_as_aria_query_does() {
        let roles = element_roles();
        let of = |name: &str, attribute: Option<(&str, &str)>| {
            roles
                .iter()
                .filter(|(concept, _)| {
                    concept.name == Some(name)
                        && concept.constraints.is_empty()
                        && match attribute {
                            None => concept.attributes.is_empty(),
                            Some((key, value)) => {
                                concept.attributes.len() == 1
                                    && concept.attributes[0].name == key
                                    && concept.attributes[0].value == Some(value)
                                    && concept.attributes[0].constraints.is_empty()
                            }
                        }
                })
                .map(|(_, roles)| roles.clone())
                .collect::<Vec<_>>()
        };

        // The roles hold 113 HTML concepts; `combobox` lists `<input list type="url">` twice,
        // joined into one entry where `combobox` is listed once: 112 entries.
        assert_eq!(roles.len(), 112);
        assert_eq!(of("input", Some(("type", "checkbox"))), [vec!["checkbox"]]);
        assert_eq!(of("button", None), [vec!["button"]]);
        // `<th>` is a base of `columnheader`; `<th scope="row">` of `rowheader`.
        assert_eq!(of("th", None), [vec!["columnheader"]]);
        assert_eq!(of("th", Some(("scope", "row"))), [vec!["rowheader"]]);
        // `<input list>` with the constraint `set`: 5 types, each of `combobox` alone.
        let listed = roles
            .iter()
            .filter(|(concept, _)| {
                concept
                    .attributes
                    .iter()
                    .any(|each| each.name == "list" && each.constraints == ["set"])
            })
            .map(|(_, roles)| roles.clone())
            .collect::<Vec<_>>();
        assert_eq!(listed, vec![vec!["combobox"]; 5]);
    }
}
