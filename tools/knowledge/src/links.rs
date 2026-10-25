//! `knowledge link-ids <paths>` (EN-154): the links and the paths to a backlog file, gone into
//! the records, written as the item's short id, which `knowledge show` opens (E49, Decisions,
//! References). A link `[text](…/backlog/stories/E49-F48-EN-151-….md)` becomes `EN-151`, its text
//! kept when it says more; a bare path `docs/backlog/features/E49-F48-….md` becomes `F48`; a link to
//! an index of the backlog keeps its text alone. The id is read from the file's name, the form the
//! backlog gave every file, so that no database is needed.

use crate::backlog::family;

/// The folders of the backlog that moved into the records.
const MOVED: [&str; 3] = ["backlog/stories/", "backlog/features/", "backlog/epics/"];

/// The id a backlog file's name gives: `E49-F48-EN-151-topic.md` → `EN-151`, `E49-F48-topic.md`
/// → `F48`, `E49-topic.md` → `E49`; `None` for any other name.
#[must_use]
pub fn id_of_file(name: &str) -> Option<String> {
    let stem = name.strip_suffix(".md")?;
    let parts: Vec<&str> = stem.split('-').collect();
    let epic = *parts.first()?;

    family(epic)?;

    let rest = &parts[1..];
    let (feature, rest) = match rest.first() {
        Some(part)
            if family(part).is_some_and(|found| found == crate::backlog::Family::Feature) =>
        {
            (Some(*part), &rest[1..])
        }
        _ => (None, rest),
    };

    if let [kind, number, ..] = rest {
        let story = format!("{kind}-{number}");

        if family(&story).is_some() {
            return Some(story);
        }
    }
    if rest.is_empty() {
        return None;
    }
    Some(feature.unwrap_or(epic).to_owned())
}

/// What a link target to the backlog names: the id of its file, or `None` for an index or a folder
/// of the backlog; `Err(())` when it is no target into the moved folders at all.
fn target_id(target: &str) -> Result<Option<String>, ()> {
    let path = target.split('#').next().unwrap_or_default();
    let Some(at) = MOVED
        .iter()
        .find_map(|folder| path.find(folder).map(|found| found + folder.len()))
    else {
        return Err(());
    };
    let name = &path[at..];

    if name.is_empty() || name == "README.md" {
        return Ok(None);
    }
    if name.contains('/') {
        return Err(());
    }
    id_of_file(name).map(Some).ok_or(())
}

/// Whether a text holds a word, cut at every character but a letter, a digit or a dash.
fn holds_word(text: &str, word: &str) -> bool {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .any(|part| part == word)
}

/// The text a link to an item becomes: its id when the link's text is the id or a file's name, the
/// text when it holds the id, else the text and the id after it.
#[must_use]
pub fn link_text(text: &str, id: &str) -> String {
    let bare = text.trim().trim_matches('`');

    let is_file = std::path::Path::new(bare)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));

    if bare == id || is_file || bare.is_empty() {
        id.to_owned()
    } else if holds_word(text, id) {
        text.to_owned()
    } else {
        format!("{text} ({id})")
    }
}

/// Where the `[` opening a link's text sits, given where its `]` closes: brackets inside counted.
fn opening(text: &str, close: usize) -> Option<usize> {
    let mut depth = 0_usize;

    for (at, character) in text[..close].char_indices().rev() {
        match character {
            ']' => depth += 1,
            '[' if depth == 0 => return Some(at),
            '[' => depth -= 1,
            '\n' => return None,
            _ => {}
        }
    }
    None
}

/// The links of a text to the backlog's moved files written as ids.
fn rewrite_markdown_links(text: &str) -> String {
    let mut out = String::new();
    let mut from = 0;
    let mut search = 0;

    while let Some(found) = text[search..].find("](") {
        let close = search + found;
        let target_start = close + 2;
        let Some(length) = text[target_start..].find(')') else {
            break;
        };
        let target = &text[target_start..target_start + length];
        let end = target_start + length + 1;

        search = target_start;
        if target.contains(char::is_whitespace) {
            continue;
        }

        let Ok(id) = target_id(target) else {
            continue;
        };
        let Some(open) = opening(text, close).filter(|open| *open >= from) else {
            continue;
        };
        let label = &text[open + 1..close];

        // A link alone between parentheses, `([feature](…))`, keeps its text before the id rather
        // than nest a second pair: `(feature F19)`.
        let is_parenthesized = text[..open].ends_with('(') && text[end..].starts_with(')');

        out.push_str(&text[from..open]);
        match id {
            Some(id) => {
                let written = link_text(label, &id);

                if is_parenthesized && written.ends_with(&format!(" ({id})")) {
                    out.push_str(label);
                    out.push(' ');
                    out.push_str(&id);
                } else {
                    out.push_str(&written);
                }
            }
            None => out.push_str(label),
        }
        from = end;
        search = end;
    }
    out.push_str(&text[from..]);
    out
}

/// Whether a character may be part of a bare path.
fn is_path_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '.' | '/' | '-' | '_' | '#')
}

/// The bare paths of a text to the backlog's moved files written as ids: a run of path characters
/// holding `backlog/<folder>/<file>.md`, an anchor after it left out.
fn rewrite_bare_paths(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;

    while let Some(found) = MOVED.iter().filter_map(|folder| rest.find(folder)).min() {
        let start = rest[..found]
            .char_indices()
            .rev()
            .take_while(|(_, character)| is_path_character(*character))
            .last()
            .map_or(found, |(at, _)| at);
        let after = &rest[found..];
        let length = after
            .char_indices()
            .find(|(_, character)| !is_path_character(*character))
            .map_or(after.len(), |(at, _)| at);
        let token = &rest[start..found + length];
        let path = token.trim_end_matches('.');
        let path_end = start + path.len();

        match target_id(path) {
            Ok(Some(id)) => {
                out.push_str(&rest[..start]);
                out.push_str(&id);
            }
            _ => out.push_str(&rest[..path_end]),
        }
        rest = &rest[path_end..];
    }
    out.push_str(rest);
    out
}

/// A text with its links and bare paths to the backlog's moved files written as ids.
#[must_use]
pub fn with_ids(text: &str) -> String {
    rewrite_bare_paths(&rewrite_markdown_links(text))
}
