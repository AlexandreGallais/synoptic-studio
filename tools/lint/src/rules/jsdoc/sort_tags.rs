//! `jsdoc/sort-tags` with the project's sequence (`the records' conventions, area documentation`):
//! `@kind`, then `@template`, `@param`, `@returns`, `@yields`, `@throws`, `@rejects`, then `@see`,
//! `@deprecated`, other tags last; no blank line inside a group (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/sortTags.js`, group spacing not reported). Autofixed.

use std::cmp::Ordering;

use crate::finding::Finding;
use crate::jsdoc::block::Tokens;
use crate::jsdoc::edit::{Edit, v8_sort};
use crate::jsdoc::{self, Doc};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct SortTags;

/// The groups of tags, in order.
const SEQUENCE: [&[&str]; 3] = [
    &["kind"],
    &[
        "template", "param", "returns", "yields", "throws", "rejects",
    ],
    &["see", "deprecated"],
];

/// The tags in order, flattened.
const ORDER: [&str; 9] = [
    "kind",
    "template",
    "param",
    "returns",
    "yields",
    "throws",
    "rejects",
    "see",
    "deprecated",
];

/// A tag's place in the order, those out of it last.
fn position(tag: &str) -> usize {
    ORDER
        .iter()
        .position(|each| *each == tag)
        .unwrap_or(ORDER.len())
}

/// A tag, its lines' tokens and where it stood.
#[derive(Clone)]
struct Item {
    /// Its name.
    tag: String,
    /// Its lines' tokens.
    lines: Vec<Tokens>,
    /// Its index among the block's tags.
    index: usize,
    /// The count of the lines of the tags up to it, it included.
    line: usize,
}

/// The empty lines ending a tag, its `*/` line left out (`countTagEmptyLines`).
fn trailing_empty_lines(lines: &[Tokens]) -> usize {
    lines.iter().fold(0, |count, tokens| {
        let empty = tokens.tag.is_empty()
            && tokens.r#type.is_empty()
            && tokens.name.is_empty()
            && tokens.description.is_empty();
        if empty {
            count + usize::from(tokens.end.is_empty())
        } else {
            0
        }
    })
}

/// The index of the last tag of each group, as the plugin walks the groups.
fn last_tags_of_groups(tags: &[&str]) -> Vec<usize> {
    let mut last_tags = Vec::new();
    let mut index = 0;
    for group in SEQUENCE {
        let mut last = None;
        loop {
            let Some(current) = tags.get(index) else {
                index += 1;
                break;
            };
            if !group.contains(current) {
                index += 1;
                break;
            }
            last = Some(index);
            index += 1;
        }
        index -= 1;
        last_tags.extend(last);
    }
    last_tags
}

impl SortTags {
    /// The block with the empty lines removed inside groups (the fixer of the intra-group
    /// spacing), the lines removed one by one as the plugin's loop removes them.
    fn without_inner_blank_lines(block: &mut crate::jsdoc::block::Block, last_tags: &[String]) {
        let mut found_first = false;
        let mut current: Option<String> = None;
        let mut index = 0;
        while index < block.source.len() {
            let tokens = block.source[index].tokens.clone();
            if !tokens.tag.is_empty() {
                found_first = true;
                current = Some(tokens.tag.clone());
            }
            let empty = tokens.tag.is_empty()
                && tokens.r#type.is_empty()
                && tokens.name.is_empty()
                && tokens.description.is_empty()
                && tokens.end.is_empty();
            if found_first
                && empty
                && let Some(current) = &current
            {
                let ignore = if last_tags.iter().any(|tag| format!("@{tag}") == *current) {
                    !block.source[index + 1..]
                        .iter()
                        .any(|line| line.tokens.tag == *current)
                } else {
                    !block.source[index + 1..]
                        .iter()
                        .take_while(|line| line.tokens.end.is_empty())
                        .any(|line| !line.tokens.tag.is_empty())
                };
                if !ignore {
                    block.source.remove(index);
                }
            }
            index += 1;
        }
    }

