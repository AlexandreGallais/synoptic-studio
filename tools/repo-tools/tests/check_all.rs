//! `check-all` run on repositories of its own, each program its checks call a stub, and the lanes
//! read against this repository's workflows and test files. Every command starts through
//! `test_support::command`, without the inherited `GIT_*` variables.

#![cfg(unix)]

use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Output;

use repo_tools::check_all::{
    AFTER_LANES, BEFORE_LANES, CHECKS, DOCS_LANE, LANES, NAMED_LANES, RUST_LANES, changed_since,
    check_named, lanes_to_run, skipped_lines, touches_rust, touches_site,
};
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

/// The stub of Vitest: its first call's arguments written into `.git/vitest-args`, a folder Git
/// does not list among the changes, so that the test lane's call is read, not the timing tests'.
const VITEST: &str = "[ -e .git/vitest-args ] || echo \"$*\" > .git/vitest-args";

/// The programs the checks call: those found on `PATH`, then the release binaries by their path.
const PROGRAMS: [&str; 12] = [
    "oxlint",
    "vitest",
    "vite",
    "vitepress",
    "npm",
    "cargo",
    "target/release/lint",
    "target/release/api-pages",
    "target/release/format",
    "target/release/secret-scan",
    "target/release/dependencies",
    "target/release/stale-binaries",
];

/// A repository whose every program the checks call is a stub that passes, Vitest's writing its
/// arguments, but those given their own shell body; `target/` is left out of the change.
fn repository(name: &str, stubs: &[(&str, &str)]) -> PathBuf {
    let folder = scratch(name);

    git(&folder, &["init", "-q"]);
    fs::write(folder.join(".git/info/exclude"), "target/\n").expect("exclude");
    fs::write(folder.join("package.json"), "{}\n").expect("manifest");
    for program in PROGRAMS {
        let body = stubs.iter().find(|(given, _)| *given == program).map_or(
            if program == "vitest" { VITEST } else { "true" },
            |(_, body)| body,
        );
        let path = if program.contains('/') {
            folder.join(program)
        } else {
            folder.join(".git/stubs").join(program)
        };

        fs::create_dir_all(path.parent().expect("parent")).expect("folder");
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("stub");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("mode");
    }
    folder
}

/// `check-all` run in a folder, in the CI unless told otherwise, the stubs first on `PATH`, Vitest's
/// arguments of the run before removed.
fn check_all(folder: &Path, in_ci: bool) -> Output {
    let mut check = command(env!("CARGO_BIN_EXE_check-all"));

    let _ = fs::remove_file(folder.join(".git/vitest-args"));
    check.env(
        "PATH",
        format!(
            "{}:{}",
            folder.join(".git/stubs").display(),
            env::var("PATH").unwrap_or_default()
        ),
    );
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
    let folder = repository("passed", &[("target/release/lint", "echo linted")]);
    let output = check_all(&folder, true);
    let text = stdout(&output);
    let lines: Vec<&str> = text.lines().collect();

    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success(), "{text}");
    // The build first, then one line per check of every lane, the timing tests alone after them,
    // then the summary.
    assert!(lines[0].starts_with("✓ rust:build "), "{text}");
    assert_eq!(
        lines.len(),
        1 + LANES.iter().map(|lane| lane.len()).sum::<usize>() + 2
    );
    assert!(
        lines[lines.len() - 2].starts_with("✓ test:timings "),
        "{text}"
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
            ("vitepress", "echo out; echo err >&2; exit 2"),
            ("vite", "echo built > built"),
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
            (
                "cargo",
                "if [ \"$1\" = build ]; then echo broken; exit 1; fi",
            ),
            ("target/release/lint", "touch linted"),
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
    let folder = repository(
        "config",
        &[("target/release/lint", "git config --local core.planted yes")],
    );
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
            "✗ {} changed while the checks ran, restored nothing:\n+ \tplanted = yes\n",
            config.display()
        )),
        "{text}"
    );
    assert!(text.trim_end().ends_with(" s: .git/config"), "{text}");
    assert_eq!(after.replace("\tplanted = yes\n", ""), before);
}

