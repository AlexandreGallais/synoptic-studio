//! Tests of a run on a tiny repository of its own, never the real one (EN-058); `[F18.AC15]` marks
//! the test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use mutate::{Kind, Outcome, Request, Verdict, run, summary, test_file};

/// A command cut off from the Git repository running the tests: a hook (`pre-push` running
/// `check:all`) sets `GIT_DIR` and its kin, which would point every `git` call at that repository.
fn isolated(program: &str, dir: &Path) -> Command {
    let mut command = test_support::command(program);

    command
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@example.com");
    command
}

fn git(dir: &Path, arguments: &[&str]) -> String {
    let output = isolated("git", dir)
        .args(["-c", "commit.gpgsign=false"])
        .args(arguments)
        .output()
        .unwrap();

    assert!(output.status.success(), "{output:?}");
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// A tiny repository in a new temporary folder, removed when dropped.
struct Project {
    dir: PathBuf,
}

impl Project {
    /// A repository holding `files`, committed.
    fn new(files: &[(&str, &str)]) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "mutate-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));

        fs::create_dir_all(&dir).unwrap();
        let dir = dir.canonicalize().unwrap();
        git(&dir, &["init", "--quiet"]);
        let project = Self { dir };

        project.write(files);
        git(&project.dir, &["add", "."]);
        git(&project.dir, &["commit", "--quiet", "-m", "start"]);
        project
    }

    fn write(&self, files: &[(&str, &str)]) {
        for (name, text) in files {
            fs::write(self.dir.join(name), text).unwrap();
        }
    }

    /// Runs the mutations of `file` against `node <test>`.
    fn mutate(&self, file: &str, test: &str, all: bool) -> Vec<Outcome> {
        let files = [file.to_owned()];
        let command = ["node".to_owned(), test.to_owned(), "{file}".to_owned()];
        let request = Request {
            dir: &self.dir,
            files: &files,
            command: &command,
            all,
            grace: Duration::from_secs(1),
        };

        run(&request, &mut |_| {}).unwrap()
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.dir);
    }
}

const CLAMP: &str = "export function clamp(value, low, high) {
  if (value < low) {
    return low;
  }
  return value > high ? high : value;
}
";

/// Tests `clamp` well, and `double` only at 0, where `x * 2`, `x / 2` and `x * 3` agree.
const TEST: &str = "import { clamp, double } from './lib.mjs';
const check = (got, want) => { if (got !== want) { process.exit(1); } };
check(clamp(-1, 0, 10), 0);
check(clamp(5, 0, 10), 5);
check(clamp(11, 0, 10), 10);
check(double(0), 0);
";

/// Each tried mutation as `kind verdict`.
fn verdicts(outcomes: &[Outcome]) -> Vec<(Kind, Verdict)> {
    outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            Outcome::Tried { kind, verdict, .. } => Some((*kind, *verdict)),
            Outcome::Skipped { .. } => None,
        })
        .collect()
}

#[test]
fn f18_ac15_reports_caught_and_surviving_mutations_in_a_copy() {
    // `double` is not committed: the copy holds the uncommitted change of a tracked file.
    let project = Project::new(&[
        (
            "lib.mjs",
            &format!("{CLAMP}export const double = (x) => 0;\n"),
        ),
        ("test.mjs", TEST),
    ]);
    let lib = format!("{CLAMP}export const double = (x) => x * 2;\n");
    project.write(&[("lib.mjs", &lib)]);

    let outcomes = project.mutate("lib.mjs", "test.mjs", false);

    // `<` flipped: clamp(5, 0, 10) returns 0. `x * 2` to `x / 2` and `2` to `3` agree at 0.
    // `if (!(value < low))` returns low for 5. `return undefined` for clamp(-1, 0, 10).
    assert_eq!(
        verdicts(&outcomes),
        [
            (Kind::Comparison, Verdict::Caught),
            (Kind::Arithmetic, Verdict::Survived),
            (Kind::Constant, Verdict::Survived),
            (Kind::Condition, Verdict::Caught),
            (Kind::Return, Verdict::Caught),
        ]
    );
    assert_eq!(
        summary(&outcomes),
        "3 caught, 2 survived, 0 file(s) skipped"
    );
    assert_eq!(
        outcomes[1].line(),
        "SURVIVED  lib.mjs:7:32  operator swapped: * → /"
    );
    // The repository is left as it was: the file unchanged, no worktree added.
    assert_eq!(
        fs::read_to_string(project.dir.join("lib.mjs")).unwrap(),
        lib
    );
    assert_eq!(git(&project.dir, &["worktree", "list"]).lines().count(), 1);
}

