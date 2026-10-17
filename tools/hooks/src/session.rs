//! What the `session-state` hook prints when a session starts (ADR-0021): the progress of each
//! feature started, read from the story tables of the feature files, so that a new or compacted
//! session starts from facts rather than from memory. Ported from `.claude/hooks/session-state.ts`
//! (EN-104).

use crate::guard::is_space;

/// Age after which a running call's file is left over from an interrupted or denied call.
pub const STALE_MS: u128 = 86_400_000;

/// A row of a feature's story table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryRow {
    pub id: String,
    pub status: String,
    pub title: String,
}

/// Whether a text is a story id: capitals, a hyphen, three digits (`[A-Z]+-\d{3}`).
fn is_story_id(text: &str) -> bool {
    text.split_once('-').is_some_and(|(prefix, digits)| {
        !prefix.is_empty()
            && prefix.bytes().all(|byte| byte.is_ascii_uppercase())
            && digits.len() == 3
            && digits.bytes().all(|byte| byte.is_ascii_digit())
    })
}

/// One row `| [ID](…) | type | title | status |`, `None` for any other line:
/// `^\|\s*\[([A-Z]+-\d{3})\]\([^)]*\)\s*\|[^|]*\|\s*([^|]*?)\s*\|\s*([a-z-]+)\s*\|$`.
fn story_row(line: &str) -> Option<StoryRow> {
    let rest = line.strip_prefix('|')?.trim_start_matches(is_space);
    let (id, rest) = rest.strip_prefix('[')?.split_once("](")?;
    let (_, rest) = rest.split_once(')')?;
    let rest = rest.trim_start_matches(is_space).strip_prefix('|')?;
    let (_, rest) = rest.split_once('|')?;
    let mut cells = rest.split('|');
    let title = cells.next()?.trim_matches(is_space);
    let status = cells.next()?.trim_matches(is_space);

    (is_story_id(id)
        && !status.is_empty()
        && status
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        && cells.next() == Some("")
        && cells.next().is_none())
    .then(|| StoryRow {
        id: id.to_owned(),
        status: status.to_owned(),
        title: title.to_owned(),
    })
}

/// The story rows of a feature file, in delivery order.
#[must_use]
pub fn story_rows(text: &str) -> Vec<StoryRow> {
    // `^` and `$` of the `m` flag: at every line terminator, a carriage return included.
    text.split(['\n', '\r', '\u{2028}', '\u{2029}'])
        .filter_map(story_row)
        .collect()
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

/// Whether a file name is a feature's: `^E\d{2}-F\d{2}-.*\.md$`.
#[must_use]
pub fn is_feature_file(name: &str) -> bool {
    let bytes = name.as_bytes();

    bytes.len() >= 11
        && bytes[0] == b'E'
        && bytes[1..3].iter().all(u8::is_ascii_digit)
        && &bytes[3..5] == b"-F"
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == b'-'
        && bytes.ends_with(b".md")
        && !name[8..].contains(['\n', '\r', '\u{2028}', '\u{2029}'])
}

/// The text the hook prints: the branch, its uncommitted files and last commit, then each feature
/// started, then the rule of ready stories.
#[must_use]
pub fn state_text(branch: &str, changed: usize, last_commit: &str, features: &[String]) -> String {
    let mut lines = vec![
        "Project state (SessionStart hook, ADR-0021):".to_owned(),
        format!("- branch {branch}, {changed} uncommitted file(s), last commit: {last_commit}"),
    ];

    if features.is_empty() {
        lines.push("- no feature started".to_owned());
    }
    lines.extend(features.iter().map(|line| format!("- {line}")));
    lines.push("- a story is implemented only when ready; the Product Owner's /run sets its scope ready (ADR-0045).".to_owned());
    lines.push(String::new());
    lines.join("\n")
}