#[test]
fn f21_ac9_passes_when_a_lane_only_tracks_a_branch_in_git_config() {
    // What a parallel worktree's `git push -u` writes into the common `.git/config` (CHK-022).
    let folder = repository(
        "tracked",
        &[(
            "target/release/lint",
            "git config --local branch.x.remote origin && git config --local branch.x.merge refs/heads/x",
        )],
    );
    let output = check_all(&folder, true);
    let text = stdout(&output);
    let after = fs::read_to_string(folder.join(".git/config")).expect("config");

    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success(), "{text}");
    assert!(
        after.contains("[branch \"x\"]\n\tremote = origin\n"),
        "{after}"
    );
    assert!(!text.contains(".git/config"), "{text}");
}

#[test]
fn f19_ac8_skips_the_docs_lane_for_a_feature_that_changed_none_of_its_inputs() {
    let root = scratch("skipped");
    let origin = root.join("origin.git");
    let folder = repository("skipped-work", &[("vitepress", "exit 1")]);

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
    git(&folder, &["switch", "-q", "-c", "feature/f1-tools"]);
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
fn runs_on_a_story_branch_changing_a_checks_input_only_the_tests_related_to_its_change() {
    let root = scratch("related");
    let origin = root.join("origin.git");
    let folder = repository("related-work", &[]);

    git(
        &root,
        &["init", "-q", "--bare", origin.to_str().expect("path")],
    );
    fs::create_dir_all(folder.join("tests")).expect("folder");
    fs::write(folder.join("tests/docs.test.ts"), "\n").expect("test");
    git(&folder, &["add", "package.json", "tests/docs.test.ts"]);
    git(&folder, &["commit", "-q", "-m", "start"]);
    git(
        &folder,
        &["remote", "add", "origin", origin.to_str().expect("path")],
    );
    git(&folder, &["push", "-q", "origin", "HEAD:develop"]);
    git(&folder, &["fetch", "-q", "origin"]);
    git(&folder, &["switch", "-q", "-c", "fix/us-001-scan"]);
    fs::create_dir_all(folder.join("src")).expect("folder");
    fs::write(folder.join("src/scan.ts"), "\n").expect("file");
    // oxlint's configuration changed: the lanes run, not the scoped checks (AUD-050).
    fs::write(folder.join(".oxlintrc.json"), "{}\n").expect("config");

    let output = check_all(&folder, false);
    let text = stdout(&output);
    let related = fs::read_to_string(folder.join(".git/vitest-args")).expect("args");

    // `vite.config.ts` changed: every input of every test, the whole script.
    fs::write(folder.join("vite.config.ts"), "\n").expect("config");
    let whole = check_all(&folder, false);
    let all = fs::read_to_string(folder.join(".git/vitest-args")).expect("args");

    // The same change on a feature branch: the whole script too.
    fs::remove_file(folder.join("vite.config.ts")).expect("removed");
    git(&folder, &["switch", "-q", "-c", "feature/f1-scan"]);
    check_all(&folder, false);
    let feature = fs::read_to_string(folder.join(".git/vitest-args")).expect("args");

    fs::remove_dir_all(&root).expect("cleaned");
    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success(), "{text}");
    assert!(text.contains(
        "- test: only the tests related to the change against origin/develop, without coverage"
    ));
    // On a story, the dependency updates wait for the review (AUD-050).
    assert!(!text.contains("deps:outdated"), "{text}");
    assert!(text.contains("✓ deps:unused "), "{text}");
    // The change, then the repository's own test, which reads files beside its imports.
    assert_eq!(
        related,
        "related --run --passWithNoTests --tags-filter=!timing .oxlintrc.json src/scan.ts \
         tests/docs.test.ts\n"
    );
    assert!(whole.status.success());
    assert!(!stdout(&whole).contains("- test: only"));
    assert_eq!(all, "run --coverage --tags-filter=!timing\n");
    assert_eq!(feature, all);
}

