//! `check-all` run on repositories of its own, their scripts plain shell commands, and the lanes
//! read against this repository's `package.json` and test files. Every command starts through
//! `test_support::command`, without the inherited `GIT_*` variables.

#![cfg(unix)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use repo_tools::check_all::{BEFORE_LANES, LANES, changed_since, command_of, touches_site};
use repo_tools::scripts::read_scripts;
use test_support::command;

/// A new folder of the test's own.
fn scratch(name: &str) -> PathBuf {
    let folder = env::temp_dir().join(format!("check-all-{}-{name}", std::process::id()));

    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).expect("folder");
    fs::canonicalize(&folder).expect("canonical")
}

/// Runs Git in a folder, as a test user; its output, trimmed.
fn git(folder: &Path, arguments: &[&str]) -> String {
    let output = command("git")
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "init.defaultBranch=main",
        ])
        .args(arguments)
        .current_dir(folder)
        .output()
        .expect("git runs");

    assert!(
        output.status.success(),
        "git {}: {}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// A repository whose every script of the lanes passes, but those given.
fn repository(name: &str, scripts: &[(&str, &str)]) -> PathBuf {
    let folder = scratch(name);
    let names =
        std::iter::once(BEFORE_LANES).chain(LANES.iter().flat_map(|lane| lane.iter().copied()));
    let members: Vec<String> = names
        .map(|script| {
            let command = scripts
                .iter()
                .find(|(given, _)| *given == script)
                .map_or("true", |(_, command)| command);

            format!("{script:?}: {command:?}")
        })
        .collect();

    git(&folder, &["init", "-q"]);
    fs::write(
        folder.join("package.json"),
        format!("{{\"scripts\": {{{}}}}}\n", members.join(", ")),
    )
    .expect("manifest");
    folder
}

/// `check-all` run in a folder, in the CI unless told otherwise.
fn check_all(folder: &Path, in_ci: bool) -> Output {
    let mut check = command(env!("CARGO_BIN_EXE_check-all"));

    check.current_dir(folder);
    if in_ci {
        check.env("CI", "1");
    } else {
        check.env_remove("CI");
    }
    check.output().expect("check-all runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn runs_every_check_and_says_it_passed() {
    let folder = repository("passed", &[("lint", "echo linted")]);
    let output = check_all(&folder, true);
    let text = stdout(&output);
    let lines: Vec<&str> = text.lines().collect();

    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success(), "{text}");
    // The build first, then one line per check of every lane, then the summary.
    assert!(lines[0].starts_with("✓ rust:build "), "{text}");
    assert_eq!(
        lines.len(),
        1 + LANES.iter().map(|lane| lane.len()).sum::<usize>() + 1
    );
    assert!(lines.iter().any(|line| line.starts_with("✓ lint ")));
    assert!(lines.iter().any(|line| line.starts_with("✓ check:rust ")));
    assert!(
        !text.contains("linted"),
        "a passed check's output is kept back"
    );
    assert!(lines[lines.len() - 1].starts_with("check:all passed in "));
}

#[test]
fn prints_a_failed_check_with_its_output_and_runs_the_rest_of_its_lane() {
    let folder = repository(
        "failed",
        &[
            ("docs:build", "echo out; echo err >&2; exit 2"),
            ("playground:build", "echo built > built"),
        ],
    );
    let output = check_all(&folder, true);
    let text = stdout(&output);
    let built = folder.join("built").exists();

    fs::remove_dir_all(&folder).expect("cleaned");
    assert_eq!(output.status.code(), Some(1));
    assert!(text.contains(" s\nout\nerr\n"), "{text}");
    assert!(text.contains("✗ docs:build "), "{text}");
    assert!(built, "the next check of the lane ran");
    assert!(text.trim_end().ends_with(" s: docs:build"), "{text}");
}

#[test]
fn starts_no_lane_when_the_release_build_fails() {
    let folder = repository(
        "unbuilt",
        &[
            ("rust:build", "echo broken; exit 1"),
            ("lint", "touch linted"),
        ],
    );
    let output = check_all(&folder, true);
    let text = stdout(&output);
    let linted = folder.join("linted").exists();

    fs::remove_dir_all(&folder).expect("cleaned");
    assert_eq!(output.status.code(), Some(1));
    assert!(text.starts_with("✗ rust:build "), "{text}");
    assert!(text.contains("\nbroken\n"));
    assert!(!linted);
    assert!(text.trim_end().ends_with(" s: rust:build"), "{text}");
}

#[test]
fn f21_ac9_fails_naming_the_lines_a_lane_wrote_into_git_config_and_restores_nothing() {
    let folder = repository("config", &[("lint", "git config --local test.planted yes")]);
    let config = folder.join(".git/config");
    let before = fs::read_to_string(&config).expect("config");
    let output = check_all(&folder, true);
    let text = stdout(&output);
    let after = fs::read_to_string(&config).expect("config");

    fs::remove_dir_all(&folder).expect("cleaned");
    assert_eq!(output.status.code(), Some(1));
    // Every check passed: the config alone fails the run.
    assert!(text.contains("✓ lint "), "{text}");
    assert!(
        text.contains(&format!(
            "✗ {} changed while the checks ran, restored nothing:\n+ [test]\n+ \tplanted = yes\n",
            config.display()
        )),
        "{text}"
    );
    assert!(text.trim_end().ends_with(" s: .git/config"), "{text}");
    assert_eq!(after, format!("{before}[test]\n\tplanted = yes\n"));
}

#[test]
fn f19_ac8_skips_the_docs_lane_for_a_story_that_changed_none_of_its_inputs() {
    let root = scratch("skipped");
    let origin = root.join("origin.git");
    let folder = repository("skipped-work", &[("docs:build", "exit 1")]);

    git(
        &root,
        &["init", "-q", "--bare", origin.to_str().expect("path")],
    );
    git(&folder, &["add", "package.json"]);
    git(&folder, &["commit", "-q", "-m", "start"]);
    git(
        &folder,
        &["remote", "add", "origin", origin.to_str().expect("path")],
    );
    git(&folder, &["push", "-q", "origin", "HEAD:develop"]);
    git(&folder, &["fetch", "-q", "origin"]);
    git(&folder, &["switch", "-q", "-c", "chore/en-1-tools"]);
    fs::create_dir_all(folder.join("tools/x")).expect("folder");
    fs::write(folder.join("tools/x/lib.rs"), "\n").expect("file");

    let output = check_all(&folder, false);
    let text = stdout(&output);

    // A page of docs/, even uncommitted, brings the lane back.
    fs::create_dir_all(folder.join("docs")).expect("folder");
    fs::write(folder.join("docs/page.md"), "# Page\n").expect("page");
    let again = check_all(&folder, false);

    fs::remove_dir_all(&root).expect("cleaned");
    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success(), "{text}");
    assert!(text.starts_with(
        "- docs:build, playground:build skipped: nothing changed in their inputs against origin/develop\n"
    ));
    assert!(!text.contains("docs:build "), "{text}");
    assert_eq!(again.status.code(), Some(1));
    assert!(stdout(&again).contains("✗ docs:build "));
}

