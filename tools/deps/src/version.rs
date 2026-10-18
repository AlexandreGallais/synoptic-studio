//! Version numbers as Semantic Versioning 2.0.0 writes them (`REF-SEMVER`): `MAJOR.MINOR.PATCH`,
//! an optional pre-release after a hyphen, build metadata after a plus sign; ordered by the
//! specification's precedence, and grouped as Cargo groups compatible versions (`REF-CARGO-SEMVER`).

use std::cmp::Ordering;

/// One identifier of a pre-release: digits only, or with letters and hyphens.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Identifier {
    /// Digits only, compared numerically.
    Number(u64),
    /// Letters or hyphens, compared in ASCII order.
    Text(String),
}

/// A version number, its build metadata dropped (it takes no part in precedence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    /// `MAJOR`.
    pub major: u64,
    /// `MINOR`.
    pub minor: u64,
    /// `PATCH`.
    pub patch: u64,
    /// The pre-release identifiers, empty for a normal version.
    pre: Vec<Identifier>,
}

/// A number of the specification: digits, without a leading zero.
fn number(text: &str) -> Option<u64> {
    let is_digits = !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit());

    if !is_digits || (text.len() > 1 && text.starts_with('0')) {
        return None;
    }
    text.parse().ok()
}

/// A pre-release identifier: ASCII alphanumerics and hyphens, never empty, a number without a
/// leading zero.
fn identifier(text: &str) -> Option<Identifier> {
    let is_allowed = !text.is_empty()
        && text
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-');

    if !is_allowed {
        return None;
    }
    if text.bytes().all(|byte| byte.is_ascii_digit()) {
        return number(text).map(Identifier::Number);
    }
    Some(Identifier::Text(text.to_owned()))
}

impl Version {
    /// The version a text writes, `None` when it is not one (`1.2`, `v1.2.3`, `1.02.3`, a range).
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let without_build = text.split_once('+').map_or(text, |(version, _)| version);
        let (normal, pre) = without_build
            .split_once('-')
            .map_or((without_build, None), |(normal, pre)| (normal, Some(pre)));
        let mut parts = normal.split('.');
        let major = number(parts.next()?)?;
        let minor = number(parts.next()?)?;
        let patch = number(parts.next()?)?;

        if parts.next().is_some() {
            return None;
        }
        let pre = match pre {
            Some(pre) => pre.split('.').map(identifier).collect::<Option<Vec<_>>>()?,
            None => Vec::new(),
        };
        Some(Self {
            major,
            minor,
            patch,
            pre,
        })
    }

    /// Whether it is a pre-release (`2.0.0-alpha.20`).
    #[must_use]
    pub fn is_pre(&self) -> bool {
        !self.pre.is_empty()
    }

    /// Whether Cargo takes it for a compatible version of `other`: the same left-most non-zero
    /// component of major, minor and patch (`1.2.3` and `1.9.0`, `0.2.3` and `0.2.9`).
    #[must_use]
    pub fn is_compatible(&self, other: &Self) -> bool {
        if self.major != 0 || other.major != 0 {
            self.major == other.major
        } else if self.minor != 0 || other.minor != 0 {
            self.minor == other.minor
        } else {
            self.patch == other.patch
        }
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => left.cmp(right),
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
            (Self::Number(_), Self::Text(_)) => Ordering::Less,
            (Self::Text(_), Self::Number(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    /// The precedence of the specification: major, minor and patch numerically; a pre-release
    /// before its normal version; pre-releases identifier by identifier, the longer list after.
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
            .then_with(|| match (self.pre.is_empty(), other.pre.is_empty()) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (false, false) => self.pre.cmp(&other.pre),
            })
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
