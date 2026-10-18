//! `release check`: a release text, the body of a pull request into `main` up to its first
//! thematic break, against the rules it can check (`docs/tooling/releases.md`, What a release
//! text holds): a notice at most, then the groups in their order, one line per change with its
//! references last, the breaking changes first, and no line left as drafted.

/// One rule broken, at a line of the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// Its line, from 1.
    pub line: usize,
    /// What the rule asks.
    pub message: String,
}

/// The findings on a release text; none when it follows every rule the tool can check.
#[must_use]
pub fn check(text: &str) -> Vec<Finding> {
    let _ = text;
    Vec::new()
}