#[test]
fn f19_ac8_counts_a_page_moved_out_of_docs_under_its_old_path_too() {
    let folder = scratch("rename");

    git(&folder, &["init", "-q"]);
    fs::create_dir_all(folder.join("docs")).expect("docs");
    fs::write(
        folder.join("docs/page.md"),
        "# A page with enough text to be a rename\n",
    )
    .expect("page");
    git(&folder, &["add", "."]);
    git(&folder, &["commit", "-q", "-m", "page"]);

    let fork = git(&folder, &["rev-parse", "HEAD"]);

    fs::create_dir_all(folder.join("notes")).expect("notes");
    git(&folder, &["mv", "docs/page.md", "notes/page.md"]);
    git(&folder, &["commit", "-q", "-m", "move"]);

    let mut paths = changed_since(&folder, &fork).expect("paths");

    fs::remove_dir_all(&folder).expect("cleaned");
    paths.sort();
    assert_eq!(paths, ["docs/page.md", "notes/page.md"]);
    assert!(touches_site(&paths));
}

/// This repository's root.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The scripts of this repository's `package.json`.
fn scripts() -> Vec<(String, String)> {
    read_scripts(&fs::read_to_string(root().join("package.json")).expect("manifest"))
}

/// The release binaries a command writes into `target/release/`, through the npm scripts it runs:
/// `cargo run` copies its crate's binary there again even when it is up to date, and the release
/// build of the workspace every binary (`workspace`).
fn release_binaries(command: &str, scripts: &[(String, String)]) -> Vec<String> {
    let words: Vec<&str> = command.split_whitespace().collect();
    let mut crates = Vec::new();

    for (at, window) in words.windows(5).enumerate() {
        if window[..4] == ["cargo", "run", "--release", "-q"]
            && window[4] == "-p"
            && let Some(name) = words.get(at + 5)
        {
            crates.push((*name).to_owned());
        }
    }
    if command.contains("cargo build --release") {
        crates.push("workspace".to_owned());
    }
    for (at, pair) in words.windows(2).enumerate() {
        if pair == ["npm", "run"] {
            let next = words.get(at + 2).copied().unwrap_or_default();
            let script = if next == "--silent" {
                words.get(at + 3).copied().unwrap_or_default()
            } else {
                next
            };
            let script = script.trim_matches('"');

            if let Some((_, nested)) = scripts.iter().find(|(name, _)| name == script) {
                crates.extend(release_binaries(nested, scripts));
            }
        }
    }
    crates
}

