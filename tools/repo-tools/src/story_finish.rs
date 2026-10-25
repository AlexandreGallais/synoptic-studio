//! The end of a story in a run (RET-002, ADR-0072, ADR-0065, EN-051, EN-103, EN-154): what
//! `story-finish` decides — where it starts from the story's pull request, when it stops waiting,
//! the clean-up of a merged story and the same steps to type when it stopped midway — and the Git
//! sequences it runs, each given its folder: the rebase on the base, the push with lease of a head
//! that differs from its remote's. A story has no `done` commit since its status lives in the
//! records (EN-154): once GitHub merged its pull request, the branch's rows of the records are
//! marked merged and the story `done`, and nothing is committed on the base. Before a pull request
//! into `develop`, every id the change cites is checked against the records (`knowledge
//! check-ids`), which the CI cannot read. Who merges, GitHub or the Product Owner, as the
//! auto-merge of its base, `story-automerge.yml` or `feature-automerge.yml`, takes the branch
//! (EN-118, EN-134).

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use git_repository::story_of_branch as story_of;
use knowledge::backlog::{merge_branch, missing_ids, set_status};
use knowledge::paths::is_test_path;
use knowledge::project::open_records;
use knowledge::store::write_or_refuse;
use stale_binaries::RELEASE_BUILD;

/// Readings of the pull request's state before giving up: three minutes, one each 10 s.
pub const POLLS: u32 = 18;

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
    /// The auto-merge of its base, once the pull request is labelled `autonomous`: awaited.
    GitHub,
    /// The Product Owner: opened without the label, never awaited (EN-118).
    ProductOwner,
}

