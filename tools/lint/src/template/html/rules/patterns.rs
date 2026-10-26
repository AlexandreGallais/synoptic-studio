//! The patterns `class-pattern`, `id-pattern` and `name-pattern` of `html-validate` 11.17.0 match
//! a value against (`src/pattern.ts`, `src/rules/base-pattern-rule.ts`, REF-HTML-VALIDATE-SOURCE):
//! a named one (`kebabcase`, `camelcase`, `underscore` or `snakecase`, `bem`, `tailwind`) or a
//! regular expression written `/…/`, a value matching one of them.

use regex::Regex;

use super::super::elements::natural_join;

/// A pattern: its regular expression and how a message names it.
#[derive(Clone, Debug)]
pub struct Pattern {
    /// The regular expression.
    regex: Option<Regex>,
    /// Its description.
    description: String,
}

impl Pattern {
    /// A pattern read from its name or its `/…/` text (`parsePattern`).
    #[must_use]
    pub fn parse(pattern: &str) -> Self {
        let named = match pattern {
            "kebabcase" => Some(r"^[a-z][\da-z]*(?:-[\da-z]+)*$"),
            "camelcase" => Some(r"^[a-z][\dA-Za-z]*$"),
            "snakecase" | "underscore" => Some(r"^[a-z][\da-z]*(?:_[\da-z]+)*$"),
            "bem" => Some(
                r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*(?:__[a-z0-9]+(?:-[a-z0-9]+)*)?(?:--[a-z0-9]+(?:-[a-z0-9]+)*){0,2}$",
            ),
            "tailwind" => Some(r"^!?(?:[\[a-z-]|\d+xl:)[\w!#%&'(),./:=>\[\\\]-]*$"),
            _ => None,
        };
        if let Some(source) = named {
            return Self {
                regex: Regex::new(source).ok(),
                description: pattern.to_owned(),
            };
        }
        let inner = pattern
            .strip_prefix('/')
            .and_then(|rest| rest.strip_suffix('/'))
            .unwrap_or(pattern);
        Self {
            regex: Regex::new(inner).ok(),
            description: format!("/{inner}/"),
        }
    }
}

/// Why a value matches none of the patterns, `None` when it matches one (`validateValue`).
#[must_use]
pub fn mismatch(attr: &str, value: &str, patterns: &[Pattern]) -> Option<String> {
    if patterns.iter().any(|pattern| {
        pattern
            .regex
            .as_ref()
            .is_some_and(|regex| regex.is_match(value))
    }) {
        return None;
    }
    let names: Vec<String> = patterns
        .iter()
        .map(|pattern| format!("\"{}\"", pattern.description))
        .collect();
    Some(if patterns.len() == 1 {
        format!(
            "{attr} \"{value}\" does not match the configured pattern {}",
            natural_join(&names)
        )
    } else {
        format!(
            "{attr} \"{value}\" does not match either of the configured patterns: {}",
            natural_join(&names)
        )
    })
}

/// The patterns of a rule's `pattern` option, its default when `None`.
#[must_use]
pub fn compiled(patterns: Option<&[String]>, default: &str) -> Vec<Pattern> {
    patterns.map_or_else(
        || vec![Pattern::parse(default)],
        |patterns| patterns.iter().map(|each| Pattern::parse(each)).collect(),
    )
}

/// A `pattern` option of a test, a text or a list of them.
#[cfg(test)]
#[must_use]
pub fn of_option(option: Option<&crate::template::corpus::Value>) -> Option<Vec<String>> {
    use crate::template::corpus::Value;
    match option? {
        Value::Text(pattern) => Some(vec![pattern.clone()]),
        Value::List(list) => Some(list.iter().map(Value::display).collect()),
        _ => None,
    }
}