/// The release binaries the test files write: a test may start `cargo run --release -p <crate>` as
/// well as a script, in the `test` lane (`tests/declarations.test.ts` runs `declarations`).
fn test_file_binaries() -> Vec<String> {
    let listed = git(&root(), &["ls-files", "*.test.ts"]);
    let marker = "\"run\", \"--release\", \"-q\", \"-p\", \"";

    listed
        .lines()
        .flat_map(|path| {
            let text = fs::read_to_string(root().join(path)).unwrap_or_default();
            let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");

            flat.match_indices(marker)
                .filter_map(|(at, _)| {
                    let rest = &flat[at + marker.len()..];

                    rest.split('"').next().map(str::to_owned)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn runs_every_check_of_check_the_playground_after_the_site() {
    let scripts = scripts();
    let check = scripts
        .iter()
        .find(|(name, _)| name == "check")
        .map(|(_, command)| command.clone())
        .expect("a check script");
    let mut listed: Vec<String> = check
        .split("npm run ")
        .skip(1)
        .filter_map(|rest| rest.split_whitespace().next())
        .map(str::to_owned)
        .chain(["deps:outdated", "docs:build", "playground:build"].map(str::to_owned))
        .collect();
    let mut lanes: Vec<String> = LANES
        .iter()
        .flat_map(|lane| lane.iter().map(|&name| name.to_owned()))
        .collect();

    listed.sort();
    lanes.sort();
    assert_eq!(lanes, listed);
    assert!(LANES.contains(&&["docs:build", "playground:build"][..]));
}

#[test]
fn f19_ac9_writes_no_release_binary_from_two_lanes_nor_the_workspace_during_the_lanes() {
    // EN-069's pre-push: one lane's `cargo run` replaced target/release/heavy-lock while another
    // lane's started it, which failed with `could not execute process … No such file or directory`.
    let scripts = scripts();
    let tests = test_file_binaries();
    let written: Vec<Vec<String>> = LANES
        .iter()
        .map(|lane| {
            let mut crates: Vec<String> = lane
                .iter()
                .flat_map(|name| {
                    let script = scripts
                        .iter()
                        .find(|(script, _)| script == name)
                        .map_or("", |(_, command)| command.as_str());

                    release_binaries(command_of(script), &scripts)
                })
                .collect();

            if lane.contains(&"test") {
                crates.extend(tests.iter().cloned());
            }
            crates
        })
        .collect();
    let shared: Vec<&String> = written
        .iter()
        .flatten()
        .filter(|name| written.iter().filter(|lane| lane.contains(name)).count() > 1)
        .collect();

    assert!(tests.contains(&"declarations".to_owned()), "{tests:?}");
    assert!(!written.iter().flatten().any(|name| name == "workspace"));
    assert_eq!(shared, Vec::<&String>::new());
    // The reading itself: a lane running `npm run lint` writes lint's binary.
    assert_eq!(
        release_binaries("npm run --silent lint", &scripts),
        ["lint"]
    );
}