#[test]
fn tries_every_mutation_when_all_are_asked() {
    let project = Project::new(&[
        (
            "lib.mjs",
            &format!("{CLAMP}export const double = (x) => x * 2;\n"),
        ),
        ("test.mjs", TEST),
    ]);

    // The condition, `<`, two returns, `>`, `*` and `2`: seven.
    assert_eq!(project.mutate("lib.mjs", "test.mjs", true).len(), 7);
}

#[test]
fn stops_a_mutation_that_never_ends() {
    // `0` to `1` makes `i` odd: it never equals 10.
    let project = Project::new(&[
        (
            "loop.mjs",
            "export function steps() {\n  let i = 0;\n  while (i !== 10) {\n    i += 2;\n  }\n  return i;\n}\n",
        ),
        (
            "test.mjs",
            "import { steps } from './loop.mjs';\nif (steps() !== 10) { process.exit(1); }\n",
        ),
    ]);

    assert_eq!(
        verdicts(&project.mutate("loop.mjs", "test.mjs", false)),
        [
            (Kind::Comparison, Verdict::Caught),
            (Kind::Constant, Verdict::TimedOut),
            (Kind::Return, Verdict::Caught),
        ]
    );
}

#[test]
fn skips_a_file_whose_tests_fail_unchanged_or_that_is_not_in_the_repository() {
    let project = Project::new(&[
        ("lib.mjs", "export const one = () => 1;\n"),
        ("test.mjs", "process.exit(1);\n"),
    ]);
    let outcomes = project.mutate("lib.mjs", "test.mjs", false);

    assert_eq!(
        outcomes,
        [Outcome::Skipped {
            file: "lib.mjs".to_owned(),
            reason: "its related tests fail, or none was found, before any mutation".to_owned(),
        }]
    );
    assert!(matches!(
        &project.mutate("missing.mjs", "test.mjs", false)[..],
        [Outcome::Skipped { .. }]
    ));
}

#[test]
fn names_the_test_file_beside_a_source_file() {
    assert_eq!(test_file("src/math/clamp.ts"), "src/math/clamp.test.ts");
    assert_eq!(test_file("lib.mjs"), "lib.test.mjs");
    assert_eq!(test_file("Makefile"), "Makefile.test");
}

#[test]
fn runs_only_the_file_s_own_test_file_when_asked() {
    // `{test}` is `lib.test.mjs`: it exits 1 when `one()` is not 1, which `1` to `2` makes it.
    let project = Project::new(&[
        ("lib.mjs", "export const one = () => 1;\n"),
        (
            "lib.test.mjs",
            "import { one } from './lib.mjs';\nif (one() !== 1) { process.exit(1); }\n",
        ),
    ]);
    let files = ["lib.mjs".to_owned()];
    let command = ["node".to_owned(), "{test}".to_owned()];
    let request = Request {
        dir: &project.dir,
        files: &files,
        command: &command,
        all: false,
        grace: Duration::from_secs(1),
    };

    assert_eq!(
        verdicts(&run(&request, &mut |_| {}).unwrap()),
        [(Kind::Constant, Verdict::Caught)]
    );
}
