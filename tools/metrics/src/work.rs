//! Units of work of the metrics report (`the records' conventions, area metrics`): what the agent's requests and
//! tool calls were spent on — a story, a feature branch, a task commit — with active time and
//! tokens.

use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::tables::group_by;
use crate::time::offset_instant_ms;
use crate::transcript::upsert;

/// A pause longer than this is not work: a gap counts for this much at most, 5 minutes.
pub const IDLE_MS: i64 = 300_000;

/// Characters of a commit hash shown, as `git log --oneline` shows them.
const SHORT_HASH: usize = 7;

/// Something the agent did at a moment: a model request or a tool call.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Moment {
    /// The Git branch it happened on.
    pub branch: String,
    /// The tool call's duration, milliseconds; 0 for a request.
    pub duration_ms: f64,
    /// The tool call failed.
    pub is_failure: bool,
    /// It is a tool call, not a request.
    pub is_tool_call: bool,
    /// Tokens the model wrote.
    pub output_tokens: i64,
    /// The session.
    pub session: String,
    /// When: a request's time, a call's end; epoch milliseconds.
    pub time: i64,
    /// Tokens of every kind.
    pub tokens: i64,
}

/// A commit with the tasks its `Refs:` lines name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TaskCommit {
    /// Its hash.
    pub hash: String,
    /// Its subject.
    pub subject: String,
    /// The tasks: `US-018.T2`.
    pub tasks: Vec<String>,
    /// Its author date, epoch milliseconds.
    pub time: i64,
}

/// Totals of one unit of work.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkStats {
    /// The unit.
    pub key: String,
    /// Active time, milliseconds.
    pub active_ms: i64,
    /// Tool calls.
    pub calls: usize,
    /// Tool calls that failed.
    pub failures: usize,
    /// Tokens the model wrote.
    pub output_tokens: i64,
    /// Tokens of every kind.
    pub tokens: i64,
    /// Time the tools ran, milliseconds.
    pub tool_ms: f64,
}

/// Active time of a series of moments: the gaps between them, each capped at [`IDLE_MS`].
#[must_use]
pub fn active_ms(times: &[i64]) -> i64 {
    let mut sorted = times.to_vec();

    sorted.sort_unstable();
    sorted
        .windows(2)
        .map(|pair| (pair[1] - pair[0]).min(IDLE_MS))
        .sum()
}

/// Story prefixes a branch's unit is read from.
const STORY_PREFIXES: [&str; 9] = ["us", "en", "sp", "chk", "aud", "val", "nam", "rev", "ret"];

/// Whether a byte is a word character for a regular expression's `\b`.
fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The story id starting at `at` in a branch name, a word of its own: `us-018`.
fn story_at(branch: &str, at: usize) -> Option<&str> {
    let bytes = branch.as_bytes();

    if at > 0 && is_word(bytes[at - 1]) {
        return None;
    }

    STORY_PREFIXES.iter().find_map(|prefix| {
        let end = at + prefix.len() + 4;
        let id = branch.get(at..end)?;
        let digits = id.get(prefix.len() + 1..)?;

        (id.starts_with(prefix)
            && id.as_bytes()[prefix.len()] == b'-'
            && digits.bytes().all(|byte| byte.is_ascii_digit())
            && bytes.get(end).is_none_or(|next| !is_word(*next)))
        .then_some(id)
    })
}

/// The unit of work of a Git branch: its story (`feat/us-018-turn-a-shape` is `US-018`), else its
/// feature (`feature/f04-free-rotation` is `F04`), else the branch itself; `(no branch)` for none.
#[must_use]
pub fn unit_of(branch: &str) -> String {
    for at in 0..branch.len() {
        if let Some(story) = story_at(branch, at) {
            return story.to_uppercase();
        }

        let feature = branch.get(at..at + 4).filter(|_| at == "feature/".len());

        if let Some(found) = feature.filter(|text| {
            branch.starts_with("feature/")
                && text.starts_with('f')
                && text[1..3].bytes().all(|byte| byte.is_ascii_digit())
                && text.ends_with('-')
        }) {
            return found[..3].to_uppercase();
        }
    }

    if branch.is_empty() {
        "(no branch)".to_owned()
    } else {
        branch.to_owned()
    }
}

