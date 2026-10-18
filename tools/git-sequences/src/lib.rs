//! The agent's frequent Git sequences, each in one call (EN-100, research note 0024): a worktree
//! refuses chained Git commands, so `fetch` then `rebase`, and `add` then `commit` then `push`,
//! cost one call of the agent per step. `git-sequences sync` and `commit` run each sequence in the
//! checkout they are started from and stop at the first failing step with Git's own message. What
//! they run is decided here, without Git.

use worktree::{base_branch, branch_refusal, story_of_branch, story_of_file};

/// The remote every sequence talks to.
pub const REMOTE: &str = "origin";

/// The integration branch: committed on by the main thread, pushed only through a pull request.
const DEVELOP: &str = "develop";

/// How a commit is pushed, if it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Push {
    /// Not pushed.
    No,
    /// `git push -u origin HEAD`.
    Plain,
    /// `git push --force-with-lease -u origin HEAD`, after a rebase, on a story branch only.
    Lease,
}

/// Where a commit message is read from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// A file, so that a message of several lines passes the shell guard.
    File(String),
    /// The standard input, a here-document.
    Stdin,
}

/// What the agent asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    /// `sync [--base <ref>]`: fetch, then bring the branch up to date with its base.
    Sync {
        /// A base typed instead of the story's feature branch.
        base: Option<String>,
    },
    /// `commit -F <file|-> [--push|--push-with-lease] [--] <path>…`.
    Commit {
        /// The message's source.
        message: Source,
        /// The files staged before the commit; none commits what is staged already.
        paths: Vec<String>,
        /// Whether the commit is pushed.
        push: Push,
    },
}

/// The usage printed on a malformed call.
pub const USAGE: &str = "usage: target/release/git-sequences sync [--base <ref>] | target/release/git-sequences commit -F <file|-> [--push|--push-with-lease] [--] <path>…";

/// The request of a command line, the subcommand first.
///
/// # Errors
///
/// An unknown subcommand or option, a missing value, or a commit without its message.
pub fn parse(argv: &[String]) -> Result<Request, String> {
    match argv.split_first() {
        Some((command, rest)) if command == "sync" => parse_sync(rest),
        Some((command, rest)) if command == "commit" => parse_commit(rest),
        _ => Err(USAGE.to_owned()),
    }
}

fn parse_sync(rest: &[String]) -> Result<Request, String> {
    match rest {
        [] => Ok(Request::Sync { base: None }),
        [flag, base] if flag == "--base" && !base.starts_with('-') => Ok(Request::Sync {
            base: Some(base.clone()),
        }),
        _ => Err(USAGE.to_owned()),
    }
}

fn parse_commit(rest: &[String]) -> Result<Request, String> {
    let mut message = None;
    let mut push = Push::No;
    let mut paths = Vec::new();
    let mut words = rest.iter();

    while let Some(word) = words.next() {
        match word.as_str() {
            "-F" | "--file" => {
                let value = words.next().ok_or("-F needs a file, or - for stdin")?;

                message = Some(if value == "-" {
                    Source::Stdin
                } else {
                    Source::File(value.clone())
                });
            }
            "--push" => push = Push::Plain,
            "--push-with-lease" => push = Push::Lease,
            "--" => paths.extend(words.by_ref().cloned()),
            option if option.starts_with('-') => {
                return Err(format!("unknown option `{option}`: {USAGE}"));
            }
            path => paths.push(path.to_owned()),
        }
    }

    let message = message.ok_or_else(|| format!("no message: {USAGE}"))?;

    Ok(Request::Commit {
        message,
        paths,
        push,
    })
}

/// The checked-out branch, as the sequences treat it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Branch {
    /// `main`: releases only, never touched.
    Main,
    /// No branch checked out.
    Detached,
    /// `<type>/<id>-<topic>`, a story's branch.
    Story(String),
    /// Any other branch: `develop`, `feature/f20-…`.
    Other(String),
}

/// The branch of `git rev-parse --abbrev-ref HEAD`.
#[must_use]
pub fn branch_of(name: &str) -> Branch {
    match name.trim() {
        "main" => Branch::Main,
        "" | "HEAD" => Branch::Detached,
        name if branch_refusal(name).is_none() => Branch::Story(name.to_owned()),
        name => Branch::Other(name.to_owned()),
    }
}

/// How `git-sequences sync` brings a branch up to date after the fetch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// A story branch rebased on its base, `origin/feature/f20-…`.
    Rebase(String),
    /// Any other branch moved forward to its upstream, never rebased: it is shared.
    FastForward,
}

