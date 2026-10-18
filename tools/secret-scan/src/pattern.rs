//! A pattern of the preset with its look-arounds, which Rust's `regex` lacks (research note 0025,
//! The rewritten packages): each tests one character before or after the match, so each is
//! checked so that the matches are the ones JavaScript's `matchAll` finds.
//!
//! - **The look-behind `(?<!\p{L})`**, at the start of 25 patterns, is checked in code: a match
//!   preceded by a letter is dropped and the search goes on one character after its start, as
//!   JavaScript tries the next position.
//! - **The look-ahead `(?![…])`**, at their end, becomes `(?:[^…]|\z)`, one character more or the
//!   end of the text: the leftmost-first match of the whole is JavaScript's backtracking match
//!   followed by that character, so the match proper is read from the group `m`, and the next
//!   search starts at its end.

use regex::{Captures, Regex};

use crate::js::{compile, is_letter};

/// The name of the group holding the match proper, the look-ahead's character left out.
const MATCH: &str = "m";

/// A compiled pattern and its look-behind.
#[derive(Debug)]
pub struct Pattern {
    /// `(?P<m>…)` and, when the pattern has a look-ahead, `(?:[^…]|\z)`.
    regex: Regex,
    /// Whether a letter before the match refuses it, `(?<!\p{L})`.
    not_after_letter: bool,
}

/// One match: where it starts and ends in the text, in bytes, and its groups.
#[derive(Debug)]
pub struct Found<'t> {
    /// The byte where the match starts.
    pub start: usize,
    /// The byte after its end.
    pub end: usize,
    /// Its groups, by number or name.
    captures: Captures<'t>,
}

impl<'t> Found<'t> {
    /// The text of a named group, `None` when it took no part in the match.
    #[must_use]
    pub fn group(&self, name: &str) -> Option<&'t str> {
        self.captures.name(name).map(|group| group.as_str())
    }

    /// The start and end bytes of a named group.
    #[must_use]
    pub fn span(&self, name: &str) -> Option<(usize, usize)> {
        self.captures
            .name(name)
            .map(|group| (group.start(), group.end()))
    }
}

impl Pattern {
    /// A pattern without look-around: `core` as it is, Rust's syntax.
    #[must_use]
    pub fn plain(core: &str) -> Self {
        Self::with(core, false, None)
    }

    /// A pattern `(?<!\p{L})core(?![ahead])`: `ahead` is the inside of the class the look-ahead
    /// refuses.
    #[must_use]
    pub fn bounded(core: &str, ahead: &str) -> Self {
        Self::with(core, true, Some(ahead))
    }

    /// The pattern `core`, refused after a letter when `not_after_letter`, and before a character
    /// of the class `ahead` when there is one.
    #[must_use]
    pub fn with(core: &str, not_after_letter: bool, ahead: Option<&str>) -> Self {
        let tail = ahead.map_or_else(String::new, |class| format!(r"(?:[^{class}]|\z)"));

        Self {
            regex: compile(&format!("(?P<{MATCH}>{core}){tail}")),
            not_after_letter,
        }
    }

    /// Every match in the text, as JavaScript's `matchAll` of the same pattern with the `g` flag
    /// finds them: leftmost first, each search starting where the last match ended.
    #[must_use]
    pub fn find_all<'t>(&self, text: &'t str) -> Vec<Found<'t>> {
        let mut found = Vec::new();
        let mut from = 0;

        while from <= text.len() {
            let Some(captures) = self.regex.captures_at(text, from) else {
                break;
            };
            let Some((start, end)) = captures.name(MATCH).map(|m| (m.start(), m.end())) else {
                break;
            };

            if self.not_after_letter && text[..start].chars().next_back().is_some_and(is_letter) {
                from = start + next_len(text, start);
                continue;
            }
            from = if end == start {
                end + next_len(text, end)
            } else {
                end
            };
            found.push(Found {
                start,
                end,
                captures,
            });
        }
        found
    }

    /// Whether the pattern matches anywhere in the text, JavaScript's `test`.
    #[must_use]
    pub fn test(&self, text: &str) -> bool {
        !self.find_all(text).is_empty()
    }
}

/// The length in bytes of the character at `at`, one past the text's end.
fn next_len(text: &str, at: usize) -> usize {
    text[at..].chars().next().map_or(1, char::len_utf8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_letter_before_the_match_refuses_it_and_the_search_goes_on() {
        // "xab ab": the first `ab` follows the letter x, the second a space: one match, at 4.
        let pattern = Pattern::bounded("ab", "a-z");
        let found = pattern.find_all("xab ab");

        assert_eq!(found.iter().map(|f| f.start).collect::<Vec<_>>(), [4]);
    }

    #[test]
    fn a_non_letter_or_the_start_lets_the_match_be() {
        // `é` is a letter (U+00E9, Ll): refused; `1` and the text's start are not.
        let pattern = Pattern::bounded("ab", "a-z");

        assert_eq!(pattern.find_all("ab").len(), 1);
        assert_eq!(pattern.find_all("1ab").len(), 1);
        assert!(pattern.find_all("éab").is_empty());
    }

    #[test]
    fn the_look_ahead_backtracks_as_javascript_does() {
        // /(?<!\p{L})a{1,3}(?![a])b?/ on "aaab": greedy a{1,3} takes "aaa", next is "b", not
        // an `a`: the match is "aaa", the `b` being outside the class. On "aaaa" every length
        // is followed by an `a`, at start 0; from 1 the look-behind sees `a`, a letter: none.
        let pattern = Pattern::bounded("a{1,3}", "a");

        assert_eq!(pattern.find_all("aaab")[0].end, 3);
        assert!(pattern.find_all("aaaa").is_empty());
    }

    #[test]
    fn the_character_the_look_ahead_reads_is_left_for_the_next_match() {
        // "ab-ab": the first `ab` is followed by `-`, read by `(?:[^a-z]|\z)` but not taken: the
        // next search starts at 2 and finds the second at 3.
        let pattern = Pattern::with("ab", false, Some("a-z"));

        assert_eq!(
            pattern
                .find_all("ab-ab")
                .iter()
                .map(|f| (f.start, f.end))
                .collect::<Vec<_>>(),
            [(0, 2), (3, 5)]
        );
    }

    #[test]
    fn an_empty_match_moves_the_search_one_character_on() {
        // `x*` on "éa": empty at 0, at 2 (after é, two bytes), at 3: three matches, no loop.
        let found = Pattern::plain("x*").find_all("éa");

        assert_eq!(found.iter().map(|f| f.start).collect::<Vec<_>>(), [0, 2, 3]);
    }

    #[test]
    fn groups_are_read_by_name() {
        let pattern = Pattern::plain("(?P<word>[a-z]+)=(?P<value>[0-9]+)?");
        let found = pattern.find_all("key=");

        assert_eq!(found[0].group("word"), Some("key"));
        assert_eq!(found[0].group("value"), None);
        assert_eq!(found[0].span("word"), Some((0, 3)));
        assert!(pattern.test("a=1"));
        assert!(!pattern.test("=1"));
    }
}
