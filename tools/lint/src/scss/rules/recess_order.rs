//! The property groups of `stylelint-config-recess-order` 7.9.0 (`groups.js`,
//! REF-STYLELINT-RECESS-ORDER, ISC, under ADR-0076), copied as data: the order Recess and Bootstrap
//! sort a block's properties in, with the properties CSS added since, each group a list of
//! properties in its order. The configuration gives each group as `{ properties }` alone, without
//! `emptyLineBefore`, `noEmptyLineBetween` nor `order`, so that `order/properties-order` neither asks
//! nor refuses an empty line between groups (F53.AC3). Each group's comment is the package's.

/// The groups, in order, each its properties in order.
pub const GROUPS: &[&[&str]] = &[
    // Compose rules from other selectors in CSS Modules.
    // <https://github.com/css-modules/css-modules#composition>
    &["composes"],
    // Cascade and inheritance.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_cascade#reference>
    &["all"],
    // Positioned layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_positioned_layout#reference>
    &[
        "position",
        "inset",
        "inset-block",
        "inset-block-start",
        "inset-block-end",
        "inset-inline",
        "inset-inline-start",
        "inset-inline-end",
        "top",
        "right",
        "bottom",
        "left",
        "z-index",
        "float",
        "clear",
    ],
    // Display.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_display#reference>
    &["box-sizing", "display", "visibility"],
    // Multi-column layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Multicol_layout#reference>
    &[
        "columns",
        "column-width",
        "column-count",
        "column-height",
        "column-wrap",
        "column-span",
        "column-fill",
    ],
    // Flexible box layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_flexible_box_layout#reference>
    &[
        "flex",
        "flex-grow",
        "flex-shrink",
        "flex-basis",
        "flex-flow",
        "flex-direction",
        "flex-wrap",
        "-webkit-box-orient",
    ],
    // Grid layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_grid_layout#reference>
    &[
        "grid",
        "grid-area",
        "grid-template",
        "grid-template-areas",
        "grid-template-rows",
        "grid-template-columns",
        "grid-row",
        "grid-row-start",
        "grid-row-end",
        "grid-column",
        "grid-column-start",
        "grid-column-end",
        "grid-auto-rows",
        "grid-auto-columns",
        "grid-auto-flow",
        "grid-gap",
        "grid-row-gap",
        "grid-column-gap",
    ],
    // Gaps.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Gaps#reference>
    &[
        "gap",
        "row-gap",
        "column-gap",
        "column-rule",
        "column-rule-width",
        "column-rule-style",
        "column-rule-color",
    ],
    // Box alignment.  Relates to both Flexbox and Grid layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_alignment#reference>
    &[
        "place-content",
        "place-items",
        "place-self",
        "align-content",
        "align-items",
        "align-self",
        "justify-content",
        "justify-items",
        "justify-self",
    ],
    // Order. Part of Display module, but relates to both Flexbox and Grid layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_alignment#reference>
    &["order"],
    // Box sizing.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_sizing#reference>
    &[
        "inline-size",
        "min-inline-size",
        "max-inline-size",
        "width",
        "min-width",
        "max-width",
        "block-size",
        "min-block-size",
        "max-block-size",
        "height",
        "min-height",
        "max-height",
        "aspect-ratio",
        "contain-intrinsic-inline-size",
        "contain-intrinsic-block-size",
        "contain-intrinsic-size",
        "contain-intrinsic-width",
        "contain-intrinsic-height",
    ],
    // Box model.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_model#reference>
    &[
        "padding",
        "padding-block",
        "padding-block-start",
        "padding-block-end",
        "padding-inline",
        "padding-inline-start",
        "padding-inline-end",
        "padding-top",
        "padding-right",
        "padding-bottom",
        "padding-left",
        "margin",
        "margin-block",
        "margin-block-start",
        "margin-block-end",
        "margin-inline",
        "margin-inline-start",
        "margin-inline-end",
        "margin-top",
        "margin-right",
        "margin-bottom",
        "margin-left",
    ],
    // Anchor positioning.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_anchor_positioning#reference>
    &[
        "anchor-name",
        "anchor-scope",
        "position-area",
        "position-anchor",
        "position-try",
        "position-try-order",
        "position-try-fallbacks",
        "position-visibility",
    ],
    // Containment.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_containment#reference>
    &[
        "contain",
        "container",
        "container-name",
        "container-type",
        "content-visibility",
    ],
    // Overflow.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_overflow#reference>
    &[
        "overflow",
        "overflow-inline",
        "overflow-block",
        "overflow-x",
        "overflow-y",
        "scrollbar-gutter",
        "-webkit-overflow-scrolling",
        "-ms-overflow-x",
        "-ms-overflow-y",
        "-ms-overflow-style",
        "text-overflow",
        "-webkit-line-clamp",
        "line-clamp",
        "scroll-behavior",
    ],
    // Overscroll behavior.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_overscroll_behavior#reference>
    &[
        "overscroll-behavior",
        "overscroll-behavior-inline",
        "overscroll-behavior-block",
        "overscroll-behavior-x",
        "overscroll-behavior-y",
    ],
    // Fonts.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_fonts#reference>
    &[
        "font",
        "font-family",
        "font-size",
        "font-size-adjust",
        "font-variation-settings",
        "font-style",
        "font-weight",
        "font-optical-sizing",
        "font-stretch",
        "font-feature-settings",
        "font-kerning",
        "font-variant",
        "font-variant-ligatures",
        "font-variant-caps",
        "font-variant-alternates",
        "font-variant-numeric",
        "font-variant-east-asian",
        "font-variant-position",
        "-webkit-font-smoothing",
        "-moz-osx-font-smoothing",
        "font-smooth",
        "font-synthesis",
        "font-synthesis-weight",
        "font-synthesis-style",
        "font-synthesis-small-caps",
    ],
    // Inline layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_inline_layout#reference>
    &[
        "line-height",
        "vertical-align",
        "alignment-baseline",
        "baseline-shift",
        "dominant-baseline",
        "text-box",
        "text-box-trim",
        "text-box-edge",
    ],
    // Colors. Although `opacity` is technically part of this module, it is grouped with the Compositing and blending module for now.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_colors#reference>
    &[
        "base-palette",
        "override-colors",
        "font-palette",
        "color",
        "-webkit-text-fill-color",
        "-webkit-text-stroke",
        "-webkit-text-stroke-width",
        "-webkit-text-stroke-color",
    ],
    // Text.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_text#reference>
    &[
        "text-align",
        "text-align-last",
        "text-justify",
        "text-indent",
        "text-transform",
        "text-autospace",
        "word-spacing",
        "letter-spacing",
        "hyphens",
        "hyphenate-character",
        "line-break",
        "word-break",
        "text-wrap",
        "text-wrap-mode",
        "text-wrap-style",
        "word-wrap",
        "overflow-wrap",
        "tab-size",
        "white-space",
        "white-space-collapse",
    ],
    // Text decoration.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_text_decoration#reference>
    &[
        "text-emphasis",
        "text-emphasis-color",
        "text-emphasis-style",
        "text-emphasis-position",
        "text-decoration",
        "text-decoration-line",
        "text-decoration-thickness",
        "text-decoration-style",
        "text-decoration-color",
        "text-decoration-skip-ink",
        "text-underline-position",
        "text-underline-offset",
        "text-shadow",
    ],
    // Ruby layout.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_ruby_layout#reference>
    &["ruby-position", "ruby-align"],
    // Font loading.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_font_loading#reference>
    &[
        "src",
        "font-display",
        "unicode-range",
        "size-adjust",
        "ascent-override",
        "descent-override",
        "line-gap-override",
    ],
    // Basic user interface.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_basic_user_interface#reference>
    &[
        "appearance",
        "accent-color",
        "pointer-events",
        "-ms-touch-action",
        "touch-action",
        "cursor",
        "caret-color",
        "zoom",
        "field-sizing",
        "resize",
        "user-select",
        "-webkit-user-select",
        "nav-index",
        "nav-up",
        "nav-right",
        "nav-down",
        "nav-left",
        "outline",
        "outline-width",
        "outline-style",
        "outline-color",
        "outline-offset",
    ],
    // Color adjustment.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_color_adjustment#reference>
    &["color-scheme", "forced-color-adjust", "print-color-adjust"],
    // Table.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_table#reference>
    &[
        "table-layout",
        "empty-cells",
        "caption-side",
        "border-spacing",
        "border-collapse",
    ],
    // Generated content.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_generated_content#reference>
    &["content", "quotes"],
    // Lists and counters.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_lists#reference>
    &[
        "list-style",
        "list-style-position",
        "list-style-type",
        "list-style-image",
        "counter-reset",
        "counter-set",
        "counter-increment",
    ],
    // Scroll snap.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_scroll_snap#reference>
    &[
        "scroll-snap-type",
        "scroll-snap-align",
        "scroll-snap-stop",
        "scroll-padding",
        "scroll-padding-block",
        "scroll-padding-block-start",
        "scroll-padding-block-end",
        "scroll-padding-inline",
        "scroll-padding-inline-start",
        "scroll-padding-inline-end",
        "scroll-padding-top",
        "scroll-padding-right",
        "scroll-padding-bottom",
        "scroll-padding-left",
        "scroll-margin",
        "scroll-margin-block",
        "scroll-margin-block-start",
        "scroll-margin-block-end",
        "scroll-margin-inline",
        "scroll-margin-inline-start",
        "scroll-margin-inline-end",
        "scroll-margin-top",
        "scroll-margin-right",
        "scroll-margin-bottom",
        "scroll-margin-left",
    ],
    // Scrollbars styling.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_scrollbars_styling#reference>
    &["scrollbar-color", "scrollbar-width"],
    // Images.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_images#reference>
    &[
        "object-fit",
        "object-position",
        "-ms-interpolation-mode",
        "image-orientation",
        "image-rendering",
        "image-resolution",
    ],
    // Backgrounds and borders.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_backgrounds_and_borders#reference>
    &[
        "background",
        "background-color",
        "background-image",
        "-ms-filter:\\'progid:DXImageTransform.Microsoft.gradient",
        "filter:progid:DXImageTransform.Microsoft.gradient",
        "filter:progid:DXImageTransform.Microsoft.AlphaImageLoader",
        "background-repeat",
        "background-attachment",
        "background-position",
        "background-position-x",
        "background-position-y",
        "background-clip",
        "background-origin",
        "background-size",
        "border",
        "border-color",
        "border-style",
        "border-width",
        "border-block",
        "border-block-color",
        "border-block-style",
        "border-block-width",
        "border-block-start",
        "border-block-start-color",
        "border-block-start-style",
        "border-block-start-width",
        "border-block-end",
        "border-block-end-color",
        "border-block-end-style",
        "border-block-end-width",
        "border-inline",
        "border-inline-color",
        "border-inline-style",
        "border-inline-width",
        "border-inline-start",
        "border-inline-start-color",
        "border-inline-start-style",
        "border-inline-start-width",
        "border-inline-end",
        "border-inline-end-color",
        "border-inline-end-style",
        "border-inline-end-width",
        "border-top",
        "border-top-color",
        "border-top-style",
        "border-top-width",
        "border-right",
        "border-right-color",
        "border-right-style",
        "border-right-width",
        "border-bottom",
        "border-bottom-color",
        "border-bottom-style",
        "border-bottom-width",
        "border-left",
        "border-left-color",
        "border-left-style",
        "border-left-width",
        "corner",
        "corner-block-start",
        "corner-block-end",
        "corner-inline-start",
        "corner-inline-end",
        "corner-start-start",
        "corner-start-end",
        "corner-end-start",
        "corner-end-end",
        "corner-top",
        "corner-right",
        "corner-bottom",
        "corner-left",
        "corner-top-left",
        "corner-top-right",
        "corner-bottom-right",
        "corner-bottom-left",
        "border-radius",
        "border-start-start-radius",
        "border-start-end-radius",
        "border-end-start-radius",
        "border-end-end-radius",
        "border-top-left-radius",
        "border-top-right-radius",
        "border-bottom-right-radius",
        "border-bottom-left-radius",
        "corner-shape",
        "corner-block-start-shape",
        "corner-block-end-shape",
        "corner-inline-start-shape",
        "corner-inline-end-shape",
        "corner-start-start-shape",
        "corner-start-end-shape",
        "corner-end-start-shape",
        "corner-end-end-shape",
        "corner-top-shape",
        "corner-right-shape",
        "corner-bottom-shape",
        "corner-left-shape",
        "corner-top-left-shape",
        "corner-top-right-shape",
        "corner-bottom-right-shape",
        "corner-bottom-left-shape",
        "border-image",
        "border-image-source",
        "border-image-slice",
        "border-image-width",
        "border-image-outset",
        "border-image-repeat",
        "box-shadow",
    ],
    // Compositing and blending.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_compositing_and_blending#reference>
    &[
        "background-blend-mode",
        "isolation",
        "mix-blend-mode",
        "filter:progid:DXImageTransform.Microsoft.Alpha(Opacity",
        "-ms-filter:\\'progid:DXImageTransform.Microsoft.Alpha",
        "opacity",
    ],
    // Filter effects.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_filter_effects#properties>
    &["filter", "backdrop-filter"],
    // Masking.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_masking#reference>
    &[
        "clip",
        "clip-path",
        "clip-rule",
        "mask-border",
        "mask-border-source",
        "mask-border-slice",
        "mask-border-width",
        "mask-border-outset",
        "mask-border-repeat",
        "mask-border-mode",
        "mask",
        "mask-image",
        "mask-mode",
        "mask-repeat",
        "mask-position",
        "mask-clip",
        "mask-origin",
        "mask-size",
        "mask-composite",
        "mask-type",
    ],
    // Shapes.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_shapes#reference>
    &["shape-outside", "shape-image-threshold", "shape-margin"],
    // Writing modes.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_writing_modes#reference>
    &[
        "direction",
        "unicode-bidi",
        "writing-mode",
        "text-orientation",
        "text-combine-upright",
    ],
    // SVG presentation attributes. Some of these may fall under a specific module, but if they only apply to SVG-specific elements, they will be grouped here.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_writing_modes#reference>
    &[
        "x",
        "y",
        "rx",
        "ry",
        "cx",
        "cy",
        "r",
        "d",
        "text-anchor",
        "fill",
        "fill-rule",
        "fill-opacity",
        "stroke",
        "stroke-opacity",
        "stroke-width",
        "stroke-linecap",
        "stroke-linejoin",
        "stroke-miterlimit",
        "stroke-dasharray",
        "stroke-dashoffset",
        "vector-effect",
        "color-interpolation",
        "color-interpolation-filters",
        "flood-color",
        "flood-opacity",
        "lighting-color",
        "marker",
        "marker-start",
        "marker-mid",
        "marker-end",
        "stop-color",
        "stop-opacity",
        "paint-order",
        "shape-rendering",
        "text-rendering",
    ],
    // MathML properties.
    // <https://w3c.github.io/mathml-core/>
    &["math-style", "math-shift", "math-depth"],
    // Transforms.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_transforms#reference>
    &[
        "transform",
        "transform-origin",
        "transform-box",
        "transform-style",
        "rotate",
        "scale",
        "translate",
        "perspective",
        "perspective-origin",
        "backface-visibility",
    ],
    // Transitions.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_transitions#reference>
    &[
        "transition",
        "transition-delay",
        "transition-timing-function",
        "transition-duration",
        "transition-property",
        "transition-behavior",
    ],
    // View transitions.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_view_transitions#reference>
    &["view-transition-name", "view-transition-class"],
    // Animations.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_animations#reference>
    &[
        "animation",
        "animation-name",
        "animation-duration",
        "animation-timing-function",
        "animation-delay",
        "animation-iteration-count",
        "animation-direction",
        "animation-fill-mode",
        "animation-play-state",
        "animation-composition",
    ],
    // Motion path.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_motion_path#reference>
    &[
        "offset",
        "offset-position",
        "offset-path",
        "offset-distance",
        "offset-rotate",
        "offset-anchor",
    ],
    // Will change.
    // <https://drafts.csswg.org/css-will-change/#will-change>
    &["will-change"],
    // Paged media.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Paged_media#reference>
    &["page"],
    // Fragmentation.
    // <https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_fragmentation#reference>
    &[
        "break-before",
        "break-after",
        "break-inside",
        "widows",
        "orphans",
    ],
];

#[cfg(test)]
mod tests {
    use super::GROUPS;
    use crate::scss::rules::tester::{Js, constants};

    #[test]
    fn f53_ac3_copies_the_groups_of_the_package_one_for_one() {
        // `groups.js` of the package, copied as it is, read by Oxc as the tester reads a test file.
        let groups = constants("stylelint-config-recess-order/groups.txt")
            .into_iter()
            .find(|(name, _)| name == "propertyGroups")
            .map(|(_, value)| value);
        let Some(Js::List(groups)) = groups else {
            panic!("propertyGroups not read: {groups:?}");
        };
        let read: Vec<Vec<String>> = groups
            .iter()
            .map(|group| group.texts("properties").unwrap_or_default())
            .collect();

        // 47 groups, 520 properties, each group an object of `properties` alone.
        assert!(
            groups
                .iter()
                .all(|group| matches!(group, Js::Object(keys) if keys.len() == 1))
        );
        assert_eq!(
            (read.len(), read.iter().map(Vec::len).sum::<usize>()),
            (47, 520)
        );
        assert_eq!(
            read,
            GROUPS
                .iter()
                .map(|group| group
                    .iter()
                    .map(|&each| each.to_owned())
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );
    }
}