/// The target of `git-sequences sync` on a branch.
///
/// # Errors
///
/// On `main` or a detached head; a base typed on a branch that is not a story's; a story branch
/// whose story file or feature branch is not found.
pub fn sync_target(
    branch: &Branch,
    base: Option<&str>,
    story_files: &[String],
    remote_branches: &[String],
) -> Result<Target, String> {
    match (branch, base) {
        (Branch::Main, _) => {
            Err("`main` takes releases only: git-sequences sync never touches it".into())
        }
        (Branch::Detached, _) => Err("no branch checked out: check out a branch first".into()),
        (Branch::Other(name), Some(_)) => Err(format!(
            "`{name}` is shared and never rebased: run git-sequences sync without --base to move it forward"
        )),
        (Branch::Other(_), None) => Ok(Target::FastForward),
        (Branch::Story(_), Some(base)) => Ok(Target::Rebase(base.to_owned())),
        (Branch::Story(name), None) => {
            let id = story_of_branch(name)
                .ok_or_else(|| format!("`{name}` names no story id such as en-100"))?;
            let story = story_files
                .iter()
                .find_map(|file| story_of_file(&id, file))
                .ok_or_else(|| {
                    format!("no file `E…-{id}-<topic>.md` in docs/backlog/stories/: give --base")
                })?;

            base_branch(story.feature.as_deref(), remote_branches).map(Target::Rebase)
        }
    }
}

/// Why `git-sequences commit` refuses to run on a branch, `None` when it may: never on `main` nor a detached
/// head, never a push of `develop`, never a force push of a shared branch. A feature branch is
/// pushed: the main thread writes the shared files of a merge there.
#[must_use]
pub fn commit_refusal(branch: &Branch, push: Push) -> Option<String> {
    match (branch, push) {
        (Branch::Main, _) => {
            Some("`main` takes releases only: git-sequences commit never touches it".into())
        }
        (Branch::Detached, _) => Some("no branch checked out: check out a branch first".into()),
        (Branch::Other(name), Push::Lease) => Some(format!(
            "`{name}` is shared: a force push, even with lease, only goes to a story branch"
        )),
        (Branch::Other(name), Push::Plain) if name == DEVELOP => Some(format!(
            "`{name}` takes merged pull requests only: git-sequences commit never pushes it"
        )),
        _ => None,
    }
}

/// Why a commit message is refused, `None` when it holds a subject.
#[must_use]
pub fn message_refusal(message: &str) -> Option<String> {
    message
        .lines()
        .all(|line| line.trim().is_empty())
        .then(|| "the commit message is empty".to_owned())
}

/// The Git steps of `git-sequences sync` after its fetch.
#[must_use]
pub fn sync_steps(target: &Target) -> Vec<Vec<String>> {
    let step = match target {
        Target::Rebase(base) => vec!["rebase", "--quiet", base.as_str()],
        Target::FastForward => vec!["merge", "--ff-only", "--quiet", "@{upstream}"],
    };

    vec![owned(&step)]
}

/// The step listing the uncommitted changes of tracked files, staged or not, against `HEAD`: the
/// changes a rebase refuses (an untracked file stops none).
#[must_use]
pub fn changes_step() -> Vec<String> {
    owned(&["diff", "--name-status", "HEAD"])
}

/// Why `git-sequences sync` does not rebase a tree with uncommitted changes, naming each file from
/// `git diff --name-status HEAD` (EN-103: it stopped saying only Git's refusal); `None` for a
/// fast-forward, which Git runs or refuses itself, or a clean tree.
#[must_use]
pub fn changes_refusal(name: &str, target: &Target, changes: &str) -> Option<String> {
    let files: Vec<String> = changes
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| format!("  {}", line.replacen('\t', " ", 1)))
        .collect();

    (matches!(target, Target::Rebase(_)) && !files.is_empty()).then(|| {
        format!(
            "{name} not rebased: the tree holds uncommitted changes, which the rebase refuses; commit or stash them, then run again:\n{}",
            files.join("\n")
        )
    })
}

/// The fetch that opens `git-sequences sync`.
#[must_use]
pub fn fetch_step() -> Vec<String> {
    owned(&["fetch", "--quiet", REMOTE])
}

/// The Git steps of `git-sequences commit`: the files staged, the commit from the standard input, the push.
/// No step bypasses the hooks or forces without lease.
#[must_use]
pub fn commit_steps(paths: &[String], push: Push) -> Vec<Vec<String>> {
    let mut steps = Vec::new();

    if !paths.is_empty() {
        let mut add = owned(&["add", "--"]);

        add.extend(paths.iter().cloned());
        steps.push(add);
    }

    steps.push(owned(&["commit", "--quiet", "-F", "-"]));

    match push {
        Push::No => {}
        Push::Plain => steps.push(owned(&["push", "--quiet", "-u", REMOTE, "HEAD"])),
        Push::Lease => steps.push(owned(&[
            "push",
            "--quiet",
            "--force-with-lease",
            "-u",
            REMOTE,
            "HEAD",
        ])),
    }

    steps
}

fn owned(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| word.to_owned()).collect()
}
