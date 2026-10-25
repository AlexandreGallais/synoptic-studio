//! The story a branch is for, read from its name.

/// The story id of a story branch (`<type>/<id>-<topic>`, ADR-0072), `chore/en-050-topic` giving
/// `EN-050`; `None` for `develop`, `main` or a feature branch.
#[must_use]
pub fn story_of(branch: &str) -> Option<String> {
    let name = branch.strip_prefix("refs/heads/").unwrap_or(branch);
    let (_, story) = name.split_once('/')?;
    let (kind, rest) = story.split_once('-')?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    let after = &rest[digits.len()..];
    let letters = !kind.is_empty() && kind.chars().all(|letter| letter.is_ascii_lowercase());

    (letters && !digits.is_empty() && (after.is_empty() || after.starts_with('-')))
        .then(|| format!("{}-{digits}", kind.to_ascii_uppercase()))
}