/// Totals of moments grouped by a key, in the order each group first came.
pub fn work_stats<K: Fn(&Moment) -> String>(moments: &[Moment], key_of: K) -> Vec<WorkStats> {
    group_by(moments, key_of)
        .into_iter()
        .map(|(key, group)| {
            let calls: Vec<&&Moment> = group.iter().filter(|moment| moment.is_tool_call).collect();

            WorkStats {
                key,
                active_ms: active_ms(&group.iter().map(|moment| moment.time).collect::<Vec<_>>()),
                calls: calls.len(),
                failures: calls.iter().filter(|moment| moment.is_failure).count(),
                output_tokens: group.iter().map(|moment| moment.output_tokens).sum(),
                tokens: group.iter().map(|moment| moment.tokens).sum(),
                tool_ms: calls.iter().map(|moment| moment.duration_ms).sum(),
            }
        })
        .collect()
}

/// The task commits of each unit, oldest first: the commits naming a task of `US-018` under
/// `US-018`.
#[must_use]
pub fn commits_by_unit(commits: &[TaskCommit]) -> HashMap<String, Vec<&TaskCommit>> {
    let mut units: HashMap<String, Vec<&TaskCommit>> = HashMap::new();

    for commit in commits {
        let mut seen: Vec<&str> = Vec::new();

        for task in &commit.tasks {
            let unit = task.split('.').next().unwrap_or_default();

            if !seen.contains(&unit) {
                seen.push(unit);
                units.entry(unit.to_owned()).or_default().push(commit);
            }
        }
    }
    units
}

/// The task a moment worked towards: the first commit of its story made at or after it, written
/// `<task ids> <short hash> <subject>`; `<unit> (no task commit)` when none is.
#[must_use]
pub fn task_of<S: BuildHasher>(
    moment: &Moment,
    commits: &HashMap<String, Vec<&TaskCommit>, S>,
) -> String {
    let unit = unit_of(&moment.branch);
    let commit = commits
        .get(&unit)
        .and_then(|list| list.get(list.partition_point(|candidate| candidate.time < moment.time)));

    match commit {
        Some(commit) => format!(
            "{} {} {}",
            commit.tasks.join(", "),
            commit.hash.get(..SHORT_HASH).unwrap_or(&commit.hash),
            commit.subject
        ),
        None => format!("{unit} (no task commit)"),
    }
}

/// Whether a text is a task id: `US-018.T2`.
fn is_task_id(id: &str) -> bool {
    let Some((story, task)) = id.split_once('.') else {
        return false;
    };
    let Some((kind, number)) = story.split_once('-') else {
        return false;
    };

    !kind.is_empty()
        && kind.bytes().all(|byte| byte.is_ascii_uppercase())
        && number.len() == 3
        && number.bytes().all(|byte| byte.is_ascii_digit())
        && task.len() > 1
        && task.starts_with('T')
        && task[1..].bytes().all(|byte| byte.is_ascii_digit())
}

/// The task ids a commit message names in its `Refs:` lines: `US-018.T2`.
#[must_use]
pub fn task_ids(body: &str) -> Vec<String> {
    body.split(['\n', '\r', '\u{2028}', '\u{2029}'])
        .filter_map(|line| line.strip_prefix("Refs:"))
        .flat_map(|ids| ids.split(','))
        .map(|id| id.trim_matches(crate::transcript::is_js_space))
        .filter(|id| is_task_id(id))
        .map(str::to_owned)
        .collect()
}

/// The task commits of a `git log` listing, records `hash␟author date␟subject␟body␞`, each commit
/// once, oldest first: a commit rebased onto another branch keeps its author date and subject,
/// which identify it, the last listed kept where the first was.
#[must_use]
pub fn task_commits(log: &str) -> Vec<TaskCommit> {
    let mut places: HashMap<String, usize> = HashMap::new();
    let mut unique: Vec<TaskCommit> = Vec::new();

    for record in log.split('\u{1e}') {
        let fields: Vec<&str> = record
            .trim_matches(crate::transcript::is_js_space)
            .split('\u{1f}')
            .collect();

        let [hash, date, subject, body] = fields[..] else {
            continue;
        };
        let commit = TaskCommit {
            hash: hash.to_owned(),
            subject: subject.to_owned(),
            tasks: task_ids(body),
            time: offset_instant_ms(date).unwrap_or(0),
        };

        if commit.tasks.is_empty() {
            continue;
        }

        let key = format!("{} {}", commit.time, commit.subject);

        upsert(&mut unique, &mut places, &key, commit);
    }

    unique.sort_by_key(|commit| commit.time);
    unique
}
