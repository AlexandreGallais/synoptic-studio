//! A pull request's body into `main` (ADR-0086, `docs/tooling/releases.md`): the text of the
//! features it brings, read up to its first thematic break, then joined with the other bodies of
//! a release into its notes.

use crate::draft::Group;

/// A release text read: its notice, and each change with its group, in the order written.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Text {
    /// Its notice, `_…_`, when it holds one.
    pub notice: Option<String>,
    /// Each change without its `- `, after its group.
    pub changes: Vec<(Group, String)>,
}

/// The part of a body the release reads: its line endings made `\n`, as GitHub may give `\r\n`,
/// cut before its first thematic break, a line `---`, after which the attribution and anything
/// else for the reviewer stay out of the notes.
#[must_use]
pub fn release_text(body: &str) -> String {
    let _ = body;
    String::new()
}

/// A text written: its notice, then each group in the order of Common Changelog with its list of
/// changes (`REF-COMMON-CHANGELOG`, 2.3, 2.4); empty without either.
#[must_use]
pub fn written(notice: Option<&str>, changes: &[(Group, String)]) -> String {
    let _ = (notice, changes);
    String::new()
}

/// A release text read, once `release check` passes on it.
#[must_use]
pub fn text(release_text: &str) -> Text {
    let _ = release_text;
    Text::default()
}

/// The notes of a release from the texts of its pull requests, the latest first: each group's
/// changes joined, the breaking ones first; a notice kept when one text holds it, the notice of
/// a release without a change when none lists one.
///
/// # Errors
///
/// Two pull requests hold a notice other than that one: a release holds one notice at most.
pub fn joined(texts: &[(u32, Text)]) -> Result<String, String> {
    let _ = texts;
    Err(String::new())
}
