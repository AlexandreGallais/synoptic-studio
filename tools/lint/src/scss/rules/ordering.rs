//! What the two rules of `stylelint-order` 8.1.2 share (`utils/`, REF-STYLELINT-ORDER-SOURCE,
//! MIT, under ADR-0076): the blocks they check, every node holding children (rules, at-rules and
//! nested properties, the root left out, as `root.walk` with `isRuleWithNodes` finds them), and
//! what they call a property; and their fix, the port of `postcss-sorting` 10.0.0
//! (REF-POSTCSS-SORTING-SOURCE, MIT): a block's children written in a new order as `PostCSS`
//! prints them, each with the spaces before it and the comments that belong to it, its line ends
//! and its other text untouched. Our own guard (`changes_meaning`, ADR-0106) leaves a block as it
//! is when the new order would swap two nodes whose order the cascade or Sass reads: a longhand
//! and its shorthand of the same importance, a property and its prefixed form, a `$`-variable and
//! what reads it, a declaration and a nested block setting the same property, any node and an
//! `@include` or `@extend`. The block's reports stay, without a fix.

use super::Context;
use super::prefixed::prefix;
use super::reference::{
    LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, SHORTHAND_TO_RESET_TO_INITIAL_PROPERTY, get,
};
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

/// Whether a block may be sorted (`isAllowedToProcess` of postcss-sorting): not the block of a
/// Sass `@function`, `@if`, `@else`, `@for`, `@each` or `@while`, nor an empty one.
#[must_use]
pub fn is_sortable(node: &Node) -> bool {
    let control = ["function", "if", "else", "for", "each", "while"];
    let in_control = node.is(Kind::AtRule) && control.contains(&node.name.as_str());
    !in_control && !node.nodes().is_empty()
}

/// The comments a node takes with it when sorted: those on their own lines just before it, the
/// first of a block too when `first_alone` (`beforeNode`), or only those on their own lines
/// (`beforeDeclaration`); and those on its line just after it (`afterNode`,
/// `afterDeclaration`). Each in order, the nearest last before and first after.
#[must_use]
pub fn attached_comments(
    sheet: &Context,
    index: usize,
    first_alone: bool,
) -> (Vec<usize>, Vec<usize>) {
    let mut before = Vec::new();
    let mut current = sheet.sheet.previous(index);
    while let Some(previous) = current {
        let node = sheet.node(previous);
        let alone =
            node.before.contains('\n') || (first_alone && sheet.sheet.previous(previous).is_none());
        if !node.is(Kind::Comment) || node.before.is_empty() || !alone {
            break;
        }
        before.insert(0, previous);
        current = sheet.sheet.previous(previous);
    }
    let mut after = Vec::new();
    let mut current = sheet.sheet.next(index);
    while let Some(next) = current {
        let node = sheet.node(next);
        if !node.is(Kind::Comment) || node.before.is_empty() || node.before.contains('\n') {
            break;
        }
        after.push(next);
        current = sheet.sheet.next(next);
    }
    (before, after)
}

/// Whether a node ends with a `;` of its own the printer writes: a declaration or an at-rule
/// without a block.
fn takes_semicolon(node: &Node) -> bool {
    (node.is(Kind::Declaration) || node.is(Kind::AtRule)) && node.children.is_none()
}

