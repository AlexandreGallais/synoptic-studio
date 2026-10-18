//! The dependency checks of the repository (EN-109), in place of `npm-check-updates`,
//! `scripts/tool-versions.ts` and `knip`: `deps outdated`, every pinned version against its
//! latest version at least 7 days old (ADR-0038); `deps unused`, the unused files, exports and
//! dev packages. Neither ports code of those packages: each follows their documented behaviour
//! (`REF-NCU-COOLDOWN`, `REF-KNIP-ISSUE-TYPES`), checked against their findings on this tree.

pub mod actions;
pub mod crates;
pub mod fetch;
pub mod modules;
pub mod node;
pub mod npm;
pub mod outdated;
pub mod unused;
pub mod version;

/// The cooldown: a version is taken once it is 7 days old (ADR-0038), in milliseconds.
pub const COOLDOWN_MS: i64 = 7 * 86_400_000;

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
