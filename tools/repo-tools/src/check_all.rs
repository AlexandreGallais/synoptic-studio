//! What `check:all` decides (RET-002, EN-085, EN-103, ADR-0103): its checks and the command each
//! runs, its lanes of checks — in order within a lane, the lanes at the same time, so that the
//! pre-push hook waits for the slowest lane instead of the sum — the docs lane and the Rust lanes
//! each left out when the branch changed none of their inputs against its base, a check's report
//! line, and the lines of `.git/config` a lane added or removed.
//!
//! The commands live here, not in `package.json` (decided 2026-10-25, ADR-0103): `package.json`
//! keeps what a person or npm runs, and the CI runs each lane by `check-all --lane <name>`.
//!
//! No release binary is written while the lanes run (EN-069's fix): `cargo run` copies its crate's
//! binary into `target/release/` again even when it is up to date, removing it first, so a lane
//! starting that binary at the same moment failed with `could not execute process … No such file
//! or directory`. The release binaries are built once before the lanes ([`BEFORE_LANES`]), and a
//! check calls them by their path.

use std::env;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::Command;

use git_repository::stdout_of;
use stale_binaries::touches;
pub use stale_binaries::touches_rust;

use crate::check_scope::Check;

/// The check run alone before the lanes: the release binaries every lane's tools run from.
pub const BEFORE_LANES: &str = "rust:build";

/// The check run alone after the lanes: the timing tests, whose limits measure a speed that the
/// lanes' load slowed past them (REV-008), the test lane leaving them out: the root's, then the
/// playground's by its `ng test` (EN-175).
pub const AFTER_LANES: &str = "test:timings";

/// The check of the test lane, which a story branch's push narrows to the tests its change can
/// affect ([`related_tests_command`]).
pub const TEST_SCRIPT: &str = "test";

/// The check of the Rust workspace's tests under the coverage floor (EN-130).
pub const RUST_COVERAGE: &str = "rust:coverage";

/// The check of the playground's types and templates, by Angular's compiler (EN-159).
pub const PLAYGROUND_TYPES: &str = "playground:types";

/// The check building the playground's Angular app (EN-159).
pub const PLAYGROUND_APP: &str = "playground:app";

/// The check of the playground's tests under its coverage floor, by its `ng test` (EN-175).
pub const PLAYGROUND_TESTS: &str = "playground:test";

/// The failure of a playground check whose packages are not installed.
macro_rules! playground_missing {
    () => {
        "The packages of the playground are missing: run npm ci --prefix playground"
    };
}

/// A command of Angular's `ng` run from the playground's folder, where `angular.json` is, its
/// packages missing failing aloud.
macro_rules! ng_in_playground {
    ($arguments:literal) => {
        concat!(
            "{ command -v ng > /dev/null || { echo '",
            playground_missing!(),
            "' >&2; exit 1; }; } && cd playground && ng ",
            $arguments
        )
    };
}

/// The playground's tests without their coverage, as a story's push runs them (EN-175).
pub const PLAYGROUND_TESTS_WITHOUT_COVERAGE: &str = ng_in_playground!("test --no-watch");

/// The playground's timing tests alone, by the `timings` configuration of its `ng test`: the
/// files of `angular.json`'s list, their timing tests no longer skipped (EN-175).
pub const PLAYGROUND_TIMINGS: &str = ng_in_playground!("test --no-watch --configuration timings");

/// Each check by its name, with the command `sh -c` runs from the repository's root, the programs
/// npm installs found first ([`search_path`]). The targets of `lint` and `format:check` are those
/// `fix` gives the same tools in `package.json`.
pub const CHECKS: &[(&str, &str)] = &[
    (BEFORE_LANES, "cargo build --release --workspace --locked"),
    (
        "lint",
        concat!(
            "oxlint --deny-warnings --report-unused-disable-directives-severity=error *.ts docs ",
            "playground scripts src tests && target/release/lint *.ts *.md .agents .github docs ",
            "playground scripts src tests"
        ),
    ),
    // The playground's Angular app, an npm project of its own (ADR-0104): its code and templates
    // type-checked by Angular's compiler on the playground's TypeScript, its packages missing
    // failing aloud (EN-159).
    (
        PLAYGROUND_TYPES,
        concat!(
            "{ command -v ngc > /dev/null || { echo '",
            playground_missing!(),
            "' >&2; exit 1; }; } && ngc -p playground/tsconfig.app.json --noEmit"
        ),
    ),
    (TEST_SCRIPT, "vitest run --coverage --tags-filter='!timing'"),
    // The playground's tests by Angular's unit-test builder on happy-dom, under the floor of its
    // `angular.json`, their timing tests skipped by its runner configuration (EN-175).
    (
        PLAYGROUND_TESTS,
        ng_in_playground!("test --no-watch --coverage"),
    ),
    (
        "docs:build",
        "target/release/api-pages src/index.ts docs/api /api/ editor && vitepress build docs",
    ),
    ("playground:build", "vite build --mode playground"),
    // Angular's build of the app into `playground/dist/browser` (EN-159).
    (PLAYGROUND_APP, ng_in_playground!("build")),
    (
        "format:check",
        concat!(
            "target/release/format --check \"*\" .cargo .agents .claude/settings.json .githooks ",
            ".github docs playground scripts src tests tools"
        ),
    ),
    ("lint:secrets", "target/release/secret-scan"),
    // Both projects' lockfiles, the playground's its own (ADR-0104).
    (
        "deps:audit",
        "npm audit --audit-level=low && npm --prefix playground audit --audit-level=low",
    ),
    ("deps:unused", "target/release/dependencies unused"),
    ("deps:outdated", "target/release/dependencies outdated"),
    // rustfmt, Clippy, the RustSec advisories read offline, the check for Windows without linking
    // and the binaries' freshness (EN-068, EN-069, EN-070); a missing cargo-deny or database fails
    // aloud.
    (
        "check:rust",
        concat!(
            "cargo fmt --all --check && cargo clippy --workspace --all-targets --locked --quiet && ",
            "{ cargo deny --version > /dev/null 2>&1 || { echo 'cargo-deny is missing: run cargo ",
            "install --locked cargo-deny@0.20.2, then cargo deny fetch db' >&2; exit 1; }; ",
            "cargo deny --offline check advisories || ",
            "{ echo 'A Rust crate has a RustSec advisory, or the advisory database is missing: ",
            "cargo deny fetch db fetches it' >&2; exit 1; }; } && ",
            "cargo check --workspace --all-targets --locked --quiet --target ",
            "x86_64-pc-windows-msvc && target/release/stale-binaries"
        ),
    ),
    (
        RUST_COVERAGE,
        concat!(
            "cargo llvm-cov --version > /dev/null 2>&1 || { echo 'cargo-llvm-cov is missing: ",
            "rustup installs llvm-tools from rust-toolchain.toml, then run cargo install --locked ",
            "cargo-llvm-cov@0.9.1 and run the check again' >&2; exit 1; }; cargo llvm-cov ",
            "--workspace --locked --quiet --summary-only --fail-under-lines 95 ",
            "--fail-under-functions 95 || { echo 'Rust tests failed, or their coverage fell under ",
            "the floor of 95 % of lines and 95 % of functions' >&2; ",
            "exit 1; }"
        ),
    ),
    (
        AFTER_LANES,
        concat!(
            "vitest run --tags-filter=timing ",
            "src/io/font-face/parse-font-face.test.ts src/math/sequence/scan.test.ts ",
            "src/model/text-wrap/without-trailing-spaces.test.ts ",
            "src/model/text-wrap/wrapped-paragraph.test.ts tests/history-timings.test.ts && ",
            ng_in_playground!("test --no-watch --configuration timings")
        ),
    ),
];

