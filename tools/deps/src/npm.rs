//! An npm package's latest version at least 7 days old, from its full metadata on the registry
//! (`REF-NPM-PACKAGE-METADATA`: `dist-tags`, `time`, `versions`), as npm-check-updates, at 23.1.1,
//! chooses it with `target: "latest"` and `cooldown: 7` (`REF-NCU-COOLDOWN`), written anew from
//! its documentation and its results on this tree, no line of it copied: the version the `latest`
//! tag names when it is old enough; otherwise the greatest version old enough; pre-releases only
//! when the current version is one; deprecated versions never.

use serde_json::Value;

use crate::version::Version;
use crate::{Behind, is_past_cooldown};

/// Whether a version of the metadata may be chosen: a version number, not deprecated, and not a
/// pre-release unless pre-releases are taken.
fn is_candidate(metadata: &Value, text: &str, takes_pre: bool) -> bool {
    let is_deprecated = metadata["versions"][text]
        .get("deprecated")
        .is_some_and(|reason| reason.as_str().is_none_or(|reason| !reason.is_empty()));

    Version::parse(text).is_some_and(|version| takes_pre || !version.is_pre()) && !is_deprecated
}

/// The epoch milliseconds a version was published at, `None` when the metadata does not say.
fn published_ms(metadata: &Value, text: &str) -> Option<i64> {
    metadata["time"][text]
        .as_str()
        .map(hooks::time::epoch_ms)
        .filter(|&ms| ms > 0)
}

/// Whether a version is out of the cooldown at `now_ms`; a version without a date is, as
/// `npm-check-updates` takes it.
fn is_old_enough(metadata: &Value, text: &str, now_ms: i64) -> bool {
    published_ms(metadata, text).is_none_or(|published| is_past_cooldown(published, now_ms))
}

/// The version an npm package pinned at `current` should be on at `now_ms`, when it is greater
/// than `current`: `None` when it is up to date, or when `current` is no version number.
#[must_use]
pub fn behind(name: &str, current: &str, metadata: &Value, now_ms: i64) -> Option<Behind> {
    behind_below(name, current, metadata, now_ms, None)
}

/// As [`behind`], only versions below `ceiling` taken when one is given: a package held under a
/// major or minor another package asks, as the playground's `typescript` under the 6.1 Angular's
/// compiler refuses (ADR-0104).
#[must_use]
pub fn behind_below(
    name: &str,
    current: &str,
    metadata: &Value,
    now_ms: i64,
    ceiling: Option<&Version>,
) -> Option<Behind> {
    let pinned = Version::parse(current)?;
    let takes_pre = pinned.is_pre();
    let is_below = |text: &str| {
        ceiling.is_none_or(|ceiling| Version::parse(text).is_some_and(|version| version < *ceiling))
    };
    let latest = metadata["dist-tags"]["latest"].as_str()?;
    let target = if (latest == current || is_old_enough(metadata, latest, now_ms))
        && is_candidate(metadata, latest, takes_pre)
        && is_below(latest)
    {
        latest.to_owned()
    } else {
        metadata["versions"]
            .as_object()?
            .keys()
            .filter(|text| is_candidate(metadata, text, takes_pre) && is_below(text))
            .filter(|text| is_old_enough(metadata, text, now_ms))
            .filter_map(|text| Some((Version::parse(text)?, text)))
            .max_by(|(left, _), (right, _)| left.cmp(right))
            .map(|(_, text)| text.clone())?
    };

    (Version::parse(&target)? > pinned).then(|| Behind {
        name: name.to_owned(),
        current: current.to_owned(),
        latest: target.clone(),
        published_ms: published_ms(metadata, &target),
    })
}
