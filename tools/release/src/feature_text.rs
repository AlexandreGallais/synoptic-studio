//! `release notes`: the draft of the features' text for the pull request into `main`, from the
//! commits a base lacks (ADR-0086), printed and never written: no branch, no file.

use std::path::Path;

use crate::history::Run;

/// What `release notes` prints.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureText {
    /// The text's draft, each line marked `[draft]`, or the notice of no change.
    pub text: String,
    /// The commits behind each line, for the agent rewriting them.
    pub hints: String,
    /// The number of lines drafted.
    pub lines: usize,
}

/// The features' text drafted from the commits of the head that `base` lacks, `package.json` and
/// the backlog read under `root`.
///
/// # Errors
///
/// `package.json` names no repository, the history cannot be read, or no commit came since
/// `base`.
pub fn feature_text(root: &Path, run: &mut Run<'_>, base: &str) -> Result<FeatureText, String> {
    let _ = (root, run, base);
    Err(String::new())
}
