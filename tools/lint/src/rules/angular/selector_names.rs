//! The names of a component's or a directive's selector, after `@angular-eslint/utils` 22.6.0's
//! `selector-utils.ts`: the selector parsed as `@angular/compiler` 22.2.3's `CssSelector.parse`
//! reads it (an element, classes, attributes and `:not()` parts, a list split at its commas),
//! then its element or attribute names checked for a type, a prefix and a style.

use oxc_ast::ast::Expression;

/// One selector of a list, as `CssSelector` holds it.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct CssSelector {
    /// Its element, `app-root` in `app-root.a[b]`.
    pub element: Option<String>,
    /// Its classes, lower-cased.
    pub class_names: Vec<String>,
    /// Its attributes, each name followed by its lower-cased value, `''` when none.
    pub attrs: Vec<String>,
    /// The selectors of its `:not()` parts.
    pub not_selectors: Vec<CssSelector>,
}

/// Whether a character is one of the pattern's `\w`.
fn is_word(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

/// The length of the word, `[-\w]+`, at the start of a text.
fn word_length(text: &str) -> usize {
    text.find(|character: char| !(is_word(character) || character == '-'))
        .unwrap_or(text.len())
}

/// An attribute `[name]` or `[name=value]` at the start of a text: its name, its value and its
/// length (`\[([-.\w*\\$]+)(?:=(["']?)([^\]"']*)\5)?\]`).
fn attribute(text: &str) -> Option<(&str, &str, usize)> {
    let rest = text.strip_prefix('[')?;
    let name_length = rest
        .find(|character: char| !(is_word(character) || "-.*\\$".contains(character)))
        .unwrap_or(rest.len());
    if name_length == 0 {
        return None;
    }
    let name = &rest[..name_length];
    let after = &rest[name_length..];
    if after.starts_with(']') {
        return Some((name, "", name_length + 2));
    }
    let after = after.strip_prefix('=')?;
    let quote = after
        .chars()
        .next()
        .filter(|character| matches!(character, '"' | '\''));
    let value_start = quote.map_or(0, char::len_utf8);
    let value_text = &after[value_start..];
    let value_length = value_text
        .find([']', '"', '\''])
        .unwrap_or(value_text.len());
    let value = &value_text[..value_length];
    let mut end = &value_text[value_length..];
    if let Some(quote) = quote {
        end = end.strip_prefix(quote)?;
    }
    end.strip_prefix(']')?;
    let length =
        1 + name_length + 1 + value_start + value_length + quote.map_or(0, char::len_utf8) + 1;
    Some((name, value, length))
}

/// An attribute's name with its escapes removed (`unescapeAttribute`); none for an unescaped `$`,
/// which Angular refuses.
fn unescaped(name: &str) -> Option<String> {
    let mut result = String::new();
    let mut escaping = false;
    for character in name.chars() {
        if character == '\\' {
            escaping = true;
            continue;
        }
        if character == '$' && !escaping {
            return None;
        }
        escaping = false;
        result.push(character);
    }
    Some(result)
}

/// The selector being read: the outer one, or the `:not()` part last opened in it.
fn current(outer: &mut CssSelector, in_not: bool) -> &mut CssSelector {
    if in_not && !outer.not_selectors.is_empty() {
        let last = outer.not_selectors.len() - 1;
        return &mut outer.not_selectors[last];
    }
    outer
}

/// Adds a selector to the list, `*` as its element when it is only `:not()` parts.
fn add_result(results: &mut Vec<CssSelector>, mut selector: CssSelector) {
    if !selector.not_selectors.is_empty()
        && selector.element.is_none()
        && selector.class_names.is_empty()
        && selector.attrs.is_empty()
    {
        selector.element = Some("*".to_owned());
    }
    results.push(selector);
}

/// The selectors of a text (`CssSelector.parse`): the pattern's matches in order, every other
/// character skipped; none where Angular throws, a nested `:not()`, a list inside one, an
/// unescaped `$`.
#[must_use]
pub fn parse(text: &str) -> Option<Vec<CssSelector>> {
    let mut results = Vec::new();
    let mut selector = CssSelector::default();
    let mut in_not = false;
    let mut at = 0;
    while at < text.len() {
        let rest = &text[at..];
        if rest.starts_with(":not(") {
            if in_not {
                return None;
            }
            in_not = true;
            selector.not_selectors.push(CssSelector::default());
            at += 5;
            continue;
        }
        let prefix = rest
            .chars()
            .next()
            .filter(|character| matches!(character, '.' | '#'));
        let prefix_length = prefix.map_or(0, char::len_utf8);
        let tag_length = word_length(&rest[prefix_length..]);
        if tag_length > 0 {
            let tag = &rest[prefix_length..prefix_length + tag_length];
            let target = current(&mut selector, in_not);
            match prefix {
                Some('#') => target.attrs.extend(["id".to_owned(), tag.to_lowercase()]),
                Some(_) => target.class_names.push(tag.to_lowercase()),
                None => target.element = Some(tag.to_owned()),
            }
            at += prefix_length + tag_length;
            continue;
        }
        if let Some((name, value, length)) = attribute(rest) {
            let name = unescaped(name)?;
            current(&mut selector, in_not)
                .attrs
                .extend([name, value.to_lowercase()]);
            at += length;
            continue;
        }
        if rest.starts_with(')') {
            in_not = false;
            at += 1;
            continue;
        }
        let spaces = rest.len() - rest.trim_start().len();
        if rest[spaces..].starts_with(',') {
            if in_not {
                return None;
            }
            add_result(&mut results, std::mem::take(&mut selector));
            let after = &rest[spaces + 1..];
            at += spaces + 1 + after.len() - after.trim_start().len();
            continue;
        }
        at += rest.chars().next().map_or(1, char::len_utf8);
    }
    add_result(&mut results, selector);
    Some(results)
}

/// The selectors a value of the metadata holds (`parseSelectorNode`): a string's source text,
/// quotes included, or a template's first part.
#[must_use]
pub fn parse_value(raw: &str, value: &Expression) -> Option<Vec<CssSelector>> {
    match value {
        Expression::StringLiteral(_) => parse(raw),
        Expression::TemplateLiteral(template) => parse(&template.quasis.first()?.value.raw),
        _ => None,
    }
}

/// The kind of names a selector is checked on.
#[derive(Clone, Copy)]
pub enum SelectorType {
    /// An element, `app-root`.
    Element,
    /// An attribute, `[appTooltip]`.
    Attribute,
}

/// The style of a selector's names.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectorStyle {
    /// `app-root`.
    KebabCase,
    /// `appTooltip`.
    CamelCase,
}

