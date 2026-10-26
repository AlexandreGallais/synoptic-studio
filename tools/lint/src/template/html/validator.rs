//! The checks of an element against its metadata, a port of `Validator` (`src/meta/validator.ts`
//! of `html-validate` 11.17.0, REF-HTML-VALIDATE-SOURCE): an element matched against a content
//! category or a selector, a list of permitted entries, the occurrences and the order of a
//! parent's children, the required ancestors and content, and an attribute's value.

use regex::Regex;

use super::elements::{AttrMeta, Entry, Enum, ValueForm};
use super::view::{Doc, Value};

/// A permitted entry without its quantifier, and its limit: `?` once, `*` or none unlimited.
fn quantified(entry: &str) -> (&str, Option<usize>) {
    match entry.strip_suffix('?') {
        Some(category) => (category, Some(1)),
        None => (entry.strip_suffix('*').unwrap_or(entry), None),
    }
}

/// Whether an element matches a category (`@flow`) or, otherwise, a selector; an element
/// without metadata matches a category as asked (`validatePermittedCategory`).
#[must_use]
pub fn matches_category(doc: &Doc, index: usize, category: &str, default_match: bool) -> bool {
    let (category, _) = quantified(category);
    if !category.starts_with('@') {
        return doc.matches(index, category);
    }
    let Some(meta) = doc.el(index).meta else {
        return default_match;
    };
    match category {
        "@meta" => meta.metadata.of(doc, index),
        "@flow" => meta.flow.of(doc, index),
        "@sectioning" => meta.sectioning.of(doc, index),
        "@heading" => meta.heading.of(doc, index),
        "@phrasing" => meta.phrasing.of(doc, index),
        "@embedded" => meta.embedded.of(doc, index),
        "@interactive" => meta.interactive.of(doc, index),
        "@script" => meta.script_supporting.of(doc, index),
        "@form" => meta.form.of(doc, index),
        _ => false,
    }
}

/// Whether an element passes an entry (`validatePermittedRule`).
fn passes(doc: &Doc, index: usize, entry: &Entry, is_exclude: bool) -> bool {
    match entry {
        Entry::Item(category) => matches_category(doc, index, category, !is_exclude),
        Entry::Exclude(excluded) => excluded
            .iter()
            .all(|each| !matches_category(doc, index, each, false)),
    }
}

/// Whether an element passes one of the permitted entries, `None` permitting all
/// (`validatePermitted`).
#[must_use]
pub fn permitted(doc: &Doc, index: usize, rules: Option<&[Entry]>) -> bool {
    rules.is_none_or(|rules| rules.iter().any(|entry| passes(doc, index, entry, false)))
}

/// The children above the limit of their entry, each with the entry's category
/// (`validateOccurrences`).
#[must_use]
pub fn over_limit(doc: &Doc, children: &[usize], rules: &[Entry]) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for entry in rules {
        let Entry::Item(rule) = entry else {
            return found;
        };
        let (category, limit) = quantified(rule);
        let Some(limit) = limit.filter(|_| !category.is_empty()) else {
            continue;
        };
        let siblings: Vec<usize> = children
            .iter()
            .copied()
            .filter(|child| matches_category(doc, *child, rule, true))
            .collect();
        for child in siblings.into_iter().skip(limit) {
            found.push((child, category.to_owned()));
        }
    }
    found
}

/// The first child out of order and the child before it (`validateOrder`).
#[must_use]
pub fn out_of_order(doc: &Doc, children: &[usize], rules: &[&str]) -> Option<(usize, usize)> {
    let mut at = 0;
    let mut previous = None;
    for child in children {
        let old = at;
        while rules
            .get(at)
            .is_some_and(|rule| !matches_category(doc, *child, rule, true))
        {
            at += 1;
        }
        if at >= rules.len() {
            let specified = rules
                .iter()
                .any(|rule| matches_category(doc, *child, rule, true));
            if specified {
                return previous.map(|previous| (*child, previous));
            }
            at = old;
        }
        previous = Some(*child);
    }
    None
}

/// Whether an element has one of the required ancestors, selectors matched from the element
/// itself (`validateAncestors`).
#[must_use]
pub fn has_ancestors(doc: &Doc, index: usize, rules: &[&str]) -> bool {
    rules.is_empty() || rules.iter().any(|rule| doc.closest(index, rule).is_some())
}

/// The required children an element lacks (`validateRequiredContent`).
#[must_use]
pub fn missing_content<'r>(doc: &Doc, index: usize, rules: &[&'r str]) -> Vec<&'r str> {
    let children = doc.child_elements(index);
    rules
        .iter()
        .filter(|rule| {
            !children
                .iter()
                .any(|child| matches_category(doc, *child, rule, false))
        })
        .copied()
        .collect()
}

/// Whether a value matches an enumerated value: a word without case, a pattern as written.
#[must_use]
pub fn matches_enum(value: &str, entry: &Enum) -> bool {
    match entry {
        Enum::Word(word) => value.to_lowercase() == *word,
        Enum::Pattern(_, pattern) => Regex::new(pattern).is_ok_and(|regex| regex.is_match(value)),
    }
}

/// Whether a value is one of an attribute's enumerated values, any when it has none
/// (`validateAttributeValue`).
fn valid_value(value: Option<&str>, rule: &AttrMeta) -> bool {
    if rule.values.is_empty() {
        return true;
    }
    value.is_some_and(|value| rule.values.iter().any(|entry| matches_enum(value, entry)))
}

/// Whether an attribute's value is allowed by its metadata (`validateAttribute`): a dynamic
/// value always is; a boolean one when omitted, empty or its own name; an omissible one when
/// omitted or empty; a list's every token.
#[must_use]
pub fn valid_attribute(key: &str, value: &Value, rule: &AttrMeta) -> bool {
    let text = match value {
        Value::Dynamic => return true,
        Value::Omitted => None,
        Value::Static(text) => Some(text.as_str()),
    };
    let empty = text.is_none_or(str::is_empty);
    if rule.form == ValueForm::Boolean {
        return empty || text == Some(key);
    }
    if rule.form == ValueForm::Omissible && empty {
        return true;
    }
    if rule.list {
        return text
            .unwrap_or_default()
            .split_ascii_whitespace()
            .all(|token| valid_value(Some(token), rule));
    }
    valid_value(text, rule)
}
