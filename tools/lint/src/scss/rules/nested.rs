//! A rule's selectors resolved against the rules it is nested in, and normalized so that two
//! selectors meaning the same compare equal: a port of `resolveNestedSelector` of
//! `@csstools/selector-resolve-nested` 4.0.1 (MIT-0) as Stylelint 17.16.0's
//! `resolveNestedSelectorsForRule` calls it, and of its `normalizeSelector` (REF-STYLELINT-SOURCE,
//! MIT, under ADR-0076), on a tree of their own read from `scss::selector`'s parts: `&` replaced
//! by the parent (its compound inlined, a list as `:is(…)`), an implicit `& ` added, compounds
//! sorted by kind, lists sorted and deduplicated, comments and spaces dropped.

use crate::scss::selector::{Kind, Selectors};

/// An item of a selector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    /// A simple selector: its kind and its text (`a`, `.b`, `#c`, `[d]`, `*`).
    Simple(Kind, String),
    /// A pseudo-class or pseudo-element and its arguments.
    Pseudo(String, Option<Vec<Vec<Self>>>),
    /// A combinator, ` ` for a descendant.
    Combinator(String),
    /// `&`.
    Nesting,
}

/// A selector: its items.
pub type Selector = Vec<Item>;

impl Item {
    /// Whether it is a pseudo-element.
    fn is_element(&self) -> bool {
        matches!(self, Self::Pseudo(name, _) if name.starts_with("::") || matches!(name.to_lowercase().as_str(), ":before" | ":after" | ":first-line" | ":first-letter"))
    }

    /// Its text.
    #[must_use]
    pub fn print(&self) -> String {
        match self {
            Self::Simple(_, text) | Self::Combinator(text) => text.clone(),
            Self::Nesting => "&".to_owned(),
            Self::Pseudo(name, None) => name.clone(),
            Self::Pseudo(name, Some(list)) => format!("{name}({})", print_list(list)),
        }
    }
}

/// A selector's text.
#[must_use]
pub fn print(selector: &[Item]) -> String {
    selector.iter().map(Item::print).collect()
}

/// A list's text, its selectors joined by commas.
#[must_use]
pub fn print_list(list: &[Selector]) -> String {
    list.iter()
        .map(|each| print(each))
        .collect::<Vec<_>>()
        .join(",")
}

/// A list read from parts, comments dropped.
#[must_use]
pub fn read(parsed: &Selectors, text: &str, selectors: &[usize]) -> Vec<Selector> {
    selectors
        .iter()
        .map(|&selector| read_selector(parsed, text, selector))
        .collect()
}

/// A selector read from its parts.
fn read_selector(parsed: &Selectors, text: &str, selector: usize) -> Selector {
    parsed
        .part(selector)
        .nodes
        .iter()
        .filter_map(|&index| {
            let part = parsed.part(index);
            match part.kind {
                Kind::Comment => None,
                Kind::Nesting => Some(Item::Nesting),
                Kind::Combinator => Some(Item::Combinator(if part.value.trim().is_empty() {
                    " ".to_owned()
                } else {
                    part.value.trim().to_owned()
                })),
                Kind::Pseudo => Some(Item::Pseudo(
                    part.value.clone(),
                    (!part.nodes.is_empty()).then(|| read(parsed, text, &part.nodes)),
                )),
                // A class by its unescaped name, an attribute by its parts, so that the same
                // selector written otherwise compares equal (`normalizeNodeEscaping`, the
                // attribute's spaces dropped).
                Kind::Class => Some(Item::Simple(Kind::Class, format!(".{}", part.value))),
                Kind::Attribute => Some(Item::Simple(
                    Kind::Attribute,
                    part.attribute.as_ref().map_or_else(
                        || format!("[{}]", part.value.trim()),
                        |attribute| {
                            let flag = attribute
                                .flag
                                .as_ref()
                                .map(|flag| format!(" {flag}"))
                                .unwrap_or_default();
                            format!(
                                "[{}{}{}{flag}]",
                                attribute.name,
                                attribute.operator.as_deref().unwrap_or_default(),
                                attribute.value.as_deref().unwrap_or_default()
                            )
                        },
                    ),
                )),
                kind => Some(Item::Simple(
                    kind,
                    parsed.text(text, index).trim().to_owned(),
                )),
            }
        })
        .collect()
}

