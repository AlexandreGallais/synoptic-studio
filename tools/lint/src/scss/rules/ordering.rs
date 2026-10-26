//! What the two rules of `stylelint-order` 8.1.2 share (`utils/`, REF-STYLELINT-ORDER-SOURCE,
//! MIT, under ADR-0076): the blocks they check, every node holding children (rules, at-rules and
//! nested properties, the root left out, as `root.walk` with `isRuleWithNodes` finds them), and
//! what they call a property; and their fix, the port of `postcss-sorting` 10.0.0
//! (REF-POSTCSS-SORTING-SOURCE, MIT): a block's children written in a new order as `PostCSS`
//! prints them, each with the spaces before it and the comments that belong to it, its line ends
//! and its other text untouched. Our own guard (`settle`, ADR-0106) reads what the new order
//! swaps: a declaration moved after one that overrides it had no effect and is dropped
//! (`margin-top: 0; margin: 8px` becomes `margin: 8px`); the block is left as it is when the new
//! order would swap two other nodes whose order the cascade or Sass reads: related properties of
//! the same importance (a longhand and its shorthand, a property and its prefixed form or legacy
//! alias, a physical and a flow-relative property, `all` and any other, an interpolated name and
//! any), a `$`-variable and what reads it, a declaration and a nested block setting a related
//! property of the same element, any node and an `@include` or `@extend`. The block's reports
//! stay, without a fix.

