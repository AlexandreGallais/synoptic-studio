//! `order` of `stylelint-order` 8.1.2 (`rules/order/`, REF-STYLELINT-ORDER-SOURCE, MIT, under ADR-0076):
//! the content of each block in the order its option gives, by kind (custom properties,
//! `$`-variables, declarations, rules, at-rules) or by pattern (a rule by its selector, an at-rule
//! by its name, parameters and block), each node compared with the last one before it that has
//! a place, comments left out, reported over the node out of place. Its option `unspecified`
//! (`top`, `bottom`, `ignore`) is ported; Less's `@`-variables and mixins, which the SCSS reader
//! never gives, are not. The project's order, after the acceptance criterion: custom properties,
//! then `$`-variables, then declarations, then nested rules, the rest left where it is (F53.AC3).

use regex::Regex;

use super::ordering::{blocks, is_standard_property};
use super::{Context, Report, SheetRule};
use crate::scss::tree::{Kind, Node};

/// Where the nodes no item places go (`unspecified`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unspecified {
    /// Anywhere (`ignore`).
    Ignore,
    /// Before the others (`top`).
    Top,
    /// After the others (`bottom`).
    Bottom,
}

/// A regular expression an item's option gives, and how its description writes it: the string
/// as given, or `/…/flags`.
#[derive(Clone, Debug)]
pub struct Pattern {
    /// The expression, as `new RegExp(…)` reads it.
    pub regex: Regex,
    /// The option as a description prints it.
    pub text: String,
}

/// An item of the order.
#[derive(Clone, Debug)]
pub enum Item {
    /// `custom-properties`, `dollar-variables` or `declarations`.
    Keyword(&'static str),
    /// A rule, `rules` or `{ type: "rule", selector, name }`.
    Rule {
        /// The selector it must match.
        selector: Option<Pattern>,
        /// The name its description gives.
        name: Option<String>,
    },
    /// An at-rule, `at-rules` or `{ type: "at-rule", name, parameter, hasBlock }`.
    AtRule {
        /// Its name.
        name: Option<String>,
        /// The parameters it must match.
        parameter: Option<Pattern>,
        /// Whether it must, or must not, have a block.
        has_block: Option<bool>,
    },
}

/// The place an item gives, and how a message names it.
#[derive(Clone, Debug)]
struct Place {
    /// Its rank from 1, none for a node no item places.
    position: Option<usize>,
    /// What a message calls it, none where the original prints `undefined`.
    description: Option<String>,
}

/// The rule and its options.
pub struct Order {
    /// The items, in order.
    pub items: Vec<Item>,
    /// Where the nodes no item places go.
    pub unspecified: Unspecified,
}

impl Order {
    /// The project's configuration: custom properties, `$`-variables, declarations, rules.
    #[must_use]
    pub fn project() -> Self {
        Self {
            items: vec![
                Item::Keyword("custom-properties"),
                Item::Keyword("dollar-variables"),
                Item::Keyword("declarations"),
                Item::Rule {
                    selector: None,
                    name: None,
                },
            ],
            unspecified: Unspecified::Ignore,
        }
    }

    /// The description of an item (`getDescription`).
    fn describe(item: &Item) -> String {
        match item {
            Item::Keyword(keyword) => keyword_description(keyword).to_owned(),
            Item::Rule { selector, name } => match (name, selector) {
                (Some(name), _) => format!("rule \"{name}\""),
                (None, Some(selector)) => {
                    format!("rule with selector matching \"{}\"", selector.text)
                }
                (None, None) => "rule".to_owned(),
            },
            Item::AtRule {
                name,
                parameter,
                has_block,
            } => at_rule_description(
                name.as_deref(),
                parameter.as_ref().map(|each| each.text.as_str()),
                *has_block,
            ),
        }
    }

