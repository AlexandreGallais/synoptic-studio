//! `jsdoc/match-description` with the project's patterns (`docs/conventions/documentation.md`): a
//! function's description is a sentence, from a capital letter or a backtick to a final period
//! or colon; the descriptions of its `@param` and `@returns` tags are fragments, without final
//! period; `@copyright`, `@example`, `@see` and `@todo` are not empty
//! (`eslint-plugin-jsdoc` 65.3.1, `src/rules/matchDescription.js`).

use crate::finding::Finding;
use crate::jsdoc::block::{Block, Spec, trim, trim_end};
use crate::jsdoc::{self, Doc, context, text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct MatchDescription;

/// The message of a main description.
const SENTENCE: &str = "Main descriptions are sentences: capital first letter, final period.";

/// The message of a tag description.
const FRAGMENT: &str = "Tag descriptions are fragments without final period.";

/// Whether a text is a sentence: a capital letter or a backtick, then anything, then a period
/// or a colon, then blanks (`^[A-Z\x60][\s\S]*[.:]\s*$`).
fn is_sentence(description: &str) -> bool {
    let kept = trim_end(description);
    kept.len() >= 2
        && kept.starts_with(|first: char| first.is_ascii_uppercase() || first == '`')
        && kept.ends_with(['.', ':'])
}

/// Whether a text is a fragment: anything, then a character neither a period nor a blank, then
/// blanks (`^[\s\S]*[^.\s]\s*$`).
fn is_fragment(description: &str) -> bool {
    trim_end(description)
        .chars()
        .next_back()
        .is_some_and(|last| last != '.')
}

/// Whether a character is an emoji, as far as the code points of the pictographs and symbols go.
fn is_emoji(character: char) -> bool {
    matches!(u32::from(character),
        0x1F000..=0x1FAFF | 0x2600..=0x27BF | 0x2B00..=0x2BFF | 0x2300..=0x23FF | 0x3030 | 0x303D
            | 0x3297 | 0x3299 | 0x00A9 | 0x00AE | 0x203C | 0x2049 | 0x2122 | 0x2139 | 0x2194..=0x21AA)
}

/// Whether a text matches the plugin's default pattern: empty, or a capital letter, a backtick,
/// a digit or `_`, then anything, then `.`, `?`, `!`, a backtick or an emoji, then blanks, after
/// an optional line break (`^\n?([A-Z\x60\d_][\s\S]*[.?!\x60\p{RGI_Emoji}]\s*)?$`).
fn is_default_description(description: &str) -> bool {
    let rest = description.strip_prefix('\n').unwrap_or(description);
    if rest.is_empty() {
        return true;
    }
    let kept = trim_end(rest);
    let mut characters = kept.chars();
    let first = characters.next();
    let last = characters.next_back();
    first.is_some_and(|first| {
        first.is_ascii_uppercase() || first.is_ascii_digit() || first == '`' || first == '_'
    }) && last.is_some_and(|last| matches!(last, '.' | '?' | '!' | '`') || is_emoji(last))
}

/// A tag's name then description, trimmed.
fn named_description(block: &Block, tag: &Spec) -> String {
    trim(&format!(
        "{} {}",
        tag.name,
        text::tag_description(block, tag)
    ))
    .to_owned()
}

impl MatchDescription {
    /// The findings of one block.
    fn check(&self, source: &Source, doc: &Doc, findings: &mut Vec<Finding>) {
        let block = &doc.block;
        let description = text::description(block).text;
        if !description.is_empty() && !is_sentence(&description) {
            findings.push(doc.finding_at(source, self.name(), 1, None, SENTENCE.to_owned()));
        }
        for name in ["description", "summary", "file", "classdesc"] {
            for tag in block.tags.iter().filter(|tag| tag.tag == name) {
                if !is_default_description(&named_description(block, tag)) {
                    findings.push(doc.finding_at(
                        source,
                        self.name(),
                        tag.lines[0],
                        None,
                        SENTENCE.to_owned(),
                    ));
                }
            }
        }
        for name in ["copyright", "example", "see", "todo"] {
            for tag in block.tags.iter().filter(|tag| tag.tag == name) {
                if named_description(block, tag).is_empty() {
                    findings.push(doc.finding_at(
                        source,
                        self.name(),
                        tag.lines[0],
                        None,
                        "JSDoc description must not be empty.".to_owned(),
                    ));
                }
            }
        }
        for tag in block
            .tags
            .iter()
            .filter(|tag| tag.tag == "param" || tag.tag == "returns")
        {
            let description = text::tag_description(block, tag);
            let without_hyphen = description.trim_start_matches(['-', ' ']);
            if !is_fragment(trim(without_hyphen)) {
                findings.push(doc.finding_at(
                    source,
                    self.name(),
                    tag.lines[0],
                    None,
                    FRAGMENT.to_owned(),
                ));
            }
        }
    }
}

impl Rule for MatchDescription {
    fn name(&self) -> &'static str {
        "jsdoc/match-description"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for (_, doc) in context::documented(source, jsdoc::is_default_context) {
            self.check(source, &doc, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MatchDescription;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&MatchDescription, "match-description", "src/a.ts");
    }
}
