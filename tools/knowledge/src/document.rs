//! A Markdown text read as a document of the knowledge (research note 0035, Its tables): its
//! front matter's fields, its sections cut at its headings and the ids it cites. The headings and
//! the fenced blocks are found by `page`'s functions, the front matter by `backlog-index`'s, so
//! that the three tools read a page alike.

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
    let _ = text;
    Vec::new()
}

/// Whether a word is an id the project cites: `REF-` and upper-case parts (`REF-GIT-BRANCH`),
/// `DERIV-` and lower-case ones (`DERIV-bit`), an epic or a feature (`E49`, `F48`), or two
/// capitals or more, a dash and three digits or more, then dashed numbers (`EN-152`, `ADR-0081`,
/// `JOURNAL-2026-10-25-3`).
#[must_use]
pub fn is_citation(word: &str) -> bool {
    let _ = word;
    false
}

/// The ids a text cites, each once, in the order first met: its words, cut at every character but
/// a letter, a digit or a dash, their outer dashes left out, that [`is_citation`] accepts.
#[must_use]
pub fn citations(text: &str) -> Vec<String> {
    let _ = text;
    Vec::new()
}

/// A text read as a document: its front matter's fields, its title, its sections and its
/// citations, its own id left out of them.
#[must_use]
pub fn read_document(text: &str) -> Document {
    let _ = text;
    Document {
        id: None,
        title: String::new(),
        epic: String::new(),
        feature: String::new(),
        status: String::new(),
        date: None,
        fields: Vec::new(),
        sections: Vec::new(),
        citations: Vec::new(),
    }
}
