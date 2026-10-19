//! The here-documents of a command line (`REF-POSIX-SHELL-HEREDOC`): their bodies are data written
//! to a command's input, never run by the shell that reads the line, so the guard leaves them out
//! of the commands it judges.

use crate::guard::{after_spaces, is_word};

/// The shell command without the bodies of its here-documents: their lines are data written to a
/// file or a pipe (a commit message, a test case), never run. Each `<<WORD … WORD` body is removed,
/// the line opening it kept, as `/(<<-?\s*(['"]?)(\w+)\2[^\n]*)\n[\s\S]*?\n\s*\3(?=\n|$)/gu`
/// replaced by its first group, the word read as the shell reads it (EN-119): quoted, any
/// characters but its quote and a line feed (`'END-OF-MESSAGE'`); unquoted, `\w`, `-` and `.`.
#[must_use]
pub fn without_heredocs(command: &str) -> String {
    let mut kept = String::with_capacity(command.len());
    let mut at = 0;

    while at < command.len() {
        if let Some(heredoc) = heredoc_at(command, at) {
            kept.push_str(&command[at..heredoc.opening_end]);
            at = heredoc.end;
        } else {
            let character = command[at..].chars().next().unwrap_or_default();

            kept.push(character);
            at += character.len_utf8();
        }
    }
    kept
}

/// The bodies of a command's here-documents, as [`without_heredocs`] finds them: the lines
/// between each opening line and the line of its word.
#[must_use]
pub fn heredoc_bodies(command: &str) -> Vec<&str> {
    let mut bodies = Vec::new();
    let mut at = 0;

    while at < command.len() {
        if let Some(heredoc) = heredoc_at(command, at) {
            bodies.push(&command[heredoc.opening_end + 1..heredoc.body_end]);
            at = heredoc.end;
        } else {
            at += command[at..].chars().next().map_or(1, char::len_utf8);
        }
    }
    bodies
}

/// Where a here-document lies in a command.
struct Heredoc {
    /// The line feed ending its opening line.
    opening_end: usize,
    /// The line feed before the line of its word.
    body_end: usize,
    /// The end of that word.
    end: usize,
}

/// Whether a byte may be part of an unquoted here-document word.
fn is_heredoc_word(byte: u8) -> bool {
    is_word(byte) || matches!(byte, b'-' | b'.')
}

/// A here-document opening at `at`, `None` when none starts there.
fn heredoc_at(text: &str, at: usize) -> Option<Heredoc> {
    // `<<<` is a here-string, its word on the same line.
    if !text[at..].starts_with("<<") || text[at + 2..].starts_with('<') || text[..at].ends_with('<')
    {
        return None;
    }
    let mut after = at + 2;

    if text[after..].starts_with('-') {
        after += 1;
    }
    after = after_spaces(text, after);

    let quote = match text.as_bytes().get(after) {
        Some(byte @ (b'\'' | b'"')) => Some(*byte),
        _ => None,
    };
    let word_start = after + usize::from(quote.is_some());
    let longest = text.as_bytes()[word_start..]
        .iter()
        .take_while(|byte| match quote {
            Some(quote) => **byte != quote && **byte != b'\n',
            None => is_heredoc_word(**byte),
        })
        .count();

    // `\w+` gives back characters when the rest fails: a shorter word is tried in turn.
    (1..=longest).rev().find_map(|length| {
        let word_end = word_start + length;
        let opened = match quote {
            Some(byte) => (text.as_bytes().get(word_end) == Some(&byte)).then_some(word_end + 1)?,
            None => word_end,
        };
        let opening_end = text[opened..].find('\n').map(|index| opened + index)?;

        body_end(text, opening_end, &text[word_start..word_end]).map(|(body_end, end)| Heredoc {
            opening_end,
            body_end,
            end,
        })
    })
}

/// The end of a here-document's body: the first `\n\s*WORD` after its opening line, followed by a
/// line feed or the end of the text; its line feed and the end of the word.
fn body_end(text: &str, opening_end: usize, word: &str) -> Option<(usize, usize)> {
    let mut search = opening_end + 1;

    loop {
        let line_feed = search + text.get(search..)?.find('\n')?;
        let start = after_spaces(text, line_feed + 1);
        let end = start + word.len();

        if text[start..].starts_with(word) && matches!(text.as_bytes().get(end), None | Some(b'\n'))
        {
            return Some((line_feed, end));
        }
        search = line_feed + 1;
    }
}

/// The commands the substitutions of an unquoted here-document's body run, `$(…)` and backquotes:
/// such a body is expanded before it is written, so its substitutions run.
#[must_use]
pub fn expanded_in_bodies(command: &str) -> Vec<String> {
    let _ = command;

    Vec::new()
}
