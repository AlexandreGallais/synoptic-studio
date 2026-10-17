//! A `JSDoc` block cut into lines and tags, as `comment-parser` 1.4.9 cuts it with the tokenizers
//! of `@es-joy/jsdoccomment` (`parseComment.js`), which `eslint-plugin-jsdoc` 65.3.1 reads: each
//! line split into its tokens (`start`, `delimiter`, `postDelimiter`, `tag`, `postTag`, `type`,
//! `postType`, `name`, `postName`, `description`, `end`, `lineEnd`), each block tag a spec over
//! its lines. Joining every line's tokens gives the block back (`stringify`), so that a rule
//! fixes a block by changing its tokens.

/// The tokens of a line, in the order they are written.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tokens {
    /// The blanks before the delimiter.
    pub start: String,
    /// `/**` on the first line, `*` on the others, or nothing.
    pub delimiter: String,
    /// The blanks after the delimiter.
    pub post_delimiter: String,
    /// The tag, `@` included.
    pub tag: String,
    /// The blanks after the tag.
    pub post_tag: String,
    /// The type, braces included.
    pub r#type: String,
    /// The blanks after the type.
    pub post_type: String,
    /// The name, as written.
    pub name: String,
    /// The blanks after the name.
    pub post_name: String,
    /// The rest of the line.
    pub description: String,
    /// `*/` and the blanks after it, on the last line.
    pub end: String,
    /// A carriage return ending the line.
    pub line_end: String,
}

impl Tokens {
    /// The line written back.
    #[must_use]
    pub fn joined(&self) -> String {
        [
            &self.start,
            &self.delimiter,
            &self.post_delimiter,
            &self.tag,
            &self.post_tag,
            &self.r#type,
            &self.post_type,
            &self.name,
            &self.post_name,
            &self.description,
            &self.end,
            &self.line_end,
        ]
        .into_iter()
        .map(String::as_str)
        .collect()
    }
}

/// A line of a block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// Its index in the block, from 0.
    pub number: usize,
    /// Its text.
    pub source: String,
    /// Its tokens.
    pub tokens: Tokens,
}

/// A block tag and the lines it spans.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Spec {
    /// The tag's name, without `@`.
    pub tag: String,
    /// The name it gives, brackets and default left out.
    pub name: String,
    /// The type, without its outer braces.
    pub r#type: String,
    /// Whether the name is written between brackets.
    pub optional: bool,
    /// The default value written in the name.
    pub default: Option<String>,
    /// The description, its lines joined as written.
    pub description: String,
    /// What the tokenizers could not read.
    pub problems: Vec<Problem>,
    /// The indices of its lines in the block.
    pub lines: Vec<usize>,
}

/// A part of a tag the tokenizers could not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    /// The code of `comment-parser`, such as `spec:name:unpaired-brackets`.
    pub code: &'static str,
    /// The block's line it is on.
    pub line: usize,
}

/// A parsed block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    /// The description before the first tag, its lines trimmed and joined by spaces.
    pub description: String,
    /// The block tags, in order.
    pub tags: Vec<Spec>,
    /// The lines.
    pub source: Vec<Line>,
}

/// The tags whose text holds no type (`defaultNoTypes`).
const NO_TYPES: [&str; 10] = [
    "default",
    "defaultvalue",
    "description",
    "example",
    "file",
    "fileoverview",
    "license",
    "overview",
    "see",
    "summary",
];

/// The tags whose text holds no name (`defaultNoNames`).
const NO_NAMES: [&str; 19] = [
    "access",
    "author",
    "default",
    "defaultvalue",
    "description",
    "example",
    "exception",
    "file",
    "fileoverview",
    "kind",
    "license",
    "overview",
    "return",
    "returns",
    "since",
    "summary",
    "throws",
    "version",
    "variation",
];

/// Whether a character is a blank as JavaScript's `\s` reads it.
#[must_use]
pub fn is_space(character: char) -> bool {
    matches!(
        character,
        '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200a}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202f}'
                | '\u{205f}'
                | '\u{3000}'
                | '\u{feff}'
    )
}

/// A text without its leading blanks, as `trimStart()`.
#[must_use]
pub fn trim_start(text: &str) -> &str {
    text.trim_start_matches(is_space)
}

/// A text without its trailing blanks, as `trimEnd()`.
#[must_use]
pub fn trim_end(text: &str) -> &str {
    text.trim_end_matches(is_space)
}

/// A text without its blanks at both ends, as `trim()`.
#[must_use]
pub fn trim(text: &str) -> &str {
    trim_end(trim_start(text))
}

