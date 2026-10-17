//! The sections of a page, found by their heading, and its first section.

use crate::fence::fenced;

/// A section of a page: the number of its heading's line, counted from 1, and its text, from its
/// heading down to the next heading of the same or a higher level, its trailing blank lines left
/// out.
#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub line: usize,
    pub text: String,
}

/// The level and the text of an ATX heading, `## Hooks` giving `(2, "Hooks")`; `None` for any
/// other line. Six `#` at most, followed by a space or nothing.
#[must_use]
pub fn heading(line: &str) -> Option<(usize, &str)> {
    let level = line.chars().take_while(|char| *char == '#').count();
    let rest = &line[level..];

    if level == 0 || level > 6 || !(rest.is_empty() || rest.starts_with(' ')) {
        return None;
    }

    Some((level, rest.trim()))
}

/// The headings of the lines, by line index, outside the fenced code blocks.
fn headings<'a>(lines: &[&'a str]) -> Vec<Option<(usize, &'a str)>> {
    lines
        .iter()
        .zip(fenced(lines))
        .map(|(line, fenced)| if fenced { None } else { heading(line) })
        .collect()
}

/// The lines from `start` to `end`, excluded, its trailing blank lines left out, each ended by a
/// line feed.
fn joined(lines: &[&str], start: usize, end: usize) -> String {
    let mut last = end;

    while last > start && lines[last - 1].trim().is_empty() {
        last -= 1;
    }

    lines[start..last]
        .iter()
        .fold(String::new(), |text, line| text + line + "\n")
}

/// Whether a heading's text names `wanted`, as written or without its backticks: `` `mutate` ``
/// is found as `mutate`.
fn names(text: &str, wanted: &str) -> bool {
    text == wanted || text.replace('`', "") == wanted
}

/// Every section whose heading's text is `wanted`, in the page's order.
#[must_use]
pub fn sections(text: &str, wanted: &str) -> Vec<Section> {
    let lines: Vec<&str> = text.lines().collect();
    let found = headings(&lines);

    found
        .iter()
        .enumerate()
        .filter_map(|(start, heading)| {
            let (level, title) = (*heading)?;

            if !names(title, wanted) {
                return None;
            }

            let end = found
                .iter()
                .enumerate()
                .skip(start + 1)
                .find(|(_, next)| next.is_some_and(|(next_level, _)| next_level <= level))
                .map_or(lines.len(), |(index, _)| index);

            Some(Section {
                line: start + 1,
                text: joined(&lines, start, end),
            })
        })
        .collect()
}

/// The page's first section: its lines from the first down to its second heading, so the front
/// matter, the title and the text under it; the whole page when it has one heading or none.
#[must_use]
pub fn first_section(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let end = headings(&lines)
        .iter()
        .enumerate()
        .filter(|(_, heading)| heading.is_some())
        .nth(1)
        .map_or(lines.len(), |(index, _)| index);

    joined(&lines, 0, end)
}