    /// The place of a node (`getOrderData`): the most specific pattern matching a rule or an
    /// at-rule, else its kind's item, else none and the description of its kind.
    fn place(&self, node: &Node) -> Place {
        let ranked = || {
            self.items
                .iter()
                .enumerate()
                .map(|(at, item)| (at + 1, item))
        };
        let keyword = match node.kind {
            Some(Kind::Declaration) if node.prop.starts_with("--") => Some("custom-properties"),
            Some(Kind::Declaration) if node.prop.starts_with('$') => Some("dollar-variables"),
            Some(Kind::Declaration) if is_standard_property(&node.prop) => Some("declarations"),
            _ => None,
        };
        if let Some(keyword) = keyword {
            // The last item of a keyword wins, as `order[item]` is written again.
            let position = ranked()
                .rfind(|(_, item)| matches!(item, Item::Keyword(each) if *each == keyword))
                .map(|(position, _)| position);
            return Place {
                position,
                description: Some(keyword_description(keyword).to_owned()),
            };
        }
        let mut best: Option<(usize, usize, &Item)> = None;
        for (position, item) in ranked() {
            let priority = match item {
                Item::Rule { selector, .. } if node.is(Kind::Rule) => match selector {
                    None => 1,
                    Some(pattern) if pattern.regex.is_match(&node.selector) => 2,
                    Some(_) => 0,
                },
                Item::AtRule {
                    name,
                    parameter,
                    has_block,
                } if node.is(Kind::AtRule) => {
                    at_rule_priority(node, name.as_deref(), parameter.as_ref(), *has_block)
                }
                _ => 0,
            };
            if priority > best.map_or(0, |(max, _, _)| max) {
                best = Some((priority, position, item));
            }
        }
        if let Some((_, position, item)) = best {
            return Place {
                position: Some(position),
                description: Some(Self::describe(item)),
            };
        }
        let description = match node.kind {
            Some(Kind::Rule) => Some(format!("rule with selector matching \"{}\"", node.selector)),
            Some(Kind::AtRule) => Some(at_rule_description(
                Some(&node.name),
                (!node.params.is_empty()).then_some(node.params.as_str()),
                Some(!node.nodes().is_empty()),
            )),
            _ => None,
        };
        Place {
            position: None,
            description,
        }
    }

    /// Whether a node with a place may come after another (`checkOrder`).
    const fn in_order(&self, first: Option<usize>, second: Option<usize>) -> bool {
        match (first, second) {
            (Some(first), Some(second)) => first <= second,
            (None, None) => true,
            (None, Some(_)) => !matches!(self.unspecified, Unspecified::Bottom),
            (Some(_), None) => !matches!(self.unspecified, Unspecified::Top),
        }
    }

    /// The reports of one block (`checkNode`).
    fn check_block(&self, sheet: &Context, block: usize, reports: &mut Vec<Report>) {
        let mut seen: Vec<Place> = Vec::new();
        for &child in sheet.node(block).nodes() {
            let node = sheet.node(child);
            if node.is(Kind::Comment) {
                continue;
            }
            let place = self.place(node);
            // The last node before with a place, else the one just before.
            let prior = seen
                .iter()
                .rev()
                .find(|each| each.position.is_some())
                .or_else(|| seen.last())
                .cloned();
            if let Some(prior) = prior
                && !self.in_order(prior.position, place.position)
            {
                let undefined = || "undefined".to_owned();
                let end = node.end.unwrap_or(sheet.text.len());
                reports.push(Report::new(node.start, end, "expected").with(&[
                    &place.description.clone().unwrap_or_else(undefined),
                    &prior.description.unwrap_or_else(undefined),
                ]));
            }
            seen.push(place);
        }
    }
}

/// What a message calls a keyword's nodes.
fn keyword_description(keyword: &str) -> &'static str {
    match keyword {
        "custom-properties" => "custom property",
        "dollar-variables" => "$-variable",
        _ => "declaration",
    }
}

/// What a message calls an at-rule item (`getDescription`).
fn at_rule_description(
    name: Option<&str>,
    parameter: Option<&str>,
    has_block: Option<bool>,
) -> String {
    let mut text = name.map_or_else(|| "at-rule".to_owned(), |name| format!("@{name}"));
    if let Some(parameter) = parameter {
        text.push_str(" \"");
        text.push_str(parameter);
        text.push('"');
    }
    match has_block {
        Some(true) => text.push_str(" with a block"),
        Some(false) => text = format!("blockless {text}"),
        None => {}
    }
    text
}

/// How well an at-rule pattern matches an at-rule (`calcAtRulePatternPriority`), 0 when it does
/// not: its block 10 010, its name 10 100, its parameters 11 100, a pattern of neither name nor
/// block 1; a pattern of several properties matching only some of them 0.
fn at_rule_priority(
    node: &Node,
    name: Option<&str>,
    parameter: Option<&Pattern>,
    has_block: Option<bool>,
) -> usize {
    let mut priority = 0;
    if has_block == Some(!node.nodes().is_empty()) {
        priority += 10_010;
    }
    if name == Some(node.name.as_str()) {
        priority += 10_100;
    }
    // `RegExp.test` of no parameters reads `"undefined"`.
    let params = if node.params.is_empty() {
        "undefined"
    } else {
        &node.params
    };
    if parameter.is_some_and(|pattern| pattern.regex.is_match(params)) {
        priority += 11_100;
    }
    // The original tests `paremeter`, which no pattern has: a pattern of parameters alone is 1.
    if has_block.is_none() && name.is_none() {
        priority = 1;
    }
    if has_block.is_some() && name.is_some() && priority < 20_000 {
        priority = 0;
    }
    if name.is_some() && parameter.is_some() && priority < 21_100 {
        priority = 0;
    }
    if name.is_some() && parameter.is_some() && has_block.is_some() && priority < 30_000 {
        priority = 0;
    }
    priority
}

