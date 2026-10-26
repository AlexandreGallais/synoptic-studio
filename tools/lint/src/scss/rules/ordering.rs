//! What the two rules of `stylelint-order` 8.1.2 share (`utils/`, REF-STYLELINT-ORDER-SOURCE,
//! MIT, under ADR-0076): the blocks they check, every node holding children (rules, at-rules and
//! nested properties, the root left out, as `root.walk` with `isRuleWithNodes` finds them), and
//! what they call a property.

use super::Context;
use super::prefixed::prefix;
use crate::scss::tree::{Kind, Node};

/// Every node holding children, in the order written (`root.walk`, `isRuleWithNodes`).
#[must_use]
pub fn blocks(sheet: &Context) -> Vec<usize> {
    sheet
        .sheet
        .walk()
        .into_iter()
        .filter(|&index| !sheet.node(index).nodes().is_empty())
        .collect()
}

/// Whether a text holds `#{…}`, `@{…}` or `$(…)` on one line (`/#{.+?}|@{.+?}|\$\(.+?\)/`).
fn is_interpolated(text: &str) -> bool {
    [("#{", '}'), ("@{", '}'), ("$(", ')')]
        .iter()
        .any(|(opener, closer)| {
            text.match_indices(opener).any(|(at, _)| {
                let rest = &text[at + opener.len()..];
                let line = rest.split(['\n', '\r']).next().unwrap_or_default();
                line.char_indices()
                    .any(|(offset, character)| offset > 0 && character == *closer)
            })
        })
}

/// Whether a property is plain CSS: not a `$` or `@` variable, not interpolated
/// (`isStandardSyntaxProperty` of stylelint-order, narrower than Stylelint's).
#[must_use]
pub fn is_standard_property(property: &str) -> bool {
    !property.starts_with('$') && !property.starts_with('@') && !is_interpolated(property)
}

/// Whether a node is a CSS property's declaration: plain CSS, not a custom property
/// (`isProperty`).
#[must_use]
pub fn is_property(node: &Node) -> bool {
    node.is(Kind::Declaration) && is_standard_property(&node.prop) && !node.prop.starts_with("--")
}

/// A property's name without its vendor prefix, in lower case, `-moz-osx-` read as `-webkit-`
/// (`vendor.unprefixed`, then the hack of `getNodeData`).
#[must_use]
pub fn unprefixed(property: &str) -> String {
    let bare = property[prefix(property).len()..].to_lowercase();
    match bare.strip_prefix("osx-") {
        Some(rest) => rest.to_owned(),
        None => bare,
    }
}

#[cfg(test)]
mod tests {
    use super::{is_standard_property, unprefixed};

    #[test]
    fn reads_a_property_as_stylelint_order_does() {
        assert!(is_standard_property("color") && is_standard_property("--a"));
        // `#{}` holds nothing between its braces; a newline ends the search.
        assert!(is_standard_property("a#{}") && is_standard_property("a#{\n}"));
        for property in ["$a", "@a", "a-#{$b}", "@{a}-b", "a$(b)"] {
            assert!(!is_standard_property(property), "{property}");
        }
        assert_eq!(unprefixed("-WEBKIT-Box-Shadow"), "box-shadow");
        assert_eq!(unprefixed("-moz-osx-font-smoothing"), "font-smoothing");
        assert_eq!(unprefixed("color"), "color");
    }
}
