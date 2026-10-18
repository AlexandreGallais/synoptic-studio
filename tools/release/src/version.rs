//! A version and its bump (`REF-SEMVER`): a breaking change raises the major version, or the minor
//! one before 1.0.0; a `feat` the minor one; anything else the patch.

use std::fmt;

/// A version `major.minor.patch`, without prerelease nor build metadata: the project makes none.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    /// Raised by a breaking change from 1.0.0 on.
    pub major: u64,
    /// Raised by a feature, or a breaking change before 1.0.0.
    pub minor: u64,
    /// Raised by anything else.
    pub patch: u64,
}

/// How much a release raises the version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bump {
    /// A fix, a refactoring, or no change of the library at all.
    Patch,
    /// A feature.
    Minor,
    /// A breaking change.
    Breaking,
}

impl Version {
    /// A version written `x.y.z`, a `v` allowed first as in a tag; `None` for anything else, a
    /// leading zero included (`REF-SEMVER`, item 2).
    #[must_use]
    pub fn parse(text: &str) -> Option<Version> {
        let mut numbers = text.strip_prefix('v').unwrap_or(text).split('.');
        let mut next = || {
            numbers
                .next()
                .filter(|number| {
                    !number.is_empty()
                        && number.bytes().all(|byte| byte.is_ascii_digit())
                        && (number.len() == 1 || !number.starts_with('0'))
                })
                .and_then(|number| number.parse().ok())
        };
        let version = Version {
            major: next()?,
            minor: next()?,
            patch: next()?,
        };

        numbers.next().is_none().then_some(version)
    }

    /// The next version: before 1.0.0 a breaking change raises the minor version
    /// (release-please's `bump-minor-pre-major`, kept), from 1.0.0 the major one.
    #[must_use]
    pub fn bumped(self, bump: Bump) -> Version {
        match bump {
            Bump::Breaking if self.major > 0 => Version {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
            Bump::Breaking | Bump::Minor => Version {
                major: self.major,
                minor: self.minor + 1,
                patch: 0,
            },
            Bump::Patch => Version {
                patch: self.patch + 1,
                ..self
            },
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