/// Whether a list is one compound selector, no combinator nor pseudo-element in it.
fn is_compound(list: &[Selector]) -> bool {
    matches!(list, [only] if !only.iter().any(|item| matches!(item, Item::Combinator(_)) || item.is_element()))
}

/// Whether a selector holds `&`, in its pseudos' arguments too.
fn has_nesting(selector: &[Item]) -> bool {
    selector.iter().any(|item| match item {
        Item::Nesting => true,
        Item::Pseudo(_, Some(list)) => list.iter().any(|each| has_nesting(each)),
        _ => false,
    })
}

/// The order of an item in a compound sorted after nesting is resolved.
fn resolve_order(item: &Item) -> u8 {
    match item {
        Item::Simple(Kind::Universal, _) => 0,
        Item::Simple(Kind::Tag, _) => 1,
        _ if item.is_element() => 2,
        Item::Nesting => 3,
        Item::Simple(Kind::Id, _) => 4,
        Item::Simple(Kind::Class, _) => 5,
        Item::Simple(Kind::Attribute, _) => 6,
        _ => 7,
    }
}

/// A complex selector's compounds sorted (`sortCompoundSelectorsInsideComplexSelector`): a second
/// `*` dropped, a second tag made `:is(tag)`.
fn sort_compounds(selector: Selector) -> Selector {
    let mut groups: Vec<Vec<Item>> = Vec::new();
    let mut current: Vec<Item> = Vec::new();
    for item in selector {
        if matches!(item, Item::Combinator(_)) {
            groups.push(std::mem::take(&mut current));
            groups.push(vec![item]);
        } else if item.is_element() {
            groups.push(std::mem::take(&mut current));
            current.push(item);
        } else if matches!(item, Item::Simple(Kind::Universal, _))
            && current
                .iter()
                .any(|each| matches!(each, Item::Simple(Kind::Universal, _)))
        {
        } else if let Item::Simple(Kind::Tag, _) = &item
            && current
                .iter()
                .any(|each| matches!(each, Item::Simple(Kind::Tag, _)))
        {
            current.push(Item::Pseudo(":is".to_owned(), Some(vec![vec![item]])));
        } else {
            current.push(item);
        }
    }
    groups.push(current);
    groups
        .into_iter()
        .flat_map(|mut group| {
            group.sort_by_key(resolve_order);
            group
        })
        .collect()
}

/// The parent's items replacing a `&`: its compound inlined, else `:is(…)`; always `:is(…)`
/// inside `:has()`.
fn replacement(parent: &[Selector], in_has: bool) -> Vec<Item> {
    if !in_has && is_compound(parent) {
        parent[0].clone()
    } else {
        vec![Item::Pseudo(":is".to_owned(), Some(parent.to_vec()))]
    }
}

/// A selector with each `&` replaced; whether it replaced one at its own level.
fn replace(selector: &[Item], parent: &[Selector], in_has: bool) -> (Selector, bool) {
    let mut replaced = false;
    let mut result = Vec::new();
    for item in selector {
        match item {
            Item::Nesting => {
                replaced = true;
                result.extend(replacement(parent, in_has));
            }
            Item::Pseudo(name, Some(list)) => {
                let has = name.eq_ignore_ascii_case(":has");
                let list = list
                    .iter()
                    .map(|each| {
                        let (inner, sorted) = replace(each, parent, has);
                        if sorted { sort_compounds(inner) } else { inner }
                    })
                    .collect();
                result.push(Item::Pseudo(name.clone(), Some(list)));
            }
            other => result.push(other.clone()),
        }
    }
    (result, replaced)
}

