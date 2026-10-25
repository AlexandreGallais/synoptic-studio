//! The footer a commit on a story branch carries (EN-108): `Refs: <ID>.T<n>`, a task of the
//! branch's own story, so that `story:finish` and the metrics find each commit's story; a story
//! has no `done` commit since its row moved into the records (EN-154), so `Closes:` is no footer
//! of it any more (AUD-053). A feature branch, `develop`, `main` and a `docs/` branch are not asked
//! for one; nor is a message commitlint ignores (a merge, a revert, a `fixup!`).

use git_repository::story_of_branch as story_of;

/// The story id a branch's commits name, `None` when the branch asks for none.
#[must_use]
pub fn story_of_branch(branch: &str) -> Option<String> {
    let name = branch.strip_prefix("refs/heads/").unwrap_or(branch);

    if name.starts_with("feature/") || name.starts_with("docs/") {
        return None;
    }

    story_of(name)
}

/// The items of a footer line after its token, split at commas and spaces.
fn items<'a>(line: &'a str, token: &str) -> Vec<&'a str> {
    line.strip_prefix(token)
        .map(|rest| {
            rest.split(|character: char| character == ',' || character.is_whitespace())
                .filter(|item| !item.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Whether an item names a task of a story: `<ID>.T<n>`.
fn is_task_of(item: &str, story: &str) -> bool {
    item.strip_prefix(story)
        .and_then(|rest| rest.strip_prefix(".T"))
        .is_some_and(|number| {
            !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
        })
}

/// Whether a message carries `Refs: <ID>.T<n>` for the story, on a line of its own.
#[must_use]
pub fn carries_footer(message: &str, story: &str) -> bool {
    message.lines().any(|line| {
        items(line, "Refs:")
            .iter()
            .any(|item| is_task_of(item, story))
    })
}

/// The refusal of a message on a branch, `None` when it carries its footer or needs none.
#[must_use]
pub fn footer_refusal(message: &str, branch: &str) -> Option<String> {
    let story = story_of_branch(branch)?;

    (!crate::message::is_ignored(message) && !carries_footer(message, &story)).then(|| {
        format!(
            "✖   a commit on the story branch {branch} carries the footer `Refs: {story}.T<n>`, \
             a task of its story [story-footer]"
        )
    })
}
