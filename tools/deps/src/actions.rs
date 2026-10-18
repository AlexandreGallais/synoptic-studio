//! The GitHub Actions of the workflows and their next major version, read from the repositories'
//! tags (`REF-GH-MATCHING-REFS`) and the date of the commit the first tag of that major names
//! (`REF-GH-COMMIT`), as `scripts/tool-versions.ts` read them, with the 7 days of ADR-0038.

/// An action a workflow uses, with the major version it is pinned to.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionUse {
    /// `owner/repository`, without the path of an action inside it.
    pub repository: String,
    /// The major version of `@vN`.
    pub major: u64,
}

/// The `owner/repository` and major version of one `uses:` value: `actions/checkout@v7`,
/// `github/codeql-action/init@v4`; `None` for a local action, a Docker image or a commit pin.
fn action_use(value: &str) -> Option<ActionUse> {
    let (path, pin) = value.split_once('@')?;
    let mut parts = path.split('/');
    let owner = parts
        .next()
        .filter(|owner| !owner.is_empty() && *owner != ".")?;
    let name = parts.next().filter(|name| !name.is_empty())?;
    let digits: String = pin
        .strip_prefix('v')?
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();

    Some(ActionUse {
        repository: format!("{owner}/{name}"),
        major: digits.parse().ok()?,
    })
}

/// The actions a workflow uses, each `uses:` line pinned to a major version.
#[must_use]
pub fn uses(workflow: &str) -> Vec<ActionUse> {
    workflow
        .lines()
        .filter_map(|line| {
            let value = line.trim_start().trim_start_matches("- ").trim_start();

            action_use(
                value
                    .strip_prefix("uses:")?
                    .trim()
                    .trim_matches(['"', '\'']),
            )
        })
        .collect()
}

/// The tags of a major version among the references GitHub matched (`refs/tags/v8`,
/// `refs/tags/v8.0.0`), `v80` left out.
#[must_use]
pub fn major_tags(references: &serde_json::Value, major: u64) -> Vec<String> {
    let exact = format!("v{major}");

    references
        .as_array()
        .map(|references| {
            references
                .iter()
                .filter_map(|reference| reference["ref"].as_str()?.strip_prefix("refs/tags/"))
                .filter(|tag| {
                    tag.strip_prefix(exact.as_str())
                        .is_some_and(|rest| rest.is_empty() || rest.starts_with('.'))
                })
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

/// The tag a major version was first released under: its least full version (`v8.0.0`), or the
/// first tag when none is a full version (`v8`).
#[must_use]
pub fn first_tag(tags: &[String]) -> Option<&String> {
    tags.iter()
        .filter_map(|tag| Some((crate::version::Version::parse(tag.strip_prefix('v')?)?, tag)))
        .min_by(|(left, _), (right, _)| left.cmp(right))
        .map(|(_, tag)| tag)
        .or_else(|| tags.first())
}

/// The committer's date of a commit GitHub returned, epoch milliseconds.
#[must_use]
pub fn commit_ms(commit: &serde_json::Value) -> Option<i64> {
    commit["commit"]["committer"]["date"]
        .as_str()
        .map(hooks::time::epoch_ms)
        .filter(|&ms| ms > 0)
}
