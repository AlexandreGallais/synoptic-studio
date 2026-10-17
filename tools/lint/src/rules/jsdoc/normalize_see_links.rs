//! `jsdoc/normalize-see-links`: a labeled link of a `@see` tag is written `{@link target|label}`,
//! not `[label](target)` nor `[label]{@link target}`; a link that cannot be rewritten safely is
//! reported as such (`eslint-plugin-jsdoc` 65.3.1, `src/rules/normalizeSeeLinks.js`, its default
//! options: the pipe form, fixer on, bare addresses left). Autofixed.

use crate::finding::Finding;
use crate::jsdoc::block::{Spec, is_space, trim};
use crate::jsdoc::{self, Doc, text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NormalizeSeeLinks;

/// Whether a character ends a line.
fn is_line_end(character: char) -> bool {
    character == '\r' || character == '\n'
}

/// The length of a label: characters but a backslash, the closing one and those refused, or a
/// backslash and a character not ending a line, as many as possible.
fn label_length(text: &str, closing: char, refused: &[char]) -> usize {
    let mut characters = text.char_indices();
    let mut length = 0;
    while let Some((at, character)) = characters.next() {
        if character == '\\' {
            match characters.next() {
                Some((next_at, next)) if !is_line_end(next) => length = next_at + next.len_utf8(),
                _ => return at,
            }
        } else if character == closing || is_line_end(character) || refused.contains(&character) {
            return at;
        } else {
            length = at + character.len_utf8();
        }
    }
    length
}

/// A link found: where it starts and ends, its label and its target.
struct Link {
    /// Its first byte.
    start: usize,
    /// The byte after it.
    end: usize,
    /// Its label, as written.
    label: String,
    /// Its target.
    target: String,
}

/// Whether the byte before an offset is a backslash.
fn after_backslash(text: &str, at: usize) -> bool {
    at > 0 && text.as_bytes()[at - 1] == b'\\'
}

/// `[label](target)` at an offset (`markdownLinkRegex`).
fn markdown_at(text: &str, at: usize) -> Option<Link> {
    if !text[at..].starts_with('[') || after_backslash(text, at) {
        return None;
    }
    let label = label_length(&text[at + 1..], ']', &['`']);
    let after = at + 1 + label;
    if label == 0 || !text[after..].starts_with("](") {
        return None;
    }
    let target = text[after + 2..]
        .find(|each: char| each == '`' || each == '(' || each == ')' || is_space(each))
        .unwrap_or(text.len() - after - 2);
    let close = after + 2 + target;
    (target > 0 && text[close..].starts_with(')')).then(|| Link {
        start: at,
        end: close + 1,
        label: text[at + 1..after].to_owned(),
        target: text[after + 2..close].to_owned(),
    })
}

/// The target of `\s+target}` after `{@link`, and the offset after the `}`.
fn link_target(text: &str, at: usize) -> Option<(String, usize)> {
    let blanks = text[at..].len() - text[at..].trim_start_matches(is_space).len();
    let from = at + blanks;
    let target = text[from..]
        .find(|each: char| each == '}' || each == '|' || is_space(each))
        .unwrap_or(text.len() - from);
    (blanks > 0 && target > 0 && text[from + target..].starts_with('}'))
        .then(|| (text[from..from + target].to_owned(), from + target + 1))
}

/// `[label]{@link target}` at an offset (`prefixLinkRegex`).
fn prefix_at(text: &str, at: usize) -> Option<Link> {
    if !text[at..].starts_with('[') || after_backslash(text, at) {
        return None;
    }
    let label = label_length(&text[at + 1..], ']', &[]);
    let after = at + 1 + label;
    if label == 0 || !text[after..].starts_with("]{@link") {
        return None;
    }
    let (target, end) = link_target(text, after + 7)?;
    Some(Link {
        start: at,
        end,
        label: text[at + 1..after].to_owned(),
        target,
    })
}

/// `{@link target|label}` at an offset (`pipeLinkRegex`).
fn pipe_at(text: &str, at: usize) -> Option<Link> {
    let rest = text[at..].strip_prefix("{@link")?;
    let from = at + 6;
    let blanks = rest.len() - rest.trim_start_matches(is_space).len();
    let start = from + blanks;
    let target = text[start..]
        .find(|each: char| each == '}' || each == '|' || is_space(each))
        .unwrap_or(text.len() - start);
    if blanks == 0 || target == 0 {
        return None;
    }
    let mut position = start + target;
    position += text[position..].len() - text[position..].trim_start_matches(is_space).len();
    if !text[position..].starts_with('|') {
        return None;
    }
    let after_pipe = position + 1;
    let spaces = text[after_pipe..].len() - text[after_pipe..].trim_start_matches(is_space).len();
    let mut label_start = after_pipe + spaces;
    let mut label = label_length(&text[label_start..], '}', &[]);
    if label == 0 && spaces > 0 {
        label_start -= text[..label_start]
            .chars()
            .next_back()
            .map_or(1, char::len_utf8);
        label = label_length(&text[label_start..], '}', &[]);
    }
    let close = label_start + label;
    (label > 0 && text[close..].starts_with('}')).then(|| Link {
        start: at,
        end: close + 1,
        label: text[label_start..close].to_owned(),
        target: text[start..start + target].to_owned(),
    })
}

/// Every link a pattern finds in a text, in order, none overlapping.
fn links(text: &str, find: fn(&str, usize) -> Option<Link>) -> Vec<Link> {
    let mut found = Vec::new();
    let mut at = 0;
    while at < text.len() {
        if !text.is_char_boundary(at) {
            at += 1;
            continue;
        }
        if let Some(link) = find(text, at) {
            at = link.end.max(at + 1);
            found.push(link);
        } else {
            at += 1;
        }
    }
    found
}

/// A text with the links a pattern finds replaced.
fn replaced(
    text: &str,
    find: fn(&str, usize) -> Option<Link>,
    with: impl Fn(&Link) -> String,
) -> String {
    let mut result = String::new();
    let mut copied = 0;
    for link in links(text, find) {
        result.push_str(&text[copied..link.start]);
        result.push_str(&with(&link));
        copied = link.end;
    }
    result.push_str(&text[copied..]);
    result
}

/// The rest of a line from an offset, up to the first `\r` or `\n`; whether a line end stops it.
fn line_rest(text: &str, at: usize) -> (&str, bool) {
    match text[at..].find(is_line_end) {
        Some(end) => (&text[at..at + end], true),
        None => (&text[at..], false),
    }
}

/// Whether a text holds an attempt at a Markdown, prefixed or piped link the patterns did not
/// read (`markdownLinkAttemptRegex`, `prefixLinkAttemptRegex`, `pipeLinkAttemptRegex`).
fn has_link_attempt(text: &str) -> bool {
    text.char_indices().any(|(at, character)| {
        if character == '[' && !after_backslash(text, at) {
            let (line, _) = line_rest(text, at + 1);
            let Some(close) = line.find(']') else {
                return false;
            };
            let after = at + 1 + close + 1;
            if text[after..].starts_with('(') {
                let (rest, stopped) = line_rest(text, after + 1);
                return rest.contains(')') || !stopped;
            }
            if text[after..].starts_with("{@link") {
                let (rest, stopped) = line_rest(text, after + 6);
                return rest.contains('}') || !stopped;
            }
            return false;
        }
        if text[at..].starts_with("{@link") {
            let (rest, stopped) = line_rest(text, at + 6);
            let segment = rest.find('}').map_or(rest, |close| &rest[..close]);
            return segment.contains('|') && (rest.contains('}') || !stopped);
        }
        false
    })
}

/// A target with `%` not opening an escape, `|`, `{` and `}` escaped (`encodeLinkTarget`).
fn encoded(target: &str) -> String {
    let mut result = String::new();
    for (at, character) in target.char_indices() {
        let escape = &target.as_bytes()[at + character.len_utf8()..];
        match character {
            '%' if !(escape.len() >= 2
                && escape[0].is_ascii_hexdigit()
                && escape[1].is_ascii_hexdigit()) =>
            {
                result.push_str("%25");
            }
            '|' => result.push_str("%7C"),
            '{' => result.push_str("%7B"),
            '}' => result.push_str("%7D"),
            _ => result.push(character),
        }
    }
    result
}

/// A label for the pipe form: trimmed, its source escapes read, then a backslash before a `}`
/// and before a backslash that ends it or precedes a backslash or `}` (`escapeLinkLabel`).
fn pipe_label(label: &str, markdown: bool) -> String {
    let mut decoded = String::new();
    let mut characters = trim(label).chars().peekable();
    while let Some(character) = characters.next() {
        if character == '\\'
            && let Some(next) = characters.peek().copied()
            && (next == '\\' || next == ']' || (markdown && next == '}'))
        {
            decoded.push(next);
            characters.next();
            continue;
        }
        decoded.push(character);
    }
    let characters: Vec<char> = decoded.chars().collect();
    let mut result = String::new();
    for (index, character) in characters.iter().enumerate() {
        let next = characters.get(index + 1);
        if *character == '}'
            || (*character == '\\' && next.is_none_or(|next| *next == '\\' || *next == '}'))
        {
            result.push('\\');
        }
        result.push(*character);
    }
    result
}

/// A description with its Markdown and prefixed links in the pipe form (`normalizeDescription`).
fn normalized(description: &str) -> String {
    let markdown = replaced(description, markdown_at, |link| {
        format!(
            "{{@link {}|{}}}",
            encoded(&link.target),
            pipe_label(&link.label, true)
        )
    });
    replaced(&markdown, prefix_at, |link| {
        format!(
            "{{@link {}|{}}}",
            encoded(&link.target),
            pipe_label(&link.label, false)
        )
    })
}

/// Whether a text holds a Markdown or prefixed link (the rule's `descriptionMatcher`).
fn has_fixable_link(text: &str) -> bool {
    !links(text, markdown_at).is_empty() || !links(text, prefix_at).is_empty()
}

/// What a `@see` tag's links need.
#[derive(PartialEq, Eq)]
enum Need {
    /// Nothing.
    Nothing,
    /// A rewrite into the pipe form.
    Rewrite,
    /// A report that it cannot be rewritten safely.
    Ambiguous,
}

/// Whether a target is a scoped package's name.
fn is_scoped(target: &str) -> bool {
    let is_name = |each: char| text::is_word(each) || each == '.' || each == '-';
    target.strip_prefix('@').is_some_and(|rest| {
        let scope = rest.find(|each: char| !is_name(each)).unwrap_or(rest.len());
        scope > 0
            && rest[scope..]
                .strip_prefix('/')
                .is_some_and(|name| name.starts_with(is_name))
    })
}

/// What a `@see` tag needs.
fn need(doc: &Doc, tag: &Spec) -> Need {
    let first = &doc.block.source[tag.lines[0]].tokens;
    let description = text::tag_description(&doc.block, tag);
    let raw = if first.name.is_empty() {
        description
    } else {
        format!("{}{}{description}", first.name, first.post_name)
    };
    let mut ambiguous = false;
    let mut noncanonical = false;
    for link in links(&raw, markdown_at) {
        if text::in_code_span(&raw, link.start)
            || is_scoped(&link.target)
            || trim(&link.label).is_empty()
        {
            ambiguous = true;
        } else {
            noncanonical = true;
        }
    }
    for inline in text::inline_tags(&tag.description) {
        if inline.tag != "link" || inline.text.is_empty() {
            continue;
        }
        if inline.format == text::Format::Space || trim(&inline.text).is_empty() {
            ambiguous = true;
            continue;
        }
        if inline.format == text::Format::Pipe {
            continue;
        }
        if text::in_code_span(&raw, inline.start) || is_scoped(&inline.target) {
            ambiguous = true;
        } else {
            noncanonical = true;
        }
    }
    let left = replaced(
        &replaced(
            &replaced(&raw, markdown_at, |_| String::new()),
            prefix_at,
            |_| String::new(),
        ),
        pipe_at,
        |_| String::new(),
    );
    if ambiguous || has_link_attempt(&left) {
        Need::Ambiguous
    } else if noncanonical {
        Need::Rewrite
    } else {
        Need::Nothing
    }
}

impl NormalizeSeeLinks {
    /// The findings of one block.
    fn check(&self, source: &Source, doc: &Doc, findings: &mut Vec<Finding>) {
        let mut fixable = Vec::new();
        for tag in doc.block.tags.iter().filter(|tag| tag.tag == "see") {
            match need(doc, tag) {
                Need::Ambiguous => findings.push(doc.finding_at(
                    source,
                    self.name(),
                    tag.lines[0],
                    None,
                    "@see link cannot be safely normalized.".to_owned(),
                )),
                Need::Rewrite => fixable.push(tag),
                Need::Nothing => {}
            }
        }
        let mut block = doc.block.clone();
        for tag in &fixable {
            let first = &mut block.source[tag.lines[0]].tokens;
            if !first.name.is_empty() {
                first.description =
                    format!("{}{}{}", first.name, first.post_name, first.description);
                first.name.clear();
                first.post_name.clear();
            }
            for _ in &tag.lines {
                if let Some(index) = tag
                    .lines
                    .iter()
                    .find(|index| has_fixable_link(&block.source[**index].tokens.description))
                {
                    let tokens = &mut block.source[*index].tokens;
                    tokens.description = normalized(&tokens.description);
                }
            }
        }
        for (position, tag) in fixable.iter().enumerate() {
            let mut finding = doc.finding_at(
                source,
                self.name(),
                tag.lines[0],
                None,
                "Expected @see link to use the pipe form.".to_owned(),
            );
            if position == 0 {
                finding.fix = Some(doc.rewrite(&block));
            }
            findings.push(finding);
        }
    }
}

impl Rule for NormalizeSeeLinks {
    fn name(&self) -> &'static str {
        "jsdoc/normalize-see-links"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            self.check(source, &doc, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NormalizeSeeLinks;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&NormalizeSeeLinks, "normalize-see-links", "src/a.ts", &[]);
    }
}
