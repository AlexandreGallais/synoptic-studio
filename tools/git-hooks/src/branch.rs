//! The branch name, checked before a push and in the CI (EN-108, in place of validate-branch-name
//! 1.3.2, `REF-VALIDATE-BRANCH-NAME`): one regular expression, the project's own pattern of
//! `.validate-branch-namerc.json`, so nothing of the package is ported. `story:finish` and the
//! auto-merge read a story's id from its branch (`docs/tooling/git-workflow.md`).

use std::sync::LazyLock;

use regex::Regex;

/// The branches the repository accepts: `main`, `develop`, `feature/f<nn>-<topic>`,
/// `<type>/<topic>` in kebab case, a release branch `release/x.y.z` (ADR-0079), and the bot's
/// `dependabot/…`; release-please's `release-please--…` no longer since EN-112. Written as
/// JavaScript read it: its `.` stops at every line terminator.
pub const BRANCH_PATTERN: &str = r"^(main|develop|feature/f[0-9]{2}-[a-z0-9]+(-[a-z0-9]+)*|(feat|fix|docs|refactor|test|chore|build|ci|perf|style|revert)/[a-z0-9]+(-[a-z0-9]+)*|release/(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)|dependabot/[^\n\r\x{2028}\x{2029}]+)$";

/// The pattern, compiled once.
static BRANCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(BRANCH_PATTERN).unwrap_or_else(|_| unreachable!("a valid pattern"))
});

/// Whether a branch name is one the repository accepts.
#[must_use]
pub fn is_valid_branch(name: &str) -> bool {
    BRANCH.is_match(name)
}

/// Why a branch name is refused, as validate-branch-name said it, `None` when it passes.
#[must_use]
pub fn branch_refusal(name: &str) -> Option<String> {
    (!is_valid_branch(name)).then(|| {
        format!(
            "✖   branch name \"{name}\" refused: it must be `main`, `develop`, \
             `feature/f01-<kebab-case-topic>`, `<type>/<kebab-case-topic>` or `release/x.y.z` \
             (docs/tooling/git-workflow.md); pattern {BRANCH_PATTERN}"
        )
    })
}
