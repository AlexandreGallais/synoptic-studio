//! `worktree-new <ID> [--type <type>]` (`target/release/worktree-new US-061`): fetches GitHub, adds
//! `.claude/worktrees/us-061` on `feat/us-061-<topic>` from the story's feature branch — the one of
//! `origin` holding the story's file when the checkout lacks it (`git ls-tree`, EN-118) — and links
//! the main checkout's `node_modules` into it, then sets the hooks folder `.githooks` if it is not
//! yet, relative so that a push runs the worktree's own `.githooks/pre-push` (EN-052, EN-108), and
//! builds the release binaries of the Rust tools in the new worktree (EN-068), from a copy of the
//! main checkout's `target/release` (EN-113).

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use git_hooks::install::install;
use git_repository::stdout_of;
use stale_binaries::RELEASE_BUILD;
use worktree::seed::seed_release;
use worktree::{Plan, Request, plan};

/// Output of a Git command, trimmed, or its error output as the reason.
fn git(arguments: &[&str]) -> Result<String, String> {
    stdout_of(Command::new("git").args(arguments))
        .map(|output| output.trim().to_owned())
        .map_err(|failure| failure.to_string())
}

/// The story id and the branch type given by `--type`.
fn arguments(argv: &[String]) -> Result<(&str, Option<&str>), String> {
    match argv {
        [id] => Ok((id, None)),
        [id, flag, kind] if flag == "--type" => Ok((id, Some(kind))),
        _ => Err("usage: target/release/worktree-new <ID> [--type <type>]".to_owned()),
    }
}

/// Names of the story files of the checkout the tool runs in.
fn story_files(top: &Path) -> Result<Vec<String>, String> {
    worktree::story_files(top)
        .map_err(|error| format!("{}: {error}", top.join(worktree::STORIES).display()))
}

/// The story files of each feature branch of `origin`, read by `git ls-tree`: the main checkout,
/// on `develop`, lacks the stories written on a feature branch (EN-118).
fn branch_story_files(remote_branches: &[String]) -> Result<Vec<(String, Vec<String>)>, String> {
    let folder = format!("{}/", worktree::STORIES);

    remote_branches
        .iter()
        .map(|branch| {
            let names = git(&["ls-tree", "--name-only", branch, &folder])?
                .lines()
                .filter_map(|path| path.strip_prefix(&folder))
                .map(str::to_owned)
                .collect();

            Ok((branch.clone(), names))
        })
        .collect()
}

/// Links a folder at `link` to the folder `target`: a symbolic link (`std::os::unix::fs::symlink`).
// Unix-only: the standard library has no portable folder link; Windows gets its own call.
#[cfg(unix)]
fn link_folder(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

/// Links a folder at `link` to the folder `target`: a folder symbolic link on Windows
/// (`std::os::windows::fs::symlink_dir`), which needs the Developer Mode or the right to make one.
#[cfg(windows)]
fn link_folder(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

/// Creates the worktree of a plan and links `node_modules` into it.
fn create(plan: &Plan, main: &Path) -> Result<(), String> {
    let dir = plan.dir.to_string_lossy();

    git(&[
        "worktree",
        "add",
        "--quiet",
        &dir,
        "-b",
        &plan.branch,
        &plan.base,
    ])?;

    let modules = main.join("node_modules");

    if modules.is_dir() {
        link_folder(&modules, &plan.dir.join("node_modules"))
            .map_err(|error| format!("node_modules: {error}"))?;
    }

    Ok(())
}

/// Builds the release binaries of the Rust tools in a new worktree holding the Cargo workspace, as
/// `npm run prepare` does in a clone: its scripts and hooks call `target/release/<tool>` (EN-068).
fn build_tools(dir: &Path) -> Result<(), String> {
    if !dir.join("Cargo.toml").exists() {
        return Ok(());
    }

    let output = Command::new("cargo")
        .args(RELEASE_BUILD)
        .current_dir(dir)
        .output()
        .map_err(|error| format!("cargo: {error}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "cargo build in {}: {}",
            dir.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn run(argv: &[String]) -> Result<Plan, String> {
    let (id, branch_type) = arguments(argv)?;
    let common = PathBuf::from(git(&[
        "rev-parse",
        "--path-format=absolute",
        "--git-common-dir",
    ])?);
    let main = common
        .parent()
        .ok_or("the Git common directory has no parent")?
        .to_path_buf();
    let top = PathBuf::from(git(&["rev-parse", "--show-toplevel"])?);

    git(&["fetch", "--quiet", "origin"])?;

    let remote_branches: Vec<String> = git(&[
        "for-each-ref",
        "--format=%(refname:short)",
        "refs/remotes/origin/feature/",
    ])?
    .lines()
    .map(str::to_owned)
    .collect();
    let files = story_files(&top)?;
    let branch_files = branch_story_files(&remote_branches)?;
    let request = Request {
        id,
        branch_type,
        story_files: &files,
        remote_branches: &remote_branches,
        branch_files: &branch_files,
        main: &main,
    };
    let plan = plan(&request, |path| path.symlink_metadata().is_ok())?;

    create(&plan, &main)?;
    // The setting is the repository's, shared by every worktree: set once, as `prepare` does.
    install(&plan.dir).map_err(|reason| format!("git-hooks install: {reason}"))?;
    // A copy that fails only costs the build its head start: Cargo builds what is missing.
    if let Err(error) = seed_release(&main, &plan.dir) {
        eprintln!("worktree-new: target/release not copied from the main checkout: {error}");
    }
    build_tools(&plan.dir)?;
    Ok(plan)
}

fn main() -> ExitCode {
    tool_use::run("worktree-new", "", add_worktree)
}

/// Adds the story's worktree; the exit code.
fn add_worktree() -> u8 {
    let argv: Vec<String> = env::args().skip(1).collect();

    match run(&argv) {
        Ok(plan) => {
            println!(
                "{} on {} from {}",
                plan.dir.display(),
                plan.branch,
                plan.base
            );
            0
        }
        Err(reason) => {
            eprintln!("worktree-new: {reason}");
            1
        }
    }
}