/// The lane of the linters and TypeScript's own errors, the playground's by Angular's compiler.
const LINT_LANE: &[&str] = &["lint", PLAYGROUND_TYPES];

/// The lane of the tests: the root's, then the playground's (EN-175).
const TEST_LANE: &[&str] = &[TEST_SCRIPT, PLAYGROUND_TESTS];

/// The lane of the documentation site and the playground built into it, then the playground's
/// Angular app built by Angular.
pub const DOCS_LANE: &[&str] = &["docs:build", "playground:build", PLAYGROUND_APP];

/// The lane of the formatter, the secret scan and the dependencies.
const FILES_LANE: &[&str] = &[
    "format:check",
    "lint:secrets",
    "deps:audit",
    "deps:unused",
    "deps:outdated",
];

/// The lane of the Rust workspace's checks: rustfmt, Clippy, its advisories, the check for Windows
/// and the binaries' freshness (EN-068, EN-069).
const RUST_CHECKS_LANE: &[&str] = &["check:rust"];

/// The lane of the Rust workspace's tests under the coverage floor, beside its checks (EN-130).
const RUST_COVERAGE_LANE: &[&str] = &[RUST_COVERAGE];

/// The lanes of the Rust workspace, left out when the branch changed none of their inputs.
pub const RUST_LANES: &[&[&str]] = &[RUST_CHECKS_LANE, RUST_COVERAGE_LANE];

/// Lanes of checks: in order within a lane, the lanes at the same time.
pub const LANES: &[&[&str]] = &[
    LINT_LANE,
    TEST_LANE,
    DOCS_LANE,
    FILES_LANE,
    RUST_CHECKS_LANE,
    RUST_COVERAGE_LANE,
];

/// Each lane by the name `check-all --lane` takes, the CI running one per job, and the timing
/// tests after the tests in theirs (EN-124, REV-008).
pub const NAMED_LANES: &[(&str, &[&str])] = &[
    ("lint", LINT_LANE),
    ("test", TEST_LANE),
    ("docs", DOCS_LANE),
    ("files", FILES_LANE),
    ("rust", RUST_CHECKS_LANE),
    ("rust-coverage", RUST_COVERAGE_LANE),
    ("timings", &[AFTER_LANES]),
];

/// The checks of a lane, by the name `check-all --lane` takes; `None` for a name no lane has.
#[must_use]
pub fn lane_named(name: &str) -> Option<&'static [&'static str]> {
    NAMED_LANES
        .iter()
        .find(|(lane, _)| *lane == name)
        .map(|(_, checks)| *checks)
}

/// A check by its name, with its command from [`CHECKS`]; a name without one gets a command that
/// fails saying so, so that a lane naming a check that has no command fails at its run.
#[must_use]
pub fn check_named(name: &str) -> Check {
    let command = CHECKS.iter().find(|(check, _)| *check == name).map_or_else(
        || format!("echo 'check-all: no command for the check {name}' >&2; exit 1"),
        |(_, command)| (*command).to_owned(),
    );

    Check {
        name: name.to_owned(),
        command,
    }
}

/// The lines `check-all --list` prints: one per check of each named lane, its lane, its name and
/// its command separated by tabs, in the lanes' order.
#[must_use]
pub fn listed_lines() -> Vec<String> {
    NAMED_LANES
        .iter()
        .flat_map(|(lane, checks)| {
            checks
                .iter()
                .map(move |name| format!("{lane}\t{name}\t{}", check_named(name).command))
        })
        .collect()
}

/// The folders searched for a check's programs: the repository's `node_modules/.bin`, where npm
/// installs oxlint, Vitest, Vite and `VitePress`, the playground's, where it installs Angular's
/// `ngc` and `ng` (ADR-0104), then those of `PATH`, so that a check finds them whether npm started
/// `check-all` or the CI called it by its path.
#[must_use]
pub fn search_path(root: &Path, path: Option<&OsString>) -> OsString {
    let mut folders = vec![
        root.join("node_modules").join(".bin"),
        root.join("playground").join("node_modules").join(".bin"),
    ];

    folders.extend(path.map(env::split_paths).into_iter().flatten());
    env::join_paths(folders).unwrap_or_else(|_| path.cloned().unwrap_or_default())
}

