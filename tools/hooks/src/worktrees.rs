//! The stories' worktrees inside a checkout (EN-052), read by the hooks in one place (AUD-033):
//! the folder `worktree::WORKTREES`, the worktree a path is inside, and the one a worker's prompt
//! names.

use worktree::WORKTREES;

/// Where each worktree a text names starts, after `.claude/worktrees/`, with its folder's name:
/// letters, digits and `-`; a folder named without one is skipped.
fn named(text: &str) -> Vec<(usize, &str)> {
    let marker = format!("{WORKTREES}/");

    text.match_indices(&marker)
        .filter_map(|(index, _)| {
            let start = index + marker.len();
            let rest = &text[start..];
            let end = rest
                .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
                .unwrap_or(rest.len());

            (end > 0).then(|| (start, &rest[..end]))
        })
        .collect()
}

/// The folder name of the first worktree a text names, `None` when it names none.
#[must_use]
pub fn worktree_id(text: &str) -> Option<&str> {
    named(text).first().map(|(_, id)| *id)
}

/// The worktree a folder is inside, its root: the last `/.claude/worktrees/<name>` of the path
/// before a line break.
#[must_use]
pub fn worktree_root(folder: &str) -> Option<&str> {
    let before_break = folder
        .find(['\n', '\r', '\u{2028}', '\u{2029}'])
        .unwrap_or(folder.len());

    let marker = format!("/{WORKTREES}/");

    named(folder)
        .into_iter()
        .rev()
        .find(|(start, _)| *start <= before_break && folder[..*start].ends_with(&marker))
        .map(|(start, id)| &folder[..start + id.len()])
}

/// The worktree a worker's prompt names: the folder of the brief's `cd "<worktree>" || exit`
/// line, the first one of the text, quoted as the prompt writes it or as its transcript's JSON
/// escapes it; else the first worktree the text names.
#[must_use]
pub fn prompt_worktree(text: &str) -> Option<&str> {
    text.match_indices("cd ")
        .find_map(|(at, call)| {
            let rest = &text[at + call.len()..];
            let (quote, rest) = rest
                .strip_prefix("\\\"")
                .map(|rest| ("\\\"", rest))
                .or_else(|| rest.strip_prefix('"').map(|rest| ("\"", rest)))?;
            let (folder, after) = rest.split_once(quote)?;

            after
                .trim_start()
                .starts_with("|| exit")
                .then(|| worktree_id(folder))
                .flatten()
        })
        .or_else(|| worktree_id(text))
}