    /// The findings of a block in order: blank lines inside a group (`reportIntraTagGroupSpacing`).
    fn check_spacing(
        &self,
        source: &Source,
        doc: &Doc,
        items: &[Item],
        findings: &mut Vec<Finding>,
    ) {
        let block = &doc.block;
        let names: Vec<&str> = block.tags.iter().map(|tag| tag.tag.as_str()).collect();
        let last_tags = last_tags_of_groups(&names);
        let last_names: Vec<String> = last_tags
            .iter()
            .map(|index| names[*index].to_owned())
            .collect();
        let mut fixed = block.clone();
        for (index, item) in items.iter().enumerate() {
            if index + 1 == items.len() || last_tags.contains(&index) {
                continue;
            }
            if trailing_empty_lines(&item.lines) == 0 {
                continue;
            }
            Self::without_inner_blank_lines(&mut fixed, &last_names);
            let mut finding = doc.finding_at(
                source,
                self.name(),
                block.tags[index].lines[0],
                None,
                "Intra-group tags have unexpected whitespace".to_owned(),
            );
            finding.fix = Some(doc.rewrite(&fixed));
            findings.push(finding);
        }
    }

    /// The findings of one block.
    fn check(&self, source: &Source, doc: &Doc, findings: &mut Vec<Finding>) {
        let block = &doc.block;
        let mut count = 0;
        let items: Vec<Item> = block
            .tags
            .iter()
            .enumerate()
            .map(|(index, tag)| {
                count += tag.lines.len();
                Item {
                    tag: tag.tag.clone(),
                    lines: tag
                        .lines
                        .iter()
                        .map(|line| block.source[*line].tokens.clone())
                        .collect(),
                    index,
                    line: count,
                }
            })
            .collect();
        let mut first_changed: Option<(usize, usize)> = None;
        let mut sorted = items.clone();
        v8_sort(&mut sorted, |new, old| {
            if new.tag == old.tag {
                return Ordering::Equal;
            }
            let order = position(&new.tag).cmp(&position(&old.tag));
            if order == Ordering::Less
                && first_changed.is_none_or(|(line, _)| line == 0 || old.line < line)
            {
                first_changed = Some((old.line, old.index));
            }
            order
        });

        let Some((_, first)) = first_changed else {
            self.check_spacing(source, doc, &items, findings);
            return;
        };

        let line_end = jsdoc::new_line_end(source);
        let mut edit = Edit::new(block, &doc.indent, &line_end);
        let first_line = edit.first_tag_line();
        let moved = items.len() - first;
        let unchanged: usize = items[..first].iter().map(|item| item.lines.len() - 1).sum();
        let offset = first_line + first + unchanged;
        for index in 0..moved {
            edit.remove_tag(index + first, 0);
        }
        let mut extra = 0;
        for (index, item) in sorted[first..].iter().enumerate() {
            edit.add_tag(
                extra + offset + index,
                Tokens {
                    end: String::new(),
                    ..item.lines[0].clone()
                },
            );
            for tokens in &item.lines[1..] {
                if tokens.end.is_empty() {
                    edit.add_line(
                        extra + offset + index + 1,
                        Tokens {
                            end: String::new(),
                            ..tokens.clone()
                        },
                    );
                    extra += 1;
                }
            }
        }
        let mut finding = doc.finding_at(
            source,
            self.name(),
            block.tags[first].lines[0],
            None,
            format!("Tags are not in the prescribed order: {}", ORDER.join(", ")),
        );
        finding.fix = Some(doc.replace(edit.text()));
        findings.push(finding);
    }
}

impl Rule for SortTags {
    fn name(&self) -> &'static str {
        "jsdoc/sort-tags"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            self.check(source, &doc, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SortTags;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&SortTags, "sort-tags", "src/a.ts");
    }
}