/// What every test reads beside its imports, so that a change to one runs every test: Vitest's
/// configuration, the packages, the TypeScript settings, and the global setup with the font file
/// it reads for every test file (EN-089).
const WHOLE_TESTS_INPUTS: &[&str] = &[
    "vite.config.ts",
    "package.json",
    "package-lock.json",
    "tsconfig.json",
    "tests/font-files-setup.ts",
    "tests/inter-regular.ttf",
];

/// What the docs lane reads: the site, the library its API pages come from, the playground built
/// into it, the API pages' generator and the Rust toolchain and crates it is built with (an Oxc
/// bump changes its pages), and the build's own configuration and packages.
const DOCS_INPUTS: &[&str] = &[
    "docs/",
    "src/",
    "playground/",
    "tools/api-pages/",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "package.json",
    "package-lock.json",
    "vite.config.ts",
];

/// Whether changed files touch an input of the docs lane: a file in one of its folders, or one of
/// its files.
#[must_use]
pub fn touches_site(paths: &[String]) -> bool {
    touches(paths, DOCS_INPUTS)
}

/// The groups of lanes a known change leaves out, each skipped whole: the docs lane when it
/// touches none of the site's inputs, then the Rust lanes when it touches none of theirs.
fn skipped_groups(paths: &[String]) -> Vec<&'static [&'static [&'static str]]> {
    let mut groups: Vec<&'static [&'static [&'static str]]> = Vec::new();

    if !touches_site(paths) {
        groups.push(&[DOCS_LANE]);
    }
    if !touches_rust(paths) {
        groups.push(RUST_LANES);
    }
    groups
}

/// The lanes to run: every lane, the docs lane and the Rust lanes left out when the change is
/// known (`Some`) and touches none of their inputs.
#[must_use]
pub fn lanes_to_run(paths: Option<&[String]>) -> Vec<&'static [&'static str]> {
    let skipped = paths.map(skipped_groups).unwrap_or_default();

    LANES
        .iter()
        .copied()
        .filter(|lane| !skipped.iter().any(|group| group.contains(lane)))
        .collect()
}

/// A path as one word of `sh`: in single quotes, each quote inside closed, escaped and opened
/// again.
#[must_use]
pub fn quoted(path: &str) -> String {
    format!("'{}'", path.replace('\'', r"'\''"))
}

/// The command of the test lane on a story branch's push (decided 2026-10-24, REV-008): Vitest's
/// `related` (`REF-VITEST-RELATED`) on the files changed against the base, through the static
/// imports of the module graph, and on the repository's own tests given (`tests/`, `scripts/`),
/// which read files through the file system rather than imports, so that a change of the docs
/// they check runs them; the timing tests left out, as `npm run test` does, and no coverage: a
/// partial run cannot hold the floor of 100 %, which the full run keeps for a feature branch,
/// `develop` and the CI. `None`, the whole `test` script, off a story branch, for an unknown
/// change, for a change to an input every test reads ([`WHOLE_TESTS_INPUTS`]), or with no file
/// to give.
#[must_use]
pub fn related_tests_command(
    is_story: bool,
    paths: Option<&[String]>,
    repository_tests: &[String],
) -> Option<String> {
    let paths = paths.filter(|paths| is_story && !touches(paths, WHOLE_TESTS_INPUTS))?;
    let mut files: Vec<&str> = paths.iter().map(String::as_str).collect();

    for test in repository_tests {
        if !files.contains(&test.as_str()) {
            files.push(test);
        }
    }

    // `vitest related` without a file: what it runs is not said, so every test runs.
    if files.is_empty() {
        return None;
    }

    let words: Vec<String> = files.iter().map(|path| quoted(path)).collect();

    Some(format!(
        "vitest related --run --passWithNoTests --tags-filter='!timing' {}",
        words.join(" ")
    ))
}

/// The line saying the test lane runs only the tests related to the change.
#[must_use]
pub fn related_tests_line(base: &str) -> String {
    format!(
        "- test: only the tests related to the change against {base}, without coverage; the full \
         run on a feature branch, develop and the CI"
    )
}

/// Whether a branch is a story's, `<type>/<id>-<topic>` (ADR-0072): its push may run only the
/// tests its change affects; a feature branch, `develop`, `main` or a detached head run them all.
#[must_use]
pub fn is_story_branch(branch: &str) -> bool {
    git_repository::story_of_branch(branch).is_some()
}

/// The current branch, `HEAD` when it is detached.
///
/// # Errors
/// When Git fails.
pub fn current_branch(root: &Path) -> io::Result<String> {
    Ok(git_lines(root, &["rev-parse", "--abbrev-ref", "HEAD"])?
        .into_iter()
        .next()
        .unwrap_or_else(|| "HEAD".to_owned()))
}

/// The repository's own tests, which read its files beside their imports: every test file of
/// `tests/` and `scripts/` Git tracks.
///
/// # Errors
/// When Git fails.
pub fn repository_tests(root: &Path) -> io::Result<Vec<String>> {
    git_lines(root, &["ls-files", "tests/*.test.ts", "scripts/*.test.ts"])
}

/// The remote branches a branch may have started from (ADR-0072): `origin/develop` and the feature
/// branches for a story, `origin/develop` for a feature, none for `main`, `develop` or a detached
/// head (`HEAD`); in the order given.
#[must_use]
pub fn base_candidates(branch: &str, remotes: &[String]) -> Vec<String> {
    if ["HEAD", "develop", "main"].contains(&branch) {
        return Vec::new();
    }

    let is_feature = branch.starts_with("feature/");

    remotes
        .iter()
        .filter(|remote| {
            *remote == "origin/develop" || (!is_feature && remote.starts_with("origin/feature/"))
        })
        .cloned()
        .collect()
}

