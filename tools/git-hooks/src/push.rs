//! What a push does, read from the refs Git gives the pre-push hook on its standard input, one line
//! each: `<local ref> <local sha> <remote ref> <remote sha>` (`REF-GITHOOKS`, `pre-push`). A
//! ref deleted has the zero sha as its local sha.
//!
//! A push that only deletes branches checks nothing: nothing new reaches the remote, and a
//! deletion once failed on a test timing out under load (decided 2026-10-24, REV-008).

/// Whether every ref a push sends deletes a branch: each line's local sha all zeros, of any
/// length (SHA-1's 40 digits, SHA-256's 64). No ref at all, or a line Git would not write, is no
/// deletion: the checks run.
#[must_use]
pub fn deletes_only(refs: &str) -> bool {
    let mut lines = refs
        .lines()
        .filter(|line| !line.trim().is_empty())
        .peekable();

    lines.peek().is_some()
        && lines.all(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();

            matches!(fields.as_slice(), [_, local, _, _]
                if !local.is_empty() && local.chars().all(|digit| digit == '0'))
        })
}