use super::Context;
use super::prefixed::prefix;
use super::reference::{
    FLOW_RELATIVE_SHORTHAND_PROPERTIES, LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES,
    PHYSICAL_TO_FLOW_RELATIVE_PROPERTIES, SHORTHAND_TO_RESET_TO_INITIAL_PROPERTY, get,
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

/// The property a legacy name alias stands for, else the property: `word-wrap` for
/// `overflow-wrap` (CSS Text 3 §5.4, REF-CSS-TEXT-3), the grid-prefixed gaps for the gaps (CSS
/// Gaps 1 §2.4, REF-CSS-GAPS-1).
fn canonical(property: &str) -> &str {
    match property {
        "word-wrap" => "overflow-wrap",
        "grid-gap" => "gap",
        "grid-row-gap" => "row-gap",
        "grid-column-gap" => "column-gap",
        _ => property,
    }
}

/// The longhands a property sets in the end, each by its name once aliases are read: itself when
/// it has none, else those of its parts, Stylelint's longhands and properties reset by a
/// shorthand, followed down (`margin` sets `margin-top`, `font` sets `font-kerning`).
fn longhands(property: &str, depth: usize) -> Vec<String> {
    let property = canonical(property);
    let parts: Vec<&str> = get(LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, property)
        .unwrap_or_default()
        .iter()
        .chain(get(SHORTHAND_TO_RESET_TO_INITIAL_PROPERTY, property).unwrap_or_default())
        .copied()
        .collect();
    if parts.is_empty() || depth == 0 {
        return vec![property.to_owned()];
    }
    parts
        .iter()
        .flat_map(|part| longhands(part, depth - 1))
        .collect()
}

/// The flow-relative family of a physical or flow-relative property: its flow-relative name with
/// its sides and axes read as one (`margin-*` for `margin-top` and `margin-inline-start`, `*-size`
/// for `width` and `block-size`), as the writing mode decides which sets which (Stylelint's
/// layout mappings, REF-STYLELINT-SOURCE); none for another property.
fn family(property: &str) -> Option<String> {
    let flow = PHYSICAL_TO_FLOW_RELATIVE_PROPERTIES
        .iter()
        .find_map(|&(physical, flow)| (physical == property || flow == property).then_some(flow))
        .or_else(|| {
            FLOW_RELATIVE_SHORTHAND_PROPERTIES
                .iter()
                .copied()
                .find(|&each| each == property)
        })?;
    let mut name = flow.to_owned();
    for side in [
        "inline-start",
        "inline-end",
        "block-start",
        "block-end",
        "start-start",
        "start-end",
        "end-start",
        "end-end",
        "inline",
        "block",
    ] {
        name = name.replace(side, "*");
    }
    Some(name)
}

/// The properties `all` leaves as they are (CSS Cascade 5 §3.2, REF-CSS-CASCADE-5), besides the
/// custom properties.
const NOT_RESET_BY_ALL: [&str; 2] = ["direction", "unicode-bidi"];

/// Whether two properties may set the same value: one name without its prefix, or two sharing a
/// longhand (a shorthand and its longhand, two shorthands of one longhand, a name and its legacy
/// alias), or a physical longhand and a flow-relative one of the same family (`margin-top` and
/// `margin-inline-start`, which sets it in a vertical writing mode), or `all` and any
/// property it resets; custom properties and `$`-variables by their exact names; an interpolated
/// name with every property, as what it sets is not read.
#[must_use]
pub fn related(one: &str, two: &str) -> bool {
    if is_interpolated(one) || is_interpolated(two) {
        return true;
    }
    let own = |name: &str| name.starts_with("--") || name.starts_with('$');
    if own(one) || own(two) {
        return one == two;
    }
    let (one, two) = (unprefixed(one), unprefixed(two));
    if one == "all" || two == "all" {
        let other = if one == "all" { &two } else { &one };
        return !NOT_RESET_BY_ALL.contains(&other.as_str());
    }
    let theirs = longhands(&two, 3);
    longhands(&one, 3).iter().any(|mine| {
        theirs.iter().any(|other| {
            mine == other
                || (is_physical(mine) != is_physical(other)
                    && family(mine).is_some_and(|name| family(other) == Some(name)))
        })
    })
}

/// Whether a property is a physical one of the layout mappings: two of them, or two
/// flow-relative ones, of one family name different sides in every writing mode.
fn is_physical(property: &str) -> bool {
    PHYSICAL_TO_FLOW_RELATIVE_PROPERTIES
        .iter()
        .any(|&(physical, _)| physical == property)
}

/// Whether a declaration has no effect because a later one of the same block sets everything it
/// sets, with the same importance and prefix and in every writing mode, from a value Sass always
/// writes (no `$`-variable nor interpolation, which may be `null` and leave the declaration out):
/// a longhand before its shorthand (`margin-top: 0; margin: 8px`), a name before its legacy
/// alias, any property `all` resets before `all`. A flow-relative property never overrides a
/// physical one here, nor the reverse: the writing mode decides.
fn overridden(sheet: &Context, earlier: &Node, later: &Node) -> bool {
    let plain = |node: &Node| is_property(node) && node.children.is_none();
    if !plain(earlier)
        || !plain(later)
        || earlier.important != later.important
        || prefix(&earlier.prop).to_lowercase() != prefix(&later.prop).to_lowercase()
    {
        return false;
    }
    let value = sheet.value(later);
    if value.contains('$') || value.contains("#{") {
        return false;
    }
    let (gone, kept) = (unprefixed(&earlier.prop), unprefixed(&later.prop));
    if kept == "all" {
        return !NOT_RESET_BY_ALL.contains(&gone.as_str());
    }
    let theirs = longhands(&kept, 3);
    longhands(&gone, 3).iter().all(|each| theirs.contains(each))
}

/// Whether a nested rule sets another element than its parent's: a selector without `&`, which
/// Sass writes as the parent's descendant, nor an interpolation, a `*` or a `:where(`, which may
/// add no specificity to the parent's.
fn leaves_the_element(node: &Node) -> bool {
    node.is(Kind::Rule)
        && !["&", "#{", "*", ":where("]
            .iter()
            .any(|each| node.selector.contains(each))
}

/// Every node of a subtree, the node first; when `own`, without the nested rules that set
/// another element (`leaves_the_element`) and what they hold.
fn subtree(sheet: &Context, index: usize, own: bool) -> Vec<usize> {
    let kept = |each: usize| !own || !leaves_the_element(sheet.node(each));
    let mut found: Vec<usize> = [index].into_iter().filter(|&each| kept(each)).collect();
    let mut at = 0;
    while at < found.len() {
        found.extend(
            sheet
                .node(found[at])
                .nodes()
                .iter()
                .copied()
                .filter(|&child| kept(child)),
        );
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
/// related property of the same element; two nested blocks setting related properties of the
/// same element; any node and a block holding an `@include`, `@extend`, `@content` or `@apply`,
/// or nested properties, whose declarations are not read here.
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
        subtree(sheet, index, false).into_iter().any(|each| {
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
        subtree(sheet, index, true)
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

/// A block's children in a new order as our own guard lets the fix write them (ADR-0106): a
/// declaration the new order moves after one that overrides it (`overridden`) is dropped, as it
/// had no effect; none when the new order swaps any other two nodes that may not change places
/// (`bound`), the block then left as it is, its reports kept without a fix.
#[must_use]
pub fn settle(sheet: &Context, block: usize, order: &[usize]) -> Option<Vec<usize>> {
    let children = sheet.node(block).nodes();
    let rank = |child: usize| order.iter().position(|&each| each == child).unwrap_or(0);
    let swapped: Vec<(usize, usize)> = children
        .iter()
        .enumerate()
        .flat_map(|(at, &one)| children[at + 1..].iter().map(move |&two| (one, two)))
        .filter(|&(one, two)| rank(two) < rank(one) && bound(sheet, one, two))
        .collect();
    let dropped: Vec<usize> = swapped
        .iter()
        .filter(|&&(one, two)| overridden(sheet, sheet.node(one), sheet.node(two)))
        .map(|&(one, _)| one)
        .collect();
    let refused = swapped
        .iter()
        .any(|(one, two)| !dropped.contains(one) && !dropped.contains(two));
    (!refused).then(|| {
        order
            .iter()
            .copied()
            .filter(|each| !dropped.contains(each))
            .collect()
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
        // `width`s keep their order; the block's last declaration takes a `;`, as postcss-sorting's
        // `sortNode` forces it and the formatter writes it (AUD-060).
        assert_eq!(
            fixed(code),
            (
                "a {\n  display: block;\n  /* why */\n  width: 1px; /* px */\n  width: 2px;\n}\n"
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
    fn f53_ac3_drops_a_longhand_written_before_the_shorthand_that_overrides_it() {
        // Own cases. `margin` comes before `margin-top` in Recess's order (146th, 153rd), and
        // here it resets the `margin-top` written before it, which has no effect: the fix drops
        // it, then orders the block. A `margin-top` after the shorthand stays, in order.
        for (code, expected) in [
            (
                "a {\n  margin-top: 0;\n  margin: 8px;\n}\n",
                "a {\n  margin: 8px;\n}\n",
            ),
            (
                "a {\n  margin-top: 0;\n  margin: 8px;\n  margin-top: 4px;\n}\n",
                "a {\n  margin: 8px;\n  margin-top: 4px;\n}\n",
            ),
            // A comment on its own line before the dropped longhand stays, after the shorthand.
            (
                "a {\n  /* top */\n  margin-top: 0;\n  margin: 8px;\n}\n",
                "a {\n  margin: 8px;\n  /* top */\n}\n",
            ),
            // `border-top` sets three longhands `border` sets (425th before 453rd).
            (
                "a {\n  border-top: 0;\n  border: 1px solid;\n}\n",
                "a {\n  border: 1px solid;\n}\n",
            ),
            // `font` (209th) sets `line-height` (238th).
            (
                "a {\n  line-height: 1;\n  font: 12px serif;\n}\n",
                "a {\n  font: 12px serif;\n}\n",
            ),
            // An `!important` longhand wins whatever the order: moved, kept.
            (
                "a {\n  margin-top: 1px !important;\n  margin: 0;\n}\n",
                "a {\n  margin: 0;\n  margin-top: 1px !important;\n}\n",
            ),
            // CR LF line ends kept.
            (
                "a {\r\n  margin-top: 0;\r\n  margin: 8px;\r\n}\r\n",
                "a {\r\n  margin: 8px;\r\n}\r\n",
            ),
        ] {
            assert_eq!(fixed(code), (expected.to_owned(), 1), "{code}");
            assert_eq!(fixed(expected), (expected.to_owned(), 0), "{expected}");
        }
        // A shorthand whose value Sass may leave out (`$m: null`) may not override: refused.
        let variable = "a {\n  margin-top: 0;\n  margin: $m;\n}\n";
        assert_eq!(fixed(variable), (variable.to_owned(), 0));
        assert!(related("-webkit-margin-top", "margin") && !related("margin", "padding"));
        assert!(related("font", "font-kerning") && related("border", "border-top-width"));
    }

    #[test]
    fn f53_ac3_drops_a_property_written_before_its_legacy_alias_or_all() {
        // Own cases: a legacy name alias sets the same property (`word-wrap` 277th before
        // `overflow-wrap` 278th, `grid-gap` 79th before `gap` 86th, `grid-row-gap` 80th before
        // `row-gap` 87th, `grid-column-gap` 81st before `column-gap` 88th), `all` (1st) resets
        // `color`: the property written before has no effect and is dropped. `all` leaves
        // `direction` as it is: it is only moved after `all`.
        for (code, expected) in [
            (
                "a {\n  overflow-wrap: anywhere;\n  word-wrap: normal;\n}\n",
                "a {\n  word-wrap: normal;\n}\n",
            ),
            (
                "a {\n  gap: 1px;\n  grid-gap: 2px;\n}\n",
                "a {\n  grid-gap: 2px;\n}\n",
            ),
            (
                "a {\n  row-gap: 1px;\n  grid-row-gap: 2px;\n}\n",
                "a {\n  grid-row-gap: 2px;\n}\n",
            ),
            (
                "a {\n  column-gap: 1px;\n  grid-column-gap: 2px;\n}\n",
                "a {\n  grid-column-gap: 2px;\n}\n",
            ),
            (
                "a {\n  color: red;\n  all: unset;\n}\n",
                "a {\n  all: unset;\n}\n",
            ),
            (
                "a {\n  direction: rtl;\n  all: unset;\n}\n",
                "a {\n  all: unset;\n  direction: rtl;\n}\n",
            ),
        ] {
            assert_eq!(fixed(code), (expected.to_owned(), 1), "{code}");
        }
    }

    #[test]
    fn f53_ac3_never_swaps_a_physical_and_a_flow_relative_property_nor_an_interpolated_one() {
        // Own cases, each left as it is: the flow-relative property comes first in Recess's
        // order (`inset-block-start` 22nd, `top` 27th; `margin-block-start` 148th, `margin-top`
        // 153rd; `inline-size` 113th, `width` 116th; `padding-inline` 139th, `padding-left`
        // 145th), but which physical side it sets depends on the writing mode: neither overrides
        // the other everywhere. An interpolated name may be any property.
        for code in [
            "a {\n  top: 1px;\n  inset-block-start: 0;\n}\n",
            "a {\n  margin-top: 1px;\n  margin-block-start: 0;\n}\n",
            "a {\n  width: 1px;\n  inline-size: 2px;\n}\n",
            "a {\n  padding-left: 1px;\n  padding-inline: 0;\n}\n",
            "a {\n  color: red;\n  #{$p}-top: 1px;\n  margin: 0;\n}\n",
        ] {
            assert_eq!(fixed(code), (code.to_owned(), 0), "{code}");
        }
        // Two physical sides never set the same value: sorted (142nd, 143rd).
        assert_eq!(
            fixed("a {\n  padding-right: 1px;\n  padding-top: 0;\n}\n"),
            (
                "a {\n  padding-top: 0;\n  padding-right: 1px;\n}\n".to_owned(),
                1
            )
        );
        assert!(related("margin-top", "margin-inline-start") && related("height", "inline-size"));
        assert!(!related("margin-top", "margin-left") && !related("width", "height"));
        assert!(related("border-color", "border-top") && related("#{$p}", "--a"));
    }

    #[test]
    fn f53_ac3_moves_a_declaration_across_a_nested_rule_setting_another_element() {
        // Own case: `.b` sets the `display` of `a .b`, not of `a`: the declaration goes before
        // the rule, each node with the text before it.
        assert_eq!(
            fixed("a {\n  .b {\n    display: none;\n  }\n\n  display: block;\n}\n"),
            (
                "a {\n\n  display: block;\n  .b {\n    display: none;\n  }\n}\n".to_owned(),
                1
            )
        );
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
            // A mixin's `@content` and nested properties hold declarations not read here.
            "@mixin m {\n  width: 1px;\n  @content;\n  position: relative;\n}\n",
            "a {\n  width: 1px;\n  font: 12px {\n    family: x;\n  }\n  position: relative;\n}\n",
        ] {
            assert_eq!(fixed(code), (code.to_owned(), 0), "{code}");
        }
    }

    #[test]
    fn f53_ac3_reads_a_variable_by_its_whole_name() {
        // Own cases: `$a_b` and `$a-b` are other variables than `$a`, which may then move
        // before the declaration, as `order` wants variables first.
        for (code, expected) in [
            (
                "a {\n  width: $a_b;\n  $a: 1px;\n}\n",
                "a {\n  $a: 1px;\n  width: $a_b;\n}\n",
            ),
            (
                "a {\n  width: $a-b;\n  $a: 1px;\n}\n",
                "a {\n  $a: 1px;\n  width: $a-b;\n}\n",
            ),
        ] {
            assert_eq!(fixed(code), (expected.to_owned(), 1), "{code}");
        }
        // A shorthand of a shorthand: `border` sets `border-color`, which sets
        // `border-left-color`.
        assert!(related("border", "border-left-color") && related("border-left-color", "border"));
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
