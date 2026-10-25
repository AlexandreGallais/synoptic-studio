//! `check-all` (`npm run check:all`, RET-002, EN-103): every check of the repository in parallel
//! lanes, run from the repository's root under the heavy lock `npm run check:all` takes. Each check
//! prints a line with its time, a failed one its output after it; the last line says whether all
//! passed. The release binaries are built first, alone; the docs lane, and the Rust lanes, run only
//! when the branch changed one of their inputs against its base (EN-085, EN-130), each skipping
//! said by a line: the nearest of `develop` and the feature branches on GitHub for a story,
//! `develop` for a feature; on `main`, `develop`, a detached head or in the CI every lane runs.
//! `.git/config` of the main checkout is read before and after the lanes: a lane that changed a
//! line a test could break fails the run, its lines named, nothing restored (F21.AC9, EN-113). The
//! timing tests run alone after the lanes (REV-008). A story or a bubble branch runs only the
//! checks its change needs, without the release build (`check_scope`, AUD-050), unless it changed
//! an input every check reads.
//!
//! `check-all --lane <name>` runs one lane's checks in turn, their output shown as it comes, as a
//! job of the CI does (ADR-0103); `check-all --list` prints each lane's checks and their commands.

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::thread;
use std::time::Instant;

use repo_tools::check_all::{
    AFTER_LANES, BEFORE_LANES, PLAYGROUND_TESTS, PLAYGROUND_TESTS_WITHOUT_COVERAGE, RUST_COVERAGE,
    TEST_SCRIPT, change_against_base, check_named, config_report, current_branch, git_lines,
    is_story_branch, lane_named, lanes_to_run, listed_lines, related_tests_command,
    related_tests_line, report_of, repository_tests, search_path, skipped_lines, summary,
};
use repo_tools::check_scope::{
    Check, Crate, RUST_TESTS, SKIPPED_ON_A_STORY, needs_full_check, read_crate, scope, scope_line,
};
use repo_tools::scripts::shell;

/// The result of one check.
struct Checked {
    name: String,
    is_ok: bool,
}

/// Runs a check's command in `sh -c` from the root, the programs npm installs found first
/// (`path`), stdout and stderr kept together in their order, then prints its report line.
fn run_check(check: &Check, root: &Path, path: &OsString) -> Checked {
    let start = Instant::now();
    let result = io::pipe().and_then(|(mut reader, writer)| {
        let mut shell = shell(&check.command, root);
        let mut child = shell
            .env("PATH", path)
            .stdin(Stdio::null())
            .stdout(writer.try_clone()?)
            .stderr(writer)
            .spawn()?;
        let mut output = Vec::new();

        // The command holds the pipe's writing ends until dropped: reading would never end.
        drop(shell);
        reader.read_to_end(&mut output)?;
        Ok((child.wait()?, output))
    });
    let (is_ok, output) = match result {
        Ok((status, output)) => (
            status.success(),
            String::from_utf8_lossy(&output).into_owned(),
        ),
        Err(error) => (false, format!("{}: {error}", check.name)),
    };
    let line = report_of(&check.name, is_ok, start.elapsed().as_millis(), &output);

    // One write per report, so that two lanes' lines never interleave.
    let _ = writeln!(io::stdout().lock(), "{line}");
    Checked {
        name: check.name.clone(),
        is_ok,
    }
}

/// Runs a check with its output shown as it comes, then prints its report line: a lane alone, as
/// a job of the CI runs it, a failing test annotated at its line by its own output.
fn run_shown(check: &Check, root: &Path, path: &OsString) -> Checked {
    let start = Instant::now();
    let status = shell(&check.command, root)
        .env("PATH", path)
        .stdin(Stdio::null())
        .status();
    let is_ok = status.as_ref().is_ok_and(std::process::ExitStatus::success);
    let error = status
        .err()
        .map(|error| error.to_string())
        .unwrap_or_default();

    println!(
        "{}",
        report_of(&check.name, is_ok, start.elapsed().as_millis(), &error)
    );
    Checked {
        name: check.name.clone(),
        is_ok,
    }
}

