//! What `check:all` decides (RET-002, EN-085, EN-103): its lanes of npm scripts — in order within
//! a lane, the lanes at the same time, so that the pre-push hook waits for the slowest lane instead
//! of the sum — the docs lane left out when the branch changed none of its inputs against its base,
//! a check's report line, and the lines of `.git/config` a lane added or removed.
//!
//! No release binary is written while the lanes run (EN-069's fix): `cargo run` copies its crate's
//! binary into `target/release/` again even when it is up to date, removing it first, so a lane
//! starting that binary at the same moment failed with `could not execute process … No such file
//! or directory`. The release binaries are built once before the lanes ([`BEFORE_LANES`]), and a
//! lane runs a script that takes the heavy lock without it ([`command_of`]), `check:all` already
//! holding that lock.

use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

/// The lane of the documentation site and the playground built into it.
pub const DOCS_LANE: &[&str] = &["docs:build", "playground:build"];

/// Lanes of npm scripts: in order within a lane, the lanes at the same time.
pub const LANES: &[&[&str]] = &[
    &["lint"],
    &["test"],
    DOCS_LANE,
    &[
        "format:check",
        "lint:secrets",
        "deps:audit",
        "deps:unused",
        "deps:outdated",
    ],
    // The Rust workspace: rustfmt, Clippy, its advisories, its tests (EN-068, EN-069).
    &["check:rust"],
];

/// The check run alone before the lanes: the release binaries every lane's tools run from.
pub const BEFORE_LANES: &str = "rust:build";

/// How a script starts when it takes the heavy lock, which `check:all` holds for its lanes.
const HEAVY_LOCK: &str = "cargo run --release -q -p heavy-lock -- ";

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
    paths.iter().any(|path| {
        DOCS_INPUTS.iter().any(|input| {
            if input.ends_with('/') {
                path.starts_with(input)
            } else {
                path == input
            }
        })
    })
}

/// The lanes to run: every lane, the docs lane left out when the change is known (`Some`) and
/// touches none of its inputs.
#[must_use]
pub fn lanes_to_run(paths: Option<&[String]>) -> Vec<&'static [&'static str]> {
    LANES
        .iter()
        .copied()
        .filter(|lane| paths.is_none_or(touches_site) || *lane != DOCS_LANE)
        .collect()
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

/// The command a lane runs for a script: the script itself, without the heavy lock `check:all`
/// already holds, so that no two lanes start `cargo run -p heavy-lock` at once.
#[must_use]
pub fn command_of(script: &str) -> &str {
    script.strip_prefix(HEAVY_LOCK).unwrap_or(script)
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

/// The line skipping the docs lane, against the base the change was read from.
#[must_use]
pub fn skipped_line(base: &str) -> String {
    format!(
        "- {} skipped: nothing changed in their inputs against {base}",
        DOCS_LANE.join(", ")
    )
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

/// The report of `.git/config` read before and after the lanes (`None` when it could not be
/// read): nothing when its bytes are the same, else the failure naming the lines added or removed.
/// Nothing is restored: what changed it is a bug to find, and the old content is in the report.
#[must_use]
pub fn config_report(path: &str, before: Option<&[u8]>, after: Option<&[u8]>) -> Option<String> {
    if before == after {
        return None;
    }

    let text = |bytes: Option<&[u8]>| {
        bytes
            .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
            .unwrap_or_default()
    };
    let mut lines = changed_lines(&text(before), &text(after));
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

    let output = command.args(arguments).current_dir(root).output()?;

    if !output.status.success() {
        return Err(io::Error::other(format!(
            "git {}: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout)
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
    let branch = git_lines(root, &["rev-parse", "--abbrev-ref", "HEAD"])?
        .into_iter()
        .next()
        .unwrap_or_else(|| "HEAD".to_owned());
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
    fn runs_a_script_that_takes_the_heavy_lock_without_it() {
        assert_eq!(
            command_of("cargo run --release -q -p heavy-lock -- vitest run --coverage"),
            "vitest run --coverage"
        );
        assert_eq!(
            command_of("cargo run --release -q -p heavy-lock -- --lock merge node x.ts"),
            "--lock merge node x.ts"
        );
        assert_eq!(command_of("tsc --noEmit"), "tsc --noEmit");
        assert_eq!(command_of(""), "");
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
            skipped_line("origin/develop"),
            "- docs:build, playground:build skipped: nothing changed in their inputs against origin/develop"
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
        let docs = owned(&["docs/index.md"]);

        assert!(!lanes_to_run(Some(&tools)).contains(&DOCS_LANE));
        assert_eq!(lanes_to_run(Some(&tools)).len(), LANES.len() - 1);
        assert_eq!(lanes_to_run(Some(&docs)), LANES);
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
            config_report("c", None, Some(b"x\n")).as_deref(),
            Some("✗ c created while the checks ran, restored nothing:\n+ x")
        );
        // Bytes differ, lines the same: a line ending changed.
        assert_eq!(
            config_report("c", Some(b"x\n"), Some(b"x\r\n")).as_deref(),
            Some(
                "✗ c changed while the checks ran, restored nothing:\n  the same lines, their endings or bytes changed"
            )
        );
    }
}