/// The fix writing a block's children in a new order, as `PostCSS` prints them once
/// `removeAll` and `append` moved them: each with the text before it, a `;` after each
/// declaration and blockless at-rule but the last node that is not a comment, which keeps the
/// block's own (`semicolon`, always one when `semicolon` is forced). The range runs from the text
/// before the first child to the end of the last; `None` when the order changes nothing.
#[must_use]
pub fn rewrite(
    sheet: &Context,
    block: usize,
    order: &[usize],
    semicolon: bool,
) -> Option<(usize, usize, String)> {
    let children = sheet.node(block).nodes();
    if order == children {
        return None;
    }
    let (&first, &last) = (children.first()?, children.last()?);
    // The text before the first child, a hack's `_` or `*`, which `before` holds, left out.
    let node = sheet.node(first);
    let hack = usize::from(!sheet.text[..node.start].ends_with(node.before.as_str()));
    let start = node.start + hack - node.before.len();
    let leads: Vec<&str> = children
        .iter()
        .enumerate()
        .map(|(at, &child)| {
            let from = if at == 0 {
                start
            } else {
                sheet.node(children[at - 1]).end.unwrap_or(start)
            };
            &sheet.text[from..sheet.node(child).start]
        })
        .collect();
    let last_placed = order
        .iter()
        .rposition(|&child| !sheet.node(child).is(Kind::Comment))
        .unwrap_or(0);
    let keeps = semicolon || sheet.node(block).semicolon;
    let mut text = String::new();
    for (at, &child) in order.iter().enumerate() {
        let node = sheet.node(child);
        let lead = children
            .iter()
            .position(|&each| each == child)
            .map_or("", |each| leads[each]);
        let body = &sheet.text[node.start..node.end.unwrap_or(sheet.text.len())];
        text.push_str(lead);
        if takes_semicolon(node) {
            text.push_str(body.strip_suffix(';').unwrap_or(body));
            if at != last_placed || keeps {
                text.push(';');
            }
        } else {
            text.push_str(body);
        }
    }
    Some((
        start,
        sheet.node(last).end.unwrap_or(sheet.text.len()),
        text,
    ))
}

/// A list sorted in place by insertion, stable, that never fails on a comparison which is not a
/// total order, as postcss-sorting's comparisons are not always: `Array.prototype.sort`
/// tolerates them, `slice::sort_by` may panic.
pub fn insertion_sort<T>(items: &mut [T], compare: impl Fn(&T, &T) -> std::cmp::Ordering) {
    for at in 1..items.len() {
        let mut to = at;
        while to > 0 && compare(&items[to - 1], &items[to]) == std::cmp::Ordering::Greater {
            items.swap(to - 1, to);
            to -= 1;
        }
    }
}

/// Whether a property sets another, or the other it (`margin` and `margin-top`, `font` and
/// `font-kerning`): Stylelint's longhands and properties reset by a shorthand, followed down.
fn sets(shorthand: &str, longhand: &str, depth: usize) -> bool {
    let parts = get(LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, shorthand)
        .unwrap_or_default()
        .iter()
        .chain(get(SHORTHAND_TO_RESET_TO_INITIAL_PROPERTY, shorthand).unwrap_or_default());
    parts.clone().any(|&part| part == longhand)
        || (depth > 0
            && parts
                .into_iter()
                .any(|&part| sets(part, longhand, depth - 1)))
}

/// Whether two properties may set the same value: one name without its prefix, or one a
/// shorthand of the other; custom properties and `$`-variables by their exact names.
#[must_use]
pub fn related(one: &str, two: &str) -> bool {
    if one.starts_with("--")
        || two.starts_with("--")
        || one.starts_with('$')
        || two.starts_with('$')
    {
        return one == two;
    }
    let (one, two) = (unprefixed(one), unprefixed(two));
    one == two || sets(&one, &two, 3) || sets(&two, &one, 3)
}

/// Every node of a subtree, the node first.
fn subtree(sheet: &Context, index: usize) -> Vec<usize> {
    let mut found = vec![index];
    let mut at = 0;
    while at < found.len() {
        found.extend(sheet.node(found[at]).nodes());
        at += 1;
    }
    found
}

/// Whether a text reads a `$`-variable: its name not followed by a character of a name.
fn reads(text: &str, variable: &str) -> bool {
    text.match_indices(variable).any(|(at, _)| {
        !text[at + variable.len()..].starts_with(|character: char| {
            character.is_alphanumeric() || character == '-' || character == '_'
        })
    })
}