#[test]
fn aud050_runs_on_a_story_branch_only_the_checks_its_change_needs() {
    let root = scratch("scoped");
    let origin = root.join("origin.git");
    // Every program fails but the project's linter, which writes its arguments, and Vitest: no
    // other runs on a story's scoped push of Markdown.
    let failing: Vec<(&str, &str)> = PROGRAMS
        .iter()
        .map(|&program| match program {
            "target/release/lint" => (program, "echo \"$*\" > .git/lint-args"),
            "vitest" => (program, VITEST),
            _ => (program, "exit 1"),
        })
        .collect();
    let folder = repository("scoped-work", &failing);

    git(
        &root,
        &["init", "-q", "--bare", origin.to_str().expect("path")],
    );
    fs::create_dir_all(folder.join("tests")).expect("folder");
    fs::write(folder.join("tests/docs.test.ts"), "\n").expect("test");
    git(&folder, &["add", "package.json", "tests/docs.test.ts"]);
    git(&folder, &["commit", "-q", "-m", "start"]);
    git(
        &folder,
        &["remote", "add", "origin", origin.to_str().expect("path")],
    );
    git(&folder, &["push", "-q", "origin", "HEAD:develop"]);
    git(&folder, &["fetch", "-q", "origin"]);
    git(&folder, &["switch", "-q", "-c", "docs/en-001-page"]);
    fs::create_dir_all(folder.join("docs")).expect("folder");
    fs::write(folder.join("docs/page.md"), "# Page\n").expect("page");

    let output = check_all(&folder, false);
    let text = stdout(&output);
    let linted = fs::read_to_string(folder.join(".git/lint-args")).unwrap_or_default();
    let related = fs::read_to_string(folder.join(".git/vitest-args")).unwrap_or_default();

    fs::remove_dir_all(&root).expect("cleaned");
    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success(), "{text}");
    assert!(
        text.starts_with(
            "- scoped push: only what the change against origin/develop needs (lint, test); the \
             full check:all on a feature branch\n"
        ),
        "{text}"
    );
    assert!(text.contains("✓ lint "), "{text}");
    assert!(text.contains("✓ test "), "{text}");
    // No release build, no lane, no timing tests for Markdown alone.
    for script in [BEFORE_LANES, "docs:build", "deps:outdated", AFTER_LANES] {
        assert!(!text.contains(&format!("{script} ")), "{script}: {text}");
    }
    assert_eq!(linted, "docs/page.md\n");
    assert_eq!(
        related,
        "related --run --passWithNoTests --tags-filter=!timing docs/page.md tests/docs.test.ts\n"
    );
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

/// The release binaries a command calls by their path, `target/release/<name>`.
fn called_binaries(command: &str) -> Vec<String> {
    command
        .split_whitespace()
        .filter_map(|word| word.strip_prefix("target/release/"))
        .map(str::to_owned)
        .collect()
}

/// The release binaries the test files write: a test may start `cargo run --release -p <crate>`
/// in the `test` lane (`tests/declarations.test.ts` runs `declarations`).
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

/// A workflow of this repository, by its file name in `.github/workflows/`.
fn workflow(name: &str) -> String {
    fs::read_to_string(root().join(".github").join("workflows").join(name)).expect("workflow")
}

/// The jobs of the CI's matrix (`ci.yml`, EN-124): each `- name:` of its `include:` list with the
/// lanes of its `lanes:` line, in their order; the list ends at the first line indented less.
fn matrix_jobs(text: &str) -> Vec<(String, Vec<String>)> {
    let mut jobs: Vec<(String, Vec<String>)> = Vec::new();
    let mut indent = None;

    for line in text.lines() {
        let trimmed = line.trim_start();
        let depth = line.len() - trimmed.len();

        if trimmed == "include:" {
            indent = Some(depth);
            continue;
        }
        let Some(list) = indent else { continue };

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if depth <= list {
            indent = None;
        } else if let Some(name) = trimmed.strip_prefix("- name: ") {
            jobs.push((name.to_owned(), Vec::new()));
        } else if let (Some(lanes), Some(job)) = (trimmed.strip_prefix("lanes: "), jobs.last_mut())
        {
            job.1 = lanes.split_whitespace().map(str::to_owned).collect();
        }
    }
    jobs
}

