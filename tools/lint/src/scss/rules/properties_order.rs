//! `properties-order` of `stylelint-order` 8.1.2 (`rules/properties-order/`, REF-STYLELINT-ORDER-SOURCE,
//! MIT, under ADR-0076): the declarations of each block in the order of a list of properties and
//! groups, a property read without its vendor prefix, a prefixed one before the plain one, each
//! property compared with the one before it, or with the last placed one before an unplaced one;
//! and the empty lines between groups each group's `emptyLineBefore` asks or refuses. Its options
//! `unspecified` (`top`, `bottom`, `ignore`), `emptyLineBeforeUnspecified` and
//! `emptyLineMinimumPropertyThreshold`, and the groups' `emptyLineBefore`, `noEmptyLineBetween`,
//! `order: "flexible"` and `groupName` are ported; `unspecified: "bottomAlphabetical"`, which
//! needs the plugin's own table of shorthands, is not. The project's order: the groups of
//! stylelint-config-recess-order (`recess_order`), none asking for an empty line, so that one is
//! allowed between groups and nowhere asked (F53.AC3).

use std::collections::HashMap;

use super::empty_lines::{has_empty_line, report};
use super::ordering::{blocks, is_property, unprefixed};
use super::prefixed::prefix;
use super::recess_order::GROUPS;
use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// Where the properties no item places go (`unspecified`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unspecified {
    /// Anywhere (`ignore`).
    Ignore,
    /// Before the others (`top`).
    Top,
    /// After the others (`bottom`).
    Bottom,
}

/// The empty line a group, or the unplaced properties, want before them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyLine {
    /// One (`always`).
    Always,
    /// None (`never`).
    Never,
    /// One when the block holds at least the threshold's number of properties, else none
    /// (`threshold`).
    Threshold,
}

/// An item of the order: a property, or a group of items.
#[derive(Clone, Debug)]
pub enum Entry {
    /// A property, without its prefix.
    Property(String),
    /// A group.
    Group(Group),
}

/// A group of the order.
#[derive(Clone, Debug)]
pub struct Group {
    /// Its items, in order.
    pub properties: Vec<Entry>,
    /// The empty line it wants before it (`emptyLineBefore`).
    pub empty_line_before: Option<EmptyLine>,
    /// Whether it refuses empty lines between its properties (`noEmptyLineBetween`).
    pub no_empty_line_between: bool,
    /// Whether its properties come in any order (`order: "flexible"`).
    pub flexible: bool,
    /// The name its messages give (`groupName`).
    pub group_name: Option<String>,
}

impl Group {
    /// A group of properties alone, as stylelint-config-recess-order writes each.
    #[must_use]
    pub fn of(properties: &[&str]) -> Self {
        Self {
            properties: properties
                .iter()
                .map(|&each| Entry::Property(each.to_owned()))
                .collect(),
            empty_line_before: None,
            no_empty_line_between: false,
            flexible: false,
            group_name: None,
        }
    }
}

/// Where the order places a property (`createOrderInfo`).
#[derive(Clone, Debug)]
pub struct Place {
    /// The rank of the groups separated by an empty line, from 1.
    pub separated_group: usize,
    /// The rank of its group, the list itself 0.
    pub group_position: usize,
    /// Its rank, shared by a flexible group's properties.
    pub expected_position: usize,
    /// Its group's name.
    pub group_name: Option<String>,
    /// Whether its group refuses empty lines between its properties.
    pub no_empty_line_inside: bool,
}

/// The places of every property of an order, built as `createOrderInfo` walks it.
struct Builder {
    /// Each property's place, the last written of a name kept.
    places: HashMap<String, Place>,
    /// The rank given last.
    expected_position: usize,
    /// The separated group counted last.
    separated_group: usize,
    /// The group's rank given last.
    group_position: usize,
}

impl Builder {
    /// A group's items (`appendGroup`).
    fn group(&mut self, group: &Group, first: bool) {
        if !first {
            self.group_position += 1;
        }
        for entry in &group.properties {
            self.entry(entry, false, group);
        }
    }

