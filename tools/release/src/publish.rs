//! `release publish`, by the button of `release.yml` (ADR-0086), on `main` alone, the tool
//! refusing another branch before any call, since the workflow chooses nothing by an `if:`
//! (ADR-0088): the version from the last tag `vx.y.z` and the commits since (`REF-SEMVER`), its
//! notes the bodies of the pull requests into `main` merged since that tag but Dependabot's, each
//! checked as `release check` does and all against the commits, then the GitHub release made on
//! `main`'s head, which GitHub tags, the test report of the last pull request's CI attached. It
//! makes no commit. Every Git and GitHub call goes through a [`Run`], so that a test gives the
//! answers.

use std::fs;
use std::path::Path;

use crate::backlog::Backlog;
use crate::body::{Text, joined, release_text, text};
use crate::check::check;
use crate::draft::{bump, lines};
use crate::history::{Run, commits, last_release, merged_pull_requests};

/// What `release publish` did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Published {
    /// The release of this tag was made.
    Released(String),
    /// No commit since this tag: nothing to release.
    Nothing(String),
}

/// The name of the CI's artifact holding the test report (`ci.yml`, ADR-0027).
pub const REPORT_ARTIFACT: &str = "test-report";

/// The test report's file in that artifact.
pub const REPORT_FILE: &str = "test-report.xml";

/// The start of the head branch of Dependabot's pull requests: a security update merged into
/// `main` (`docs/tooling/ci.md`), whose body is Dependabot's, not a release text; its change is
/// written in the next review's text.
pub const DEPENDABOT_BRANCH: &str = "dependabot/";

/// The one branch a release is made on: its button may be pressed on any branch, the workflow
/// choosing nothing by an `if:` (ADR-0088), so the tool refuses the others.
pub const RELEASE_BRANCH: &str = "main";

/// Checks that the run is on [`RELEASE_BRANCH`], before any Git or GitHub call.
///
/// # Errors
///
/// The branch, `GITHUB_REF_NAME` of the run, is another one, or empty.
pub fn on_release_branch(branch: &str) -> Result<(), String> {
    if branch == RELEASE_BRANCH {
        Ok(())
    } else {
        Err(format!(
            "the run is on the branch {branch:?}, not {RELEASE_BRANCH:?}: a release is made from \
             {RELEASE_BRANCH} only, so press the button on {RELEASE_BRANCH} (GITHUB_REF_NAME)"
        ))
    }
}

/// The first line of a command's output, trimmed.
fn first_line(output: &str) -> String {
    output.lines().next().unwrap_or_default().trim().to_owned()
}

/// The release text of each pull request, the latest first, each checked.
fn texts(run: &mut Run<'_>, numbers: &[u32]) -> Result<Vec<(u32, Text)>, String> {
    let mut read = Vec::new();

    for &number in numbers {
        let body = run(
            "gh",
            &[
                "pr",
                "view",
                &number.to_string(),
                "--json",
                "body",
                "--jq",
                ".body",
            ],
        )?;
        let released = release_text(&body);

        if let Some(finding) = check(&released).first() {
            return Err(format!(
                "pull request #{number}:{}: {} (release check)",
                finding.line, finding.message
            ));
        }
        read.push((number, text(&released)));
    }
    Ok(read)
}

/// Checks that the texts agree with the commits (`docs/tooling/releases.md`, The version): a
/// change listed only when a commit since the tag changes the library or the playground, and one
/// listed at least when one does.
fn agreeing(texts: &[(u32, Text)], user_visible: bool, tag: &str) -> Result<(), String> {
    let listing = texts.iter().find(|(_, text)| !text.changes.is_empty());

    match (listing, user_visible) {
        (Some((number, _)), false) => Err(format!(
            "pull request #{number} lists changes, but no commit since {tag} changes the library \
             nor the playground: write the notice of no change in its body \
             (docs/tooling/releases.md)"
        )),
        (None, true) => Err(format!(
            "commits since {tag} change the library or the playground, but no pull request's \
             body lists a change: write the features' texts in the review's body \
             (docs/tooling/releases.md)"
        )),
        _ => Ok(()),
    }
}

/// The run of `ci.yml` that passed on the head of the pull request `main`'s head merged, for its
/// test report; `main`'s head and that run.
fn ci_run(run: &mut Run<'_>) -> Result<(String, String), String> {
    let head = first_line(&run("git", &["rev-parse", "HEAD"])?);
    let pull_request_head = first_line(
        &run("git", &["rev-parse", "HEAD^2"])
            .map_err(|_| format!("{head} is not the merge commit of a pull request"))?,
    );
    let ci_run = first_line(&run(
        "gh",
        &[
            "run",
            "list",
            "--workflow",
            "ci.yml",
            "--commit",
            &pull_request_head,
            "--status",
            "success",
            "--limit",
            "1",
            "--json",
            "databaseId",
            "--jq",
            ".[0].databaseId",
        ],
    )?);

    if ci_run.is_empty() || ci_run == "null" {
        return Err(format!(
            "no successful CI run of {pull_request_head}, the pull request's head: no test report"
        ));
    }
    Ok((head, ci_run))
}

/// Publishes the release of the head of `main`, the run's `branch`, the notes and the report
/// written in `folder`.
///
/// # Errors
///
/// The branch is not `main`, no tag `vx.y.z` is behind the head, a body breaks a rule or two hold
/// a notice, the bodies list changes the commits do not make or none the commits make, the head
/// is not a merge, no successful CI run of the pull request's head is found, or a Git or GitHub
/// command fails.
pub fn publish(run: &mut Run<'_>, branch: &str, folder: &Path) -> Result<Published, String> {
    on_release_branch(branch)?;
    let (previous, current) = last_release(run, "HEAD")?;
    let range = format!("{previous}..HEAD");
    let released = commits(run, &range)?;

    if released.is_empty() {
        return Ok(Published::Nothing(previous));
    }

    let drafted = lines(&released, &Backlog::default());
    let version = current.bumped(bump(&drafted)).to_string();
    let tag = format!("v{version}");
    let merged: Vec<u32> = merged_pull_requests(run, &range)?
        .into_iter()
        .filter(|(_, branch)| !branch.starts_with(DEPENDABOT_BRANCH))
        .map(|(number, _)| number)
        .collect();
    let read = texts(run, &merged)?;

    agreeing(&read, !drafted.is_empty(), &previous)?;
    let notes = joined(&read)?;
    let (head, ci_run) = ci_run(run)?;
    let notes_file = folder.join("notes.md");
    let folder_text = folder.to_string_lossy().into_owned();

    fs::create_dir_all(folder).map_err(|error| format!("{folder_text}: {error}"))?;
    fs::write(&notes_file, notes).map_err(|error| format!("notes.md: {error}"))?;
    run(
        "gh",
        &[
            "run",
            "download",
            &ci_run,
            "--name",
            REPORT_ARTIFACT,
            "--dir",
            &folder_text,
        ],
    )?;
    run(
        "gh",
        &[
            "release",
            "create",
            &tag,
            "--target",
            &head,
            "--title",
            &version,
            "--notes-file",
            &notes_file.to_string_lossy(),
            &folder.join(REPORT_FILE).to_string_lossy(),
        ],
    )?;
    Ok(Published::Released(tag))
}