/// Whether two nodes of a block may not change places without changing what the sheet means
/// (our own guard; postcss-sorting has none): two declarations of related properties of the same
/// importance; a `$`-variable and a node reading it; a declaration and a nested block setting a
/// related property; two nested blocks setting related properties; any node and a block holding
/// an `@include`, `@extend`, `@content` or `@apply`, or nested properties, whose declarations are
/// not read here.
fn bound(sheet: &Context, one: usize, two: usize) -> bool {
    let (first, second) = (sheet.node(one), sheet.node(two));
    if first.is(Kind::Comment) || second.is(Kind::Comment) {
        return false;
    }
    for (variable, other) in [(first, second), (second, first)] {
        if variable.is(Kind::Declaration)
            && variable.prop.starts_with('$')
            && reads(sheet.source(other), &variable.prop)
        {
            return true;
        }
    }
    let opaque = |index: usize| {
        subtree(sheet, index).into_iter().any(|each| {
            let node = sheet.node(each);
            node.is_nested()
                || (node.is(Kind::AtRule)
                    && ["include", "extend", "content", "apply"].contains(&node.name.as_str()))
        })
    };
    if opaque(one) || opaque(two) {
        return true;
    }
    let plain = |node: &Node| node.is(Kind::Declaration) && node.children.is_none();
    if plain(first) && plain(second) {
        return first.important == second.important && related(&first.prop, &second.prop);
    }
    let declarations = |index: usize| -> Vec<usize> {
        subtree(sheet, index)
            .into_iter()
            .filter(|&each| plain(sheet.node(each)))
            .collect()
    };
    let theirs = declarations(two);
    declarations(one).iter().any(|&mine| {
        theirs
            .iter()
            .any(|&other| related(&sheet.node(mine).prop, &sheet.node(other).prop))
    })
}

/// Whether a new order of a block's children swaps two nodes that may not change places
/// (`bound`): then the block is left as it is, its reports kept without a fix.
#[must_use]
pub fn changes_meaning(sheet: &Context, block: usize, order: &[usize]) -> bool {
    let children = sheet.node(block).nodes();
    let rank = |child: usize| order.iter().position(|&each| each == child).unwrap_or(0);
    children.iter().enumerate().any(|(at, &one)| {
        children[at + 1..]
            .iter()
            .any(|&two| rank(two) < rank(one) && bound(sheet, one, two))
    })
}

#[cfg(test)]
mod tests {
    use super::{is_standard_property, related, unprefixed};
    use crate::lines::Lines;
    use crate::scss::parser::parse;
    use crate::scss::rules::order::Order;
    use crate::scss::rules::properties_order::PropertiesOrder;
    use crate::scss::rules::{Context, SheetRule};

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

    /// A sheet fixed by the two rules of the project, pass after pass as `lint --fix` runs, and
    /// the number of passes that changed it.
    fn fixed(code: &str) -> (String, usize) {
        let rules: [Box<dyn SheetRule>; 2] = [
            Box::new(Order::project()),
            Box::new(PropertiesOrder::project()),
        ];
        let mut text = code.to_owned();
        for pass in 0..crate::FIX_PASSES {
            let Ok(sheet) = parse(&text) else {
                return (text, pass);
            };
            let context = Context {
                path: "a.scss",
                text: &text,
                lines: Lines::new(&text),
                sheet: &sheet,
            };
            let mut reports = Vec::new();
            for rule in &rules {
                rule.check(&context, &mut reports);
            }
            let fixes: Vec<_> = reports
                .iter()
                .filter_map(|each| each.fix.as_ref())
                .collect();
            if fixes.is_empty() {
                return (text, pass);
            }
            text = crate::finding::apply(&text, &fixes);
        }
        (text, crate::FIX_PASSES)
    }

