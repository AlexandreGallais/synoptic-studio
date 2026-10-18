//! The release, in place of release-please (EN-112, ADR-0079, `docs/tooling/releases.md`): the
//! version from the commits since the last tag (`REF-SEMVER`), `package.json`, `package-lock.json`
//! and the draft of the changelog's entry written by one command, the entry checked against the
//! rules of good release notes (`REF-COMMON-CHANGELOG`, `REF-KEEP-A-CHANGELOG`), and read back for
//! the pull request into `main`. Nothing of release-please is ported.

pub mod backlog;
pub mod changelog;
pub mod check;
pub mod commit;
pub mod draft;
pub mod history;
pub mod manifest;
pub mod prepare;
pub mod version;