/// The leading blanks of a text and the rest (`splitSpace`).
#[must_use]
pub fn split_space(text: &str) -> (String, String) {
    let rest = trim_start(text);
    (text[..text.len() - rest.len()].to_owned(), rest.to_owned())
}

/// Whether a description starts a block tag: `@`, then a name without blank nor `/`, then a blank
/// or the end (`reTag` of `block-parser`).
fn starts_tag(description: &str) -> bool {
    let Some(rest) = description.strip_prefix('@') else {
        return false;
    };
    let length = rest
        .find(|character: char| is_space(character) || character == '/')
        .unwrap_or(rest.len());

    length > 0 && rest[length..].chars().next().is_none_or(is_space)
}

/// A line's tokens, as `source-parser` cuts them; whether the line closes the block.
fn line_tokens(line: &str, first: bool) -> (Tokens, bool) {
    let mut tokens = Tokens::default();
    let without_cr = line.trim_end_matches('\r');
    line[without_cr.len()..].clone_into(&mut tokens.line_end);
    let (start, mut rest) = split_space(without_cr);
    tokens.start = start;

    if first && rest.starts_with("/**") && !rest.starts_with("/***") {
        "/**".clone_into(&mut tokens.delimiter);
        (tokens.post_delimiter, rest) = split_space(&rest[3..]);
    }
    let closed = trim_end(&rest).ends_with("*/");
    if tokens.delimiter.is_empty() && rest.starts_with('*') && !rest.starts_with("*/") {
        "*".clone_into(&mut tokens.delimiter);
        (tokens.post_delimiter, rest) = split_space(&rest[1..]);
    }
    if closed {
        let trimmed = trim_end(&rest).len();
        rest[trimmed - 2..].clone_into(&mut tokens.end);
        rest.truncate(trimmed - 2);
    }
    tokens.description = rest;
    (tokens, closed)
}

/// Cuts a comment's text, from `/**` to `*/`, into its lines and tags; the first comment block
/// of the text only, as `parseComment` keeps it.
#[must_use]
pub fn parse(text: &str) -> Block {
    let mut source = Vec::new();
    let mut started = false;

    for line in text.split('\n') {
        let (tokens, closed) = line_tokens(line, !started);
        if !started && tokens.delimiter != "/**" {
            continue;
        }
        started = true;
        source.push(Line {
            number: source.len(),
            source: line.to_owned(),
            tokens,
        });
        if closed {
            break;
        }
    }
    let sections = sections(&source);
    let description = compact(&source, &sections[0]);
    let tags = sections[1..]
        .iter()
        .map(|lines| spec(&mut source, lines.clone()))
        .collect();

    Block {
        description,
        tags,
        source,
    }
}

/// The lines of the description, then of each tag (`block-parser`): a tag starts a section,
/// except between two fences of ```` ``` ````.
fn sections(source: &[Line]) -> Vec<Vec<usize>> {
    let mut sections = vec![Vec::new()];
    let mut fenced = false;

    for (index, line) in source.iter().enumerate() {
        if starts_tag(&line.tokens.description) && !fenced {
            sections.push(vec![index]);
        } else if let Some(last) = sections.last_mut() {
            last.push(index);
        }
        if line.tokens.description.split("```").count() % 2 == 0 {
            fenced = !fenced;
        }
    }
    sections
}