/// Who merges a story branch into its base, as the auto-merge of that base lets it through: into
/// a feature branch a `US`, `EN`, `SP`, `CHK`, `AUD` or `VAL`, into `develop` a `REV` or an epic's
/// refinement `SP` (REV-009); anything else is the Product Owner's.
#[must_use]
pub fn merger(base: &str, branch: &str) -> Merger {
    let kind = story_of(branch)
        .and_then(|id| id.split_once('-').map(|(kind, _)| kind.to_owned()))
        .unwrap_or_default();
    let taken: &[&str] = if base == "develop" {
        // An epic's final audit merges into `develop` as a review does (E49, 2026-10-25).
        &["REV", "SP", "AUD"]
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
    if base == "develop" {
        "feature-automerge.yml"
    } else {
        "story-automerge.yml"
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

/// Whether a changed file is one whose citations `check-ids` reads before a pull request into
/// `develop`: a page of `docs/`, a file of `src/` or `playground/`, a Markdown page at the root;
/// never a test, whose fixtures cite ids that exist nowhere on purpose.
#[must_use]
pub fn is_checked_path(path: &str) -> bool {
    let is_test = is_test_path(path);
    let is_markdown = Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));
    let in_folder = ["docs/", "src/", "playground/"]
        .iter()
        .any(|folder| path.starts_with(folder));

    !is_test && (in_folder || (is_markdown && !path.contains('/')))
}

/// Why the pull request into `develop` is not opened: the ids the change cites that the records
/// lack, one line each, `<path>:<line>: <id>`; `None` when there are none.
#[must_use]
pub fn ids_refusal(missing: &[String]) -> Option<String> {
    (!missing.is_empty()).then(|| {
        format!(
            "ids cited that the records lack, which no check of the pull request can read: write their rows, or cite another id (`target/release/knowledge check-ids <paths>`):\n{}",
            missing.join("\n")
        )
    })
}

/// The ids cited by the files a branch changed from its base that the records lack, each
/// `<path>:<line>: <id>`, read in the checkout.
///
/// # Errors
/// When Git fails or the records cannot be read.
pub fn missing_cited_ids(base: &str, cwd: &Path) -> Result<Vec<String>, String> {
    let changed = run_words(
        "git",
        &[
            "diff",
            "--name-only",
            "--diff-filter=d",
            &format!("origin/{base}...HEAD"),
        ],
        cwd,
    )?;
    let database = open_records(cwd)?;
    let mut missing = Vec::new();

    for path in changed.lines().filter(|path| is_checked_path(path)) {
        let Ok(text) = fs::read_to_string(cwd.join(path)) else {
            continue;
        };

        for (line, id) in missing_ids(&database, &text).map_err(|error| error.to_string())? {
            missing.push(format!("{path}:{line}: {id}"));
        }
    }
    Ok(missing)
}

/// Once GitHub merged a story's branch: its rows of the records marked merged and its story set
/// `done`, in one write; what to print.
///
/// # Errors
/// When the records cannot be read or written.
pub fn record_the_merge(branch: &str, cwd: &Path) -> Result<String, String> {
    let database = open_records(cwd)?;
    let story = story_of(branch);
    let (marked, done) = write_or_refuse(&database, |transaction| {
        let marked = merge_branch(transaction, branch).map_err(|error| error.to_string())?;
        let done = match &story {
            Some(id) => set_status(transaction, id, "done")?,
            None => false,
        };

        Ok((marked, done))
    })?;
    let story = match (&story, done) {
        (Some(id), true) => format!(", {id} done"),
        _ => String::new(),
    };

    Ok(format!(
        "the records: {marked} row(s) of {branch} marked merged{story}"
    ))
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

/// A Git command's output, its standard error kept back; `None` when it fails.
fn run_quiet(words: &[&str], cwd: &Path) -> Option<String> {
    git_repository::stdout_of(
        Command::new("git")
            .args(words)
            .current_dir(cwd)
            .stdin(Stdio::null()),
    )
    .ok()
}

/// How the story's branch met its base.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rebase {
    /// The branch already held the base.
    UpToDate,
    /// The branch was rebased.
    Rebased,
}

/// Brings the story's branch onto its base as the remote holds it now, after another story
/// merged; a conflict aborts the rebase, the branch left as it was. No status row conflicts any
/// more: the statuses live in the records (EN-154).
///
/// # Errors
/// On a conflict, or when the fetch fails.
pub fn rebase_on_base(base: &str, cwd: &Path) -> Result<Rebase, String> {
    let remote = format!("origin/{base}");

    run_words("git", &["fetch", "--quiet", "origin", base], cwd)?;
    if git_succeeds(&["merge-base", "--is-ancestor", &remote, "HEAD"], cwd) {
        return Ok(Rebase::UpToDate);
    }
    if git_succeeds(&["rebase", "--quiet", &remote], cwd) {
        return Ok(Rebase::Rebased);
    }

    let _ = git_succeeds(&["rebase", "--abort"], cwd);
    Err(format!(
        "conflict rebasing on {remote}: rebase by hand, then run again"
    ))
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
        // [F23.AC2] A `REV` or an epic's `SP` into `develop` and a story into its feature are
        // merged by GitHub (REV-009); any other story into `develop` by the Product Owner.
        assert_eq!(merger("develop", "docs/rev-006-review"), Merger::GitHub);
        assert_eq!(
            merger("develop", "chore/aud-051-the-final-clean-up"),
            Merger::GitHub
        );
        assert_eq!(
            merger("develop", "docs/sp-030-refinement-of-e18"),
            Merger::GitHub
        );
        assert_eq!(
            merger("develop", "chore/en-114-a-tool"),
            Merger::ProductOwner
        );
        assert_eq!(merger("develop", "feat/us-061-x"), Merger::ProductOwner);
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
                // [F23.AC1] nothing committed nor pushed after the merge: the status is in the
                // records (EN-154).
                format!("cd \"/repo\" && git worktree remove {here}"),
                format!("cd \"/repo\" && git branch -D {STORY}"),
                format!("cd \"/repo\" && gh {}", remote_deletion().join(" ")),
            ]
        );
    }

    #[test]
    fn checks_the_ids_of_the_docs_and_the_code_never_of_a_test() {
        for path in [
            "docs/tooling/knowledge.md",
            "src/geometry/a.ts",
            "playground/page/b.ts",
            "README.md",
        ] {
            assert!(is_checked_path(path), "{path}");
        }
        for path in [
            "tests/backlog.test.ts",
            "playground/page/b.test.ts",
            "tools/knowledge/tests/backlog.rs",
            "tools/lint/tests/fixtures/x.md",
            "tools/knowledge/src/backlog.rs",
            ".github/workflows/ci.yml",
            "docs",
        ] {
            assert!(!is_checked_path(path), "{path}");
        }
        assert_eq!(ids_refusal(&[]), None);
        assert!(
            ids_refusal(&["docs/a.md:3: EN-999".to_owned()])
                .is_some_and(|refusal| refusal.ends_with(":\ndocs/a.md:3: EN-999"))
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
