//! The accessibility objects of `axobject-query` 4.1.0 (Apache-2.0, Copyright 2020 A11yance)
//! the accessibility rules read (REF-AXOBJECT-QUERY), from the `lib/` of its npm tarball:
//! `AXObjects`, each object with its type (`lib/AXObjectsMap.js` and `lib/etc/objects/`, in
//! that order) and its related concepts: those of the HTML module, from which
//! `elementAXObjects` is built (`lib/elementAXObjectMap.js`), and the names of those of the ARIA
//! module, its roles (`AXObjectRoles`, `lib/AXObjectRoleMap.js`). An ARIA concept without a
//! name is left out: the rules compare a role's name to it, which never matches.

use super::aria::{Concept, ConceptAttribute};

/// An accessibility object.
#[derive(Debug)]
pub struct AxObject {
    /// Its name.
    pub name: &'static str,
    /// Its type: `widget`, `structure`, `window` or `generic`.
    pub kind: &'static str,
    /// Its related concepts of the HTML module.
    pub html: &'static [Concept],
    /// The names of its related concepts of the ARIA module.
    pub roles: &'static [&'static str],
}

/// `AXObjects`: the accessibility objects.
pub const AX_OBJECTS: [AxObject; 124] = [
    AxObject {
        name: "AbbrRole",
        kind: "structure",
        html: &[Concept {
            name: Some("abbr"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "AlertDialogRole",
        kind: "window",
        html: &[],
        roles: &["alertdialog"],
    },
    AxObject {
        name: "AlertRole",
        kind: "structure",
        html: &[],
        roles: &["alert"],
    },
    AxObject {
        name: "AnnotationRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ApplicationRole",
        kind: "window",
        html: &[],
        roles: &["application"],
    },
    AxObject {
        name: "ArticleRole",
        kind: "structure",
        html: &[Concept {
            name: Some("article"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["article"],
    },
    AxObject {
        name: "AudioRole",
        kind: "widget",
        html: &[Concept {
            name: Some("audio"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "BannerRole",
        kind: "structure",
        html: &[],
        roles: &["banner"],
    },
    AxObject {
        name: "BlockquoteRole",
        kind: "structure",
        html: &[Concept {
            name: Some("blockquote"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "BusyIndicatorRole",
        kind: "widget",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ButtonRole",
        kind: "widget",
        html: &[Concept {
            name: Some("button"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["button"],
    },
    AxObject {
        name: "CanvasRole",
        kind: "widget",
        html: &[Concept {
            name: Some("canvas"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "CaptionRole",
        kind: "structure",
        html: &[Concept {
            name: Some("caption"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "CellRole",
        kind: "widget",
        html: &[Concept {
            name: Some("td"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["cell", "gridcell"],
    },
    AxObject {
        name: "CheckBoxRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("checkbox"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["checkbox"],
    },
    AxObject {
        name: "ColorWellRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("color"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "ColumnHeaderRole",
        kind: "widget",
        html: &[Concept {
            name: Some("th"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["columnheader"],
    },
    AxObject {
        name: "ColumnRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ComboBoxRole",
        kind: "widget",
        html: &[Concept {
            name: Some("select"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["combobox"],
    },
    AxObject {
        name: "ComplementaryRole",
        kind: "structure",
        html: &[],
        roles: &["complementary"],
    },
    AxObject {
        name: "ContentInfoRole",
        kind: "structure",
        html: &[],
        roles: &["structureinfo"],
    },
    AxObject {
        name: "DateRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("date"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DateTimeRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("datetime"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DefinitionRole",
        kind: "structure",
        html: &[Concept {
            name: Some("dfn"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DescriptionListDetailRole",
        kind: "structure",
        html: &[Concept {
            name: Some("dd"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DescriptionListRole",
        kind: "structure",
        html: &[Concept {
            name: Some("dl"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DescriptionListTermRole",
        kind: "structure",
        html: &[Concept {
            name: Some("dt"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DetailsRole",
        kind: "structure",
        html: &[Concept {
            name: Some("details"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DialogRole",
        kind: "window",
        html: &[Concept {
            name: Some("dialog"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["dialog"],
    },
    AxObject {
        name: "DirectoryRole",
        kind: "structure",
        html: &[Concept {
            name: Some("dir"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["directory"],
    },
    AxObject {
        name: "DisclosureTriangleRole",
        kind: "widget",
        html: &[Concept {
            name: Some("summary"),
            attributes: &[],
            constraints: &["scoped to a details element"],
        }],
        roles: &[],
    },
    AxObject {
        name: "DivRole",
        kind: "generic",
        html: &[Concept {
            name: Some("div"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "DocumentRole",
        kind: "structure",
        html: &[],
        roles: &["document"],
    },
    AxObject {
        name: "EmbeddedObjectRole",
        kind: "widget",
        html: &[Concept {
            name: Some("embed"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "FeedRole",
        kind: "structure",
        html: &[],
        roles: &["feed"],
    },
    AxObject {
        name: "FigcaptionRole",
        kind: "structure",
        html: &[Concept {
            name: Some("figcaption"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "FigureRole",
        kind: "structure",
        html: &[Concept {
            name: Some("figure"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["figure"],
    },
    AxObject {
        name: "FooterRole",
        kind: "structure",
        html: &[Concept {
            name: Some("footer"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "FormRole",
        kind: "structure",
        html: &[Concept {
            name: Some("form"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["form"],
    },
    AxObject {
        name: "GridRole",
        kind: "widget",
        html: &[],
        roles: &["grid"],
    },
    AxObject {
        name: "GroupRole",
        kind: "structure",
        html: &[],
        roles: &["group"],
    },
    AxObject {
        name: "HeadingRole",
        kind: "structure",
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
        roles: &["heading"],
    },
    AxObject {
        name: "IframePresentationalRole",
        kind: "window",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "IframeRole",
        kind: "window",
        html: &[Concept {
            name: Some("iframe"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "IgnoredRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ImageMapLinkRole",
        kind: "widget",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ImageMapRole",
        kind: "structure",
        html: &[Concept {
            name: Some("img"),
            attributes: &[ConceptAttribute {
                name: "usemap",
                value: None,
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "ImageRole",
        kind: "structure",
        html: &[Concept {
            name: Some("img"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["img"],
    },
    AxObject {
        name: "InlineTextBoxRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "InputTimeRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("time"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "LabelRole",
        kind: "structure",
        html: &[Concept {
            name: Some("label"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "LegendRole",
        kind: "structure",
        html: &[Concept {
            name: Some("legend"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "LineBreakRole",
        kind: "structure",
        html: &[Concept {
            name: Some("br"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "LinkRole",
        kind: "widget",
        html: &[Concept {
            name: Some("a"),
            attributes: &[ConceptAttribute {
                name: "href",
                value: None,
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["link"],
    },
    AxObject {
        name: "ListBoxOptionRole",
        kind: "widget",
        html: &[Concept {
            name: Some("option"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["option"],
    },
    AxObject {
        name: "ListBoxRole",
        kind: "widget",
        html: &[
            Concept {
                name: Some("datalist"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("select"),
                attributes: &[],
                constraints: &[],
            },
        ],
        roles: &["listbox"],
    },
    AxObject {
        name: "ListItemRole",
        kind: "structure",
        html: &[Concept {
            name: Some("li"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["listitem"],
    },
    AxObject {
        name: "ListMarkerRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ListRole",
        kind: "structure",
        html: &[
            Concept {
                name: Some("ul"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("ol"),
                attributes: &[],
                constraints: &[],
            },
        ],
        roles: &["list"],
    },
    AxObject {
        name: "LogRole",
        kind: "structure",
        html: &[],
        roles: &["log"],
    },
    AxObject {
        name: "MainRole",
        kind: "structure",
        html: &[Concept {
            name: Some("main"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["main"],
    },
    AxObject {
        name: "MarkRole",
        kind: "structure",
        html: &[Concept {
            name: Some("mark"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "MarqueeRole",
        kind: "structure",
        html: &[Concept {
            name: Some("marquee"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["marquee"],
    },
    AxObject {
        name: "MathRole",
        kind: "structure",
        html: &[],
        roles: &["math"],
    },
    AxObject {
        name: "MenuBarRole",
        kind: "structure",
        html: &[],
        roles: &["menubar"],
    },
    AxObject {
        name: "MenuButtonRole",
        kind: "widget",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "MenuItemRole",
        kind: "widget",
        html: &[Concept {
            name: Some("menuitem"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["menuitem"],
    },
    AxObject {
        name: "MenuItemCheckBoxRole",
        kind: "widget",
        html: &[],
        roles: &["menuitemcheckbox"],
    },
    AxObject {
        name: "MenuItemRadioRole",
        kind: "widget",
        html: &[],
        roles: &["menuitemradio"],
    },
    AxObject {
        name: "MenuListOptionRole",
        kind: "widget",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "MenuListPopupRole",
        kind: "widget",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "MenuRole",
        kind: "structure",
        html: &[Concept {
            name: Some("menu"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["menu"],
    },
    AxObject {
        name: "MeterRole",
        kind: "structure",
        html: &[Concept {
            name: Some("meter"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "NavigationRole",
        kind: "structure",
        html: &[Concept {
            name: Some("nav"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["navigation"],
    },
    AxObject {
        name: "NoneRole",
        kind: "structure",
        html: &[],
        roles: &["none"],
    },
    AxObject {
        name: "NoteRole",
        kind: "structure",
        html: &[],
        roles: &["note"],
    },
    AxObject {
        name: "OutlineRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ParagraphRole",
        kind: "structure",
        html: &[Concept {
            name: Some("p"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "PopUpButtonRole",
        kind: "widget",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "PreRole",
        kind: "structure",
        html: &[Concept {
            name: Some("pre"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "PresentationalRole",
        kind: "structure",
        html: &[],
        roles: &["presentation"],
    },
    AxObject {
        name: "ProgressIndicatorRole",
        kind: "structure",
        html: &[Concept {
            name: Some("progress"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["progressbar"],
    },
    AxObject {
        name: "RadioButtonRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("radio"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["radio"],
    },
    AxObject {
        name: "RadioGroupRole",
        kind: "structure",
        html: &[],
        roles: &["radiogroup"],
    },
    AxObject {
        name: "RegionRole",
        kind: "structure",
        html: &[],
        roles: &["region"],
    },
    AxObject {
        name: "RootWebAreaRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "RowHeaderRole",
        kind: "widget",
        html: &[Concept {
            name: Some("th"),
            attributes: &[ConceptAttribute {
                name: "scope",
                value: Some("row"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["rowheader"],
    },
    AxObject {
        name: "RowRole",
        kind: "structure",
        html: &[Concept {
            name: Some("tr"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["row"],
    },
    AxObject {
        name: "RubyRole",
        kind: "structure",
        html: &[Concept {
            name: Some("ruby"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "RulerRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ScrollAreaRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ScrollBarRole",
        kind: "widget",
        html: &[],
        roles: &["scrollbar"],
    },
    AxObject {
        name: "SeamlessWebAreaRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "SearchRole",
        kind: "structure",
        html: &[],
        roles: &["search"],
    },
    AxObject {
        name: "SearchBoxRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("search"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["searchbox"],
    },
    AxObject {
        name: "SliderRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("range"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["slider"],
    },
    AxObject {
        name: "SliderThumbRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "SpinButtonRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("number"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["spinbutton"],
    },
    AxObject {
        name: "SpinButtonPartRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "SplitterRole",
        kind: "widget",
        html: &[],
        roles: &["separator"],
    },
    AxObject {
        name: "StaticTextRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "StatusRole",
        kind: "structure",
        html: &[],
        roles: &["status"],
    },
    AxObject {
        name: "SVGRootRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "SwitchRole",
        kind: "widget",
        html: &[Concept {
            name: Some("input"),
            attributes: &[ConceptAttribute {
                name: "type",
                value: Some("checkbox"),
                constraints: &[],
            }],
            constraints: &[],
        }],
        roles: &["switch"],
    },
    AxObject {
        name: "TabGroupRole",
        kind: "structure",
        html: &[],
        roles: &["tablist"],
    },
    AxObject {
        name: "TabRole",
        kind: "widget",
        html: &[],
        roles: &["tab"],
    },
    AxObject {
        name: "TableHeaderContainerRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "TableRole",
        kind: "structure",
        html: &[Concept {
            name: Some("table"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["table"],
    },
    AxObject {
        name: "TabListRole",
        kind: "structure",
        html: &[],
        roles: &["tablist"],
    },
    AxObject {
        name: "TabPanelRole",
        kind: "structure",
        html: &[],
        roles: &["tabpanel"],
    },
    AxObject {
        name: "TermRole",
        kind: "structure",
        html: &[],
        roles: &["term"],
    },
    AxObject {
        name: "TextAreaRole",
        kind: "widget",
        html: &[Concept {
            name: Some("textarea"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &["textbox"],
    },
    AxObject {
        name: "TextFieldRole",
        kind: "widget",
        html: &[
            Concept {
                name: Some("input"),
                attributes: &[],
                constraints: &[],
            },
            Concept {
                name: Some("input"),
                attributes: &[ConceptAttribute {
                    name: "type",
                    value: Some("text"),
                    constraints: &[],
                }],
                constraints: &[],
            },
        ],
        roles: &["textbox"],
    },
    AxObject {
        name: "TimeRole",
        kind: "structure",
        html: &[Concept {
            name: Some("time"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "TimerRole",
        kind: "structure",
        html: &[],
        roles: &["timer"],
    },
    AxObject {
        name: "ToggleButtonRole",
        kind: "widget",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "ToolbarRole",
        kind: "structure",
        html: &[],
        roles: &["toolbar"],
    },
    AxObject {
        name: "TreeRole",
        kind: "widget",
        html: &[],
        roles: &["tree"],
    },
    AxObject {
        name: "TreeGridRole",
        kind: "widget",
        html: &[],
        roles: &["treegrid"],
    },
    AxObject {
        name: "TreeItemRole",
        kind: "widget",
        html: &[],
        roles: &["treeitem"],
    },
    AxObject {
        name: "UserInterfaceTooltipRole",
        kind: "structure",
        html: &[],
        roles: &["tooltip"],
    },
    AxObject {
        name: "VideoRole",
        kind: "widget",
        html: &[Concept {
            name: Some("video"),
            attributes: &[],
            constraints: &[],
        }],
        roles: &[],
    },
    AxObject {
        name: "WebAreaRole",
        kind: "structure",
        html: &[],
        roles: &[],
    },
    AxObject {
        name: "WindowRole",
        kind: "window",
        html: &[],
        roles: &[],
    },
];

/// `elementAXObjects`: each HTML concept and the objects it has, in the order
/// `axobject-query` builds them: the objects walked in order, each concept joined to an equal one
/// already met, an object listed once per concept.
#[must_use]
pub fn element_ax_objects() -> Vec<(&'static Concept, Vec<&'static str>)> {
    let mut found: Vec<(&'static Concept, Vec<&'static str>)> = Vec::new();
    for object in &AX_OBJECTS {
        for concept in object.html {
            match found.iter_mut().find(|(each, _)| *each == concept) {
                Some((_, objects)) if !objects.contains(&object.name) => {
                    objects.push(object.name);
                }
                Some(_) => {}
                None => found.push((concept, vec![object.name])),
            }
        }
    }
    found
}

/// An object by its name (`AXObjects.get`).
#[must_use]
pub fn ax_object(name: &str) -> Option<&'static AxObject> {
    AX_OBJECTS.iter().find(|each| each.name == name)
}

#[cfg(test)]
mod tests {
    use super::{AX_OBJECTS, ax_object, element_ax_objects};

    #[test]
    fn holds_axobject_query_s_objects() {
        // `src/AXObjectsMap.js` lists 124 objects: 43 widgets, 74 structures, 6 windows, 1
        // generic.
        assert_eq!(AX_OBJECTS.len(), 124);
        let widgets = AX_OBJECTS
            .iter()
            .filter(|each| each.kind == "widget")
            .count();
        assert_eq!(widgets, 43);
        assert_eq!(
            ax_object("SwitchRole").map(|each| each.roles),
            Some(&["switch"][..])
        );
        assert!(ax_object("Switch").is_none());
    }

    #[test]
    fn joins_equal_concepts_as_axobject_query_does() {
        let objects = element_ax_objects();
        let checkbox = objects.iter().find(|(concept, _)| {
            concept.name == Some("input")
                && concept.attributes.len() == 1
                && concept.attributes[0].value == Some("checkbox")
        });

        // 73 HTML concepts, 70 of them different: `<input type="checkbox">` is the concept of
        // both `CheckBoxRole` and `SwitchRole`.
        assert_eq!(objects.len(), 70);
        assert_eq!(
            checkbox.map(|(_, names)| names.clone()),
            Some(vec!["CheckBoxRole", "SwitchRole"])
        );
    }
}