    #[test]
    fn f53_ac3_moves_each_comment_with_its_declaration_and_keeps_duplicates_in_order() {
        let code = "a {\n  /* why */\n  width: 1px; /* px */\n  display: block;\n  width: 2px\n}\n";

        // Own case: `display` (Recess's 19th) before `width` (74th). The comment on its own line
        // before the first `width` and the one on its line after it move with it; the two
        // `width`s keep their order; the block's last declaration still has no `;`.
        assert_eq!(
            fixed(code),
            (
                "a {\n  display: block;\n  /* why */\n  width: 1px; /* px */\n  width: 2px\n}\n"
                    .to_owned(),
                1
            )
        );
    }

    #[test]
    fn f53_ac3_keeps_the_line_ends_of_a_crlf_sheet() {
        let code = "a {\r\n  width: 1px;\r\n  position: relative;\r\n}\r\n";

        // Own case: `position` (3rd) before `width`, each line still ending in CR LF.
        assert_eq!(
            fixed(code),
            (
                "a {\r\n  position: relative;\r\n  width: 1px;\r\n}\r\n".to_owned(),
                1
            )
        );
    }

    #[test]
    fn f53_ac3_never_moves_a_longhand_after_the_shorthand_that_overrides_it() {
        let longhand_first = "a {\n  margin-top: 1px;\n  margin: 0;\n}\n";
        let important = "a {\n  margin-top: 1px !important;\n  margin: 0;\n}\n";

        // Own cases: `margin` comes before `margin-top` in Recess's order, but here it resets the
        // `margin-top` written before it: swapped, the top margin would be 1px instead of 0, so
        // the block is left as it is. An `!important` longhand wins whatever the order: swapped.
        assert_eq!(fixed(longhand_first), (longhand_first.to_owned(), 0));
        assert_eq!(
            fixed(important),
            (
                "a {\n  margin: 0;\n  margin-top: 1px !important;\n}\n".to_owned(),
                1
            )
        );
        assert!(related("-webkit-margin-top", "margin") && !related("margin", "padding"));
        assert!(related("font", "font-kerning") && related("border", "border-top-width"));
    }

    #[test]
    fn f53_ac3_never_moves_a_node_across_one_that_may_set_the_same_value() {
        // Own cases, each left as it is: a `$`-variable read before it is set; a property moved
        // across an `@include`, whose declarations are not read; a declaration moved before a
        // nested rule setting the same property for the same element.
        for code in [
            "a {\n  width: $c;\n  $c: 1px;\n}\n",
            "a {\n  width: 1px;\n  @include m;\n  position: relative;\n}\n",
            "a {\n  & {\n    position: static;\n  }\n\n  position: relative;\n}\n",
        ] {
            assert_eq!(fixed(code), (code.to_owned(), 0), "{code}");
        }
    }

    #[test]
    fn f53_ac3_sorts_a_sheet_until_stable_and_a_further_run_changes_nothing() {
        let code = "a {\n  width: 1px;\n  $c: 1px;\n  --d: 2px;\n  & b {\n    width: 1px;\n    top: 0;\n  }\n\n  opacity: 1;\n  position: absolute;\n}\n";

        // Own case. Pass 1, the outer block in the `order`: custom property, `$`-variable, the
        // declarations, the rule last, each node with the spaces before it, each declaration
        // ending with `;` (`properties-order`'s fix of the same range waits). Pass 2, its
        // properties in the Recess order, position 3rd, width 74th, opacity later, written where
        // the first was (the nested rule's fix, inside that range, waits). Pass 3, the nested
        // rule's: top before width. A further run finds nothing.
        let (once, passes) = fixed(code);
        assert_eq!(
            once,
            "a {\n  --d: 2px;\n  $c: 1px;\n  position: absolute;\n  width: 1px;\n\n  opacity: 1;\n  & b {\n    top: 0;\n    width: 1px;\n  }\n}\n"
        );
        assert_eq!(passes, 3);
        assert_eq!(fixed(&once), (once.clone(), 0));
    }
}
