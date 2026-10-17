//! `story-finish <base> <title> <body file>` (`npm run story:finish`, RET-002, ADR-0045, EN-051,
//! EN-103), in the story's checkout once its branch is pushed, under the merge lock of
//! `tools/heavy-lock/`: stories merge one at a time, the next one waiting, then rebasing on the
//! updated base. Its pull request opened with the `autonomous` label, its merge by GitHub awaited
//! for at most three minutes, then the base brought up to date where it is checked out, the rows the
//! story file holds for the logs moved into them by `move-rows`, committed and pushed on the base,
//! and the story's worktree and branch removed here and on GitHub. A rebase that met the status rows
//! of another story's done commit writes the story's own again after it; a head that differs from
//! its branch on GitHub is pushed with lease before the pull request. Run again after a stop, it
//! reuses the story's pull request, and goes straight to the clean-up once merged (AUD-020).

use std::env;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::thread;
use std::time::Duration;

use repo_tools::story_finish::{
    Next, Rebase, Start, Story, clean_up, clean_up_plan, manual_steps, move_rows, needs_push,
    next_step, push_with_lease, rebase_on_base, run_words, start_step, worktrees,
    write_status_again,
};

/// Time between two readings of the pull request's state.
const POLL: Duration = Duration::from_secs(10);

/// The pull request of a branch, read from GitHub: its state and address, both empty when it has
/// none.
fn pull_request_of(branch: &str, cwd: &Path) -> (String, String) {
    let output = Command::new("gh")
        .args([
            "pr",
            "view",
            branch,
            "--json",
            "state,url",
            "-q",
            r#".state + " " + .url"#,
        ])
        .current_dir(cwd)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            let mut words = text.split(' ');

            (
                words.next().unwrap_or_default().to_owned(),
                words.next().unwrap_or_default().to_owned(),
            )
        }
        _ => (String::new(), String::new()),
    }
}

/// The pull request's state, read until it merges, closes, or time runs out.
fn await_merge(url: &str, cwd: &Path) -> Result<String, String> {
    let mut readings = 0;

    loop {
        let state = run_words(
            "gh",
            &["pr", "view", url, "--json", "state", "-q", ".state"],
            cwd,
        )?;

        readings += 1;
        if next_step(&state, readings) != Next::Wait {
            return Ok(state);
        }
        thread::sleep(POLL);
    }
}

/// The rebase on the base, the status rows written again after it, and the push with lease of a
/// head that differs from GitHub's.
fn bring_up(base: &str, branch: &str, here: &Path) -> Result<(), String> {
    let rebased = rebase_on_base(base, here)?;

    if let Rebase::Rebased(resolved) = &rebased {
        if !resolved.is_empty() {
            println!(
                "{branch}: the status rows of {} left to origin/{base}",
                resolved.join(", ")
            );
        }

        let written = write_status_again(branch, here)?;

        if !written.is_empty() {
            println!(
                "{branch}: its status written again after the rebase, in its done commit: {}",
                written.join(", ")
            );
        }
    }
    if needs_push(here, branch) {
        push_with_lease(branch, here)?;
        if rebased == Rebase::UpToDate {
            println!("{branch} pushed with lease: its head differed from origin/{branch}");
        } else {
            println!("{branch} rebased on origin/{base} and pushed");
        }
    }
    Ok(())
}

fn finish(base: &str, title: &str, body: &str) -> Result<u8, String> {
    let current = env::current_dir().map_err(|error| error.to_string())?;
    let top = run_words("git", &["rev-parse", "--show-toplevel"], &current)?;
    let here = Path::new(&top);
    let branch = run_words("git", &["branch", "--show-current"], here)?;
    let (state, existing) = pull_request_of(&branch, here);
    let start = start_step(&state);

    if start != Start::Clean {
        bring_up(base, &branch, here)?;
    }

    let url = if start == Start::Create {
        run_words(
            "gh",
            &[
                "pr",
                "create",
                "--base",
                base,
                "--label",
                "autonomous",
                "--title",
                title,
                "--body-file",
                body,
            ],
            here,
        )?
    } else {
        existing
    };

    println!("{url}");

    let state = if start == Start::Clean {
        "MERGED".to_owned()
    } else {
        await_merge(&url, here)?
    };
    let repository = run_words(
        "gh",
        &[
            "repo",
            "view",
            "--json",
            "nameWithOwner",
            "-q",
            ".nameWithOwner",
        ],
        here,
    )?;
    let checkouts = worktrees(&run_words(
        "git",
        &["worktree", "list", "--porcelain"],
        here,
    )?);
    let plan = clean_up_plan(
        Story {
            base,
            branch: &branch,
            repository: &repository,
        },
        &top,
        &checkouts,
    );

    match clean_up(&state, &plan, |cwd| move_rows(&branch, cwd)) {
        Ok(true) => {
            println!("merged; {base} up to date, {branch} deleted here and on GitHub");
            Ok(0)
        }
        Ok(false) => {
            eprintln!(
                "{state} after 3 min, {branch} kept: read gh run list --workflow story-automerge.yml --limit 1, then run again: it reuses the pull request"
            );
            Ok(1)
        }
        Err(error) => {
            eprintln!(
                "the clean-up stopped: {error}\nRun again from the worktree while it exists, or finish by hand, skipping the steps already done:\n{}",
                manual_steps(&plan, &branch)
            );
            Ok(1)
        }
    }
}

fn work() -> u8 {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let [base, title, body] = [0, 1, 2].map(|at| arguments.get(at).map_or("", String::as_str));

    // Without its three arguments it stops: without a base, it rebased on `origin/undefined`
    // (AUD-020, N2).
    if [base, title, body].contains(&"") {
        eprintln!("usage: npm run story:finish -- <base> <title> <body file>");
        return 1;
    }

    finish(base, title, body).unwrap_or_else(|error| {
        eprintln!("story-finish: {error}");
        1
    })
}

fn main() -> ExitCode {
    tool_use::run("story-finish", "", work)
}
