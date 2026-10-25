//! A pull request's body into `main` (ADR-0086, `the records' conventions, area release`): the text of the
//! features it brings, read up to its first thematic break, then joined with the other bodies of
//! a release into its notes.

use crate::draft::{BREAKING_MARK, GROUPS, Group, NO_CHANGE_NOTICE};

/// A release text read: its notice, and each change with its group, in the order written.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Text {
    /// Its notice, `_…_`, when it holds one.
    pub notice: Option<String>,
    /// Each change without its `- `, after its group.
    pub changes: Vec<(Group, String)>,
}

/// The line that ends the release text of a body: a thematic break.
const BREAK: &str = "---";

/// The part of a body the release reads: its line endings made `\n`, as GitHub may give `\r\n`,
/// cut before its first thematic break, a line `---`, after which the attribution and anything
/// else for the reviewer stay out of the notes.
#[must_use]
pub fn release_text(body: &str) -> String {
    let mut text = String::new();

    for line in body.lines().take_while(|line| line.trim_end() != BREAK) {
        text.push_str(line);
        text.push('\n');
    }
    text
}

/// A text written: its notice, then each group in the order of Common Changelog with its list of
/// changes (`REF-COMMON-CHANGELOG`, 2.3, 2.4); empty without either.
#[must_use]
pub fn written(notice: Option<&str>, changes: &[(Group, String)]) -> String {
    let mut paragraphs: Vec<String> = notice.map(str::to_owned).into_iter().collect();

    for (group, heading) in GROUPS {
        let listed: Vec<String> = changes
            .iter()
            .filter(|(of, _)| *of == group)
            .map(|(_, change)| format!("- {change}"))
            .collect();

        if !listed.is_empty() {
            paragraphs.push(format!("### {heading}\n\n{}", listed.join("\n")));
        }
    }
    if paragraphs.is_empty() {
        String::new()
    } else {
        format!("{}\n", paragraphs.join("\n\n"))
    }
}

/// A release text read, once `release check` passes on it.
#[must_use]
pub fn text(release_text: &str) -> Text {
    let mut read = Text::default();
    let mut group = None;

    for line in release_text.lines().filter(|line| !line.trim().is_empty()) {
        if let Some(name) = line.strip_prefix("### ") {
            group = GROUPS
                .iter()
                .find(|(_, heading)| *heading == name)
                .map(|(group, _)| *group);
        } else if let (Some(change), Some(group)) = (line.strip_prefix("- "), group) {
            read.changes.push((group, change.to_owned()));
        } else if group.is_none() && read.notice.is_none() {
            read.notice = Some(line.to_owned());
        }
    }
    read
}

/// The notes of a release from the texts of its pull requests, the latest first: each group's
/// changes joined, the breaking ones first; a notice kept when one text holds it, the notice of
/// a release without a change when none lists one.
///
/// # Errors
///
/// Two pull requests hold a notice other than that one: a release holds one notice at most.
pub fn joined(texts: &[(u32, Text)]) -> Result<String, String> {
    let mut notice: Option<(u32, &str)> = None;

    for (number, text) in texts {
        match (text.notice.as_deref(), notice) {
            (None | Some(NO_CHANGE_NOTICE), _) => {}
            (Some(own), None) => notice = Some((*number, own)),
            (Some(own), Some((_, kept))) if own == kept => {}
            (Some(_), Some((first, _))) => {
                return Err(format!(
                    "pull requests #{first} and #{number} each hold a notice: a release holds \
                     one at most, so write it in one body"
                ));
            }
        }
    }

    let mut changes: Vec<(Group, String)> = texts
        .iter()
        .flat_map(|(_, text)| text.changes.iter().cloned())
        .collect();

    changes.sort_by_key(|(_, change)| !change.starts_with(BREAKING_MARK));

    let notice = notice
        .map(|(_, own)| own)
        .or_else(|| changes.is_empty().then_some(NO_CHANGE_NOTICE));

    Ok(written(notice, &changes))
}
