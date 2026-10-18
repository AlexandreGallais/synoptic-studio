//! Node.js's latest LTS line, from the index of its releases (`REF-NODE-DIST-INDEX`): newest
//! version first, each with its `date` and its `lts` codename, `false` outside an LTS line.

/// An LTS release of Node.js.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lts {
    /// Its major version.
    pub major: u64,
    /// Its version as the index writes it: `v24.11.0`.
    pub version: String,
    /// Its date, epoch milliseconds.
    pub published_ms: i64,
}

/// The newest release of an LTS line published at least 7 days before `now_ms`, `None` when the
/// index holds none.
#[must_use]
pub fn latest_lts(index: &serde_json::Value, now_ms: i64) -> Option<Lts> {
    index.as_array()?.iter().find_map(|release| {
        release["lts"].as_str()?;
        let version = release["version"].as_str()?;
        let published_ms =
            hooks::time::epoch_ms(&format!("{}T00:00:00Z", release["date"].as_str()?));
        let major = version.strip_prefix('v')?.split('.').next()?.parse().ok()?;

        (published_ms > 0 && crate::is_past_cooldown(published_ms, now_ms)).then(|| Lts {
            major,
            version: version.to_owned(),
            published_ms,
        })
    })
}
