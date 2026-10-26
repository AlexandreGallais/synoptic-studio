//! What stylelint-scss 7.3.0's selector rules share (REF-STYLELINT-SOURCE, MIT, under ADR-0076):
//! its own `isStandardRule`, `isStandardSelector` (a `%placeholder` standard) and
//! `hasInterpolatingAmpersand`, and a port of `postcss-resolve-nested-selector` 0.1.6 (MIT), which
//! resolves a selector's `&` against the rules around it as text.

use super::Context;
use crate::scss::tree::Kind;

/// Whether a selector is plain for stylelint-scss (`isStandardSyntaxSelector` of its utils).
#[must_use]
pub fn is_standard_syntax_selector(selector: &str) -> bool {
    !(super::has_any_interpolation(selector)
        || selector.starts_with('%')
        || selector.contains(":extend")
        || selector.contains("<%")
        || selector.contains("%>")
        || super::is_standard_selector_less_call(selector))
}

/// Whether a selector is plain, a `%placeholder` without interpolation counting
/// (`isStandardSelector`).
#[must_use]
pub fn is_standard_selector(selector: &str) -> bool {
    is_standard_syntax_selector(selector)
        || (selector.starts_with('%') && !super::has_any_interpolation(selector))
}

/// Whether a rule is plain for stylelint-scss (`isStandardRule`, which imports its
/// `isStandardSelector`: a placeholder is plain).
#[must_use]
pub fn is_standard_rule(sheet: &Context, index: usize) -> bool {
    let node = sheet.node(index);
    let raw = sheet.selector(node);
    // `@a:` ends with `:` too, so the original's check of it is the last one's.
    is_standard_selector(&node.selector)
        && (!raw.ends_with(')') || raw.contains(':'))
        && !is_less_guard(raw)
        && !raw.ends_with(':')
}

/// Whether a selector holds a Less guard, `when (…)` (`/when\s+(?:not\s+)*\(/`).
fn is_less_guard(selector: &str) -> bool {
    selector.match_indices("when").any(|(at, _)| {
        let mut rest = &selector[at + 4..];
        if !rest.starts_with(char::is_whitespace) {
            return false;
        }
        loop {
            rest = rest.trim_start();
            match rest.strip_prefix("not") {
                Some(after) if after.starts_with(char::is_whitespace) => rest = after,
                _ => return rest.starts_with('('),
            }
        }
    })
}

/// Whether a `&` of a selector is written against something but a combinator or a space
/// (`hasInterpolatingAmpersand`).
#[must_use]
pub fn has_interpolating_ampersand(selector: &str) -> bool {
    let characters: Vec<char> = selector.chars().collect();
    let is_combinator = |c: char| c.is_whitespace() || matches!(c, '+' | '>' | '~');
    characters.iter().enumerate().any(|(at, &c)| {
        c == '&'
            && (at
                .checked_sub(1)
                .is_some_and(|before| !is_combinator(characters[before]))
                || characters
                    .get(at + 1)
                    .is_some_and(|after| !is_combinator(*after)))
    })
}

/// A text split at a separator outside quotes and brackets, inside parentheses too when
/// `functions` (`split` of `postcss-resolve-nested-selector`).
fn split(text: &str, separator: char, functions: bool) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut closing: Vec<char> = Vec::new();
    let mut quote: Option<char> = None;
    let mut escape = false;
    for letter in text.chars() {
        let mut cut = false;
        if escape {
            escape = false;
        } else if letter == '\\' {
            escape = true;
        } else if let Some(open) = quote {
            if letter == open {
                quote = None;
            }
        } else if letter == '"' || letter == '\'' {
            quote = Some(letter);
        } else if let Some(close) = match letter {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => None,
        } {
            closing.push(close);
        } else if closing.last() == Some(&letter) {
            closing.pop();
        } else if (closing.is_empty() || (functions && closing.iter().all(|each| *each == ')')))
            && letter == separator
        {
            cut = true;
        }
        if cut {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(letter);
        }
    }
    parts.push(current);
    parts
}

/// A selector resolved against the rules around a node (`resolveNestedSelector`).
#[must_use]
pub fn resolve(sheet: &Context, selector: &str, index: usize) -> Vec<String> {
    let Some(parent) = sheet.node(index).parent else {
        return vec![selector.to_owned()];
    };
    let outer = sheet.node(parent);
    if outer.is(Kind::Root) {
        return vec![selector.to_owned()];
    }
    let nest = outer.is(Kind::AtRule) && outer.name == "nest";
    if !outer.is(Kind::Rule) && !nest {
        return resolve(sheet, selector, parent);
    }
    let parents: Vec<String> = if nest {
        split(&outer.params, ',', false)
            .into_iter()
            .map(|each| each.trim().to_owned())
            .collect()
    } else {
        super::selectors::comma_list(&outer.selector)
    };
    let mut resolved = Vec::new();
    for parent_selector in parents {
        if selector.contains('&') {
            for outer_resolved in resolve(sheet, &parent_selector, parent) {
                resolved.push(split(selector, '&', true).join(&outer_resolved));
            }
        } else {
            resolved.extend(resolve(
                sheet,
                &format!("{parent_selector} {selector}"),
                parent,
            ));
        }
    }
    resolved
}

#[cfg(test)]
mod tests {
    use super::{has_interpolating_ampersand, is_less_guard, is_standard_selector, split};

    #[test]
    fn reads_selectors_as_stylelint_scss_does() {
        assert!(is_standard_selector("%a") && !is_standard_selector("%#{$a}"));
        assert!(has_interpolating_ampersand("&-a") && !has_interpolating_ampersand("a & b"));
        assert!(is_less_guard(".m() when not (@a)") && !is_less_guard("when"));
        assert_eq!(split("a&b(&)", '&', true), ["a", "b(", ")"]);
    }
}
