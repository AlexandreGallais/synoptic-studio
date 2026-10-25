//! What the `session-state` hook prints when a session starts (ADR-0021): the progress of each
//! feature started, read from the stories of the records (EN-154), so that a new or compacted
//! session starts from facts rather than from memory. Rewritten from `.claude/hooks/session-state.ts`
//! (EN-104).

/// Age after which a running call's file is left over from an interrupted or denied call.
pub const STALE_MS: u128 = 86_400_000;

/// Whether a running call's file of that age, in milliseconds, is left over: older than
/// [`STALE_MS`], a day, the day itself not.
#[must_use]
pub fn is_stale(age_ms: u128) -> bool {
    age_ms > STALE_MS
}

/// A story of a feature, as the records hold it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryRow {
    pub id: String,
    pub status: String,
    pub title: String,
}

/// The statement `knowledge sql` runs for the hook: each story of a feature, its feature named
/// `E49-F48-<topic>` as its file was, by epic, feature and order in the feature.
pub const FEATURE_STORIES: &str = "SELECT features.epic || '-' || features.id || '-' || features.topic, stories.id, stories.status, stories.title FROM stories JOIN features ON features.id = stories.feature ORDER BY features.epic, features.id, stories.position, stories.id";

/// The features and their stories in order, from the rows of [`FEATURE_STORIES`]; a row of another
/// shape left out.
#[must_use]
pub fn feature_stories(rows: &[Vec<String>]) -> Vec<(String, Vec<StoryRow>)> {
    let mut features: Vec<(String, Vec<StoryRow>)> = Vec::new();

    for row in rows {
        let [feature, id, status, title] = row.as_slice() else {
            continue;
        };
        let story = StoryRow {
            id: id.clone(),
            status: status.clone(),
            title: title.clone(),
        };

        match features.last_mut() {
            Some((name, stories)) if name == feature => stories.push(story),
            _ => features.push((feature.clone(), vec![story])),
        }
    }
    features
}

/// Whether a status is a finished story's.
fn is_finished(status: &str) -> bool {
    matches!(status, "done" | "dropped")
}

/// The state line of a started feature: what is done and what comes next; `None` for a feature
/// not started (no story finished nor in progress) or finished.
#[must_use]
pub fn feature_line(feature: &str, rows: &[StoryRow]) -> Option<String> {
    let done = rows.iter().filter(|row| is_finished(&row.status)).count();
    let next = rows.iter().find(|row| !is_finished(&row.status))?;
    // A first story is `in-progress` on its own branch only: the feature has started.
    let is_started = done > 0 || rows.iter().any(|row| row.status == "in-progress");

    is_started.then(|| {
        format!(
            "{}: {done}/{} stories finished; next {} ({}) — {}",
            feature.strip_suffix(".md").unwrap_or(feature),
            rows.len(),
            next.id,
            next.status,
            next.title
        )
    })
}

/// The text the hook prints: the branch, its uncommitted files and last commit, then each feature
/// started, the release binaries older than their sources if any (ADR-0078), then the rule of
/// ready stories.
#[must_use]
pub fn state_text(
    branch: &str,
    changed: usize,
    last_commit: &str,
    features: &[String],
    stale: &[String],
) -> String {
    let mut lines = vec![
        "Project state (SessionStart hook, ADR-0021):".to_owned(),
        format!("- branch {branch}, {changed} uncommitted file(s), last commit: {last_commit}"),
    ];

    if features.is_empty() {
        lines.push("- no feature started".to_owned());
    }
    lines.extend(features.iter().map(|line| format!("- {line}")));
    if !stale.is_empty() {
        lines.push(format!(
            "- release binaries older than their sources, which the hooks run: {}; `npm run rust:build` rebuilds them",
            stale.join("; ")
        ));
    }
    lines.push("- a story is implemented only when ready; the Product Owner's /run sets its scope ready (ADR-0072).".to_owned());
    lines.push(String::new());
    lines.join("\n")
}
