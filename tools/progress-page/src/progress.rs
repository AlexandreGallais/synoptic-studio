//! A run's progress, assembled from what was read (EN-087): each story's commits, agents, tokens
//! and duration, summed per feature. Decided without the file system nor Git.

use run_totals::tally::Tokens;

use crate::agents::{Agent, is_running};
use crate::backlog::StoryRow;
use crate::commits::Commit;

/// A feature of the run, as the feature branch holds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureInput {
    /// `E17`.
    pub epic: String,
    /// The epic's title.
    pub epic_title: String,
    /// `F20`.
    pub id: String,
    /// The feature's title.
    pub title: String,
    /// Its table of stories.
    pub rows: Vec<StoryRow>,
}

/// A story's progress.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoryProgress {
    /// Its row of the feature's table.
    pub row: StoryRow,
    /// Being worked on: not `done` nor `dropped`, with a worktree or an agent running.
    pub in_progress: bool,
    /// Its first activity of the run (commit or agent), epoch milliseconds; 0 for none.
    pub start: i64,
    /// Its `done` commit, else now while an agent of it runs, else its last activity; 0 for none.
    pub end: i64,
    /// The tokens of its agents.
    pub tokens: Tokens,
    /// Its commits of the run, oldest first.
    pub commits: Vec<Commit>,
}

impl StoryProgress {
    /// Its duration in the run, 0 without activity.
    #[must_use]
    pub fn duration_ms(&self) -> i64 {
        if self.start > 0 && self.end >= self.start {
            self.end - self.start
        } else {
            0
        }
    }
}

/// A feature's progress, its stories in the table's order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureProgress {
    /// The feature as read.
    pub feature: FeatureInput,
    /// Its stories.
    pub stories: Vec<StoryProgress>,
}

impl FeatureProgress {
    /// From its stories' first start to their last end, 0 without activity.
    #[must_use]
    pub fn duration_ms(&self) -> i64 {
        let start = self
            .stories
            .iter()
            .map(|story| story.start)
            .filter(|start| *start > 0)
            .min()
            .unwrap_or(0);
        let end = self
            .stories
            .iter()
            .map(|story| story.end)
            .max()
            .unwrap_or(0);

        if start > 0 && end >= start {
            end - start
        } else {
            0
        }
    }

    /// Its stories' tokens.
    #[must_use]
    pub fn tokens(&self) -> Tokens {
        self.stories
            .iter()
            .fold(Tokens::default(), |sum, story| sum.plus(&story.tokens))
    }

    /// Its stories' commits, counted.
    #[must_use]
    pub fn commit_count(&self) -> usize {
        self.stories.iter().map(|story| story.commits.len()).sum()
    }
}

/// A story's progress from the run's `commits`, `agents` and the story ids having a worktree, at
/// `now`.
#[must_use]
pub fn story_progress(
    row: &StoryRow,
    commits: &[Commit],
    agents: &[Agent],
    worktrees: &[String],
    now: i64,
) -> StoryProgress {
    let mut own: Vec<Commit> = commits
        .iter()
        .filter(|commit| commit.is_of(&row.id))
        .cloned()
        .collect();
    let workers: Vec<&Agent> = agents
        .iter()
        .filter(|agent| agent.story.as_deref() == Some(row.id.as_str()))
        .collect();
    let working = workers.iter().any(|agent| is_running(agent, now));
    let finished = matches!(row.status.as_str(), "done" | "dropped");
    let in_progress =
        !finished && (working || worktrees.iter().any(|id| id.eq_ignore_ascii_case(&row.id)));

    own.sort_by_key(|commit| commit.time);

    let start = own
        .iter()
        .map(|commit| commit.time)
        .chain(workers.iter().map(|agent| agent.start))
        .filter(|time| *time > 0)
        .min()
        .unwrap_or(0);
    let closed = own
        .iter()
        .filter(|commit| commit.closes.contains(&row.id))
        .map(|commit| commit.time)
        .max();
    let last = own
        .iter()
        .map(|commit| commit.time)
        .chain(workers.iter().map(|agent| agent.last))
        .max()
        .unwrap_or(0);
    let end = match closed {
        Some(time) => time,
        None if working => now,
        None => last,
    };

    StoryProgress {
        row: row.clone(),
        in_progress,
        start,
        end,
        tokens: workers
            .iter()
            .fold(Tokens::default(), |sum, agent| sum.plus(&agent.tokens)),
        commits: own,
    }
}

/// Each feature's progress.
#[must_use]
pub fn features_progress(
    features: &[FeatureInput],
    commits: &[Commit],
    agents: &[Agent],
    worktrees: &[String],
    now: i64,
) -> Vec<FeatureProgress> {
    features
        .iter()
        .map(|feature| FeatureProgress {
            feature: feature.clone(),
            stories: feature
                .rows
                .iter()
                .map(|row| story_progress(row, commits, agents, worktrees, now))
                .collect(),
        })
        .collect()
}