/// The names of a workflow's jobs, a matrix job's `${{ matrix.name }}` read as its matrix's names.
fn job_names(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("    name: "))
        .flat_map(|name| {
            if name == "${{ matrix.name }}" {
                matrix_jobs(text)
                    .into_iter()
                    .map(|(name, _)| name)
                    .collect()
            } else {
                vec![name.to_owned()]
            }
        })
        .collect()
}

/// The checks `main`'s ruleset requires, as `docs/tooling/ci.md` writes them: the names in code
/// font after "require status checks" on the ruleset's row.
fn required_checks(page: &str) -> Vec<String> {
    let row = page
        .lines()
        .find(|line| line.trim_start().starts_with("| `main` | default branch |"))
        .unwrap_or_default();
    let checks = row
        .split_once("require status checks")
        .map_or("", |(_, rest)| rest);

    checks
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

#[test]
fn f24_ac1_runs_each_lane_of_check_all_in_exactly_one_job_of_the_ci() {
    let text = workflow("ci.yml");
    let jobs = matrix_jobs(&text);
    let run: Vec<&String> = jobs.iter().flat_map(|(_, lanes)| lanes).collect();
    let not_once: Vec<&str> = NAMED_LANES
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| run.iter().filter(|lane| *lane == name).count() != 1)
        .collect();

    // Each named lane in one job, the timing tests after the tests in theirs, alone on its runner
    // as after the lanes; one job per lane of `check:all`, every name one `check-all` knows.
    assert_eq!(not_once, Vec::<&str>::new());
    assert_eq!(run.len(), NAMED_LANES.len());
    assert_eq!(jobs.len(), LANES.len(), "{jobs:?}");
    assert!(jobs.iter().any(|(_, lanes)| lanes == &["test", "timings"]));
    // Each job a check of its own name, run to its end whichever other fails (`REF-GH-JOB-PERMISSIONS`).
    assert!(text.contains("\n    name: ${{ matrix.name }}\n"));
    assert!(text.contains("\n      fail-fast: false\n"));
    assert!(text.contains(
        "- run: for lane in $LANES; do target/release/check-all --lane \"$lane\"; done\n        env:\n          LANES: ${{ matrix.lanes }}\n"
    ));
}
#[test]
fn f24_ac1_reads_the_jobs_of_a_matrix_to_its_end() {
    let text = "jobs:\n  lanes:\n    name: ${{ matrix.name }}\n    strategy:\n      matrix:\n        \
                include:\n          # A comment.\n          - name: A\n            lanes: a b\n\n          \
                - name: B\n            lanes: c\n    steps:\n      - name: x\n        lanes: d\n  \
                other:\n    name: Other\n";

    assert_eq!(
        matrix_jobs(text),
        [
            ("A".to_owned(), vec!["a".to_owned(), "b".to_owned()]),
            ("B".to_owned(), vec!["c".to_owned()])
        ]
    );
    assert_eq!(job_names(text), ["A", "B", "Other"]);
    assert_eq!(matrix_jobs("jobs:\n"), Vec::<(String, Vec<String>)>::new());
    assert_eq!(
        required_checks(
            "   | `main` | default branch | none | merge method **Merge**, require status checks: `A`, `B` |"
        ),
        ["A", "B"]
    );
    assert_eq!(required_checks("| `develop` |"), Vec::<String>::new());
}

#[test]
fn f24_ac1_requires_on_main_every_job_of_the_ci_and_codeql_by_its_name() {
    let mut jobs = job_names(&workflow("ci.yml"));
    let mut required =
        required_checks(&fs::read_to_string(root().join("docs/tooling/ci.md")).expect("ci.md"));

    jobs.extend(job_names(&workflow("codeql.yml")));
    jobs.sort();
    required.sort();
    assert_eq!(required, jobs);
    assert!(jobs.contains(&"Rust".to_owned()), "{jobs:?}");
}

