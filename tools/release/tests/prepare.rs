//! Tests of `release prepare`, `release check` and `release notes` (EN-112, F21.AC6) on temporary
//! repositories whose history is recorded here: a tag, commits on `develop`, a feature merged by
//! GitHub's merge commit, a revert, then the release prepared, checked, rewritten and read back.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use release::changelog::HEADER;

const PACKAGE: &str = "{\n  \"name\": \"editor\",\n  \"version\": \"0.2.0\",\n  \"repository\": {\n    \"type\": \"git\",\n    \"url\": \"git+https://github.com/o/r.git\"\n  }\n}\n";

const LOCK: &str = "{\n  \"name\": \"editor\",\n  \"version\": \"0.2.0\",\n  \"lockfileVersion\": 3,\n  \"packages\": {\n    \"\": {\n      \"name\": \"editor\",\n      \"version\": \"0.2.0\"\n    }\n  }\n}\n";

const OLD_ENTRY: &str = "## [0.2.0](https://github.com/o/r/compare/v0.1.0...v0.2.0) (2026-10-14)\n\n### Features\n\n* **model:** x ([abc1234](https://github.com/o/r/commit/abc1234))\n";

/// A temporary repository, removed when the test ends, even on a failure.
struct Repository(PathBuf);

impl Drop for Repository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Runs Git in a folder, its inherited `GIT_*` variables removed, and returns its output.
fn git(folder: &Path, args: &[&str]) -> String {
    let output = test_support::command("git")
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "tag.gpgsign=false",
        ])
        .args(args)
        .current_dir(folder)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// Writes a file of the repository, its folders made.