/// Runs lanes at the same time, the checks of a lane one after the other, every one even after a
/// failure.
fn run_lanes(lanes: &[Vec<Check>], root: &Path, path: &OsString) -> Vec<Checked> {
    thread::scope(|scope| {
        let handles: Vec<_> = lanes
            .iter()
            .map(|lane| {
                scope.spawn(|| {
                    lane.iter()
                        .map(|check| run_check(check, root, path))
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap_or_default())
            .collect()
    })
}

/// The main checkout's `.git/config`: the Git common directory's.
fn config_path(root: &Path) -> Option<PathBuf> {
    git_lines(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )
    .ok()?
    .first()
    .map(|common| Path::new(common).join("config"))
}

/// The workspace's crates, read from each `tools/*/Cargo.toml`.
fn workspace_crates(root: &Path) -> Vec<Crate> {
    let mut crates: Vec<Crate> = fs::read_dir(root.join("tools"))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let folder = entry.file_name().to_string_lossy().into_owned();
            let manifest = fs::read_to_string(entry.path().join("Cargo.toml")).ok()?;

            read_crate(&folder, &manifest)
        })
        .collect();

    crates.sort_by(|left, right| left.folder.cmp(&right.folder));
    crates
}

/// The lanes and the check after them of a full run: every lane the change does not leave out;
/// on a story whose change is known, without `deps:outdated`, and the Rust and the playground's
/// tests without their coverage.
fn full_run(
    change: Option<&(String, Vec<String>)>,
    is_story: bool,
    root: &Path,
) -> (Vec<Vec<Check>>, Check) {
    // In the CI, or without a base, every lane runs as it is.
    let is_story = is_story && change.is_some();
    let tests = change.and_then(|(_, paths)| {
        related_tests_command(
            is_story,
            Some(paths),
            &repository_tests(root).unwrap_or_default(),
        )
    });

    if let Some((base, paths)) = change {
        for line in skipped_lines(paths, base) {
            println!("{line}");
        }
        if tests.is_some() {
            println!("{}", related_tests_line(base));
        }
    }

    let lanes = lanes_to_run(change.map(|(_, paths)| paths.as_slice()))
        .iter()
        .map(|lane| {
            lane.iter()
                .filter(|name| !(is_story && SKIPPED_ON_A_STORY.contains(name)))
                .map(|name| {
                    let mut check = check_named(name);

                    if *name == TEST_SCRIPT
                        && let Some(command) = &tests
                    {
                        check.command.clone_from(command);
                    }
                    if is_story && *name == RUST_COVERAGE {
                        RUST_TESTS.clone_into(&mut check.command);
                    }
                    if is_story && *name == PLAYGROUND_TESTS {
                        PLAYGROUND_TESTS_WITHOUT_COVERAGE.clone_into(&mut check.command);
                    }
                    check
                })
                .collect()
        })
        .collect();

    (lanes, check_named(AFTER_LANES))
}

/// The checks of every lane, as the full run decides them.
fn check_everything(root: &Path, path: &OsString) -> Vec<Checked> {
    // In the CI, or when Git fails, the change is unknown: every lane runs.
    let change = if env::var_os("CI").is_some() {
        None
    } else {
        change_against_base(root).ok().flatten()
    };
    let is_story = current_branch(root).is_ok_and(|branch| is_story_branch(&branch));
    let scoped = change
        .as_ref()
        .filter(|(_, paths)| is_story && !needs_full_check(paths));
    let mut checked = Vec::new();

    if let Some((base, paths)) = scoped {
        let present: Vec<String> = paths
            .iter()
            .filter(|path| root.join(path).is_file())
            .cloned()
            .collect();
        let tests = repository_tests(root).unwrap_or_default();
        let plan = scope(paths, &present, &workspace_crates(root), &tests);

        println!("{}", scope_line(base, &plan));
        checked.extend(run_lanes(&plan.lanes, root, path));
        // Alone, so that no other lane's load slows the speeds they measure.
        checked.extend(plan.after.iter().map(|check| run_check(check, root, path)));
    } else {
        let (lanes, after) = full_run(change.as_ref(), is_story, root);
        let built = run_check(&check_named(BEFORE_LANES), root, path);

        // Without the release binaries no lane's tools can start: none starts.
        if built.is_ok {
            checked.extend(run_lanes(&lanes, root, path));
            checked.push(run_check(&after, root, path));
        }
        checked.insert(0, built);
    }
    checked
}

fn work() -> u8 {
    let start = Instant::now();
    let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let path = search_path(&root, env::var_os("PATH").as_ref());
    let arguments: Vec<String> = env::args().skip(1).collect();
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();

    match arguments.as_slice() {
        [] => {}
        ["--list"] => {
            for line in listed_lines() {
                println!("{line}");
            }
            return 0;
        }
        ["--lane", name] => {
            let Some(lane) = lane_named(name) else {
                eprintln!("check-all: no lane {name}");
                return 2;
            };
            let failed: Vec<String> = lane
                .iter()
                .map(|check| run_shown(&check_named(check), &root, &path))
                .filter(|checked| !checked.is_ok)
                .map(|checked| checked.name)
                .collect();

            println!("{}", summary(start.elapsed().as_millis(), &failed));
            return u8::from(!failed.is_empty());
        }
        _ => {
            eprintln!("{USAGE}");
            return 2;
        }
    }

    let config = config_path(&root);
    let before = config.as_ref().and_then(|path| fs::read(path).ok());
    let checked = check_everything(&root, &path);
    let mut failed: Vec<String> = checked
        .iter()
        .filter(|checked| !checked.is_ok)
        .map(|checked| checked.name.clone())
        .collect();

    if let Some(path) = &config {
        let after = fs::read(path).ok();

        if let Some(report) = config_report(
            &path.display().to_string(),
            before.as_deref(),
            after.as_deref(),
        ) {
            println!("{report}");
            failed.push(".git/config".to_owned());
        }
    }

    println!("{}", summary(start.elapsed().as_millis(), &failed));
    u8::from(!failed.is_empty())
}

/// How `check-all` is called.
const USAGE: &str = "usage: check-all [--lane <name> | --list]";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    tool_use::run("check-all", "", work)
}