/// What a rule asks of a selector.
pub struct SelectorConfig {
    /// The kind of names.
    pub selector_type: SelectorType,
    /// The prefixes, one of which starts a name.
    pub prefixes: &'static [&'static str],
    /// The style of a name.
    pub style: SelectorStyle,
}

/// What a selector misses, in the order the rules report it.
#[derive(Debug, PartialEq, Eq)]
pub enum Failure {
    /// No name of the type.
    Type,
    /// A prefix with nothing after it.
    SelectorAfterPrefix,
    /// A name of another style, its prefix missing too.
    StyleAndPrefix,
    /// A name of another style.
    Style,
    /// A name without one of the prefixes.
    Prefix,
}

/// Whether a name is kebab-case, `/^[a-z0-9]+(-[a-z0-9]+)*$/`.
fn is_kebab_case(name: &str) -> bool {
    !name.is_empty()
        && name.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        })
}

/// Whether a name is camelCase, `/^[a-zA-Z0-9[\]]+$/`.
fn is_camel_case(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '[' | ']'))
}

/// A name without a prefix it starts with, `^\[?(prefix)` removed; none when it does not.
#[must_use]
pub fn after_prefix<'n>(name: &'n str, prefix: &str) -> Option<&'n str> {
    name.strip_prefix('[')
        .unwrap_or(name)
        .strip_prefix(prefix)
        .or_else(|| name.strip_prefix(prefix))
}

