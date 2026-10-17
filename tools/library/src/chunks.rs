//! Chunks and freshness of the local source library (`docs/tooling/sources.md`): a page is split
//! at its headings, so that a search returns a section and a reading costs only that section; the
//! freshness rule says when a stored page is read again online. The cut is the one
//! `library-chunks.ts` made, lengths counted in UTF-16 code units as JavaScript counts them.

use crate::text;

/// One section of a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    /// Heading path of the section, e.g. `Hooks › PostToolUse › Input`.
    pub heading: String,
    /// Its position in the page, from 0.
    pub index: i64,
    /// Its Markdown text, heading line included.
    pub text: String,
}

/// What the library knows of a stored page's age, epoch milliseconds each.
#[derive(Clone, Copy, Debug)]
pub struct PageAge {
    /// When the library last read it online.
    pub fetched_at: i64,
    /// When the page says it last changed, 0 when it does not say.
    pub modified_at: i64,
    /// Time now.
    pub now: i64,
}

/// A section being read: its heading path and its lines.
struct Section<'a> {
    heading: Vec<&'a str>,
    lines: Vec<&'a str>,
}

/// One day, in milliseconds.
pub const DAY_MS: i64 = 86_400_000;

/// A page changed less than this before it was read is young: it may still be corrected.
const YOUNG_DAYS: i64 = 14;

/// A young page is read again after this many days.
const YOUNG_REFRESH_DAYS: i64 = 1;

/// Any page is read again after this many days.
const STABLE_REFRESH_DAYS: i64 = 90;

/// Longest section kept in one chunk, in UTF-16 code units; a longer one is cut at blank lines.
const MAX_CHUNK: usize = 4000;

/// Level of a Markdown heading line: 1 to 6 for `#` to `######` followed by a space, 0 for any
/// other line.
fn heading_level(line: &str) -> usize {
    let hashes = line.bytes().take_while(|byte| *byte == b'#').count();

    if (1..=6).contains(&hashes) && line.as_bytes().get(hashes) == Some(&b' ') {
        hashes
    } else {
        0
    }
}

/// Parts of a section no longer than `MAX_CHUNK`, cut between paragraphs: each paragraph joins
/// the part being built while the two stay shorter than `MAX_CHUNK`, else opens the next part.
fn parts(text: &str) -> Vec<String> {
    let mut paragraphs = text.split("\n\n");
    let mut part = paragraphs.next().unwrap_or_default().to_owned();
    let mut done = Vec::new();

    for next in paragraphs {
        if text::length(&part) + text::length(next) < MAX_CHUNK {
            part = format!("{part}\n\n{next}");
        } else {
            done.push(std::mem::replace(&mut part, next.to_owned()));
        }
    }

    done.push(part);

    done
}

/// A page split into sections at its headings, in page order, each with its heading path; text
/// before the first heading is a chunk of empty heading. A `#` line inside a code block is text.
#[must_use]
pub fn page_chunks(markdown: &str) -> Vec<Chunk> {
    let mut sections: Vec<Section<'_>> = Vec::new();
    let mut is_fence = false;

    for line in markdown.split('\n') {
        let level = if is_fence { 0 } else { heading_level(line) };

        is_fence = line.starts_with("```") != is_fence;

        if level > 0 {
            let mut heading: Vec<&str> = sections
                .last()
                .map(|last| last.heading.iter().take(level - 1).copied().collect())
                .unwrap_or_default();

            heading.push(&line[level + 1..]);
            sections.push(Section {
                heading,
                lines: vec![line],
            });
        } else if let Some(last) = sections.last_mut() {
            last.lines.push(line);
        } else {
            sections.push(Section {
                heading: Vec::new(),
                lines: vec![line],
            });
        }
    }

    sections
        .iter()
        .flat_map(|section| {
            let heading = section.heading.join(" › ");

            parts(text::trim(&section.lines.join("\n")))
                .into_iter()
                .map(move |text| (heading.clone(), text))
        })
        .filter(|(_, text)| !text.is_empty())
        .zip(0..)
        .map(|((heading, text), index)| Chunk {
            heading,
            index,
            text,
        })
        .collect()
}

/// Whether a stored page must be read again online before use. A page changed shortly before it
/// was read may still be corrected: it is read again after a day. Any other page is read again
/// after 90 days.
#[must_use]
pub fn is_stale(age: PageAge) -> bool {
    let is_young = age.modified_at > 0 && age.fetched_at - age.modified_at < YOUNG_DAYS * DAY_MS;
    let limit = if is_young {
        YOUNG_REFRESH_DAYS
    } else {
        STABLE_REFRESH_DAYS
    } * DAY_MS;

    age.now - age.fetched_at > limit
}
