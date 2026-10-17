//! The commits of a run, from `git log` (EN-087): each tied to its stories by its footers, `Refs:
//! EN-087.T1` for a task and `Closes: EN-087` for the `done` commit. A rebased commit keeps its
//! author time and subject: the same pair seen twice is counted once.

/// The format given to `git log --format`: hash, author time in seconds, subject, body; fields
/// split by `0x1f`, commits by `0x1e`.
pub const LOG_FORMAT: &str = "%H%x1f%at%x1f%s%x1f%B%x1e";

/// A commit, as the page shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    /// The full hash.
    pub hash: String,
    /// Author time, epoch milliseconds.
    pub time: i64,
    /// The subject line.
    pub subject: String,
    /// The stories its `Refs:` footers name.
    pub refs: Vec<String>,
    /// The stories its `Closes:` footers name.
    pub closes: Vec<String>,
}

impl Commit {
    /// Whether the commit belongs to `story`, by a task or by its closing.
    #[must_use]
    pub fn is_of(&self, story: &str) -> bool {
        self.refs.iter().chain(&self.closes).any(|id| id == story)
    }
}

/// The story of a reference: `EN-087.T1` and `EN-087` give `EN-087`; `None` for a word that is
/// not one (`#12`, `F20`).
#[must_use]
pub fn story_of_reference(word: &str) -> Option<String> {
    let id = word.split('.').next()?;
    let (kind, number) = id.split_once('-')?;

    (kind.len() >= 2
        && kind.chars().all(|letter| letter.is_ascii_uppercase())
        && !number.is_empty()
        && number.chars().all(|digit| digit.is_ascii_digit()))
    .then(|| id.to_owned())
}

/// The stories named by the footers `key:` of a message, several allowed per line, split by
/// commas or spaces.
#[must_use]
pub fn footer_stories(body: &str, key: &str) -> Vec<String> {
    let prefix = format!("{key}:");
    let mut stories: Vec<String> = Vec::new();

    for line in body.lines() {
        if let Some(rest) = line.trim().strip_prefix(&prefix) {
            for story in rest
                .split([',', ' '])
                .filter_map(|word| story_of_reference(word.trim()))
            {
                if !stories.contains(&story) {
                    stories.push(story);
                }
            }
        }
    }

    stories
}

/// The commits of `git log --format=LOG_FORMAT`, newest first as Git gives them, each rebased
/// copy dropped.
#[must_use]
pub fn parse_log(text: &str) -> Vec<Commit> {
    let mut commits: Vec<Commit> = Vec::new();

    for record in text.split('\u{1e}') {
        let fields: Vec<&str> = record.trim_start_matches('\n').split('\u{1f}').collect();

        let [hash, time, subject, body] = fields[..] else {
            continue;
        };
        let commit = Commit {
            hash: hash.to_owned(),
            time: time.parse::<i64>().unwrap_or(0) * 1000,
            subject: subject.to_owned(),
            refs: footer_stories(body, "Refs"),
            closes: footer_stories(body, "Closes"),
        };

        if !commits
            .iter()
            .any(|seen| seen.time == commit.time && seen.subject == commit.subject)
        {
            commits.push(commit);
        }
    }

    commits
}
