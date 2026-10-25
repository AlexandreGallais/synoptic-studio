//! `knowledge link-ids <paths>` (EN-154, E49): the links and the paths to a moved file — a backlog
//! item, a decision, a note, a derivation, the bibliography, a log, a guide page — gone into
//! the records, written as the item's short id, which `knowledge show` opens (E49, Decisions,
//! References). A link `[text](…/backlog/stories/E49-F48-EN-151-….md)` becomes `EN-151`, its text
//! kept when it says more; a bare path `docs/backlog/features/E49-F48-….md` becomes `F48`; a link to
//! an index of the backlog keeps its text alone. The id is read from the file's name, the form the
//! backlog gave every file, so that no database is needed.

use crate::backlog::family;
use crate::moved::{guide_id, log_id};
use crate::rules::{page_of, rule_id};

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

/// The folders and files of the docs that moved into the records: the backlog's (EN-154), then
/// the decisions, derivations, research, bibliography, logs and guide (E49, the move).
const MOVED: [&str; 9] = [
    "backlog/stories/",
    "backlog/features/",
    "backlog/epics/",
    "adr/",
    "derivations/",
    "research/",
    "references.md",
    "process/",
    "guide/",
];

/// The logs of `docs/process/` that moved; its other pages stay.
const LOGS: [&str; 5] = [
    "journal.md",
    "history.md",
    "improvements.md",
    "feedback.md",
    "tool-incidents.md",
];

/// The four digits a numbered file's name opens with, then a dash.
fn numbered(name: &str) -> Option<&str> {
    let digits = name.get(..4)?;

    (digits.bytes().all(|byte| byte.is_ascii_digit()) && name[4..].starts_with('-'))
        .then_some(digits)
}

/// The id a file or a folder of the moved docs gets, given its moved folder and what follows it:
/// `Ok(None)` for a folder or an index whose link keeps its text alone, `Err(())` for a path that
/// did not move or names nothing.
fn moved_id(folder: &str, rest: &str) -> Result<Option<String>, ()> {
    let is_index = rest.is_empty() || rest == "README.md";
    let stem = rest.strip_suffix(".md").filter(|stem| !stem.contains('/'));

    match folder {
        "adr/" if is_index => Ok(Some("PAGE-adr".to_owned())),
        "adr/" => numbered(rest)
            .map(|number| Some(format!("ADR-{number}")))
            .ok_or(()),
        "derivations/" if is_index => Ok(Some("PAGE-derivations".to_owned())),
        "derivations/" => stem.map(|stem| Some(format!("DERIV-{stem}"))).ok_or(()),
        "research/" if is_index => Ok(Some("PAGE-research".to_owned())),
        "research/" => match rest.split_once('/') {
            Some(("notes" | "requests", "")) => Ok(None),
            Some(("notes", name)) => numbered(name).map(|n| Some(format!("NOTE-{n}"))).ok_or(()),
            Some(("requests", "_template.md")) => {
                Ok(Some("PAGE-research-requests-template".to_owned()))
            }
            Some(("requests", name)) => numbered(name)
                .map(|n| Some(format!("REQUEST-{n}")))
                .ok_or(()),
            _ => Err(()),
        },
        "references.md" if rest.is_empty() => Ok(Some("PAGE-references".to_owned())),
        "process/" if LOGS.contains(&rest) => Ok(Some(log_id(rest))),
        "process/" => match rest.strip_prefix("retrospectives/") {
            Some("" | "README.md") => Ok(Some("PAGE-retrospectives".to_owned())),
            Some(name) => name
                .strip_suffix("-retrospective.md")
                .filter(|epic| !epic.contains('/'))
                .map(|epic| Some(format!("RETRO-{epic}")))
                .ok_or(()),
            None => Err(()),
        },
        "guide/" if is_index => Ok(Some("PAGE-guide".to_owned())),
        "guide/" if stem.is_some() => Ok(Some(guide_id(rest))),
        "backlog/stories/" | "backlog/features/" | "backlog/epics/" if is_index => Ok(None),
        "backlog/stories/" | "backlog/features/" | "backlog/epics/" if stem.is_some() => {
            id_of_file(rest).map(Some).ok_or(())
        }
        _ => Err(()),
    }
}

/// What a link target to the moved docs names: the id of its file, or `None` for an index or a
/// folder whose link keeps its text; `Err(())` when it is no target into the moved docs at all,
/// an address of the web among them. A moved folder is found where a path's part starts.
fn target_id(target: &str) -> Result<Option<String>, ()> {
    let path = target.split('#').next().unwrap_or_default();

    if path.contains("://") {
        return Err(());
    }

    let found = MOVED
        .iter()
        .filter_map(|folder| {
            path.match_indices(folder)
                .find(|(at, _)| *at == 0 || path[..*at].ends_with('/'))
                .map(|(at, _)| (at, *folder))
        })
        .min();
    let Some((at, folder)) = found else {
        return Err(());
    };

    moved_id(folder, &path[at + folder.len()..])
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

        // A bare path names the docs by their folder, `docs/adr/…`: a word `derivations/` alone
        // in a sentence is no path.
        let is_docs = MOVED
            .iter()
            .any(|folder| path.contains(&format!("docs/{folder}")));

        match target_id(path) {
            Ok(Some(id)) if is_docs => {
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
    with_rule_ids(&rewrite_bare_paths(&rewrite_markdown_links(text)))
}

/// A `@see` line's citations of a domain page written as the rule's id (E49): the page and its
/// anchor, `docs/domain/shapes.md#_2-corner-radius` → `RULE-shapes-2-corner-radius`, the page
/// alone `RULE-shapes`. Elsewhere a domain page, which stays for the Product Owner, keeps its path.
#[must_use]
pub fn with_rule_ids(text: &str) -> String {
    const PAGE: &str = "docs/domain/";

    text.split_inclusive('\n')
        .map(|line| {
            if !line.contains("@see") {
                return line.to_owned();
            }

            let mut out = String::new();
            let mut rest = line;

            while let Some(at) = rest.find(PAGE) {
                let after = &rest[at + PAGE.len()..];
                let length = after
                    .find(|character: char| !is_path_character(character))
                    .unwrap_or(after.len());
                let token = after[..length].trim_end_matches('.');
                let (file, anchor) = token.split_once('#').unwrap_or((token, ""));

                out.push_str(&rest[..at]);
                match file.strip_suffix(".md").filter(|stem| !stem.contains('/')) {
                    Some(_) => out.push_str(&rule_id(&page_of(file), anchor)),
                    None => out.push_str(&rest[at..at + PAGE.len() + token.len()]),
                }
                rest = &after[token.len()..];
            }
            out.push_str(rest);
            out
        })
        .collect()
}
