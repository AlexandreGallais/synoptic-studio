//! What Stylelint's selector rules share: `isStandardSyntaxTypeSelector`, `isKeyframeSelector`,
//! `isCustomElement`, `normalizePseudoName` and `PostCSS`'s `list.comma`, of Stylelint 17.16.0 and
//! `postcss` 8.5.30 (REF-STYLELINT-SOURCE, MIT, under ADR-0076).

use super::reference::{
    A_NPLUS_BNOTATION_PSEUDO_CLASSES, A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES,
    LINGUISTIC_PSEUDO_CLASSES, MATHML_TAGS, NAMED_TIMELINE_RANGE_KEYWORDS,
    SHADOW_TREE_PSEUDO_ELEMENTS, SVG_TAGS, has,
};
use crate::javascript::{JsText, is_js_space};
use crate::scss::selector::{Kind, Selectors};

/// A pseudo's name: its colons dropped, in lower case (`normalizePseudoName`).
#[must_use]
pub fn pseudo_name(value: &str) -> String {
    value.trim_start_matches(':').to_lowercase()
}

/// The pseudo a part is an argument of: its selector's parent, when a pseudo.
#[must_use]
pub fn argument_of(parsed: &Selectors, index: usize) -> Option<usize> {
    let selector = parsed.part(index).parent?;
    let pseudo = parsed.part(selector).parent?;
    (parsed.part(pseudo).kind == Kind::Pseudo).then_some(pseudo)
}

/// Whether a tag is a type selector: not an `An+B` or language argument, not after `&`, not a
/// placeholder nor a named combinator (`isStandardSyntaxTypeSelector`).
#[must_use]
pub fn is_standard_type_selector(parsed: &Selectors, index: usize) -> bool {
    let part = parsed.part(index);
    let Some(selector) = part.parent else {
        return false;
    };
    if parsed.part(selector).parent.is_none() && parsed.part(selector).kind != Kind::Selector {
        return false;
    }
    if let Some(pseudo) = argument_of(parsed, index) {
        let name = pseudo_name(&parsed.part(pseudo).value);
        if has(A_NPLUS_BNOTATION_PSEUDO_CLASSES, &name)
            || has(A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES, &name)
            || has(LINGUISTIC_PSEUDO_CLASSES, &name)
            || has(SHADOW_TREE_PSEUDO_ELEMENTS, &name)
        {
            return false;
        }
    }
    if parsed
        .previous(index)
        .is_some_and(|previous| parsed.part(previous).kind == Kind::Nesting)
    {
        return false;
    }
    !(part.value.starts_with('%') || (part.value.starts_with('/') && part.value.ends_with('/')))
}

/// Whether a selector is a keyframe's: `from`, `to`, a percentage, or a timeline range and a
/// percentage (`isKeyframeSelector`).
#[must_use]
pub fn is_keyframe_selector(selector: &str) -> bool {
    let percentage = |text: &str| {
        text.strip_suffix('%').is_some_and(|number| {
            let (whole, fraction) = number.split_once('.').unwrap_or((number, ""));
            !number.is_empty()
                && whole.bytes().all(|byte| byte.is_ascii_digit())
                && fraction.bytes().all(|byte| byte.is_ascii_digit())
                && number.contains('.') != fraction.is_empty()
        })
    };
    if selector == "from" || selector == "to" || percentage(selector) {
        return true;
    }
    selector
        .split_once(is_js_space)
        .is_some_and(|(range, rest)| {
            has(NAMED_TIMELINE_RANGE_KEYWORDS, &range.to_lowercase())
                && percentage(rest.js_trim_start())
        })
}

/// Whether a type is a custom element: lower case, starting with a letter, holding a `-`, not
/// an SVG nor a `MathML` element (`isCustomElement`).
#[must_use]
pub fn is_custom_element(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_lowercase())
        && name.contains('-')
        && name.to_lowercase() == name
        && !SVG_TAGS.contains(&name)
        && !MATHML_TAGS.contains(&name)
}

/// Whether a selector may start a type selector somewhere (`/(?:[^.#[:a-z-]|^)[a-z]/i`).
#[must_use]
pub fn may_hold_a_type(selector: &str) -> bool {
    let bytes = selector.as_bytes();
    bytes.iter().enumerate().any(|(at, byte)| {
        byte.is_ascii_alphabetic()
            && (at == 0
                || !matches!(
                    bytes[at - 1].to_ascii_lowercase(),
                    b'.' | b'#' | b'[' | b':' | b'-' | b'a'..=b'z'
                ))
    })
}

/// A list split at its top-level commas, each item trimmed, as `PostCSS`'s `list.comma` splits a
/// rule's `selectors`.
#[must_use]
pub fn comma_list(text: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut current = String::new();
    let mut depth = 0_i32;
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for character in text.chars() {
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if let Some(open) = quote {
            if character == open {
                quote = None;
            }
        } else if character == '"' || character == '\'' {
            quote = Some(character);
        } else if character == '(' {
            depth += 1;
        } else if character == ')' {
            depth -= 1;
        } else if character == ',' && depth == 0 {
            items.push(current.js_trim().to_owned());
            current.clear();
            continue;
        }
        current.push(character);
    }
    if !current.js_trim().is_empty() || !items.is_empty() {
        items.push(current.js_trim().to_owned());
    }
    items.into_iter().filter(|item| !item.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        comma_list, is_custom_element, is_keyframe_selector, is_standard_type_selector,
        may_hold_a_type, pseudo_name,
    };
    use crate::scss::selector::parse;

    #[test]
    fn tells_the_kinds_of_selectors_the_rules_skip() {
        assert_eq!(pseudo_name("::BEFORE"), "before");
        let parsed = parse("a:nth-child(2n) &-b").unwrap_or_default();
        let tags: Vec<bool> = parsed
            .walk()
            .into_iter()
            .filter(|&index| parsed.part(index).kind == crate::scss::selector::Kind::Tag)
            .map(|index| is_standard_type_selector(&parsed, index))
            .collect();
        assert_eq!(tags, [true, false, false]);
        assert!(
            is_keyframe_selector("12.5%")
                && is_keyframe_selector("entry 10%")
                && !is_keyframe_selector("a")
        );
        assert!(
            is_custom_element("app-root")
                && !is_custom_element("font-face")
                && !is_custom_element("App-x")
        );
        assert!(may_hold_a_type("a") && may_hold_a_type(".b > c") && !may_hold_a_type(".b"));
        assert_eq!(
            comma_list("a, b:is(c, d), 'e,f'"),
            ["a", "b:is(c, d)", "'e,f'"]
        );
    }
}
