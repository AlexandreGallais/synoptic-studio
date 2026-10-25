//! A Markdown text read as a document of the knowledge (research note 0035, Its tables): its
//! front matter's fields, its sections cut at its headings and the ids it cites. The headings and
//! the fenced blocks are found by `page`'s functions, the front matter by its own module,
//! so that the tools read a page alike.

use crate::front_matter::front_matter;
use page::fence::fenced;
use page::section::heading;

/// A part of a document, from one heading down to the next heading of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    /// Its heading's text, `#` marks left out; empty for the text above the first heading.
    pub heading: String,
    /// Its lines as written, its heading's line included, each with its line end.
    pub text: String,
}

/// What a text gives a row of `documents` and the rows of its tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// The front matter's `id`, `None` without one: the store then makes it.
    pub id: Option<String>,
    /// The front matter's `title`, else the first heading's text, else the first line's.
    pub title: String,
    /// The front matter's `epic`, empty without one.
    pub epic: String,
    /// The front matter's `feature`, empty without one.
    pub feature: String,
    /// The front matter's `status`, empty without one.
    pub status: String,
    /// The front matter's `date`, `None` without one.
    pub date: Option<String>,
    /// Every field of the front matter, sorted by key.
    pub fields: Vec<(String, String)>,
    /// The sections, in the text's order: joined, they give the text back.
    pub sections: Vec<Section>,
    /// The ids the text cites, each once, in the order first met.
    pub citations: Vec<String>,
}

/// The sections of a text, cut before each heading outside a fenced block; the text above the
/// first heading is a section of an empty heading, left out when empty. Joined, they are the text.
#[must_use]
pub fn sections_of(text: &str) -> Vec<Section> {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let bare: Vec<&str> = lines
        .iter()
        .map(|line| line.trim_end_matches(['\n', '\r']))
        .collect();
    let mut sections = Vec::new();
    let mut current = Section {
        heading: String::new(),
        text: String::new(),
    };

    for ((line, bare), is_fenced) in lines.iter().zip(&bare).zip(fenced(&bare)) {
        if let Some((_, title)) = heading(bare).filter(|_| !is_fenced) {
            let next = Section {
                heading: title.to_owned(),
                text: String::new(),
            };

            sections.push(std::mem::replace(&mut current, next));
        }
        current.text.push_str(line);
    }
    sections.push(current);
    sections.retain(|section| !section.text.is_empty());
    sections
}

/// Whether a word is an id the project cites: `REF-` and upper-case parts (`REF-GIT-BRANCH`),
/// `DERIV-` and lower-case ones (`DERIV-bit`), an epic or a feature (`E49`, `F48`), or two
/// capitals or more, a dash and three digits or more, then dashed numbers (`EN-152`, `ADR-0081`,
/// `JOURNAL-2026-10-25-3`).
#[must_use]
pub fn is_citation(word: &str) -> bool {
    let all = |part: &str, accepted: fn(&u8) -> bool| {
        !part.is_empty() && part.bytes().all(|byte| accepted(&byte))
    };
    let upper = |part: &&str| {
        all(part, |byte| {
            byte.is_ascii_uppercase() || byte.is_ascii_digit()
        })
    };
    let lower = |part: &&str| {
        all(part, |byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit()
        })
    };
    let digits = |part: &str| all(part, u8::is_ascii_digit);
    let parts: Vec<&str> = word.split('-').collect();

    match parts.as_slice() {
        ["REF", rest @ ..] => !rest.is_empty() && rest.iter().all(upper),
        ["DERIV", rest @ ..] => !rest.is_empty() && rest.iter().all(lower),
        [single] => single
            .split_at_checked(1)
            .is_some_and(|(letter, number)| matches!(letter, "E" | "F") && digits(number)),
        [kind, number, rest @ ..] => {
            kind.len() >= 2
                && all(kind, u8::is_ascii_uppercase)
                && number.len() >= 3
                && digits(number)
                && rest.iter().all(|part| digits(part))
        }
        [] => false,
    }
}

/// The ids a text cites, each once, in the order first met: its words, cut at every character but
/// a letter, a digit or a dash, their outer dashes left out, that [`is_citation`] accepts.
#[must_use]
pub fn citations(text: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();

    for word in
        text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
    {
        let word = word.trim_matches('-');

        if is_citation(word) && !found.iter().any(|known| known == word) {
            found.push(word.to_owned());
        }
    }
    found
}

/// A text read as a document: its front matter's fields, its title, its sections and its
/// citations, its own id left out of them.
#[must_use]
pub fn read_document(text: &str) -> Document {
    let mut fields: Vec<(String, String)> = front_matter(text).into_iter().collect();

    fields.sort();

    let field = |key: &str| {
        fields
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
    };
    let sections = sections_of(text);
    let id = field("id");
    let title = field("title")
        .or_else(|| {
            sections
                .iter()
                .find(|section| !section.heading.is_empty())
                .map(|section| section.heading.clone())
        })
        .or_else(|| {
            text.lines()
                .map(str::trim)
                .find(|line| !line.is_empty())
                .map(str::to_owned)
        })
        .unwrap_or_default();
    let citations = citations(text)
        .into_iter()
        .filter(|cited| Some(cited) != id.as_ref())
        .collect();

    Document {
        title,
        epic: field("epic").unwrap_or_default(),
        feature: field("feature").unwrap_or_default(),
        status: field("status").unwrap_or_default(),
        date: field("date"),
        id,
        fields,
        sections,
        citations,
    }
}
