//! The texts the rules read in a block (`eslint-plugin-jsdoc` 65.3.1, `src/iterateJsdoc.js` and
//! `src/jsdocUtils.js`; `@es-joy/jsdoccomment`, `src/parseInlineTags.js`): the description before
//! the tags, a tag's description, the code spans of Markdown and the inline tags `{@link …}`.

use super::block::{Block, Spec, is_space};

/// A block's description, as `utils.getDescription` gathers it.
#[derive(Debug, PartialEq, Eq)]
pub struct Description {
    /// Its lines joined by line breaks; an empty first line read as a line break.
    pub text: String,
    /// The lines.
    pub lines: Vec<String>,
    /// The index of the block line before the first tag or the `*/` line.
    pub last_line: usize,
}

/// A block's description: the descriptions of its lines up to the first tag, the `/**` line's
/// only when it holds text (`utils.getDescription`).
#[must_use]
pub fn description(block: &Block) -> Description {
    let mut lines: Vec<String> = Vec::new();
    let mut last_line = 0;
    let mut tags_begun = false;

    for (index, line) in block.source.iter().enumerate() {
        let tokens = &line.tokens;
        if !tokens.tag.is_empty() {
            tags_begun = true;
        }
        if index > 0 && (!tokens.tag.is_empty() || !tokens.end.is_empty()) {
            last_line = index - 1;
            if !tags_begun && !tokens.description.is_empty() {
                lines.push(tokens.description.clone());
            }
            break;
        }
        if !tags_begun && (index > 0 || !tokens.description.is_empty()) {
            if tokens.description.is_empty() {
                lines.push(if lines.is_empty() { "\n" } else { "" }.to_owned());
            } else {
                lines.push(tokens.description.clone());
            }
        }
    }
    Description {
        text: lines.join("\n"),
        lines,
        last_line,
    }
}

/// The descriptions of a tag's lines, the space after its tag or its `*` left out, up to the
/// `*/` line, kept when it holds text (`getTagDescription` with `returnArray`).
#[must_use]
pub fn tag_descriptions(block: &Block, tag: &Spec) -> Vec<String> {
    let mut descriptions = Vec::new();
    for index in &tag.lines {
        let tokens = &block.source[*index].tokens;
        let before = if !tokens.tag.is_empty() && !tokens.post_tag.is_empty() {
            tokens.post_tag.as_str()
        } else if tokens.tag.is_empty()
            && tokens.name.is_empty()
            && tokens.r#type.is_empty()
            && !tokens.post_delimiter.is_empty()
        {
            tokens.post_delimiter.as_str()
        } else {
            ""
        };
        let description = format!(
            "{}{}{}",
            before.get(1..).unwrap_or_default(),
            tokens.description,
            tokens.line_end
        );
        if !tokens.end.is_empty() {
            if !description.is_empty() {
                descriptions.push(description);
            }
            break;
        }
        descriptions.push(description);
    }
    descriptions
}

/// A tag's description, its lines joined by line breaks (`getTagDescription`).
#[must_use]
pub fn tag_description(block: &Block, tag: &Spec) -> String {
    tag_descriptions(block, tag).join("\n")
}

/// The content ranges of the Markdown code spans of a text, in order: a run of backticks not
/// after a backslash, then the shortest text up to the same run, neither run touching another
/// backtick (`` (?<!\\)(`+)(?!`)[\s\S]*?(?<!`)\1(?!`) ``).
#[must_use]
pub fn code_spans(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut spans = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] != b'`' || (at > 0 && bytes[at - 1] == b'\\') {
            at += 1;
            continue;
        }
        let run = bytes[at..].iter().take_while(|each| **each == b'`').count();
        let content = at + run;
        let close = (content..bytes.len()).find(|close| {
            bytes[*close..].starts_with(&bytes[at..content])
                && bytes[*close - 1] != b'`'
                && bytes.get(close + run) != Some(&b'`')
        });
        if let Some(close) = close {
            spans.push((content, close));
            at = close + run;
        } else {
            at += 1;
        }
    }
    spans
}