/// Whether a name starts with a prefix followed by a break of its style
/// (`SelectorValidator.prefix`): an upper-case letter in camelCase, a hyphen in kebab-case.
#[must_use]
pub fn has_prefix(name: &str, prefix: &str, style: SelectorStyle) -> bool {
    let Some(after) = after_prefix(name, prefix) else {
        return false;
    };
    match (style, after.chars().next()) {
        (_, None) => true,
        (SelectorStyle::CamelCase, Some(first)) => first.to_uppercase().eq(std::iter::once(first)),
        (SelectorStyle::KebabCase, Some(first)) => first == '-',
    }
}

/// The names a list of selectors offers for a type (`getValidSelectors`): each element, or
/// each attribute's name and value.
fn names(selectors: &[CssSelector], selector_type: SelectorType) -> Vec<&str> {
    selectors
        .iter()
        .flat_map(|selector| match selector_type {
            SelectorType::Element => selector.element.iter().map(String::as_str).collect(),
            SelectorType::Attribute => selector
                .attrs
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        })
        .filter(|name| !name.is_empty() || matches!(selector_type, SelectorType::Attribute))
        .collect()
}

/// What a list of selectors misses of a configuration (`checkSelector` and the rules' order of
/// reports), none when it has all.
#[must_use]
pub fn failure(selectors: &[CssSelector], config: &SelectorConfig) -> Option<Failure> {
    let valid = names(selectors, config.selector_type);
    let style_check = match config.style {
        SelectorStyle::KebabCase => is_kebab_case,
        SelectorStyle::CamelCase => is_camel_case,
    };
    let any_prefix = |check: &dyn Fn(&str, &str) -> bool| {
        config.prefixes.is_empty()
            || valid
                .iter()
                .any(|name| config.prefixes.iter().any(|prefix| check(name, prefix)))
    };
    let has_expected_prefix = any_prefix(&|name, prefix| has_prefix(name, prefix, config.style));
    let has_selector_after_prefix =
        any_prefix(&|name, prefix| !after_prefix(name, prefix).unwrap_or(name).is_empty());
    let has_expected_style = valid.iter().any(|name| style_check(name));
    if valid.is_empty() {
        Some(Failure::Type)
    } else if !has_selector_after_prefix {
        Some(Failure::SelectorAfterPrefix)
    } else if !has_expected_style {
        Some(if has_expected_prefix {
            Failure::Style
        } else {
            Failure::StyleAndPrefix
        })
    } else if !has_expected_prefix {
        Some(Failure::Prefix)
    } else {
        None
    }
}