impl SheetRule for Order {
    fn name(&self) -> &'static str {
        "order/order"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected {} to come before {}")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for block in blocks(sheet) {
            self.check_block(sheet, block, reports);
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::{Item, Order, Pattern, Unspecified};
    use crate::scss::rules::tester::{Js, Tally, primary, reports, run, secondary};

    /// A pattern from a string or a regular expression of a test's option.
    fn pattern(value: &Js) -> Result<Pattern, &'static str> {
        let (source, flags, text) = match value {
            Js::Text(text) => (text.as_str(), "", text.clone()),
            Js::Regex(source, flags) => (
                source.as_str(),
                flags.as_str(),
                format!("/{source}/{flags}"),
            ),
            _ => return Err("an option value not read"),
        };
        let prefix = if flags.contains('i') { "(?i)" } else { "" };
        Regex::new(&format!("{prefix}{source}"))
            .map(|regex| Pattern { regex, text })
            .map_err(|_| "a regular expression Rust cannot read")
    }

    /// The rule a test's configuration builds.
    fn build(config: &Js) -> Result<Order, &'static str> {
        let Js::List(entries) = primary(config) else {
            return Err("an option value not read");
        };
        let mut items = Vec::new();
        for entry in entries {
            let item = match entry {
                Js::Text(keyword) => match keyword.as_str() {
                    "custom-properties" => Item::Keyword("custom-properties"),
                    "dollar-variables" => Item::Keyword("dollar-variables"),
                    "declarations" => Item::Keyword("declarations"),
                    "rules" => Item::Rule {
                        selector: None,
                        name: None,
                    },
                    "at-rules" => Item::AtRule {
                        name: None,
                        parameter: None,
                        has_block: None,
                    },
                    _ => return Err("Less's @-variables and mixins"),
                },
                Js::Object(_) => match entry.get("type").and_then(Js::text) {
                    Some("rule") => Item::Rule {
                        selector: entry.get("selector").map(pattern).transpose()?,
                        name: entry.get("name").and_then(Js::text).map(str::to_owned),
                    },
                    Some("at-rule") => Item::AtRule {
                        name: entry.get("name").and_then(Js::text).map(str::to_owned),
                        parameter: entry.get("parameter").map(pattern).transpose()?,
                        has_block: match entry.get("hasBlock") {
                            Some(Js::Bool(value)) => Some(*value),
                            _ => None,
                        },
                    },
                    _ => return Err("an option value not read"),
                },
                _ => return Err("an option value not read"),
            };
            items.push(item);
        }
        let unspecified = match secondary(config).and_then(|options| options.get("unspecified")) {
            None => Unspecified::Ignore,
            Some(Js::Text(value)) if value == "ignore" => Unspecified::Ignore,
            Some(Js::Text(value)) if value == "top" => Unspecified::Top,
            Some(Js::Text(value)) if value == "bottom" => Unspecified::Bottom,
            Some(_) => return Err("an option value not read"),
        };
        Ok(Order { items, unspecified })
    }

    /// The cases of the rule's test file run.
    fn cases() -> Tally {
        run(
            "stylelint-order/order/index.txt",
            &|config| {
                build(config).map(|rule| Box::new(rule) as Box<dyn crate::scss::rules::SheetRule>)
            },
            &[],
        )
    }

    #[test]
    fn f53_ac3_wants_custom_properties_then_variables_then_declarations_then_rules() {
        let code = "a {\n  @include m;\n\n  & b {\n    color: red;\n  }\n\n  color: blue;\n  --c: 1px;\n  $d: 2px;\n  @media (width > 1px) {\n    color: green;\n  }\n}\n";
        let found = reports(&Order::project(), code);

        // Own case: `@include` and `@media` have no place and go anywhere; `color` after the
        // nested rule; `--c` after the declaration; `$d` after `--c`, the last placed node,
        // in order.
        assert_eq!(
            found,
            [
                (8, 3, "Expected declaration to come before rule".to_owned()),
                (
                    9,
                    3,
                    "Expected custom property to come before declaration".to_owned()
                ),
            ]
        );
        let ordered =
            "a {\n  --c: 1px;\n  $d: 2px;\n  color: blue;\n\n  & b {\n    color: red;\n  }\n}\n";
        assert_eq!(reports(&Order::project(), ordered), []);
    }

    #[test]
    fn f53_ac3_passes_the_cases_of_stylelint_order() {
        let tally = cases();
        // The fixes come with the reordering of a block (EN-168.T3).
        let (fixes, failures): (Vec<&String>, Vec<&String>) = tally
            .failures
            .iter()
            .partition(|each| each.starts_with("fix "));

        assert_eq!(failures, Vec::<&String>::new());
        assert_eq!(fixes.len(), 27);
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (31, 29, 16),
            "{:#?}",
            tally.apart
        );
    }
}
