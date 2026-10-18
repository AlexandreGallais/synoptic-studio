//! The release by a button (ADR-0086, `docs/tooling/releases.md`), after the project's own tool
//! of EN-112: the features' text drafted from the commits for the pull request into `main`, that
//! text checked against the rules of good release notes (`REF-COMMON-CHANGELOG`,
//! `REF-KEEP-A-CHANGELOG`), and, by the button of `release.yml`, the version computed from the
//! last tag and the commits since (`REF-SEMVER`) and the GitHub release made, its notes the bodies
//! of the pull requests into `main` merged since that tag. No file holds the version: the tags do.

pub mod backlog;
pub mod body;
pub mod check;
pub mod commit;
pub mod draft;
pub mod feature_text;
pub mod history;
pub mod manifest;
pub mod publish;
pub mod version;
