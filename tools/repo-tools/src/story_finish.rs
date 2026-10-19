//! The end of a story in a run (RET-002, ADR-0045, ADR-0065, EN-051, EN-103): what `story-finish`
//! decides — where it starts from the story's pull request, when it stops waiting, the clean-up of a
//! merged story and the same steps to type when it stopped midway — and the Git sequences it runs,
//! each given its folder: the rebase on the base, whose conflicts in the backlog's status rows only
//! are left to the base and the rows written again after it, the push with lease of a head that
//! differs from its remote's. The story's rows for the logs are in its `done` commit (ADR-0091):
//! a story file still holding some stops it before its pull request, and nothing is committed on
//! the base after the merge. Who merges, GitHub or the Product Owner, as `story-automerge.yml`
//! takes the branch into its base (EN-118).

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use git_repository::stdout_of;
use shared_files::backlog::{STORIES, file_of};
use shared_files::rows::take_rows;
use shared_files::shared::story_of;
use stale_binaries::RELEASE_BUILD;

use crate::backlog_status::{front_field, item_file, write_status};

/// Readings of the pull request's state before giving up: three minutes, one each 10 s.
pub const POLLS: u32 = 18;

/// The generated stories index: a conflict there is left to the base, then written again.
const STORIES_INDEX: &str = "docs/backlog/stories/README.md";

/// Where a run starts, from the story's pull request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    /// None, or closed: one is opened.
    Create,
    /// An open one is awaited.
    Await,
    /// A merged one goes straight to the clean-up.
    Clean,
}

/// Where a run starts from the pull request's state (`""` when there is none), so that a run
/// stopped midway is run again as it is (AUD-020).
#[must_use]
pub fn start_step(state: &str) -> Start {
    match state {
        "MERGED" => Start::Clean,
        "OPEN" => Start::Await,
        _ => Start::Create,
    }
}

/// What to do after a reading of the pull request's state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Next {
    /// Merged: clean up.
    Clean,
    /// Open and time left: read it again.
    Wait,
    /// Closed, or out of time.
    Stop,
}

/// What to do after a reading, the readings made so far counting this one.
#[must_use]
pub fn next_step(state: &str, readings: u32) -> Next {
    if state == "MERGED" {
        Next::Clean
    } else if state == "OPEN" && readings < POLLS {
        Next::Wait
    } else {
        Next::Stop
    }
}

/// Who merges a story's pull request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Merger {
    /// `story-automerge.yml`, once the pull request is labelled `autonomous`: awaited.
    GitHub,
    /// The Product Owner: opened without the label, never awaited (AUD-024).
    ProductOwner,
}

/// Who merges a story branch into its base, as `story-automerge.yml` lets it through: into a
/// feature branch a `US`, `EN`, `SP`, `CHK`, `AUD` or `VAL`, into `develop` a `REV`; anything else,
/// an epic's refinement `SP` into `develop` first, is the Product Owner's.
#[must_use]
pub fn merger(base: &str, branch: &str) -> Merger {
    let kind = story_of(branch)
        .and_then(|id| id.split_once('-').map(|(kind, _)| kind.to_owned()))
        .unwrap_or_default();
    let taken: &[&str] = if base == "develop" {
        &["REV"]
    } else if base.starts_with("feature/") {
        &["US", "EN", "SP", "CHK", "AUD", "VAL"]
    } else {
        &[]
    };

    if taken.contains(&kind.as_str()) {
        Merger::GitHub
    } else {
        Merger::ProductOwner
    }
}

/// The auto-merge workflow a pull request into a base starts, as its trigger's `branches` filter
/// chooses it (EN-134): `feature-automerge.yml` into `develop`, `story-automerge.yml` otherwise,
/// the one into a feature branch; its runs are what a merge not done is read from.
#[must_use]
pub fn auto_merge_of(base: &str) -> &'static str {
    let _ = base;
    "story-automerge.yml"
}

/// A checkout of the repository: the main one or a linked worktree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worktree {
    /// The branch checked out, empty when detached.
    pub branch: String,
    /// Its folder.
    pub path: String,
}