/// Of a branch's candidate bases, each with the branch's commits ahead of it, the nearest: the
/// fewest commits ahead, the first given on a tie.
#[must_use]
pub fn nearest_base(candidates: &[(u64, String)]) -> Option<String> {
    candidates
        .iter()
        .enumerate()
        .min_by_key(|(rank, (ahead, _))| (*ahead, *rank))
        .map(|(_, (_, remote))| remote.clone())
}

/// Seconds with one decimal, rounded half up from whole milliseconds: `23.9` for 23,940 ms.
#[must_use]
pub fn seconds(milliseconds: u128) -> String {
    let tenths = (milliseconds + 50) / 100;

    format!("{}.{}", tenths / 10, tenths % 10)
}

/// The line reporting a check, its output after it when it failed: `✓ lint 23.9 s`, or
/// `✗ test 26.2 s` and its output on the next lines.
#[must_use]
pub fn report_of(name: &str, is_ok: bool, milliseconds: u128, output: &str) -> String {
    if is_ok {
        format!("✓ {name} {} s", seconds(milliseconds))
    } else {
        format!("✗ {name} {} s\n{output}", seconds(milliseconds))
    }
}

/// The lines skipping the lanes a known change leaves out, the docs lane's first, then the Rust
/// lanes', against the base the change was read from.
#[must_use]
pub fn skipped_lines(paths: &[String], base: &str) -> Vec<String> {
    skipped_groups(paths)
        .iter()
        .map(|group| {
            format!(
                "- {} skipped: nothing changed in their inputs against {base}",
                group.concat().join(", ")
            )
        })
        .collect()
}

/// The last line of `check:all`: passed, or failed naming the failed checks.
#[must_use]
pub fn summary(milliseconds: u128, failed: &[String]) -> String {
    if failed.is_empty() {
        format!("check:all passed in {} s", seconds(milliseconds))
    } else {
        format!(
            "check:all failed in {} s: {}",
            seconds(milliseconds),
            failed.join(", ")
        )
    }
}

/// The lines of a text removed (`- `) and added (`+ `) in another, in the order of a longest
/// common subsequence of lines: the change a lane made to `.git/config`.
#[must_use]
pub fn changed_lines(before: &str, after: &str) -> Vec<String> {
    let old: Vec<&str> = before.lines().collect();
    let new: Vec<&str> = after.lines().collect();
    // common[i][j]: the longest common subsequence of old[i..] and new[j..].
    let mut common = vec![vec![0_usize; new.len() + 1]; old.len() + 1];

    for i in (0..old.len()).rev() {
        for j in (0..new.len()).rev() {
            common[i][j] = if old[i] == new[j] {
                common[i + 1][j + 1] + 1
            } else {
                common[i + 1][j].max(common[i][j + 1])
            };
        }
    }

    let (mut i, mut j, mut lines) = (0, 0, Vec::new());

    while i < old.len() || j < new.len() {
        if i < old.len() && j < new.len() && old[i] == new[j] {
            i += 1;
            j += 1;
        } else if j == new.len() || (i < old.len() && common[i + 1][j] >= common[i][j + 1]) {
            lines.push(format!("- {}", old[i]));
            i += 1;
        } else {
            lines.push(format!("+ {}", new[j]));
            j += 1;
        }
    }

    lines
}

/// The sections of `.git/config` a test could break, every key of them guarded, their names in
/// lower case: a `core.bare` or a `core.hooksPath` turns the checkout into another, a `user` or
/// `commit` key signs its commits as another (EN-103, EN-113); a `url.<base>.insteadOf` sends its
/// pushes elsewhere, an `include.path` or `includeIf.<condition>.path` reads another file, an
/// `extensions` key changes how the repository is read (AUD-023).
const GUARDED_SECTIONS: [&str; 7] = [
    "core",
    "user",
    "commit",
    "url",
    "include",
    "includeif",
    "extensions",
];

/// The keys of a `[remote "…"]` section that are guarded: where the checkout pushes and fetches.
const GUARDED_REMOTE_KEYS: [&str; 2] = ["url", "pushurl"];

/// The lines of a `.git/config` a test could break, with their endings, each under its section's
/// header: every key of `[core]`, `[user]` and `[commit]`, and the URLs of each remote. The rest
/// is left out, the `[branch "…"]` tracking lines that a parallel worktree's `git push -u`
/// writes first (a false failure at CHK-022). Section and key names are read without case, as
/// Git reads them, and the rest of a header's line sets a key too (`[core] bare = true`,
/// `REF-GIT-HOOKS-PATH`, Syntax).
#[must_use]
pub fn guarded_lines(text: &str) -> String {
    let (mut kept, mut header, mut section) = (String::new(), "", String::new());

    for line in text.split_inclusive('\n') {
        let trimmed = line.trim();
        let (setting, on_header) = if let Some(inside) = trimmed.strip_prefix('[') {
            // A `]` inside the subsection's quotes closes nothing.
            let quotes_end = inside.rfind('"').map_or(0, |at| at + 1);
            let close = inside[quotes_end..]
                .find(']')
                .map_or(inside.len(), |at| quotes_end + at + 1);

            header = line;
            section = inside
                .split(|letter: char| letter.is_whitespace() || matches!(letter, '"' | '.' | ']'))
                .next()
                .unwrap_or_default()
                .to_ascii_lowercase();
            (inside[close..].trim(), true)
        } else {
            (trimmed, false)
        };
        let key = setting
            .split(|letter: char| letter.is_whitespace() || letter == '=')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let comment = setting.starts_with('#') || setting.starts_with(';');
        let guarded = GUARDED_SECTIONS.contains(&section.as_str())
            || (section == "remote" && GUARDED_REMOTE_KEYS.contains(&key.as_str()));

        if guarded && !key.is_empty() && !comment {
            kept.push_str(header);
            if !on_header {
                kept.push_str(line);
            }
            header = "";
        }
    }

    kept
}