/// Whether an offset of a text sits inside a Markdown code span; the spans after the first one
/// starting beyond the offset are not read (`isInsideMarkdownCodeSpan` of `escape-inline-tags`).
#[must_use]
pub fn in_code_span_first(text: &str, offset: usize) -> bool {
    for (start, end) in code_spans(text) {
        if offset < start {
            return false;
        }
        if offset < end {
            return true;
        }
    }
    false
}

/// Whether an offset of a text sits inside any Markdown code span (`isInsideMarkdownCodeSpan`
/// of `normalize-see-links`).
#[must_use]
pub fn in_code_span(text: &str, offset: usize) -> bool {
    code_spans(text)
        .iter()
        .any(|(start, end)| offset >= *start && offset < *end)
}

/// How an inline tag gives its text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// `{@link target}`, no text.
    Plain,
    /// `{@link target|text}`.
    Pipe,
    /// `[text]{@link target}`.
    Prefix,
    /// `{@link target text}`.
    Space,
}

/// An inline tag of a description.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlineTag {
    /// Its name, without `@`.
    pub tag: String,
    /// The name path or address it points to.
    pub target: String,
    /// Its text, escapes read.
    pub text: String,
    /// How it gives its text.
    pub format: Format,
    /// Its first byte in the description.
    pub start: usize,
}

/// Whether a character is a word character of a regular expression (`\w`).
#[must_use]
pub fn is_word(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

/// The length of a text of characters other than a backslash and a closing character, or a
/// backslash and any character, as many as possible (`(?:[^\\\]]|\\[\s\S])*`).
fn escaped_text(text: &str, closing: char) -> usize {
    let mut characters = text.char_indices().peekable();
    let mut length = 0;
    while let Some((at, character)) = characters.next() {
        if character == '\\' {
            match characters.next() {
                Some((next_at, next)) => length = next_at + next.len_utf8(),
                None => return at,
            }
        } else if character == closing {
            return at;
        } else {
            length = at + character.len_utf8();
        }
    }
    length
}

/// The tag and target of `{@tag target`, from the `{`, and the offset after them: the tag up to a
/// blank or `}`, one optional blank, then the target up to a blank, `|` or `}`.
fn tag_and_target(text: &str) -> Option<(String, String, usize)> {
    let rest = text.strip_prefix("{@")?;
    let tag = rest
        .find(|each: char| each == '}' || is_space(each))
        .unwrap_or(rest.len());
    if tag == 0 {
        return None;
    }
    let mut at = tag;
    if rest[at..].starts_with(is_space) {
        at += rest[at..].chars().next().map_or(1, char::len_utf8);
    }
    let target = rest[at..]
        .find(|each: char| each == '}' || each == '|' || is_space(each))
        .unwrap_or(rest.len() - at);
    Some((
        rest[..tag].to_owned(),
        rest[at..at + target].to_owned(),
        2 + at + target,
    ))
}

/// The inline tag `[text]{@tag target}` starting at an offset.
fn prefixed_at(text: &str, start: usize) -> Option<(InlineTag, usize)> {
    let rest = text[start..].strip_prefix('[')?;
    let label = escaped_text(rest, ']');
    if label == 0 || !rest[label..].starts_with("]{@") {
        return None;
    }
    let brace = start + 1 + label + 1;
    let (tag, target, after) = tag_and_target(&text[brace..])?;
    if !text[brace + after..].starts_with('}') {
        return None;
    }
    let raw = &rest[..label];
    Some((
        InlineTag {
            tag,
            target,
            text: decode(raw, Format::Prefix),
            format: Format::Prefix,
            start,
        },
        brace + after + 1,
    ))
}

/// The inline tag `{@tag target separator text}` starting at an offset, not after `]`.
fn suffixed_at(text: &str, start: usize) -> Option<(InlineTag, usize)> {
    if start > 0 && text[..start].ends_with(']') {
        return None;
    }
    let (tag, target, after) = tag_and_target(&text[start..])?;
    let mut at = start + after;
    at += text[at..].len() - text[at..].trim_start_matches(is_space).len();
    let mut separator = None;
    if let Some(next) = text[at..].chars().next()
        && next == '|'
    {
        separator = Some('|');
        at += 1;
    }
    at += text[at..].len() - text[at..].trim_start_matches(is_space).len();
    let length = escaped_text(&text[at..], '}');
    if !text[at + length..].starts_with('}') {
        return None;
    }
    let raw = &text[at..at + length];
    let format = if raw.is_empty() {
        Format::Plain
    } else if separator == Some('|') {
        Format::Pipe
    } else {
        Format::Space
    };
    Some((
        InlineTag {
            tag,
            target,
            text: decode(raw, format),
            format,
            start,
        },
        at + length + 1,
    ))
}

/// A text with its escapes of a backslash and of the closing character read
/// (`decodeInlineTagText`).
fn decode(text: &str, format: Format) -> String {
    let closing = if format == Format::Prefix { ']' } else { '}' };
    let mut decoded = String::new();
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '\\'
            && let Some(next) = characters.peek().copied()
            && (next == '\\' || next == closing)
        {
            decoded.push(next);
            characters.next();
            continue;
        }
        decoded.push(character);
    }
    decoded
}