/// The checkouts of the repository, the main one first, from `git worktree list --porcelain`.
#[must_use]
pub fn worktrees(porcelain: &str) -> Vec<Worktree> {
    let field = |lines: &[&str], key: &str| {
        let start = format!("{key} ");

        lines
            .iter()
            .find_map(|line| line.strip_prefix(&start))
            .unwrap_or_default()
            .to_owned()
    };

    porcelain
        .split("\n\n")
        .filter(|block| block.starts_with("worktree "))
        .map(|block| {
            let lines: Vec<&str> = block.split('\n').collect();
            let branch = field(&lines, "branch");

            Worktree {
                branch: branch
                    .strip_prefix("refs/heads/")
                    .unwrap_or(&branch)
                    .to_owned(),
                path: field(&lines, "worktree"),
            }
        })
        .collect()
}

/// A command and the folder it runs in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// The program: `git`, or `gh` for the branch on GitHub.
    pub command: &'static str,
    /// The folder.
    pub cwd: String,
    /// Its arguments.
    pub options: Vec<String>,
}

impl Step {
    fn new(command: &'static str, cwd: &str, options: &[&str]) -> Step {
        Step {
            command,
            cwd: cwd.to_owned(),
            options: options.iter().map(|&option| option.to_owned()).collect(),
        }
    }
}

/// The clean-up of a merged story.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CleanUp {
    /// The checkout of the base, brought up to date.
    pub place: String,
    /// The base brought up to its remote.
    pub update: Vec<Step>,
    /// The story's worktree and branch removed, here then on GitHub, from the main checkout.
    pub remove: Vec<Step>,
}

/// A story and the branch it merges into.
#[derive(Clone, Copy, Debug)]
pub struct Story<'a> {
    /// The base branch, `feature/f18-topic`.
    pub base: &'a str,
    /// The story's branch.
    pub branch: &'a str,
    /// The repository on GitHub, `owner/name`.
    pub repository: &'a str,
}

/// The clean-up of a merged story, from the checkout it was finished in. From a worktree, the
/// base is brought up to date where it is checked out — the main checkout in a run — without
/// switching any branch, then the worktree is removed; elsewhere the checkout switches to the base
/// first. Every step after the worktree's removal runs from the main checkout: the worktree may be
/// the checkout of the base, and a command started in a removed folder fails (EN-051, PR #272).
#[must_use]
pub fn clean_up_plan(story: Story, here: &str, checkouts: &[Worktree]) -> CleanUp {
    let main = checkouts
        .first()
        .map_or(here, |checkout| checkout.path.as_str());
    let is_linked = checkouts
        .first()
        .is_none_or(|checkout| checkout.path != here);
    let place = checkouts
        .iter()
        .find(|checkout| is_linked && checkout.branch == story.base)
        .map_or(here, |checkout| checkout.path.as_str());
    let mut update = vec![Step::new(
        "git",
        here,
        &["fetch", "--quiet", "--prune", "origin"],
    )];
    let mut remove = Vec::new();
    let reference = format!("repos/{}/git/refs/heads/{}", story.repository, story.branch);

    if place == here {
        update.push(Step::new("git", here, &["switch", "--quiet", story.base]));
    }
    update.push(Step::new(
        "git",
        place,
        &[
            "merge",
            "--ff-only",
            "--quiet",
            &format!("origin/{}", story.base),
        ],
    ));
    // The base's checkout rebuilds its release binaries, which its hooks and the agent call by
    // their path (ADR-0078); a worktree switched to the base is removed, so nothing is built there.
    if place != here || here == main {
        update.push(Step::new("cargo", place, &RELEASE_BUILD));
    }
    if here != main {
        remove.push(Step::new("git", main, &["worktree", "remove", here]));
    }
    remove.push(Step::new("git", main, &["branch", "-D", story.branch]));
    // Through GitHub's API rather than `git push --delete`, which would run the pre-push checks.
    remove.push(Step::new("gh", main, &["api", "-X", "DELETE", &reference]));

    CleanUp {
        place: place.to_owned(),
        update,
        remove,
    }
}

/// The rows and card lines a story file still holds for the logs, which its `done` commit moves
/// (`move-rows`, ADR-0091), and the lines `move-rows` leaves under a log's heading, a card at
/// another level among them (AUD-025): `story-finish` opens no pull request while there are some.
#[must_use]
pub fn rows_left(story: &str) -> usize {
    let taken = take_rows(story);

    taken
        .rows
        .iter()
        .chain(&taken.left)
        .map(|(_, lines)| lines.len())
        .sum()
}

