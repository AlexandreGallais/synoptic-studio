//! `jsdoc/escape-inline-tags`: no `@word` at the start of a description's line or after a
//! blank, where TypeScript would read a tag, unless inside a Markdown code span or the
//! package of a `{@link …}` (`eslint-plugin-jsdoc` 65.3.1, `src/rules/escapeInlineTags.js`,
//! typescript mode, no fixer).

use crate::finding::Finding;
use crate::jsdoc::block::{Spec, is_space};
use crate::jsdoc::{self, Doc, text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct EscapeInlineTags;

/// The inline tags whose target may be a scoped package.
const REFERENCE_TAGS: [&str; 4] = ["inheritDoc", "link", "linkcode", "linkplain"];

/// The `@word` matches of a text, at its start or after a blank: where the match starts (its
/// blank, or the `@` at the start), where its `@` is, and the word (`(?:^|\s)@(\w+)`).
fn at_words(line: &str) -> Vec<(usize, usize, String)> {
    let mut found = Vec::new();
    let mut at = 0;
    while at < line.len() {
        let character = line[at..].chars().next().unwrap_or(' ');
        let sign = if at == 0 && character == '@' {
            Some(at)
        } else if is_space(character) && line[at + character.len_utf8()..].starts_with('@') {
            Some(at + character.len_utf8())
        } else {
            None
        };
        let word = sign.map_or(0, |sign| {
            line[sign + 1..]
                .chars()
                .take_while(|each| text::is_word(*each))
                .count()
        });
        if let Some(sign) = sign
            && word > 0
        {
            found.push((at, sign, line[sign + 1..sign + 1 + word].to_owned()));
            at = sign + 1 + word;
        } else {
            at += character.len_utf8();
        }
    }
    found
}

/// The name of the inline tag an offset sits in: the last `{@` at or before it, closed by a
/// `}` after it (`getInlineTagName`).
fn inline_tag_name(description: &str, sign: usize) -> &str {
    let Some(start) = description[..(sign + 2).min(description.len())].rfind("{@") else {
        return "";
    };
    match description[start..].find('}') {
        Some(end) if start + end > sign => {
            let rest = &description[start + 2..];
            &rest[..rest
                .find(|each: char| !text::is_word(each))
                .unwrap_or(rest.len())]
        }
        _ => "",
    }
}

/// Whether a text starts with a scoped package's name, `@scope/name`.
fn starts_scoped_package(text: &str) -> bool {
    let is_name = |each: char| text::is_word(each) || each == '.' || each == '-';
    text.strip_prefix('@').is_some_and(|rest| {
        let scope = rest.find(|each: char| !is_name(each)).unwrap_or(rest.len());
        scope > 0
            && rest[scope..]
                .strip_prefix('/')
                .is_some_and(|name| name.starts_with(is_name))
    })
}

/// Whether a `@word` at an offset is left alone (`shouldIgnoreMatch`).
fn is_ignored(description: &str, sign: usize) -> bool {
    text::in_code_span_first(description, sign)
        || (REFERENCE_TAGS.contains(&inline_tag_name(description, sign))
            && starts_scoped_package(&description[sign..]))
}

/// The first `@word` of a text not left alone (`getUnescapedInlineTagName`).
fn first_unescaped(description: &str) -> Option<String> {
    at_words(description)
        .into_iter()
        .find(|(_, sign, _)| !is_ignored(description, *sign))
        .map(|(_, _, word)| word)
}

/// A text with each `@name` at its start or after a blank escaped as `\@name`, those left alone
/// kept (the setter of `escapeInlineTags`, which does not end the name at a word boundary).
fn escaped(description: &str, name: &str) -> String {
    let pattern = format!("@{name}");
    let mut result = String::new();
    let mut copied = 0;
    let mut at = 0;
    while at < description.len() {
        let character = description[at..].chars().next().unwrap_or(' ');
        let sign = if at == 0 && description.starts_with(&pattern) {
            Some(at)
        } else if is_space(character)
            && description[at + character.len_utf8()..].starts_with(&pattern)
        {
            Some(at + character.len_utf8())
        } else {
            None
        };
        if let Some(sign) = sign {
            if !is_ignored(description, sign) {
                result.push_str(&description[copied..sign]);
                result.push('\\');
                copied = sign;
            }
            at = sign + pattern.len();
        } else {
            at += character.len_utf8();
        }
    }
    result.push_str(&description[copied..]);
    result
}

/// The message for an inline tag.
fn message(name: &str) -> String {
    format!(
        "Unexpected inline JSDoc tag. Did you mean to use {{@{name}}}, \\@{name}, or `@{name}`?"
    )
}

impl EscapeInlineTags {
    /// The findings of one block.
    fn check(&self, source: &Source, doc: &Doc, findings: &mut Vec<Finding>) {
        let description = text::description(&doc.block).text;
        let normalized = description.strip_prefix('\n').unwrap_or(&description);
        let mut line_start = 0;
        for (index, line) in normalized.split('\n').enumerate() {
            for (_, sign, word) in at_words(line) {
                if !is_ignored(normalized, line_start + sign) {
                    findings.push(doc.finding_at(
                        source,
                        self.name(),
                        index + 1,
                        None,
                        message(&word),
                    ));
                }
            }
            line_start += line.len() + 1;
        }

        let mut block = doc.block.clone();
        let tags: Vec<Spec> = block.tags.clone();
        for tag in tags.iter().filter(|tag| tag.tag != "example") {
            while let Some(word) = text::tag_descriptions(&block, tag)
                .iter()
                .find_map(|description| first_unescaped(description))
            {
                let pattern = format!("@{word}");
                let changed = tag.lines.iter().enumerate().find_map(|(position, index)| {
                    let current = &block.source[*index].tokens.description;
                    let found = !current.is_empty()
                        && at_words(current)
                            .iter()
                            .any(|(_, sign, _)| current[*sign..].starts_with(&pattern))
                        || current.starts_with(&pattern);
                    found.then_some(position)
                });
                let position = changed.unwrap_or(0);
                if let Some(position) = changed {
                    let index = tag.lines[position];
                    let current = block.source[index].tokens.description.clone();
                    block.source[index].tokens.description = escaped(&current, &word);
                    if block.source[index].tokens.description == current {
                        break;
                    }
                }
                findings.push(doc.finding_at(
                    source,
                    self.name(),
                    tag.lines[0] + position,
                    None,
                    message(&word),
                ));
                if changed.is_none() {
                    break;
                }
            }
        }
    }
}

impl Rule for EscapeInlineTags {
    fn name(&self) -> &'static str {
        "jsdoc/escape-inline-tags"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            self.check(source, &doc, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EscapeInlineTags;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&EscapeInlineTags, "escape-inline-tags", "src/a.ts");
    }
}