    /// An item (`appendItem`).
    fn entry(&mut self, entry: &Entry, flexible: bool, group: &Group) {
        match entry {
            Entry::Property(property) => {
                if !flexible {
                    self.expected_position += 1;
                }
                self.places.insert(
                    property.clone(),
                    Place {
                        separated_group: self.separated_group,
                        group_position: self.group_position,
                        expected_position: self.expected_position,
                        group_name: group.group_name.clone(),
                        no_empty_line_inside: group.no_empty_line_between,
                    },
                );
            }
            Entry::Group(inner) => {
                if inner.empty_line_before.is_some() {
                    self.separated_group += 1;
                }
                if inner.flexible {
                    self.expected_position += 1;
                    self.group_position += 1;
                    for each in &inner.properties {
                        self.entry(each, true, inner);
                    }
                } else {
                    self.group(inner, false);
                }
            }
        }
    }
}

/// A declaration as the rule reads it (`getNodeData`).
#[derive(Clone, Debug)]
struct Property<'p> {
    /// The node.
    index: usize,
    /// Its name as written.
    name: &'p str,
    /// Its name without prefix, in lower case.
    bare: String,
    /// Its place, none when the order does not hold it.
    place: Option<&'p Place>,
}

/// The rule and its options.
pub struct PropertiesOrder {
    /// The order's items.
    pub order: Vec<Entry>,
    /// Where the unplaced properties go.
    pub unspecified: Unspecified,
    /// The empty line wanted before the first unplaced property after a placed one
    /// (`emptyLineBeforeUnspecified`).
    pub empty_line_before_unspecified: Option<EmptyLine>,
    /// The number of properties from which a group's `threshold` asks for an empty line
    /// (`emptyLineMinimumPropertyThreshold`).
    pub threshold: usize,
    /// The places of the order's properties.
    places: HashMap<String, Place>,
}

impl PropertiesOrder {
    /// The rule for an order and its options.
    #[must_use]
    pub fn new(
        order: Vec<Entry>,
        unspecified: Unspecified,
        empty_line_before_unspecified: Option<EmptyLine>,
        threshold: usize,
    ) -> Self {
        let mut builder = Builder {
            places: HashMap::new(),
            expected_position: 0,
            separated_group: 1,
            group_position: 0,
        };
        builder.group(
            &Group {
                properties: order.clone(),
                empty_line_before: None,
                no_empty_line_between: false,
                flexible: false,
                group_name: None,
            },
            true,
        );
        Self {
            order,
            unspecified,
            empty_line_before_unspecified,
            threshold,
            places: builder.places,
        }
    }

    /// The project's configuration: the Recess groups, the unplaced properties anywhere.
    #[must_use]
    pub fn project() -> Self {
        Self::new(
            GROUPS
                .iter()
                .map(|group| Entry::Group(Group::of(group)))
                .collect(),
            Unspecified::Ignore,
            None,
            0,
        )
    }

    /// The order's place of a property (`getNodeData`).
    fn property<'s>(&'s self, sheet: &'s Context, index: usize) -> Property<'s> {
        let name = sheet.node(index).prop.as_str();
        let bare = unprefixed(name);
        let place = self.places.get(&bare);
        Property {
            index,
            name,
            bare,
            place,
        }
    }

    /// The empty line the order's group of a separated rank wants (`groups[group - 2]`): the
    /// order's top-level groups counted, its lone properties left out.
    fn group_empty_line(&self, separated_group: usize) -> Option<EmptyLine> {
        self.order
            .iter()
            .filter_map(|entry| match entry {
                Entry::Group(group) => Some(group),
                Entry::Property(_) => None,
            })
            .nth(separated_group.checked_sub(2)?)
            .and_then(|group| group.empty_line_before)
    }