/// Why a story cannot be finished yet, from its file's text at the head, or the error reading it:
/// the rows and lines for the logs it still holds, which its `done` commit, made again, moves;
/// `None` when it holds none.
#[must_use]
pub fn story_refusal(path: &str, branch: &str, story: Result<&str, &str>) -> Option<String> {
    let text = match story {
        Ok(text) => text,
        Err(error) => {
            return Some(format!(
                "{path} cannot be read at HEAD ({error}): its rows for the logs are unknown, \
                 so the story is not finished (ADR-0091)"
            ));
        }
    };
    let count = rows_left(text);

    (count > 0).then(|| {
        format!(
            "{path} still holds lines for the logs ({count}) at HEAD: make the done commit again \
             (ADR-0091): `git reset --soft HEAD~1`, `target/release/move-rows {branch}`, the lines \
             it names as left written as rows or moved out of their heading by hand, then \
             `target/release/git-sequences commit -F <message file> --push-with-lease <the files \
             it printed> docs/backlog`, and run again"
        )
    })
}

/// Why the story cannot be finished yet: its file, read at the head that is pushed (`git show
/// HEAD:<path>`), still holds rows or lines for the logs, or cannot be read; `None` when it holds
/// none, or the branch names no story file at the head (AUD-025).
#[must_use]
pub fn rows_refusal(branch: &str, cwd: &Path) -> Option<String> {
    let id = story_of(branch)?;
    let folder = format!("{STORIES}/");
    let names: Vec<String> =
        match run_words("git", &["ls-tree", "--name-only", "HEAD", &folder], cwd) {
            Ok(listed) => listed
                .lines()
                .filter_map(|path| path.strip_prefix(&folder))
                .map(str::to_owned)
                .collect(),
            Err(error) => return story_refusal(STORIES, branch, Err(&error)),
        };
    let path = format!("{folder}{}", file_of(&names, &id)?);
    let story = run_words("git", &["show", &format!("HEAD:{path}")], cwd);

    story_refusal(&path, branch, story.as_deref().map_err(String::as_str))
}

/// A step written as a command to type: `cd "<folder>" && <command> <arguments>`.
fn typed(step: &Step) -> String {
    format!(
        "cd \"{}\" && {} {}",
        step.cwd,
        step.command,
        step.options.join(" ")
    )
}

/// The clean-up's commands to type when it stopped midway, one per line: the base brought up to
/// date, then the worktree and the branch removed (AUD-020).
#[must_use]
pub fn manual_steps(plan: &CleanUp) -> String {
    plan.update
        .iter()
        .chain(&plan.remove)
        .map(typed)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Whether a conflicted file is one of the backlog's status rows the done commit writes: an index
/// or a feature's or an epic's file, never a story file nor anything outside `docs/backlog/`.
#[must_use]
pub fn is_status_file(path: &str) -> bool {
    path == STORIES_INDEX
        || path.strip_prefix("docs/backlog/").is_some_and(|rest| {
            rest.strip_prefix("features/")
                .or_else(|| rest.strip_prefix("epics/"))
                .is_some_and(|name| {
                    !name.contains('/')
                        && Path::new(name)
                            .extension()
                            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
                })
        })
}

/// A file's conflicts left to the base's side, written as Git leaves them (`<<<<<<<`, an optional
/// `|||||||` base, `=======`, `>>>>>>>`): the text with the base's lines in each, or `None` when a
/// conflict holds a line outside a table, or a conflict is not closed.
#[must_use]
pub fn resolved_to_base(text: &str) -> Option<String> {
    let mut kept = Vec::new();
    let mut lines = text.split('\n');

    while let Some(line) = lines.next() {
        if !line.starts_with("<<<<<<< ") && line != "<<<<<<<" {
            kept.push(line);
            continue;
        }

        // Ours, in a rebase, is the base the story is replayed on.
        let mut section = 0;

        loop {
            let line = lines.next()?;

            if line.starts_with("|||||||") {
                section = 1;
            } else if line == "=======" {
                section = 2;
            } else if line.starts_with(">>>>>>>") {
                break;
            } else if !line.starts_with('|') {
                return None;
            } else if section == 0 {
                kept.push(line);
            }
        }
    }

    Some(kept.join("\n"))
}

/// The error of a command that failed, as Node.js's `execFileSync` wrote it: `Error: Command failed:
/// <command> <arguments>` and its standard error.
fn failure(command: &str, options: &[String], stderr: &str) -> String {
    let said = format!("Error: Command failed: {command} {}", options.join(" "));

    if stderr.is_empty() {
        said
    } else {
        format!("{said}\n{stderr}")
    }
}

/// Runs a command in a folder, its standard error shown, and its trimmed standard output.
///
/// # Errors
/// When it cannot start or fails: the text `execFileSync` would have thrown.
pub fn run(command: &str, options: &[String], cwd: &Path) -> Result<String, String> {
    let output = Command::new(command)
        .args(options)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("Error: spawnSync {command} {error}"))?;
    let stderr = String::from_utf8_lossy(&output.stderr);

    let _ = io::stderr().write_all(&output.stderr);
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        Err(failure(command, options, &stderr))
    }
}

