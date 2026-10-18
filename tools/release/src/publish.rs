//! `release publish`, on the push of a release's merge into `main` (ADR-0079): the version of
//! `package.json`, its entry checked, then the GitHub release `vx.y.z` made on the merge commit
//! with the entry as its notes and the test report of the pull request's CI attached. It makes no
//! commit: GitHub makes the tag with the release. Every Git and GitHub call goes through a [`Run`],
//! so that a test gives the answers.

use std::fs;
use std::path::Path;

use crate::changelog::notes;
use crate::check::check;
use crate::history::Run;
use crate::manifest::{repository, version};

/// What `release publish` did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Published {
    /// The release of this tag was made.
    Released(String),
    /// The tag's release already existed: nothing done.
    Already(String),
}

/// The name of the CI's artifact holding the test report (`ci.yml`, ADR-0027).
pub const REPORT_ARTIFACT: &str = "test-report";

/// The test report's file in that artifact.
pub const REPORT_FILE: &str = "test-report.xml";

/// The first line of a command's output, trimmed.
fn first_line(output: &str) -> String {
    output.lines().next().unwrap_or_default().trim().to_owned()
}

/// Publishes the release of the head of `main`, its files read under `root`, the notes and the
/// report written in `folder`.
///
/// # Errors
///
/// A file cannot be read, the entry breaks a rule, the head is not a merge, no successful CI run of
/// the pull request's head is found, or a Git or GitHub command fails.
pub fn publish(root: &Path, run: &mut Run<'_>, folder: &Path) -> Result<Published, String> {
    let read = |file: &str| {
        fs::read_to_string(root.join(file)).map_err(|error| format!("{file}: {error}"))
    };
    let package = read("package.json")?;
    let changelog = read("CHANGELOG.md")?;
    let written = version(&package).ok_or("package.json holds no version".to_owned())?;
    let address = repository(&package).ok_or("package.json names no repository url".to_owned())?;
    let tag = format!("v{written}");

    if let Some(finding) = check(&changelog, &address, written).first() {
        return Err(format!(
            "CHANGELOG.md:{}: {} (release check)",
            finding.line, finding.message
        ));
    }
    if run("gh", &["release", "view", &tag, "--json", "tagName"]).is_ok() {
        return Ok(Published::Already(tag));
    }

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

    let notes_text = notes(&changelog, written).unwrap_or_default();
    let notes_file = folder.join("notes.md");
    let folder_text = folder.to_string_lossy().into_owned();

    fs::create_dir_all(folder).map_err(|error| format!("{folder_text}: {error}"))?;
    fs::write(&notes_file, notes_text).map_err(|error| format!("notes.md: {error}"))?;
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
            written,
            "--notes-file",
            &notes_file.to_string_lossy(),
            &folder.join(REPORT_FILE).to_string_lossy(),
        ],
    )?;
    Ok(Published::Released(tag))
}