#[test]
fn f19_ac9_writes_no_release_binary_a_lane_calls_during_the_lanes() {
    // EN-069's pre-push: one lane's `cargo run` replaced target/release/heavy-lock while another
    // lane's started it, which failed with `could not execute process … No such file or directory`.
    // The checks call the binaries built before the lanes by their path; a test file may still
    // start `cargo run --release -p <crate>`, whose binary no lane may call.
    let tests = test_file_binaries();
    let called: Vec<String> = CHECKS
        .iter()
        .filter(|(name, _)| *name != BEFORE_LANES)
        .flat_map(|(_, command)| called_binaries(command))
        .collect();

    assert!(tests.contains(&"declarations".to_owned()), "{tests:?}");
    assert!(called.contains(&"lint".to_owned()), "{called:?}");
    assert_eq!(
        tests
            .iter()
            .filter(|name| called.contains(name))
            .collect::<Vec<_>>(),
        Vec::<&String>::new()
    );
    assert_eq!(
        called_binaries("a && target/release/lint x && target/release/format"),
        ["lint", "format"]
    );
}

/// Paths given as owned strings, as `check-all` reads them from Git.
fn owned(paths: &[&str]) -> Vec<String> {
    paths.iter().map(|&path| path.to_owned()).collect()
}

/// The root's tracked entries the Rust lanes do not read whole — neither Cargo, rustup nor
/// cargo-deny, a file inside one that the crates' tests read being named among the Rust inputs
/// (`.github/workflows/`): a new entry fails `[F29.AC2]` until it is listed here or there.
const NOT_RUST_INPUTS: &[&str] = &[
    ".agents",
    ".claude",
    ".gitattributes",
    ".githooks",
    ".github",
    ".gitignore",
    ".lycheeignore",
    ".mcp.json",
    ".nvmrc",
    ".oxlintrc.json",
    "AGENTS.md",
    "CLAUDE.md",
    "LICENSE",
    "NOTICE",
    "README.md",
    "SECURITY.md",
    "THIRD-PARTY-LICENSES.md",
    "docs",
    "playground",
    "scripts",
    "src",
    "tests",
    "tsconfig.json",
    "vite.config.ts",
];

#[test]
fn f29_ac2_reads_cargo_s_inputs_and_the_files_the_crates_tests_read() {
    // Cargo's own: a crate, the workspace's manifest and lock, the toolchain, Cargo's settings,
    // cargo-deny's.
    for path in [
        "tools/lint/src/lib.rs",
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        ".cargo/config.toml",
        "deny.toml",
    ] {
        assert!(touches_rust(&owned(&[path])), "{path}");
    }
    // Read by the crates' tests outside tools/: the scripts, the formatter's oxfmt and settings,
    // the CI and its required checks, the lint's folders, the hooks' settings and the test files.
    for path in [
        "package.json",
        "package-lock.json",
        ".oxfmtrc.json",
        ".github/workflows/ci.yml",
        "docs/tooling/ci.md",
        "docs/conventions/files.md",
        ".claude/settings.json",
        "src/math/angle/degrees.test.ts",
        "tests/declarations.test.ts",
    ] {
        assert!(touches_rust(&owned(&[path])), "{path}");
    }
    // A story of Markdown, of the library's code or of a script: none of them.
    assert!(!touches_rust(&owned(&[
        "playground/checklists/epic-17.ts",
        "docs/backlog/stories/x.md",
        "docs/tooling/commands.md",
        "src/math/angle/degrees.ts",
        "playground/main.ts",
        "scripts/default-font-metrics.ts",
        "README.md",
        ".github/dependabot.yml",
        ".claude/rules/tooling.md"
    ])));
    // A name that only starts like a folder or a file of them is not one.
    assert!(!touches_rust(&owned(&[
        "toolsy/x.rs",
        "Cargo.toml.bak",
        "deny.tomlx",
        ".cargox/config.toml",
        "x.test.ts.snap"
    ])));
    assert!(!touches_rust(&[]));
}

