//! What the empty-line rules share: the helpers of Stylelint 17.16.0 (`hasEmptyLine`,
//! `isFirstNodeOfRoot`, `isFirstNested`, `isAfterComment`, `isAfterSingleLineComment`,
//! `isSharedLineComment`, `getPreviousNonSharedLineCommentNode`,
//! `getNextNonSharedLineCommentNode`, the blockless at-rule tests, `addEmptyLineBefore`,
//! `removeEmptyLinesBefore`; REF-STYLELINT-SOURCE, MIT, under ADR-0076) and their options.

use super::{Context, Matcher, Report, matches_any};
use crate::javascript::is_js_space;
use crate::scss::tree::Kind;

/// An empty-line rule's options: its primary, `ignore` and `except` words, and the at-rules and
/// comments its options leave out.
#[derive(Clone, Debug, Default)]
pub struct Lines {
    /// The primary option: `always`, `never`, `always-multi-line`, `never-multi-line`.
    pub primary: String,
    /// The `ignore` words.
    pub ignore: Vec<String>,
    /// The `except` words.
    pub except: Vec<String>,
    /// The at-rules left out (`ignoreAtRules`).
    pub ignore_at_rules: Vec<Matcher>,
    /// The comments left out by their text (`ignoreComments`).
    pub ignore_comments: Vec<Matcher>,
}

impl Lines {
    /// Options of a primary and words.
    #[must_use]
    pub fn new(primary: &str, ignore: &[&str], except: &[&str]) -> Self {
        Self {
            primary: primary.to_owned(),
            ignore: ignore.iter().map(|&each| each.to_owned()).collect(),
            except: except.iter().map(|&each| each.to_owned()).collect(),
            ..Self::default()
        }
    }

    /// Whether `ignore` holds a word.
    #[must_use]
    pub fn ignores(&self, word: &str) -> bool {
        self.ignore.iter().any(|each| each == word)
    }

    /// Whether `except` holds a word.
    #[must_use]
    pub fn excepts(&self, word: &str) -> bool {
        self.except.iter().any(|each| each == word)
    }

    /// Whether an at-rule's name is left out.
    #[must_use]
    pub fn ignores_at_rule(&self, name: &str) -> bool {
        matches_any(&self.ignore_at_rules, name)
    }

    /// Whether a comment's text is left out.
    #[must_use]
    pub fn ignores_comment(&self, text: &str) -> bool {
        matches_any(&self.ignore_comments, text)
    }
}

/// Whether spaces hold an empty line (`/\n[\r\t ]*\n/`).
#[must_use]
pub fn has_empty_line(before: &str) -> bool {
    before.match_indices('\n').any(|(at, _)| {
        before[at + 1..]
            .find(|c: char| !matches!(c, '\r' | '\t' | ' '))
            .is_some_and(|next| before[at + 1 + next..].starts_with('\n'))
    })
}

/// Whether a node is the root's first.
#[must_use]
pub fn is_first_of_root(sheet: &Context, index: usize) -> bool {
    let node = sheet.node(index);
    node.parent == Some(0) && sheet.node(0).nodes().first() == Some(&index)
}

/// The line a node starts on.
fn line(sheet: &Context, index: usize) -> usize {
    sheet.lines.line(sheet.node(index).start)
}

/// The line a node ends on.
fn end_line(sheet: &Context, index: usize) -> usize {
    let node = sheet.node(index);
    sheet.lines.line(
        node.end
            .unwrap_or(sheet.text.len())
            .saturating_sub(1)
            .max(node.start),
    )
}

/// Whether a node is a comment.
fn is_comment(sheet: &Context, index: Option<usize>) -> bool {
    index.is_some_and(|index| sheet.node(index).is(Kind::Comment))
}

/// Whether a node is the first of a block, comments sharing the block's opening line before it
/// counted out (`isFirstNested`).
#[must_use]
pub fn is_first_nested(sheet: &Context, index: usize) -> bool {
    let Some(parent) = sheet.node(index).parent else {
        return false;
    };
    if parent == 0 {
        return false;
    }
    let siblings = sheet.node(parent).nodes();
    if siblings.first() == Some(&index) {
        return true;
    }
    let Some(&first) = siblings.first() else {
        return false;
    };
    if !sheet.node(first).is(Kind::Comment) || sheet.node(first).before.contains('\n') {
        return false;
    }
    let opening = line(sheet, first);
    if end_line(sheet, first) != opening {
        return false;
    }
    for &each in &siblings[1..] {
        if each == index {
            return true;
        }
        if !sheet.node(each).is(Kind::Comment) || end_line(sheet, each) != opening {
            return false;
        }
    }
    false
}

/// The node before one, comments sharing a line with what precedes them skipped
/// (`getPreviousNonSharedLineCommentNode`).
#[must_use]
pub fn previous_non_shared(sheet: &Context, index: usize) -> Option<usize> {
    let mut current = index;
    loop {
        let previous = sheet.sheet.previous(current)?;
        if !sheet.node(previous).is(Kind::Comment) {
            return Some(previous);
        }
        let shares = line(sheet, current) == line(sheet, previous)
            || sheet
                .sheet
                .previous(previous)
                .is_some_and(|before| line(sheet, previous) == line(sheet, before));
        if !shares {
            return Some(previous);
        }
        current = previous;
    }
}

/// The node after one, comments sharing a line skipped (`getNextNonSharedLineCommentNode`).
#[must_use]
pub fn next_non_shared(sheet: &Context, index: usize) -> Option<usize> {
    let mut current = index;
    loop {
        let next = sheet.sheet.next(current)?;
        if !sheet.node(next).is(Kind::Comment) {
            return Some(next);
        }
        let shares = line(sheet, current) == line(sheet, next)
            || sheet
                .sheet
                .next(next)
                .is_some_and(|after| line(sheet, next) == line(sheet, after));
        if !shares {
            return Some(next);
        }
        current = next;
    }
}

