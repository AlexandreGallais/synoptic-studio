//! `declaration-block-no-redundant-longhand-properties` of Stylelint 17.16.0
//! (`lib/rules/declaration-block-no-redundant-longhand-properties/index.mjs`,
//! REF-STYLELINT-SOURCE): in a block, every longhand of a shorthand written (prefixes matched,
//! `!important` alike), reported from the first, or the last when they are apart, to the last,
//! and fixed by one shorthand where its value can be built; a longhand repeating the value of its
//! shorthand written before it, reported over it and fixed by removing it. Its options leave out
//! shorthands and longhands; the project sets none.

use super::prefixed::prefix;
use super::reference::{BASIC_KEYWORDS, LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, get, has};
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::javascript::JsText;
use crate::scss::tree::Node;
use crate::scss::value::{self, Kind as Part};

/// The rule and its options.
pub struct DeclarationBlockNoRedundantLonghandProperties {
    /// The shorthands left out (`ignoreShorthands`).
    pub ignore_shorthands: Vec<Matcher>,
    /// The longhands left out (`ignoreLonghands`).
    pub ignore_longhands: Vec<String>,
}

impl DeclarationBlockNoRedundantLonghandProperties {
    /// The project's configuration: nothing left out.
    pub const PROJECT: Self = Self {
        ignore_shorthands: Vec::new(),
        ignore_longhands: Vec::new(),
    };
}

/// The shorthands whose fix removes longhands other shorthands collect.
const OVERLAPPING: &[&str] = &[
    "border-width",
    "border-style",
    "border-color",
    "border-top",
    "border-right",
    "border-bottom",
    "border-left",
    "grid-column",
    "grid-row",
];

/// A value's comma-separated items, trimmed (`commaSeparated`).
fn comma_separated(input: Option<&str>) -> Vec<String> {
    let Some(trimmed) = input.map(str::trim).filter(|each| !each.is_empty()) else {
        return Vec::new();
    };
    if !trimmed.contains(',') {
        return vec![trimmed.to_owned()];
    }
    let parsed = value::parse(trimmed);
    let mut parts: Vec<String> = vec![String::new()];
    for &at in &parsed.nodes {
        let part = parsed.part(at);
        if part.kind == Part::Div && part.value == "," {
            parts.push(String::new());
        } else if let Some(last) = parts.last_mut() {
            last.push_str(&trimmed[part.start..part.end]);
        }
    }
    parts
        .into_iter()
        .map(|each| each.js_trim().to_owned())
        .filter(|each| !each.is_empty())
        .collect()
}

/// `grid-template` built from its areas, columns and rows, none with `repeat()` or areas and
/// rows not paired.
fn grid_template(get: &dyn Fn(&str) -> Option<String>) -> Option<String> {
    let (areas, columns, rows) = (
        get("grid-template-areas")?,
        get("grid-template-columns")?,
        get("grid-template-rows")?,
    );
    if columns.contains("repeat(") || rows.contains("repeat(") {
        return None;
    }
    let split_areas: Vec<&str> = areas
        .split('"')
        .skip(1)
        .step_by(2)
        .filter(|each| !each.is_empty())
        .collect();
    let split_rows: Vec<&str> = rows.split(' ').collect();
    if split_areas.is_empty() || split_areas.len() != split_rows.len() {
        return None;
    }
    let zipped: Vec<String> = split_areas
        .iter()
        .zip(&split_rows)
        .map(|(area, row)| format!("\"{area}\" {row}"))
        .collect();
    Some(format!("{} / {columns}", zipped.join(" ")))
}

/// `transition` built from its four lists, each property with the durations, timings and
/// delays repeated as CSS repeats them.
fn transition(values: &dyn Fn(&str) -> Option<String>) -> Option<String> {
    let delays = comma_separated(values("transition-delay").as_deref());
    let durations = comma_separated(values("transition-duration").as_deref());
    let timings = comma_separated(values("transition-timing-function").as_deref());
    let properties = comma_separated(values("transition-property").as_deref());
    if delays.is_empty() || durations.is_empty() || timings.is_empty() || properties.is_empty() {
        return None;
    }
    Some(
        properties
            .iter()
            .enumerate()
            .map(|(at, property)| {
                [
                    property,
                    &durations[at % durations.len()],
                    &timings[at % timings.len()],
                    &delays[at % delays.len()],
                ]
                .iter()
                .map(|each| each.as_str())
                .collect::<Vec<_>>()
                .join(" ")
            })
            .collect::<Vec<_>>()
            .join(", "),
    )
}