#[test]
fn f29_ac2_counts_a_new_crate_of_the_workspace_without_a_change_of_the_list() {
    // The workspace's members are the folders of `tools/` (`members = ["tools/*"]`): a crate not
    // yet written is one of the inputs, and so is every crate there.
    let manifest = fs::read_to_string(root().join("Cargo.toml")).expect("manifest");
    let members = manifest
        .lines()
        .find_map(|line| line.trim().strip_prefix("members = "))
        .expect("members");
    let crates: Vec<String> = fs::read_dir(root().join("tools"))
        .expect("tools")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("Cargo.toml").is_file())
        .map(|entry| format!("tools/{}/Cargo.toml", entry.file_name().to_string_lossy()))
        .collect();

    assert_eq!(members, "[\"tools/*\"]");
    assert!(touches_rust(&owned(&["tools/a-crate-to-come/src/main.rs"])));
    assert!(crates.len() > 10, "{crates:?}");
    assert_eq!(
        crates
            .iter()
            .filter(|path| !touches_rust(std::slice::from_ref(path)))
            .collect::<Vec<_>>(),
        Vec::<&String>::new()
    );
}

#[test]
fn f29_ac2_names_each_root_entry_as_a_rust_input_or_one_the_rust_lanes_never_read() {
    // A file added at the root that Cargo reads (a `clippy.toml`, a `rustfmt.toml`) fails here
    // until it joins the Rust inputs, or this list if no Rust lane reads it.
    let tracked = git(&root(), &["ls-files"]);
    let mut entries: Vec<&str> = tracked
        .lines()
        .filter_map(|path| path.split('/').next())
        .collect();
    // The entry as a file, and a file inside it as a folder.
    let read = |entry: &str| touches_rust(&[entry.to_owned(), format!("{entry}/x")]);

    entries.sort_unstable();
    entries.dedup();
    let unnamed: Vec<&str> = entries
        .iter()
        .copied()
        .filter(|entry| !NOT_RUST_INPUTS.contains(entry) && !read(entry))
        .collect();
    let both: Vec<&str> = NOT_RUST_INPUTS
        .iter()
        .copied()
        .filter(|entry| read(entry))
        .collect();

    assert!(
        entries.contains(&"tools") && entries.contains(&"docs"),
        "{entries:?}"
    );
    assert_eq!(unnamed, Vec::<&str>::new());
    // An entry the Rust lanes read whole is never listed as one they do not.
    assert_eq!(both, Vec::<&str>::new());
}

#[test]
fn f29_ac2_runs_the_rust_coverage_as_a_lane_of_its_own() {
    let script = |name: &str| check_named(name).command;

    assert_eq!(RUST_LANES, [&["check:rust"][..], &["rust:coverage"][..]]);
    assert!(
        RUST_LANES.iter().all(|lane| LANES.contains(lane)),
        "{LANES:?}"
    );
    // The coverage left out of `check:rust`, which keeps rustfmt, Clippy, the advisories, the
    // check for Windows and the binaries' freshness.
    assert!(!script("check:rust").contains("llvm-cov"));
    for part in [
        "cargo fmt --all --check",
        "cargo clippy --workspace --all-targets --locked --quiet",
        "cargo deny --offline check advisories",
        "--target x86_64-pc-windows-msvc",
        "target/release/stale-binaries",
    ] {
        assert!(script("check:rust").contains(part), "{part}");
    }
    assert!(script("rust:coverage").contains(
        "cargo llvm-cov --workspace --locked --quiet --summary-only --fail-under-lines 95 --fail-under-functions 95"
    ));
}

