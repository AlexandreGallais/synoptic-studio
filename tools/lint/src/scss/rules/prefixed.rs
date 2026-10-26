//! What Autoprefixer would prefix: a port of `lib/utils/isAutoprefixable.mjs` and `vendor.mjs` of
//! `stylelint` 17.16.0 (REF-STYLELINT-SOURCE, MIT, under ADR-0076), the `no-vendor-prefix` rules
//! reporting a prefix only where the unprefixed form is the one to write.

const AT_RULES: &[&str] = &[
    "@-moz-keyframes",
    "@-moz-document",
    "@-ms-keyframes",
    "@-ms-viewport",
    "@-o-keyframes",
    "@-o-viewport",
    "@-webkit-keyframes",
    "@-webkit-viewport",
];
const SELECTORS: &[&str] = &[
    ":-moz-any-link",
    ":-moz-full-screen",
    ":-moz-placeholder",
    ":-moz-placeholder-shown",
    ":-moz-read-only",
    ":-moz-read-write",
    ":-ms-fullscreen",
    ":-ms-input-placeholder",
    ":-webkit-any-link",
    ":-webkit-full-screen",
    "::-moz-placeholder",
    "::-moz-selection",
    "::-ms-input-placeholder",
    "::-webkit-backdrop",
    "::-webkit-input-placeholder",
];
const PROPERTIES: &[&str] = &[
    "align-content",
    "align-items",
    "align-self",
    "animation",
    "animation-delay",
    "animation-direction",
    "animation-duration",
    "animation-fill-mode",
    "animation-iteration-count",
    "animation-name",
    "animation-play-state",
    "animation-timing-function",
    "appearance",
    "backdrop-filter",
    "backface-visibility",
    "background-clip",
    "background-origin",
    "background-size",
    "border-block-end",
    "border-block-start",
    "border-bottom-left-radius",
    "border-bottom-right-radius",
    "border-image",
    "border-inline-end",
    "border-inline-start",
    "border-radius",
    "border-top-left-radius",
    "border-top-right-radius",
    "box-decoration-break",
    "box-shadow",
    "box-sizing",
    "break-after",
    "break-before",
    "break-inside",
    "clip-path",
    "color-adjust",
    "column-count",
    "column-fill",
    "column-gap",
    "column-rule",
    "column-rule-color",
    "column-rule-style",
    "column-rule-width",
    "column-span",
    "column-width",
    "columns",
    "filter",
    "flex",
    "flex-basis",
    "flex-direction",
    "flex-flow",
    "flex-grow",
    "flex-shrink",
    "flex-wrap",
    "flow-from",
    "flow-into",
    "font-feature-settings",
    "font-kerning",
    "font-language-override",
    "font-variant-ligatures",
    "grid-area",
    "grid-column",
    "grid-column-align",
    "grid-column-end",
    "grid-column-start",
    "grid-row",
    "grid-row-align",
    "grid-row-end",
    "grid-row-start",
    "grid-template",
    "grid-template-areas",
    "grid-template-columns",
    "grid-template-rows",
    "hyphens",
    "image-rendering",
    "justify-content",
    "margin-block-end",
    "margin-block-start",
    "margin-inline-end",
    "margin-inline-start",
    "mask",
    "mask-border",
    "mask-border-outset",
    "mask-border-repeat",
    "mask-border-slice",
    "mask-border-source",
    "mask-border-width",
    "mask-clip",
    "mask-composite",
    "mask-image",
    "mask-origin",
    "mask-position",
    "mask-repeat",
    "mask-size",
    "object-fit",
    "object-position",
    "order",
    "overscroll-behavior",
    "padding-block-end",
    "padding-block-start",
    "padding-inline-end",
    "padding-inline-start",
    "perspective",
    "perspective-origin",
    "place-self",
    "region-fragment",
    "scroll-snap-coordinate",
    "scroll-snap-destination",
    "scroll-snap-points-x",
    "scroll-snap-points-y",
    "scroll-snap-type",
    "shape-image-threshold",
    "shape-margin",
    "shape-outside",
    "tab-size",
    "text-align-last",
    "text-decoration",
    "text-decoration-color",
    "text-decoration-line",
    "text-decoration-skip",
    "text-decoration-skip-ink",
    "text-decoration-style",
    "text-emphasis",
    "text-emphasis-color",
    "text-emphasis-position",
    "text-emphasis-style",
    "text-orientation",
    "text-overflow",
    "text-size-adjust",
    "text-spacing",
    "touch-action",
    "transform",
    "transform-origin",
    "transform-style",
    "transition",
    "transition-delay",
    "transition-duration",
    "transition-property",
    "transition-timing-function",
    "user-select",
    "writing-mode",
];
const PROPERTY_VALUES: &[&str] = &[
    "-moz-all",
    "-moz-calc",
    "-moz-crisp-edges",
    "-moz-element",
    "-moz-fit-content",
    "-moz-grab",
    "-moz-grabbing",
    "-moz-initial",
    "-moz-isolate",
    "-moz-isolate-override",
    "-moz-linear-gradient",
    "-moz-max-content",
    "-moz-min-content",
    "-moz-pre-wrap",
    "-moz-plaintext",
    "-moz-radial-gradient",
    "-moz-repeating-linear-gradient",
    "-moz-repeating-radial-gradient",
    "-moz-zoom-in",
    "-moz-zoom-out",
    "-ms-flexbox",
    "-ms-grid",
    "-ms-inline-grid",
    "-ms-linear-gradient",
    "-ms-radial-gradient",
    "-ms-repeating-linear-gradient",
    "-ms-repeating-radial-gradient",
    "-o-crisp-edges",
    "-o-linear-gradient",
    "-o-pre-wrap",
    "-o-radial-gradient",
    "-o-repeating-linear-gradient",
    "-o-repeating-radial-gradient",
    "-webkit-calc",
    "-webkit-cross-fade",
    "-webkit-filter",
    "-webkit-fit-content",
    "-webkit-flex",
    "-webkit-grab",
    "-webkit-grabbing",
    "-webkit-image-set",
    "-webkit-inline-flex",
    "-webkit-isolate",
    "-webkit-linear-gradient",
    "-webkit-max-content",
    "-webkit-min-content",
    "-webkit-plaintext",
    "-webkit-radial-gradient",
    "-webkit-repeating-linear-gradient",
    "-webkit-repeating-radial-gradient",
    "-webkit-sticky",
    "-webkit-zoom-in",
    "-webkit-zoom-out",
    "-xv-digits",
    "-xv-literal-punctuation",
    "-xv-no-punctuation",
    "-moz-arabic-indic",
    "-moz-bengali",
    "-moz-cjk-earthly-branch",
    "-moz-cjk-heavenly-stem",
    "-moz-devanagari",
    "-moz-ethiopic-numeric",
    "-moz-gujarati",
    "-moz-gurmukhi",
    "-moz-hangul",
    "-moz-hangul-consonant",
    "-moz-japanese-formal",
    "-moz-japanese-informal",
    "-moz-kannada",
    "-moz-khmer",
    "-moz-lao",
    "-moz-malayalam",
    "-moz-myanmar",
    "-moz-oriya",
    "-moz-persian",
    "-moz-simp-chinese-formal",
    "-moz-simp-chinese-informal",
    "-moz-tamil",
    "-moz-telugu",
    "-moz-thai",
    "-moz-trad-chinese-formal",
    "-moz-trad-chinese-informal",
];