/// Whether a comment shares a line with a node before or after it, or opens its block on the
/// block's line (`isSharedLineComment`).
#[must_use]
pub fn is_shared_line_comment(sheet: &Context, index: usize, ignored: Option<usize>) -> bool {
    let node = sheet.node(index);
    if !node.is(Kind::Comment) {
        return false;
    }
    if previous_non_shared(sheet, index)
        .is_some_and(|previous| end_line(sheet, previous) == line(sheet, index))
    {
        return true;
    }
    if let Some(next) = next_non_shared(sheet, index)
        && Some(next) != ignored
        && end_line(sheet, index) == line(sheet, next)
    {
        return true;
    }
    node.parent.is_some_and(|parent| {
        parent != 0
            && sheet.node(parent).nodes().first() == Some(&index)
            && !node.before.contains('\n')
    })
}

/// Whether a node follows a comment of its own line (`isAfterComment`).
#[must_use]
pub fn is_after_comment(sheet: &Context, index: usize) -> bool {
    let previous = sheet.sheet.previous(index);
    is_comment(sheet, previous)
        && previous.is_some_and(|previous| !is_shared_line_comment(sheet, previous, None))
}

/// Whether a node follows a one-line comment of its own line (`isAfterSingleLineComment`).
#[must_use]
pub fn is_after_single_line_comment(sheet: &Context, index: usize) -> bool {
    let previous = sheet.sheet.previous(index);
    previous.is_some_and(|previous| {
        sheet.node(previous).is(Kind::Comment)
            && !is_shared_line_comment(sheet, previous, Some(index))
            && line(sheet, previous) == end_line(sheet, previous)
    })
}

/// Whether a blockless at-rule follows another (`isBlocklessAtRuleAfterBlocklessAtRule`).
#[must_use]
pub fn is_blockless_after_blockless(sheet: &Context, index: usize) -> bool {
    let node = sheet.node(index);
    node.is(Kind::AtRule)
        && node.children.is_none()
        && previous_non_shared(sheet, index).is_some_and(|previous| {
            let previous = sheet.node(previous);
            previous.is(Kind::AtRule) && previous.children.is_none()
        })
}

/// Whether a blockless at-rule follows another of its name.
#[must_use]
pub fn is_blockless_after_same_name(sheet: &Context, index: usize) -> bool {
    is_blockless_after_blockless(sheet, index)
        && previous_non_shared(sheet, index)
            .is_some_and(|previous| sheet.node(previous).name == sheet.node(index).name)
}

/// The newline of a text: its first, else a line feed.
fn newline(text: &str) -> &'static str {
    match text.find('\n') {
        Some(at) if at > 0 && text.as_bytes()[at - 1] == b'\r' => "\r\n",
        _ => "\n",
    }
}

/// A report over a node, its fix adding or removing the empty line before it, scoped to its
/// parent (`fixEmptyLinesBefore`).
#[must_use]
pub fn report(sheet: &Context, index: usize, key: &'static str, add: bool) -> Report {
    let node = sheet.node(index);
    let end = node.end.unwrap_or(sheet.text.len());
    let start = node.start.saturating_sub(node.before.len());
    let newline = newline(sheet.text);
    let before = &node.before;
    let fixed = if add {
        match before.find('\n') {
            Some(at) => {
                let at = if at > 0 && before.as_bytes()[at - 1] == b'\r' {
                    at - 1
                } else {
                    at
                };
                format!("{}{newline}{}", &before[..at], &before[at..])
            }
            None => format!("{newline}{newline}{before}"),
        }
    } else {
        remove_empty_lines(before, newline)
    };
    let parent = sheet.node(node.parent.unwrap_or(0));
    Report::new(node.start, end, key)
        .fixed(start, node.start, fixed)
        .scoped(parent.start, parent.end.unwrap_or(sheet.text.len()))
}

/// Spaces with each run of empty lines made one newline (`replace(/(\r?\n\s*\n)+/g, newline)`):
/// from a line feed, or the carriage return before it, to the last line feed of the spaces that
/// follow, when there is a second.
fn remove_empty_lines(before: &str, newline: &str) -> String {
    let mut result = String::with_capacity(before.len());
    let mut rest = before;
    while let Some(at) = rest.find('\n') {
        let start = if at > 0 && rest.as_bytes()[at - 1] == b'\r' {
            at - 1
        } else {
            at
        };
        let run = rest[at + 1..]
            .find(|c: char| !is_js_space(c))
            .map_or(rest.len(), |end| at + 1 + end);
        if let Some(last) = rest[at + 1..run].rfind('\n') {
            result.push_str(&rest[..start]);
            result.push_str(newline);
            rest = &rest[at + 1 + last + 1..];
        } else {
            result.push_str(&rest[..=at]);
            rest = &rest[at + 1..];
        }
    }
    result.push_str(rest);
    result
}

#[cfg(test)]
mod tests {
    use super::{has_empty_line, remove_empty_lines};

    #[test]
    fn finds_and_removes_empty_lines() {
        assert!(has_empty_line("\n \t\n  ") && !has_empty_line("\n  ") && !has_empty_line(""));
        assert_eq!(remove_empty_lines("\n\n\n  ", "\n"), "\n  ");
        assert_eq!(remove_empty_lines("\r\n\r\n  ", "\r\n"), "\r\n  ");
        assert_eq!(remove_empty_lines("  \n ", "\n"), "  \n ");
    }
}