#[test]
fn f29_ac2_skips_the_rust_lanes_only_when_the_change_is_known_and_touches_none_of_their_inputs() {
    let docs = owned(&["docs/index.md"]);
    let script = owned(&["scripts/default-font-metrics.ts"]);
    let tools = owned(&["tools/lint/src/lib.rs"]);
    let without = |paths: &[String]| {
        LANES
            .iter()
            .copied()
            .filter(|lane| !lanes_to_run(Some(paths)).contains(lane))
            .collect::<Vec<_>>()
    };

    // A story of Markdown: the Rust lanes left out, the docs lane run.
    assert_eq!(without(&docs), RUST_LANES);
    // A script outside both: the docs lane and the Rust lanes left out.
    assert_eq!(without(&script), [&[DOCS_LANE][..], RUST_LANES].concat());
    // A crate changed: every Rust lane run.
    assert_eq!(without(&tools), [DOCS_LANE]);
    // No base known (main, develop, a detached head, the CI): every lane.
    assert_eq!(lanes_to_run(None), LANES);

    let skipped = "skipped: nothing changed in their inputs against origin/develop";

    assert_eq!(
        skipped_lines(&docs, "origin/develop"),
        [format!("- check:rust, rust:coverage {skipped}")]
    );
    assert_eq!(
        skipped_lines(&script, "origin/develop"),
        [
            format!("- docs:build, playground:build {skipped}"),
            format!("- check:rust, rust:coverage {skipped}")
        ]
    );
    assert_eq!(
        skipped_lines(&owned(&["docs/index.md", "Cargo.lock"]), "origin/develop"),
        Vec::<String>::new()
    );
}

#[test]
fn f29_ac2_runs_every_lane_in_the_ci_even_with_a_base_known() {
    let root = scratch("ci-known-base");
    let origin = root.join("origin.git");
    let folder = repository("ci-known-base-work", &[]);

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
    git(&folder, &["switch", "-q", "-c", "docs/en-001-page"]);
    // A story of docs alone: outside the CI it skips both Rust lanes (the test above).
    fs::create_dir_all(folder.join("docs")).expect("folder");
    fs::write(folder.join("docs/page.md"), "# Page\n").expect("page");

    let output = check_all(&folder, true);
    let text = stdout(&output);

    fs::remove_dir_all(&root).expect("cleaned");
    fs::remove_dir_all(&folder).expect("cleaned");
    // In the CI the change is never read (AUD-031, F8): every lane runs, none said skipped.
    assert!(output.status.success(), "{text}");
    assert!(!text.contains("skipped"), "{text}");
    for lane in LANES.iter().flat_map(|lane| lane.iter()) {
        assert!(text.contains(&format!("✓ {lane} ")), "{lane}: {text}");
    }
}

#[test]
fn f29_ac2_skips_the_rust_lanes_for_a_feature_that_changed_none_of_their_inputs() {
    let root = scratch("rust-skipped");
    let origin = root.join("origin.git");
    let folder = repository(
        "rust-skipped-work",
        // cargo fails but for the release build, failing `check:rust` and `rust:coverage`.
        &[("cargo", "[ \"$1\" = build ]")],
    );

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
    git(&folder, &["switch", "-q", "-c", "feature/f1-docs"]);
    fs::create_dir_all(folder.join("docs")).expect("folder");
    fs::write(folder.join("docs/page.md"), "# Page\n").expect("page");

    let output = check_all(&folder, false);
    let text = stdout(&output);

    // A crate of tools/, even uncommitted, brings both lanes back.
    fs::create_dir_all(folder.join("tools/x")).expect("folder");
    fs::write(folder.join("tools/x/lib.rs"), "\n").expect("file");
    let again = check_all(&folder, false);
    let again_text = stdout(&again);

    fs::remove_dir_all(&root).expect("cleaned");
    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success(), "{text}");
    assert!(text.starts_with(
        "- check:rust, rust:coverage skipped: nothing changed in their inputs against origin/develop\n"
    ), "{text}");
    // Neither lane reported, passed nor failed.
    for lane in ["check:rust", "rust:coverage"] {
        assert!(!text.contains(&format!("✓ {lane} ")), "{text}");
        assert!(!text.contains(&format!("✗ {lane} ")), "{text}");
    }
    assert!(text.contains("✓ docs:build "), "{text}");
    assert_eq!(again.status.code(), Some(1), "{again_text}");
    assert!(again_text.contains("✗ check:rust "), "{again_text}");
    assert!(again_text.contains("✗ rust:coverage "), "{again_text}");
    assert!(!again_text.contains("skipped"), "{again_text}");
}
