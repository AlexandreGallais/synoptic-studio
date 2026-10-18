//! A commit of the release as the history gives it, and what its Conventional Commits header,
//! body and footers say: its type and scope, whether it breaks, its story, the commit it reverts.

use std::sync::LazyLock;

use regex::Regex;

/// The scopes of the library and the playground (`SCOPES` of `tools/git-hooks`, less the docs',
/// the backlog's, the CI's, the dependencies' and the tooling's): a commit without a scope counts
/// as the library's, as `refactor: …` does here.
pub const USER_SCOPES: [&str; 8] = [
    "geometry",
    "interaction",
    "io",
    "math",
    "model",
    "playground",
    "render",
    "routing",
];

/// The types that change what a user of the library or the playground gets.
pub const USER_TYPES: [&str; 5] = ["feat", "fix", "perf", "refactor", "revert"];

/// One commit, as `git log` gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    /// The full hash.
    pub hash: String,
    /// The first line.
    pub subject: String,
    /// The rest of the message, footers included.
    pub body: String,
    /// The pull request that brought it, when one did.
    pub pull_request: Option<u32>,
}

/// A Conventional Commits header: `type(scope)!: description`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    /// `feat`.
    pub kind: String,
    /// `model`, `None` without a scope.
    pub scope: Option<String>,
    /// Whether a `!` marks it breaking.
    pub bang: bool,
    /// What follows `: `.
    pub description: String,
}

/// The header: a type in lower case, a scope in parentheses, a `!`, then `: `.
static HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([a-z]+)(?:\(([a-z0-9-]+)\))?(!)?: (.+)$")
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// A footer naming a story or one of its tasks: `Refs: EN-112.T2`.
static REFS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^Refs: ([A-Z]+-[0-9]+)").unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// The footer of a breaking change, with its text down to a blank line, the next footer
/// (`Refs: `) or the end: a footer's text may go on over several lines (Conventional Commits).
static BREAKING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?ms)^BREAKING[ -]CHANGE: (.+?)(?:\n\n|\n[A-Za-z-]+: |\n[A-Za-z-]+ #|\z)")
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// The line Git writes in a revert's body.
static REVERTS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"This reverts commit ([0-9a-f]{40})")
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// The pull request number a squash merge puts at a subject's end: `… (#123)`.
static SQUASHED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r" \(#([0-9]+)\)$").unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// The subject of GitHub's merge commit: `Merge pull request #340 from owner/branch`.
static MERGED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^Merge pull request #([0-9]+) from ")
        .unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// A subject's header, `None` for a subject outside Conventional Commits (`Revert "…"`).
#[must_use]
pub fn header(subject: &str) -> Option<Header> {
    let captures = HEADER.captures(subject)?;

    Some(Header {
        kind: captures[1].to_owned(),
        scope: captures.get(2).map(|scope| scope.as_str().to_owned()),
        bang: captures.get(3).is_some(),
        description: captures[4].to_owned(),
    })
}

impl Commit {
    /// Its header, when it has one.
    #[must_use]
    pub fn header(&self) -> Option<Header> {
        header(&self.subject)
    }

    /// Whether it changes what a user gets: a type of [`USER_TYPES`], in a scope of
    /// [`USER_SCOPES`] or none. Git's own revert (`Revert "…"`) counts as `revert`.
    #[must_use]
    pub fn is_user_visible(&self) -> bool {
        if self.subject.starts_with("Revert \"") {
            return true;
        }
        self.header().is_some_and(|header| {
            USER_TYPES.contains(&header.kind.as_str())
                && header
                    .scope
                    .as_deref()
                    .is_none_or(|scope| USER_SCOPES.contains(&scope))
        })
    }

    /// The text of its breaking change: the footer's, else its description when `!` marks it;
    /// `None` when it breaks nothing.
    #[must_use]
    pub fn breaking(&self) -> Option<String> {
        BREAKING
            .captures(&self.body)
            .map(|captures| captures[1].split_whitespace().collect::<Vec<_>>().join(" "))
            .or_else(|| {
                self.header()
                    .filter(|header| header.bang)
                    .map(|header| header.description)
            })
    }

    /// The story its first `Refs:` footer names: `EN-112` for `Refs: EN-112.T2`.
    #[must_use]
    pub fn story(&self) -> Option<String> {
        REFS.captures(&self.body)
            .map(|captures| captures[1].to_owned())
    }

    /// The full hash of the commit it reverts, from Git's `This reverts commit …` line.
    #[must_use]
    pub fn reverted(&self) -> Option<String> {
        REVERTS
            .captures(&self.body)
            .map(|captures| captures[1].to_owned())
    }

    /// The pull request a squash merge wrote into its subject.
    #[must_use]
    pub fn squashed_pull_request(&self) -> Option<u32> {
        SQUASHED
            .captures(&self.subject)
            .and_then(|captures| captures[1].parse().ok())
    }

    /// Its seven first characters, as Git abbreviates it.
    #[must_use]
    pub fn short(&self) -> &str {
        self.hash.get(..7).unwrap_or(&self.hash)
    }
}

/// The pull request a merge commit's subject names: `340` for GitHub's
/// `Merge pull request #340 from …`.
#[must_use]
pub fn merged_pull_request(subject: &str) -> Option<u32> {
    MERGED
        .captures(subject)
        .and_then(|captures| captures[1].parse().ok())
}
