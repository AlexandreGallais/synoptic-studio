//! `package.json` and `package-lock.json`: the version read and written in place, every other byte
//! kept, and the repository's address read for the links.

use std::sync::LazyLock;

use regex::Regex;

/// A `"version": "…"` member.
static VERSION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""version": "([^"]*)""#).unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// The `url` of `package.json`'s `repository`.
static REPOSITORY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""repository": \{[^}]*"url": "([^"]+)""#)
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// The first `"version"` of a manifest: the package's own in `package.json`, the top one in
/// `package-lock.json`.
#[must_use]
pub fn version(manifest: &str) -> Option<&str> {
    VERSION
        .captures(manifest)
        .and_then(|captures| captures.get(1))
        .map(|version| version.as_str())
}

/// The text with the first `"version"` from byte `from` on set to `new`; `None` without one.
fn with_version_from(text: &str, from: usize, new: &str) -> Option<String> {
    let found = VERSION.captures_at(text, from)?.get(1)?;

    Some(format!(
        "{}{new}{}",
        &text[..found.start()],
        &text[found.end()..]
    ))
}

/// `package.json` with its version set.
#[must_use]
pub fn with_package_version(package: &str, new: &str) -> Option<String> {
    with_version_from(package, 0, new)
}

/// `package-lock.json` with its two versions of the package set: the top one, and the one of the
/// root package, `packages[""]`.
#[must_use]
pub fn with_lock_version(lock: &str, new: &str) -> Option<String> {
    let top = with_version_from(lock, 0, new)?;
    let root = top.find("\"packages\": {")?;
    let root = root + top[root..].find("\"\": {")?;

    with_version_from(&top, root, new)
}

/// The versions `package-lock.json` holds for the package: the top one and the root package's.
#[must_use]
pub fn lock_versions(lock: &str) -> (Option<&str>, Option<&str>) {
    let root = lock
        .find("\"packages\": {")
        .and_then(|start| lock[start..].find("\"\": {").map(|offset| start + offset));
    let root_version = root
        .and_then(|start| VERSION.captures_at(lock, start))
        .and_then(|captures| captures.get(1))
        .map(|version| version.as_str());

    (version(lock), root_version)
}

/// The repository's address on GitHub from `package.json`:
/// `git+https://github.com/o/r.git` gives `https://github.com/o/r`.
#[must_use]
pub fn repository(package: &str) -> Option<String> {
    let url = REPOSITORY.captures(package)?.get(1)?.as_str();
    let url = url.strip_prefix("git+").unwrap_or(url);

    Some(url.strip_suffix(".git").unwrap_or(url).to_owned())
}