/// The report of `.git/config` read before and after the lanes (`None` when it could not be
/// read): nothing when its [`guarded_lines`] are the same, else the failure naming the guarded
/// lines added or removed. Nothing is restored: what changed it is a bug to find, and the old
/// content is in the report.
#[must_use]
pub fn config_report(path: &str, before: Option<&[u8]>, after: Option<&[u8]>) -> Option<String> {
    let guarded =
        |bytes: Option<&[u8]>| bytes.map(|bytes| guarded_lines(&String::from_utf8_lossy(bytes)));
    let (old, new) = (guarded(before), guarded(after));

    if old == new {
        return None;
    }

    let mut lines = changed_lines(
        old.as_deref().unwrap_or_default(),
        new.as_deref().unwrap_or_default(),
    );
    let what = match (before, after) {
        (Some(_), None) => "removed",
        (None, Some(_)) => "created",
        _ => "changed",
    };

    if lines.is_empty() {
        lines.push("  the same lines, their endings or bytes changed".to_owned());
    }
    lines.insert(
        0,
        format!("✗ {path} {what} while the checks ran, restored nothing:"),
    );
    Some(lines.join("\n"))
}

/// Runs Git in the repository, without the `GIT_*` variables a hook sets (`GIT_DIR` would point it
/// at another repository); its output's lines, empty ones left out.
///
/// # Errors
/// When Git cannot start or fails, with its message.
pub fn git_lines(root: &Path, arguments: &[&str]) -> io::Result<Vec<String>> {
    let mut command = Command::new("git");

    for (name, _) in env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }

    Ok(stdout_of(command.args(arguments).current_dir(root))
        .map_err(io::Error::other)?
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

/// The files changed in a repository since a commit, committed or not: a file renamed or moved
/// counts under its old path and its new one (`--no-renames`), so that moving a page out of
/// `docs/` still builds the site.
///
/// # Errors
/// When Git fails.
pub fn changed_since(root: &Path, fork: &str) -> io::Result<Vec<String>> {
    let mut paths = git_lines(root, &["diff", "--name-only", "--no-renames", fork])?;

    paths.extend(git_lines(
        root,
        &["ls-files", "--others", "--exclude-standard"],
    )?);
    Ok(paths)
}

/// The branch's base, the nearest of its candidates, and the files it changed against it; `None`
/// without a base.
///
/// # Errors
/// When Git fails.
pub fn change_against_base(root: &Path) -> io::Result<Option<(String, Vec<String>)>> {
    let branch = current_branch(root)?;
    let remotes = git_lines(
        root,
        &[
            "for-each-ref",
            "--format=%(refname:short)",
            "refs/remotes/origin/",
        ],
    )?;
    let mut candidates = Vec::new();

    for remote in base_candidates(&branch, &remotes) {
        let ahead = git_lines(root, &["rev-list", "--count", &format!("{remote}..HEAD")])?;

        candidates.push((
            ahead
                .first()
                .and_then(|count| count.parse().ok())
                .unwrap_or(0),
            remote,
        ));
    }

    let Some(base) = nearest_base(&candidates) else {
        return Ok(None);
    };
    let fork = git_lines(root, &["merge-base", &base, "HEAD"])?
        .into_iter()
        .next()
        .unwrap_or_else(|| base.clone());

    Ok(Some((base.clone(), changed_since(root, &fork)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|&path| path.to_owned()).collect()
    }

    #[test]
    fn f19_ac9_runs_the_rust_checks_in_a_lane_of_their_own() {
        assert!(LANES.contains(&&["check:rust"][..]));
    }

    #[test]
    fn f19_ac9_builds_the_release_binaries_once_before_the_lanes() {
        assert_eq!(BEFORE_LANES, "rust:build");
        assert!(!LANES.iter().any(|lane| lane.contains(&BEFORE_LANES)));
    }

    #[test]
    fn runs_on_a_story_branch_only_the_tests_related_to_its_change() {
        let change = owned(&["src/math/sequence/scan.ts", "docs/domain/x.md"]);
        let ours = owned(&["tests/backlog.test.ts", "scripts/x.test.ts"]);

        // The changed files, then the repository's own tests, each quoted.
        assert_eq!(
            related_tests_command(true, Some(&change), &ours).as_deref(),
            Some(
                "vitest related --run --passWithNoTests --tags-filter='!timing' \
                 'src/math/sequence/scan.ts' 'docs/domain/x.md' 'tests/backlog.test.ts' \
                 'scripts/x.test.ts'"
            )
        );
        // A repository test changed is named once; a quote in a path stays one word.
        assert_eq!(
            related_tests_command(
                true,
                Some(&owned(&["tests/backlog.test.ts", "a'b.ts"])),
                &ours
            )
            .as_deref(),
            Some(
                "vitest related --run --passWithNoTests --tags-filter='!timing' \
                 'tests/backlog.test.ts' 'a'\\''b.ts' 'scripts/x.test.ts'"
            )
        );
        // Nothing changed: the repository's tests alone; no file at all, every test.
        assert_eq!(
            related_tests_command(true, Some(&[]), &ours).as_deref(),
            Some(
                "vitest related --run --passWithNoTests --tags-filter='!timing' \
                 'tests/backlog.test.ts' 'scripts/x.test.ts'"
            )
        );
        assert_eq!(related_tests_command(true, Some(&[]), &[]), None);
    }

    #[test]
    fn runs_every_test_off_a_story_branch_or_for_an_input_of_every_test() {
        let change = owned(&["src/math/sequence/scan.ts"]);

        // A feature branch, develop, main; an unknown change (the CI, no base).
        assert_eq!(related_tests_command(false, Some(&change), &[]), None);
        assert_eq!(related_tests_command(true, None, &[]), None);
        for input in [
            "vite.config.ts",
            "package.json",
            "package-lock.json",
            "tsconfig.json",
            "tests/font-files-setup.ts",
            "tests/inter-regular.ttf",
        ] {
            let both = owned(&["src/math/sequence/scan.ts", input]);

            assert_eq!(
                related_tests_command(true, Some(&both), &[]),
                None,
                "{input}"
            );
        }
        assert_eq!(
            related_tests_line("origin/feature/f47-x"),
            "- test: only the tests related to the change against origin/feature/f47-x, without \
             coverage; the full run on a feature branch, develop and the CI"
        );
    }

    #[test]
    fn reads_a_story_branch_by_its_name() {
        assert!(is_story_branch("build/rev-008-pre-push-by-change"));
        assert!(is_story_branch("fix/us-091-space"));
        for other in [
            "feature/f47-space",
            "develop",
            "main",
            "HEAD",
            "dependabot/npm/x",
        ] {
            assert!(!is_story_branch(other), "{other}");
        }
    }

    #[test]
    fn runs_the_timing_tests_alone_after_the_lanes() {
        assert_eq!(AFTER_LANES, "test:timings");
        assert!(!LANES.iter().any(|lane| lane.contains(&AFTER_LANES)));
    }

    #[test]
    fn aud052_gives_every_check_of_a_lane_and_around_them_a_command_of_its_own() {
        let mut names: Vec<&str> = LANES.iter().flat_map(|lane| lane.iter().copied()).collect();

        names.extend([BEFORE_LANES, AFTER_LANES]);
        // Each name has exactly one command, and every command a check runs.
        for name in &names {
            assert_eq!(
                CHECKS.iter().filter(|(check, _)| check == name).count(),
                1,
                "{name}"
            );
        }
        assert_eq!(CHECKS.len(), names.len());
        // A check runs a release binary by its path, never through `cargo run`, which rewrites it
        // (EN-069), and takes no heavy lock, `check:all` holding it.
        for (name, command) in CHECKS {
            assert!(!command.contains("cargo run"), "{name}");
            assert!(!command.contains("heavy-lock"), "{name}");
            assert!(!command.contains("npm run"), "{name}");
        }
        assert_eq!(
            check_named(BEFORE_LANES).command,
            "cargo build --release --workspace --locked"
        );
        assert_eq!(
            check_named("odd").command,
            "echo 'check-all: no command for the check odd' >&2; exit 1"
        );
    }

    #[test]
    fn aud052_names_each_lane_once_for_the_ci_the_timing_tests_after_the_tests() {
        let named: Vec<&[&str]> = NAMED_LANES.iter().map(|(_, checks)| *checks).collect();

        assert_eq!(named[..LANES.len()], *LANES);
        assert_eq!(lane_named("timings"), Some(&[AFTER_LANES][..]));
        assert_eq!(lane_named("docs"), Some(DOCS_LANE));
        assert_eq!(lane_named("build"), None);
        // One line per check: its lane, its name and its command.
        let lines = listed_lines();

        assert_eq!(lines.len(), CHECKS.len() - 1);
        assert_eq!(
            lines[4],
            "docs\tdocs:build\ttarget/release/api-pages src/index.ts docs/api /api/ editor && \
             vitepress build docs"
        );
    }

    #[test]
    fn f50_ac3_runs_the_playground_s_tests_under_their_floor_and_their_timings_after_the_root_s() {
        assert_eq!(
            lane_named("test"),
            Some(&[TEST_SCRIPT, PLAYGROUND_TESTS][..])
        );
        // Its floor is `angular.json`'s, asked by `--coverage`; no watch outside a terminal.
        assert!(
            check_named(PLAYGROUND_TESTS)
                .command
                .ends_with("&& cd playground && ng test --no-watch --coverage")
        );
        // The root's timing tests, then the playground's alone by their configuration.
        let timings = check_named(AFTER_LANES).command;

        assert!(timings.starts_with("vitest run --tags-filter=timing "));
        assert!(timings.ends_with(&format!(
            "tests/history-timings.test.ts && {PLAYGROUND_TIMINGS}"
        )));
        for command in [PLAYGROUND_TESTS_WITHOUT_COVERAGE, PLAYGROUND_TIMINGS] {
            assert!(
                command.starts_with(
                    "{ command -v ng > /dev/null || { echo 'The packages of the playground are \
                     missing: run npm ci --prefix playground' >&2; exit 1; }; } && cd playground"
                ),
                "{command}"
            );
        }
    }

    #[test]
    fn f50_ac3_checks_the_playground_by_angular_s_compiler_and_builds_it() {
        assert_eq!(lane_named("lint"), Some(&["lint", PLAYGROUND_TYPES][..]));
        assert_eq!(
            lane_named("docs"),
            Some(&["docs:build", "playground:build", PLAYGROUND_APP][..])
        );
        // Its own tsconfig, no file written; the build from its folder, where `angular.json` is.
        assert!(
            check_named(PLAYGROUND_TYPES)
                .command
                .ends_with("&& ngc -p playground/tsconfig.app.json --noEmit")
        );
        assert!(
            check_named(PLAYGROUND_APP)
                .command
                .ends_with("&& cd playground && ng build")
        );
        // Its packages missing, each fails saying how to install them.
        for (name, program) in [(PLAYGROUND_TYPES, "ngc"), (PLAYGROUND_APP, "ng")] {
            assert!(
                check_named(name).command.starts_with(&format!(
                    "{{ command -v {program} > /dev/null || {{ echo 'The packages of the \
                     playground are missing: run npm ci --prefix playground' >&2; exit 1; }}; }}"
                )),
                "{name}"
            );
        }
    }

    #[test]
    fn aud052_finds_the_programs_npm_installs_first() {
        let root = Path::new("/repo");
        let path = OsString::from("/usr/bin:/bin");

        assert_eq!(
            search_path(root, Some(&path)),
            OsString::from(
                "/repo/node_modules/.bin:/repo/playground/node_modules/.bin:/usr/bin:/bin"
            )
        );
        assert_eq!(
            search_path(root, None),
            OsString::from("/repo/node_modules/.bin:/repo/playground/node_modules/.bin")
        );
    }

    #[test]
    fn reports_a_check_by_its_time_a_failed_one_with_its_output() {
        // 23,940 ms: 239.4 tenths, rounded to 239, so 23.9 s.
        assert_eq!(report_of("lint", true, 23_940, "x"), "✓ lint 23.9 s");
        assert_eq!(report_of("test", false, 1000, "boom"), "✗ test 1.0 s\nboom");
        // Half a tenth rounds up: 1,250 ms is 1.3 s; 49 ms is 0.0 s.
        assert_eq!(seconds(1250), "1.3");
        assert_eq!(seconds(49), "0.0");
        assert_eq!(seconds(113_051), "113.1");
    }

    #[test]
    fn says_how_it_ended() {
        assert_eq!(summary(80_000, &[]), "check:all passed in 80.0 s");
        assert_eq!(
            summary(1_000, &owned(&["lint", "test"])),
            "check:all failed in 1.0 s: lint, test"
        );
        assert_eq!(
            skipped_lines(&owned(&["tools/lint/src/lib.rs"]), "origin/develop"),
            [
                "- docs:build, playground:build, playground:app skipped: nothing changed in their inputs against origin/develop"
            ]
        );
    }

    #[test]
    fn f19_ac8_builds_the_docs_for_a_change_in_docs_src_the_playground_or_the_build_inputs() {
        assert!(touches_site(&owned(&["docs/adr/README.md"])));
        assert!(touches_site(&owned(&[
            "scripts/x.ts",
            "src/math/angle/degrees.ts"
        ])));
        assert!(touches_site(&owned(&["playground/main.ts"])));
        assert!(touches_site(&owned(&["tools/api-pages/src/main.rs"])));
        assert!(touches_site(&owned(&["package-lock.json"])));
        // The playground's packages, a project of its own built into the site (ADR-0104).
        assert!(touches_site(&owned(&["playground/package-lock.json"])));
        assert!(touches_site(&owned(&["vite.config.ts"])));
        assert!(touches_site(&owned(&["Cargo.lock"])));
        assert!(touches_site(&owned(&["Cargo.toml"])));
        assert!(touches_site(&owned(&["rust-toolchain.toml"])));
        // A code story of the tools: none of the site's inputs.
        assert!(!touches_site(&owned(&[
            "tools/repo-tools/src/lib.rs",
            "tools/lint/src/lib.rs",
            "tests/x.test.ts"
        ])));
        // A name that only starts like a folder or a file of the site is not in it.
        assert!(!touches_site(&owned(&["docsy.md", "srcmap.ts"])));
        assert!(!touches_site(&owned(&["package.json.bak"])));
        assert!(!touches_site(&[]));
    }

    #[test]
    fn f19_ac8_skips_the_docs_lane_only_when_the_change_is_known_and_touches_none_of_its_inputs() {
        let tools = owned(&["tools/lint/src/lib.rs"]);
        let both = owned(&["docs/index.md", "tools/lint/src/lib.rs"]);

        assert!(!lanes_to_run(Some(&tools)).contains(&DOCS_LANE));
        assert_eq!(lanes_to_run(Some(&tools)).len(), LANES.len() - 1);
        assert_eq!(lanes_to_run(Some(&both)), LANES);
        // No base found (main, develop, the CI): every lane.
        assert_eq!(lanes_to_run(None), LANES);
    }

    #[test]
    fn f19_ac8_compares_a_story_with_its_feature_or_develop_a_feature_with_develop() {
        let remotes = owned(&[
            "origin/develop",
            "origin/feature/f19-x",
            "origin/feature/f21-y",
            "origin/main",
        ]);

        assert_eq!(
            base_candidates("chore/en-085-x", &remotes),
            owned(&[
                "origin/develop",
                "origin/feature/f19-x",
                "origin/feature/f21-y"
            ])
        );
        assert_eq!(
            base_candidates("feature/f19-x", &remotes),
            owned(&["origin/develop"])
        );
        assert_eq!(base_candidates("develop", &remotes), Vec::<String>::new());
        assert_eq!(base_candidates("main", &remotes), Vec::<String>::new());
        assert_eq!(base_candidates("HEAD", &remotes), Vec::<String>::new());
    }

    #[test]
    fn f19_ac8_takes_the_base_with_the_fewest_commits_ahead_the_first_on_a_tie() {
        // A story of F19: 5 commits ahead of develop, 2 ahead of its feature branch.
        assert_eq!(
            nearest_base(&[
                (5, "origin/develop".to_owned()),
                (2, "origin/feature/f19-x".to_owned())
            ]),
            Some("origin/feature/f19-x".to_owned())
        );
        assert_eq!(
            nearest_base(&[
                (3, "origin/develop".to_owned()),
                (3, "origin/feature/f19-x".to_owned())
            ]),
            Some("origin/develop".to_owned())
        );
        assert_eq!(nearest_base(&[]), None);
    }

    #[test]
    fn names_the_lines_a_lane_added_or_removed() {
        let before = "[core]\n\tbare = false\n[user]\n\tname = me\n";
        let after = "[core]\n\tbare = true\n[user]\n\tname = me\n\temail = t@t\n";

        assert_eq!(
            changed_lines(before, after),
            ["- \tbare = false", "+ \tbare = true", "+ \temail = t@t"]
        );
        assert_eq!(changed_lines(before, before), Vec::<String>::new());
        assert_eq!(changed_lines("", "a\n"), ["+ a"]);
        assert_eq!(changed_lines("a\n", ""), ["- a"]);
    }

    #[test]
    fn f21_ac9_fails_on_a_config_changed_and_says_nothing_otherwise() {
        let before = b"[core]\n\tbare = false\n".as_slice();
        let after = b"[core]\n\tbare = true\n".as_slice();

        assert_eq!(
            config_report(".git/config", Some(before), Some(before)),
            None
        );
        assert_eq!(config_report(".git/config", None, None), None);
        assert_eq!(
            config_report(".git/config", Some(before), Some(after)).as_deref(),
            Some(
                "✗ .git/config changed while the checks ran, restored nothing:\n- \tbare = false\n+ \tbare = true"
            )
        );
        assert_eq!(
            config_report("c", Some(before), None).as_deref(),
            Some("✗ c removed while the checks ran, restored nothing:\n- [core]\n- \tbare = false")
        );
        assert_eq!(
            config_report("c", None, Some(b"[user]\n\tname = t\n")).as_deref(),
            Some("✗ c created while the checks ran, restored nothing:\n+ [user]\n+ \tname = t")
        );
        // Bytes differ, lines the same: a line ending changed.
        assert_eq!(
            config_report("c", Some(b"[core]\nbare\n"), Some(b"[core]\r\nbare\r\n")).as_deref(),
            Some(
                "✗ c changed while the checks ran, restored nothing:\n  the same lines, their endings or bytes changed"
            )
        );
    }

    /// A main checkout's `.git/config`: the four guarded sections and a branch tracked.
    const CONFIG: &str = "[core]\n\tbare = false\n\thooksPath = .githooks\n[user]\n\tname = t\n\
                          [commit]\n\tgpgsign = false\n[remote \"origin\"]\n\turl = https://h/r.git\n\
                          \tfetch = +refs/heads/*:refs/remotes/origin/*\n[branch \"develop\"]\n\
                          \tremote = origin\n\tmerge = refs/heads/develop\n";

    #[test]
    fn keeps_core_user_commit_and_the_remotes_urls_only() {
        // The fetch refspec and the branch's tracking lines are left out, the remote's header
        // kept above its URL.
        assert_eq!(
            guarded_lines(CONFIG),
            "[core]\n\tbare = false\n\thooksPath = .githooks\n[user]\n\tname = t\n[commit]\n\
             \tgpgsign = false\n[remote \"origin\"]\n\turl = https://h/r.git\n"
        );
        // Names without case, as Git reads them; comments and blank lines left out; a section
        // without a guarded key leaves no header.
        assert_eq!(
            guarded_lines(
                "[CORE]\n# a comment\n\n\tBare = true\n[Remote \"o\"]\n\tPushURL = u\n[gc]\n\tauto = 0\n[remote \"p\"]\n\tfetch = f\n"
            ),
            "[CORE]\n\tBare = true\n[Remote \"o\"]\n\tPushURL = u\n"
        );
        assert_eq!(guarded_lines(""), "");
    }

    #[test]
    fn f21_ac9_guards_the_urls_rewritten_the_files_included_and_a_key_on_its_header_line() {
        // `url.<base>.insteadOf` sends every push elsewhere, `include.path` and
        // `includeIf.<condition>.path` read another file, `extensions` changes how the repository
        // is read: each of their keys is guarded (AUD-023, `REF-GIT-HOOKS-PATH`, Syntax).
        assert_eq!(
            guarded_lines(
                "[url \"/tmp/x\"]\n\tinsteadOf = https://h/r.git\n[include]\n\tpath = /tmp/evil\n\
                 [includeIf \"gitdir:/x/\"]\n\tpath = y\n[extensions]\n\tworktreeConfig = true\n"
            ),
            "[url \"/tmp/x\"]\n\tinsteadOf = https://h/r.git\n[include]\n\tpath = /tmp/evil\n\
             [includeIf \"gitdir:/x/\"]\n\tpath = y\n[extensions]\n\tworktreeConfig = true\n"
        );
        // The rest of a header's line sets a key; a comment there, or a `;` comment line, sets
        // none; a `]` inside a subsection's quotes ends nothing.
        assert_eq!(
            guarded_lines(
                "[core] bare = true\n[gc] auto = 0\n[user] ; who\n\t; name = x\n[remote \"a]b\"] url = u\n"
            ),
            "[core] bare = true\n[remote \"a]b\"] url = u\n"
        );
        let header_key = "[core]\n[core] bare = true\n";

        assert!(config_report("c", Some(b"[core]\n"), Some(header_key.as_bytes())).is_some());
    }

    #[test]
    fn f21_ac9_lets_a_branch_tracked_by_another_worktree_through() {
        // A parallel worktree's `git push -u` writes its branch's tracking lines into the
        // common `.git/config` (CHK-022): no failure.
        let tracked = format!(
            "{CONFIG}[branch \"chore/en-113-x\"]\n\tremote = origin\n\tmerge = refs/heads/chore/en-113-x\n"
        );

        assert_eq!(
            config_report("c", Some(CONFIG.as_bytes()), Some(tracked.as_bytes())),
            None
        );
        // A remote's URL changed, or a hooks path, fails.
        let moved = CONFIG.replace("https://h/r.git", "/tmp/r.git");
        assert_eq!(
            config_report("c", Some(CONFIG.as_bytes()), Some(moved.as_bytes())).as_deref(),
            Some(
                "✗ c changed while the checks ran, restored nothing:\n- \turl = https://h/r.git\n+ \turl = /tmp/r.git"
            )
        );
        let unhooked = CONFIG.replace("\thooksPath = .githooks\n", "");
        assert_eq!(
            config_report("c", Some(CONFIG.as_bytes()), Some(unhooked.as_bytes())).as_deref(),
            Some("✗ c changed while the checks ran, restored nothing:\n- \thooksPath = .githooks")
        );
    }
}
