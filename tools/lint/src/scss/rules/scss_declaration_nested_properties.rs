//! `declaration-nested-properties` of stylelint-scss 7.3.0
//! (`src/rules/declaration-nested-properties/index.js` and `src/utils/parseNestedPropRoot.js`,
//! REF-STYLELINT-SOURCE): with `"never"` (the project's), each nested property
//! (`font: { family: x }`, `margin: 0 { left: 1px }`) reported over its name; with `"always"`, a
//! property of a namespace (`font-family`) written whole, reported over it, and with
//! `except: "only-of-namespace"` a namespace of one property nested.

use super::{Context, Report, SheetRule, is_standard_property};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct DeclarationNestedProperties {
    /// Whether nested properties are wanted (`"always"`), else refused (`"never"`).
    pub always: bool,
    /// Whether a namespace of one property is written whole (`except: "only-of-namespace"`).
    pub except_only_of_namespace: bool,
}

impl DeclarationNestedProperties {
    /// The project's configuration: `"never"`.
    pub const PROJECT: Self = Self {
        always: false,
        except_only_of_namespace: false,
    };
}

/// The property a nested rule's selector names, `margin:` or `margin: 0` (`parseNestedPropRoot`).
fn nested_name(selector: &str) -> Option<String> {
    let mut stack: Vec<char> = vec!['n'];
    let characters: Vec<char> = selector.chars().collect();
    for (at, &character) in characters.iter().enumerate() {
        let previous = at.checked_sub(1).map(|before| characters[before]);
        if character == '"' || character == '\'' {
            let top = *stack.last().unwrap_or(&'n');
            if top != '"' && top != '\'' {
                stack.push(character);
            } else if top == character && previous != Some('\\') {
                stack.pop();
            }
            continue;
        }
        if character == '{' {
            stack.push('{');
            continue;
        }
        if character == '}' {
            stack.pop();
            continue;
        }
        if stack.last() != Some(&'n') || character != ':' || previous == Some('\\') {
            continue;
        }
        let name: String = characters[..at].iter().collect();
        let value: String = characters[at + 1..].iter().collect();
        if !value.is_empty() {
            let spaces = value.len() - value.trim_start().len();
            let trimmed = value.trim();
            let starts_value = trimmed
                .starts_with(|c: char| c.is_ascii_digit() || matches!(c, '.' | '$' | '\'' | '"'));
            if spaces == 0 && !starts_value {
                return None;
            }
        }
        return Some(name.trim().to_owned());
    }
    None
}

/// A property's namespace, its first word before `-` (`/^([a-z\d]+)-/i`).
fn namespace(prop: &str) -> Option<&str> {
    let word = prop.bytes().take_while(u8::is_ascii_alphanumeric).count();
    (word > 0 && prop.as_bytes().get(word) == Some(&b'-')).then(|| &prop[..word])
}

impl SheetRule for DeclarationNestedProperties {
    fn name(&self) -> &'static str {
        "scss/declaration-nested-properties"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "expected",
                "Expected property \"{}\" to be in a nested form",
            ),
            ("rejected", "Unexpected nested property \"{}\""),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut report = |index: usize, key: &'static str, name: &str| {
            let (start, end) = sheet.word_range(sheet.node(index), name);
            reports.push(Report::new(start, end, key).with(&[name]));
        };
        if !self.always {
            for index in sheet.sheet.walk() {
                let node = sheet.node(index);
                if node.is_nested() {
                    report(index, "rejected", &node.prop.clone());
                } else if node.is(Kind::Rule)
                    && let Some(name) = nested_name(&node.selector)
                {
                    report(index, "rejected", &name);
                }
            }
            return;
        }
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            if !(node.is(Kind::Rule) || node.is(Kind::AtRule)) {
                continue;
            }
            let mut candidates: Vec<(String, Vec<(usize, bool)>)> = Vec::new();
            for &child in node.nodes() {
                let each = sheet.node(child);
                let (name, nested) = if each.is(Kind::Declaration) && !each.is_nested() {
                    if !is_standard_property(&each.prop) {
                        continue;
                    }
                    let Some(name) = namespace(&each.prop) else {
                        continue;
                    };
                    (name.to_owned(), false)
                } else if each.is(Kind::Declaration) || each.is(Kind::Rule) {
                    let written = if each.is(Kind::Rule) {
                        each.selector.clone()
                    } else {
                        format!("{}: {}", each.prop, each.value)
                    };
                    let Some(name) = nested_name(&written) else {
                        continue;
                    };
                    (name, true)
                } else {
                    continue;
                };
                match candidates.iter_mut().find(|(key, _)| *key == name) {
                    Some((_, list)) => list.push((child, nested)),
                    None => candidates.push((name, vec![(child, nested)])),
                }
            }
            for (name, list) in &candidates {
                let several = list.len() > 1;
                for &(child, nested) in list {
                    if nested {
                        if self.except_only_of_namespace && sheet.node(child).nodes().len() == 1 {
                            report(child, "rejected", name);
                        }
                        continue;
                    }
                    if self.except_only_of_namespace && !several {
                        continue;
                    }
                    report(child, "expected", &sheet.node(child).prop.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeclarationNestedProperties, nested_name};
    use crate::scss::rules::tester::{Js, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/declaration-nested-properties.txt",
            &|config| {
                let except = secondary(config)
                    .and_then(|each| each.texts("except"))
                    .unwrap_or_default();
                Ok(Box::new(DeclarationNestedProperties {
                    always: primary(config) == &Js::Text("always".to_owned()),
                    except_only_of_namespace: except.iter().any(|each| each == "only-of-namespace"),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (26, 26, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(nested_name("font:"), Some("font".to_owned()));
        assert_eq!(nested_name("a:hover"), None);
    }
}
