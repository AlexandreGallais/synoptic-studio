//! Which stories a `done` commit closes, which files their commits change, and which docs a rule
//! they change still lacks.

use std::path::Path;

/// The folder of the domain's rules.
const DOMAIN: &str = "docs/domain/";

/// The domain's index, holding the Q rows of its settled questions.
pub const Q_ROWS: &str = "docs/domain/README.md";

/// The folder of the story files, a feature's VAL among them.
const STORIES: &str = "docs/backlog/stories/";

/// The folder of the guide pages, one per feature (`f18-…`) or epic (`e17-…`).
const GUIDE: &str = "docs/guide/";

/// The line of a `done` commit's message saying no doc is affected, followed by its reason.
pub const NONE_AFFECTED: &str = "Rule-docs: none affected, ";

/// A doc a story that changes a rule brings in the same change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Doc {
    /// The Q row of `docs/domain/README.md`.
    QRow,
    /// The decisions to confirm of a feature's VAL, e.g. `F18`.
    Decisions(String),
    /// The guide page of a feature or an epic, by its file name's start, e.g. `f18-`.
    Guide(String),
}

impl Doc {
    /// Whether a changed file, relative to the repository's root, is this doc.
    #[must_use]
    pub fn is(&self, path: &str) -> bool {
        match self {
            Doc::QRow => path == Q_ROWS,
            Doc::Decisions(feature) => path
                .strip_prefix(STORIES)
                .is_some_and(|name| name.contains(&format!("-{feature}-VAL-"))),
            Doc::Guide(start) => path.strip_prefix(GUIDE).is_some_and(|name| {
                name.starts_with(start.as_str())
                    && !name.contains('/')
                    && Path::new(name)
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
            }),
        }
    }

    /// The doc, named for the refusal.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Doc::QRow => format!("the Q row in {Q_ROWS}"),
            Doc::Decisions(feature) => {
                format!(
                    "the decisions to confirm of {feature}'s VAL ({STORIES}*-{feature}-VAL-*.md)"
                )
            }
            Doc::Guide(start) => format!("the guide page {GUIDE}{start}*.md"),
        }
    }
}

/// The lines of a commit message, without the comment lines Git drops.
fn lines(message: &str) -> impl Iterator<Item = &str> {
    message.lines().filter(|line| !line.starts_with('#'))
}

/// The ids a footer of a message lists, e.g. `Closes: F18, EN-059` giving `F18` and `EN-059`.
fn footer<'a>(message: &'a str, key: &'a str) -> impl Iterator<Item = &'a str> {
    lines(message)
        .filter_map(move |line| line.strip_prefix(key))
        .flat_map(|ids| ids.split(','))
        .map(str::trim)
}

/// Whether an id is a story's, letters, a dash and digits: `EN-059`, not `F18` nor `EN-059.T1`.
#[must_use]
pub fn is_story(id: &str) -> bool {
    id.split_once('-').is_some_and(|(kind, number)| {
        !kind.is_empty()
            && kind.chars().all(|letter| letter.is_ascii_uppercase())
            && !number.is_empty()
            && number.chars().all(|digit| digit.is_ascii_digit())
    })
}

/// The stories a `done` commit closes (`Closes: <ID>`), in order, a feature or an epic left out.
#[must_use]
pub fn closed_stories(message: &str) -> Vec<String> {
    footer(message, "Closes:")
        .filter(|id| is_story(id))
        .map(str::to_owned)
        .collect()
}

/// Whether a commit is one of a story's: its `Refs:` or `Closes:` footer names the story or one of
/// its tasks (`EN-059`, `EN-059.T1`).
#[must_use]
pub fn of_story(message: &str, story: &str) -> bool {
    footer(message, "Refs:")
        .chain(footer(message, "Closes:"))
        .any(|id| {
            id.strip_prefix(story).is_some_and(|rest| {
                rest.is_empty()
                    || rest.strip_prefix(".T").is_some_and(|task| {
                        !task.is_empty() && task.chars().all(|digit| digit.is_ascii_digit())
                    })
            })
        })
}

/// Whether a message holds the line saying no doc is affected, with its reason.
#[must_use]
pub fn none_affected(message: &str) -> bool {
    lines(message).any(|line| {
        line.strip_prefix(NONE_AFFECTED)
            .is_some_and(|reason| !reason.trim().is_empty())
    })
}

/// The files the story's commits change, read from `git log --format=%x1e%B%x1f --name-only`: one
/// record per commit, its message, then its files one per line.
#[must_use]
pub fn story_files(log: &str, story: &str) -> Vec<String> {
    log.split('\u{1e}')
        .filter_map(|record| record.split_once('\u{1f}'))
        .filter(|(message, _)| of_story(message, story))
        .flat_map(|(_, files)| files.lines().map(str::trim).filter(|path| !path.is_empty()))
        .map(str::to_owned)
        .collect()
}

/// The docs a story brings when it changes a rule, from its story file's name
/// (`E17-F18-EN-059-topic.md`): the Q row, then for a feature's story its VAL's decisions and its
/// guide page, for an epic's story its guide page; `None` when the name is not the story's.
#[must_use]
pub fn docs_of(file_name: &str, story: &str) -> Option<Vec<Doc>> {
    let (place, _) = file_name.split_once(&format!("-{story}-"))?;
    let mut parts = place.split('-');
    let epic = parts.next().filter(|epic| epic.starts_with('E'))?;
    let feature = parts.next();

    if parts.next().is_some() {
        return None;
    }

    Some(match feature {
        Some(feature) if feature.starts_with('F') => vec![
            Doc::QRow,
            Doc::Decisions(feature.to_owned()),
            Doc::Guide(format!("{}-", feature.to_ascii_lowercase())),
        ],
        Some(_) => return None,
        None => vec![
            Doc::QRow,
            Doc::Guide(format!("{}-", epic.to_ascii_lowercase())),
        ],
    })
}

/// Whether a change touches a rule: a file of `docs/domain/`.
#[must_use]
pub fn changes_rule(files: &[String]) -> bool {
    files.iter().any(|path| path.starts_with(DOMAIN))
}

/// The docs a change lacks: none when it touches no rule.
#[must_use]
pub fn missing(docs: &[Doc], files: &[String]) -> Vec<Doc> {
    if !changes_rule(files) {
        return Vec::new();
    }

    docs.iter()
        .filter(|doc| !files.iter().any(|path| doc.is(path)))
        .cloned()
        .collect()
}

/// The refusal printed for a story, empty when no doc is missing.
#[must_use]
pub fn refusal(story: &str, missing: &[Doc]) -> String {
    if missing.is_empty() {
        return String::new();
    }

    let lines: Vec<String> = missing
        .iter()
        .map(|doc| format!("  {}", doc.name()))
        .collect();

    format!(
        "commit-msg: {story} changes a rule in {DOMAIN} and its change lacks (EN-059):\n{}\n\
         Bring them in this done commit, or add to its message the line \
         `{NONE_AFFECTED}<why>`.\n",
        lines.join("\n")
    )
}