    /// Whether a property may come after another, and which pair a message names (`checkOrder`).
    fn in_order<'a>(
        &self,
        first: &'a Property<'a>,
        second: &'a Property<'a>,
        before: &'a [Property<'a>],
    ) -> Result<(), (&'a Property<'a>, &'a Property<'a>)> {
        if first.bare == second.bare {
            // A plain property before its prefixed form is out of order.
            let plain_then_prefixed =
                prefix(first.name).is_empty() && !prefix(second.name).is_empty();
            return if plain_then_prefixed {
                Err((first, second))
            } else {
                Ok(())
            };
        }
        match (first.place, second.place) {
            (Some(one), Some(two)) => {
                if one.expected_position <= two.expected_position {
                    Ok(())
                } else {
                    Err((first, second))
                }
            }
            (None, Some(two)) => {
                let prior = before[..before.len().saturating_sub(1)]
                    .iter()
                    .rev()
                    .find(|each| each.place.is_some());
                if let Some(prior) = prior
                    && prior
                        .place
                        .is_some_and(|place| place.expected_position > two.expected_position)
                {
                    return Err((prior, second));
                }
                if self.unspecified == Unspecified::Bottom {
                    Err((first, second))
                } else {
                    Ok(())
                }
            }
            (Some(_), None) if self.unspecified == Unspecified::Top => Err((first, second)),
            _ => Ok(()),
        }
    }

    /// The order's reports of a block (`checkNodeForOrder`).
    fn check_order(&self, sheet: &Context, block: usize, reports: &mut Vec<Report>) {
        let properties: Vec<Property> = sheet
            .node(block)
            .nodes()
            .iter()
            .filter(|&&child| is_property(sheet.node(child)))
            .map(|&child| self.property(sheet, child))
            .collect();
        for at in 1..properties.len() {
            if let Err((first, second)) =
                self.in_order(&properties[at - 1], &properties[at], &properties[..at])
            {
                let node = sheet.node(second.index);
                let mut args = vec![second.name, first.name];
                if let Some(group) = second.place.and_then(|place| place.group_name.as_deref()) {
                    args.push(group);
                }
                reports.push(
                    Report::new(node.start, node.end.unwrap_or(sheet.text.len()), "expected")
                        .with(&args),
                );
            }
        }
    }

    /// Whether a declaration has an empty line before it, or before the comment just before it
    /// (`hasEmptyLineBefore`).
    fn has_empty_line_before(sheet: &Context, index: usize) -> bool {
        has_empty_line(&sheet.node(index).before)
            || sheet.sheet.previous(index).is_some_and(|previous| {
                let node = sheet.node(previous);
                node.is(Kind::Comment) && has_empty_line(&node.before)
            })
    }

    /// The empty-line reports of a block (`checkNodeForEmptyLines`).
    fn check_empty_lines(&self, sheet: &Context, block: usize, reports: &mut Vec<Report>) {
        let children = sheet.node(block).nodes();
        let count = children
            .iter()
            .filter(|&&child| is_property(sheet.node(child)))
            .count();
        for at in 1..children.len() {
            let mut previous = Some(at - 1);
            // A comment on the line of the node before stands for that node.
            let shared = |index: usize| {
                let node = sheet.node(children[index]);
                node.is(Kind::Comment) && !node.before.contains('\n')
            };
            if shared(at - 1) {
                previous = at.checked_sub(2);
            }
            let Some(previous) = previous else {
                continue;
            };
            let (first, second) = (children[previous], children[at]);
            if !is_property(sheet.node(first)) || !is_property(sheet.node(second)) {
                continue;
            }
            self.check_pair(
                sheet,
                &self.property(sheet, first),
                &self.property(sheet, second),
                count,
                reports,
            );
        }
        let first = children[0];
        if is_property(sheet.node(first)) {
            let wanted = match self.property(sheet, first).place {
                Some(place) => self.group_empty_line(place.separated_group).is_some(),
                None => self.empty_line_before_unspecified.is_some(),
            };
            if wanted && Self::has_empty_line_before(sheet, first) {
                let name = sheet.node(first).prop.clone();
                reports.push(report(sheet, first, "rejectedEmptyLineBefore", false).with(&[&name]));
            }
        }
    }

    /// The empty-line reports of a property after another (`checkEmptyLineBefore`).
    fn check_pair(
        &self,
        sheet: &Context,
        first: &Property,
        second: &Property,
        count: usize,
        reports: &mut Vec<Report>,
    ) {
        // The separated group of an unplaced property is the first: the original never moves it.
        let first_group = first.place.map_or(1, |place| place.separated_group);
        let second_group = second.place.map_or(1, |place| place.separated_group);
        let starts_placed = second.place.is_some() && first_group != second_group;
        let starts_unplaced = first.place.is_some() && second.place.is_none();
        let has_empty = Self::has_empty_line_before(sheet, second.index);
        if starts_placed || starts_unplaced {
            let wanted = if starts_unplaced {
                self.empty_line_before_unspecified
            } else {
                self.group_empty_line(second_group)
            };
            let below = count < self.threshold;
            let add = wanted == Some(EmptyLine::Always)
                || (wanted == Some(EmptyLine::Threshold) && !below);
            let remove =
                wanted == Some(EmptyLine::Never) || (wanted == Some(EmptyLine::Threshold) && below);
            if add && !has_empty {
                reports.push(
                    report(sheet, second.index, "expectedEmptyLineBefore", true)
                        .with(&[second.name]),
                );
            } else if remove && has_empty {
                reports.push(
                    report(sheet, second.index, "rejectedEmptyLineBefore", false)
                        .with(&[second.name]),
                );
            }
        }
        if let (Some(one), Some(two)) = (first.place, second.place)
            && one.group_position == two.group_position
            && two.no_empty_line_inside
            && has_empty
        {
            reports.push(
                report(sheet, second.index, "rejectedEmptyLineBefore", false).with(&[second.name]),
            );
        }
    }
}