/// A list resolved against its parent's list (`resolveNestedSelector`).
#[must_use]
pub fn resolve(list: &[Selector], parent: &[Selector]) -> Vec<Selector> {
    list.iter()
        .map(|selector| {
            let mut selector = selector.clone();
            if !has_nesting(&selector) {
                selector.insert(0, Item::Combinator(" ".to_owned()));
                selector.insert(0, Item::Nesting);
            } else if matches!(selector.first(), Some(Item::Combinator(_))) {
                selector.insert(0, Item::Nesting);
            }
            let (replaced, own) = replace(&selector, parent, false);
            if own {
                sort_compounds(replaced)
            } else {
                replaced
            }
        })
        .collect()
}

/// The priority of an item when normalizing (`getSelectorTypePriority`).
fn normal_order(item: &Item) -> u8 {
    match item {
        Item::Simple(Kind::Universal, _) => 0,
        Item::Simple(Kind::Tag, _) => 1,
        Item::Nesting => 2,
        Item::Simple(Kind::Id, _) => 3,
        Item::Simple(Kind::Class, _) => 4,
        Item::Simple(Kind::Attribute, _) => 5,
        Item::Pseudo(..) => 6,
        _ => 7,
    }
}

/// A list normalized (`normalizeSelector`): each selector's compounds sorted, each pseudo's
/// arguments and the list sorted and deduplicated.
#[must_use]
pub fn normalize(list: &[Selector]) -> Vec<Selector> {
    let mut normalized: Vec<Selector> = list.iter().map(|each| normalize_selector(each)).collect();
    normalized.sort_by_key(|each| print(each));
    normalized.dedup_by_key(|each| print(each));
    normalized
}

/// A selector normalized.
fn normalize_selector(selector: &[Item]) -> Selector {
    let items: Vec<Item> = selector
        .iter()
        .map(|item| match item {
            Item::Pseudo(name, Some(list)) => Item::Pseudo(name.clone(), Some(normalize(list))),
            other => other.clone(),
        })
        .collect();
    let mut groups: Vec<Vec<Item>> = Vec::new();
    let mut current: Vec<Item> = Vec::new();
    for item in items {
        if matches!(item, Item::Combinator(_)) || item.is_element() {
            groups.push(std::mem::take(&mut current));
            groups.push(vec![item]);
        } else {
            current.push(item);
        }
    }
    groups.push(current);
    groups
        .into_iter()
        .flat_map(|mut group| {
            group.sort_by(|a, b| {
                normal_order(a)
                    .cmp(&normal_order(b))
                    .then_with(|| a.print().cmp(&b.print()))
            });
            group
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{normalize, print_list, read, resolve};
    use crate::scss::selector::parse;

    /// A list read from a text.
    fn list(text: &str) -> Vec<Vec<super::Item>> {
        let parsed = parse(text).unwrap_or_default();
        read(&parsed, text, &parsed.top)
    }

    #[test]
    fn resolves_and_normalizes_nested_selectors() {
        assert_eq!(
            print_list(&resolve(&list("&:hover, b"), &list("a"))),
            "a:hover,a b"
        );
        assert_eq!(
            print_list(&resolve(&list("& + &"), &list("a, b"))),
            ":is(a,b)+:is(a,b)"
        );
        assert_eq!(
            print_list(&resolve(&list("& > c"), &list(".a .b"))),
            ":is(.a .b)>c"
        );
        assert_eq!(
            print_list(&normalize(&list("[ a = \"b\" i ], [a=\"b\"i]"))),
            "[a=\"b\" i]"
        );
        // `b&`: the parent's tag second in the compound, made `:is(a)`.
        assert_eq!(print_list(&resolve(&list("b&"), &list("a"))), "b:is(a)");
        assert_eq!(print_list(&normalize(&list("b.c#d, a"))), "a,b#d.c");
        assert_eq!(
            print_list(&normalize(&list(":is(b, a) /* x */"))),
            ":is(a,b)"
        );
    }
}
