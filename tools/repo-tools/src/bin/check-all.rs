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

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::thread;
use std::time::Instant;

use repo_tools::check_all::{
    AFTER_LANES, BEFORE_LANES, RUST_COVERAGE, TEST_SCRIPT, change_against_base, command_of,
    config_report, current_branch, git_lines, is_story_branch, lanes_to_run, related_tests_command,
    related_tests_line, report_of, repository_tests, skipped_lines, summary,
};
use repo_tools::check_scope::{
    Check, Crate, RUST_TESTS, SKIPPED_ON_A_STORY, needs_full_check, read_crate, scope, scope_line,
};
use repo_tools::scripts::{read_scripts, script_command, shell};

/// The result of one check.
struct Checked {
    name: String,
    is_ok: bool,
}

/// Runs a command in the shell npm runs a script in, stdout and stderr kept together in their
/// order, then prints its report line; `check:all` is started by npm, so the command finds the
/// tools of `node_modules/.bin`.
fn run_check(check: &Check, root: &Path) -> Checked {
    let start = Instant::now();
    let result = io::pipe().and_then(|(mut reader, writer)| {
        let mut shell = shell(command_of(&check.command), root);
        let mut child = shell
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

/// Runs lanes at the same time, the checks of a lane one after the other, every one even after a
/// failure.
fn run_lanes(lanes: &[Vec<Check>], root: &Path) -> Vec<Checked> {
    thread::scope(|scope| {
        let handles: Vec<_> = lanes
            .iter()
            .map(|lane| {
                scope.spawn(|| {
                    lane.iter()
                        .map(|check| run_check(check, root))
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

/// A script as a check: its command from `package.json`.
fn script(name: &str, scripts: &[(String, String)]) -> Check {
    Check {
        name: name.to_owned(),
        command: script_command(scripts, name),
    }
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
/// on a story whose change is known, without `deps:outdated`, and the Rust tests without their
/// coverage.
fn full_run(
    scripts: &[(String, String)],
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
                    let mut check = script(name, scripts);

                    if *name == TEST_SCRIPT
                        && let Some(command) = &tests
                    {
                        check.command.clone_from(command);
                    }
                    if is_story && *name == RUST_COVERAGE {
                        RUST_TESTS.clone_into(&mut check.command);
                    }
                    check
                })
                .collect()
        })
        .collect();

    (lanes, script(AFTER_LANES, scripts))
}

fn work() -> u8 {
    let start = Instant::now();
    let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let scripts = read_scripts(&fs::read_to_string(root.join("package.json")).unwrap_or_default());
    // In the CI, or when Git fails, the change is unknown: every lane runs.
    let change = if env::var_os("CI").is_some() {
        None
    } else {
        change_against_base(&root).ok().flatten()
    };
    let is_story = current_branch(&root).is_ok_and(|branch| is_story_branch(&branch));
    let scoped = change
        .as_ref()
        .filter(|(_, paths)| is_story && !needs_full_check(paths));
    let config = config_path(&root);
    let before = config.as_ref().and_then(|path| fs::read(path).ok());
    let mut checked = Vec::new();

    if let Some((base, paths)) = scoped {
        let present: Vec<String> = paths
            .iter()
            .filter(|path| root.join(path).is_file())
            .cloned()
            .collect();
        let tests = repository_tests(&root).unwrap_or_default();
        let plan = scope(paths, &present, &workspace_crates(&root), &tests);

        println!("{}", scope_line(base, &plan));
        checked.extend(run_lanes(&plan.lanes, &root));
        // Alone, so that no other lane's load slows the speeds they measure.
        checked.extend(plan.after.iter().map(|check| run_check(check, &root)));
    } else {
        let (lanes, after) = full_run(&scripts, change.as_ref(), is_story, &root);
        let built = run_check(&script(BEFORE_LANES, &scripts), &root);

        // Without the release binaries every lane's `cargo run` would build them at once: none
        // starts.
        if built.is_ok {
            checked.extend(run_lanes(&lanes, &root));
            checked.push(run_check(&after, &root));
        }
        checked.insert(0, built);
    }

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

fn main() -> ExitCode {
    tool_use::run("check-all", "", work)
}
