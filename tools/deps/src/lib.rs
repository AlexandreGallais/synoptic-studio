//! The dependency checks of the repository (EN-109), in place of `npm-check-updates`,
//! `scripts/tool-versions.ts` and `knip`: `deps outdated`, every pinned version against its
//! latest version at least 7 days old (ADR-0072); `deps unused`, the unused files, exports and
//! dev packages. Neither ports code of those packages: each follows their documented behaviour
//! (`REF-NCU-COOLDOWN`, `REF-KNIP-ISSUE-TYPES`), checked against their findings on this tree.

pub mod actions;
pub mod cargo_tools;
pub mod crates;
pub mod fetch;
pub mod modules;
pub mod node;
pub mod npm;
pub mod outdated;
pub mod unused;
pub mod version;

/// The cooldown: a version is taken once it is 7 days old (ADR-0072), in milliseconds.
pub const COOLDOWN_MS: i64 = 7 * calendar::DAY_MS;

/// Whether a version published at `published_ms` is past the cooldown at `now_ms`: 7 days old or
/// more, the seventh day included (`REF-NCU-COOLDOWN`). The one test of the cooldown, for npm,
/// the crates, Node.js and the actions (NAM-002).
#[must_use]
pub fn is_past_cooldown(published_ms: i64, now_ms: i64) -> bool {
    now_ms - published_ms >= COOLDOWN_MS
}

/// A pinned version behind the latest one old enough.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Behind {
    /// The package, crate or action.
    pub name: String,
    /// The version pinned.
    pub current: String,
    /// The latest version at least 7 days old.
    pub latest: String,
    /// When it was published, epoch milliseconds; `None` when its source does not say.
    pub published_ms: Option<i64>,
}
