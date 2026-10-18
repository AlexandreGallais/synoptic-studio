//! `check-all` run on repositories of its own, their scripts plain shell commands, and the lanes
//! read against this repository's `package.json` and test files. Every command starts through
//! `test_support::command`, without the inherited `GIT_*` variables.

#![cfg(unix)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use repo_tools::check_all::{
    BEFORE_LANES, DOCS_LANE, LANES, RUST_LANES, changed_since, command_of, lanes_to_run,
    skipped_lines, touches_rust, touches_site,
};
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
    let folder = repository("config", &[("lint", "git config --local core.planted yes")]);
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
            "lint",
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

/// A workflow of this repository, by its file name in `.github/workflows/`.
fn workflow(name: &str) -> String {
    fs::read_to_string(root().join(".github").join("workflows").join(name)).expect("workflow")
}

/// The jobs of the CI's matrix (`ci.yml`, EN-124): each `- name:` of its `include:` list with the
/// scripts of its `scripts:` line, in their order; the list ends at the first line indented less.
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
        } else if let (Some(scripts), Some(job)) =
            (trimmed.strip_prefix("scripts: "), jobs.last_mut())
        {
            job.1 = scripts.split_whitespace().map(str::to_owned).collect();
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
    let starts_with = |scripts: &[String], lane: &[&str]| {
        scripts.len() >= lane.len()
            && scripts
                .iter()
                .zip(lane)
                .all(|(script, name)| script == name)
    };
    let without_one_job: Vec<&[&str]> = LANES
        .iter()
        .copied()
        .filter(|lane| {
            jobs.iter()
                .filter(|(_, scripts)| starts_with(scripts, lane))
                .count()
                != 1
        })
        .collect();
    let run: Vec<&String> = jobs.iter().flat_map(|(_, scripts)| scripts).collect();
    let twice: Vec<&str> = LANES
        .iter()
        .flat_map(|lane| lane.iter().copied())
        .filter(|name| run.iter().filter(|script| *script == name).count() != 1)
        .collect();

    // One job per lane, starting with the lane's scripts in its order; a lane's script run by no
    // other job; the build after the docs lane, as `check:all` and `npm run build` ran in turn.
    assert_eq!(without_one_job, Vec::<&[&str]>::new());
    assert_eq!(twice, Vec::<&str>::new());
    assert_eq!(jobs.len(), LANES.len(), "{jobs:?}");
    assert!(
        jobs.iter()
            .any(|(_, scripts)| scripts.last().is_some_and(|last| last == "build"))
    );
    // Each job a check of its own name, run to its end whichever other fails (`REF-GH-JOB-PERMISSIONS`).
    assert!(text.contains("\n    name: ${{ matrix.name }}\n"));
    assert!(text.contains("\n      fail-fast: false\n"));
    assert!(text.contains(
        "- run: for script in $SCRIPTS; do npm run \"$script\"; done\n        env:\n          SCRIPTS: ${{ matrix.scripts }}\n"
    ));
}

#[test]
fn f24_ac1_reads_the_jobs_of_a_matrix_to_its_end() {
    let text = "jobs:\n  lanes:\n    name: ${{ matrix.name }}\n    strategy:\n      matrix:\n        \
                include:\n          # A comment.\n          - name: A\n            scripts: a b\n\n          \
                - name: B\n            scripts: c\n    steps:\n      - name: x\n        scripts: d\n  \
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
fn runs_a_script_of_package_json_for_each_check_the_playground_after_the_site() {
    // `script_command` falls back on `npm run <name>`, which npm refuses without the script: a
    // lane naming a script that EN-115 removed would fail only when it runs.
    let scripts = scripts();
    let missing: Vec<&str> = LANES
        .iter()
        .flat_map(|lane| lane.iter().copied())
        .chain([BEFORE_LANES])
        .filter(|name| !scripts.iter().any(|(script, _)| script == name))
        .collect();

    assert_eq!(missing, Vec::<&str>::new());
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

/// Paths given as owned strings, as `check-all` reads them from Git.
fn owned(paths: &[&str]) -> Vec<String> {
    paths.iter().map(|&path| path.to_owned()).collect()
}

/// The root's tracked entries the Rust lanes do not read whole — neither Cargo, rustup nor
/// cargo-deny, a file inside one that the crates' tests read being named among the Rust inputs
/// (`.github/workflows/`): a new entry fails `[F29.AC2]` until it is listed here or there.
const NOT_RUST_INPUTS: &[&str] = &[
    ".claude",
    ".editorconfig",
    ".gitattributes",
    ".githooks",
    ".github",
    ".gitignore",
    ".idea",
    ".lycheeignore",
    ".mcp.json",
    ".npmrc",
    ".nvmrc",
    ".oxlintrc.json",
    ".secret-scan-ignore",
    ".vscode",
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
    // the CI and its required checks, the lint's folders, the hooks' settings, the checklists,
    // and the test files.
    for path in [
        "package.json",
        "package-lock.json",
        ".oxfmtrc.json",
        ".github/workflows/ci.yml",
        "docs/tooling/ci.md",
        "docs/conventions/files.md",
        ".claude/settings.json",
        "playground/checklists/epic-17.ts",
        "src/math/angle/degrees.test.ts",
        "tests/declarations.test.ts",
    ] {
        assert!(touches_rust(&owned(&[path])), "{path}");
    }
    // A story of Markdown, of the library's code or of a script: none of them.
    assert!(!touches_rust(&owned(&[
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
    let scripts = scripts();
    let script = |name: &str| {
        scripts
            .iter()
            .find(|(script, _)| script == name)
            .map_or(String::new(), |(_, command)| command.clone())
    };

    assert_eq!(RUST_LANES, [&["check:rust"][..], &["rust:coverage"][..]]);
    assert!(
        RUST_LANES.iter().all(|lane| LANES.contains(lane)),
        "{LANES:?}"
    );
    // The coverage left out of `check:rust`, which keeps rustfmt, Clippy, the advisories, the
    // check for Windows and the binaries' freshness.
    assert!(!script("check:rust").contains("rust:coverage"));
    assert!(script("check:rust").contains("rust:advisories"));
    assert!(script("rust:coverage").contains(
        "cargo llvm-cov --workspace --locked --quiet --summary-only --fail-under-lines 93 --fail-under-functions 93"
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
fn f29_ac2_skips_the_rust_lanes_for_a_story_that_changed_none_of_their_inputs() {
    let root = scratch("rust-skipped");
    let origin = root.join("origin.git");
    let folder = repository(
        "rust-skipped-work",
        &[("check:rust", "exit 1"), ("rust:coverage", "exit 1")],
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
    git(&folder, &["switch", "-q", "-c", "chore/en-1-docs"]);
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
