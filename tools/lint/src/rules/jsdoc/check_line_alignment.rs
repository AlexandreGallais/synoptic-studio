//! `jsdoc/check-line-alignment` with `never`: on the first line of a `@param`, `@arg`,
//! `@argument`, `@property`, `@prop`, `@returns`, `@return` or `@template` tag, one space between
//! the tag, its type, its name and its description, and one after a hyphen; its next lines start
//! one space after their `*` (`eslint-plugin-jsdoc` 65.3.1, `src/rules/checkLineAlignment.js`).
//! Autofixed; the first misplaced next line ends the block's check.

use crate::finding::Finding;
use crate::jsdoc::block::{Block, Spec, Tokens, is_space};
use crate::jsdoc::{self, Doc};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct CheckLineAlignment;

/// The tags checked.
const TAGS: [&str; 8] = [
    "param", "arg", "argument", "property", "prop", "returns", "return", "template",
];

/// The tags that may hold a name (`tagMightHaveNameOrNamepath`, typescript mode).
const NAMED: [&str; 6] = ["param", "arg", "argument", "property", "prop", "template"];

/// A spacing token of the tag line.
#[derive(Clone, Copy)]
enum Spacer {
    /// After the delimiter.
    Delimiter,
    /// After the tag.
    Tag,
    /// After the type.
    Type,
    /// After the name.
    Name,
}

/// A content token of the tag line.
#[derive(Clone, Copy)]
enum Content {
    /// The tag.
    Tag,
    /// The type.
    Type,
    /// The name.
    Name,
    /// The description.
    Description,
}

/// A spacing token.
fn spacer(tokens: &mut Tokens, which: Spacer) -> &mut String {
    match which {
        Spacer::Delimiter => &mut tokens.post_delimiter,
        Spacer::Tag => &mut tokens.post_tag,
        Spacer::Type => &mut tokens.post_type,
        Spacer::Name => &mut tokens.post_name,
    }
}

/// A content token.
fn content(tokens: &mut Tokens, which: Content) -> &mut String {
    match which {
        Content::Tag => &mut tokens.tag,
        Content::Type => &mut tokens.r#type,
        Content::Name => &mut tokens.name,
        Content::Description => &mut tokens.description,
    }
}

/// Whether a description starts with blanks, then a hyphen, then a blank (`hasNoHyphen` denied).
fn has_hyphen(description: &str) -> bool {
    let rest = jsdoc::block::trim_start(description);
    rest.strip_prefix('-')
        .is_some_and(|after| after.starts_with(is_space))
}

/// Whether a description starts with blanks, a hyphen, then exactly one blank before text.
fn has_exact_hyphen_spacing(description: &str) -> bool {
    let rest = jsdoc::block::trim_start(description);
    rest.strip_prefix('-').is_some_and(|after| {
        let mut characters = after.chars();
        characters.next().is_some_and(is_space) && !characters.next().is_some_and(is_space)
    })
}

/// The tag line's spacers and contents, by whether the tag may hold a name.
fn layout(tag: &str) -> (Vec<Spacer>, Vec<Content>) {
    if NAMED.contains(&tag) {
        (
            vec![Spacer::Delimiter, Spacer::Tag, Spacer::Type, Spacer::Name],
            vec![
                Content::Tag,
                Content::Type,
                Content::Name,
                Content::Description,
            ],
        )
    } else {
        (
            vec![Spacer::Delimiter, Spacer::Tag, Spacer::Type],
            vec![Content::Tag, Content::Type, Content::Description],
        )
    }
}

/// Whether a tag line holds more than one space somewhere, or a hyphen spaced otherwise
/// (`checkNotAlignedPerTag`).
fn is_aligned(tokens: &mut Tokens, spacers: &[Spacer]) -> bool {
    let spaced = spacers.iter().all(|which| {
        let length = spacer(tokens, *which).encode_utf16().count();
        length == 1 || length == 0
    });
    spaced && (!has_hyphen(&tokens.description) || has_exact_hyphen_spacing(&tokens.description))
}