/// The inline tags of a description: every `[text]{@tag target}`, then every `{@tag …}` not
/// after `]`, each list in order (`parseDescription`).
#[must_use]
pub fn inline_tags(text: &str) -> Vec<InlineTag> {
    let mut found = Vec::new();
    for parse in [prefixed_at, suffixed_at] {
        let mut at = 0;
        while at < text.len() {
            if !text.is_char_boundary(at) {
                at += 1;
                continue;
            }
            if let Some((tag, end)) = parse(text, at) {
                found.push(tag);
                at = end;
            } else {
                at += 1;
            }
        }
    }
    found
}

/// The names a type reads, and those it declares (`infer U`, `[K in …]`), as far as a scan of
/// its words tells them: a word not after `.`, not a key before `:`, not a keyword, outside
/// quotes (the `JsdocTypeName`, `JsdocTypeInfer` and `JsdocTypeTypeParameter` nodes of
/// `jsdoc-type-pratt-parser`).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TypeNames {
    /// The names read.
    pub read: Vec<String>,
    /// The names declared.
    pub declared: Vec<String>,
}

/// The words of a type that are keywords, not names.
const TYPE_KEYWORDS: [&str; 13] = [
    "typeof",
    "keyof",
    "import",
    "infer",
    "is",
    "asserts",
    "readonly",
    "unique",
    "in",
    "extends",
    "null",
    "undefined",
    "new",
];

/// The names a type reads and declares.
#[must_use]
pub fn type_names(written: &str) -> TypeNames {
    let characters: Vec<char> = written.chars().collect();
    let mut names = TypeNames::default();
    let mut index = 0;
    let mut previous = ' ';
    let mut before_previous_word = String::new();
    while index < characters.len() {
        let character = characters[index];
        if character == '"' || character == '\'' || character == '`' {
            let close = characters[index + 1..]
                .iter()
                .position(|each| *each == character)
                .map_or(characters.len(), |at| index + 1 + at);
            index = close + 1;
            previous = character;
            continue;
        }
        let starts = character.is_alphabetic() || character == '_' || character == '$';
        if !starts {
            if !is_space(character) {
                previous = character;
            }
            index += 1;
            continue;
        }
        let start = index;
        while index < characters.len()
            && (characters[index].is_alphanumeric()
                || characters[index] == '_'
                || characters[index] == '$')
        {
            index += 1;
        }
        let word: String = characters[start..index].iter().collect();
        let next = characters[index..]
            .iter()
            .copied()
            .find(|each| !is_space(*each));
        let after_next = characters[index..]
            .iter()
            .copied()
            .filter(|each| !is_space(*each))
            .nth(1);
        let is_key = next == Some(':') || (next == Some('?') && after_next == Some(':'));
        let mapped_key = previous == '['
            && characters[index..]
                .iter()
                .collect::<String>()
                .trim_start()
                .starts_with("in ");
        if before_previous_word == "infer" || mapped_key {
            names.declared.push(word.clone());
        } else if previous != '.' && !is_key && !TYPE_KEYWORDS.contains(&word.as_str()) {
            names.read.push(word.clone());
        }
        before_previous_word = word;
        previous = 'a';
    }
    names
}