/// [`run`] of words.
///
/// # Errors
/// As [`run`].
pub fn run_words(command: &str, words: &[&str], cwd: &Path) -> Result<String, String> {
    run(
        command,
        &words
            .iter()
            .map(|&word| word.to_owned())
            .collect::<Vec<_>>(),
        cwd,
    )
}

/// A command run with its output shown as it runs (`stdio: "inherit"`).
///
/// # Errors
/// As [`run`].
pub fn run_shown(command: &str, words: &[&str], cwd: &Path) -> Result<(), String> {
    let options: Vec<String> = words.iter().map(|&word| word.to_owned()).collect();
    let status = Command::new(command)
        .args(&options)
        .current_dir(cwd)
        .status()
        .map_err(|error| format!("Error: spawnSync {command} {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(failure(command, &options, ""))
    }
}

/// Whether a Git command, its output hidden, ends with success.
fn git_succeeds(words: &[&str], cwd: &Path) -> bool {
    Command::new("git")
        .args(words)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// How the story's branch met its base.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rebase {
    /// The branch already held the base.
    UpToDate,
    /// The branch was rebased; the status files whose conflicts were left to the base.
    Rebased(Vec<String>),
}

/// The text of a file at one stage of a conflict (`:2:` ours), `None` when that stage is missing.
fn stage(path: &str, number: u8, cwd: &Path) -> Option<String> {
    run_quiet(&["show", &format!(":{number}:{path}")], cwd)
}

/// A Git command's output, its standard error kept back; `None` when it fails.
fn run_quiet(words: &[&str], cwd: &Path) -> Option<String> {
    stdout_of(
        Command::new("git")
            .args(words)
            .current_dir(cwd)
            .stdin(Stdio::null()),
    )
    .ok()
}

/// Leaves the conflicts of the rebase step that stopped to the base, when each conflicted file is
/// a status file whose conflicts are table rows only: the stories index whole (it is written
/// again), the rows of a feature or an epic; the files resolved, or `None` to give up.
fn resolve_status_conflicts(cwd: &Path) -> Option<Vec<String>> {
    let conflicted: Vec<String> = run_quiet(&["diff", "--name-only", "--diff-filter=U"], cwd)?
        .lines()
        .map(str::to_owned)
        .collect();

    if conflicted.is_empty() || !conflicted.iter().all(|path| is_status_file(path)) {
        return None;
    }

    for path in &conflicted {
        let text = if path == STORIES_INDEX {
            stage(path, 2, cwd)?
        } else {
            resolved_to_base(&fs::read_to_string(cwd.join(path)).ok()?)?
        };

        fs::write(cwd.join(path), text).ok()?;
        if !git_succeeds(&["add", "--", path], cwd) {
            return None;
        }
    }

    Some(conflicted)
}

/// Brings the story's branch onto its base as the remote holds it now, after another story merged.
/// A conflict in the status rows only — the next story's done commit beside the first's — is left
/// to the base, whose rows the story's are written again over ([`write_status_again`]); any other
/// conflict aborts the rebase, the branch left as it was.
///
/// # Errors
/// On a conflict it does not resolve, or when the fetch fails.
pub fn rebase_on_base(base: &str, cwd: &Path) -> Result<Rebase, String> {
    let remote = format!("origin/{base}");

    run_words("git", &["fetch", "--quiet", "origin", base], cwd)?;
    if git_succeeds(&["merge-base", "--is-ancestor", &remote, "HEAD"], cwd) {
        return Ok(Rebase::UpToDate);
    }

    let conflict = || format!("conflict rebasing on {remote}: rebase by hand, then run again");
    let mut resolved: Vec<String> = Vec::new();
    let mut step = git_succeeds(
        &[
            "-c",
            "merge.conflictStyle=merge",
            "rebase",
            "--quiet",
            &remote,
        ],
        cwd,
    );

    while !step {
        let Some(files) = resolve_status_conflicts(cwd) else {
            let _ = git_succeeds(&["rebase", "--abort"], cwd);
            return Err(conflict());
        };

        for file in files {
            if !resolved.contains(&file) {
                resolved.push(file);
            }
        }
        // The message kept as it is: no editor opens, whatever `GIT_EDITOR` says.
        step = Command::new("git")
            .args(["-c", "merge.conflictStyle=merge", "rebase", "--continue"])
            .env("GIT_EDITOR", "true")
            .current_dir(cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success());
    }

    Ok(Rebase::Rebased(resolved))
}

/// Whether a commit message closes a story (`Closes: <ID>`): the story's `done` commit.
#[must_use]
pub fn closes_story(message: &str, story: &str) -> bool {
    message
        .lines()
        .filter_map(|line| line.strip_prefix("Closes:"))
        .flat_map(|ids| ids.split(','))
        .any(|id| id.trim() == story)
}

/// After a rebase, the story's status written again from its front matter — its file, the stories
/// index generated again, its feature's row — and folded into its `done` commit, so that no status
/// row is carried through the rebase from the base the story started on. Nothing when the head is
/// not the story's `done` commit or the rows are already right; else the files written.
///
/// # Errors
/// When a file cannot be written or Git fails.
pub fn write_status_again(branch: &str, cwd: &Path) -> Result<Vec<String>, String> {
    let Some(id) = story_of(branch) else {
        return Ok(Vec::new());
    };
    let message = run_words("git", &["log", "-1", "--format=%B"], cwd)?;
    let file = item_file(cwd, &id);

    if !closes_story(&message, &id) || file.is_empty() {
        return Ok(Vec::new());
    }

    let story = fs::read_to_string(cwd.join(STORIES).join(&file))
        .map_err(|error| format!("{file}: {error}"))?;
    let status = front_field(&story, "status");

    if !write_status(cwd, &file, &id, &status).map_err(|error| error.to_string())? {
        return Err("backlog:index failed".to_owned());
    }

    let changed: Vec<String> = run_words(
        "git",
        &["diff", "--name-only", "HEAD", "--", "docs/backlog"],
        cwd,
    )?
    .lines()
    .map(str::to_owned)
    .collect();

    if !changed.is_empty() {
        let mut words = vec!["commit", "--quiet", "--amend", "--no-edit", "--"];

        words.extend(changed.iter().map(String::as_str));
        run_words("git", &words, cwd)?;
    }
    Ok(changed)
}

/// Whether the story's head differs from its branch on GitHub as last fetched: a rebase, or a done
/// commit rebased by hand, not pushed yet. A branch never pushed differs too.
#[must_use]
pub fn needs_push(cwd: &Path, branch: &str) -> bool {
    let head = run_quiet(&["rev-parse", "HEAD"], cwd);
    let remote = run_quiet(
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/remotes/origin/{branch}"),
        ],
        cwd,
    );

    head.is_none() || head != remote
}