/// Spaces a tag line by one (the fix of `checkNotAlignedPerTag`).
fn unalign(tokens: &mut Tokens, spacers: &[Spacer], contents: &[Content]) {
    let exact = has_exact_hyphen_spacing(&tokens.description);
    for (index, which) in spacers.iter().enumerate() {
        if content(tokens, contents[index]).is_empty() {
            spacer(tokens, *which).clear();
            continue;
        }
        " ".clone_into(spacer(tokens, *which));
        for (later, later_spacer) in spacers.iter().enumerate().skip(index + 1) {
            if !spacer(tokens, *later_spacer).is_empty() {
                break;
            }
            content(tokens, contents[later]).clear();
        }
    }
    if !exact {
        let rest = jsdoc::block::trim_start(&tokens.description);
        if let Some(after) = rest.strip_prefix('-')
            && after.starts_with(is_space)
        {
            tokens.description = format!("- {}", jsdoc::block::trim_start(after));
        }
    }
}

/// Whether a line's description starts a list: `-` or `*`, or a number then `.` or `)`, then a
/// blank (`startsWithListMarker`).
fn starts_list(description: &str) -> bool {
    let rest = jsdoc::block::trim_start(description);
    let marker = if rest.starts_with(['-', '*']) {
        1
    } else {
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        if digits > 0 && rest[digits..].starts_with(['.', ')']) {
            digits + 1
        } else {
            return false;
        }
    };
    rest[marker..].starts_with(is_space)
}

/// Whether a tag's lines up to one of its next lines hold a list (`shouldAllowExtraIndent`).
fn in_list(block: &Block, tag: &Spec, upto: usize) -> bool {
    tag.lines
        .iter()
        .take(upto + 1)
        .enumerate()
        .any(|(position, index)| {
            position > 0 && starts_list(&block.source[*index].tokens.description)
        })
}

impl CheckLineAlignment {
    /// The findings of one block.
    fn check(&self, source: &Source, doc: &Doc, findings: &mut Vec<Finding>) {
        let mut block = doc.block.clone();
        let mut rewritten = Vec::new();
        let tags: Vec<Spec> = block
            .tags
            .iter()
            .filter(|tag| TAGS.contains(&tag.tag.as_str()))
            .cloned()
            .collect();

        for (position, tag) in tags.iter().enumerate() {
            let (spacers, contents) = layout(&tag.tag);
            let tokens = &mut block.source[tag.lines[0]].tokens;
            if is_aligned(tokens, &spacers) {
                continue;
            }
            unalign(tokens, &spacers, &contents);
            let mut finding = doc.finding_at(
                source,
                self.name(),
                tag.lines[0],
                None,
                "Expected JSDoc block lines to not be aligned.".to_owned(),
            );
            finding.fix = Some(doc.rewrite(&block));
            findings.push(finding);
            rewritten.push(position);
        }

        for (position, tag) in tags.iter().enumerate() {
            if tag.lines.len() < 2 || rewritten.contains(&position) {
                continue;
            }
            for (offset, index) in tag.lines.iter().enumerate().skip(1) {
                let tokens = &block.source[*index].tokens;
                if tokens.description.is_empty()
                    || !tokens.r#type.is_empty()
                    || !tokens.name.is_empty()
                {
                    continue;
                }
                let indent = tokens.post_delimiter.get(1..).unwrap_or_default();
                if indent.is_empty() || in_list(&block, tag, offset) {
                    continue;
                }
                let line = tag.lines[0] + offset;
                let tokens = &mut block.source[*index].tokens;
                tokens.post_delimiter = tokens.post_delimiter.chars().take(1).collect();
                let mut finding = doc.finding_at(
                    source,
                    self.name(),
                    line,
                    None,
                    "Expected wrap indent".to_owned(),
                );
                finding.fix = Some(doc.rewrite(&block));
                findings.push(finding);
                return;
            }
        }
    }
}

impl Rule for CheckLineAlignment {
    fn name(&self) -> &'static str {
        "jsdoc/check-line-alignment"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            self.check(source, &doc, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CheckLineAlignment;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&CheckLineAlignment, "check-line-alignment", "src/a.ts", &[]);
    }
}