/// A list of names as the plugin writes it (`toHumanReadableText`): `"a"`, `"a" or "b"`,
/// `"a", "b" or "c"`.
#[must_use]
pub fn human_list(items: &[&str]) -> String {
    let quoted: Vec<String> = items.iter().map(|item| format!("\"{item}\"")).collect();
    match quoted.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, others)) => format!("{} or {last}", others.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CssSelector, Failure, SelectorConfig, SelectorStyle, SelectorType, failure, human_list,
        parse,
    };

    /// The failure of a selector's text under a configuration.
    fn check(
        text: &str,
        selector_type: SelectorType,
        prefixes: &'static [&'static str],
        style: SelectorStyle,
    ) -> Option<Failure> {
        let config = SelectorConfig {
            selector_type,
            prefixes,
            style,
        };
        failure(&parse(text).expect("a selector"), &config)
    }

    #[test]
    fn parses_selectors_as_angular_s_compiler_does() {
        // The quotes of a string's source text are skipped, as any character the pattern leaves.
        assert_eq!(
            parse("'app-root'"),
            Some(vec![CssSelector {
                element: Some("app-root".to_owned()),
                ..CssSelector::default()
            }])
        );
        let parsed = parse("app-foo[Baz=Qux].Bar:not(.b), [appX], #main").expect("a list");
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].element.as_deref(), Some("app-foo"));
        assert_eq!(parsed[0].attrs, ["Baz", "qux"]);
        assert_eq!(parsed[0].class_names, ["bar"]);
        assert_eq!(parsed[0].not_selectors[0].class_names, ["b"]);
        assert_eq!(parsed[1].attrs, ["appX", ""]);
        assert_eq!(parsed[2].attrs, ["id", "main"]);
        assert_eq!(
            parse(":not(.a)").expect("one")[0].element.as_deref(),
            Some("*")
        );
        assert_eq!(parse("[a=\"b c\"]").expect("one")[0].attrs, ["a", "b c"]);
        // Angular throws on a nested `:not()`, a list inside one and an unescaped `$`.
        assert_eq!(parse(":not(:not(a))"), None);
        assert_eq!(parse(":not(a, b)"), None);
        assert_eq!(parse("[a$]"), None);
        assert_eq!(parse("[a\\$]").expect("one")[0].attrs, ["a$", ""]);
    }

    #[test]
    fn checks_names_as_the_plugin_s_cases() {
        use SelectorStyle::{CamelCase, KebabCase};
        use SelectorType::{Attribute, Element};
        // component-selector's valid cases with one type.
        assert_eq!(check("'sg-foo-bar'", Element, &["sg"], KebabCase), None);
        assert_eq!(
            check("'[ng-foo-bar]'", Attribute, &["app", "ng"], KebabCase),
            None
        );
        assert_eq!(
            check(
                "'app-foo-bar[baz].app'",
                Element,
                &["app", "cd", "ng"],
                KebabCase
            ),
            None
        );
        assert_eq!(check("'appBarFoo'", Element, &["app"], CamelCase), None);
        assert_eq!(check("'app1-foo-bar'", Element, &["app1"], KebabCase), None);
        assert_eq!(
            check(
                "'baz-[app-bar-foo][foe].bar'",
                Attribute,
                &["app", "baz"],
                KebabCase
            ),
            None
        );
        assert_eq!(
            check(
                "[appFooBar], [appBarFoo]",
                Attribute,
                &["app", "ng"],
                CamelCase
            ),
            None
        );
        assert_eq!(
            check("button[appFooBar]", Attribute, &["app", "ng"], CamelCase),
            None
        );
        assert_eq!(check("'singleword'", Element, &[], KebabCase), None);
        // Its invalid cases.
        assert_eq!(
            check("'foo-bar'", Element, &["sg"], KebabCase),
            Some(Failure::Prefix)
        );
        assert_eq!(
            check("'app-foo-bar'", Element, &["sg"], KebabCase),
            Some(Failure::Prefix)
        );
        assert_eq!(
            check("'[app-foo-bar]'", Attribute, &["cd", "ng"], KebabCase),
            Some(Failure::Prefix)
        );
        assert_eq!(
            check("'[ng-bar-foo]'", Attribute, &["ng"], CamelCase),
            Some(Failure::Style)
        );
        assert_eq!(
            check("'appFooBar'", Element, &["app"], KebabCase),
            Some(Failure::StyleAndPrefix)
        );
        assert_eq!(
            check("'app'", Element, &["app"], KebabCase),
            Some(Failure::SelectorAfterPrefix)
        );
        assert_eq!(
            check("'app'", Element, &["app"], CamelCase),
            Some(Failure::SelectorAfterPrefix)
        );
        assert_eq!(
            check("'[appFooBar]'", Element, &["app", "ng"], CamelCase),
            Some(Failure::Type)
        );
        assert_eq!(
            check("app-foo-bar", Attribute, &["app", "ng"], KebabCase),
            Some(Failure::Type)
        );
        assert_eq!(
            check("'root'", Element, &["app", "toh"], KebabCase),
            Some(Failure::Prefix)
        );
        assert_eq!(
            check("'sgggg-bar'", Element, &["sg"], KebabCase),
            Some(Failure::Prefix)
        );
        assert_eq!(
            check("'[libFooBar]'", Attribute, &["app"], CamelCase),
            Some(Failure::Prefix)
        );
    }

    #[test]
    fn writes_lists_as_the_plugin() {
        assert_eq!(human_list(&["sg"]), "\"sg\"");
        assert_eq!(human_list(&["cd", "ng"]), "\"cd\" or \"ng\"");
        assert_eq!(
            human_list(&["foo", "cd", "ng"]),
            "\"foo\", \"cd\" or \"ng\""
        );
    }
}