/// Pushes the story's branch with lease: refused when GitHub's branch moved since it was fetched.
///
/// # Errors
/// When the push fails.
pub fn push_with_lease(branch: &str, cwd: &Path) -> Result<(), String> {
    run_shown(
        "git",
        &["push", "--quiet", "--force-with-lease", "origin", branch],
        cwd,
    )
}

/// Cleans up a story once its pull request merged: the base brought up to date, the worktree and
/// the branch removed here and on GitHub; nothing is committed on the base (ADR-0091). A pull
/// request not merged keeps everything: false.
///
/// # Errors
/// The first step that failed.
pub fn clean_up(state: &str, plan: &CleanUp) -> Result<bool, String> {
    if state != "MERGED" {
        return Ok(false);
    }

    for step in plan.update.iter().chain(&plan.remove) {
        run(step.command, &step.options, Path::new(&step.cwd))?;
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "feature/f99-topic";
    const STORY: &str = "chore/en-999-topic";
    const STORY_OF_BASE: Story = Story {
        base: BASE,
        branch: STORY,
        repository: "owner/name",
    };

    fn checkout(branch: &str, path: &str) -> Worktree {
        Worktree {
            branch: branch.to_owned(),
            path: path.to_owned(),
        }
    }

    fn step(command: &'static str, cwd: &str, options: &[&str]) -> Step {
        Step::new(command, cwd, options)
    }

    fn remote_deletion() -> Vec<&'static str> {
        vec![
            "api",
            "-X",
            "DELETE",
            "repos/owner/name/git/refs/heads/chore/en-999-topic",
        ]
    }

    #[test]
    fn cleans_once_merged_waits_while_open_for_three_minutes_then_stops() {
        assert_eq!(next_step("MERGED", 1), Next::Clean);
        assert_eq!(next_step("OPEN", 17), Next::Wait);
        assert_eq!(next_step("OPEN", 18), Next::Stop);
        assert_eq!(next_step("CLOSED", 1), Next::Stop);
    }

    #[test]
    fn f23_ac2_leaves_to_the_product_owner_what_the_auto_merge_does_not_take() {
        // [F23.AC2] A `REV` into `develop` and a story into its feature are merged by GitHub;
        // an epic's `SP`, or any other story, into `develop` by the Product Owner (AUD-024).
        assert_eq!(merger("develop", "docs/rev-006-review"), Merger::GitHub);
        assert_eq!(
            merger("develop", "docs/sp-030-refinement-of-e18"),
            Merger::ProductOwner
        );
        assert_eq!(
            merger("develop", "chore/en-114-a-tool"),
            Merger::ProductOwner
        );
        for branch in [
            "feat/us-061-x",
            "chore/en-118-x",
            "docs/sp-023-x",
            "docs/chk-012-x",
            "docs/aud-025-x",
            "feat/val-024-x",
        ] {
            assert_eq!(merger(BASE, branch), Merger::GitHub, "{branch}");
        }
        // A review into a feature, a branch without a story id: the auto-merge refuses both.
        assert_eq!(merger(BASE, "docs/rev-006-review"), Merger::ProductOwner);
        assert_eq!(merger(BASE, "fix/tidy-up"), Merger::ProductOwner);
        assert_eq!(merger("develop", "fix/tidy-up"), Merger::ProductOwner);
        // Nothing reaches `main` by the auto-merge.
        assert_eq!(merger("main", "chore/en-118-x"), Merger::ProductOwner);
    }

    #[test]
    fn f34_ac1_names_the_auto_merge_each_base_starts() {
        // [F34.AC1] A feature or a review into `develop`, a story into its feature branch.
        assert_eq!(auto_merge_of("develop"), "feature-automerge.yml");
        assert_eq!(auto_merge_of(BASE), "story-automerge.yml");
    }

    #[test]
    fn opens_a_pull_request_when_there_is_none_awaits_an_open_one_cleans_after_a_merged_one() {
        assert_eq!(start_step(""), Start::Create);
        assert_eq!(start_step("CLOSED"), Start::Create);
        assert_eq!(start_step("OPEN"), Start::Await);
        assert_eq!(start_step("MERGED"), Start::Clean);
    }

    #[test]
    fn writes_the_clean_up_steps_to_type_when_it_stopped_midway_in_their_order() {
        let here = "/repo/.claude/worktrees/en-999";
        let plan = clean_up_plan(
            STORY_OF_BASE,
            here,
            &[checkout(BASE, "/repo"), checkout(STORY, here)],
        );

        assert_eq!(
            manual_steps(&plan).split('\n').collect::<Vec<_>>(),
            [
                format!("cd \"{here}\" && git fetch --quiet --prune origin"),
                format!("cd \"/repo\" && git merge --ff-only --quiet origin/{BASE}"),
                "cd \"/repo\" && cargo build --release --workspace --locked --quiet".to_owned(),
                // [F23.AC1] no rows moved nor pushed after the merge: they are in the done commit.
                format!("cd \"/repo\" && git worktree remove {here}"),
                format!("cd \"/repo\" && git branch -D {STORY}"),
                format!("cd \"/repo\" && gh {}", remote_deletion().join(" ")),
            ]
        );
    }

    #[test]
    fn f23_ac1_counts_the_rows_and_card_lines_a_story_file_still_holds() {
        // Two journal bullets and one card of two lines: 4; a note, a placeholder and a line of
        // another shape are not rows (`move-rows` leaves them).
        let story = "# S\n\n## For the main thread\n\nA note.\n\n### Journal\n\n- `#run` a\n- `#open` b\n\n### Feedback\n\n- None.\n\n### Improvements\n\n- Not a row.\n\n### Incidents\n\n#### 2026-10-19 — a card\n\n- Used: c\n";

        assert_eq!(rows_left(story), 5);
        assert_eq!(rows_left("# S\n\n## For the main thread\n\nA note.\n"), 0);
        assert_eq!(rows_left("# S\n"), 0);
        // A card written at the wrong level moves nothing but its two lines are left: 2
        // (AUD-025).
        assert_eq!(
            rows_left(
                "# S\n\n## For the main thread\n\n### Incidents\n\n### 2026-10-19 — a card\n\n- Used: a\n"
            ),
            2
        );
    }

    #[test]
    fn f23_ac1_refuses_a_story_whose_file_holds_lines_or_cannot_be_read() {
        let path = "docs/backlog/stories/E99-F99-EN-901-first.md";
        let branch = "chore/en-901-first";
        let rows = "# S\n\n## For the main thread\n\n### Journal\n\n- `#run` a\n";
        let refusal = story_refusal(path, branch, Ok(rows)).unwrap_or_default();

        // The done commit made again (ADR-0091): undone, the rows moved, committed and pushed by
        // `git-sequences`, never amended (AUD-025).
        for part in [
            "docs/backlog/stories/E99-F99-EN-901-first.md still holds lines for the logs (1)",
            "git reset --soft HEAD~1",
            "target/release/move-rows chore/en-901-first",
            "target/release/git-sequences commit -F",
            "--push-with-lease",
        ] {
            assert!(refusal.contains(part), "{part}: {refusal}");
        }
        assert!(!refusal.contains("--amend"), "{refusal}");
        assert_eq!(story_refusal(path, branch, Ok("# S\n")), None);

        // A story file that cannot be read at the head is a refusal, never a pass.
        let unread = story_refusal(path, branch, Err("fatal: bad object")).unwrap_or_default();

        assert!(unread.contains(path), "{unread}");
        assert!(unread.contains("fatal: bad object"), "{unread}");
    }

    #[test]
    fn reads_each_checkout_folder_and_branch_the_main_one_first() {
        let porcelain = [
            "worktree /repo\nHEAD 1111\nbranch refs/heads/feature/f18-topic\n",
            "worktree /repo/.claude/worktrees/en-051\nHEAD 2222\nbranch refs/heads/chore/en-051-topic\n",
            "worktree /repo/.claude/worktrees/spare\nHEAD 3333\ndetached\n",
        ]
        .join("\n");

        assert_eq!(
            worktrees(&porcelain),
            [
                checkout("feature/f18-topic", "/repo"),
                checkout("chore/en-051-topic", "/repo/.claude/worktrees/en-051"),
                checkout("", "/repo/.claude/worktrees/spare"),
            ]
        );
        assert_eq!(worktrees(""), []);
    }

    #[test]
    fn from_a_worktree_updates_the_base_where_it_is_checked_out_then_removes_the_worktree() {
        let here = "/repo/.claude/worktrees/en-999";

        assert_eq!(
            clean_up_plan(
                STORY_OF_BASE,
                here,
                &[checkout(BASE, "/repo"), checkout(STORY, here)]
            ),
            CleanUp {
                place: "/repo".to_owned(),
                remove: vec![
                    step("git", "/repo", &["worktree", "remove", here]),
                    step("git", "/repo", &["branch", "-D", STORY]),
                    step("gh", "/repo", &remote_deletion()),
                ],
                update: vec![
                    step("git", here, &["fetch", "--quiet", "--prune", "origin"]),
                    step(
                        "git",
                        "/repo",
                        &["merge", "--ff-only", "--quiet", "origin/feature/f99-topic"]
                    ),
                    step(
                        "cargo",
                        "/repo",
                        &["build", "--release", "--workspace", "--locked", "--quiet"]
                    ),
                ],
            }
        );
    }

    #[test]
    fn from_a_worktree_whose_base_no_checkout_holds_switches_the_worktree_to_the_base_first() {
        let here = "/repo/.claude/worktrees/en-999";
        let plan = clean_up_plan(
            STORY_OF_BASE,
            here,
            &[checkout("develop", "/repo"), checkout(STORY, here)],
        );
        let places = |steps: &[Step]| {
            steps
                .iter()
                .map(|step| (step.cwd.clone(), step.options[0].clone()))
                .collect::<Vec<_>>()
        };

        assert_eq!(plan.place, here);
        assert_eq!(
            places(&plan.update),
            [
                (here.to_owned(), "fetch".to_owned()),
                (here.to_owned(), "switch".to_owned()),
                (here.to_owned(), "merge".to_owned()),
            ]
        );
        assert_eq!(
            places(&plan.remove),
            [
                ("/repo".to_owned(), "worktree".to_owned()),
                ("/repo".to_owned(), "branch".to_owned()),
                ("/repo".to_owned(), "api".to_owned()),
            ]
        );
    }

    #[test]
    fn from_the_main_checkout_switches_to_the_base_and_removes_no_worktree() {
        let plan = clean_up_plan(
            STORY_OF_BASE,
            "/repo",
            &[
                checkout(STORY, "/repo"),
                checkout(BASE, "/repo/.claude/worktrees/other"),
            ],
        );

        assert_eq!(plan.place, "/repo");
        assert_eq!(
            plan.update
                .iter()
                .map(|step| step.options[0].as_str())
                .collect::<Vec<_>>(),
            ["fetch", "switch", "merge", "build"]
        );
        assert_eq!(
            plan.remove,
            [
                step("git", "/repo", &["branch", "-D", STORY]),
                step("gh", "/repo", &remote_deletion()),
            ]
        );
        // No checkout listed: the one it runs in is the main one.
        assert_eq!(clean_up_plan(STORY_OF_BASE, "/repo", &[]).remove.len(), 2);
    }

    #[test]
    fn leaves_to_the_base_only_the_status_files_of_the_backlog() {
        assert!(is_status_file("docs/backlog/stories/README.md"));
        assert!(is_status_file(
            "docs/backlog/features/E17-F21-agent-tools-in-rust.md"
        ));
        assert!(is_status_file("docs/backlog/features/README.md"));
        assert!(is_status_file("docs/backlog/epics/E17-fast-tooling.md"));
        // A story's own file, another page, a file in a sub-folder: never.
        assert!(!is_status_file("docs/backlog/stories/E17-F21-EN-103-x.md"));
        assert!(!is_status_file("docs/process/journal.md"));
        assert!(!is_status_file("docs/backlog/features/old/x.md"));
        assert!(!is_status_file("docs/backlog/features/x.txt"));
    }

    #[test]
    fn f21_ac2_leaves_conflicting_table_rows_to_the_base_and_keeps_the_rest() {
        let text = "# F\n\n| ID | Status |\n<<<<<<< HEAD\n| [A](a) | done |\n| [B](b) | ready |\n=======\n| [A](a) | ready |\n| [B](b) | done |\n>>>>>>> 1234 (docs(backlog): set B done)\n| [C](c) | draft |\n";

        assert_eq!(
            resolved_to_base(text).as_deref(),
            Some(
                "# F\n\n| ID | Status |\n| [A](a) | done |\n| [B](b) | ready |\n| [C](c) | draft |\n"
            )
        );
        // The base of a diff3 conflict is left out too.
        assert_eq!(
            resolved_to_base("<<<<<<< a\n| x |\n||||||| b\n| y |\n=======\n| z |\n>>>>>>> c")
                .as_deref(),
            Some("| x |")
        );
        // A line outside a table, or a conflict left open: given up.
        assert_eq!(
            resolved_to_base("<<<<<<< a\nprose\n=======\n| z |\n>>>>>>> c"),
            None
        );
        assert_eq!(resolved_to_base("<<<<<<< a\n| x |\n"), None);
        assert_eq!(
            resolved_to_base("no conflict").as_deref(),
            Some("no conflict")
        );
    }

    #[test]
    fn finds_the_done_commit_of_a_story() {
        assert!(closes_story(
            "docs(backlog): set EN-1 done\n\nCloses: EN-1\n",
            "EN-1"
        ));
        assert!(closes_story("x\n\nCloses: EN-2, EN-1", "EN-1"));
        assert!(!closes_story("x\n\nRefs: EN-1", "EN-1"));
        assert!(!closes_story("x\n\nCloses: EN-10", "EN-1"));
    }

    #[test]
    fn writes_a_failure_as_node_did() {
        let options = ["a".to_owned(), "b".to_owned()];

        assert_eq!(
            failure("git", &options, ""),
            "Error: Command failed: git a b"
        );
        assert_eq!(
            failure("git", &options, "fatal: x\n"),
            "Error: Command failed: git a b\nfatal: x\n"
        );
    }
}