/// The value of a shorthand built from its longhands (`resolveShorthandValue` and its custom
/// resolvers), none when it cannot be built.
fn resolve(
    shorthand: &str,
    longhands: &[String],
    values: &dyn Fn(&str) -> Option<String>,
) -> Option<String> {
    let get = |name: &str| values(name).filter(|each| !each.is_empty());
    match shorthand {
        "font" => {
            let parts: Option<Vec<String>> = [
                "font-style",
                "font-variant",
                "font-weight",
                "font-stretch",
                "font-size",
                "line-height",
                "font-family",
            ]
            .iter()
            .map(|name| get(name))
            .collect();
            let parts = parts?;
            Some(format!(
                "{} {} {} {} {}/{} {}",
                parts[0], parts[1], parts[2], parts[3], parts[4], parts[5], parts[6]
            ))
        }
        "font-synthesis" => {
            let read = [
                ("font-synthesis-weight", "weight"),
                ("font-synthesis-style", "style"),
                ("font-synthesis-small-caps", "small-caps"),
            ];
            let mut auto = Vec::new();
            for (name, word) in read {
                match values(name).as_deref() {
                    Some("auto") => auto.push(word),
                    Some("none") => {}
                    _ => return None,
                }
            }
            Some(if auto.is_empty() {
                "none".to_owned()
            } else {
                auto.join(" ")
            })
        }
        "grid-column" | "grid-row" => Some(format!(
            "{} / {}",
            get(&format!("{shorthand}-start"))?,
            get(&format!("{shorthand}-end"))?
        )),
        "grid-template" => grid_template(&get),
        "transition" => transition(values),
        _ => {
            let bare = &shorthand[prefix(shorthand).len()..];
            if bare == "background"
                && values(&format!("{}background-size", prefix(shorthand))).is_some()
            {
                return None;
            }
            Some(
                longhands
                    .iter()
                    .filter_map(|name| get(name))
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        }
    }
}

impl DeclarationBlockNoRedundantLonghandProperties {
    /// The shorthands each longhand belongs to, the options' left out, in the map's order.
    fn shorthands_of(&self, longhand: &str) -> Vec<&'static str> {
        LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES
            .iter()
            .filter(|(shorthand, longhands)| {
                !matches_any(&self.ignore_shorthands, shorthand)
                    && longhands.contains(&longhand)
                    && !self.ignore_longhands.iter().any(|each| each == longhand)
            })
            .map(|(shorthand, _)| *shorthand)
            .collect()
    }

    /// The longhands a shorthand wants, in its order, when a group holds them all, each
    /// `!important` or none.
    fn complete(
        &self,
        sheet: &Context,
        group: &[usize],
        shorthand: &str,
        vendor: &str,
    ) -> Option<Vec<String>> {
        let order: Vec<String> = get(LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, shorthand)
            .unwrap_or_default()
            .iter()
            .filter(|each| !self.ignore_longhands.iter().any(|ignored| ignored == *each))
            .map(|each| format!("{vendor}{each}"))
            .collect();
        let mut wanted = order.clone();
        let mut collected: Vec<String> = group
            .iter()
            .map(|&each| sheet.node(each).prop.to_lowercase())
            .collect();
        wanted.sort();
        collected.sort();
        let important = group
            .iter()
            .filter(|&&each| sheet.node(each).important)
            .count();
        (wanted == collected && (important == 0 || important == group.len())).then_some(order)
    }

    /// The fix replacing the first longhand by the shorthand and removing the others.
    fn fix(
        sheet: &Context,
        group: &[usize],
        shorthand: &str,
        value: &str,
    ) -> Option<(usize, usize, String)> {
        let first = sheet.node(group[0]);
        let parent = sheet.node(first.parent?);
        let children = parent.nodes();
        let from = children.iter().position(|each| *each == group[0])?;
        let to = children
            .iter()
            .position(|each| Some(each) == group.last())?;
        let end_of = |node: &Node| node.end.unwrap_or(sheet.text.len());
        let mut text = String::new();
        for (at, &index) in children[from..=to].iter().enumerate() {
            let node = sheet.node(index);
            if at == 0 {
                let important = if node.important {
                    node.important_raw
                        .clone()
                        .unwrap_or_else(|| " !important".to_owned())
                } else {
                    String::new()
                };
                let semicolon = if sheet.source(node).ends_with(';') {
                    ";"
                } else {
                    ""
                };
                text.push_str(shorthand);
                text.push_str(&node.between);
                text.push_str(value);
                text.push_str(&important);
                text.push_str(semicolon);
            } else if !group.contains(&index) {
                text.push_str(&node.before);
                text.push_str(sheet.source(node));
            }
        }
        if !sheet.source(sheet.node(children[to])).ends_with(';') && text.ends_with(';') {
            text.pop();
        }
        Some((first.start, end_of(sheet.node(children[to])), text))
    }
}