/// A name's vendor prefix, `-webkit-` of `-webkit-a`, empty when it has none (`/^(-\w+-)/`).
#[must_use]
pub fn prefix(name: &str) -> &str {
    let Some(rest) = name.strip_prefix('-') else {
        return "";
    };
    let word = rest
        .bytes()
        .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
        .count();
    if word > 0 && rest.as_bytes().get(word) == Some(&b'-') {
        &name[..word + 2]
    } else {
        ""
    }
}

/// A name without its first vendor prefix anywhere (`replace(/-\w+-/, '')`).
#[must_use]
pub fn unprefix(name: &str) -> String {
    for (at, _) in name.match_indices('-') {
        let rest = &name[at + 1..];
        let word = rest
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
            .count();
        if word > 0 && rest.as_bytes().get(word) == Some(&b'-') {
            return format!("{}{}", &name[..at], &rest[word + 1..]);
        }
    }
    name.to_owned()
}

/// Whether a prefixed at-rule's name (without `@`) has a standard form (`atRuleName`).
#[must_use]
pub fn at_rule_name(name: &str) -> bool {
    AT_RULES.contains(&format!("@{}", name.to_lowercase()).as_str())
}

/// Whether a prefixed selector has a standard form (`selector`).
#[must_use]
pub fn selector(name: &str) -> bool {
    SELECTORS.contains(&name.to_lowercase().as_str())
}

/// Whether a prefixed media feature has a standard form (`mediaFeatureName`).
#[must_use]
pub fn media_feature_name(name: &str) -> bool {
    name.to_lowercase().contains("device-pixel-ratio")
}

/// Whether a prefixed property has a standard form (`property`).
#[must_use]
pub fn property(name: &str) -> bool {
    let lower = name.to_lowercase();
    let prefix = prefix(&lower);
    !prefix.is_empty() && PROPERTIES.contains(&&lower[prefix.len()..])
}

/// Whether a prefixed value has a standard form (`propertyValue`).
#[must_use]
pub fn property_value(value: &str) -> bool {
    PROPERTY_VALUES.contains(&value.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::{
        at_rule_name, media_feature_name, prefix, property, property_value, selector, unprefix,
    };

    #[test]
    fn tells_a_vendor_prefix_and_its_standard_form() {
        assert_eq!(
            (
                prefix("-webkit-box"),
                prefix("box"),
                prefix("--a"),
                prefix("-a")
            ),
            ("-webkit-", "", "", "")
        );
        assert_eq!(
            (
                unprefix("-moz-keyframes"),
                unprefix("a-b-c"),
                unprefix("ab")
            ),
            ("keyframes".to_owned(), "ac".to_owned(), "ab".to_owned())
        );
        assert!(at_rule_name("-WEBKIT-keyframes") && !at_rule_name("keyframes"));
        assert!(selector("::-moz-selection") && !selector("::selection"));
        assert!(media_feature_name("-webkit-min-device-pixel-ratio"));
        assert!(property("-webkit-transform") && !property("transform") && !property("-webkit-x"));
        assert!(property_value("-webkit-sticky") && !property_value("sticky"));
    }
}
