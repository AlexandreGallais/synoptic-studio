//! A block edited as `eslint-plugin-jsdoc` 65.3.1 edits it in its fixes (`src/iterateJsdoc.js`:
//! `removeTag`, `addTag`, `addLine`, `moveClosingDelimiterToOwnLine`, `getFirstLine`): its lines
//! are shared by the block and its tags, each keeping a number that the edits renumber, and a
//! tag's lines are found back in the block by that number.

use super::block::{Block, Tokens, trim_end};

/// A line of the edited block.
#[derive(Clone, Debug)]
struct Line {
    /// Its number, renumbered by the edits.
    number: usize,
    /// Its tokens.
    tokens: Tokens,
}

/// An edited block.
#[derive(Clone, Debug)]
pub struct Edit {
    /// Every line ever made.
    pool: Vec<Line>,
    /// The block's lines, in order, as indices in the pool.
    order: Vec<usize>,
    /// Each tag's lines, as indices in the pool.
    tags: Vec<Vec<usize>>,
    /// The indent of the block's lines (`indent` + a space).
    start: String,
    /// The line end of an added line.
    line_end: String,
}

/// The tokens that hold content, from the last written to the first (`contentKeys`).
fn last_content(tokens: &mut Tokens) -> Option<&mut String> {
    if !tokens.description.is_empty() {
        Some(&mut tokens.description)
    } else if !tokens.post_name.is_empty() {
        Some(&mut tokens.post_name)
    } else if !tokens.name.is_empty() {
        Some(&mut tokens.name)
    } else if !tokens.post_type.is_empty() {
        Some(&mut tokens.post_type)
    } else if !tokens.r#type.is_empty() {
        Some(&mut tokens.r#type)
    } else if !tokens.post_tag.is_empty() {
        Some(&mut tokens.post_tag)
    } else if !tokens.tag.is_empty() {
        Some(&mut tokens.tag)
    } else {
        None
    }
}

/// Whether a line's tokens hold content (`hasContent`).
#[must_use]
pub fn has_content(tokens: &Tokens) -> bool {
    [
        &tokens.description,
        &tokens.post_name,
        &tokens.name,
        &tokens.post_type,
        &tokens.r#type,
        &tokens.post_tag,
        &tokens.tag,
    ]
    .iter()
    .any(|each| !each.is_empty())
}

/// A copy of tokens without the closing delimiter and the blanks before it (`withoutClosing`).
#[must_use]
pub fn without_closing(tokens: &Tokens, line_end: &str) -> Tokens {
    let mut copy = Tokens {
        end: String::new(),
        ..tokens.clone()
    };
    if !tokens.end.is_empty() {
        line_end.clone_into(&mut copy.line_end);
        if let Some(last) = last_content(&mut copy) {
            let kept = trim_end(last).len();
            last.truncate(kept);
        }
    }
    copy
}

impl Edit {
    /// A block ready for edits, with its lines' indent and the line end of added lines.
    #[must_use]
    pub fn new(block: &Block, indent: &str, line_end: &str) -> Self {
        let pool: Vec<Line> = block
            .source
            .iter()
            .map(|line| Line {
                number: line.number,
                tokens: line.tokens.clone(),
            })
            .collect();
        Self {
            order: (0..pool.len()).collect(),
            tags: block.tags.iter().map(|tag| tag.lines.clone()).collect(),
            pool,
            start: format!("{indent} "),
            line_end: line_end.to_owned(),
        }
    }

