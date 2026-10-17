//! `jsdoc/tag-lines` with `never` and one line after the description: no empty line ends a tag,
//! and a description followed by tags ends with exactly one empty line
//! (`eslint-plugin-jsdoc` 65.3.1, `src/rules/tagLines.js`). Autofixed.

use crate::finding::Finding;
use crate::jsdoc::block::{Tokens, trim};
use crate::jsdoc::edit::{Edit, set_block_description};
use crate::jsdoc::{self, Doc, text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct TagLines;

/// Whether a line holds no tag, name, type nor description.
fn is_empty(tokens: &Tokens) -> bool {
    tokens.tag.is_empty()
        && tokens.name.is_empty()
        && tokens.r#type.is_empty()
        && tokens.description.is_empty()
}

impl TagLines {
    /// The finding of the first tag ending with an empty line, if any.
    fn check_tags(&self, source: &Source, doc: &Doc) -> Option<Finding> {
        for (tag_index, tag) in doc.block.tags.iter().enumerate() {
            let mut report = None;
            for (position, index) in tag.lines.iter().enumerate() {
                let tokens = &doc.block.source[*index].tokens;
                if !tokens.description.is_empty() {
                    report = None;
                }
                if is_empty(tokens) && tokens.end.is_empty() {
                    report = Some(position);
                }
            }
            if let Some(position) = report {
                let mut edit = Edit::new(&doc.block, &doc.indent, &jsdoc::new_line_end(source));
                edit.remove_tag(tag_index, position);
                let mut finding = doc.finding_at(
                    source,
                    self.name(),
                    tag.lines[0] + 1,
                    None,
                    "Expected no lines between tags".to_owned(),
                );
                finding.fix = Some(doc.replace(edit.text()));
                return Some(finding);
            }
        }
        None
    }

    /// The finding of a description not followed by one empty line before the tags, if any.
    fn check_description(&self, source: &Source, doc: &Doc) -> Option<Finding> {
        if doc.block.tags.is_empty() {
            return None;
        }
        let description = text::description(&doc.block);
        if trim(&description.text).is_empty() {
            return None;
        }
        let trailing = description.text.len() - description.text.trim_end_matches('\n').len();
        let line_end = jsdoc::new_line_end(source);
        let mut block = doc.block.clone();
        let (line, message) = match trailing {
            0 => {
                set_block_description(&mut block, |info, descriptions, blanks| {
                    descriptions
                        .iter()
                        .cloned()
                        .chain(std::iter::once(String::new()))
                        .enumerate()
                        .map(|(index, description)| Tokens {
                            post_delimiter: if trim(&description).is_empty() {
                                String::new()
                            } else {
                                blanks[index].clone()
                            },
                            description,
                            line_end: line_end.clone(),
                            ..info.clone()
                        })
                        .collect()
                });
                (
                    description.last_line,
                    "Expected 1 lines after block description",
                )
            }
            1 => return None,
            _ => {
                let extra = trailing - 1;
                set_block_description(&mut block, |info, descriptions, blanks| {
                    descriptions[..descriptions.len().saturating_sub(extra)]
                        .iter()
                        .enumerate()
                        .map(|(index, description)| Tokens {
                            description: description.clone(),
                            line_end: line_end.clone(),
                            post_delimiter: blanks[index].clone(),
                            ..info.clone()
                        })
                        .collect()
                });
                (
                    description.last_line.saturating_sub(extra),
                    "Expected only 1 line after block description",
                )
            }
        };
        let mut finding = doc.finding_at(source, self.name(), line, None, message.to_owned());
        finding.fix = Some(doc.rewrite(&block));
        Some(finding)
    }
}

impl Rule for TagLines {
    fn name(&self) -> &'static str {
        "jsdoc/tag-lines"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            findings.extend(self.check_tags(source, &doc));
            findings.extend(self.check_description(source, &doc));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TagLines;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&TagLines, "tag-lines", "src/a.ts", &[]);
    }
}
