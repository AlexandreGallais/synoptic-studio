//! `release publish`, by the button of `release.yml` on `main` (ADR-0086): the version from the
//! last tag `vx.y.z` and the commits since (`REF-SEMVER`), its notes the bodies of the pull
//! requests into `main` merged since that tag, each checked as `release check` does, then the
//! GitHub release made on `main`'s head, which GitHub tags, the test report of the last pull
//! request's CI attached. It makes no commit. Every Git and GitHub call goes through a [`Run`],
//! so that a test gives the answers.

use std::path::Path;

use crate::history::Run;

/// What `release publish` did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Published {
    /// The release of this tag was made.
    Released(String),
    /// No commit since this tag: nothing to release.
    Nothing(String),
}

/// The name of the CI's artifact holding the test report (`ci.yml`, ADR-0027).
pub const REPORT_ARTIFACT: &str = "test-report";

/// The test report's file in that artifact.
pub const REPORT_FILE: &str = "test-report.xml";

/// Publishes the release of the head of `main`, the notes and the report written in `folder`.
///
/// # Errors
///
/// No tag `vx.y.z` is behind the head, a body breaks a rule or two hold a notice, the head is not
/// a merge, no successful CI run of the pull request's head is found, or a Git or GitHub command
/// fails.
pub fn publish(run: &mut Run<'_>, folder: &Path) -> Result<Published, String> {
    let _ = (run, folder);
    Err(String::new())
}