    /// The block written back.
    #[must_use]
    pub fn text(&self) -> String {
        self.order
            .iter()
            .map(|id| self.pool[*id].tokens.joined())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// A new line in the pool.
    fn make(&mut self, number: usize, tokens: Tokens) -> usize {
        self.pool.push(Line { number, tokens });
        self.pool.len() - 1
    }

    /// Removes, from a tag's first line found in the block and an offset of lines after it, the
    /// next lines of its description (`removeTag` with `tagSourceOffset`); a line opening or
    /// closing the block is emptied instead; the lines after are renumbered.
    pub fn remove_tag(&mut self, tag: usize, offset: usize) {
        let first_number = self.order.first().map_or(0, |id| self.pool[*id].number);
        let mut last_index = None;
        for position in 0..self.tags[tag].len() {
            let number = self.pool[self.tags[tag][position]].number;
            let Some(index) = self
                .order
                .iter()
                .position(|id| self.pool[*id].number == number)
            else {
                continue;
            };
            let count = 1 + self.tags[tag][position + 1..]
                .iter()
                .take_while(|id| {
                    let tokens = &self.pool[**id].tokens;
                    tokens.tag.is_empty() && tokens.end.is_empty()
                })
                .count();
            let at = index + offset;
            let Some(&line) = self.order.get(at) else {
                break;
            };
            let tokens = &self.pool[line].tokens;
            if (at == 0 && self.tags.len() >= 2)
                || !tokens.end.is_empty()
                || tokens.delimiter == "/**"
            {
                let tokens = &mut self.pool[line].tokens;
                for token in [
                    &mut tokens.post_delimiter,
                    &mut tokens.tag,
                    &mut tokens.post_tag,
                    &mut tokens.r#type,
                    &mut tokens.post_type,
                    &mut tokens.name,
                    &mut tokens.post_name,
                    &mut tokens.description,
                ] {
                    token.clear();
                }
            } else {
                let removed =
                    count.saturating_sub(offset) + if at == 0 { self.order.len() } else { 0 };
                let end = (at + removed).min(self.order.len());
                self.order.drain(at..end);
                let tag_start = (position + offset).min(self.tags[tag].len());
                let tag_end = (tag_start + removed).min(self.tags[tag].len());
                self.tags[tag].drain(tag_start..tag_end);
            }
            last_index = Some(index);
            break;
        }
        if let Some(last) = last_index {
            for (step, id) in self.order.iter().skip(last).enumerate() {
                self.pool[*id].number = first_number + last + step;
            }
        }
    }

    /// Inserts a line at an index of the block, numbered after the line before it; the lines
    /// from the index given by that number on are renumbered (`addLine`).
    pub fn add_line(&mut self, index: usize, tokens: Tokens) {
        let number = index
            .checked_sub(1)
            .and_then(|before| self.order.get(before))
            .map_or(0, |id| self.pool[*id].number)
            + 1;
        let id = self.make(number, tokens);
        self.order.insert(index.min(self.order.len()), id);
        for id in self.order.iter().skip(number + 1) {
            self.pool[*id].number += 1;
        }
    }

    /// Inserts a tag's line at an index of the block, its tokens over the defaults of a tag line;
    /// the lines after are renumbered (`addTag` with a number).
    pub fn add_tag(&mut self, index: usize, tokens: Tokens) {
        let id = self.make(index, tokens);
        let index = index.min(self.order.len());
        self.order.insert(index, id);
        for id in self.order.iter().skip(index + 1) {
            self.pool[*id].number += 1;
        }
    }

    /// Moves a `*/` sharing its line with text to a line of its own
    /// (`moveClosingDelimiterToOwnLine`).
    pub fn move_closing_to_own_line(&mut self) -> bool {
        let Some(&closing) = self.order.last() else {
            return false;
        };
        let original = self.pool[closing].tokens.clone();
        if original.end.is_empty() || !has_content(&original) {
            return false;
        }
        let line_end = self.line_end.clone();
        let tokens = &mut self.pool[closing].tokens;
        tokens.end.clear();
        tokens.line_end = line_end;
        if let Some(last) = last_content(tokens) {
            let kept = trim_end(last).len();
            last.truncate(kept);
        }
        let index = self.order.len();
        let start = self.start.clone();
        self.add_line(
            index,
            Tokens {
                end: original.end,
                line_end: original.line_end,
                start,
                ..Tokens::default()
            },
        );
        true
    }

    /// The number of the block's first line holding a tag (`getFirstLine`).
    #[must_use]
    pub fn first_tag_line(&self) -> usize {
        self.order
            .iter()
            .map(|id| &self.pool[*id])
            .find(|line| !line.tokens.tag.is_empty())
            .map_or(0, |line| line.number)
    }

    /// The line end of an added line.
    #[must_use]
    pub fn line_end(&self) -> &str {
        &self.line_end
    }

    /// The number of tags.
    #[must_use]
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }

    /// The number of lines of a tag.
    #[must_use]
    pub fn tag_length(&self, tag: usize) -> usize {
        self.tags[tag].len()
    }
}