fn write(folder: &Path, file: &str, text: &str) {
    let path = folder.join(file);

    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// A commit of a new file named after its message's first word count; its hash.
fn commit(folder: &Path, message: &str) -> String {
    let count = git(folder, &["rev-list", "--count", "--all"]);

    write(folder, &format!("work/{count}.txt"), message);
    git(folder, &["add", "."]);
    git(folder, &["commit", "-q", "-m", message]);
    git(folder, &["rev-parse", "HEAD"])
}

/// A repository at `v0.2.0` on `develop`: the manifests, release-please's changelog, a story of a
/// feature in the backlog.
fn repository(name: &str) -> Repository {
    let folder = std::env::temp_dir().join(format!("release-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    fs::create_dir_all(&folder).unwrap();
    git(&folder, &["init", "-q", "-b", "develop"]);
    write(&folder, "package.json", PACKAGE);
    write(&folder, "package-lock.json", LOCK);
    write(
        &folder,
        "CHANGELOG.md",
        &format!("# Changelog\n\n{OLD_ENTRY}"),
    );
    write(
        &folder,
        "docs/backlog/stories/E1-F16-US-041-colors.md",
        "---\nid: US-041\nepic: E1\nfeature: F16\ntitle: the colors chosen\nstatus: done\n---\n",
    );
    write(
        &folder,
        "docs/backlog/features/E1-F16-colors.md",
        "---\nid: F16\nepic: E1\ntitle: Colors\nstatus: done\n---\n",
    );
    git(&folder, &["add", "."]);
    git(&folder, &["commit", "-q", "-m", "chore: release 0.2.0"]);
    git(&folder, &["tag", "v0.2.0"]);
    Repository(folder)
}

/// Runs the release binary in a folder.
fn release(folder: &Path, args: &[&str]) -> Output {
    test_support::command(env!("CARGO_BIN_EXE_release"))
        .args(args)
        .current_dir(folder)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The recorded history of 0.3.0: a fix and a log on `develop`, a feature merged as pull request
/// #12 with its story's feat and fix and a tooling commit, a feat and its revert. Returns the
/// fix's hash.
fn history_of_0_3_0(folder: &Path) -> String {
    commit(folder, "docs(docs): log the run");
    let fix = commit(folder, "fix(model): refuse a NaN");
    git(folder, &["switch", "-q", "-c", "feat/val-1-validate-f16"]);
    commit(
        folder,
        "feat(model): give a text its colors\n\nRefs: US-041.T1",
    );
    commit(folder, "fix(playground): keep the hue\n\nRefs: US-041.T2");
    commit(folder, "feat(tooling): a status line");
    git(folder, &["switch", "-q", "develop"]);
    git(
        folder,
        &[
            "merge",
            "-q",
            "--no-ff",
            "-m",
            "Merge pull request #12 from o/feat/val-1-validate-f16",
            "feat/val-1-validate-f16",
        ],
    );
    commit(folder, "feat(model): add a flag");
    git(folder, &["revert", "--no-edit", "HEAD"]);
    fix
}

#[test]
fn f21_ac6_prepares_the_version_and_the_draft_on_its_release_branch() {
    let repository = repository("prepare");
    let folder = &repository.0;
    let fix = history_of_0_3_0(folder);

    let prepared = release(folder, &["prepare", "--date", "2026-10-18"]);

    assert!(prepared.status.success(), "{}", stderr(&prepared));
    // A feat since v0.2.0, no break: 0.3.0. The story's feat and fix are one Added line, with the
    // pull request; the fix of no story is a line with its commit; the log, the tooling, the
    // feat and its revert are left out.
    let entry = format!(
        "## [0.3.0] - 2026-10-18\n\n### Added\n\n\
         - [draft] The colors chosen ([#12](https://github.com/o/r/pull/12))\n\n### Fixed\n\n\
         - [draft] Refuse a NaN ([`{}`](https://github.com/o/r/commit/{fix}))\n",
        &fix[..7]
    );
    assert_eq!(
        fs::read_to_string(folder.join("CHANGELOG.md")).unwrap(),
        format!(
            "{HEADER}\n{entry}\n{}\n\n[0.3.0]: https://github.com/o/r/releases/tag/v0.3.0\n",
            OLD_ENTRY.trim_end()
        )
    );
    assert_eq!(
        fs::read_to_string(folder.join("package.json")).unwrap(),
        PACKAGE.replace("0.2.0", "0.3.0")
    );
    assert_eq!(
        fs::read_to_string(folder.join("package-lock.json")).unwrap(),
        LOCK.replace("0.2.0", "0.3.0")
    );
    assert_eq!(git(folder, &["branch", "--show-current"]), "release/0.3.0");
    let said = stdout(&prepared);
    assert!(
        said.contains("Added · F16 Colors · US-041: The colors chosen\n"),
        "{said}"
    );
    assert!(
        said.contains("feat(model): give a text its colors\n"),
        "{said}"
    );
    assert!(
        said.contains("v0.2.0 → 0.3.0 on release/0.3.0: 2 line(s) drafted"),
        "{said}"
    );

    // The draft is refused until each line is rewritten.
    let drafted = release(folder, &["check"]);
    assert_eq!(drafted.status.code(), Some(1));
    assert_eq!(
        stderr(&drafted)
            .matches("its `[draft]` mark removed")
            .count(),
        2
    );

    let rewritten = fs::read_to_string(folder.join("CHANGELOG.md"))
        .unwrap()
        .replace("[draft] The colors chosen", "Choose a text's colors")
        .replace("[draft] ", "");
    write(folder, "CHANGELOG.md", &rewritten);
    let checked = release(folder, &["check"]);
    assert!(checked.status.success(), "{}", stderr(&checked));
    assert!(stdout(&checked).contains("the entry of 0.3.0 follows the rules"));

    let notes = release(folder, &["notes"]);
    assert_eq!(
        stdout(&notes),
        "### Added\n\n- Choose a text's colors ([#12](https://github.com/o/r/pull/12))\n\n\
         ### Fixed\n\n"
            .to_owned()
            + &format!(
                "- Refuse a NaN ([`{}`](https://github.com/o/r/commit/{fix}))\n",
                &fix[..7]
            )
    );
    assert!(stdout(&release(folder, &["notes", "0.2.0"])).starts_with("### Features\n"));
    let missing = release(folder, &["notes", "9.9.9"]);
    assert_eq!(missing.status.code(), Some(1));
    assert!(stderr(&missing).contains("holds no entry of 9.9.9"));
}

#[test]
fn f21_ac6_raises_the_minor_version_for_a_break_and_the_patch_for_a_fix() {
    let breaking = repository("breaking");
    commit(&breaking.0, "refactor(geometry)!: one path writer");
    assert!(
        release(&breaking.0, &["prepare", "--date", "2026-10-18"])
            .status
            .success()
    );
    let changelog = fs::read_to_string(breaking.0.join("CHANGELOG.md")).unwrap();
    assert!(changelog.contains(
        "## [0.3.0] - 2026-10-18\n\n### Changed\n\n- **Breaking:** [draft] One path writer"
    ));

    let fix = repository("fix");
    commit(&fix.0, "fix(io): read the tables once");
    assert!(
        release(&fix.0, &["prepare", "--date", "2026-10-18"])
            .status
            .success()
    );
    assert_eq!(git(&fix.0, &["branch", "--show-current"]), "release/0.2.1");

    // Tooling only: a patch, and the notice that says why the entry lists no change.
    let tooling = repository("tooling");
    commit(&tooling.0, "feat(tooling): a status line");
    assert!(
        release(&tooling.0, &["prepare", "--date", "2026-10-18"])
            .status
            .success()
    );
    let changelog = fs::read_to_string(tooling.0.join("CHANGELOG.md")).unwrap();
    assert!(
        changelog
            .contains("## [0.2.1] - 2026-10-18\n\n_No change to the library nor the playground")
    );
    assert!(release(&tooling.0, &["check"]).status.success());
}

#[test]
fn refuses_a_dirty_checkout_no_commit_a_disagreeing_version_and_no_tag() {
    let refused = |repository: &Repository, expected: &str| {
        let output = release(&repository.0, &["prepare", "--date", "2026-10-18"]);

        assert_eq!(output.status.code(), Some(1), "{expected}");
        assert!(stderr(&output).contains(expected), "{}", stderr(&output));
        // Nothing written, no branch made.
        assert_eq!(git(&repository.0, &["branch", "--show-current"]), "develop");
    };

    let empty = repository("empty");
    refused(&empty, "no commit since v0.2.0: nothing to release");

    let dirty = repository("dirty");
    commit(&dirty.0, "fix(io): x");
    write(&dirty.0, "package.json", "{}");
    refused(&dirty, "the checkout has uncommitted changes");

    let disagreeing = repository("disagreeing");
    write(
        &disagreeing.0,
        "package.json",
        &PACKAGE.replace("0.2.0", "0.2.5"),
    );
    commit(&disagreeing.0, "fix(io): x");
    refused(&disagreeing, "package.json says 0.2.5, the last tag v0.2.0");

    let untagged = repository("untagged");
    git(&untagged.0, &["tag", "-d", "v0.2.0"]);
    refused(&untagged, "no release tag vx.y.z behind HEAD");

    let held = repository("held");
    write(
        &held.0,
        "CHANGELOG.md",
        "# Changelog\n\n## [0.2.1] - 2026-10-15\n\n_x._\n",
    );
    commit(&held.0, "fix(io): x");
    refused(&held, "CHANGELOG.md already holds 0.2.1");
}

#[test]
fn says_how_to_call_it() {
    let usage = repository("usage");

    for args in [&["publish-nothing"][..], &["prepare", "--when", "x"][..]] {
        let output = release(&usage.0, args);

        assert_eq!(output.status.code(), Some(2));
        assert!(
            stderr(&output).starts_with("usage: release"),
            "{}",
            stderr(&output)
        );
    }

    let lock = repository("lock");
    write(
        &lock.0,
        "package-lock.json",
        &LOCK.replacen("0.2.0", "0.1.0", 1),
    );
    let output = release(&lock.0, &["check"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output)
            .contains("package-lock.json: its two versions of the package must be 0.2.0")
    );
}