impl SheetRule for PropertiesOrder {
    fn name(&self) -> &'static str {
        "order/properties-order"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected \"{}\" to come before \"{}\""),
            (
                "expectedEmptyLineBefore",
                "Expected an empty line before property \"{}\"",
            ),
            (
                "rejectedEmptyLineBefore",
                "Unexpected empty line before property \"{}\"",
            ),
        ]
    }

    fn computed(&self, key: &str, args: &[String]) -> Option<String> {
        let group = args
            .get(2)
            .filter(|group| !group.is_empty() && *group != "undefined")?;
        (key == "expected").then(|| {
            format!(
                "Expected \"{}\" to come before \"{}\" in group \"{group}\"",
                args[0], args[1]
            )
        })
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for block in blocks(sheet) {
            self.check_order(sheet, block, reports);
            self.check_empty_lines(sheet, block, reports);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EmptyLine, Entry, Group, PropertiesOrder, Unspecified};
    use crate::scss::rules::SheetRule;
    use crate::scss::rules::tester::{Js, primary, reports, run, secondary};

    /// The empty line an option's string asks.
    fn empty_line(value: Option<&Js>) -> Result<Option<EmptyLine>, &'static str> {
        match value {
            None | Some(Js::Undefined) => Ok(None),
            Some(Js::Text(text)) => match text.as_str() {
                "always" => Ok(Some(EmptyLine::Always)),
                "never" => Ok(Some(EmptyLine::Never)),
                "threshold" => Ok(Some(EmptyLine::Threshold)),
                _ => Err("an option value not read"),
            },
            Some(_) => Err("an option value not read"),
        }
    }

    /// The items of a test's order.
    fn entries(items: &[Js]) -> Result<Vec<Entry>, &'static str> {
        items
            .iter()
            .map(|item| match item {
                Js::Text(property) => Ok(Entry::Property(property.clone())),
                Js::Object(_) => {
                    let Some(Js::List(properties)) = item.get("properties") else {
                        return Err("an option value not read");
                    };
                    Ok(Entry::Group(Group {
                        properties: entries(properties)?,
                        empty_line_before: empty_line(item.get("emptyLineBefore"))?,
                        no_empty_line_between: item.get("noEmptyLineBetween")
                            == Some(&Js::Bool(true)),
                        flexible: item.get("order").and_then(Js::text) == Some("flexible"),
                        group_name: item.get("groupName").and_then(Js::text).map(str::to_owned),
                    }))
                }
                _ => Err("an option value not read"),
            })
            .collect()
    }

    /// The rule a test's configuration builds.
    fn build(config: &Js) -> Result<Box<dyn SheetRule>, &'static str> {
        let Js::List(items) = primary(config) else {
            return Err("an option value not read");
        };
        let options = secondary(config);
        let unspecified = match options.and_then(|each| each.get("unspecified")) {
            None => Unspecified::Ignore,
            Some(Js::Text(value)) => match value.as_str() {
                "ignore" => Unspecified::Ignore,
                "top" => Unspecified::Top,
                "bottom" => Unspecified::Bottom,
                _ => return Err("an option needing the plugin's shorthands: bottomAlphabetical"),
            },
            Some(_) => return Err("an option value not read"),
        };
        let threshold = match options.and_then(|each| each.get("emptyLineMinimumPropertyThreshold"))
        {
            Some(Js::Number(number)) => format!("{number}")
                .parse()
                .map_err(|_| "an option value not read")?,
            _ => 0,
        };
        Ok(Box::new(PropertiesOrder::new(
            entries(items)?,
            unspecified,
            empty_line(options.and_then(|each| each.get("emptyLineBeforeUnspecified")))?,
            threshold,
        )))
    }

    /// The accepted, rejected and set apart cases of a test file, and its fix failures, once
    /// every other case passes.
    fn counts(file: &str) -> (usize, usize, usize, usize) {
        let tally = run(
            &format!("stylelint-order/properties-order/{file}.txt"),
            &build,
            &[],
        );
        // The fixes come with the reordering of a block (EN-168.T3).
        let (fixes, failures): (Vec<&String>, Vec<&String>) = tally
            .failures
            .iter()
            .partition(|each| each.starts_with("fix "));
        assert_eq!(failures, Vec::<&String>::new(), "{file}");
        (
            tally.accepted,
            tally.rejected,
            tally.apart_count(),
            fixes.len(),
        )
    }

    #[test]
    fn f53_ac3_wants_the_recess_order_an_empty_line_allowed_between_groups() {
        let code = "a {\n  color: red;\n  display: block;\n\n  position: relative;\n  -webkit-box-shadow: none;\n  box-shadow: none;\n  unknown-thing: 1;\n  width: 1px;\n}\n\nb {\n  transition: none;\n  -webkit-transition: none;\n}\n";
        let found = reports(&PropertiesOrder::project(), code);

        // Own cases, the Recess ranks of groups.js: position 3rd, display 19th, width 74th,
        // color 179th, box-shadow 402nd. `display` after `color`, `position` after `display`
        // (its empty line before allowed, no group asking for one or refusing it); the prefixed
        // `box-shadow` before the plain one in order, `unknown-thing` anywhere; `width` after
        // the unplaced property is compared with `box-shadow`, the last placed before it; a
        // plain `transition` before its prefixed form out of order.
        assert_eq!(
            found,
            [
                (
                    3,
                    3,
                    "Expected \"display\" to come before \"color\"".to_owned()
                ),
                (
                    5,
                    3,
                    "Expected \"position\" to come before \"display\"".to_owned()
                ),
                (
                    9,
                    3,
                    "Expected \"width\" to come before \"box-shadow\"".to_owned()
                ),
                (
                    14,
                    3,
                    "Expected \"-webkit-transition\" to come before \"transition\"".to_owned()
                ),
            ]
        );
        let ordered =
            "a {\n  position: relative;\n\n  display: block;\n  width: 1px;\n  color: red;\n}\n";
        assert_eq!(reports(&PropertiesOrder::project(), ordered), []);
    }

    #[test]
    fn f53_ac3_passes_the_cases_of_stylelint_order() {
        // (accepted, rejected, set apart, fixes awaiting the reordering) of each file.
        // Set apart in `flat`: 17 cases of CSS-in-JS, HTML or Less, 5 of `bottomAlphabetical`,
        // 1 of a `stylelint-disable` comment; in `no-empty-line-between`, 2 of CSS-in-JS.
        assert_eq!(counts("flat"), (30, 17, 23, 17));
        assert_eq!(counts("grouped-flexible"), (17, 9, 0, 7));
        assert_eq!(counts("grouped-strict"), (7, 12, 0, 10));
        assert_eq!(counts("empty-line-before"), (53, 33, 0, 1));
        assert_eq!(counts("empty-line-before-unspecified"), (18, 7, 0, 0));
        assert_eq!(
            counts("empty-line-minimum-property-threshold"),
            (28, 19, 0, 1)
        );
        assert_eq!(counts("no-empty-line-between"), (7, 5, 2, 0));
    }
}