/// Replaces a block's description lines, from its first line other than `/**` up to its first
/// tag or `*/` line, by the lines a setter makes from the first line's start, delimiter and
/// blank after it, the descriptions and the blanks after each delimiter (`setBlockDescription`).
pub fn set_block_description(
    block: &mut super::block::Block,
    setter: impl FnOnce(&Tokens, &[String], &[String]) -> Vec<Tokens>,
) {
    let Some(start) = block
        .source
        .iter()
        .position(|line| line.tokens.delimiter != "/**")
    else {
        return;
    };
    let end = (start..block.source.len())
        .find(|index| {
            let tokens = &block.source[*index].tokens;
            !tokens.tag.is_empty() || !tokens.end.is_empty()
        })
        .unwrap_or(block.source.len());
    if end == start {
        return;
    }
    let first = &block.source[start].tokens;
    let info = Tokens {
        delimiter: first.delimiter.clone(),
        post_delimiter: first.post_delimiter.clone(),
        start: first.start.clone(),
        ..Tokens::default()
    };
    let descriptions: Vec<String> = block.source[start..end]
        .iter()
        .map(|line| line.tokens.description.clone())
        .collect();
    let blanks: Vec<String> = block.source[start..end]
        .iter()
        .map(|line| line.tokens.post_delimiter.clone())
        .collect();
    let lines = setter(&info, &descriptions, &blanks)
        .into_iter()
        .map(|tokens| super::block::Line {
            number: 0,
            source: String::new(),
            tokens,
        });
    block.source.splice(start..end, lines);
}

/// Sorts items as V8 sorts an array shorter than 64 items (`ArrayTimSort`): the run at its start
/// counted, reversed when strictly descending, then the rest put in by binary insertion; the
/// comparisons made in that order, so that a comparator noting them sees what V8 shows it.
pub fn v8_sort<T: Clone>(items: &mut [T], mut compare: impl FnMut(&T, &T) -> std::cmp::Ordering) {
    use std::cmp::Ordering;
    let length = items.len();
    if length < 2 {
        return;
    }
    let mut run = 2;
    let descending = compare(&items[1], &items[0]) == Ordering::Less;
    while run < length {
        let order = compare(&items[run], &items[run - 1]);
        if (descending && order != Ordering::Less) || (!descending && order == Ordering::Less) {
            break;
        }
        run += 1;
    }
    if descending {
        items[..run].reverse();
    }
    for start in run..length {
        let pivot = items[start].clone();
        let (mut left, mut right) = (0, start);
        while left < right {
            let middle = left + (right - left) / 2;
            if compare(&pivot, &items[middle]) == Ordering::Less {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        items[left..=start].rotate_right(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{Edit, v8_sort, without_closing};
    use crate::jsdoc::block::parse;

    #[test]
    fn removes_a_tag_and_its_description_lines() {
        let block = parse("/**\n * Text.\n * @param a - one\n *   more\n * @param b - two\n */");
        let mut edit = Edit::new(&block, "", "");

        edit.remove_tag(0, 0);

        assert_eq!(edit.text(), "/**\n * Text.\n * @param b - two\n */");
    }

    #[test]
    fn empties_a_tag_on_the_closing_line_instead_of_removing_it() {
        // The tag shares its line with `*/`: the line stays, all but its start, `*` and `*/` gone.
        let block = parse("/**\n * Text.\n * @param a - one */");
        let mut edit = Edit::new(&block, "", "");

        edit.remove_tag(0, 0);

        assert_eq!(edit.text(), "/**\n * Text.\n **/");
    }

    #[test]
    fn moves_the_closing_delimiter_to_its_own_line() {
        let block = parse("/**\n * @param a - one */");
        let mut edit = Edit::new(&block, "  ", "");

        assert!(edit.move_closing_to_own_line());
        assert!(!edit.move_closing_to_own_line());
        assert_eq!(edit.text(), "/**\n * @param a - one\n   */");
    }

    #[test]
    fn copies_a_line_without_its_closing_and_the_blanks_before_it() {
        // The last content is the name, then the type, then the tag: each trimmed at its end.
        let named = parse("/**\n * @param a */").source[1].tokens.clone();
        let typed = parse("/**\n * @returns {number} */").source[1]
            .tokens
            .clone();
        let tag = parse("/**\n * @ignore */").source[1].tokens.clone();

        assert_eq!(without_closing(&named, "\r").joined(), " * @param a\r");
        assert_eq!(without_closing(&typed, "").joined(), " * @returns {number}");
        assert_eq!(without_closing(&tag, "").joined(), " * @ignore");
    }

    #[test]
    fn sorts_as_v8_with_its_comparisons() {
        // The run 2, 1 is descending: reversed; 3 then 0 put in by binary insertion. Node.js 24
        // shows the same comparisons for `[2, 1, 3, 0].sort((a, b) => a - b)`.
        let mut items = [2, 1, 3, 0];
        let mut compared = Vec::new();
        v8_sort(&mut items, |left, right| {
            compared.push((*left, *right));
            left.cmp(right)
        });

        assert_eq!(items, [0, 1, 2, 3]);
        assert_eq!(compared, [(1, 2), (3, 1), (3, 2), (0, 2), (0, 1)]);
    }
}