impl SheetRule for DeclarationBlockNoRedundantLonghandProperties {
    fn name(&self) -> &'static str {
        "stylelint/declaration-block-no-redundant-longhand-properties"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected shorthand property \"{}\""),
            (
                "unexpectedLonghand",
                "Redundant longhand property \"{}\" after shorthand property \"{}\"",
            ),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for block in sheet.declaration_blocks() {
            let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
            let mut declared: Vec<(String, String)> = Vec::new();
            for &index in &block {
                let node = sheet.node(index);
                let value = node.value.js_trim().to_owned();
                if has(BASIC_KEYWORDS, &value) {
                    continue;
                }
                let prop = node.prop.to_lowercase();
                let vendor = prefix(&prop).to_owned();
                let bare = prop[vendor.len()..].to_owned();
                if get(LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, &bare).is_some() {
                    declared.retain(|(key, _)| *key != prop);
                    declared.push((prop.clone(), value.clone()));
                }
                let parent = sheet.node(node.parent.unwrap_or(0));
                let scope = (parent.start, parent.end.unwrap_or(sheet.text.len()));
                for shorthand in self.shorthands_of(&bare) {
                    let prefixed = format!("{vendor}{shorthand}");
                    if declared
                        .iter()
                        .any(|(key, each)| *key == prefixed && *each == value)
                    {
                        let (from, to) = sheet.word_range(node, &node.prop);
                        let (start, end) = sheet.removal(node);
                        reports.push(
                            Report::new(from, to, "unexpectedLonghand")
                                .with(&[&node.prop, &prefixed])
                                .fixed(start, end, "")
                                .scoped(scope.0, scope.1),
                        );
                        break;
                    }
                    let at = groups
                        .iter()
                        .position(|(key, _)| *key == prefixed)
                        .unwrap_or_else(|| {
                            groups.push((prefixed.clone(), Vec::new()));
                            groups.len() - 1
                        });
                    groups[at].1.push(index);
                    let group = groups[at].1.clone();
                    let Some(order) = self.complete(sheet, &group, shorthand, &vendor) else {
                        continue;
                    };
                    let values = |name: &str| {
                        group
                            .iter()
                            .map(|&each| sheet.node(each))
                            .find(|each| each.prop.to_lowercase() == name)
                            .map(|each| each.value.js_trim().to_owned())
                    };
                    let fix = resolve(&prefixed, &order, &values)
                        .and_then(|resolved| Self::fix(sheet, &group, &prefixed, &resolved));
                    if fix.is_some() && OVERLAPPING.contains(&shorthand) {
                        for (_, others) in &mut groups {
                            others.retain(|each| !group.contains(each));
                        }
                    }
                    let first = group[0];
                    let last = *group.last().unwrap_or(&first);
                    let siblings = parent.nodes();
                    let span = siblings.iter().position(|each| *each == last).unwrap_or(0) + 1
                        - siblings.iter().position(|each| *each == first).unwrap_or(0);
                    let start = if span == group.len() {
                        sheet.node(first).start
                    } else {
                        sheet.node(last).start
                    };
                    let report = Report::new(
                        start,
                        sheet.node(last).end.unwrap_or(sheet.text.len()),
                        "expected",
                    )
                    .with(&[&prefixed])
                    .scoped(scope.0, scope.1);
                    reports.push(match fix {
                        Some((from, to, text)) => report.fixed(from, to, text),
                        None => report,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeclarationBlockNoRedundantLonghandProperties, comma_separated};
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/declaration-block-no-redundant-longhand-properties.txt",
            &|config| {
                let options = secondary(config);
                Ok(Box::new(DeclarationBlockNoRedundantLonghandProperties {
                    ignore_shorthands: matchers(options, "ignoreShorthands")?,
                    ignore_longhands: options
                        .and_then(|each| each.texts("ignoreLonghands"))
                        .unwrap_or_default(),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (26, 71, 9),
            "{:#?}",
            tally.apart
        );
        assert_eq!(comma_separated(Some(" a, b(c, d) ")), ["a", "b(c, d)"]);
    }
}
