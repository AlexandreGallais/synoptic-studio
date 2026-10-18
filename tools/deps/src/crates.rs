//! A crate's versions from its file of the crates.io index, read as Cargo reads it over the sparse
//! protocol (`REF-CARGO-REGISTRY-INDEX`): one JSON line per version, its `vers`, its `yanked` flag
//! and its `pubtime`, the publish time crates.io writes since 2025. A version without a `pubtime`
//! is older than the field, so old enough.

use crate::version::Version;
use crate::{Behind, COOLDOWN_MS};

/// One version of the index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// The version number as written.
    pub text: String,
    /// The version number.
    pub version: Version,
    /// Whether it is yanked.
    pub is_yanked: bool,
    /// When it was published, epoch milliseconds; `None` before the index held the field.
    pub published_ms: Option<i64>,
}

/// A locked crate published less than 7 days ago, and the version to pin it to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TooNew {
    /// The crate.
    pub name: String,
    /// Its version in `Cargo.lock`.
    pub locked: String,
    /// When that version was published, epoch milliseconds.
    pub published_ms: i64,
    /// The greatest compatible version at least 7 days old, `None` when there is none.
    pub pin: Option<String>,
}

/// The versions of an index file, its lines that are no version left out.
#[must_use]
pub fn releases(index: &str) -> Vec<Release> {
    index
        .lines()
        .filter_map(|line| {
            let entry: serde_json::Value = serde_json::from_str(line).ok()?;
            let text = entry["vers"].as_str()?.to_owned();

            Some(Release {
                version: Version::parse(&text)?,
                is_yanked: entry["yanked"].as_bool().unwrap_or(false),
                published_ms: entry["pubtime"]
                    .as_str()
                    .map(hooks::time::epoch_ms)
                    .filter(|&ms| ms > 0),
                text,
            })
        })
        .collect()
}

/// Whether a release is out of the cooldown at `now_ms`.
fn is_old_enough(release: &Release, now_ms: i64) -> bool {
    release
        .published_ms
        .is_none_or(|published| now_ms - published >= COOLDOWN_MS)
}

/// The greatest release, not yanked, at least 7 days old, a pre-release only when `takes_pre`,
/// among those `keep` keeps.
fn greatest(
    releases: &[Release],
    now_ms: i64,
    takes_pre: bool,
    keep: impl Fn(&Release) -> bool,
) -> Option<&Release> {
    releases
        .iter()
        .filter(|release| !release.is_yanked && (takes_pre || !release.version.is_pre()))
        .filter(|release| is_old_enough(release, now_ms) && keep(release))
        .max_by(|left, right| left.version.cmp(&right.version))
}

/// The version a crate pinned at `current` in `[workspace.dependencies]` should be on at `now_ms`,
/// when it is greater: `None` when it is up to date or `current` is no version number.
#[must_use]
pub fn behind(name: &str, current: &str, releases: &[Release], now_ms: i64) -> Option<Behind> {
    let pinned = Version::parse(current)?;
    let latest = greatest(releases, now_ms, pinned.is_pre(), |_| true)?;

    (latest.version > pinned).then(|| Behind {
        name: name.to_owned(),
        current: current.to_owned(),
        latest: latest.text.clone(),
        published_ms: latest.published_ms,
    })
}

/// A crate locked at `locked` that was published less than 7 days before `now_ms`, with the
/// greatest compatible version old enough to pin it to; `None` when it is old enough.
#[must_use]
pub fn too_new(name: &str, locked: &str, releases: &[Release], now_ms: i64) -> Option<TooNew> {
    let version = Version::parse(locked)?;
    let release = releases.iter().find(|release| release.version == version)?;
    let published_ms = release.published_ms?;

    (now_ms - published_ms < COOLDOWN_MS).then(|| TooNew {
        name: name.to_owned(),
        locked: locked.to_owned(),
        published_ms,
        pin: greatest(releases, now_ms, version.is_pre(), |candidate| {
            candidate.version.is_compatible(&version)
        })
        .map(|release| release.text.clone()),
    })
}

/// The path of a crate's file in the index (`REF-CARGO-REGISTRY-INDEX`, Index files): `1/a`,
/// `2/ab`, `3/a/abc`, `se/rd/serde_json`, from its name in lowercase.
#[must_use]
pub fn index_path(name: &str) -> String {
    let lower = name.to_lowercase();

    match lower.len() {
        1 => format!("1/{lower}"),
        2 => format!("2/{lower}"),
        3 => format!("3/{}/{lower}", &lower[..1]),
        _ => format!("{}/{}/{lower}", &lower[..2], &lower[2..4]),
    }
}
