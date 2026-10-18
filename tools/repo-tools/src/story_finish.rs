//! The end of a story in a run (RET-002, ADR-0045, ADR-0065, EN-051, EN-103): what `story-finish`
//! decides — where it starts from the story's pull request, when it stops waiting, the clean-up of a
//! merged story and the same steps to type when it stopped midway — and the Git sequences it runs,
//! each given its folder: the rebase on the base, whose conflicts in the backlog's status rows only
//! are left to the base and the rows written again after it, the push with lease of a head that
//! differs from its remote's, the rows moved into the logs on the base.

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use shared_files::shared::story_of;

use crate::backlog_status::{front_field, item_file, write_status};

/// Readings of the pull request's state before giving up: three minutes, one each 10 s.
pub const POLLS: u32 = 18;

/// The Rust tool moving a story's rows into the logs, a binary of the Cargo workspace (EN-068).
pub const MOVE_ROWS: [&str; 8] = [
    "run",
    "--release",
    "-q",
    "-p",
    "shared-files",
    "--bin",
    "move-rows",
    "--",
];

/// The repository's formatter writing files: oxfmt, then the tables without padding (EN-073).
pub const FORMAT_WRITE: [&str; 7] = ["run", "--release", "-q", "-p", "format", "--", "--write"];

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
    /// The checkout of the base, where its rows are moved.
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
        update.push(Step::new(
            "cargo",
            place,
            &["build", "--release", "--workspace", "--locked", "--quiet"],
        ));
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

/// The message of the commit moving a merged story's rows into the logs, its `Refs:` footer
/// naming the story.
#[must_use]
pub fn rows_message(branch: &str) -> String {
    let id = story_of(branch).unwrap_or_default();

    format!("docs(docs): move the rows of {id} into the logs\n\nRefs: {id}\n")
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
/// date, the rows moved, committed and pushed, then the worktree and the branch removed (AUD-020).
#[must_use]
pub fn manual_steps(plan: &CleanUp, branch: &str) -> String {
    let paragraphs: Vec<String> = rows_message(branch)
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| format!("-m \"{line}\""))
        .collect();
    let rows = [
        format!(
            "cd \"{}\" && cargo {} {branch}",
            plan.place,
            MOVE_ROWS.join(" ")
        ),
        format!(
            "cd \"{}\" && git commit {} -- <the files it printed>",
            plan.place,
            paragraphs.join(" ")
        ),
        format!("cd \"{}\" && git push", plan.place),
    ];

    plan.update
        .iter()
        .map(typed)
        .chain(rows)
        .chain(plan.remove.iter().map(typed))
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
    let output = Command::new("git")
        .args(words)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .ok()?;

    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
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

    let story = fs::read_to_string(cwd.join("docs/backlog/stories").join(&file))
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

/// Commits the files a merged story's rows were moved into, and only them: whatever else the
/// checkout of the base holds staged stays staged, out of the commit (AUD-020).
///
/// # Errors
/// When the commit fails.
pub fn commit_rows(branch: &str, files: &[String], cwd: &Path) -> Result<(), String> {
    let message = rows_message(branch);
    let mut words = vec!["commit", "--quiet", "-m", message.as_str(), "--"];

    words.extend(files.iter().map(String::as_str));
    run_words("git", &words, cwd).map(|_| ())
}

/// Pushes the base when it holds commits its remote has not: those of this run, or those a run
/// stopped before pushing (AUD-020); whether it pushed.
///
/// # Errors
/// When Git fails.
pub fn push_ahead(cwd: &Path) -> Result<bool, String> {
    if run_words("git", &["rev-list", "--count", "@{upstream}..HEAD"], cwd)? == "0" {
        return Ok(false);
    }

    run_shown("git", &["push", "--quiet"], cwd)?;
    Ok(true)
}

/// Moves a merged story's rows into the logs, then commits and pushes them on the base.
///
/// # Errors
/// When a step fails.
pub fn move_rows(branch: &str, cwd: &Path) -> Result<(), String> {
    let mut words: Vec<&str> = MOVE_ROWS.to_vec();

    words.push(branch);

    let moved = run_words("cargo", &words, cwd)?;
    let files: Vec<String> = moved
        .split('\n')
        .filter(|line| !moved.is_empty() && !line.is_empty())
        .map(str::to_owned)
        .collect();

    if !files.is_empty() {
        let mut format: Vec<&str> = FORMAT_WRITE.to_vec();

        format.extend(files.iter().map(String::as_str));
        run_words("cargo", &format, cwd)?;
        commit_rows(branch, &files, cwd)?;
    }
    if push_ahead(cwd)? {
        println!(
            "base pushed, rows moved: {}",
            if files.is_empty() {
                "none new".to_owned()
            } else {
                files.join(", ")
            }
        );
    }
    Ok(())
}

/// Cleans up a story once its pull request merged: the base brought up to date, the rows moved
/// there, the worktree and the branch removed here and on GitHub. A pull request not merged keeps
/// everything: false.
///
/// # Errors
/// The first step that failed.
pub fn clean_up(
    state: &str,
    plan: &CleanUp,
    move_rows_in: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<bool, String> {
    if state != "MERGED" {
        return Ok(false);
    }

    for step in &plan.update {
        run(step.command, &step.options, Path::new(&step.cwd))?;
    }
    move_rows_in(Path::new(&plan.place))?;
    for step in &plan.remove {
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
            manual_steps(&plan, STORY).split('\n').collect::<Vec<_>>(),
            [
                format!("cd \"{here}\" && git fetch --quiet --prune origin"),
                format!("cd \"/repo\" && git merge --ff-only --quiet origin/{BASE}"),
                "cd \"/repo\" && cargo build --release --workspace --locked --quiet".to_owned(),
                format!(
                    "cd \"/repo\" && cargo run --release -q -p shared-files --bin move-rows -- {STORY}"
                ),
                "cd \"/repo\" && git commit -m \"docs(docs): move the rows of EN-999 into the logs\" -m \"Refs: EN-999\" -- <the files it printed>".to_owned(),
                "cd \"/repo\" && git push".to_owned(),
                format!("cd \"/repo\" && git worktree remove {here}"),
                format!("cd \"/repo\" && git branch -D {STORY}"),
                format!("cd \"/repo\" && gh {}", remote_deletion().join(" ")),
            ]
        );
    }

    #[test]
    fn names_the_merged_story_in_the_commit_moving_its_rows_into_the_logs() {
        assert_eq!(
            rows_message("chore/en-050-shared-files-written-at-merge"),
            "docs(docs): move the rows of EN-050 into the logs\n\nRefs: EN-050\n"
        );
        assert_eq!(
            rows_message("feat/us-061-font-list"),
            "docs(docs): move the rows of US-061 into the logs\n\nRefs: US-061\n"
        );
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