/// The description's lines, trimmed, the empty ones left out, joined by spaces (`compact`).
fn compact(source: &[Line], lines: &[usize]) -> String {
    lines
        .iter()
        .map(|index| trim(&source[*index].tokens.description))
        .filter(|description| !description.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// A tag read by the four tokenizers of `parseComment`, which stop at a critical problem.
fn spec(source: &mut [Line], lines: Vec<usize>) -> Spec {
    let mut spec = Spec {
        lines,
        ..Spec::default()
    };
    if !tag_tokenizer(source, &mut spec) {
        return spec;
    }
    if !NO_TYPES.contains(&spec.tag.as_str()) && !type_tokenizer(source, &mut spec) {
        return spec;
    }
    if spec.tag == "template" {
        template_tokenizer(source, &mut spec);
    } else if !NO_NAMES.contains(&spec.tag.as_str())
        && (spec.tag != "see" || !has_link(&source[spec.lines[0]].source))
        && !name_tokenizer(source, &mut spec)
    {
        return spec;
    }
    spec.description = preserved(source, &spec.lines);
    spec
}

/// Whether a line holds `{@link`, then anything, then `}` (`hasSeeWithLink`).
fn has_link(line: &str) -> bool {
    line.find("{@link")
        .is_some_and(|at| line[at + 6..].chars().skip(1).any(|each| each == '}'))
}

/// The tag tokenizer: `@name` and the blanks after it; false on a critical problem.
fn tag_tokenizer(source: &mut [Line], spec: &mut Spec) -> bool {
    let first = spec.lines[0];
    let tokens = &mut source[first].tokens;
    let blank = tokens.description.len() - trim_start(&tokens.description).len();
    let rest = &tokens.description[blank..];
    if !rest.starts_with('@') || rest.len() == 1 || rest[1..].starts_with(is_space) {
        spec.problems.push(Problem {
            code: "spec:tag:prefix",
            line: first,
        });
        return false;
    }
    let tag_length = rest.find(is_space).unwrap_or(rest.len());
    let tag = &rest[..tag_length];
    if tag.contains('/') {
        return true;
    }
    let post = rest[tag_length..].len() - trim_start(&rest[tag_length..]).len();
    tag[1..].clone_into(&mut spec.tag);
    tag.clone_into(&mut tokens.tag);
    rest[tag_length..tag_length + post].clone_into(&mut tokens.post_tag);
    tokens.description = rest[tag_length + post..].to_string();
    true
}

/// The type tokenizer, spacing preserved: the braces from the first line holding text, over as
/// many lines as they span; false on a critical problem.
fn type_tokenizer(source: &mut [Line], spec: &mut Spec) -> bool {
    let mut curlies: i32 = 0;
    let mut found: Vec<(usize, usize)> = Vec::new();
    let mut begun = false;

    for &index in &spec.lines {
        let description = &source[index].tokens.description;
        if !begun && !trim(description).is_empty() {
            begun = true;
        } else if !begun {
            continue;
        }
        if found.is_empty() && !description.starts_with('{') {
            return true;
        }
        let mut length = 0;
        for character in description.chars() {
            if character == '{' {
                curlies += 1;
            }
            if character == '}' {
                curlies -= 1;
            }
            length += character.len_utf8();
            if curlies == 0 {
                break;
            }
        }
        found.push((index, length));
        if curlies == 0 {
            break;
        }
    }
    if !begun {
        return true;
    }
    if curlies != 0 {
        spec.problems.push(Problem {
            code: "spec:type:unpaired-curlies",
            line: spec.lines[0],
        });
        return false;
    }
    let offset = source[found[0].0].tokens.post_delimiter.len();
    let mut parts = Vec::new();
    for (position, &(index, length)) in found.iter().enumerate() {
        let tokens = &mut source[index].tokens;
        let written = tokens.description[..length].to_owned();
        tokens.r#type.clone_from(&written);
        if position > 0 {
            let cut = offset.min(tokens.post_delimiter.len());
            tokens.r#type = format!("{}{written}", &tokens.post_delimiter[cut..]);
            tokens.post_delimiter.truncate(cut);
        }
        (tokens.post_type, tokens.description) = split_space(&tokens.description[length..]);
        parts.push(tokens.r#type.clone());
    }
    let joined = parts.join("\n");
    joined[1..joined.len() - 1].clone_into(&mut spec.r#type);
    true
}

/// The `@template` names: up to the first blank that follows neither a blank nor a comma, or
/// after the last `]` when the text starts with `[`.
fn template_tokenizer(source: &mut [Line], spec: &mut Spec) {
    let tokens = &mut source[spec.lines[0]].tokens;
    let remainder = tokens.description.clone();
    let from = if remainder.starts_with('[') && remainder.contains(']') {
        remainder.rfind(']').unwrap_or(0)
    } else {
        0
    };
    let mut previous: Option<char> = None;
    let mut position = None;
    for (at, character) in remainder[from..].char_indices() {
        if is_space(character) && !previous.is_some_and(|before| is_space(before) || before == ',')
        {
            position = Some(from + at);
            break;
        }
        previous = Some(character);
    }
    let name = position.map_or(remainder.as_str(), |at| &remainder[..at]);
    let (post_name, description) = position.map_or_else(Default::default, |at| {
        let extra = &remainder[at..];
        let (blank, rest) = split_space(extra);
        let line = rest.find('\r').map_or(rest.as_str(), |cr| &rest[..cr]);
        (blank, line.to_owned())
    });

    spec.optional = is_optional_template(name);
    name.clone_into(&mut spec.name);
    name.clone_into(&mut tokens.name);
    tokens.post_name = post_name;
    tokens.description = description;
    tokens.line_end = String::new();
}

/// Whether a template name is `[name=default]`: `[`, then `=`, then `]` (`optionalBrackets`).
fn is_optional_template(name: &str) -> bool {
    name.strip_prefix('[')
        .and_then(|inner| inner.find('=').map(|equal| &inner[equal..]))
        .is_some_and(|after| after.contains(']'))
}

/// Whether a value is written between double quotes.
fn is_quoted(value: &str) -> bool {
    value.len() >= 2 && value.starts_with('"') && value.ends_with('"')
}

/// Whether a text holds `=` not followed by `>`.
fn has_bare_equal(text: &str) -> bool {
    text.char_indices()
        .any(|(at, character)| character == '=' && !text[at + 1..].starts_with('>'))
}

/// Whether a name with a default starts as an identifier then `=` (`ID_Start`, then
/// `ID_Continue`, `$`, `_`, `.` or the joiners).
fn is_identifier_default(name: &str) -> bool {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    if !(first.is_alphabetic() || matches!(first, '$' | '_' | '.')) {
        return false;
    }
    for character in characters {
        if character == '=' {
            return true;
        }
        if !(character.is_alphanumeric()
            || matches!(character, '$' | '_' | '.' | '\u{200c}' | '\u{200d}'))
        {
            return false;
        }
    }
    false
}

/// A name token read: `[name=value]` optional with its default, `name=value` with its default
/// when the name is an identifier; the code of the critical problem otherwise.
fn read_name(token: &str) -> Result<(String, bool, Option<String>), &'static str> {
    let mut name = token.to_owned();
    let mut default = None;
    let optional = name.starts_with('[') && name.ends_with(']') && name.len() >= 2;

    if optional {
        let inner = name[1..name.len() - 1].to_owned();
        let mut parts = inner.splitn(2, '=');
        trim(parts.next().unwrap_or_default()).clone_into(&mut name);
        default = parts.next().map(|value| trim(value).to_owned());
        if name.is_empty() {
            return Err("spec:name:empty-name");
        }
        if default.as_deref() == Some("") {
            return Err("spec:name:empty-default");
        }
        if default
            .as_deref()
            .is_some_and(|value| !is_quoted(value) && has_bare_equal(value))
        {
            return Err("spec:name:invalid-default");
        }
    }
    if !optional
        && let Some(equal) = name
            .char_indices()
            .find(|(at, character)| *character == '=' && !name[at + 1..].starts_with('>'))
            .map(|(at, _)| at)
    {
        let before = trim(&name[..equal]).to_owned();
        let after = trim(&name[equal + 1..]).to_owned();
        if before.is_empty() {
            return Err("spec:name:empty-name");
        }
        if after.is_empty() {
            return Err("spec:name:empty-default");
        }
        if is_identifier_default(&name) {
            if !is_quoted(&after) && has_bare_equal(&after) {
                return Err("spec:name:invalid-default");
            }
            name = before;
            default = Some(after);
        }
    }
    Ok((name, optional, default))
}

/// The name tokenizer: a quoted name, or the text up to a blank outside brackets, `[name=value]`
/// optional; false on a critical problem.
fn name_tokenizer(source: &mut [Line], spec: &mut Spec) -> bool {
    let last_type = spec
        .lines
        .iter()
        .enumerate()
        .fold(0, |last, (position, index)| {
            if source[*index].tokens.r#type.is_empty() {
                last
            } else {
                position
            }
        });
    let mut position = last_type;
    if !spec.r#type.is_empty() {
        while position + 1 < spec.lines.len()
            && trim(&source[spec.lines[position]].tokens.description).is_empty()
        {
            position += 1;
        }
    }
    let line = spec.lines[position];
    let first = spec.lines[0];
    let tokens = &mut source[line].tokens;
    let text = trim_start(&tokens.description).to_owned();

    let quoted: Vec<&str> = text.split('"').collect();
    if quoted.len() > 1 && quoted[0].is_empty() && quoted.len() % 2 == 1 {
        quoted[1].clone_into(&mut spec.name);
        tokens.name = format!("\"{}\"", quoted[1]);
        (tokens.post_name, tokens.description) = split_space(&text[tokens.name.len()..]);
        return true;
    }

    let mut brackets: i32 = 0;
    let mut length = 0;
    for character in text.chars() {
        if brackets == 0 && is_space(character) {
            break;
        }
        if character == '[' {
            brackets += 1;
        }
        if character == ']' {
            brackets -= 1;
        }
        length += character.len_utf8();
    }
    if brackets != 0 {
        spec.problems.push(Problem {
            code: "spec:name:unpaired-brackets",
            line: first,
        });
        return false;
    }
    let token = text[..length].to_owned();
    let (name, optional, default) = match read_name(&token) {
        Ok(read) => read,
        Err(code) => {
            spec.problems.push(Problem { code, line: first });
            return false;
        }
    };
    spec.optional = optional;
    spec.name = name;
    spec.default = default;
    (tokens.post_name, tokens.description) = split_space(&text[token.len()..]);
    tokens.name = token;
    true
}

/// A tag's description, its lines kept as written (`preserve`): from the last line holding a
/// type, a first empty line opening the block and a last empty line closing it left out.
fn preserved(source: &[Line], lines: &[usize]) -> String {
    let mut lines = lines;
    if let Some(first) = lines.first()
        && source[*first].tokens.description.is_empty()
        && source[*first].tokens.delimiter == "/**"
    {
        lines = &lines[1..];
    }
    if let Some(last) = lines.last()
        && source[*last].tokens.description.is_empty()
        && source[*last].tokens.end.ends_with("*/")
    {
        lines = &lines[..lines.len() - 1];
    }
    let from = lines.iter().enumerate().fold(0, |last, (position, index)| {
        if source[*index].tokens.r#type.is_empty() {
            last
        } else {
            position
        }
    });
    lines[from.min(lines.len())..]
        .iter()
        .map(|index| {
            let tokens = &source[*index].tokens;
            let before = if tokens.delimiter.is_empty() {
                tokens.start.as_str()
            } else {
                tokens.post_delimiter.get(1..).unwrap_or_default()
            };
            format!("{before}{}", tokens.description)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// A block written back from its lines' tokens (`stringify`).
#[must_use]
pub fn stringify(block: &Block) -> String {
    block
        .source
        .iter()
        .map(|line| line.tokens.joined())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{parse, stringify};

    #[test]
    fn cuts_a_one_line_block_into_its_tokens() {
        let block = parse("/** Separator. */");
        let tokens = &block.source[0].tokens;

        // `/**`, one space, the text up to `*/`, its trailing space kept in the description.
        assert_eq!(
            (
                tokens.delimiter.as_str(),
                tokens.post_delimiter.as_str(),
                tokens.description.as_str(),
                tokens.end.as_str()
            ),
            ("/**", " ", "Separator. ", "*/")
        );
        assert_eq!(block.description, "Separator.");
        assert_eq!(block.tags, []);
    }

    #[test]
    fn reads_tags_with_their_type_name_and_description_over_lines() {
        let text = "/**\n * Sum\n *   of two.\n * @param {number} a - first\n *   term\n * @returns the sum\n */";
        let block = parse(text);
        let param = &block.tags[0];
        let line = &block.source[3].tokens;

        assert_eq!(block.description, "Sum of two.");
        assert_eq!(
            (
                param.tag.as_str(),
                param.r#type.as_str(),
                param.name.as_str()
            ),
            ("param", "number", "a")
        );
        // The second line keeps its own indent after the space of the `*` (`preserve`).
        assert_eq!(param.description, "- first\n  term");
        assert_eq!(param.lines, [3, 4]);
        assert_eq!(
            (
                line.tag.as_str(),
                line.post_tag.as_str(),
                line.r#type.as_str(),
                line.post_type.as_str()
            ),
            ("@param", " ", "{number}", " ")
        );
        // `@returns` holds no name: its text is all description.
        assert_eq!(
            (
                block.tags[1].name.as_str(),
                block.tags[1].description.as_str()
            ),
            ("", "the sum")
        );
        assert_eq!(stringify(&block), text);
    }

    #[test]
    fn reads_optional_names_defaults_and_their_problems() {
        let block = parse("/**\n * @param [a=1] one\n * @param b=2 two\n * @param [c rest\n */");

        assert_eq!(
            (
                block.tags[0].name.as_str(),
                block.tags[0].optional,
                block.tags[0].default.as_deref()
            ),
            ("a", true, Some("1"))
        );
        assert_eq!(
            (
                block.tags[1].name.as_str(),
                block.tags[1].optional,
                block.tags[1].default.as_deref()
            ),
            ("b", false, Some("2"))
        );
        assert_eq!(
            block.tags[2].problems[0].code,
            "spec:name:unpaired-brackets"
        );
    }

    #[test]
    fn leaves_tags_inside_a_fence_and_reads_template_names() {
        let block = parse("/**\n * ```\n * @param x\n * ```\n * @template T, U types\n */");

        assert_eq!(block.tags.len(), 1);
        assert_eq!(
            (
                block.tags[0].name.as_str(),
                block.tags[0].description.as_str()
            ),
            ("T, U", "types")
        );
    }

    #[test]
    fn keeps_a_carriage_return_as_the_line_end() {
        let block = parse("/**\r\n * Text.\r\n */");

        assert_eq!(block.source[1].tokens.line_end, "\r");
        assert_eq!(stringify(&block), "/**\r\n * Text.\r\n */");
    }
}
