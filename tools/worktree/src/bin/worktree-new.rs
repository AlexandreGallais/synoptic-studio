//! `worktree-new <ID> [--type <type>]` (`npm run worktree:new -- US-061`): fetches GitHub, adds
//! `.claude/worktrees/us-061` on `feat/us-061-<topic>` from the story's feature branch, and links
//! the main checkout's `node_modules` into it, then installs Husky's hooks folder in every
//! worktree that has none, so that a push runs the worktree's own `.husky/pre-push` (EN-052), and
//! builds the release binaries of the Rust tools in the new worktree (EN-068).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use worktree::{Plan, Request, plan, worktrees_without_hooks};

/// Output of a Git command, or its error output as the reason.
fn git(arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(arguments)
        .output()
        .map_err(|error| format!("git: {error}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        Err(format!(
            "git {}: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// The story id and the branch type given by `--type`.
fn arguments(argv: &[String]) -> Result<(&str, Option<&str>), String> {
    match argv {
        [id] => Ok((id, None)),
        [id, flag, kind] if flag == "--type" => Ok((id, Some(kind))),
        _ => Err("usage: npm run worktree:new -- <ID> [--type <type>]".to_owned()),
    }
}

/// Names of the story files of the checkout the tool runs in.
fn story_files(top: &Path) -> Result<Vec<String>, String> {
    let folder = top.join("docs/backlog/stories");
    let entries =
        fs::read_dir(&folder).map_err(|error| format!("{}: {error}", folder.display()))?;

    Ok(entries
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect())
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

/// Runs Husky in each worktree without its hooks folder, as `npm run prepare` does in a clone.
fn install_hooks(main: &Path) -> Result<(), String> {
    let husky = main.join("node_modules/.bin/husky");

    if !husky.exists() {
        return Ok(());
    }

    let porcelain = git(&["worktree", "list", "--porcelain"])?;

    for dir in worktrees_without_hooks(&porcelain, Path::exists) {
        let output = Command::new(&husky)
            .current_dir(&dir)
            .output()
            .map_err(|error| format!("husky: {error}"))?;
        // Husky exits with 0 and prints its reason when it fails.
        let reason = String::from_utf8_lossy(&output.stdout).trim().to_owned();

        if !output.status.success() || !reason.is_empty() {
            return Err(format!("husky in {}: {reason}", dir.display()));
        }
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
        .args(["build", "--release", "--workspace", "--locked", "--quiet"])
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
    let request = Request {
        id,
        branch_type,
        story_files: &files,
        remote_branches: &remote_branches,
        main: &main,
    };
    let plan = plan(&request, |path| path.symlink_metadata().is_ok())?;

    create(&plan, &main)?;
    install_hooks(&main)?;
    build_tools(&plan.dir)?;
    Ok(plan)
}

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().skip(1).collect();

    match run(&argv) {
        Ok(plan) => {
            println!(
                "{} on {} from {}",
                plan.dir.display(),
                plan.branch,
                plan.base
            );
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("worktree:new: {reason}");
            ExitCode::from(1)
        }
    }
}
