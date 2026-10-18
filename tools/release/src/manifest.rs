//! `package.json`: the repository's address, read for the links of a drafted text. Its version
//! stays `0.0.0-development` (ADR-0086): the version lives in the tags.

use std::sync::LazyLock;

use regex::Regex;

/// The `url` of `package.json`'s `repository`.
static REPOSITORY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""repository": \{[^}]*"url": "([^"]+)""#)
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// The repository's address on GitHub from `package.json`:
/// `git+https://github.com/o/r.git` gives `https://github.com/o/r`.
#[must_use]
pub fn repository(package: &str) -> Option<String> {
    let url = REPOSITORY.captures(package)?.get(1)?.as_str();
    let url = url.strip_prefix("git+").unwrap_or(url);

    Some(url.strip_suffix(".git").unwrap_or(url).to_owned())
}
