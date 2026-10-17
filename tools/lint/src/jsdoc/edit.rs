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

    /// The tokens of a tag's lines, in order.
    #[must_use]
    pub fn tag_tokens(&self, tag: usize) -> Vec<Tokens> {
        self.tags[tag]
            .iter()
            .map(|id| self.pool[*id].tokens.clone())
            .collect()
    }

    /// A new line in the pool.
    fn make(&mut self, number: usize, tokens: Tokens) -> usize {
        self.pool.push(Line { number, tokens });
        self.pool.len() - 1
    }

    /// Removes a tag's first line found in the block and the next lines of its description
    /// (`removeTag`, its default options); a line opening or closing the block is emptied
    /// instead; the lines after are renumbered.
    pub fn remove_tag(&mut self, tag: usize) {
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
            let tokens = &self.pool[self.order[index]].tokens;
            if (index == 0 && self.tags.len() >= 2)
                || !tokens.end.is_empty()
                || tokens.delimiter == "/**"
            {
                let tokens = &mut self.pool[self.order[index]].tokens;
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
                let removed = if index == 0 {
                    count + self.order.len()
                } else {
                    count
                };
                let end = (index + removed).min(self.order.len());
                self.order.drain(index..end);
                let tag_end = (position + removed).min(self.tags[tag].len());
                self.tags[tag].drain(position..tag_end);
            }
            last_index = Some(index);
            break;
        }
        if let Some(last) = last_index {
            for (offset, id) in self.order.iter().skip(last).enumerate() {
                self.pool[*id].number = first_number + last + offset;
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
